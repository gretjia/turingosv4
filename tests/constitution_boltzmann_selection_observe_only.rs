//! S3 Boltzmann observe-only selection trace — triple-coupled constitution gate.
//!
//! Brings the previously dead-but-tested integer-rational scheduler
//! `boltzmann_select_parent_v2` ONLINE as a tape-anchored OBSERVE-ONLY selection
//! record (`src/runtime/boltzmann_selection_trace.rs`, nested as a `#[path]`
//! submodule of the UNPINNED `src/runtime/agent_scheduler.rs`). The recorder runs
//! the full live derivation chain — `compute_price_index` → `compute_mask_set` →
//! `boltzmann_select_parent_v2` — over a BORROWED `&EconomicState` and persists
//! the recommendation to CAS via `CasStore::put` (the L4 anchor). "Price is
//! signal, not truth" (CR-14.1): the trace is a derived view, never canonical
//! state, never an admission/predicate input.
//!
//! Four coupled properties proved here, each constructed to fail if the property
//! breaks:
//!
//!   1. **RECONSTRUCTABLE (Art. 0.2)** — a realistic `EconomicState` with three
//!      DISTINCT-priced nodes is recorded, then round-tripped from CAS:
//!      self-addressed (`Cid::from_content(stored) == trace_id`),
//!      `restore_*_from_cas_bytes == read_*_from_cas == in-memory`, and the
//!      schema-id is discoverable via `boltzmann_selection_trace_cids`.
//!
//!   2. **REPLAY-DETERMINISM / REAL-WIRING** — the recorded `selected_parent`
//!      EQUALS a FRESH independent call to `boltzmann_select_parent_v2(
//!      &compute_price_index(&econ), &mask, &policy, &mut
//!      StdRng::seed_from_u64(seed))` with the same inputs/seed. This proves the
//!      recorder records the REAL function output, not a hand-set value.
//!
//!   3. **NON-VACUOUS positive controls** — with `epsilon_exploration_num=0` the
//!      recorded `selected_parent` == the node with the highest `price_yes`
//!      (genuine argmax) and `selection_branch == ArgmaxExploit`; AND with an
//!      epsilon policy + a firing seed, `selection_branch == EpsilonExploration`
//!      is reachable. The branch label is the genuine computed branch.
//!
//!   4. **OBSERVE-ONLY witness** — `econ` is byte-identical before/after the
//!      recorder call (no mutation, no head advance — the recorder returns only a
//!      `Cid`), and writing the trace mints NO filesystem side-store next to the
//!      CAS repo (canonical store is CAS/tape, no `skills/`, `memory/`, or
//!      `*.json` sidecar).
//!
//! TRACE_MATRIX FC2-N29 (economic derived view) + FC1-N7 (scheduler read-view).

use std::collections::BTreeSet;
use tempfile::TempDir;

use rand::rngs::StdRng;
use rand::SeedableRng;

use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::economy::money::MicroCoin;
use turingosv4::runtime::agent_scheduler::boltzmann_selection_trace::{
    boltzmann_selection_trace_cids, read_boltzmann_selection_from_cas,
    record_boltzmann_selection_over_econ, restore_boltzmann_selection_from_cas_bytes,
    SelectionBranch, BOLTZMANN_SELECTION_TRACE_SCHEMA_ID,
};
use turingosv4::sdk::actor::boltzmann_select_parent_v2;
use turingosv4::state::price_index::{
    compute_mask_set, compute_price_index, BoltzmannMaskPolicy, CanonicalNodeGraph, RationalPrice,
};
use turingosv4::state::q_state::{AgentId, EconomicState, TaskId};
use turingosv4::state::typed_tx::{NodePosition, PositionKind, PositionSide};
use turingosv4::state::TxId;

// --- realistic-econ builders (accepted WorkTx → NodeMarketEntry with distinct
//     price_yes). price_yes = long_micro / (long_micro + short_micro), so mixing
//     long/short per node yields DISTINCT integer-rational prices. ---

fn position(
    position_id: &str,
    node_id: &str,
    task_id: &str,
    owner: &str,
    side: PositionSide,
    amount_micro: i64,
) -> NodePosition {
    NodePosition {
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

fn econ_with(positions: Vec<NodePosition>) -> EconomicState {
    let mut econ = EconomicState::default();
    for p in positions {
        econ.node_positions_t.0.insert(p.position_id.clone(), p);
    }
    econ
}

/// Three DISTINCT-priced nodes:
///   - "node_low":  long=100k short=900k  → price_yes = 100/1000 = 1/10
///   - "node_mid":  long=500k short=500k  → price_yes = 500/1000 = 1/2
///   - "node_high": long=900k short=100k  → price_yes = 900/1000 = 9/10  (argmax)
/// Distinct prices make the argmax control non-vacuous (the pick is the genuine
/// highest-price node, not a tie broken by lex order).
fn three_distinct_priced_nodes() -> EconomicState {
    econ_with(vec![
        position("pL_l", "node_low", "t", "aL", PositionSide::Long, 100_000),
        position("pL_s", "node_low", "t", "aL2", PositionSide::Short, 900_000),
        position("pM_l", "node_mid", "t", "aM", PositionSide::Long, 500_000),
        position("pM_s", "node_mid", "t", "aM2", PositionSide::Short, 500_000),
        position("pH_l", "node_high", "t", "aH", PositionSide::Long, 900_000),
        position("pH_s", "node_high", "t", "aH2", PositionSide::Short, 100_000),
    ])
}

fn open_cas_dir() -> (TempDir, std::path::PathBuf) {
    let parent = TempDir::new().expect("tempdir");
    let cas_dir = parent.path().join("cas_repo");
    std::fs::create_dir_all(&cas_dir).expect("mkdir cas_repo");
    (parent, cas_dir)
}

/// The canonical highest-`price_yes` node under the *real* `RationalPrice`
/// cross-multiplication ordering (integer-only; never f64). Returns the argmax
/// node id over the priced candidate set. This is the independent oracle the
/// epsilon=0 control compares the recorder against.
fn argmax_price_yes_node(econ: &EconomicState) -> TxId {
    let price_index = compute_price_index(econ);
    let mut best: Option<(TxId, RationalPrice)> = None;
    for (node, entry) in price_index.iter() {
        if let Some(p) = entry.price_yes.as_ref() {
            match &best {
                None => best = Some((node.clone(), *p)),
                Some((_, bp)) => {
                    // p > bp via cross-multiplication (integer only).
                    let lhs = p.numerator.saturating_mul(bp.denominator);
                    let rhs = bp.numerator.saturating_mul(p.denominator);
                    if lhs > rhs {
                        best = Some((node.clone(), *p));
                    }
                }
            }
        }
    }
    best.expect("at least one priced candidate").0
}

// =====================================================================
// (1) RECONSTRUCTABLE — record then round-trip from CAS alone (Art. 0.2)
// =====================================================================
#[test]
fn trace_is_reconstructable_from_cas() {
    let (_parent, cas_dir) = open_cas_dir();
    let econ = three_distinct_priced_nodes();
    let policy = BoltzmannMaskPolicy {
        epsilon_exploration_num: 0,
        epsilon_exploration_den: 1,
        ..BoltzmannMaskPolicy::default()
    };
    let edges = CanonicalNodeGraph::default();

    let mut cas = CasStore::open(&cas_dir).expect("cas open");
    let cid = record_boltzmann_selection_over_econ(&mut cas, &econ, &edges, &policy, 1234, 9)
        .expect("record trace");

    // Self-addressed: the stored bytes hash to the returned Cid.
    let stored = cas.get(&cid).expect("Art. 0.2: cas.get(&trace_id) MUST succeed");
    assert_eq!(
        Cid::from_content(&stored),
        cid,
        "self-addressed: sha256(stored bytes) == trace_id"
    );

    // restore_*_from_cas_bytes == read_*_from_cas == in-memory trace.
    let from_bytes = restore_boltzmann_selection_from_cas_bytes(&stored).expect("restore bytes");
    let from_cas = read_boltzmann_selection_from_cas(&cas, &cid).expect("read from cas");
    assert_eq!(from_bytes, from_cas, "two restore paths agree");
    assert_eq!(from_cas.trace_id, cid, "restored trace_id self-addresses");
    assert_eq!(
        from_cas.candidate_nodes.len(),
        3,
        "all three priced nodes are candidates"
    );
    assert!(from_cas.observe_only, "trace is observe-only");
    assert_eq!(from_cas.schema_tag, BOLTZMANN_SELECTION_TRACE_SCHEMA_ID);

    // Schema-id discoverable.
    let cids = boltzmann_selection_trace_cids(&cas);
    assert!(
        cids.contains(&cid),
        "schema-id discovery must surface the written trace"
    );
}

// =====================================================================
// (2) REPLAY-DETERMINISM / REAL-WIRING — recorded pick == fresh selector call
// =====================================================================
#[test]
fn recorded_pick_equals_fresh_boltzmann_select_parent_v2() {
    let (_parent, cas_dir) = open_cas_dir();
    let econ = three_distinct_priced_nodes();
    // Use a real epsilon policy so the seeded RNG path is genuinely exercised
    // (not the trivial epsilon_den==0 short-circuit).
    let policy = BoltzmannMaskPolicy {
        epsilon_exploration_num: 1,
        epsilon_exploration_den: 2,
        ..BoltzmannMaskPolicy::default()
    };
    let edges = CanonicalNodeGraph::default();
    let seed: u64 = 4242;
    let logical_t: u64 = 11;

    let mut cas = CasStore::open(&cas_dir).expect("cas open");
    let cid = record_boltzmann_selection_over_econ(&mut cas, &econ, &edges, &policy, seed, logical_t)
        .expect("record");
    let recorded = read_boltzmann_selection_from_cas(&cas, &cid).expect("read");

    // FRESH, independent re-derivation with identical inputs + seed.
    let price_index = compute_price_index(&econ);
    let mask = compute_mask_set(&econ, &edges, &policy, &price_index);
    let mut rng = StdRng::seed_from_u64(seed);
    let fresh = boltzmann_select_parent_v2(&price_index, &mask, &policy, &mut rng);

    assert_eq!(
        recorded.selected_parent, fresh,
        "recorded pick MUST equal a fresh boltzmann_select_parent_v2 call \
         with the same inputs+seed — proves real wiring, not a hand-set value"
    );

    // The recorded rng_seed is the actual seed used (replay reproducibility).
    assert_eq!(recorded.rng_seed, seed);

    // Replay-determinism across independent CAS instances: same inputs+seed →
    // identical self-addressed trace_id.
    let (_p2, dir2) = open_cas_dir();
    let mut cas2 = CasStore::open(&dir2).expect("cas2");
    let cid2 = record_boltzmann_selection_over_econ(&mut cas2, &econ, &edges, &policy, seed, logical_t)
        .expect("record again");
    assert_eq!(cid, cid2, "identical inputs+seed → identical trace_id");
}

// =====================================================================
// (3a) NON-VACUOUS positive control — epsilon=0 → argmax(price_yes) exploit
// =====================================================================
#[test]
fn epsilon_zero_picks_highest_price_argmax_exploit() {
    let (_parent, cas_dir) = open_cas_dir();
    let econ = three_distinct_priced_nodes();
    // epsilon_num=0 → exploration roll can never fire → pure deterministic argmax.
    let policy = BoltzmannMaskPolicy {
        epsilon_exploration_num: 0,
        epsilon_exploration_den: 1,
        ..BoltzmannMaskPolicy::default()
    };
    let edges = CanonicalNodeGraph::default();

    let mut cas = CasStore::open(&cas_dir).expect("cas open");
    let cid = record_boltzmann_selection_over_econ(&mut cas, &econ, &edges, &policy, 7, 1)
        .expect("record");
    let trace = read_boltzmann_selection_from_cas(&cas, &cid).expect("read");

    // Genuine argmax: "node_high" (9/10) > "node_mid" (1/2) > "node_low" (1/10).
    let expected = argmax_price_yes_node(&econ);
    assert_eq!(
        expected,
        TxId("node_high".into()),
        "sanity: the independent oracle picks node_high (highest price_yes)"
    );
    assert_eq!(
        trace.selected_parent,
        Some(expected),
        "epsilon=0 recorder MUST pick the genuine highest-price_yes node (argmax)"
    );
    assert_eq!(
        trace.selection_branch,
        SelectionBranch::ArgmaxExploit,
        "epsilon=0 → genuine ArgmaxExploit branch label"
    );
}

// =====================================================================
// (3b) NON-VACUOUS reachability — epsilon policy + firing seed → exploration
// =====================================================================
#[test]
fn epsilon_branch_is_reachable_with_firing_seed() {
    let econ = three_distinct_priced_nodes();
    // 50% epsilon: some seeds fire the exploration roll, some do not. We discover
    // a firing seed at runtime (no rand-version-fragile hardcoded seed) and prove
    // the recorder's GENUINE computed branch label is EpsilonExploration there.
    let policy = BoltzmannMaskPolicy {
        epsilon_exploration_num: 1,
        epsilon_exploration_den: 2,
        ..BoltzmannMaskPolicy::default()
    };
    let edges = CanonicalNodeGraph::default();
    let argmax = argmax_price_yes_node(&econ);

    let mut found_exploration = false;
    let mut explored_diff_from_argmax = false;
    for seed in 0u64..256 {
        let (_p, dir) = open_cas_dir();
        let mut cas = CasStore::open(&dir).expect("cas");
        let cid = record_boltzmann_selection_over_econ(&mut cas, &econ, &edges, &policy, seed, 1)
            .expect("record");
        let trace = read_boltzmann_selection_from_cas(&cas, &cid).expect("read");
        if trace.selection_branch == SelectionBranch::EpsilonExploration {
            found_exploration = true;
            // The exploration branch's pick must still be a real candidate.
            let candidate_ids: BTreeSet<TxId> =
                trace.candidate_nodes.iter().map(|(id, _)| id.clone()).collect();
            assert!(
                trace
                    .selected_parent
                    .as_ref()
                    .map(|p| candidate_ids.contains(p))
                    .unwrap_or(false),
                "exploration pick must be a real candidate"
            );
            if trace.selected_parent.as_ref() != Some(&argmax) {
                explored_diff_from_argmax = true;
            }
            if found_exploration && explored_diff_from_argmax {
                break;
            }
        }
    }
    assert!(
        found_exploration,
        "EpsilonExploration branch MUST be reachable with a firing seed (non-vacuous)"
    );
    assert!(
        explored_diff_from_argmax,
        "at least one exploration roll picks a non-argmax candidate \
         (exploration genuinely diverges from exploit)"
    );
}

// =====================================================================
// (4a) OBSERVE-ONLY — econ byte-identical before/after; recorder returns only Cid
// =====================================================================
#[test]
fn recorder_does_not_mutate_econ_or_advance_head() {
    let (_parent, cas_dir) = open_cas_dir();
    let econ = three_distinct_priced_nodes();
    let before = econ.clone();
    let policy = BoltzmannMaskPolicy::default();
    let edges = CanonicalNodeGraph::default();

    let mut cas = CasStore::open(&cas_dir).expect("cas open");
    // The recorder's ONLY return value is a Cid — it exposes no QState/head
    // mutation handle (observe-only: the read view cannot advance Q_{t+1}).
    let ret: Cid = record_boltzmann_selection_over_econ(&mut cas, &econ, &edges, &policy, 99, 1)
        .expect("record");
    let _ = ret;

    assert_eq!(
        econ, before,
        "observe-only: borrowed econ MUST be byte-identical after the recorder \
         (no QState/EconomicState mutation, no head advance)"
    );
}

// =====================================================================
// (4b) NO FILESYSTEM SIDE-STORE — canonical store is CAS/tape (Art. 0.2)
// =====================================================================
#[test]
fn recorder_creates_no_filesystem_side_store() {
    let parent = TempDir::new().expect("tempdir");
    let cas_dir = parent.path().join("cas_repo");
    std::fs::create_dir_all(&cas_dir).expect("mkdir cas_repo");

    // Snapshot the parent (working) dir BEFORE the recorder writes.
    let before: BTreeSet<String> = std::fs::read_dir(parent.path())
        .expect("read parent")
        .filter_map(|e| e.ok().map(|e| e.file_name().to_string_lossy().to_string()))
        .collect();

    let econ = three_distinct_priced_nodes();
    let policy = BoltzmannMaskPolicy::default();
    let edges = CanonicalNodeGraph::default();
    {
        let mut cas = CasStore::open(&cas_dir).expect("cas open");
        record_boltzmann_selection_over_econ(&mut cas, &econ, &edges, &policy, 3, 1).expect("record");
    }

    let after: BTreeSet<String> = std::fs::read_dir(parent.path())
        .expect("read parent")
        .filter_map(|e| e.ok().map(|e| e.file_name().to_string_lossy().to_string()))
        .collect();

    assert_eq!(
        before, after,
        "recorder must not mint a filesystem-side store next to the CAS repo \
         (canonical store is CAS/tape — Art. 0.2)"
    );
    for name in &after {
        assert!(
            !name.contains("boltzmann")
                && !name.contains("selection")
                && !name.ends_with(".json")
                && name != "skills"
                && name != "memory",
            "no filesystem-side trace store: unexpected entry `{name}`"
        );
    }
}

// =====================================================================
// MUTATION SANITY — the gate is falsifiable. We assert the trace's recorded
// pick against a DELIBERATELY-WRONG node id and confirm that assertion is RED
// (the comparison the real gate makes would fail). Wrapped in
// `std::panic::catch_unwind` so a RED inner assertion makes THIS test GREEN and
// a (hypothetical) silently-permissive gate would make it RED.
// =====================================================================
#[test]
fn mutation_wrong_pick_would_be_caught() {
    let (_parent, cas_dir) = open_cas_dir();
    let econ = three_distinct_priced_nodes();
    let policy = BoltzmannMaskPolicy {
        epsilon_exploration_num: 0,
        epsilon_exploration_den: 1,
        ..BoltzmannMaskPolicy::default()
    };
    let edges = CanonicalNodeGraph::default();

    let mut cas = CasStore::open(&cas_dir).expect("cas open");
    let cid = record_boltzmann_selection_over_econ(&mut cas, &econ, &edges, &policy, 7, 1)
        .expect("record");
    let trace = read_boltzmann_selection_from_cas(&cas, &cid).expect("read");

    // Deliberately-wrong expectation: the recorder picks node_high (argmax), so
    // asserting it equals node_low MUST panic. If it did NOT panic, the recorded
    // pick would be wrong/unconstrained and the real gate's (3a) assertion would
    // be vacuous.
    let wrong = Some(TxId("node_low".into()));
    let red = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        assert_eq!(trace.selected_parent, wrong);
    }));
    assert!(
        red.is_err(),
        "mutation sanity: asserting the recorded pick against a wrong node id \
         MUST be RED — the gate is falsifiable, not vacuous"
    );
    // And the real pick is the genuine argmax (the thing the wrong assertion missed).
    assert_eq!(trace.selected_parent, Some(TxId("node_high".into())));
}
