//! TB-18R R3 Integration Test — failure-path AttemptTelemetry routes to L4.E
//! with the right `RejectionClass`.
//!
//! Maps to charter v2 SG-18R.5 (FR-18R.5): real Lean rejects appear in L4.E
//! with rejection_class set; sorry-blocks distinguished from generic Lean
//! failures; LLM errors carry their own class.
//!
//! Verifies the per-outcome → per-RejectionClass mapping for each of the 4
//! R3 NEW variants. Companion to `tb_18r_attempt_routes_to_l4_or_l4e.rs`
//! (which covers the broader fallback / disjoint-base / non-Work cases).
//!
//! TRACE_MATRIX FC1-N42 (TB-18R R3 NEW witness — outcome→rejection_class).

use std::sync::{Arc, RwLock};
use tempfile::TempDir;

use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::rejection_evidence::RejectionClass;
use turingosv4::runtime::attempt_telemetry::{
    write_attempt_telemetry_to_cas, AttemptKind, AttemptOutcome, AttemptTelemetry,
};
use turingosv4::runtime::proposal_telemetry::TokenCounts;
use turingosv4::state::q_state::{AgentId, Hash, TxId};
use turingosv4::state::sequencer::refine_rejection_class_via_attempt_telemetry;
use turingosv4::state::typed_tx::{TypedTx, WorkTx};

fn fresh_cas() -> (TempDir, Arc<RwLock<CasStore>>) {
    let dir = TempDir::new().expect("tempdir");
    let cas = Arc::new(RwLock::new(CasStore::open(dir.path()).expect("cas open")));
    (dir, cas)
}

fn write_failure_attempt(cas: &Arc<RwLock<CasStore>>, outcome: AttemptOutcome, tag: &str) -> Cid {
    let attempt = AttemptTelemetry::new_root(
        TxId(format!("att-{tag}")),
        "test-run".into(),
        "task-test".into(),
        AgentId("agent_0".into()),
        "n0.b0".into(),
        Hash(Cid::from_content(b"ctx").0),
        Cid::from_content(format!("candidate-{tag}").as_bytes()),
        AttemptKind::ExternalizedLlmCycle,
        outcome,
        TokenCounts::default(),
        tag.into(),
    );
    let mut cas_w = cas.write().expect("cas write");
    write_attempt_telemetry_to_cas(&mut *cas_w, &attempt, "test", 0).expect("write")
}

fn fixture_failure_work_tx(proposal_cid: Cid) -> TypedTx {
    use std::collections::{BTreeMap, BTreeSet};
    use turingosv4::economy::money::StakeMicroCoin;
    use turingosv4::state::q_state::TaskId;
    use turingosv4::state::typed_tx::{
        AgentSignature, BoolWithProof, PredicateId, PredicateResultsBundle, ReadKey,
        SafetyOrCreation, WriteKey,
    };

    let mut acceptance = BTreeMap::new();
    acceptance.insert(
        PredicateId("acc1".into()),
        BoolWithProof {
            value: false,
            proof_cid: None,
        },
    );
    TypedTx::Work(WorkTx {
        tx_id: TxId("worktx-l4e-test".into()),
        task_id: TaskId("task-test".into()),
        parent_state_root: Default::default(),
        agent_id: AgentId("agent_0".into()),
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

#[test]
fn sorry_block_attempt_routes_to_sorry_blocked_class() {
    // Per preflight §3.5: sorry/forbidden_payload candidates map to
    // SorryBlock outcome → SorryBlocked=8 RejectionClass.
    let (_dir, cas) = fresh_cas();
    let attempt_cid = write_failure_attempt(&cas, AttemptOutcome::SorryBlock, "sorry");
    let tx = fixture_failure_work_tx(attempt_cid);
    let refined =
        refine_rejection_class_via_attempt_telemetry(&cas, &tx, RejectionClass::PredicateFailed);
    assert_eq!(refined, RejectionClass::SorryBlocked);
    assert_eq!(refined as u8, 8);
}

#[test]
fn llm_err_attempt_routes_to_llm_error_class() {
    let (_dir, cas) = fresh_cas();
    let attempt_cid = write_failure_attempt(&cas, AttemptOutcome::LlmErr, "llmerr");
    let tx = fixture_failure_work_tx(attempt_cid);
    let refined =
        refine_rejection_class_via_attempt_telemetry(&cas, &tx, RejectionClass::PredicateFailed);
    assert_eq!(refined, RejectionClass::LlmError);
    assert_eq!(refined as u8, 9);
}

#[test]
fn full_outcome_to_rejection_class_mapping_table() {
    // End-to-end mapping table per preflight §1.2 Design D mapping.
    // The 4 NEW R3 variants (LeanFail / ParseFail / SorryBlock / LlmErr)
    // each route 1:1 to their RejectionClass counterpart (6/7/8/9).
    let (_dir, cas) = fresh_cas();
    let cases = [
        (
            AttemptOutcome::LeanFail,
            RejectionClass::LeanFailed,
            6u8,
            "leanfail",
        ),
        (
            AttemptOutcome::ParseFail,
            RejectionClass::ParseFailed,
            7,
            "parsefail",
        ),
        (
            AttemptOutcome::SorryBlock,
            RejectionClass::SorryBlocked,
            8,
            "sorryblock",
        ),
        (
            AttemptOutcome::LlmErr,
            RejectionClass::LlmError,
            9,
            "llmerr",
        ),
    ];
    for (outcome, expected_rc, expected_u8, tag) in cases {
        let cid = write_failure_attempt(&cas, outcome, tag);
        let tx = fixture_failure_work_tx(cid);
        let refined = refine_rejection_class_via_attempt_telemetry(
            &cas,
            &tx,
            RejectionClass::PredicateFailed,
        );
        assert_eq!(
            refined, expected_rc,
            "{outcome:?} did not route to {expected_rc:?}"
        );
        assert_eq!(refined as u8, expected_u8);
    }
}
