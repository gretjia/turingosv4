// Tier 2: Immutable universe snapshot — agents read, never mutate
// Constitutional basis: Art. III.3 (decorrelation via independent snapshots)
//
// TB-14 Atom 6 (2026-05-03 closing OBS_TB_12_LEGACY_CPMM_QUARANTINE):
// Legacy decimal-float `MarketSnapshot` + `UniverseSnapshot.markets`
// HashMap CPMM read-view was excised together with `prediction_market.rs`.
// The snapshot now carries integer-rational `price_index` + `mask_set`
// derived from canonical `EconomicState` via `state::compute_price_index`
// + `state::compute_mask_set`. Pricing is signal, not truth.
//
// Dead post-TB-9-collapse `balances: HashMap<String, f64>` and
// `portfolios: HashMap<String, HashMap<NodeId, (f64, f64, f64)>>` were
// also retired in this atom — bus.snapshot already populated both with
// empty HashMaps (no live values flowed through them). Removal is purely
// additive cleanup that closes the f64 surface in this file under the
// G-14.11 "no f64 in TB-14 module surface" ship gate.

use crate::ledger::Tape;
use crate::state::{NodeMarketEntry, TxId};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// Complete frozen state of the universe.
/// Agents receive this as read-only input — they cannot mutate it.
/// Art. III.3: each agent sees the same snapshot, maintaining decorrelation.
///
/// TRACE_MATRIX TB-14 Atom 6 (FC2-N28 + FC3-N42; architect §5.1 + charter
/// §3 Atom 6): the snapshot's price-signal surface.
///
/// Field semantics:
/// - `tape` — the current `Tape` (DAG of attempt nodes); read-only mirror.
/// - `price_index` — derived `BTreeMap<TxId, NodeMarketEntry>` per
///   `compute_price_index(econ)`. Empty when bus runs sequencer-less.
/// - `mask_set` — derived `BTreeSet<TxId>` per `compute_mask_set(...)`.
///   Empty when bus runs sequencer-less. Mask is read-view only — masked
///   parents remain in `tape.nodes()` (CR-14.3 / SG-14.3 / halt-trigger #3).
/// - `generation`, `tx_count` — bus-level counters, unchanged from TB-3.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniverseSnapshot {
    pub tape: Tape,
    pub price_index: BTreeMap<TxId, NodeMarketEntry>,
    pub mask_set: BTreeSet<TxId>,
    pub generation: u32,
    pub tx_count: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snapshot_default_empty_signal_surface() {
        // TB-14 Atom 6: a freshly-constructed snapshot has empty
        // price_index + mask_set; consumers (evaluator / dashboard) must
        // tolerate this as "no signal yet" without crashing.
        let snap = UniverseSnapshot {
            tape: Tape::new(),
            price_index: BTreeMap::new(),
            mask_set: BTreeSet::new(),
            generation: 0,
            tx_count: 0,
        };
        assert!(snap.price_index.is_empty());
        assert!(snap.mask_set.is_empty());
        assert_eq!(snap.generation, 0);
        assert_eq!(snap.tx_count, 0);
    }
}
