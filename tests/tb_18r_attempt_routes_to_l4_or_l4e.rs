//! TB-18R R3 Integration Test — sequencer's `refine_rejection_class_via_attempt_telemetry`
//! Design D mapping (preflight `handover/ai-direct/TB-18R_R3_STEP_B_admission.md` §3.1).
//!
//! Maps to charter v2 SG-18R.2: every AttemptTelemetry (failure outcome)
//! routes to L4.E with the corresponding fine-grained `RejectionClass`.
//! Verifies:
//! - LeanFail / ParseFail / SorryBlock / LlmErr → RejectionClass 6/7/8/9.
//! - LeanPass on rejection arm → fall-back to PredicateFailed (release mode).
//!   (Cannot test the debug-build panic from integration tests; it would
//!   actually panic. Documented in preflight §3.6.)
//! - Aborted → fall-back to PredicateFailed (R4 territory).
//! - Legacy ProposalTelemetry CID (non-AttemptTelemetry) → fall-back to
//!   PredicateFailed (backward-compat).
//! - Non-PredicateFailed base class (e.g., EscrowMissing) → unchanged.
//! - Non-Work TypedTx variant → unchanged base class.
//!
//! TRACE_MATRIX FC1-N42 (TB-18R R3 NEW witness on the sequencer admission side).

use std::sync::{Arc, RwLock};
use tempfile::TempDir;

use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::rejection_evidence::RejectionClass;
use turingosv4::runtime::attempt_telemetry::{
    write_attempt_telemetry_to_cas, AttemptKind, AttemptOutcome, AttemptTelemetry,
};
use turingosv4::runtime::proposal_telemetry::{
    self, ProposalTelemetry, TokenCounts,
};
use turingosv4::state::q_state::{AgentId, Hash, TxId};
use turingosv4::state::sequencer::refine_rejection_class_via_attempt_telemetry;
use turingosv4::state::typed_tx::{TypedTx, WorkTx};

fn fresh_cas() -> (TempDir, Arc<RwLock<CasStore>>) {
    let dir = TempDir::new().expect("tempdir");
    let cas = Arc::new(RwLock::new(CasStore::open(dir.path()).expect("cas open")));
    (dir, cas)
}

fn ctx_hash(domain: &[u8]) -> Hash {
    Hash(Cid::from_content(domain).0)
}

fn write_failure_attempt(
    cas: &Arc<RwLock<CasStore>>,
    outcome: AttemptOutcome,
    tag: &str,
) -> Cid {
    let attempt = AttemptTelemetry::new_root(
        TxId(format!("att-{tag}")),
        "test-run".into(),
        "task-test".into(),
        AgentId("agent_0".into()),
        "n0.b0".into(),
        ctx_hash(b"ctx"),
        Cid::from_content(format!("candidate-{tag}").as_bytes()),
        AttemptKind::ExternalizedLlmCycle,
        outcome,
        TokenCounts::default(),
        tag.into(),
    );
    let mut cas_w = cas.write().expect("cas write");
    write_attempt_telemetry_to_cas(&mut *cas_w, &attempt, "test", 0).expect("write")
}

fn fixture_work_tx(proposal_cid: Cid) -> TypedTx {
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
        tx_id: TxId("worktx-test".into()),
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
fn lean_fail_attempt_refines_to_lean_failed_rejection_class() {
    let (_dir, cas) = fresh_cas();
    let attempt_cid = write_failure_attempt(&cas, AttemptOutcome::LeanFail, "leanfail");
    let tx = fixture_work_tx(attempt_cid);
    let refined = refine_rejection_class_via_attempt_telemetry(
        &cas,
        &tx,
        RejectionClass::PredicateFailed,
    );
    assert_eq!(refined, RejectionClass::LeanFailed);
}

#[test]
fn parse_fail_attempt_refines_to_parse_failed_rejection_class() {
    let (_dir, cas) = fresh_cas();
    let attempt_cid = write_failure_attempt(&cas, AttemptOutcome::ParseFail, "parsefail");
    let tx = fixture_work_tx(attempt_cid);
    let refined = refine_rejection_class_via_attempt_telemetry(
        &cas,
        &tx,
        RejectionClass::PredicateFailed,
    );
    assert_eq!(refined, RejectionClass::ParseFailed);
}

#[test]
fn legacy_proposal_telemetry_proposal_cid_falls_back_to_predicate_failed() {
    // Pre-TB-18R chains carry WorkTx.proposal_cid → ProposalTelemetry. The
    // R3 helper must NOT misinterpret these as AttemptTelemetry.
    // canonical_decode<AttemptTelemetry> on ProposalTelemetry bytes returns
    // Err(Codec(...)) → fall-back to base class.
    let (_dir, cas) = fresh_cas();
    let pt = {
        let mut cas_w = cas.write().expect("cas write");
        let pt = ProposalTelemetry::build_for_evaluator_append(
            &mut *cas_w,
            "legacy-run",
            "agent_0",
            0,
            b"legacy-payload",
            "complete",
            TokenCounts::default(),
            "legacy-test",
            0,
        )
        .expect("build pt");
        proposal_telemetry::write_to_cas(&mut *cas_w, &pt, "legacy-test", 0)
            .expect("write pt")
    };
    let tx = fixture_work_tx(pt);
    let refined = refine_rejection_class_via_attempt_telemetry(
        &cas,
        &tx,
        RejectionClass::PredicateFailed,
    );
    assert_eq!(
        refined,
        RejectionClass::PredicateFailed,
        "legacy ProposalTelemetry must not refine; falls back to base PredicateFailed"
    );
}

#[test]
fn non_predicate_failed_base_class_unchanged() {
    // Per preflight §3.1: "Refinement only occurs on base_class ==
    // PredicateFailed". EscrowMissing / InsufficientBalance / etc. keep
    // their existing semantics.
    let (_dir, cas) = fresh_cas();
    let attempt_cid = write_failure_attempt(&cas, AttemptOutcome::LeanFail, "nofbase");
    let tx = fixture_work_tx(attempt_cid);
    for base in [
        RejectionClass::EscrowMissing,
        RejectionClass::InsufficientBalance,
        RejectionClass::PolicyViolation,
        RejectionClass::InvariantViolation,
        RejectionClass::MalformedPayload,
    ] {
        let refined = refine_rejection_class_via_attempt_telemetry(&cas, &tx, base);
        assert_eq!(
            refined, base,
            "base_class={base:?} must NOT refine; only PredicateFailed refines"
        );
    }
}

#[test]
fn aborted_outcome_falls_back_to_base_class() {
    // Per preflight §1.2 mapping table: Aborted is R4 TerminalAbortRecord
    // territory; the helper falls back to base class without a panic.
    let (_dir, cas) = fresh_cas();
    let attempt_cid = write_failure_attempt(&cas, AttemptOutcome::Aborted, "aborted");
    let tx = fixture_work_tx(attempt_cid);
    let refined = refine_rejection_class_via_attempt_telemetry(
        &cas,
        &tx,
        RejectionClass::PredicateFailed,
    );
    assert_eq!(refined, RejectionClass::PredicateFailed);
}
