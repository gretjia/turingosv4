//! TB-G G3.1 — `compute_agent_pnl` derived-view binding tests (Class 2).
//!
//! Charter: `handover/tracer_bullets/TB_G_GENERATIVE_ARENA_charter_2026-05-11.md`
//! §1 Module G3 atom G3.1.
//!
//! Directive: `handover/directives/2026-05-11_G_PHASE_GENERATIVE_ARENA_DIRECTIVE.md`
//! §G3 verbatim 7-field `AgentMarketState` shape + SG-G3.1..G3.5.
//!
//! Ship gates per charter row G3.1:
//!
//! - **SG-G3.1** Genesis QState (empty `EconomicState`) yields
//!   `realized_pnl == 0` for an agent at its preseed baseline AND
//!   `unrealized_pnl == 0` with empty `open_positions`. Anchors the
//!   architect's directive §G3 SG-G3.1 "agent balance changes persist
//!   across tasks" by witnessing the zero-baseline.
//! - **SG-G3.2** Post-`BuyWithCoinRouter`-equivalent state (asymmetric
//!   YES-heavy holding under an Active CpmmPool) yields signed
//!   `unrealized_pnl` AND `realized_pnl` reflects the cash debit.
//!   Anchors the architect's "post-BuyRouter unrealized updates"
//!   binding (charter §1 Module G3 atom G3.1 row 2).
//! - **SG-G3.3** Five distinct scenarios are covered by the binding
//!   tests: (a) empty genesis, (b) balanced mint with no pool, (c)
//!   balanced mint with skewed active pool (PnL still 0), (d)
//!   asymmetric mint with active pool (signed PnL), (e) post-resolve
//!   pool yields 0 unrealized.
//! - **SG-G3.9** Source-grep witness: the 7 architect-spec'd field names
//!   (`agent_id`, `balance`, `open_positions`, `realized_pnl`,
//!   `unrealized_pnl`, `solvency_status`, `reputation_score`) all
//!   appear in `src/runtime/agent_pnl.rs` as struct fields.
//! - **SG-G3.9.a** Source-grep witness for the 3-tier solvency enum:
//!   `Solvent`, `NearInsolvent`, `Bankrupt` all appear in
//!   `src/runtime/agent_pnl.rs`.
//! - **SG-G3.9.b** Source-grep witness: the module reads canonical state
//!   indices `balances_t` / `stakes_t` / `claims_t` / `reputations_t` /
//!   `conditional_share_balances_t` / `cpmm_pools_t` /
//!   `lp_share_balances_t` (architect's "Pure derivation" binding).
//! - **SG-G3.9.c** No-f64 lint: `src/runtime/agent_pnl.rs` contains no
//!   `f32` / `f64` tokens (CLAUDE.md §13 no-f64-in-money-path; integer
//!   math throughout).
//!
//! `FC-trace: FC1-N7 + §13 + Art. III.2 — agent perceives PnL state at
//! the δ / Agent externalized output node. Per-viewer materialized view;
//! never aggregates across agents.`

use turingosv4::economy::money::MicroCoin;
use turingosv4::runtime::agent_pnl::{
    compute_agent_pnl, initial_balance_micro_from_default_preseed, OpenPosition, SolvencyStatus,
};
use turingosv4::state::q_state::{
    AgentId, ClaimEntry, ClaimStatus, CpmmPool, LpShareAmount, PoolStatus, QState, ShareSidePair,
    StakeEntry, TaskId, TxId,
};
use turingosv4::state::typed_tx::{EventId, ShareAmount};

const AGENT_PNL_SRC: &str = "src/runtime/agent_pnl.rs";

fn agent(name: &str) -> AgentId {
    AgentId(name.into())
}

fn event(name: &str) -> EventId {
    EventId(TaskId(name.into()))
}

fn empty_q() -> QState {
    QState::default()
}

// ────────────────────────────────────────────────────────────────────────
// SG-G3.1 — genesis returns zero PnL at preseed baseline
// ────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g3_1_genesis_at_preseed_baseline_yields_zero_pnl() {
    let mut q = empty_q();
    let a = agent("Agent_0");
    let initial = initial_balance_micro_from_default_preseed(&a);
    assert_eq!(initial, 1_000_000, "Agent_0 preseed baseline (bootstrap.rs)");
    q.economic_state_t
        .balances_t
        .0
        .insert(a.clone(), MicroCoin::from_micro_units(initial));
    let view = compute_agent_pnl(&q, &a, initial);
    assert_eq!(view.balance, initial);
    assert_eq!(view.realized_pnl, 0, "balance == baseline ⇒ realized 0");
    assert_eq!(view.unrealized_pnl, 0, "no positions ⇒ unrealized 0");
    assert!(
        view.open_positions.is_empty(),
        "fresh-genesis agent has no open positions"
    );
    assert!(matches!(view.solvency_status, SolvencyStatus::Solvent));
}

// ────────────────────────────────────────────────────────────────────────
// SG-G3.2 — post-BuyWithCoinRouter cash drops + unrealized updates
// ────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g3_2_post_buy_router_cash_drops_and_unrealized_updates() {
    // Reproduces the post-BuyYes router state from MEMORY.md Stage C P-M6:
    // agent pays C=100k μC, receives 150k YES shares + 50k NO shares from
    // the 9-step atomic mint+swap arm. Pool ends with skewed reserves
    // (YES side drained by the swap) at pool_yes=50, pool_no=150.
    let mut q = empty_q();
    let a = agent("Agent_0");
    let ev = event("task-A");
    q.economic_state_t
        .balances_t
        .0
        .insert(a.clone(), MicroCoin::from_micro_units(900_000));
    let mut holdings = std::collections::BTreeMap::new();
    holdings.insert(
        ev.clone(),
        ShareSidePair {
            yes: ShareAmount::from_units(150_000),
            no: ShareAmount::from_units(50_000),
        },
    );
    q.economic_state_t
        .conditional_share_balances_t
        .0
        .insert(a.clone(), holdings);
    q.economic_state_t.cpmm_pools_t.0.insert(
        ev.clone(),
        CpmmPool {
            event_id: ev.clone(),
            pool_yes: ShareAmount::from_units(50),
            pool_no: ShareAmount::from_units(150),
            lp_total_shares: LpShareAmount::from_units(0),
            status: PoolStatus::Active,
        },
    );

    let view = compute_agent_pnl(&q, &a, 1_000_000);
    // Cash dropped by mint cost basis = (150 + 50)/2 × 1000 = 100k:
    assert_eq!(view.realized_pnl, -100_000, "post-BuyRouter cash drop");
    // Signed MTM PnL: yes_mtm + no_mtm − cost_basis = 125k − 100k = +25k:
    assert_eq!(
        view.unrealized_pnl, 25_000,
        "post-BuyRouter unrealized PnL signed"
    );
    // Both legs visible in open_positions:
    let yes_count = view
        .open_positions
        .iter()
        .filter(|p| matches!(p, OpenPosition::ConditionalShare { .. }))
        .count();
    assert_eq!(yes_count, 2, "YES + NO legs visible as open positions");
}

// ────────────────────────────────────────────────────────────────────────
// SG-G3.3 — five distinct scenarios are covered by the binding tests
// ────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g3_3_scenario_a_empty_genesis() {
    let view = compute_agent_pnl(&empty_q(), &agent("Agent_X"), 1_000_000);
    assert_eq!(view.balance, 0);
    assert_eq!(view.realized_pnl, -1_000_000);
    assert_eq!(view.unrealized_pnl, 0);
}

#[test]
fn sg_g3_3_scenario_b_balanced_mint_no_pool() {
    let mut q = empty_q();
    let a = agent("Agent_0");
    let ev = event("task-A");
    q.economic_state_t
        .balances_t
        .0
        .insert(a.clone(), MicroCoin::from_micro_units(900_000));
    let mut h = std::collections::BTreeMap::new();
    h.insert(
        ev.clone(),
        ShareSidePair {
            yes: ShareAmount::from_units(100_000),
            no: ShareAmount::from_units(100_000),
        },
    );
    q.economic_state_t
        .conditional_share_balances_t
        .0
        .insert(a.clone(), h);
    let view = compute_agent_pnl(&q, &a, 1_000_000);
    assert_eq!(view.realized_pnl, -100_000);
    assert_eq!(
        view.unrealized_pnl, 0,
        "no pool ⇒ no MTM signal ⇒ unrealized 0"
    );
}

#[test]
fn sg_g3_3_scenario_c_balanced_mint_with_skewed_pool() {
    let mut q = empty_q();
    let a = agent("Agent_0");
    let ev = event("task-A");
    q.economic_state_t
        .balances_t
        .0
        .insert(a.clone(), MicroCoin::from_micro_units(900_000));
    let mut h = std::collections::BTreeMap::new();
    h.insert(
        ev.clone(),
        ShareSidePair {
            yes: ShareAmount::from_units(100_000),
            no: ShareAmount::from_units(100_000),
        },
    );
    q.economic_state_t
        .conditional_share_balances_t
        .0
        .insert(a.clone(), h);
    q.economic_state_t.cpmm_pools_t.0.insert(
        ev.clone(),
        CpmmPool {
            event_id: ev.clone(),
            pool_yes: ShareAmount::from_units(20),
            pool_no: ShareAmount::from_units(80),
            lp_total_shares: LpShareAmount::from_units(0),
            status: PoolStatus::Active,
        },
    );
    let view = compute_agent_pnl(&q, &a, 1_000_000);
    assert_eq!(
        view.unrealized_pnl, 0,
        "balanced holding ⇒ MTM = face regardless of pool skew"
    );
}

#[test]
fn sg_g3_3_scenario_d_asymmetric_mint_with_active_pool() {
    // Same fixture as SG-G3.2 — included here so the SG-G3.3 binding
    // covers all 5 scenarios independently.
    let mut q = empty_q();
    let a = agent("Agent_0");
    let ev = event("task-A");
    let mut h = std::collections::BTreeMap::new();
    h.insert(
        ev.clone(),
        ShareSidePair {
            yes: ShareAmount::from_units(150_000),
            no: ShareAmount::from_units(50_000),
        },
    );
    q.economic_state_t
        .conditional_share_balances_t
        .0
        .insert(a.clone(), h);
    q.economic_state_t.cpmm_pools_t.0.insert(
        ev.clone(),
        CpmmPool {
            event_id: ev.clone(),
            pool_yes: ShareAmount::from_units(50),
            pool_no: ShareAmount::from_units(150),
            lp_total_shares: LpShareAmount::from_units(0),
            status: PoolStatus::Active,
        },
    );
    let view = compute_agent_pnl(&q, &a, 0);
    assert_eq!(view.unrealized_pnl, 25_000);
}

#[test]
fn sg_g3_3_scenario_e_post_resolve_pool_zero_unrealized() {
    let mut q = empty_q();
    let a = agent("Agent_0");
    let ev = event("task-A");
    let mut h = std::collections::BTreeMap::new();
    h.insert(
        ev.clone(),
        ShareSidePair {
            yes: ShareAmount::from_units(150_000),
            no: ShareAmount::from_units(50_000),
        },
    );
    q.economic_state_t
        .conditional_share_balances_t
        .0
        .insert(a.clone(), h);
    q.economic_state_t.cpmm_pools_t.0.insert(
        ev.clone(),
        CpmmPool {
            event_id: ev.clone(),
            pool_yes: ShareAmount::from_units(50),
            pool_no: ShareAmount::from_units(150),
            lp_total_shares: LpShareAmount::from_units(0),
            status: PoolStatus::Resolved,
        },
    );
    let view = compute_agent_pnl(&q, &a, 0);
    assert_eq!(
        view.unrealized_pnl, 0,
        "Resolved pool ⇒ no live MTM signal"
    );
}

// ────────────────────────────────────────────────────────────────────────
// SG-G3.9 — source-grep witness for the architect's 7-field shape
// ────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g3_9_source_grep_seven_field_shape() {
    let src = std::fs::read_to_string(AGENT_PNL_SRC).expect("read agent_pnl src");
    for field in [
        "agent_id",
        "balance",
        "open_positions",
        "realized_pnl",
        "unrealized_pnl",
        "solvency_status",
        "reputation_score",
    ] {
        let occurrences = src.matches(field).count();
        assert!(
            occurrences >= 2,
            "SG-G3.9: field {field:?} must appear ≥ 2× in {AGENT_PNL_SRC} (struct def + accessor); found {occurrences}"
        );
    }
}

// ────────────────────────────────────────────────────────────────────────
// SG-G3.9.a — solvency 3-tier source-grep witness
// ────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g3_9_a_solvency_three_tier_source_grep() {
    let src = std::fs::read_to_string(AGENT_PNL_SRC).expect("read agent_pnl src");
    for tier in ["Solvent", "NearInsolvent", "Bankrupt"] {
        assert!(
            src.contains(tier),
            "SG-G3.9.a: SolvencyStatus tier {tier:?} must appear in {AGENT_PNL_SRC}"
        );
    }
}

// ────────────────────────────────────────────────────────────────────────
// SG-G3.9.b — canonical state index reads source-grep witness
// ────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g3_9_b_canonical_state_index_reads_source_grep() {
    let src = std::fs::read_to_string(AGENT_PNL_SRC).expect("read agent_pnl src");
    for index in [
        "balances_t",
        "stakes_t",
        "claims_t",
        "reputations_t",
        "conditional_share_balances_t",
        "cpmm_pools_t",
        "lp_share_balances_t",
        "node_positions_t",
    ] {
        assert!(
            src.contains(index),
            "SG-G3.9.b: canonical state index {index:?} must appear in {AGENT_PNL_SRC}"
        );
    }
}

// ────────────────────────────────────────────────────────────────────────
// SG-G3.9.c — no-f64 lint (CLAUDE.md §13 economy laws)
// ────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g3_9_c_no_f64_in_money_path() {
    let src = std::fs::read_to_string(AGENT_PNL_SRC).expect("read agent_pnl src");
    for forbidden in [": f64", ": f32", "as f64", "as f32", " f64::", " f32::"] {
        assert!(
            !src.contains(forbidden),
            "SG-G3.9.c: forbidden token {forbidden:?} found in {AGENT_PNL_SRC} (CLAUDE.md §13 no-f64-in-money-path)"
        );
    }
}

// ────────────────────────────────────────────────────────────────────────
// SG-G3.9.d — stakes / claims visible as open_positions (Art. III shielding)
// ────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g3_9_d_stakes_and_claims_visible_in_open_positions() {
    let mut q = empty_q();
    let a = agent("Agent_0");
    q.economic_state_t.stakes_t.0.insert(
        TxId("worktx-A".into()),
        StakeEntry {
            amount: MicroCoin::from_micro_units(50_000),
            staker: a.clone(),
            task_id: TaskId("task-A".into()),
        },
    );
    q.economic_state_t.claims_t.0.insert(
        TxId("claim-A".into()),
        ClaimEntry {
            amount: MicroCoin::from_micro_units(30_000),
            claimant: a.clone(),
            task_id: TaskId("task-A".into()),
            status: ClaimStatus::Open,
            ..Default::default()
        },
    );
    let view = compute_agent_pnl(&q, &a, 1_000_000);
    let stake_count = view
        .open_positions
        .iter()
        .filter(|p| matches!(p, OpenPosition::Stake { .. }))
        .count();
    let claim_count = view
        .open_positions
        .iter()
        .filter(|p| matches!(p, OpenPosition::Claim { .. }))
        .count();
    assert_eq!(stake_count, 1, "SG-G3.9.d: 1 stake visible");
    assert_eq!(claim_count, 1, "SG-G3.9.d: 1 open claim visible");
    assert_eq!(
        view.unrealized_pnl, 0,
        "stakes/claims have neutral PnL contribution (face = MTM)"
    );
}
