//! REAL-6C — ConvictionBudget / PnL feedback gates.
//!
//! Architect gates:
//! - SG-6C.1 PnL derived from ChainTape/CAS.
//! - SG-6C.2 No PnL HashMap sidecar source-of-truth.
//! - SG-6C.3 Agent prompt sees scoped PnL summary.
//! - SG-6C.4 Low-balance agent blocked from high-risk market actions.
//! - SG-6C.5 Low-balance agent not erased / reset.
//! - SG-6C.6 AutopsyCapsule generated after significant loss.

use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::economy::money::MicroCoin;
use turingosv4::runtime::autopsy_capsule::restore_autopsy_capsule_from_cas_bytes;
use turingosv4::runtime::real5_roles::AgentRole;
use turingosv4::runtime::real5_roles::{MarketInvestPayload, RoleAction, RoleActionRoute};
use turingosv4::runtime::real6_conviction_budget::{
    conviction_action_allowed, derive_conviction_budget, render_scoped_conviction_budget_summary,
    route_role_action_with_conviction_budget, write_significant_loss_autopsy_to_cas,
    ConvictionAction,
};
use turingosv4::state::q_state::{AgentId, QState, StakeEntry, TaskId, TxId};
use turingosv4::state::typed_tx::CapsulePrivacyPolicy;

const SRC: &str = "src/runtime/real6_conviction_budget.rs";
const YOUR_POSITION_SRC: &str = "src/sdk/your_position.rs";
const EVALUATOR_SRC: &str = "experiments/minif2f_v4/src/bin/evaluator.rs";

fn agent(name: &str) -> AgentId {
    AgentId(name.into())
}

fn q_with_balance(agent_id: &AgentId, balance_micro: i64) -> QState {
    let mut q = QState::default();
    q.economic_state_t
        .balances_t
        .0
        .insert(agent_id.clone(), MicroCoin::from_micro_units(balance_micro));
    q
}

#[test]
fn sg_6c_1_and_6c_2_conviction_budget_is_chain_derived_not_hashmap_sidecar() {
    let src = std::fs::read_to_string(SRC).expect("read REAL-6C source");
    assert!(
        src.contains("compute_agent_pnl"),
        "SG-6C.1: ConvictionBudget must derive PnL via canonical agent_pnl view"
    );
    assert!(
        src.contains("bankruptcy_risk_cap_micro"),
        "SG-6C.1: risk cap must reuse G3 per-agent ChainTape/QState-derived cap"
    );
    assert!(
        !src.contains("HashMap"),
        "SG-6C.2: no HashMap sidecar source-of-truth in REAL-6C module"
    );

    let a = agent("Agent_0");
    let mut q = q_with_balance(&a, 80_000);
    q.economic_state_t.stakes_t.0.insert(
        TxId("reserved-work".into()),
        StakeEntry {
            amount: MicroCoin::from_micro_units(25_000),
            staker: a.clone(),
            task_id: TaskId("task-reserved".into()),
        },
    );

    let budget = derive_conviction_budget(&q, &a);
    assert_eq!(budget.agent_id, a);
    assert_eq!(budget.available_micro, 80_000);
    assert_eq!(budget.reserved_micro, 25_000);
    assert_eq!(budget.realized_pnl, -920_000);
    assert_eq!(budget.risk_cap, 100_000);
}

#[test]
fn sg_6c_3_prompt_sees_scoped_pnl_summary_without_other_agent_leak() {
    let prompt_src = std::fs::read_to_string(YOUR_POSITION_SRC).expect("read your_position src");
    assert!(
        prompt_src.contains("render_scoped_conviction_budget_summary"),
        "SG-6C.3: production prompt read view must include REAL-6C scoped ConvictionBudget"
    );

    let alice = agent("Agent_0");
    let bob = agent("Agent_1");
    let mut q = q_with_balance(&alice, 900_000);
    q.economic_state_t
        .balances_t
        .0
        .insert(bob.clone(), MicroCoin::from_micro_units(123_456));
    q.economic_state_t.stakes_t.0.insert(
        TxId("bob-private-position".into()),
        StakeEntry {
            amount: MicroCoin::from_micro_units(77_777),
            staker: bob,
            task_id: TaskId("task-bob".into()),
        },
    );

    let summary = render_scoped_conviction_budget_summary(&q, &alice);
    assert!(summary.contains("=== Conviction Budget ==="));
    assert!(summary.contains("available_micro=900000"));
    assert!(summary.contains("risk_cap=100000"));
    assert!(
        !summary.contains("Agent_1") && !summary.contains("bob-private-position"),
        "SG-6C.3: scoped summary must not leak another agent's PnL or positions; got:\n{summary}"
    );
}

#[test]
fn sg_6c_4_and_6c_5_low_balance_blocks_only_high_risk_actions_without_erasure() {
    let evaluator_src = std::fs::read_to_string(EVALUATOR_SRC).expect("read evaluator src");
    assert!(
        evaluator_src.contains("derive_conviction_budget")
            && evaluator_src.contains("route_role_action_with_conviction_budget"),
        "SG-6C.4: production role gateway must route high-risk actions through ConvictionBudget"
    );

    let a = agent("Agent_0");
    let q = q_with_balance(&a, 90_000);
    let budget = derive_conviction_budget(&q, &a);
    assert_eq!(
        budget.agent_id, a,
        "SG-6C.5: low-balance agent remains visible"
    );

    for action in [
        ConvictionAction::Observe,
        ConvictionAction::Read,
        ConvictionAction::Abstain,
        ConvictionAction::Solve,
        ConvictionAction::Verify,
    ] {
        assert!(
            conviction_action_allowed(&budget, AgentRole::Trader, action).allowed,
            "SG-6C.4: low-balance agent can still {action:?}"
        );
    }
    assert!(
        !conviction_action_allowed(&budget, AgentRole::Trader, ConvictionAction::HighRiskMarket)
            .allowed,
        "SG-6C.4: low-balance Trader cannot take high-risk market action"
    );
    assert!(
        !conviction_action_allowed(
            &budget,
            AgentRole::Challenger,
            ConvictionAction::HighRiskChallenge
        )
        .allowed,
        "SG-6C.4: low-balance Challenger cannot take high-risk challenge action"
    );
    let invest_route = route_role_action_with_conviction_budget(
        AgentRole::Trader,
        &RoleAction::Invest(MarketInvestPayload::default()),
        Some(&budget),
    );
    assert!(
        matches!(
            invest_route,
            RoleActionRoute::L4E {
                ref public_summary,
                ..
            } if public_summary.contains("below_conviction_risk_cap")
        ),
        "SG-6C.4: production role route should L4.E low-balance high-risk invest; got {invest_route:?}"
    );

    let topped_up = q_with_balance(&a, 150_000);
    let topped_up_budget = derive_conviction_budget(&topped_up, &a);
    assert!(
        conviction_action_allowed(
            &topped_up_budget,
            AgentRole::Trader,
            ConvictionAction::HighRiskMarket
        )
        .allowed,
        "above risk cap, Trader high-risk market action is available"
    );
}

#[test]
fn sg_6c_6_significant_loss_generates_audit_only_autopsy_capsule() {
    use std::sync::{Arc, RwLock};

    let a = agent("Agent_0");
    let q = q_with_balance(&a, 50_000);
    let budget = derive_conviction_budget(&q, &a);
    let tmp = tempfile::TempDir::new().expect("tempdir");
    let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("open cas")));
    let evidence_cid = Cid::from_content(b"real6c-loss-evidence");
    let capsule = write_significant_loss_autopsy_to_cas(
        &cas,
        &budget,
        TaskId("task-loss".into()),
        MicroCoin::from_micro_units(250_000),
        vec![evidence_cid],
        "real6c-test",
        7,
        3,
    )
    .expect("autopsy writer should succeed")
    .expect("significant loss should generate an autopsy capsule");

    assert_eq!(capsule.agent_id, a);
    assert_eq!(capsule.loss_amount.micro_units(), 950_000);
    assert_eq!(capsule.privacy_policy, CapsulePrivacyPolicy::AuditOnly);
    assert_eq!(capsule.evidence_cids, vec![evidence_cid]);
    assert_eq!(capsule.created_at_logical_t, 7);
    assert_eq!(capsule.created_at_round, 3);
    let bytes = cas
        .read()
        .expect("cas read lock")
        .get(&capsule.capsule_id)
        .expect("capsule must be CAS-resident");
    let restored = restore_autopsy_capsule_from_cas_bytes(&bytes).expect("restore capsule");
    assert_eq!(restored.capsule_id, capsule.capsule_id);
    assert!(
        capsule.public_summary.contains("reason=Overleverage"),
        "SG-6C.6: loss reason should be visible in low-info summary"
    );
}
