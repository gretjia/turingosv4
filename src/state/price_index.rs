//! TB-14 Atom 2 — PriceIndex v0 derived view.
//!
//! TRACE_MATRIX FC3-N42 (architect 2026-05-03 ruling §5.1 + §5.2 + §5.4 +
//! charter §1 goal): pure deterministic function over canonical
//! `EconomicState` that derives `NodeMarketEntry` per `TxId` from
//! `node_positions_t` (TB-12 substrate) plus `conditional_share_balances_t`
//! (TB-13 substrate). **Price is signal, not truth** (architect §5.1):
//! the derived view is read-only broadcast input to the scheduler mask
//! (FR-14.5 / FR-14.6) and dashboard render (SG-14.6); it MUST NOT
//! influence predicate gates (CR-14.1 / halt-trigger #1) or L4 / L4.E
//! decision (CR-14.2 / halt-trigger #2).
//!
//! All arithmetic is integer-rational (`u128` numerator + denominator).
//! Decimal-float types are forbidden in this module per charter §5
//! Forbidden list and halt-trigger #4. Replay-deterministic per
//! Art.0.2: no env input, no clock, no randomness.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::economy::money::MicroCoin;
use crate::state::q_state::{AgentId, EconomicState, ShareSidePair};
use crate::state::typed_tx::{EventId, NodePosition, PositionSide, ShareAmount};
use crate::state::{TaskId, TxId};

// ─────────────────────────────────────────────────────────────────────────
// RationalPrice — architect §5.2 verbatim shape
// ─────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX TB-14 Atom 2 (architect §5.2 verbatim): integer-rational
/// price representation. `numerator / denominator` ∈ \[0, 1\] when
/// constructed by `compute_price_index` (architect FR-14.1 + FR-14.2). All
/// comparisons use cross-multiplication; no division until the dashboard
/// render layer (SG-14.6) where it is purely cosmetic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RationalPrice {
    pub numerator: u128,
    pub denominator: u128,
}

impl RationalPrice {
    /// TRACE_MATRIX FC3-N42 (architect §5.5 SG-14.x mask-margin gate; helper
    /// for FC2-N28 `compute_mask_set` in Atom 3): cross-multiplication
    /// dominance predicate.
    ///
    /// True iff `self - other >= margin`, computed by cross-multiplication
    /// to avoid division. Used by Atom 3's `compute_mask_set` to enforce
    /// the price-margin gate (FR-14.5 / SG-14.x). Defensive: returns
    /// `false` on any zero denominator (`compute_price_index` never
    /// produces a `RationalPrice` with zero denominator — that case is
    /// `Option::None` per FR-14.3 / halt-trigger #5 — but defense-in-depth
    /// is cheap).
    pub fn dominates_by(&self, other: &RationalPrice, margin: &RationalPrice) -> bool {
        if self.denominator == 0 || other.denominator == 0 || margin.denominator == 0 {
            return false;
        }
        // Goal: self - other >= margin
        //   (self.n * other.d - other.n * self.d) / (self.d * other.d)
        //       >= margin.n / margin.d
        // Cross-multiply by (self.d * other.d * margin.d) > 0:
        //   (self.n * other.d - other.n * self.d) * margin.d
        //       >= margin.n * (self.d * other.d)
        let self_d = self.denominator;
        let other_d = other.denominator;
        let cross_diff = self
            .numerator
            .saturating_mul(other_d)
            .saturating_sub(other.numerator.saturating_mul(self_d));
        let lhs = cross_diff.saturating_mul(margin.denominator);
        let rhs = margin
            .numerator
            .saturating_mul(self_d)
            .saturating_mul(other_d);
        lhs >= rhs
    }
}

// ─────────────────────────────────────────────────────────────────────────
// NodeMarketEntry — architect §5.2 verbatim shape (10 fields)
// ─────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX TB-14 Atom 2 (architect §5.2 verbatim): per-node market
/// signal entry. **Derived view** populated by `compute_price_index`;
/// never stored as canonical state (architect §5.1: "price is signal,
/// not truth"; charter §7 auto-resolution A: "no second source-of-truth").
///
/// Field semantics:
/// - `node_id` — the `TxId` of the WorkTx attempt-node these positions reference
/// - `task_id` — the `TaskId` (Q-derived from any underlying `NodePosition`)
/// - `event_id` — `EventId(task_id)` (TB-13: 1:1 with TaskId per `typed_tx.rs:1075`)
/// - `long_interest` / `short_interest` — sum of `NodePosition.amount` per side
/// - `yes_share_depth` / `no_share_depth` — sum of `ConditionalShareBalances` for `event_id`
/// - `price_yes` / `price_no` — `Option<RationalPrice>`; `None` iff zero liquidity (FR-14.3)
/// - `liquidity_depth` — `long_interest + short_interest` (saturating)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct NodeMarketEntry {
    pub node_id: TxId,
    pub task_id: TaskId,
    pub event_id: EventId,
    pub long_interest: MicroCoin,
    pub short_interest: MicroCoin,
    pub yes_share_depth: ShareAmount,
    pub no_share_depth: ShareAmount,
    pub price_yes: Option<RationalPrice>,
    pub price_no: Option<RationalPrice>,
    pub liquidity_depth: MicroCoin,
}

// ─────────────────────────────────────────────────────────────────────────
// compute_price_index — pure fn over EconomicState
// ─────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX TB-14 Atom 2 (FC3-N42; architect §5.1 + charter §3 Atom 2):
/// derive the per-node `PriceIndex` from `EconomicState.node_positions_t`
/// (long / short interest aggregation; FR-14.1 / FR-14.2) and
/// `conditional_share_balances_t` (yes / no share depth aggregation per
/// `event_id`).
///
/// **Replay-deterministic** (Art.0.2): pure over the canonical state
/// vector; no env / clock / RNG. Iteration order is `BTreeMap` order on
/// `TxId`, which is lexicographic on the inner `String`.
///
/// **No predicate side-effect** (CR-14.1 / halt-trigger #1): this is a
/// read-only derivation; the sequencer never reads its result during
/// `dispatch_transition` (predicate gate at `sequencer.rs:516-558`).
///
/// **Empty / zero-stake → None** (FR-14.3 / halt-trigger #5): a node with
/// zero long AND zero short interest yields
/// `price_yes == None && price_no == None`. Rationale: division-by-zero
/// avoidance and architect §5.7 halt trigger 5.
pub fn compute_price_index(econ: &EconomicState) -> BTreeMap<TxId, NodeMarketEntry> {
    // Pass 1: group NodePositions by node_id; collect (task_id, long_micro, short_micro).
    let mut groups: BTreeMap<TxId, (TaskId, u128, u128)> = BTreeMap::new();
    for position in econ.node_positions_t.0.values() {
        let amount_micro = position.amount.micro_units();
        let amount_u128 = if amount_micro < 0 {
            0u128
        } else {
            amount_micro as u128
        };
        let entry = groups
            .entry(position.node_id.clone())
            .or_insert_with(|| (position.task_id.clone(), 0u128, 0u128));
        match position.side {
            PositionSide::Long => entry.1 = entry.1.saturating_add(amount_u128),
            PositionSide::Short => entry.2 = entry.2.saturating_add(amount_u128),
        }
    }

    // Pass 2: per node, derive NodeMarketEntry.
    let mut out: BTreeMap<TxId, NodeMarketEntry> = BTreeMap::new();
    for (node_id, (task_id, long_micro, short_micro)) in groups.into_iter() {
        let total_micro = long_micro.saturating_add(short_micro);
        let event_id = EventId(task_id.clone());

        let to_micro = |u: u128| -> MicroCoin {
            // Saturating cast u128 → i64 (positive values only; cap at i64::MAX).
            let capped = if u > i64::MAX as u128 { i64::MAX } else { u as i64 };
            MicroCoin::from_micro_units(capped)
        };

        let (price_yes, price_no) = if total_micro == 0 {
            (None, None)
        } else {
            (
                Some(RationalPrice {
                    numerator: long_micro,
                    denominator: total_micro,
                }),
                Some(RationalPrice {
                    numerator: short_micro,
                    denominator: total_micro,
                }),
            )
        };

        // yes_share_depth / no_share_depth: sum across all owners' balances
        // for this event_id. The conditional_share_balances_t shape is
        // `BTreeMap<AgentId, BTreeMap<EventId, ShareSidePair>>`.
        let mut yes_share_total: u128 = 0;
        let mut no_share_total: u128 = 0;
        for owner_map in econ.conditional_share_balances_t.0.values() {
            if let Some(pair) = owner_map.get(&event_id) {
                yes_share_total = yes_share_total.saturating_add(pair.yes.units);
                no_share_total = no_share_total.saturating_add(pair.no.units);
            }
        }

        out.insert(
            node_id.clone(),
            NodeMarketEntry {
                node_id,
                task_id,
                event_id,
                long_interest: to_micro(long_micro),
                short_interest: to_micro(short_micro),
                yes_share_depth: ShareAmount::from_units(yes_share_total),
                no_share_depth: ShareAmount::from_units(no_share_total),
                price_yes,
                price_no,
                liquidity_depth: to_micro(total_micro),
            },
        );
    }

    out
}

// ─────────────────────────────────────────────────────────────────────────
// Inline unit tests — pure-fn coverage of FR-14.1..3 + determinism +
// rational-equality invariant. The decimal-float fence test lives in
// `tests/tb_14_halt_triggers.rs` (halt-trigger #4); this module never
// reads its own source.
// ─────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::typed_tx::PositionKind;
    use std::collections::BTreeMap as Map;

    fn micro(units: i64) -> MicroCoin {
        MicroCoin::from_micro_units(units)
    }

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
            amount: micro(amount_micro),
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

    #[test]
    fn empty_state_yields_empty_index() {
        let econ = EconomicState::default();
        let idx = compute_price_index(&econ);
        assert!(idx.is_empty(), "empty positions → empty PriceIndex");
    }

    #[test]
    fn single_long_position_yields_price_yes_full() {
        // FR-14.1: price_yes = long / (long + short).
        let econ = econ_with_positions(vec![make_position(
            "p1",
            "n1",
            "t1",
            "a1",
            PositionSide::Long,
            PositionKind::FirstLong,
            500_000,
        )]);
        let idx = compute_price_index(&econ);
        let entry = idx.get(&TxId("n1".into())).expect("n1 present");
        assert_eq!(
            entry.price_yes,
            Some(RationalPrice {
                numerator: 500_000,
                denominator: 500_000
            })
        );
        assert_eq!(
            entry.price_no,
            Some(RationalPrice {
                numerator: 0,
                denominator: 500_000
            })
        );
        assert_eq!(entry.long_interest, micro(500_000));
        assert_eq!(entry.short_interest, MicroCoin::zero());
        assert_eq!(entry.liquidity_depth, micro(500_000));
        assert_eq!(entry.task_id, TaskId("t1".into()));
        assert_eq!(entry.event_id, EventId(TaskId("t1".into())));
    }

    #[test]
    fn single_short_position_yields_price_no_full() {
        // FR-14.2: price_no = short / (long + short).
        let econ = econ_with_positions(vec![make_position(
            "p1",
            "n1",
            "t1",
            "a1",
            PositionSide::Short,
            PositionKind::ChallengeShort,
            300_000,
        )]);
        let idx = compute_price_index(&econ);
        let entry = idx.get(&TxId("n1".into())).expect("n1 present");
        assert_eq!(
            entry.price_no,
            Some(RationalPrice {
                numerator: 300_000,
                denominator: 300_000
            })
        );
        assert_eq!(
            entry.price_yes,
            Some(RationalPrice {
                numerator: 0,
                denominator: 300_000
            })
        );
    }

    #[test]
    fn equal_long_short_yields_half_each() {
        let econ = econ_with_positions(vec![
            make_position(
                "p1",
                "n1",
                "t1",
                "a1",
                PositionSide::Long,
                PositionKind::FirstLong,
                400_000,
            ),
            make_position(
                "p2",
                "n1",
                "t1",
                "a2",
                PositionSide::Short,
                PositionKind::ChallengeShort,
                400_000,
            ),
        ]);
        let idx = compute_price_index(&econ);
        let entry = idx.get(&TxId("n1".into())).expect("n1 present");
        assert_eq!(
            entry.price_yes,
            Some(RationalPrice {
                numerator: 400_000,
                denominator: 800_000
            })
        );
        assert_eq!(
            entry.price_no,
            Some(RationalPrice {
                numerator: 400_000,
                denominator: 800_000
            })
        );
    }

    #[test]
    fn rational_equality_invariant() {
        // For any non-zero-liquidity node:
        //   price_yes.num + price_no.num == price_yes.den == price_no.den.
        let econ = econ_with_positions(vec![
            make_position(
                "p1",
                "n1",
                "t1",
                "a1",
                PositionSide::Long,
                PositionKind::FirstLong,
                700_000,
            ),
            make_position(
                "p2",
                "n1",
                "t1",
                "a2",
                PositionSide::Short,
                PositionKind::ChallengeShort,
                300_000,
            ),
        ]);
        let idx = compute_price_index(&econ);
        let entry = idx.get(&TxId("n1".into())).expect("n1 present");
        let py = entry.price_yes.expect("price_yes present");
        let pn = entry.price_no.expect("price_no present");
        assert_eq!(py.denominator, pn.denominator, "denominators must match");
        assert_eq!(
            py.numerator + pn.numerator,
            py.denominator,
            "rational equality: long_n + short_n == total"
        );
    }

    #[test]
    fn determinism_n_calls_identical() {
        let econ = econ_with_positions(vec![
            make_position(
                "p1",
                "n1",
                "t1",
                "a1",
                PositionSide::Long,
                PositionKind::FirstLong,
                700_000,
            ),
            make_position(
                "p2",
                "n2",
                "t2",
                "a2",
                PositionSide::Short,
                PositionKind::ChallengeShort,
                300_000,
            ),
            make_position(
                "p3",
                "n1",
                "t1",
                "a3",
                PositionSide::Short,
                PositionKind::ChallengeShort,
                100_000,
            ),
        ]);
        let first = compute_price_index(&econ);
        for _ in 0..10 {
            assert_eq!(
                compute_price_index(&econ),
                first,
                "deterministic across calls"
            );
        }
    }

    #[test]
    fn yes_share_depth_aggregates_across_owners() {
        use crate::state::typed_tx::EventId;
        // Two owners hold YES/NO shares for event_id=task_x; depths sum.
        let mut econ = econ_with_positions(vec![make_position(
            "p1",
            "node_x",
            "task_x",
            "a1",
            PositionSide::Long,
            PositionKind::FirstLong,
            100_000,
        )]);
        let event_id = EventId(TaskId("task_x".into()));
        let mut a1_map: Map<EventId, ShareSidePair> = Map::new();
        a1_map.insert(
            event_id.clone(),
            ShareSidePair {
                yes: ShareAmount::from_units(50_000),
                no: ShareAmount::from_units(20_000),
            },
        );
        let mut a2_map: Map<EventId, ShareSidePair> = Map::new();
        a2_map.insert(
            event_id.clone(),
            ShareSidePair {
                yes: ShareAmount::from_units(70_000),
                no: ShareAmount::from_units(30_000),
            },
        );
        econ.conditional_share_balances_t
            .0
            .insert(AgentId("a1".into()), a1_map);
        econ.conditional_share_balances_t
            .0
            .insert(AgentId("a2".into()), a2_map);

        let idx = compute_price_index(&econ);
        let entry = idx.get(&TxId("node_x".into())).expect("node_x present");
        assert_eq!(entry.yes_share_depth, ShareAmount::from_units(120_000));
        assert_eq!(entry.no_share_depth, ShareAmount::from_units(50_000));
    }

    #[test]
    fn rational_dominates_by() {
        // 0.60 - 0.40 = 0.20 >= 0.10 ✓
        let p60 = RationalPrice {
            numerator: 60,
            denominator: 100,
        };
        let p40 = RationalPrice {
            numerator: 40,
            denominator: 100,
        };
        let m10 = RationalPrice {
            numerator: 1,
            denominator: 10,
        };
        assert!(p60.dominates_by(&p40, &m10));
        assert!(!p40.dominates_by(&p60, &m10));
        // 0.60 vs 0.50 by 0.20 → does not dominate.
        let p50 = RationalPrice {
            numerator: 50,
            denominator: 100,
        };
        let m20 = RationalPrice {
            numerator: 1,
            denominator: 5,
        };
        assert!(!p60.dominates_by(&p50, &m20));
        // 0.70 vs 0.50 by 0.20 → dominates.
        let p70 = RationalPrice {
            numerator: 70,
            denominator: 100,
        };
        assert!(p70.dominates_by(&p50, &m20));
        // Defense: zero denominator returns false.
        let zero_den = RationalPrice {
            numerator: 1,
            denominator: 0,
        };
        assert!(!p60.dominates_by(&zero_den, &m10));
        assert!(!zero_den.dominates_by(&p60, &m10));
    }
}
