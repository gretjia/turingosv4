//! S3 — Boltzmann scheduler observe-only selection trace.
//!
//! Brings the dead-but-tested integer-rational scheduler
//! [`boltzmann_select_parent_v2`](crate::sdk::actor::boltzmann_select_parent_v2)
//! ONLINE as a tape-anchored OBSERVE-ONLY selection record. It runs the full
//! live derivation chain — `compute_price_index` → `compute_mask_set` →
//! `boltzmann_select_parent_v2` — over a borrowed canonical
//! [`EconomicState`](crate::state::q_state::EconomicState) and persists the
//! recommendation to CAS via [`CasStore::put`] (the L4 anchor; fail-closed on
//! `refs/chaintape/cas`). It mirrors the live derivation in `src/bus.rs:560-563`
//! but, unlike `bus.rs`, it actually CALLS the scheduler and records the result.
//!
//! **Observe-only discipline** (mirrors `src/runtime/agent_scheduler.rs`):
//! this module is a read-view recommendation record. "Price is signal, not
//! truth." It NEVER mutates `QState`/`EconomicState` (econ is borrowed `&`),
//! NEVER advances a head, NEVER changes sequencer admission or L4/L4.E
//! predicates, and is NEVER a source of truth. The recorded trace is
//! reconstructable from CAS alone (Art.0.2 tape-canonical); there is no
//! `std::fs::write`, no filesystem side-store.
//!
//! **Integer-rational only**: every persisted numeric field is an integer
//! (`u128` numerator/denominator, `i64`/`u64` policy fields, `u64` seed). No
//! `f64`/`f32` ever touches this path — the `boltzmann_softmax_select_parent`
//! float variant in `sdk::actor` is deliberately NOT used here.
//!
//! TRACE_MATRIX FC2-N29 (economic derived view) + FC1-N7 (scheduler read-view):
//! mirrors the `agent_scheduler.rs` module-family role.

use serde::{Deserialize, Serialize};

use crate::bottom_white::cas::schema::{Cid, ObjectType};
use crate::bottom_white::cas::store::{CasError, CasStore};
use crate::bottom_white::ledger::transition_ledger::canonical_encode;
use crate::sdk::actor::boltzmann_select_parent_v2;
use crate::state::price_index::{
    compute_mask_set, compute_price_index, BoltzmannMaskPolicy, CanonicalNodeGraph, RationalPrice,
};
use crate::state::q_state::EconomicState;
use crate::state::TxId;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

/// TRACE_MATRIX FC2-N29 + FC1-N7: free-form CAS schema id for the observe-only
/// Boltzmann selection trace (mirrors `SCHEDULER_DECISION_TRACE_SCHEMA_ID`).
pub const BOLTZMANN_SELECTION_TRACE_SCHEMA_ID: &str = "s3.boltzmann_selection_trace.v1";

/// TRACE_MATRIX FC2-N29 + FC1-N7: canonical published SHA-256 of constitution
/// Flowchart 2 (Boot + full architecture) per
/// `handover/alignment/TRACE_FLOWCHART_MATRIX.md` §2. This observe-only
/// economic-derived view anchors to the FC2 architectural contract; recording
/// the hash binds the trace to the flowchart it materializes without reading
/// the filesystem (machine-independent, replay-deterministic).
pub const BOLTZMANN_TRACE_FC2_CONSTITUTION_HASH: &str =
    "6a4bc9195bafd55bde968fd445cdd2926d6906a7f6a2b38071d4774a7f0de333";

/// TRACE_MATRIX FC2-N29 + FC1-N7: which branch of
/// `boltzmann_select_parent_v2` produced the recommendation. Recomputed
/// deterministically by reproducing the same seeded RNG sequence (honest
/// branch attribution, no guessing).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelectionBranch {
    /// TRACE_MATRIX FC2-N29 + FC1-N7: candidate set was empty after the
    /// price/mask filter; the scheduler returned `None`.
    EmptyCandidates,
    /// TRACE_MATRIX FC2-N29 + FC1-N7: the epsilon-greedy exploration roll
    /// fired (`r < epsilon_num`); a uniform-random candidate was picked.
    EpsilonExploration,
    /// TRACE_MATRIX FC2-N29 + FC1-N7: the argmax-by-`price_yes` exploitation
    /// branch (no exploration roll, or roll did not fire).
    ArgmaxExploit,
}

/// TRACE_MATRIX FC2-N29 + FC1-N7: integer-only snapshot of the
/// `BoltzmannMaskPolicy` fields that drive the selection. Mirrors the seven
/// integer-rational policy fields; no `f64` is representable here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoltzmannPolicySnapshot {
    /// TRACE_MATRIX FC2-N29 + FC1-N7: epsilon-exploration probability numerator.
    pub epsilon_exploration_num: u64,
    /// TRACE_MATRIX FC2-N29 + FC1-N7: epsilon-exploration probability denominator.
    pub epsilon_exploration_den: u64,
    /// TRACE_MATRIX FC2-N29 + FC1-N7: argmax-tiebreak temperature numerator.
    pub beta_num: i64,
    /// TRACE_MATRIX FC2-N29 + FC1-N7: argmax-tiebreak temperature denominator.
    pub beta_den: i64,
    /// TRACE_MATRIX FC2-N29 + FC1-N7: min-liquidity floor (micro-coin integer units).
    pub min_liquidity_micro: i64,
    /// TRACE_MATRIX FC2-N29 + FC1-N7: price-margin dominance gap numerator.
    pub price_margin_num: u128,
    /// TRACE_MATRIX FC2-N29 + FC1-N7: price-margin dominance gap denominator.
    pub price_margin_den: u128,
}

impl BoltzmannPolicySnapshot {
    /// TRACE_MATRIX FC2-N29 + FC1-N7: project the live integer-rational policy
    /// into the persisted integer-only snapshot.
    pub fn from_policy(policy: &BoltzmannMaskPolicy) -> Self {
        Self {
            epsilon_exploration_num: policy.epsilon_exploration_num,
            epsilon_exploration_den: policy.epsilon_exploration_den,
            beta_num: policy.beta_num,
            beta_den: policy.beta_den,
            min_liquidity_micro: policy.min_liquidity.micro_units(),
            price_margin_num: policy.price_margin.numerator,
            price_margin_den: policy.price_margin.denominator,
        }
    }
}

/// TRACE_MATRIX FC2-N29 + FC1-N7: tape-anchored OBSERVE-ONLY Boltzmann
/// selection trace. Derived view (CR-14.1 / "price is signal, not truth");
/// never a source of truth, never an admission/predicate input. Self-addressing
/// per R3: stored bytes have `trace_id` zeroed so
/// `Cid::from_content(stored_bytes) == trace_id`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoltzmannSelectionTrace {
    /// TRACE_MATRIX FC2-N29 + FC1-N7: CAS Cid of this trace's canonical bytes
    /// (with `trace_id` zeroed during the hash). Computed by the recorder.
    pub trace_id: Cid,
    /// TRACE_MATRIX FC2-N29 + FC1-N7: logical time at which the trace was taken.
    pub logical_t: u64,
    /// TRACE_MATRIX FC2-N29 + FC1-N7: post-filter candidate nodes with their
    /// integer-rational `price_yes` (num/den only). The exact set the scheduler
    /// chose from (price_yes present AND not masked).
    pub candidate_nodes: Vec<(TxId, RationalPrice)>,
    /// TRACE_MATRIX FC2-N29 + FC1-N7: parent nodes suppressed by the
    /// `compute_mask_set` read-view filter.
    pub masked_nodes: Vec<TxId>,
    /// TRACE_MATRIX FC2-N29 + FC1-N7: integer-only snapshot of the policy used.
    pub policy: BoltzmannPolicySnapshot,
    /// TRACE_MATRIX FC2-N29 + FC1-N7: seed for the replay-deterministic RNG.
    pub rng_seed: u64,
    /// TRACE_MATRIX FC2-N29 + FC1-N7: the recommended parent (`None` iff
    /// candidate set empty). A recommendation only; not an admission decision.
    pub selected_parent: Option<TxId>,
    /// TRACE_MATRIX FC2-N29 + FC1-N7: which scheduler branch produced the pick.
    pub selection_branch: SelectionBranch,
    /// TRACE_MATRIX FC2-N29 + FC1-N7: published FC2 constitution-flowchart hash
    /// this derived view anchors to.
    pub constitution_hash: String,
    /// TRACE_MATRIX FC2-N29 + FC1-N7: always `true`. This trace can never be
    /// canonical state or an admission/predicate input.
    pub observe_only: bool,
    /// TRACE_MATRIX FC2-N29 + FC1-N7: free-form schema tag for discovery.
    pub schema_tag: String,
}

/// TRACE_MATRIX FC2-N29 + FC1-N7: honestly recompute which branch of
/// `boltzmann_select_parent_v2` fires, by reproducing the EXACT same seeded
/// RNG draw sequence the selector consumes. Deterministic given identical
/// `(candidate_count, policy, seed)`. Does not pick a candidate — it only
/// classifies the branch.
fn recompute_selection_branch(
    candidate_count: usize,
    policy: &BoltzmannMaskPolicy,
    rng_seed: u64,
) -> SelectionBranch {
    if candidate_count == 0 {
        return SelectionBranch::EmptyCandidates;
    }
    // Mirror `boltzmann_select_parent_v2`: the first RNG draw is the epsilon
    // roll, taken iff `epsilon_exploration_den > 0`. If `r < epsilon_num`, the
    // exploration branch is taken; otherwise argmax.
    if policy.epsilon_exploration_den > 0 {
        let mut rng = StdRng::seed_from_u64(rng_seed);
        let r: u64 = rng.gen_range(0..policy.epsilon_exploration_den);
        if r < policy.epsilon_exploration_num {
            return SelectionBranch::EpsilonExploration;
        }
    }
    SelectionBranch::ArgmaxExploit
}

/// TRACE_MATRIX FC2-N29 + FC1-N7: run the full live derivation chain over a
/// BORROWED canonical `EconomicState` and persist the observe-only Boltzmann
/// selection trace to CAS. Returns the self-addressed `trace_id`.
///
/// Chain (mirrors `src/bus.rs:560-563`, but actually calls the scheduler):
/// 1. `price_index = compute_price_index(econ)`
/// 2. `mask = compute_mask_set(econ, edges, policy, &price_index)`
/// 3. `selected = boltzmann_select_parent_v2(&price_index, &mask, policy,
///    &mut StdRng::seed_from_u64(rng_seed))`
/// 4. classify the branch deterministically via `recompute_selection_branch`
/// 5. persist via `cas.put` (`ObjectType::Generic` + free-form schema_id),
///    self-addressed (R3 zero-then-hash) — the CAS write IS the L4 anchor.
///
/// **Observe-only**: `econ` is `&EconomicState`; nothing is mutated, no head
/// advances. **No fs**: persistence is CAS-only. **Integer-only**: the float
/// `boltzmann_softmax_select_parent` is intentionally not called.
pub fn record_boltzmann_selection_over_econ(
    cas: &mut CasStore,
    econ: &EconomicState,
    edges: &CanonicalNodeGraph,
    policy: &BoltzmannMaskPolicy,
    rng_seed: u64,
    logical_t: u64,
) -> Result<Cid, CasError> {
    // Step 1 + 2: live derivation chain over the borrowed canonical econ.
    let price_index = compute_price_index(econ);
    let mask = compute_mask_set(econ, edges, policy, &price_index);

    // Step 3: run the integer-rational scheduler with a replay-deterministic RNG.
    let mut rng = StdRng::seed_from_u64(rng_seed);
    let selected_parent = boltzmann_select_parent_v2(&price_index, &mask, policy, &mut rng);

    // Candidate set = exactly what the scheduler chose from: price_yes present
    // AND not masked. Recorded with integer-rational price_yes (num/den only).
    let candidate_nodes: Vec<(TxId, RationalPrice)> = price_index
        .iter()
        .filter(|(node_id, entry)| entry.price_yes.is_some() && !mask.contains(node_id))
        .map(|(node_id, entry)| {
            (
                node_id.clone(),
                *entry.price_yes.as_ref().expect("filtered for Some"),
            )
        })
        .collect();

    let masked_nodes: Vec<TxId> = mask.iter().cloned().collect();

    // Step 4: honest deterministic branch classification.
    let selection_branch =
        recompute_selection_branch(candidate_nodes.len(), policy, rng_seed);

    // Build the trace with trace_id zeroed (R3 self-addressing discipline).
    let mut trace = BoltzmannSelectionTrace {
        trace_id: Cid::default(),
        logical_t,
        candidate_nodes,
        masked_nodes,
        policy: BoltzmannPolicySnapshot::from_policy(policy),
        rng_seed,
        selected_parent,
        selection_branch,
        constitution_hash: BOLTZMANN_TRACE_FC2_CONSTITUTION_HASH.to_string(),
        observe_only: true,
        schema_tag: BOLTZMANN_SELECTION_TRACE_SCHEMA_ID.to_string(),
    };

    // R3: store the bytes with trace_id zeroed so
    // Cid::from_content(stored_bytes) == trace_id, and cas.get(&trace_id)
    // resolves the very bytes we stored.
    let stored_bytes = canonical_encode(&trace)
        .map_err(|e| CasError::BackendCorruption(format!("boltzmann trace encode: {e:?}")))?;
    let returned_cid = cas.put(
        &stored_bytes,
        ObjectType::Generic,
        "s3-boltzmann-selection-trace",
        logical_t,
        Some(BOLTZMANN_SELECTION_TRACE_SCHEMA_ID.to_string()),
    )?;
    debug_assert_eq!(
        returned_cid,
        Cid::from_content(&stored_bytes),
        "CAS-returned cid must equal sha256(stored_bytes); CasStore::put contract"
    );

    trace.trace_id = returned_cid;
    Ok(returned_cid)
}

/// TRACE_MATRIX FC2-N29 + FC1-N7: rebuild a `BoltzmannSelectionTrace` from
/// CAS-resident bytes. Caller supplies the bytes returned by
/// `cas.get(&trace_id)`. Re-derives `trace_id` from `Cid::from_content(bytes)`,
/// returning the ergonomic in-memory view identical to what the recorder
/// returned (R3 round-trip).
pub fn restore_boltzmann_selection_from_cas_bytes(
    bytes: &[u8],
) -> Result<BoltzmannSelectionTrace, CasError> {
    use crate::bottom_white::ledger::transition_ledger::canonical_decode;
    let mut trace: BoltzmannSelectionTrace = canonical_decode(bytes)
        .map_err(|e| CasError::BackendCorruption(format!("boltzmann trace decode: {e:?}")))?;
    trace.trace_id = Cid::from_content(bytes);
    Ok(trace)
}

/// TRACE_MATRIX FC2-N29 + FC1-N7: read + restore a trace by Cid from CAS.
pub fn read_boltzmann_selection_from_cas(
    cas: &CasStore,
    cid: &Cid,
) -> Result<BoltzmannSelectionTrace, CasError> {
    let bytes = cas.get(cid)?;
    restore_boltzmann_selection_from_cas_bytes(&bytes)
}

/// TRACE_MATRIX FC2-N29 + FC1-N7: discover all Boltzmann selection trace Cids
/// in a CAS by schema_id (mirrors `scheduler_decision_trace_cids`).
pub fn boltzmann_selection_trace_cids(cas: &CasStore) -> Vec<Cid> {
    cas.list_all_cids()
        .into_iter()
        .filter(|cid| {
            cas.metadata(cid).and_then(|meta| meta.schema_id.as_deref())
                == Some(BOLTZMANN_SELECTION_TRACE_SCHEMA_ID)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::price_index::NodeMarketEntry;
    use crate::state::q_state::{AgentId, TaskId};
    use crate::state::typed_tx::{PositionKind, PositionSide};
    use crate::economy::money::MicroCoin;
    use std::sync::{Arc, RwLock};
    use tempfile::TempDir;

    fn make_position(
        position_id: &str,
        node_id: &str,
        task_id: &str,
        owner: &str,
        side: PositionSide,
        amount_micro: i64,
    ) -> crate::state::typed_tx::NodePosition {
        crate::state::typed_tx::NodePosition {
            position_id: TxId(position_id.into()),
            node_id: TxId(node_id.into()),
            task_id: TaskId(task_id.into()),
            owner: AgentId(owner.into()),
            side,
            kind: PositionKind::FirstLong,
            amount: MicroCoin::from_micro_units(amount_micro),
            source_tx: TxId(position_id.into()),
            opened_at_round: 1,
        }
    }

    fn econ_with(positions: Vec<crate::state::typed_tx::NodePosition>) -> EconomicState {
        let mut econ = EconomicState::default();
        for p in positions {
            econ.node_positions_t.0.insert(p.position_id.clone(), p);
        }
        econ
    }

    fn open_cas() -> (TempDir, Arc<RwLock<CasStore>>) {
        let tmp = TempDir::new().expect("tempdir");
        let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas")));
        (tmp, cas)
    }

    /// Empty econ → empty candidate set → EmptyCandidates branch, None pick.
    #[test]
    fn empty_econ_records_empty_candidates_branch() {
        let (_tmp, cas) = open_cas();
        let econ = EconomicState::default();
        let policy = BoltzmannMaskPolicy::default();
        let edges = CanonicalNodeGraph::default();
        let cid = {
            let mut w = cas.write().unwrap();
            record_boltzmann_selection_over_econ(&mut w, &econ, &edges, &policy, 42, 7).unwrap()
        };
        let trace = {
            let r = cas.read().unwrap();
            read_boltzmann_selection_from_cas(&r, &cid).unwrap()
        };
        assert_eq!(trace.selection_branch, SelectionBranch::EmptyCandidates);
        assert_eq!(trace.selected_parent, None);
        assert!(trace.candidate_nodes.is_empty());
        assert!(trace.observe_only);
    }

    /// Non-empty econ → trace records candidates, selects a parent, and the
    /// trace is reconstructable from CAS alone (Art.0.2).
    #[test]
    fn records_selection_and_reconstructs_from_cas() {
        let (_tmp, cas) = open_cas();
        let econ = econ_with(vec![
            make_position("p1", "low", "t1", "a1", PositionSide::Long, 300_000),
            make_position("p2", "high", "t2", "a2", PositionSide::Long, 900_000),
        ]);
        // epsilon=0 → deterministic argmax path → picks "high".
        let policy = BoltzmannMaskPolicy {
            epsilon_exploration_num: 0,
            epsilon_exploration_den: 1,
            ..BoltzmannMaskPolicy::default()
        };
        let edges = CanonicalNodeGraph::default();
        let cid = {
            let mut w = cas.write().unwrap();
            record_boltzmann_selection_over_econ(&mut w, &econ, &edges, &policy, 1234, 9).unwrap()
        };
        let trace = {
            let r = cas.read().unwrap();
            read_boltzmann_selection_from_cas(&r, &cid).unwrap()
        };
        assert_eq!(trace.selection_branch, SelectionBranch::ArgmaxExploit);
        // Both single-long positions yield price_yes = 1/1; argmax ties break
        // by BTreeMap lex order ("high" < "low"). Either way a parent is picked.
        assert!(trace.selected_parent.is_some());
        assert_eq!(trace.candidate_nodes.len(), 2);
        assert!(trace.observe_only);
        // R3: trace_id self-addresses the stored bytes.
        assert_eq!(trace.trace_id, cid);
    }

    /// Recorder does NOT mutate econ (observe-only): econ before == after.
    #[test]
    fn recorder_does_not_mutate_econ() {
        let (_tmp, cas) = open_cas();
        let econ = econ_with(vec![make_position(
            "p1", "n1", "t1", "a1", PositionSide::Long, 500_000,
        )]);
        let before = econ.clone();
        let policy = BoltzmannMaskPolicy::default();
        let edges = CanonicalNodeGraph::default();
        {
            let mut w = cas.write().unwrap();
            record_boltzmann_selection_over_econ(&mut w, &econ, &edges, &policy, 5, 5).unwrap();
        }
        assert_eq!(econ, before, "observe-only: econ must be byte-identical");
    }

    /// Same seed + same econ → identical trace_id (replay-determinism).
    #[test]
    fn replay_determinism_same_seed_same_cid() {
        let econ = econ_with(vec![
            make_position("p1", "n1", "t1", "a1", PositionSide::Long, 600_000),
            make_position("p2", "n1", "t1", "a2", PositionSide::Short, 400_000),
        ]);
        let policy = BoltzmannMaskPolicy::default();
        let edges = CanonicalNodeGraph::default();
        let run = || {
            let tmp = TempDir::new().unwrap();
            let mut cas = CasStore::open(tmp.path()).unwrap();
            let cid =
                record_boltzmann_selection_over_econ(&mut cas, &econ, &edges, &policy, 77, 3)
                    .unwrap();
            (tmp, cid)
        };
        let (_t1, a) = run();
        let (_t2, b) = run();
        assert_eq!(a, b, "identical seed+econ must yield identical trace_id");
    }

    /// Discovery: schema_id filter finds the trace we wrote.
    #[test]
    fn discovery_lists_written_trace() {
        let (_tmp, cas) = open_cas();
        let econ = econ_with(vec![make_position(
            "p1", "n1", "t1", "a1", PositionSide::Long, 100_000,
        )]);
        let policy = BoltzmannMaskPolicy::default();
        let edges = CanonicalNodeGraph::default();
        let cid = {
            let mut w = cas.write().unwrap();
            record_boltzmann_selection_over_econ(&mut w, &econ, &edges, &policy, 1, 1).unwrap()
        };
        let r = cas.read().unwrap();
        let cids = boltzmann_selection_trace_cids(&r);
        assert!(cids.contains(&cid), "discovery must find the written trace");
    }

    /// Branch recompute matches the selector: high epsilon → exploration.
    #[test]
    fn high_epsilon_classifies_exploration() {
        // epsilon = 1/1 → roll always fires → EpsilonExploration.
        let policy = BoltzmannMaskPolicy {
            epsilon_exploration_num: 1,
            epsilon_exploration_den: 1,
            ..BoltzmannMaskPolicy::default()
        };
        assert_eq!(
            recompute_selection_branch(3, &policy, 2026),
            SelectionBranch::EpsilonExploration
        );
        // Zero candidates always wins regardless of epsilon.
        assert_eq!(
            recompute_selection_branch(0, &policy, 2026),
            SelectionBranch::EmptyCandidates
        );
    }

    /// Persisted policy snapshot + prices carry no decimal point (integer-only).
    #[test]
    fn trace_json_is_integer_only() {
        let (_tmp, cas) = open_cas();
        let econ = econ_with(vec![
            make_position("p1", "n1", "t1", "a1", PositionSide::Long, 700_000),
            make_position("p2", "n1", "t1", "a2", PositionSide::Short, 300_000),
        ]);
        let policy = BoltzmannMaskPolicy::default();
        let edges = CanonicalNodeGraph::default();
        let cid = {
            let mut w = cas.write().unwrap();
            record_boltzmann_selection_over_econ(&mut w, &econ, &edges, &policy, 9, 9).unwrap()
        };
        let trace = {
            let r = cas.read().unwrap();
            read_boltzmann_selection_from_cas(&r, &cid).unwrap()
        };
        // constitution_hash is a hex string and legitimately has no '.'; the
        // numeric fields (prices, policy, seed) are all integers — assert no
        // float ever serialized into the numeric surface.
        let json = serde_json::to_string(&trace.candidate_nodes).unwrap();
        assert!(!json.contains('.'), "candidate prices must be integer-rational");
        let pol = serde_json::to_string(&trace.policy).unwrap();
        assert!(!pol.contains('.'), "policy snapshot must be integer-only");
    }

    // Silence unused import warning for NodeMarketEntry in case the helper set
    // shrinks; it documents the price_index entry type the candidate set uses.
    #[allow(dead_code)]
    fn _entry_type_doc(_e: NodeMarketEntry) {}
}
