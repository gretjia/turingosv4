//! CONSTITUTION GATE — FC3 OBSERVABLE + CANARY half is LIVE and tape-anchored.
//!
//! TRACE_MATRIX FC3-N33 + FC3-N41 + FC1-N12.
//!
//! ── WHAT THIS GATE PROVES (the OBSERVABLE half only) ─────────────────────
//! Token APPROVE-FC3-OBSERVABLE-CANARY-HALF authorizes the OBSERVABLE +
//! SAFETY-EVALUATION halves of the FC3 self-evolution loop. This gate proves
//! those two halves are live and reconstructable from tape:
//!
//!   (1) PROPOSER EMITS A REAL SPEC: the runtime `fc3_proposer` reads an
//!       accepted `LogFeedbackArchive` row + the live L4.E rejection cluster,
//!       synthesizes a REAL candidate spec, and emits an `ArchitectProposalTx`
//!       whose `ArchitectProposalCapsule` carries that spec — a NON-`Noop`
//!       kind with a populated `target_path` + `proposed_artifact_cid`, NOT the
//!       inert `{proposal_id}` shell (`ToolProposalPayload::default()`) that
//!       the dead role path produces. The proposal is a system-only L4 row,
//!       CAS-backed.
//!
//!   (2) CANARY WRITES A REAL METRIC: the runtime `fc3_canary` scores the
//!       candidate against the EXISTING executable `Predicate` (ground-truth
//!       evaluation) and writes an integer-only `MetricEstimate` to CAS as a
//!       `MetricEstimateCapsule`, recoverable from tape by schema id.
//!
//!   (3) THE LOOP DOES NOT CLOSE: the canary terminal disposition is
//!       `"sandbox:canary_only"`. This gate asserts that status does NOT close
//!       the FC3 loop (no re-init / trust-root recompute / commit activation).
//!       This is the INVERSE of the standing-pending G5 leg (B) — the
//!       irreversible-commit path remains §8 Class-4 territory and is NOT
//!       implemented here.
//!
//! ── WHAT THIS GATE DELIBERATELY DOES NOT ASSERT ──────────────────────────
//! It does NOT require the FC3 loop to close. `tests/pending/
//! constitution_fc3_meta_loop_closure.rs` (G5) stays RED/standing-pending: this
//! observable-half work flips neither of its two RED observations (the inert
//! `ToolProposalPayload::default()` shell, nor the `sandbox:canary_only`
//! Accept-terminal). Promotion of G5 still requires per-atom §8 ratification of
//! the irreversible-commit / trust-root-recompute leg.

use std::sync::{Arc, RwLock};

use tempfile::TempDir;

use turingosv4::bottom_white::cas::schema::{Cid, ObjectType};
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::rejection_evidence::{
    RejectionClass, RejectionEvidenceWriter,
};
use turingosv4::bottom_white::ledger::system_keypair::{
    Ed25519Keypair, PinnedSystemPubkeys, SystemEpoch,
};
use turingosv4::bottom_white::ledger::transition_ledger::{
    canonical_decode, canonical_encode, cas_metadata_root_before_logical_t,
    constitution_source_hash, InMemoryLedgerWriter, LedgerEntry, LedgerWriter, TxKind,
};
use turingosv4::bottom_white::tools::registry::ToolRegistry;
use turingosv4::runtime::real5_roles::fc3_canary::{
    closes_fc3_loop, run_canary, CanaryRequest, CANARY_ONLY_TERMINAL_STATUS,
    FC3_METRIC_ESTIMATE_SCHEMA_ID,
};
use turingosv4::runtime::real5_roles::fc3_proposer::{
    synthesize_proposal, FC3_CANDIDATE_ARTIFACT_SCHEMA_ID, FC3_CANDIDATE_SPEC_SCHEMA_ID,
};
use turingosv4::state::q_state::{AgentId, Hash, QState, TxId};
use turingosv4::state::sequencer::{Sequencer, SubmissionEnvelope, SystemEmitCommand};
use turingosv4::state::typed_tx::{
    ArchitectFeedbackCapsule, ArchitectProposalCapsule, ArchitectProposalKind,
    LogFeedbackArchiveTx, PredicateId, TypedTx, VetoVerdict, ARCHITECT_FEEDBACK_SCHEMA_ID,
};
use turingosv4::top_white::predicates::registry::{BootPredicateManifest, PredicateRegistry};

struct Harness {
    _tmp: TempDir,
    cas: Arc<RwLock<CasStore>>,
    writer: Arc<RwLock<dyn LedgerWriter>>,
    rejections: Arc<RwLock<RejectionEvidenceWriter>>,
    seq: Arc<Sequencer>,
    rx: tokio::sync::mpsc::Receiver<SubmissionEnvelope>,
}

fn harness() -> Harness {
    let tmp = TempDir::new().expect("tempdir");
    let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas")));
    let keypair = Arc::new(Ed25519Keypair::generate_with_secure_entropy().expect("keypair"));
    let epoch = SystemEpoch::new(1);
    let writer: Arc<RwLock<dyn LedgerWriter>> = Arc::new(RwLock::new(InMemoryLedgerWriter::new()));
    let rejections = Arc::new(RwLock::new(RejectionEvidenceWriter::default()));
    let mut pinned_map = PinnedSystemPubkeys::new();
    pinned_map.insert(epoch, keypair.public_key());
    let pinned = Arc::new(pinned_map);
    // v8 production predicate catalog — the canary scores candidates against the
    // EXISTING executable `acc1` (ProposalPayloadNotEmpty) predicate.
    let registry = PredicateRegistry::from_boot_manifest(BootPredicateManifest::v8_production())
        .expect("v8 predicate registry");
    let (seq, rx) = Sequencer::new(
        Arc::clone(&cas),
        keypair,
        epoch,
        Arc::clone(&writer),
        Arc::clone(&rejections),
        Arc::new(registry),
        Arc::new(ToolRegistry::new()),
        pinned,
        QState::default(),
        16,
    );
    Harness {
        _tmp: tmp,
        cas,
        writer,
        rejections,
        seq: Arc::new(seq),
        rx,
    }
}

fn decode_entry_tx(h: &Harness, entry: &LedgerEntry) -> TypedTx {
    let cas = h.cas.read().expect("cas read");
    let bytes = cas.get(&entry.tx_payload_cid).expect("entry payload");
    canonical_decode(&bytes).expect("typed tx decode")
}

/// Write a real feedback capsule and return its CID (mirrors the FC3 governance
/// binary / closure gate `put_feedback_capsule`).
fn put_feedback_capsule(h: &Harness) -> Cid {
    let logical_t = h.seq.next_logical_t_peek() + 1;
    let q = h.seq.q_snapshot().expect("q snapshot");
    let l4e = h.rejections.read().expect("l4e read");
    let cas_root = {
        let cas = h.cas.read().expect("cas read");
        cas_metadata_root_before_logical_t(&cas, logical_t).expect("cas root")
    };
    let capsule = ArchitectFeedbackCapsule {
        schema_version: ARCHITECT_FEEDBACK_SCHEMA_ID.to_string(),
        source_ledger_root: q.ledger_root_t,
        source_l4e_root: l4e.last_hash(),
        cas_metadata_root: cas_root,
        constitution_hash: constitution_source_hash(),
        public_summary: "fc3 runtime logs feedback (proposer/canary observable half)".to_string(),
        private_detail_cid: None,
    };
    drop(l4e);
    let bytes = canonical_encode(&capsule).expect("capsule encode");
    h.cas
        .write()
        .expect("cas write")
        .put(
            &bytes,
            ObjectType::Generic,
            "constitution_fc3_proposer_canary_observable",
            logical_t.saturating_sub(1),
            Some(ARCHITECT_FEEDBACK_SCHEMA_ID.to_string()),
        )
        .expect("put feedback capsule")
}

/// Seed a non-trivial L4.E rejection cluster so the proposer synthesizes a
/// non-`Noop`, actionable candidate spec.
fn seed_rejection_cluster(h: &Harness) {
    let mut rej = h.rejections.write().expect("l4e write");
    let stub_cid = Cid::from_content(b"fc3-observable-rejection-stub");
    for (i, class) in [
        RejectionClass::CheckerFailed,
        RejectionClass::CheckerFailed,
        RejectionClass::ParseFailed,
    ]
    .into_iter()
    .enumerate()
    {
        rej.append_rejected(
            i as u64,
            Hash([0u8; 32]),
            AgentId("fc3-solver".to_string()),
            TxKind::Work,
            stub_cid,
            class,
            None,
            None,
        );
    }
}

/// Drive feedback -> proposal via the runtime proposer + the existing
/// `SystemEmitCommand::ArchitectProposal` path. Returns the accepted
/// proposal tx, the proposal entry, and the on-chain capsule.
async fn run_proposer(
    h: &mut Harness,
) -> (
    LogFeedbackArchiveTx,
    LedgerEntry,
    ArchitectProposalCapsule,
    Cid,
) {
    // 1. accepted feedback row.
    let feedback_capsule_cid = put_feedback_capsule(h);
    h.seq
        .emit_system_tx(SystemEmitCommand::LogFeedbackArchive {
            feedback_capsule_cid,
            veto_verdict: VetoVerdict::Pass,
        })
        .await
        .expect("emit feedback");
    let feedback_entry = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("apply feedback")
        .expect("feedback accepts");
    let TypedTx::LogFeedbackArchive(feedback_tx) = decode_entry_tx(h, &feedback_entry) else {
        panic!("expected feedback tx")
    };

    // 2. live failure cluster.
    seed_rejection_cluster(h);

    // 3. runtime proposer synthesizes a REAL candidate spec + capsule in CAS.
    let tool_registry_root = h.seq.q_snapshot().expect("q snapshot").tool_registry_root_t;
    let synthesized = {
        let mut cas = h.cas.write().expect("cas write");
        let rej = h.rejections.read().expect("l4e read");
        synthesize_proposal(
            &mut cas,
            &feedback_tx,
            feedback_capsule_cid,
            &rej,
            tool_registry_root,
            b"fc3 runtime candidate artifact: tool-registry-safe patch bytes",
            h.seq.next_logical_t_peek(),
        )
        .expect("synthesize proposal")
    };

    // The synthesized spec MUST be actionable (not the empty shell).
    assert!(
        synthesized.spec.is_actionable(),
        "proposer produced a non-actionable (empty-shell-equivalent) spec"
    );

    // 4. emit via the EXISTING ArchitectProposal path (system-only).
    h.seq
        .emit_system_tx(synthesized.emit_command(&feedback_tx))
        .await
        .expect("emit architect proposal");
    let proposal_entry = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("apply proposal")
        .expect("proposal accepts");
    assert_eq!(proposal_entry.tx_kind, TxKind::ArchitectProposal);

    (
        feedback_tx,
        proposal_entry,
        synthesized.proposal_capsule,
        synthesized.candidate_artifact_cid,
    )
}

/// GATE (1) — the proposer emits a tape-anchored ArchitectProposal whose capsule
/// carries a REAL, non-empty spec (NOT the `{proposal_id}` shell).
#[tokio::test]
async fn fc3_proposer_emits_real_spec_on_tape() {
    let mut h = harness();
    let (feedback_tx, proposal_entry, capsule, candidate_artifact_cid) = run_proposer(&mut h).await;

    // The proposal is a system-only L4 row anchored on the ChainTape.
    assert_eq!(proposal_entry.tx_kind, TxKind::ArchitectProposal);
    let TypedTx::ArchitectProposal(proposal_tx) = decode_entry_tx(&h, &proposal_entry) else {
        panic!("expected ArchitectProposal tx")
    };
    assert_eq!(proposal_tx.feedback_tx_id, feedback_tx.tx_id);

    // The on-chain capsule is reconstructable from CAS and carries the REAL
    // spec — NOT the inert `Noop`/empty shell.
    let cas = h.cas.read().expect("cas read");
    let bytes = cas
        .get(&proposal_tx.proposal_capsule_cid)
        .expect("proposal capsule bytes");
    let on_chain: ArchitectProposalCapsule =
        canonical_decode(&bytes).expect("decode proposal capsule");
    assert_eq!(on_chain, capsule, "on-chain capsule must equal synthesized");

    assert_ne!(
        on_chain.proposal_kind,
        ArchitectProposalKind::Noop,
        "proposer must emit a non-Noop (actionable) proposal kind"
    );
    assert!(
        on_chain.target_path.is_some(),
        "proposer must populate target_path (real touched path)"
    );
    assert_eq!(
        on_chain.proposed_artifact_cid,
        Some(candidate_artifact_cid),
        "proposer must reference a real candidate artifact CID"
    );
    assert!(
        !on_chain.tools_used.is_empty(),
        "proposer must record tools_used"
    );
    // public_summary carries the required_gates + rollback_plan text.
    assert!(
        on_chain.public_summary.contains("rollback_plan")
            && on_chain.public_summary.contains("required_gates"),
        "proposer public_summary must carry rollback_plan + required_gates"
    );

    // The candidate artifact + structured spec sidecar are both tape-anchored.
    let artifact_meta = cas
        .metadata(&candidate_artifact_cid)
        .expect("candidate artifact metadata");
    assert_eq!(
        artifact_meta.schema_id.as_deref(),
        Some(FC3_CANDIDATE_ARTIFACT_SCHEMA_ID)
    );
    let spec_cids = cas.list_cids_by_schema_id(FC3_CANDIDATE_SPEC_SCHEMA_ID);
    assert_eq!(
        spec_cids.len(),
        1,
        "exactly one structured candidate spec must be anchored in CAS"
    );
}

/// GATE (2) + (3) — the canary writes a tape-anchored MetricEstimate scored via
/// the real Predicate, and the loop does NOT close.
#[tokio::test]
async fn fc3_canary_writes_metric_and_loop_stays_open() {
    let mut h = harness();
    let (_feedback_tx, _proposal_entry, _capsule, candidate_artifact_cid) =
        run_proposer(&mut h).await;

    // Run the canary against the real executable `acc1` predicate
    // (ProposalPayloadNotEmpty) — the candidate artifact bytes are non-empty,
    // so the predicate PASSES and the metric is +1/1.
    let registry = PredicateRegistry::from_boot_manifest(BootPredicateManifest::v8_production())
        .expect("v8 predicate registry");
    let registry_root = registry.merkle_root_hash();
    let request = CanaryRequest {
        candidate_artifact_cid,
        predicate_id: PredicateId("acc1".to_string()),
        metric_label: "fc3.canary.expected_error_reduction".to_string(),
    };
    let outcome = {
        let mut cas = h.cas.write().expect("cas write");
        run_canary(
            &mut cas,
            &registry,
            registry_root,
            &request,
            h.seq.next_logical_t_peek(),
        )
        .expect("run canary")
    };

    // The metric is integer-only and reflects a real predicate verdict.
    assert!(
        outcome.capsule.predicate_passed,
        "acc1 over non-empty candidate bytes must PASS"
    );
    assert_eq!(outcome.metric.numerator_delta, 1);
    assert_eq!(outcome.metric.denominator, 1);
    assert_eq!(
        outcome.capsule.predicate_id,
        PredicateId("acc1".to_string())
    );

    // The MetricEstimate evidence capsule is tape-anchored and reconstructable.
    let cas = h.cas.read().expect("cas read");
    let metric_cids = cas.list_cids_by_schema_id(FC3_METRIC_ESTIMATE_SCHEMA_ID);
    assert_eq!(
        metric_cids.len(),
        1,
        "exactly one MetricEstimate capsule must be anchored in CAS"
    );
    assert!(
        metric_cids.contains(&outcome.metric_capsule_cid),
        "canary outcome cid must be the anchored metric capsule"
    );
    let bytes = cas
        .get(&outcome.metric_capsule_cid)
        .expect("metric capsule bytes");
    let reconstructed: turingosv4::runtime::real5_roles::fc3_canary::MetricEstimateCapsule =
        canonical_decode(&bytes).expect("decode metric capsule");
    assert_eq!(reconstructed, outcome.capsule);

    // (3) THE LOOP DOES NOT CLOSE — terminal status is the sandbox-canary
    // dead-end; it must NOT be a loop-closing re-init / commit. This is the
    // inverse of the standing-pending G5 leg (B).
    assert_eq!(
        outcome.capsule.terminal_status, CANARY_ONLY_TERMINAL_STATUS,
        "canary terminal status must be the sandbox-canary dead-end"
    );
    assert!(
        !closes_fc3_loop(&outcome.capsule.terminal_status),
        "OBSERVABLE-HALF SCOPE VIOLATION: the canary terminal status closes the \
         FC3 loop (re-init / trust-root recompute / commit). That is the \
         FORBIDDEN irreversible leg and is NOT authorized by token \
         APPROVE-FC3-OBSERVABLE-CANARY-HALF. The loop must stay open."
    );
    assert!(
        outcome.loop_stays_open(),
        "canary outcome must structurally keep the FC3 loop open"
    );
}

/// GUARD — the proposer NEVER emits a Veto/Commit/Reinit on the observable
/// path. Only feedback + proposal rows reach the ChainTape; nothing downstream
/// of ArchitectProposal is constructed by this half.
#[tokio::test]
async fn fc3_observable_half_emits_no_irreversible_tx() {
    let mut h = harness();
    let _ = run_proposer(&mut h).await;

    let writer = h.writer.read().expect("writer read");
    let kinds: Vec<TxKind> = (1..=writer.len())
        .map(|t| writer.read_at(t).expect("read_at").tx_kind)
        .collect();

    for kind in &kinds {
        assert!(
            !matches!(
                kind,
                TxKind::VetoDecision
                    | TxKind::ArchitectCommit
                    | TxKind::ReinitRequest
                    | TxKind::ReinitBoot
            ),
            "OBSERVABLE-HALF SCOPE VIOLATION: an irreversible FC3 tx ({kind:?}) \
             reached the ChainTape. The observable+canary half must emit only \
             LogFeedbackArchive + ArchitectProposal; Veto/Commit/Reinit are the \
             FORBIDDEN leg."
        );
    }
    // The proposal row is present (sanity: we actually emitted the observable half).
    assert!(
        kinds.contains(&TxKind::ArchitectProposal),
        "observable half must emit an ArchitectProposal row"
    );
}

/// META-GUARD — this gate must NOT accidentally close the G5 loop. Asserts the
/// canary's loop-closure predicate agrees with the pending gate's: the canary
/// terminal status is NOT a closing status. (Defends against a future refactor
/// silently turning `sandbox:canary_only` into a re-init token, which would make
/// this gate falsely green while smuggling in the irreversible leg.)
#[test]
fn fc3_canary_terminal_status_is_not_loop_closing() {
    assert_eq!(CANARY_ONLY_TERMINAL_STATUS, "sandbox:canary_only");
    assert!(!closes_fc3_loop(CANARY_ONLY_TERMINAL_STATUS));
    // The inverse must hold for the known closing tokens (non-vacuous probe).
    assert!(closes_fc3_loop("reinit:committed"));
    assert!(closes_fc3_loop("re-init"));
}

/// NON-VACUOUS NEGATIVE CONTROL — a deliberately empty (Noop) candidate must be
/// REFUSED by the proposer, proving the actionable-spec assertion above is a
/// real gate and not always-true. The proposer refuses to dress an empty shell
/// as a real proposal (`SpecNotActionable`).
#[tokio::test]
async fn fc3_proposer_refuses_empty_noop_shell() {
    let mut h = harness();

    let feedback_capsule_cid = put_feedback_capsule(&h);
    h.seq
        .emit_system_tx(SystemEmitCommand::LogFeedbackArchive {
            feedback_capsule_cid,
            veto_verdict: VetoVerdict::Pass,
        })
        .await
        .expect("emit feedback");
    let feedback_entry = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("apply feedback")
        .expect("feedback accepts");
    let TypedTx::LogFeedbackArchive(feedback_tx) = decode_entry_tx(&h, &feedback_entry) else {
        panic!("expected feedback tx")
    };

    // NO rejection cluster seeded -> empty failure signal -> Noop -> refused.
    let tool_registry_root = h.seq.q_snapshot().expect("q snapshot").tool_registry_root_t;
    let mut cas = h.cas.write().expect("cas write");
    let rej = h.rejections.read().expect("l4e read");
    let result = synthesize_proposal(
        &mut cas,
        &feedback_tx,
        feedback_capsule_cid,
        &rej,
        tool_registry_root,
        b"unused candidate bytes for noop refusal control",
        h.seq.next_logical_t_peek(),
    );
    assert!(
        result.is_err(),
        "proposer must REFUSE to emit a Noop/empty-shell proposal when there is \
         no actionable failure signal (proves the actionable-spec gate is real)"
    );
}

// Reference the feedback default constructor so the irreversible-leg-free import
// set stays honest (LogFeedbackArchiveTx is the proposer's input edge type).
#[allow(dead_code)]
fn _input_edge_type() -> TxId {
    LogFeedbackArchiveTx::default().tx_id
}
