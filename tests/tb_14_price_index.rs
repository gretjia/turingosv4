//! TB-14 Atom 2 — SG-14.1 + SG-14.2 explicit witness suite for
//! `compute_price_index`.
//!
//! TRACE_MATRIX TB-14 SG-14.1 + SG-14.2 (charter §6 ship-gates table).
//! These two ship gates are the named integration-test targets per
//! `handover/tracer_bullets/TB-14_charter_2026-05-03.md` §6:
//!
//!   SG-14.1  PriceIndex computes expected YES/NO probabilities.
//!   SG-14.2  No-liquidity node has price=None.
//!
//! Defense-in-depth tests on the same algorithm live as inline `#[cfg(test)]`
//! in `src/state/price_index.rs` (FR-14.1..3, determinism, rational equality,
//! `dominates_by`). The integration-level tests in this file map the named
//! ship gates to discoverable test names.

use turingosv4::economy::money::MicroCoin;
use turingosv4::state::q_state::AgentId;
use turingosv4::state::typed_tx::{EventId, NodePosition, PositionKind, PositionSide};
use turingosv4::state::{compute_price_index, EconomicState, RationalPrice, TaskId, TxId};

fn make_position(
    position_id: &str,
    node_id: &str,
    task_id: &str,
    owner: &str,
    side: PositionSide,
    kind: PositionKind,
    amount_micro: i64,
) -> NodePosition {
    NodePosition {
        position_id: TxId(position_id.into()),
        node_id: TxId(node_id.into()),
        task_id: TaskId(task_id.into()),
        owner: AgentId(owner.into()),
        side,
        kind,
        amount: MicroCoin::from_micro_units(amount_micro),
        source_tx: TxId(position_id.into()),
        opened_at_round: 1,
    }
}

fn econ_with_positions(positions: Vec<NodePosition>) -> EconomicState {
    let mut econ = EconomicState::default();
    for p in positions {
        econ.node_positions_t.0.insert(p.position_id.clone(), p);
    }
    econ
}

/// SG-14.1 — price_yes / price_no follow long/(long+short) and short/(long+short).
#[test]
fn sg_14_1_price_index_computes_yes_no_probabilities() {
    // 600k Long + 400k Short → price_yes = 0.60, price_no = 0.40.
    let econ = econ_with_positions(vec![
        make_position(
            "p_long",
            "node_main",
            "task_main",
            "agent_a",
            PositionSide::Long,
            PositionKind::FirstLong,
            600_000,
        ),
        make_position(
            "p_short",
            "node_main",
            "task_main",
            "agent_b",
            PositionSide::Short,
            PositionKind::ChallengeShort,
            400_000,
        ),
    ]);

    let idx = compute_price_index(&econ);
    let entry = idx
        .get(&TxId("node_main".into()))
        .expect("node_main present");

    // FR-14.1: price_yes = long / (long + short) = 600_000 / 1_000_000.
    assert_eq!(
        entry.price_yes,
        Some(RationalPrice {
            numerator: 600_000,
            denominator: 1_000_000,
        }),
        "SG-14.1: price_yes must equal long / (long + short)"
    );
    // FR-14.2: price_no = short / (long + short) = 400_000 / 1_000_000.
    assert_eq!(
        entry.price_no,
        Some(RationalPrice {
            numerator: 400_000,
            denominator: 1_000_000,
        }),
        "SG-14.1: price_no must equal short / (long + short)"
    );
    // Non-trivial integer-rational invariant: price_yes.num + price_no.num
    // == denominator (no decimal float used anywhere).
    let py = entry.price_yes.expect("price_yes present");
    let pn = entry.price_no.expect("price_no present");
    assert_eq!(py.numerator + pn.numerator, py.denominator);
    // task_id and event_id wired through correctly.
    assert_eq!(entry.task_id, TaskId("task_main".into()));
    assert_eq!(entry.event_id, EventId(TaskId("task_main".into())));
    // Coin holdings: long_interest + short_interest == liquidity_depth.
    assert_eq!(entry.long_interest, MicroCoin::from_micro_units(600_000));
    assert_eq!(entry.short_interest, MicroCoin::from_micro_units(400_000));
    assert_eq!(
        entry.liquidity_depth,
        MicroCoin::from_micro_units(1_000_000)
    );
}

/// SG-14.2 — zero-liquidity node yields `price_yes == None` and `price_no == None`.
#[test]
fn sg_14_2_no_liquidity_node_has_price_none() {
    // Single zero-amount Long position → entry exists but prices are None.
    let econ = econ_with_positions(vec![make_position(
        "p_zero",
        "node_zero",
        "task_zero",
        "agent_z",
        PositionSide::Long,
        PositionKind::FirstLong,
        0,
    )]);

    let idx = compute_price_index(&econ);
    let entry = idx
        .get(&TxId("node_zero".into()))
        .expect("zero-amount node still receives an index entry");

    // FR-14.3: zero liquidity → both prices are None.
    assert_eq!(
        entry.price_yes, None,
        "SG-14.2: zero-liquidity node MUST have price_yes == None (FR-14.3)"
    );
    assert_eq!(
        entry.price_no, None,
        "SG-14.2: zero-liquidity node MUST have price_no == None (FR-14.3)"
    );
    assert_eq!(entry.liquidity_depth, MicroCoin::zero());
}

/// SG-14.2 (boundary): empty `node_positions_t` yields empty index.
#[test]
fn sg_14_2_empty_state_yields_empty_index() {
    let econ = EconomicState::default();
    let idx = compute_price_index(&econ);
    assert!(
        idx.is_empty(),
        "SG-14.2 (boundary): empty node_positions_t MUST yield empty PriceIndex"
    );
}

/// Determinism: per Art.0.2, `compute_price_index` is replay-deterministic.
/// Repeated calls on identical input must return identical output.
#[test]
fn compute_price_index_is_replay_deterministic() {
    let econ = econ_with_positions(vec![
        make_position(
            "p1",
            "node_a",
            "task_x",
            "agent_1",
            PositionSide::Long,
            PositionKind::FirstLong,
            123_456,
        ),
        make_position(
            "p2",
            "node_b",
            "task_y",
            "agent_2",
            PositionSide::Short,
            PositionKind::ChallengeShort,
            789_012,
        ),
        make_position(
            "p3",
            "node_a",
            "task_x",
            "agent_3",
            PositionSide::Short,
            PositionKind::ChallengeShort,
            55_555,
        ),
    ]);

    let first = compute_price_index(&econ);
    for _ in 0..10 {
        assert_eq!(
            compute_price_index(&econ),
            first,
            "compute_price_index must be replay-deterministic (Art.0.2)"
        );
    }
}

/// Multiple distinct nodes are aggregated independently.
#[test]
fn distinct_nodes_aggregated_independently() {
    let econ = econ_with_positions(vec![
        make_position(
            "p1",
            "node_a",
            "task_a",
            "agent_1",
            PositionSide::Long,
            PositionKind::FirstLong,
            100_000,
        ),
        make_position(
            "p2",
            "node_b",
            "task_b",
            "agent_2",
            PositionSide::Long,
            PositionKind::FirstLong,
            200_000,
        ),
        make_position(
            "p3",
            "node_a",
            "task_a",
            "agent_3",
            PositionSide::Short,
            PositionKind::ChallengeShort,
            50_000,
        ),
    ]);

    let idx = compute_price_index(&econ);
    assert_eq!(idx.len(), 2, "expected 2 distinct nodes");

    let node_a = idx.get(&TxId("node_a".into())).expect("node_a present");
    assert_eq!(
        node_a.price_yes,
        Some(RationalPrice {
            numerator: 100_000,
            denominator: 150_000,
        })
    );

    let node_b = idx.get(&TxId("node_b".into())).expect("node_b present");
    // node_b: 200k Long + 0 Short → price_yes = 200k / 200k = 1.
    assert_eq!(
        node_b.price_yes,
        Some(RationalPrice {
            numerator: 200_000,
            denominator: 200_000,
        })
    );
    assert_eq!(
        node_b.price_no,
        Some(RationalPrice {
            numerator: 0,
            denominator: 200_000,
        })
    );
}
