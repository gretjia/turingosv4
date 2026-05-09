//! TuringOS Constitution Gate — Stage C P-M8 audit views (architect
//! 2026-05-07 ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL §7.9
//! verbatim).
//!
//! # Scope
//!
//! Architect alignment doc §7.9 mandates 3 hardening tests for the new
//! `audit_tape view-shares` / `view-pools` / `view-prices` /
//! `view-positions` pure-projection audit views:
//!
//!   - audit_view_shares_matches_state
//!   - audit_view_pools_matches_state
//!   - dashboard_regenerates_market_view
//!
//! TRACE_FLOWCHART_MATRIX:
//!   - FC1 §6 monetary invariant (views are pure derivations from EconomicState)
//!   - §7.9 architect Polymarket manual

use std::collections::BTreeMap;

use turingosv4::economy::money::MicroCoin;
use turingosv4::runtime::audit_views::{
    view_pools, view_positions, view_prices, view_shares,
};
use turingosv4::state::q_state::{
    AgentId, CpmmPool, EconomicState, LpShareAmount, PoolEventKind, PoolStatus,
    QState, ShareSidePair, TaskId,
};
use turingosv4::state::typed_tx::{EventId, ShareAmount};

/// Build a state with: (a) two owners with conditional shares at one event;
/// (b) a CPMM pool at the same event with backing collateral; (c) no node
/// positions (clean slate for the dashboard regeneration test).
fn build_fixture_state() -> QState {
    let mut q = QState::genesis();
    let event = EventId(TaskId("event-fixture".into()));

    // Owner Alice: 50 YES + 30 NO.
    let mut alice_shares = BTreeMap::new();
    alice_shares.insert(
        event.clone(),
        ShareSidePair {
            yes: ShareAmount::from_units(50),
            no: ShareAmount::from_units(30),
        },
    );
    q.economic_state_t
        .conditional_share_balances_t
        .0
        .insert(AgentId("alice".into()), alice_shares);

    // Owner Bob: 20 YES + 40 NO.
    let mut bob_shares = BTreeMap::new();
    bob_shares.insert(
        event.clone(),
        ShareSidePair {
            yes: ShareAmount::from_units(20),
            no: ShareAmount::from_units(40),
        },
    );
    q.economic_state_t
        .conditional_share_balances_t
        .0
        .insert(AgentId("bob".into()), bob_shares);

    // Pool: 1000 YES + 1000 NO; LP supply 1000.
    q.economic_state_t.cpmm_pools_t.0.insert(
        event.clone(),
        CpmmPool {
            event_id_kind: PoolEventKind::BinaryYesNo,
            pool_yes: ShareAmount::from_units(1000),
            pool_no: ShareAmount::from_units(1000),
            lp_total_shares: LpShareAmount::from_units(1000),
            status: PoolStatus::Active,
        },
    );

    // Collateral covers max(pool_yes, pool_no, owner_yes_sum, owner_no_sum) = 1000.
    q.economic_state_t.conditional_collateral_t.0.insert(
        event,
        MicroCoin::from_micro_units(1000),
    );

    q
}

// ════════════════════════════════════════════════════════════════════════════
// §7.9 P-M8 audit views (3 verbatim names)
// ════════════════════════════════════════════════════════════════════════════

/// §7.9 verbatim — `audit_view_shares_matches_state`.
///
/// Per architect manual §7.9: `audit_tape view-shares` shows owner YES/NO
/// shares. Asserts `view_shares` is a faithful, byte-equal projection of
/// `EconomicState.conditional_share_balances_t`.
#[test]
fn audit_view_shares_matches_state() {
    let q = build_fixture_state();
    let v = view_shares(&q.economic_state_t);

    let event = EventId(TaskId("event-fixture".into()));
    let alice_pair = v
        .holdings
        .get(&AgentId("alice".into()))
        .and_then(|m| m.get(&event))
        .copied()
        .expect("alice has shares at event");
    assert_eq!(alice_pair.yes.units, 50);
    assert_eq!(alice_pair.no.units, 30);

    let bob_pair = v
        .holdings
        .get(&AgentId("bob".into()))
        .and_then(|m| m.get(&event))
        .copied()
        .expect("bob has shares at event");
    assert_eq!(bob_pair.yes.units, 20);
    assert_eq!(bob_pair.no.units, 40);

    // Faithful projection: view holdings equal underlying state.
    assert_eq!(
        v.holdings, q.economic_state_t.conditional_share_balances_t.0,
        "view_shares is byte-equal to conditional_share_balances_t"
    );
}

/// §7.9 verbatim — `audit_view_pools_matches_state`.
///
/// Per architect manual §7.9: `audit_tape view-pools` shows pool reserves
/// + LP shares. Asserts `view_pools` is a faithful projection joining
/// `cpmm_pools_t` with `conditional_collateral_t`.
#[test]
fn audit_view_pools_matches_state() {
    let q = build_fixture_state();
    let v = view_pools(&q.economic_state_t);

    let event = EventId(TaskId("event-fixture".into()));
    let entry = v.pools.get(&event).expect("pool entry exists");
    assert_eq!(entry.pool.pool_yes.units, 1000);
    assert_eq!(entry.pool.pool_no.units, 1000);
    assert_eq!(entry.pool.lp_total_shares.units, 1000);
    assert_eq!(entry.pool.status, PoolStatus::Active);
    assert_eq!(entry.collateral_micro_units, 1000);
    assert_eq!(v.pools.len(), 1, "single pool in fixture");
}

/// §7.9 verbatim — `dashboard_regenerates_market_view`.
///
/// Per architect manual §7.9 + CR-StageC-PM.4 "no dashboard
/// source-of-truth": views regenerate deterministically from chain state.
/// Asserts (a) determinism (same input → bit-equal output across calls),
/// (b) views over an empty state are empty (genesis regenerates as empty
/// dashboard), (c) all 4 views compose into a coherent market snapshot.
#[test]
fn dashboard_regenerates_market_view() {
    let q = build_fixture_state();

    // Determinism: 3 calls → identical outputs.
    let s1 = view_shares(&q.economic_state_t);
    let s2 = view_shares(&q.economic_state_t);
    let s3 = view_shares(&q.economic_state_t);
    assert_eq!(s1, s2);
    assert_eq!(s2, s3);

    let p1 = view_pools(&q.economic_state_t);
    let p2 = view_pools(&q.economic_state_t);
    assert_eq!(p1, p2);

    let pr1 = view_prices(&q.economic_state_t, 100);
    let pr2 = view_prices(&q.economic_state_t, 100);
    assert_eq!(pr1, pr2);

    let pos1 = view_positions(&q.economic_state_t);
    let pos2 = view_positions(&q.economic_state_t);
    assert_eq!(pos1, pos2);

    // Empty-state regeneration: genesis has empty conditional state.
    let q_empty = QState::genesis();
    let empty_shares = view_shares(&q_empty.economic_state_t);
    let empty_pools = view_pools(&q_empty.economic_state_t);
    let empty_positions = view_positions(&q_empty.economic_state_t);
    assert!(empty_shares.holdings.is_empty(), "genesis: no shares");
    assert!(empty_pools.pools.is_empty(), "genesis: no pools");
    assert!(empty_positions.positions.is_empty(), "genesis: no positions");

    // Coherence: pool entry's event_id appears in shares too (Alice + Bob
    // both hold shares at the pool's event in fixture).
    let event = EventId(TaskId("event-fixture".into()));
    assert!(p1.pools.contains_key(&event));
    assert!(s1.holdings.values().all(|m| m.contains_key(&event)));

    // Price quote returns Some for the pool (non-trivial liquidity).
    let pr_entry = pr1.prices.get(&event).expect("pool has price entry");
    assert!(pr_entry.buy_yes_get_total.is_some());
    assert!(pr_entry.buy_no_get_total.is_some());
    // Pool reserves 1000 each ≥ STAGE_C_LOW_LIQUIDITY_THRESHOLD_UNITS=100;
    // no warning expected.
    assert!(!pr_entry.low_liquidity_warning);
}
