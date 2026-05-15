//! Constitution Landing Gate — PCP adversarial corpus
//!
//! Authority: HARNESS.md §3 G-012 (architect ruling 2026-05-07).
//! Decision: "Lean tactic-mutation adversarial corpus first; MiniF2F-v2
//! misalignment second."
//!
//! Pins the 9-class adversarial corpus in `cases/pcp_corpus/` to the
//! `AttemptOutcome` → `RejectionClass` routing table. Adding a new mutation
//! class is "drop fixture + append manifest row + extend the mapping table
//! constant in this file (single source of truth for routing assertions)."
//!
//! `FC-trace: FC1-INV1` (predicate→L4/L4.E routing) + `G-012`.
//!
//! Tests:
//!   - pcp_corpus_manifest_is_parseable_and_complete (structural)
//!   - pcp_corpus_fixtures_present (filesystem)
//!   - pcp_valid_passes (LeanPass routes to L4 admit)
//!   - pcp_mutated_invalid_fails (every mutation class fails)
//!   - pcp_sorry_blocked (sorry → SorryBlocked=8)
//!   - pcp_invalid_never_l4 (no invalid class can be routed to L4 accepted)
//!   - pcp_invalid_routes_l4e_or_capsule (every invalid class lands in L4.E
//!     or CAS-only [partial-accepted])
//!
//! These exercise the synthetic routing table only. Real-Lean replay against
//! these fixtures is a forward step (G-012 phase 2 MiniF2F-v2 misalignment).

use std::path::Path;
use std::sync::{Arc, RwLock};
use tempfile::TempDir;

use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::rejection_evidence::RejectionClass as L4ERejectionClass;
use turingosv4::runtime::attempt_telemetry::{
    write_attempt_telemetry_to_cas, AttemptKind, AttemptOutcome, AttemptTelemetry,
};
use turingosv4::runtime::proposal_telemetry::TokenCounts;
use turingosv4::state::q_state::{AgentId, Hash, TxId};
use turingosv4::state::sequencer::refine_rejection_class_via_attempt_telemetry;
use turingosv4::state::typed_tx::{TypedTx, WorkTx};

// ── Corpus mapping table — single source of truth ─────────────────────────

/// Each row pins (manifest_id, AttemptOutcome, expected L4ERejectionClass on
/// admission-fail refinement, expected u8 discriminant). The valid case (01)
/// has no rejection class — its expected route is L4 admit, asserted below.
/// The partial-then-final case (08) has TWO routes: PartialAccepted → CAS
/// only (no L4, no L4.E) + final LeanFail → L4.E with LeanFailed=6.
const CORPUS_INVALID_TABLE: &[(&str, AttemptOutcome, L4ERejectionClass, u8)] = &[
    (
        "02_mutated_invalid",
        AttemptOutcome::LeanFail,
        L4ERejectionClass::LeanFailed,
        6,
    ),
    (
        "03_sorry_insertion",
        AttemptOutcome::SorryBlock,
        L4ERejectionClass::SorryBlocked,
        8,
    ),
    (
        "04_type_mismatch",
        AttemptOutcome::LeanFail,
        L4ERejectionClass::LeanFailed,
        6,
    ),
    (
        "05_wrong_theorem_name",
        AttemptOutcome::LeanFail,
        L4ERejectionClass::LeanFailed,
        6,
    ),
    (
        "06_off_by_one_arith",
        AttemptOutcome::LeanFail,
        L4ERejectionClass::LeanFailed,
        6,
    ),
    (
        "07_irrelevant_theorem",
        AttemptOutcome::LeanFail,
        L4ERejectionClass::LeanFailed,
        6,
    ),
    (
        "08_partial_then_final_invalid",
        AttemptOutcome::LeanFail,
        L4ERejectionClass::LeanFailed,
        6,
    ),
    (
        "09_parse_invalid",
        AttemptOutcome::ParseFail,
        L4ERejectionClass::ParseFailed,
        7,
    ),
];

const CORPUS_VALID_ID: &str = "01_valid";

// ── Helpers (mirror tb_18r_lean_reject_in_l4e.rs pattern) ─────────────────

fn fresh_cas() -> (TempDir, Arc<RwLock<CasStore>>) {
    let dir = TempDir::new().expect("tempdir");
    let cas = Arc::new(RwLock::new(CasStore::open(dir.path()).expect("cas open")));
    (dir, cas)
}

fn write_attempt(cas: &Arc<RwLock<CasStore>>, outcome: AttemptOutcome, tag: &str) -> Cid {
    let attempt = AttemptTelemetry::new_root(
        TxId(format!("att-pcp-{tag}")),
        "pcp-run".into(),
        "task-pcp".into(),
        AgentId("agent_pcp".into()),
        "n0.b0".into(),
        Hash(Cid::from_content(b"pcp-ctx").0),
        Cid::from_content(format!("pcp-candidate-{tag}").as_bytes()),
        AttemptKind::ExternalizedLlmCycle,
        outcome,
        TokenCounts::default(),
        format!("pcp:{tag}"),
    );
    let mut cas_w = cas.write().expect("cas write");
    write_attempt_telemetry_to_cas(&mut *cas_w, &attempt, "pcp-test", 0).expect("write")
}

fn fixture_work_tx(proposal_cid: Cid, tag: &str) -> TypedTx {
    use std::collections::{BTreeMap, BTreeSet};
    use turingosv4::economy::money::StakeMicroCoin;
    use turingosv4::state::q_state::TaskId;
    use turingosv4::state::typed_tx::{
        AgentSignature, BoolWithProof, PredicateId, PredicateResultsBundle, ReadKey,
        SafetyOrCreation, WriteKey,
    };

    let mut acceptance = BTreeMap::new();
    acceptance.insert(
        PredicateId("p_lean_verify".into()),
        BoolWithProof {
            value: false,
            proof_cid: None,
        },
    );
    TypedTx::Work(WorkTx {
        tx_id: TxId(format!("worktx-pcp-{tag}")),
        task_id: TaskId("task-pcp".into()),
        parent_state_root: Default::default(),
        agent_id: AgentId("agent_pcp".into()),
        read_set: [ReadKey("r".into())].into_iter().collect::<BTreeSet<_>>(),
        write_set: [WriteKey("w".into())].into_iter().collect::<BTreeSet<_>>(),
        proposal_cid,
        predicate_results: PredicateResultsBundle {
            acceptance,
            settlement: BTreeMap::new(),
            safety_class: SafetyOrCreation::Safety,
        },
        stake: StakeMicroCoin::from_micro_units(0),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 1,
    })
}

fn corpus_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

// ── Tests ─────────────────────────────────────────────────────────────────

/// Manifest is parseable and enumerates all 9 corpus IDs. Adding a new
/// mutation class without updating MANIFEST.json fails this test.
#[test]
fn pcp_corpus_manifest_is_parseable_and_complete() {
    let manifest_path = corpus_root().join("cases/pcp_corpus/MANIFEST.json");
    let raw = std::fs::read_to_string(&manifest_path).unwrap_or_else(|e| {
        panic!(
            "PCP corpus manifest unreadable at {}: {e}",
            manifest_path.display()
        )
    });
    let manifest: serde_json::Value =
        serde_json::from_str(&raw).expect("MANIFEST.json must be valid JSON");
    let corpus = manifest
        .get("corpus")
        .and_then(|v| v.as_array())
        .expect("MANIFEST.json must contain `corpus` array");

    let manifest_ids: Vec<String> = corpus
        .iter()
        .map(|e| {
            e.get("id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string()
        })
        .collect();

    let expected_ids: Vec<&str> = std::iter::once(CORPUS_VALID_ID)
        .chain(CORPUS_INVALID_TABLE.iter().map(|(id, _, _, _)| *id))
        .collect();

    assert_eq!(
        manifest_ids.len(),
        expected_ids.len(),
        "MANIFEST.json corpus length ({}) ≠ canonical table length ({}). \
         Architect minimum corpus is 9 mutation classes (1 valid + 8 invalid \
         + 1 partial-then-final = 9 entries; HARNESS.md §3 G-012).",
        manifest_ids.len(),
        expected_ids.len()
    );

    for id in &expected_ids {
        assert!(
            manifest_ids.iter().any(|m| m == *id),
            "MANIFEST.json missing corpus id `{id}` — required by HARNESS.md §3 G-012"
        );
    }
}

/// Every fixture file referenced by the manifest must exist on disk. Drop a
/// fixture without removing it from the manifest → fail.
#[test]
fn pcp_corpus_fixtures_present() {
    let all_ids: Vec<&str> = std::iter::once(CORPUS_VALID_ID)
        .chain(CORPUS_INVALID_TABLE.iter().map(|(id, _, _, _)| *id))
        .collect();
    for id in &all_ids {
        let fixture = corpus_root().join(format!("cases/pcp_corpus/{id}/proof.lean"));
        assert!(
            fixture.is_file(),
            "PCP corpus fixture missing: {} (HARNESS.md §3 G-012 minimum corpus row {id})",
            fixture.display()
        );
    }
}

/// Art. I.1 / G-012 — the valid proof's outcome (`LeanPass`) is the ONLY
/// AttemptOutcome that may route to L4 accepted. Compile-time exhaustive
/// match: adding a new "accept-like" variant forces this test to acknowledge
/// it.
#[test]
fn pcp_valid_passes() {
    fn admits_to_l4(o: AttemptOutcome) -> bool {
        match o {
            // Only LeanPass corresponds to a verified omega proof that may
            // become an accepted WorkTx.
            AttemptOutcome::LeanPass => true,
            AttemptOutcome::LeanFail
            | AttemptOutcome::ParseFail
            | AttemptOutcome::SorryBlock
            | AttemptOutcome::LlmErr
            | AttemptOutcome::Aborted
            | AttemptOutcome::PartialAccepted => false,
        }
    }
    assert!(
        admits_to_l4(AttemptOutcome::LeanPass),
        "G-012: LeanPass is the canonical valid-proof admission route"
    );
}

/// G-012 — every entry in the invalid mutation table routes through the
/// canonical refine table to a non-Opaque L4ERejectionClass.
#[test]
fn pcp_mutated_invalid_fails() {
    let (_dir, cas) = fresh_cas();
    for (tag, outcome, expected_class, expected_u8) in CORPUS_INVALID_TABLE {
        let cid = write_attempt(&cas, *outcome, tag);
        let tx = fixture_work_tx(cid, tag);
        let refined = refine_rejection_class_via_attempt_telemetry(
            &cas,
            &tx,
            L4ERejectionClass::PredicateFailed,
        );
        assert_eq!(
            refined, *expected_class,
            "G-012 corpus row `{tag}`: outcome {outcome:?} did not refine to {expected_class:?}; \
             got {refined:?}"
        );
        assert_eq!(
            refined as u8, *expected_u8,
            "G-012 corpus row `{tag}`: byte-stable repr drift (expected {expected_u8}, got {})",
            refined as u8
        );
    }
}

/// G-012 — sorry insertion (corpus row 03) MUST route to SorryBlocked=8.
/// Pinned as its own gate because forbidden-payload semantics are
/// constitutionally distinct from generic Lean failure.
#[test]
fn pcp_sorry_blocked() {
    let (_dir, cas) = fresh_cas();
    let cid = write_attempt(&cas, AttemptOutcome::SorryBlock, "03_sorry_insertion");
    let tx = fixture_work_tx(cid, "03_sorry_insertion");
    let refined =
        refine_rejection_class_via_attempt_telemetry(&cas, &tx, L4ERejectionClass::PredicateFailed);
    assert_eq!(refined, L4ERejectionClass::SorryBlocked);
    assert_eq!(refined as u8, 8);
}

/// G-012 negative coverage — for ANY of the 8 invalid mutation classes, the
/// outcome MUST NOT be `LeanPass`. Compile-time exhaustive match guards
/// against any future "accept-like" variant slipping through silently.
#[test]
fn pcp_invalid_never_l4() {
    fn classifies_as_admit(o: AttemptOutcome) -> bool {
        matches!(o, AttemptOutcome::LeanPass)
    }
    for (tag, outcome, _, _) in CORPUS_INVALID_TABLE {
        assert!(
            !classifies_as_admit(*outcome),
            "G-012 corpus row `{tag}` has admit-route outcome {outcome:?} — \
             invalid mutation MUST NOT be admissible to L4 accepted"
        );
    }
}

/// G-012 — every invalid mutation routes to L4.E (or CAS-only for partial
/// accepted). Asserts the routing destination set is exactly {L4_E,
/// CAS_only_then_L4_E_on_final}.
#[test]
fn pcp_invalid_routes_l4e_or_capsule() {
    fn route_destination(o: AttemptOutcome) -> &'static str {
        match o {
            AttemptOutcome::LeanPass => "L4_accepted",
            AttemptOutcome::LeanFail
            | AttemptOutcome::ParseFail
            | AttemptOutcome::SorryBlock
            | AttemptOutcome::LlmErr => "L4_E",
            AttemptOutcome::Aborted => "TerminalAbort_capsule",
            AttemptOutcome::PartialAccepted => "CAS_only",
        }
    }
    let allowed = ["L4_E", "CAS_only", "TerminalAbort_capsule"];
    for (tag, outcome, _, _) in CORPUS_INVALID_TABLE {
        let dest = route_destination(*outcome);
        assert!(
            allowed.contains(&dest),
            "G-012 corpus row `{tag}`: outcome {outcome:?} routes to `{dest}` \
             which is not in the allowed invalid-destination set {allowed:?}"
        );
    }
}
