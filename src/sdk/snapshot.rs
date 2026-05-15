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
///   `compute_price_index(econ)`. Empty when bus runs sequencer-less OR
///   when sequencer is wired but no canonical positions have accumulated.
///   The two cases are distinguished by the `sequencer_wired` field below.
/// - `mask_set` — derived `BTreeSet<TxId>` per `compute_mask_set(...)`.
///   Empty when bus runs sequencer-less OR when canonical edges are empty.
///   Mask is read-view only — masked parents remain in canonical state
///   (CR-14.3 / SG-14.3 / halt-trigger #3).
/// - `sequencer_wired` — TB-14 Atom 6 B′ R2 closure (Gemini R2 Q11
///   architectural-clarity CHALLENGE): explicit two-state disambiguator.
///   `true` means the bus has a wired sequencer AND `q_snapshot()`
///   succeeded AND the canonical-graph builder ran. `price_index` /
///   `mask_set` may still be empty in this case — that is the "running
///   but no canonical positions yet" state. `false` means the bus runs
///   in legacy ledger-only mode (`sequencer == None`) OR the sequencer's
///   `q_snapshot()` failed (lock poisoned). In the `false` case,
///   `price_index` + `mask_set` are always empty by construction.
///   Consumers that need to distinguish "no signal possible" from "no
///   signal yet" read this field; consumers that don't care continue to
///   treat empty maps uniformly (no breaking change for the existing
///   call surface). `#[serde(default)]` for backward-compat with
///   pre-B′-R2 chain snapshots.
/// - `generation`, `tx_count` — bus-level counters, unchanged from TB-3.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniverseSnapshot {
    pub tape: Tape,
    pub price_index: BTreeMap<TxId, NodeMarketEntry>,
    pub mask_set: BTreeSet<TxId>,
    #[serde(default)]
    pub sequencer_wired: bool,
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
            sequencer_wired: false,
            generation: 0,
            tx_count: 0,
        };
        assert!(snap.price_index.is_empty());
        assert!(snap.mask_set.is_empty());
        assert!(!snap.sequencer_wired);
        assert_eq!(snap.generation, 0);
        assert_eq!(snap.tx_count, 0);
    }

    /// TB-14 Atom 6 B′ R2 closure (Gemini R2 Q11): the `sequencer_wired`
    /// field disambiguates "sequencer unavailable" from "sequencer running
    /// but no canonical positions yet". `serde(default)` ensures
    /// pre-B′-R2 serialized snapshots round-trip without explicit field
    /// values (default = false; consistent with the legacy ledger-only
    /// mode that produced those snapshots).
    #[test]
    fn test_snapshot_sequencer_wired_serde_default_false() {
        // Construct a JSON payload WITHOUT the `sequencer_wired` field
        // (mirrors pre-B′-R2 serialized snapshots).
        let json = r#"{
            "tape": {"nodes": {}, "reverse_citations": {}, "time_arrow": []},
            "price_index": {},
            "mask_set": [],
            "generation": 0,
            "tx_count": 0
        }"#;
        let snap: UniverseSnapshot = serde_json::from_str(json).expect("legacy JSON deserialize");
        assert!(
            !snap.sequencer_wired,
            "Q11 closure: serde(default) on sequencer_wired must produce \
             false for legacy snapshots without the field"
        );
    }

    #[test]
    fn test_snapshot_sequencer_wired_distinguishes_two_states() {
        // Two snapshots, both with empty price_index + mask_set, are
        // semantically distinct via sequencer_wired.
        let unavailable = UniverseSnapshot {
            tape: Tape::new(),
            price_index: BTreeMap::new(),
            mask_set: BTreeSet::new(),
            sequencer_wired: false,
            generation: 0,
            tx_count: 0,
        };
        let running_empty = UniverseSnapshot {
            tape: Tape::new(),
            price_index: BTreeMap::new(),
            mask_set: BTreeSet::new(),
            sequencer_wired: true,
            generation: 0,
            tx_count: 0,
        };
        assert_eq!(unavailable.price_index, running_empty.price_index);
        assert_eq!(unavailable.mask_set, running_empty.mask_set);
        assert_ne!(
            unavailable.sequencer_wired, running_empty.sequencer_wired,
            "Q11 closure: same empty maps + opposite sequencer_wired = \
             two distinct semantic states (sequencer unavailable vs \
             running-but-empty); consumers that need to disambiguate \
             read this field"
        );
    }
}
