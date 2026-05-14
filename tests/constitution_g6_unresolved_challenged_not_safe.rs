//! TB-G G6 — unresolved-challenged targets must not appear in market context.

use turingosv4::sdk::market_context::render_market_context;
use turingosv4::state::q_state::{AgentId, ChallengeCase, ChallengeStatus, QState, TaskId, TxId};

fn insert_active_pool(q: &mut QState, work_tx_id: &str) {
    let event_id = turingosv4::state::typed_tx::node_survive_event_id(&TxId(work_tx_id.into()));
    q.economic_state_t.cpmm_pools_t.0.insert(
        event_id.clone(),
        turingosv4::state::q_state::CpmmPool {
            event_id,
            pool_yes: turingosv4::state::typed_tx::ShareAmount::from_units(1_000_000),
            pool_no: turingosv4::state::typed_tx::ShareAmount::from_units(1_000_000),
            lp_total_shares: turingosv4::state::q_state::LpShareAmount::from_units(1_000_000),
            status: turingosv4::state::q_state::PoolStatus::Active,
        },
    );
}

#[test]
fn sg_g6_5_open_challenge_target_is_filtered_from_top_k() {
    let mut q = QState::default();
    insert_active_pool(&mut q, "worktx-open-challenge");
    insert_active_pool(&mut q, "worktx-clean");
    q.economic_state_t.challenge_cases_t.0.insert(
        TxId("challenge-1".into()),
        ChallengeCase {
            target_work_tx: TxId("worktx-open-challenge".into()),
            status: ChallengeStatus::Open,
            ..ChallengeCase::default()
        },
    );

    let out = render_market_context(
        &q,
        &TaskId("task-1".into()),
        &[
            TxId("worktx-open-challenge".into()),
            TxId("worktx-clean".into()),
        ],
        10,
        &AgentId("Agent_2".into()),
    );

    assert!(!out.contains("worktx-open-challenge"));
    assert!(out.contains("worktx-clean"));
}

#[test]
fn sg_g6_6_resolved_challenge_target_can_reappear_as_signal() {
    let mut q = QState::default();
    insert_active_pool(&mut q, "worktx-resolved-challenge");
    q.economic_state_t.challenge_cases_t.0.insert(
        TxId("challenge-1".into()),
        ChallengeCase {
            target_work_tx: TxId("worktx-resolved-challenge".into()),
            status: ChallengeStatus::Released,
            ..ChallengeCase::default()
        },
    );

    let out = render_market_context(
        &q,
        &TaskId("task-1".into()),
        &[TxId("worktx-resolved-challenge".into())],
        10,
        &AgentId("Agent_2".into()),
    );

    assert!(out.contains("worktx-resolved-challenge"));
}
