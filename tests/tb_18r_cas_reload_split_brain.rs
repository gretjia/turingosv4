//! TB-18R R3.fix Integration Test — CasStore split-brain reload.
//!
//! Reproduces the L0-smoke-2026-05-06 bug: long-lived CasStore handle has a
//! stale in-memory index wrt entries written by other short-lived handles
//! on the same disk path. Verifies `reload_index_from_sidecar` resolves it
//! and the sequencer's `refine_rejection_class_via_attempt_telemetry` uses
//! the reload retry path to produce the correct fine-grained
//! `RejectionClass`.
//!
//! Maps to TB-18R R3.fix preflight §5 + §6.
//!
//! TRACE_MATRIX FC1-N41 (R3.fix witness on the reload retry path).

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, RwLock};
use tempfile::TempDir;

use turingosv4::bottom_white::cas::schema::{Cid, ObjectType};
use turingosv4::bottom_white::cas::store::{CasError, CasStore};
use turingosv4::bottom_white::ledger::rejection_evidence::RejectionClass;
use turingosv4::economy::money::StakeMicroCoin;
use turingosv4::runtime::attempt_telemetry::{
    write_attempt_telemetry_to_cas, AttemptKind, AttemptOutcome, AttemptTelemetry,
};
use turingosv4::runtime::proposal_telemetry::TokenCounts;
use turingosv4::state::q_state::{AgentId, Hash, TaskId, TxId};
use turingosv4::state::sequencer::refine_rejection_class_via_attempt_telemetry;
use turingosv4::state::typed_tx::{
    AgentSignature, BoolWithProof, PredicateId, PredicateResultsBundle, ReadKey, SafetyOrCreation,
    TypedTx, WorkTx, WriteKey,
};

#[test]
fn cas_store_split_brain_repro_reload_recovers() {
    // Repro the L0 smoke 2026-05-06 condition: handle B opened BEFORE
    // handle A's write; B.get returns CidNotFound; B.reload_index_from_sidecar
    // picks up A's write; B.get succeeds.
    let dir = TempDir::new().expect("tempdir");
    let mut a = CasStore::open(dir.path()).expect("open a");
    let mut b = CasStore::open(dir.path()).expect("open b");
    let payload = b"r3fix-split-brain-payload";
    let cid = a
        .put(payload, ObjectType::Generic, "test-a", 0, None)
        .expect("a.put");

    // B opened before A's write; in-memory index does NOT see it.
    match b.get(&cid) {
        Err(CasError::CidNotFound(_)) => {}
        Err(other) => panic!("expected CidNotFound, got {other:?}"),
        Ok(_) => panic!("B should not see A's write before reload"),
    }

    // R3.fix: reload sidecar; entry now visible.
    b.reload_index_from_sidecar().expect("reload");
    let recovered = b.get(&cid).expect("b.get post-reload");
    assert_eq!(recovered, payload.to_vec());
}

#[test]
fn cas_store_reload_is_idempotent_on_already_loaded_cids() {
    // Calling reload twice with no on-disk changes is a no-op (no error,
    // existing entries unchanged).
    let dir = TempDir::new().expect("tempdir");
    let mut a = CasStore::open(dir.path()).expect("open a");
    let cid = a
        .put(b"idem-payload", ObjectType::Generic, "test-a", 0, None)
        .expect("a.put");

    a.reload_index_from_sidecar().expect("first reload");
    let bytes_after_first = a.get(&cid).expect("get after first reload");
    a.reload_index_from_sidecar().expect("second reload");
    let bytes_after_second = a.get(&cid).expect("get after second reload");
    assert_eq!(bytes_after_first, bytes_after_second);
    assert_eq!(bytes_after_first, b"idem-payload".to_vec());
}

fn write_attempt_via_handle(cas: &mut CasStore, outcome: AttemptOutcome, tag: &str) -> Cid {
    let attempt = AttemptTelemetry::new_root(
        TxId(format!("att-{tag}")),
        "test-run-r3fix".into(),
        "task-test-r3fix".into(),
        AgentId("agent_0".into()),
        "n0.b0".into(),
        Hash(Cid::from_content(b"r3fix-ctx").0),
        Cid::from_content(format!("r3fix-candidate-{tag}").as_bytes()),
        AttemptKind::ExternalizedLlmCycle,
        outcome,
        TokenCounts::default(),
        tag.into(),
    );
    write_attempt_telemetry_to_cas(cas, &attempt, "test", 0).expect("write attempt")
}

fn fixture_failure_work_tx(proposal_cid: Cid) -> TypedTx {
    let mut acceptance = BTreeMap::new();
    acceptance.insert(
        PredicateId("acc1".into()),
        BoolWithProof {
            value: false,
            proof_cid: None,
        },
    );
    TypedTx::Work(WorkTx {
        tx_id: TxId("worktx-r3fix-test".into()),
        task_id: TaskId("task-test-r3fix".into()),
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
fn refine_rejection_class_recovers_via_reload_lean_failed() {
    // The L0-smoke-2026-05-06 scenario, exactly: stale sequencer.cas handle
    // (sequencer_cas) opened BEFORE evaluator-side handle (evaluator_cas)
    // wrote the AttemptTelemetry. Refine helper's first read misses; the
    // R3.fix retry-on-miss path reloads sidecar and recovers the
    // fine-grained class.
    let dir = TempDir::new().expect("tempdir");
    let sequencer_cas = Arc::new(RwLock::new(
        CasStore::open(dir.path()).expect("open sequencer_cas"),
    ));
    let mut evaluator_cas = CasStore::open(dir.path()).expect("open evaluator_cas");

    // Evaluator (separate handle) writes the AttemptTelemetry AFTER the
    // sequencer_cas was opened — split-brain condition replicated.
    let attempt_cid = write_attempt_via_handle(
        &mut evaluator_cas,
        AttemptOutcome::LeanFail,
        "leanfail-stale",
    );
    let tx = fixture_failure_work_tx(attempt_cid);

    // Pre-fix this would return PredicateFailed (the L0 smoke 2026-05-06
    // bug); R3.fix recovers via the reload retry.
    let refined = refine_rejection_class_via_attempt_telemetry(
        &sequencer_cas,
        &tx,
        RejectionClass::PredicateFailed,
    );
    assert_eq!(
        refined,
        RejectionClass::LeanFailed,
        "R3.fix retry-on-miss must recover the fine-grained class even \
         when sequencer.cas in-memory index is stale wrt the evaluator's \
         AttemptTelemetry write"
    );

    // After the helper ran, sequencer_cas's in-memory index should have
    // been reloaded; subsequent direct `cas.read().get(...)` calls should
    // succeed without another reload.
    let cas_g = sequencer_cas.read().expect("read post-fix");
    let bytes = cas_g.get(&tx.work_proposal_cid_for_test()).expect("get");
    assert!(!bytes.is_empty(), "sidecar reload populated index");
}

#[test]
fn refine_rejection_class_falls_back_when_truly_absent_after_reload() {
    // Negative case: a Cid that never resolves to anything in CAS. After
    // reload retry still misses; helper falls back to base_class.
    let dir = TempDir::new().expect("tempdir");
    let sequencer_cas = Arc::new(RwLock::new(
        CasStore::open(dir.path()).expect("open sequencer_cas"),
    ));
    let bogus_cid = Cid::from_content(b"never-written-to-cas-r3fix-negative");
    let tx = fixture_failure_work_tx(bogus_cid);
    let refined = refine_rejection_class_via_attempt_telemetry(
        &sequencer_cas,
        &tx,
        RejectionClass::PredicateFailed,
    );
    assert_eq!(
        refined,
        RejectionClass::PredicateFailed,
        "truly-absent CID must fall back to base_class even after retry"
    );
}

#[test]
fn refine_rejection_class_recovers_sorry_block() {
    // Companion to the LeanFail recovery test: same split-brain pattern,
    // but the AttemptTelemetry has outcome=SorryBlock. Refine must recover
    // RejectionClass::SorryBlocked = 8.
    let dir = TempDir::new().expect("tempdir");
    let sequencer_cas = Arc::new(RwLock::new(
        CasStore::open(dir.path()).expect("open sequencer_cas"),
    ));
    let mut evaluator_cas = CasStore::open(dir.path()).expect("open evaluator_cas");

    let attempt_cid = write_attempt_via_handle(
        &mut evaluator_cas,
        AttemptOutcome::SorryBlock,
        "sorry-stale",
    );
    let tx = fixture_failure_work_tx(attempt_cid);

    let refined = refine_rejection_class_via_attempt_telemetry(
        &sequencer_cas,
        &tx,
        RejectionClass::PredicateFailed,
    );
    assert_eq!(refined, RejectionClass::SorryBlocked);
    assert_eq!(refined as u8, 8);
}

// Test helper: extract the WorkTx.proposal_cid from a TypedTx::Work for the
// post-reload index sanity check above.
trait WorkProposalCidForTest {
    fn work_proposal_cid_for_test(&self) -> Cid;
}
impl WorkProposalCidForTest for TypedTx {
    fn work_proposal_cid_for_test(&self) -> Cid {
        match self {
            TypedTx::Work(w) => w.proposal_cid.clone(),
            _ => panic!("expected TypedTx::Work in test fixture"),
        }
    }
}
