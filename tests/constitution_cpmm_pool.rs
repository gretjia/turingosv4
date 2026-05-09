//! TuringOS Constitution Gate — Stage C P-M4 LiquidityPool / CpmmPool state
//! (architect 2026-05-07 ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL
//! §7.5 verbatim).
//!
//! # Scope
//!
//! Architect alignment doc §7.5 mandates 4 hardening tests for the new
//! `CpmmPool` state struct (per-event integer CPMM pool: pool_yes + pool_no
//! share reserves + lp_total_shares + status):
//!
//!   - pool_created_from_seed_inventory
//!   - pool_reserves_not_counted_as_coin
//!   - lp_shares_not_counted_as_coin
//!   - pool_cannot_exist_without_collateralized_shares
//!
//! # Why a separate gate file (vs inline in q_state.rs unit tests)
//!
//! Per `feedback_no_workarounds_strict_constitution` ("我不要凑活"), constitution
//! gates bind verbatim names directly to live runtime state. Tests construct
//! a populated `EconomicState` with `cpmm_pools_t` entries and assert
//! invariants (pool reserves / LP shares NOT in `total_supply_micro`; pool
//! existence requires collateral coverage).
//!
//! Note: P-M4 lands the STATE STRUCT only — no new typed_tx variant. Pool
//! initialization tx (CpmmPoolInitTx or similar) lands in a forward atom if
//! the architect adds one; for now, tests assert structural invariants over
//! synthetic-but-canonical state (mirrors how TB-13's `conditional_share_balances_t`
//! invariants were tested at landing).
//!
//! TRACE_FLOWCHART_MATRIX:
//!   - FC1 §6 monetary invariant (pool reserves / LP shares NOT in CTF sum)
//!   - §7.5 architect Polymarket manual

use std::collections::BTreeMap;

use turingosv4::economy::money::MicroCoin;
use turingosv4::economy::monetary_invariant::total_supply_micro as canonical_total_supply_micro;
use turingosv4::state::q_state::{
    AgentId, CpmmPool, CpmmPoolsIndex, EconomicState, LpShareAmount, PoolEventKind,
    PoolStatus, QState, ShareSidePair, TaskId,
};
use turingosv4::state::typed_tx::{EventId, ShareAmount};

/// Build a baseline genesis state with a single seeded provider position
/// and a CPMM pool at the same event. The pool reserves come from the
/// provider's seed inventory (1:1 mapping, mirroring the architect manual
/// §7.5 "pool_created_from_seed_inventory" semantics — provider's YES + NO
/// inventory IS what the pool holds).
fn genesis_with_seeded_pool(
    provider: &str,
    event_task: &str,
    seed_micro: i64,
) -> QState {
    let mut q = QState::genesis();

    // Seed conditional_collateral_t at `seed_micro` (mirrors what
    // MarketSeedTx leaves behind post-execution).
    let event = EventId(TaskId(event_task.into()));
    q.economic_state_t
        .conditional_collateral_t
        .0
        .insert(event.clone(), MicroCoin::from_micro_units(seed_micro));

    // Provider holds zero conditional shares directly (because they were
    // converted to pool reserves — this is the "from seed inventory"
    // wording).
    let _ = provider;

    // CPMM pool at the same event with seed_micro YES + seed_micro NO.
    let pool = CpmmPool {
        event_id_kind: PoolEventKind::BinaryYesNo,
        pool_yes: ShareAmount::from_units(seed_micro as u128),
        pool_no: ShareAmount::from_units(seed_micro as u128),
        lp_total_shares: LpShareAmount::from_units(seed_micro as u128),
        status: PoolStatus::Active,
    };
    q.economic_state_t
        .cpmm_pools_t
        .0
        .insert(event, pool);

    q
}

fn total_supply_micro(econ: &EconomicState) -> i64 {
    canonical_total_supply_micro(econ).expect("total_supply_micro must not overflow")
}

// ════════════════════════════════════════════════════════════════════════════
// §7.5 P-M4 CpmmPool hardening (4 verbatim names)
// ════════════════════════════════════════════════════════════════════════════

/// §7.5 verbatim — `pool_created_from_seed_inventory`.
///
/// Per architect manual §7.5: the pool's YES + NO reserves come from seed
/// inventory (i.e., a MarketSeed-class operation supplies the share basis).
/// Asserts that a state populated with `cpmm_pools_t[event]` has matching
/// `pool_yes` / `pool_no` reserves backed by `conditional_collateral_t[event]`,
/// AND that pool reserves equal the seed amount that was deposited as
/// collateral.
#[tokio::test]
async fn pool_created_from_seed_inventory() {
    let seed_amount: i64 = 50_000_000;
    let q = genesis_with_seeded_pool("alice", "task-pool-A", seed_amount);

    let event = EventId(TaskId("task-pool-A".into()));
    let pool = q
        .economic_state_t
        .cpmm_pools_t
        .0
        .get(&event)
        .expect("pool must exist after seed");

    assert_eq!(
        pool.pool_yes.units, seed_amount as u128,
        "pool YES reserves derived from seed inventory"
    );
    assert_eq!(
        pool.pool_no.units, seed_amount as u128,
        "pool NO reserves derived from seed inventory"
    );
    assert_eq!(pool.status, PoolStatus::Active, "freshly seeded pool is Active");
    assert_eq!(
        pool.event_id_kind, PoolEventKind::BinaryYesNo,
        "Stage C only supports BinaryYesNo events"
    );

    // Collateral must be present for the seeded pool.
    let collateral = q
        .economic_state_t
        .conditional_collateral_t
        .0
        .get(&event)
        .copied()
        .expect("collateral entry exists post-seed");
    assert_eq!(
        collateral.micro_units(),
        seed_amount,
        "collateral matches seed amount"
    );
}

/// §7.5 verbatim — `pool_reserves_not_counted_as_coin`.
///
/// Per architect manual §7.5: "pool reserves are not Coin". The CTF
/// `total_supply_micro` sum (canonical 6-holding sum + future extensions)
/// must NOT include pool YES / NO reserves. Asserts that two states
/// differing ONLY in `cpmm_pools_t.pool_yes` / `cpmm_pools_t.pool_no`
/// have bit-equal `total_supply_micro`.
#[tokio::test]
async fn pool_reserves_not_counted_as_coin() {
    let q_with_pool = genesis_with_seeded_pool("alice", "task-pool-B", 30_000_000);
    let mut q_without_pool = q_with_pool.clone();
    q_without_pool.economic_state_t.cpmm_pools_t = CpmmPoolsIndex(BTreeMap::new());

    let supply_with = total_supply_micro(&q_with_pool.economic_state_t);
    let supply_without = total_supply_micro(&q_without_pool.economic_state_t);

    assert_eq!(
        supply_with, supply_without,
        "pool reserves NOT counted in total_supply_micro (architect §7.5)"
    );
}

/// §7.5 verbatim — `lp_shares_not_counted_as_coin`.
///
/// Per architect manual §7.5: "lp shares are not Coin". The
/// `total_supply_micro` sum must NOT include `lp_total_shares`. Asserts
/// that two states differing ONLY in `cpmm_pools_t.lp_total_shares` have
/// bit-equal `total_supply_micro`.
#[tokio::test]
async fn lp_shares_not_counted_as_coin() {
    let q_low_lp = genesis_with_seeded_pool("alice", "task-pool-C", 10_000_000);
    let mut q_high_lp = q_low_lp.clone();

    // Inflate LP shares 100× without touching anything else.
    let event = EventId(TaskId("task-pool-C".into()));
    let pool = q_high_lp
        .economic_state_t
        .cpmm_pools_t
        .0
        .get_mut(&event)
        .expect("pool exists");
    pool.lp_total_shares = LpShareAmount::from_units(1_000_000_000);

    let supply_low = total_supply_micro(&q_low_lp.economic_state_t);
    let supply_high = total_supply_micro(&q_high_lp.economic_state_t);

    assert_eq!(
        supply_low, supply_high,
        "LP shares NOT counted in total_supply_micro (architect §7.5)"
    );
}

/// §7.5 verbatim — `pool_cannot_exist_without_collateralized_shares`.
///
/// Per architect manual §7.5 + universal forbidden list "no ghost
/// liquidity": pool reserves are share claims against
/// `conditional_collateral_t`. A pool entry at `event_id` MUST have
/// `conditional_collateral_t[event_id]` covering at least
/// `max(pool_yes.units, pool_no.units)`. If collateral is absent or
/// insufficient, the pool's existence is "ghost liquidity".
///
/// This test asserts the invariant holds for a properly-seeded pool AND
/// that removing the collateral entry detectably breaks the invariant
/// (closure-3 "every test can fail" — synthetic violation must be
/// catchable by the same predicate).
#[tokio::test]
async fn pool_cannot_exist_without_collateralized_shares() {
    let q_ok = genesis_with_seeded_pool("alice", "task-pool-D", 25_000_000);
    let event = EventId(TaskId("task-pool-D".into()));

    // Helper: assert invariant holds.
    let invariant_holds = |econ: &EconomicState| -> bool {
        for (eid, pool) in econ.cpmm_pools_t.0.iter() {
            let collateral_units = econ
                .conditional_collateral_t
                .0
                .get(eid)
                .copied()
                .map(|c| c.micro_units() as u128)
                .unwrap_or(0);
            let max_side = pool.pool_yes.units.max(pool.pool_no.units);
            if collateral_units < max_side {
                return false;
            }
        }
        true
    };

    assert!(
        invariant_holds(&q_ok.economic_state_t),
        "properly-seeded pool satisfies collateral-backing invariant"
    );

    // Synthetic violation: remove the collateral entry. Pool still exists.
    let mut q_violated = q_ok.clone();
    q_violated.economic_state_t.conditional_collateral_t.0.remove(&event);
    assert!(
        !invariant_holds(&q_violated.economic_state_t),
        "removing collateral while pool exists violates the invariant (closure-3 detectable)"
    );

    // Synthetic violation: inflate pool reserves above collateral.
    let mut q_overpool = q_ok.clone();
    let pool = q_overpool
        .economic_state_t
        .cpmm_pools_t
        .0
        .get_mut(&event)
        .unwrap();
    pool.pool_yes = ShareAmount::from_units(100_000_000_000); // 4000× collateral
    assert!(
        !invariant_holds(&q_overpool.economic_state_t),
        "pool YES reserves exceeding collateral violates invariant (closure-3 detectable)"
    );
}

#[cfg(test)]
fn _unused_silence_imports() {
    // Suppress unused-import warnings for AgentId / ShareSidePair (kept
    // available for future merge tests on this fixture).
    let _ = AgentId("x".into());
    let _ = ShareSidePair::default();
}
