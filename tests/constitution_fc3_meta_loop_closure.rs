//! CONSTITUTION GATE — FC3 meta-architecture loop ACTUALLY CLOSES (the
//! IRREVERSIBLE leg is LIVE and tape/sandbox-anchored).
//!
//! TRACE_MATRIX FC3-N32/N43 (Veto-AI) + FC3-N44/N45 (commit + re-init).
//!
//! ── PROVENANCE ───────────────────────────────────────────────────────────
//! Promoted from `tests/pending/constitution_fc3_meta_loop_closure.rs` (the G5
//! standing-pending gate) under the §8 Class-4 token
//! `APPROVE-FC3-RUNTIME-VETO-AND-TRUSTROOT-REINIT`. Pre-promotion this gate was
//! RED by design: (A) the live role-path proposal carried no real spec, and
//! (B) the only Accept terminal was the dead-end `"sandbox:canary_only"`. Both
//! observations now flip GREEN because the FC3 runtime engine landed:
//!   * `fc3_proposer` synthesizes a REAL `CandidateSpec` and emits a non-`Noop`
//!     `ArchitectProposalCapsule` (the observable half, already live), and
//!   * the runtime Veto-AI clause-walker (`fc3_veto`) + the PASS-gated SANDBOX
//!     trust-root recompute + re-init driver (`fc3_commit_reinit`) close the
//!     loop to a tape-visible `"reinit:committed"` terminal.
//!
//! ── WHAT THIS GATE PROVES (both observations now GREEN) ───────────────────
//!   (A) PROPOSER CARRIES A REAL SPEC: the live `fc3_proposer` path synthesizes
//!       an ACTIONABLE `CandidateSpec` (a real artifact CID + touched path +
//!       intended FC edge) and projects it onto an `ArchitectProposalCapsule`
//!       with a non-`Noop` kind + populated `target_path` + `proposed_artifact_cid`
//!       — NOT the inert `ToolProposalPayload::default()` shell.
//!   (B) LOOP CLOSES PAST CANARY: a runtime Veto-AI PASS over that real proposal
//!       drives the `fc3_commit_reinit` SANDBOX activation, whose terminal status
//!       is `committed_reinit_activation_status(Accept) == "reinit:committed"` —
//!       a loop-closing re-init, NOT the `"sandbox:canary_only"` dead-end. The
//!       SANDBOX trust-root recompute verifies via the SOLE `boot::verify_trust_root`
//!       against a TEMP-DIR manifest (never the real `genesis_payload.toml`), the
//!       constitution-bound hash is asserted UNCHANGED (constitution out of
//!       range, Art. V.1.1), and the recompute records a reversible prior
//!       snapshot (Art. V.2).
//!
//! ── HARD GUARDS ASSERTED ─────────────────────────────────────────────────
//!   * The runtime Veto-AI is deterministic + whitelisted to constitutionality
//!     ({Accept,Reject} only); a constitution.md-touching candidate is REJECTED
//!     and the constitution hash stays unchanged (G-GUARD-1/2).
//!   * The SANDBOX recompute reuses `boot::verify_trust_root` (no second
//!     verifier) and never writes the real boot manifest (G-GUARD-4).
//!   * Reversibility: the recompute records the prior trust-root + prior Q so a
//!     rollback to Q_{t-1} is a tape op (G-GUARD-3 / Art. V.2).
//!
//! ── NON-VACUITY ──────────────────────────────────────────────────────────
//! Negative controls keep the GREEN assertions honest:
//!   * a Veto-AI REJECT (constitution.md in range) yields NO commit and a
//!     non-loop-closing terminal (the loop stays open on Reject);
//!   * the canary's own `sandbox:canary_only` terminal still does NOT close the
//!     loop (the observable half is untouched).

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
use turingosv4::runtime::real5_roles::fc3_canary::closes_fc3_loop;
use turingosv4::runtime::real5_roles::fc3_commit_reinit::{
    activate_sandbox_on_pass, recompute_sandbox_trust_root, runtime_verdict_to_typed,
    CandidatePayloadFile, CommitReinitError,
};
use turingosv4::runtime::real5_roles::fc3_proposer::synthesize_proposal;
use turingosv4::runtime::real5_roles::fc3_veto::{veto_walk, veto_walk_live};
use turingosv4::runtime::real5_roles::{
    committed_reinit_activation_status, VetoVerdict, COMMITTED_REINIT_TERMINAL_STATUS,
};
use turingosv4::state::q_state::{AgentId, Hash, QState, TxId};
use turingosv4::state::sequencer::{Sequencer, SubmissionEnvelope, SystemEmitCommand};
use turingosv4::state::typed_tx::{
    ArchitectFeedbackCapsule, ArchitectProposalCapsule, ArchitectProposalKind,
    LogFeedbackArchiveTx, VetoVerdict as TypedVetoVerdict, ARCHITECT_FEEDBACK_SCHEMA_ID,
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

fn decode_entry_tx(h: &Harness, entry: &LedgerEntry) -> turingosv4::state::typed_tx::TypedTx {
    let cas = h.cas.read().expect("cas read");
    let bytes = cas.get(&entry.tx_payload_cid).expect("entry payload");
    canonical_decode(&bytes).expect("typed tx decode")
}

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
        public_summary: "fc3 runtime logs feedback (irreversible-leg closure gate)".to_string(),
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
            "constitution_fc3_meta_loop_closure",
            logical_t.saturating_sub(1),
            Some(ARCHITECT_FEEDBACK_SCHEMA_ID.to_string()),
        )
        .expect("put feedback capsule")
}

fn seed_rejection_cluster(h: &Harness) {
    let mut rej = h.rejections.write().expect("l4e write");
    let stub_cid = Cid::from_content(b"fc3-closure-rejection-stub");
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

/// Drive feedback -> a REAL proposal via the live `fc3_proposer` engine + the
/// EXISTING `SystemEmitCommand::ArchitectProposal` path. Returns the on-chain
/// capsule + the candidate artifact CID (observation (A) input).
async fn run_proposer(h: &mut Harness) -> (ArchitectProposalCapsule, Cid) {
    let feedback_capsule_cid = put_feedback_capsule(h);
    h.seq
        .emit_system_tx(SystemEmitCommand::LogFeedbackArchive {
            feedback_capsule_cid,
            veto_verdict: TypedVetoVerdict::Pass,
        })
        .await
        .expect("emit feedback");
    let feedback_entry = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("apply feedback")
        .expect("feedback accepts");
    let turingosv4::state::typed_tx::TypedTx::LogFeedbackArchive(feedback_tx) =
        decode_entry_tx(h, &feedback_entry)
    else {
        panic!("expected feedback tx")
    };

    seed_rejection_cluster(h);

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
        synthesized.proposal_capsule,
        synthesized.candidate_artifact_cid,
    )
}

/// G5 (A) — the live `fc3_proposer` path carries a REAL spec on tape: a non-Noop
/// `ArchitectProposalCapsule` with a populated `target_path` + a real candidate
/// artifact CID — NOT the inert `ToolProposalPayload::default()` shell.
#[tokio::test]
async fn fc3_proposer_carries_real_spec() {
    let mut h = harness();
    let (capsule, candidate_artifact_cid) = run_proposer(&mut h).await;

    assert_ne!(
        capsule.proposal_kind,
        ArchitectProposalKind::Noop,
        "the live proposer must carry a non-Noop (actionable) kind, not the empty shell"
    );
    assert!(
        capsule
            .target_path
            .as_deref()
            .map(|p| !p.is_empty())
            .unwrap_or(false),
        "the live proposer must populate a real touched path"
    );
    assert_eq!(
        capsule.proposed_artifact_cid,
        Some(candidate_artifact_cid),
        "the live proposer must reference a real candidate artifact CID"
    );
    // The capsule binds the live axiom (constitution out of range starts here).
    assert_eq!(
        capsule.constitution_hash,
        constitution_source_hash(),
        "the proposer capsule must bind the live constitution axiom hash"
    );
}

/// G5 (B) — a runtime Veto-AI PASS over the REAL proposal closes the FC3 loop to
/// a tape-visible `"reinit:committed"` terminal, via a SANDBOX trust-root
/// recompute verified by the SOLE `boot::verify_trust_root` against a TEMP-DIR
/// manifest (never the real `genesis_payload.toml`). The constitution-bound hash
/// is UNCHANGED; the recompute is reversible to Q_{t-1}.
#[tokio::test]
async fn fc3_meta_loop_closes_with_committed_reinit() {
    let mut h = harness();
    let (capsule, candidate_artifact_cid) = run_proposer(&mut h).await;

    // 1. The runtime Veto-AI walks constitutionality clauses over the REAL
    //    capsule and PASSES (the proposer bound the live axiom + a real path +
    //    a real artifact + a committable kind).
    let outcome = veto_walk_live(&capsule);
    assert_eq!(
        outcome.verdict,
        VetoVerdict::Accept,
        "deterministic Veto-AI must PASS a constitutionally-clean real proposal (reason: {})",
        outcome.reason
    );
    // Projection onto the existing typed-tx verdict is a Pass (no schema change).
    assert_eq!(
        runtime_verdict_to_typed(outcome.verdict),
        TypedVetoVerdict::Pass
    );

    // 2. The PASS drives the SANDBOX activation: trust-root recompute in a TEMP
    //    dir + the loop-closing terminal.
    let constitution_before = constitution_source_hash();
    // G-GUARD-4 (G) SANDBOX-ONLY: capture the REAL boot manifest bytes BEFORE the
    // activation so we can prove the closing leg never touched it. The real
    // genesis_payload.toml lives at the repo root; CARGO_MANIFEST_DIR is the repo
    // root for an integration test.
    let real_manifest =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("genesis_payload.toml");
    let real_manifest_before =
        std::fs::read(&real_manifest).expect("read real genesis_payload.toml before activation");
    let prior_q = h.seq.q_snapshot().expect("q snapshot");
    let sandbox = TempDir::new().expect("sandbox tempdir");
    // The candidate touches a real ArchitectAI-range payload path — NOT constitution.md.
    let candidate_files = vec![CandidatePayloadFile {
        rel_path: "src/bottom_white/tools/runtime_meta_tool.rs".to_string(),
        bytes: b"fc3 runtime candidate artifact: tool-registry-safe patch bytes".to_vec(),
    }];
    let activation =
        activate_sandbox_on_pass(outcome.verdict, &candidate_files, sandbox.path(), &prior_q)
            .expect("sandbox activation on PASS");

    // (B) THE LOOP CLOSES — the terminal status is the loop-closing re-init token,
    // NOT the sandbox-canary dead-end.
    assert_eq!(
        activation.terminal_status, COMMITTED_REINIT_TERMINAL_STATUS,
        "PASS activation must reach the committed-reinit terminal"
    );
    assert!(
        closes_fc3_loop(activation.terminal_status),
        "the committed-reinit terminal must CLOSE the FC3 loop"
    );
    assert!(
        activation.closes_loop(),
        "the activation outcome must structurally close the FC3 loop"
    );
    assert_ne!(
        activation.terminal_status, "sandbox:canary_only",
        "the closing terminal must NOT be the canary dead-end (loop actually closes)"
    );

    // G-GUARD-2: constitution-bound hash UNCHANGED while the sandbox payload
    // manifest hash ADVANCED.
    assert_eq!(
        constitution_source_hash(),
        constitution_before,
        "constitution-bound hash must be unchanged across the recompute (out of range)"
    );
    assert!(
        activation
            .recompute
            .constitution_unchanged_and_manifest_advanced(),
        "the sandbox manifest hash must advance while the constitution hash stays fixed"
    );

    // G-GUARD-4: the SANDBOX recompute verified its TEMP-DIR manifest, NOT the
    // real boot manifest. The temp dir holds a genesis_payload.toml; the real
    // repo's is untouched (it was never passed in).
    assert!(
        sandbox.path().join("genesis_payload.toml").exists(),
        "the recompute must write the SANDBOX manifest into the temp dir"
    );
    // (G) SANDBOX-ONLY, byte-exact: the REAL repo genesis_payload.toml is
    // BYTE-UNCHANGED by this test run. The closing leg recomputes only into the
    // temp dir; the live boot manifest is never rewritten or re-init'd.
    assert!(
        sandbox.path() != real_manifest.parent().unwrap(),
        "the sandbox dir must NOT be the real repo root (no in-place rewrite)"
    );
    let real_manifest_after =
        std::fs::read(&real_manifest).expect("read real genesis_payload.toml after activation");
    assert_eq!(
        real_manifest_after, real_manifest_before,
        "the real genesis_payload.toml MUST be byte-unchanged by the closing leg (G-GUARD-4 sandbox only)"
    );

    // G-GUARD-3 / Art. V.2: the recompute recorded a reversible prior snapshot
    // (prior trust-root + prior Q) so rollback to Q_{t-1} is a tape op.
    assert!(
        activation.recompute.prior.is_reversible(),
        "the activation must record a reversible prior snapshot (rollback to Q_t-1)"
    );
    assert_eq!(
        activation.recompute.prior.prior_state_root, prior_q.state_root_t,
        "the reversibility snapshot must capture the prior Q state root"
    );

    // candidate_artifact_cid is the proposer's CAS artifact; the SANDBOX payload
    // is the same byte content (reconstructable linkage).
    let cas = h.cas.read().expect("cas read");
    assert!(
        cas.metadata(&candidate_artifact_cid).is_some(),
        "the candidate artifact must remain tape-anchored in CAS"
    );
}

/// NON-VACUOUS NEGATIVE CONTROL #1 — a Veto-AI REJECT (constitution.md in range)
/// yields NO commit and a NON-loop-closing terminal. Proves the GREEN closure is
/// gated on a real PASS, not always-true.
#[test]
fn fc3_reject_does_not_close_loop() {
    // A candidate that names constitution.md is REJECTED by the deterministic
    // walker (Art. V.1.1 — human sudo only).
    let bad_capsule = ArchitectProposalCapsule {
        schema_version: "x".to_string(),
        feedback_tx_id: TxId("f".into()),
        feedback_root: Hash([0u8; 32]),
        constitution_hash: constitution_source_hash(),
        tool_registry_root: Hash([0u8; 32]),
        proposal_kind: ArchitectProposalKind::ToolRegistryPatch,
        target_path: Some("constitution.md".to_string()),
        proposed_artifact_cid: Some(Cid([1u8; 32])),
        tools_used: vec!["x".to_string()],
        public_summary: "candidate proposing a forbidden constitution edit".to_string(),
    };
    let outcome = veto_walk(&bad_capsule, constitution_source_hash());
    assert_eq!(
        outcome.verdict,
        VetoVerdict::Reject,
        "a constitution.md-touching candidate MUST be REJECTED (Art. V.1.1)"
    );

    // The rejected terminal must NOT close the loop.
    let terminal = committed_reinit_activation_status(outcome.verdict);
    assert!(
        !closes_fc3_loop(terminal),
        "a Reject terminal must NOT close the FC3 loop"
    );

    // The sandbox activation refuses outright on a non-Accept verdict.
    let sandbox = TempDir::new().expect("sandbox tempdir");
    let err = activate_sandbox_on_pass(
        outcome.verdict,
        &[CandidatePayloadFile {
            rel_path: "src/bottom_white/tools/runtime_meta_tool.rs".to_string(),
            bytes: b"unused".to_vec(),
        }],
        sandbox.path(),
        &QState::default(),
    )
    .expect_err("Reject must refuse the activation");
    assert_eq!(err, CommitReinitError::VetoNotAccepted);
    // No sandbox manifest was written on the refused path.
    assert!(!sandbox.path().join("genesis_payload.toml").exists());
}

/// NON-VACUOUS NEGATIVE CONTROL #2 — the SANDBOX recompute itself REFUSES a
/// candidate that names constitution.md (G-GUARD-2 enforced at the recompute
/// layer too, defense-in-depth), proving constitution.md is out of range even if
/// a verdict were mis-driven.
#[test]
fn fc3_sandbox_refuses_constitution_in_range() {
    let sandbox = TempDir::new().expect("sandbox tempdir");
    let err = recompute_sandbox_trust_root(
        sandbox.path(),
        &[CandidatePayloadFile {
            rel_path: "constitution.md".to_string(),
            bytes: b"malicious constitution rewrite".to_vec(),
        }],
        &QState::default(),
    )
    .expect_err("recompute must refuse constitution.md in range");
    assert_eq!(err, CommitReinitError::ConstitutionInRange);
    // The constitution-bound hash is untouched.
    assert!(!sandbox.path().join("genesis_payload.toml").exists());
}

/// NON-VACUOUS CONTROL #3 — the canary's own terminal still does NOT close the
/// loop (the observable half is untouched by this leg). Defends against a
/// refactor that silently turns the canary dead-end into a closing token.
#[test]
fn fc3_canary_terminal_still_not_closing() {
    assert!(!closes_fc3_loop("sandbox:canary_only"));
    // ...while the new committed-reinit token DOES close (non-vacuous probe).
    assert!(closes_fc3_loop(COMMITTED_REINIT_TERMINAL_STATUS));
    assert_eq!(COMMITTED_REINIT_TERMINAL_STATUS, "reinit:committed");
}

/// G5 (D) — VETO-AI DETERMINISM + DOMAIN (G-GUARD-1 / Art. V.1.3). The runtime
/// Veto-AI is replay-stable (same proposal capsule + same axiom hash -> same
/// verdict, byte-identical reason) and its verdict domain is EXACTLY the
/// two-valued `{Accept, Reject}` — no score, rank, confidence, or third value.
/// Both an Accept proposal and a Reject proposal are checked so the domain is
/// witnessed at both poles, and each is re-walked to prove no I/O / RNG /
/// probabilistic drift.
#[test]
fn fc3_veto_is_deterministic_and_domain_two_valued() {
    let axiom = constitution_source_hash();

    // A constitutionally-clean proposal -> Accept, deterministically.
    let clean = ArchitectProposalCapsule {
        schema_version: "x".to_string(),
        feedback_tx_id: TxId("f".into()),
        feedback_root: Hash([0u8; 32]),
        constitution_hash: axiom,
        tool_registry_root: Hash([7u8; 32]),
        proposal_kind: ArchitectProposalKind::ToolRegistryPatch,
        target_path: Some("src/bottom_white/tools/runtime_meta_tool.rs".to_string()),
        proposed_artifact_cid: Some(Cid([2u8; 32])),
        tools_used: vec!["fc3".to_string()],
        public_summary: "deterministic clean candidate".to_string(),
    };
    // A constitutionally-violating proposal (constitution.md in range) -> Reject.
    let dirty = ArchitectProposalCapsule {
        target_path: Some("constitution.md".to_string()),
        ..clean.clone()
    };

    // DETERMINISM: re-walking the SAME proposal yields a byte-identical outcome.
    for capsule in [&clean, &dirty] {
        let first = veto_walk(capsule, axiom);
        let second = veto_walk(capsule, axiom);
        let third = veto_walk(capsule, axiom);
        assert_eq!(
            first, second,
            "Veto-AI must be deterministic: same proposal -> same verdict/reason"
        );
        assert_eq!(
            second, third,
            "Veto-AI must be replay-stable across repeated walks (no I/O / RNG / probabilistic model)"
        );
    }

    // DOMAIN: the verdict is EXACTLY one of the two values. Enumerating the full
    // domain `VetoVerdict::{Accept, Reject}` and matching exhaustively proves
    // there is no third value / score / rank smuggled in.
    let clean_v = veto_walk(&clean, axiom).verdict;
    let dirty_v = veto_walk(&dirty, axiom).verdict;
    assert_eq!(clean_v, VetoVerdict::Accept, "clean candidate must Accept");
    assert_eq!(dirty_v, VetoVerdict::Reject, "dirty candidate must Reject");
    for v in [clean_v, dirty_v] {
        // Exhaustive match over the two-valued domain — a non-binary verdict
        // would fail to compile here, and any future third variant would force
        // this gate RED (it has no arm).
        match v {
            VetoVerdict::Accept | VetoVerdict::Reject => {}
        }
    }
    // The two poles are distinct (non-vacuous: not always-Accept, not always-Reject).
    assert_ne!(clean_v, dirty_v, "the two domain poles must be distinct");
}

/// G5 (E) — FAIL-CLOSED (G-GUARD-5 / admission fail-closed default). An
/// ambiguous / underspecified proposal that carries NO committable evidence
/// (a Noop-kind shell with no touched path and no artifact CID) must resolve to
/// `Reject`, NOT `Accept`. The default disposition of an unrecognized /
/// insufficient condition is VETO. Three independent ambiguity shapes are
/// checked so a single lenient clause cannot pass the gate.
#[test]
fn fc3_veto_fails_closed_on_ambiguous_proposal() {
    let axiom = constitution_source_hash();

    // (i) Empty Noop shell — no path, no artifact, no committable kind. This is
    // exactly the `ToolProposalPayload::default()`-equivalent ambiguity the leg
    // must refuse.
    let empty_shell = ArchitectProposalCapsule {
        schema_version: "x".to_string(),
        feedback_tx_id: TxId("f".into()),
        feedback_root: Hash([0u8; 32]),
        constitution_hash: axiom,
        tool_registry_root: Hash([0u8; 32]),
        proposal_kind: ArchitectProposalKind::Noop,
        target_path: None,
        proposed_artifact_cid: None,
        tools_used: vec![],
        public_summary: String::new(),
    };
    assert_eq!(
        veto_walk(&empty_shell, axiom).verdict,
        VetoVerdict::Reject,
        "an empty/ambiguous Noop shell must FAIL CLOSED to Reject (not Accept)"
    );

    // (ii) A proposal that bound a STALE / forged constitution hash (it did not
    // reason over the live axiom text). Fail-closed -> Reject.
    let stale_axiom = ArchitectProposalCapsule {
        constitution_hash: Hash([0xABu8; 32]),
        proposal_kind: ArchitectProposalKind::ToolRegistryPatch,
        target_path: Some("src/bottom_white/tools/runtime_meta_tool.rs".to_string()),
        proposed_artifact_cid: Some(Cid([3u8; 32])),
        ..empty_shell.clone()
    };
    assert_ne!(
        stale_axiom.constitution_hash, axiom,
        "precondition: the stale hash must actually differ from the live axiom"
    );
    assert_eq!(
        veto_walk(&stale_axiom, axiom).verdict,
        VetoVerdict::Reject,
        "a proposal bound to a stale/forged constitution hash must FAIL CLOSED"
    );

    // (iii) A committable kind but MISSING the artifact CID (insufficient
    // evidence) — still Reject, never Accept.
    let no_artifact = ArchitectProposalCapsule {
        constitution_hash: axiom,
        proposal_kind: ArchitectProposalKind::ToolRegistryPatch,
        target_path: Some("src/bottom_white/tools/runtime_meta_tool.rs".to_string()),
        proposed_artifact_cid: None,
        ..empty_shell.clone()
    };
    assert_eq!(
        veto_walk(&no_artifact, axiom).verdict,
        VetoVerdict::Reject,
        "a proposal missing its candidate artifact CID must FAIL CLOSED (insufficient evidence)"
    );

    // And the sandbox activation refuses outright on the resulting Reject — the
    // ambiguous candidate never reaches a commit/re-init.
    let sandbox = TempDir::new().expect("sandbox tempdir");
    let err = activate_sandbox_on_pass(
        veto_walk(&empty_shell, axiom).verdict,
        &[CandidatePayloadFile {
            rel_path: "src/bottom_white/tools/runtime_meta_tool.rs".to_string(),
            bytes: b"unused".to_vec(),
        }],
        sandbox.path(),
        &QState::default(),
    )
    .expect_err("ambiguous -> Reject must refuse activation");
    assert_eq!(err, CommitReinitError::VetoNotAccepted);
}

// Keep the proposer input-edge type referenced (parity with the observable gate).
#[allow(dead_code)]
fn _input_edge_type() -> TxId {
    LogFeedbackArchiveTx::default().tx_id
}
