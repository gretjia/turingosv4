use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, RwLock};

use tempfile::TempDir;

use turingosv4::bottom_white::cas::schema::{Cid, ObjectType};
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::rejection_evidence::RejectionEvidenceWriter;
use turingosv4::bottom_white::ledger::system_keypair::{
    Ed25519Keypair, PinnedSystemPubkeys, SystemEpoch,
};
use turingosv4::bottom_white::ledger::transition_ledger::{
    canonical_decode, canonical_encode, cas_metadata_root_before_logical_t,
    constitution_source_hash, fc3_architect_commit_root, fc3_architect_proposal_root,
    fc3_feedback_root, fc3_veto_decision_root,
    replay_full_transition_with_predicate_binding_and_l4e, InMemoryLedgerWriter, LedgerEntry,
    LedgerWriter, TxKind,
};
use turingosv4::bottom_white::tools::registry::ToolRegistry;
use turingosv4::economy::money::StakeMicroCoin;
use turingosv4::state::q_state::{AgentId, Hash, QState, TaskId, TxId};
use turingosv4::state::sequencer::{
    ApplyError, Sequencer, SubmissionEnvelope, SubmitError, SystemEmitCommand,
};
use turingosv4::state::typed_tx::{
    AgentSignature, ArchitectCommitCapsule, ArchitectCommitTx, ArchitectFeedbackCapsule,
    ArchitectProposalCapsule, ArchitectProposalKind, ArchitectProposalTx, BoolWithProof,
    BootProfileId, LogFeedbackArchiveTx, MetaRoleMode, PredicateId, PredicateResultsBundle,
    ReinitBootTx, ReinitReason, ReinitReasonCapsule, ReinitRequestTx, RunId, RunOutcome,
    SafetyOrCreation, TransitionError, TypedTx, VetoDecisionCapsule, VetoDecisionTx,
    VetoReasonCode, VetoVerdict, WorkTx, ARCHITECT_COMMIT_SCHEMA_ID, ARCHITECT_FEEDBACK_SCHEMA_ID,
    ARCHITECT_PROPOSAL_SCHEMA_ID, REINIT_REASON_SCHEMA_ID, VETO_DECISION_SCHEMA_ID,
};
use turingosv4::top_white::predicates::registry::{BootPredicateManifest, PredicateRegistry};

struct Harness {
    _tmp: TempDir,
    cas: Arc<RwLock<CasStore>>,
    writer: Arc<RwLock<dyn LedgerWriter>>,
    rejections: Arc<RwLock<RejectionEvidenceWriter>>,
    seq: Arc<Sequencer>,
    rx: tokio::sync::mpsc::Receiver<SubmissionEnvelope>,
    pinned: Arc<PinnedSystemPubkeys>,
    registry: PredicateRegistry,
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
    let registry = PredicateRegistry::from_boot_manifest(BootPredicateManifest { entries: vec![] })
        .expect("empty predicate registry");
    let (seq, rx) = Sequencer::new(
        Arc::clone(&cas),
        keypair,
        epoch,
        Arc::clone(&writer),
        Arc::clone(&rejections),
        Arc::new(registry.clone()),
        Arc::new(ToolRegistry::new()),
        Arc::clone(&pinned),
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
        pinned,
        registry,
    }
}

fn stale_parent_work_tx() -> TypedTx {
    let mut acceptance = BTreeMap::new();
    acceptance.insert(
        PredicateId("fc3.legacy.true".to_string()),
        BoolWithProof {
            value: true,
            proof_cid: None,
        },
    );
    TypedTx::Work(WorkTx {
        tx_id: TxId("fc3-stale-work".to_string()),
        task_id: TaskId("fc3-task".to_string()),
        parent_state_root: Hash([7u8; 32]),
        agent_id: AgentId("fc3-agent".to_string()),
        read_set: BTreeSet::new(),
        write_set: BTreeSet::new(),
        proposal_cid: Cid::from_content(b"fc3-stale-work"),
        predicate_results: PredicateResultsBundle {
            acceptance,
            settlement: BTreeMap::new(),
            safety_class: SafetyOrCreation::Safety,
        },
        stake: StakeMicroCoin::from_micro_units(1),
        signature: AgentSignature::default(),
        timestamp_logical: 1,
    })
}

fn decode_entry_tx(h: &Harness, entry: &LedgerEntry) -> TypedTx {
    let cas = h.cas.read().expect("cas read");
    let bytes = cas.get(&entry.tx_payload_cid).expect("entry payload");
    canonical_decode(&bytes).expect("typed tx decode")
}

fn entries(h: &Harness) -> Vec<LedgerEntry> {
    let writer = h.writer.read().expect("writer read");
    (1..=writer.len())
        .map(|t| writer.read_at(t).expect("read_at"))
        .collect()
}

fn replay(h: &Harness, entries: &[LedgerEntry]) -> QState {
    let cas = h.cas.read().expect("cas read");
    let rejections = h.rejections.read().expect("l4e read");
    replay_full_transition_with_predicate_binding_and_l4e(
        &QState::default(),
        entries,
        &*cas,
        &*cas,
        &*rejections,
        &h.pinned,
        &h.registry,
        &ToolRegistry::new(),
    )
    .expect("fc3 replay")
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
        public_summary: "fc3 feedback public summary".to_string(),
        private_detail_cid: None,
    };
    let bytes = canonical_encode(&capsule).expect("capsule encode");
    h.cas
        .write()
        .expect("cas write")
        .put(
            &bytes,
            ObjectType::Generic,
            "constitution_fc3_closure",
            logical_t.saturating_sub(1),
            Some(ARCHITECT_FEEDBACK_SCHEMA_ID.to_string()),
        )
        .expect("put feedback capsule")
}

fn put_artifact(h: &Harness, label: &str) -> Cid {
    h.cas
        .write()
        .expect("cas write")
        .put(
            label.as_bytes(),
            ObjectType::Generic,
            "constitution_fc3_closure",
            h.seq.next_logical_t_peek(),
            Some("fc3.meta_artifact.fixture.v1".to_string()),
        )
        .expect("put artifact")
}

fn put_proposal_capsule(
    h: &Harness,
    feedback: &LogFeedbackArchiveTx,
    proposal_kind: ArchitectProposalKind,
    target_path: Option<&str>,
    proposed_artifact_cid: Option<Cid>,
    tools_used: Vec<String>,
) -> Cid {
    let q = h.seq.q_snapshot().expect("q snapshot");
    let capsule = ArchitectProposalCapsule {
        schema_version: ARCHITECT_PROPOSAL_SCHEMA_ID.to_string(),
        feedback_tx_id: feedback.tx_id.clone(),
        feedback_root: feedback.feedback_root,
        constitution_hash: constitution_source_hash(),
        tool_registry_root: q.tool_registry_root_t,
        proposal_kind,
        target_path: target_path.map(str::to_string),
        proposed_artifact_cid,
        tools_used,
        public_summary: "runtime ArchitectAI proposal".to_string(),
    };
    let bytes = canonical_encode(&capsule).expect("proposal capsule encode");
    h.cas
        .write()
        .expect("cas write")
        .put(
            &bytes,
            ObjectType::Generic,
            "constitution_fc3_closure",
            h.seq.next_logical_t_peek(),
            Some(ARCHITECT_PROPOSAL_SCHEMA_ID.to_string()),
        )
        .expect("put proposal capsule")
}

fn put_veto_capsule(
    h: &Harness,
    proposal: &ArchitectProposalTx,
    verdict: VetoVerdict,
    reason_code: VetoReasonCode,
) -> Cid {
    let capsule = VetoDecisionCapsule {
        schema_version: VETO_DECISION_SCHEMA_ID.to_string(),
        proposal_tx_id: proposal.tx_id.clone(),
        proposal_root: proposal.proposal_root,
        constitution_hash: constitution_source_hash(),
        verdict,
        reason_code,
        public_summary: match verdict {
            VetoVerdict::Pass => "PASS".to_string(),
            VetoVerdict::Veto => "VETO".to_string(),
        },
    };
    let bytes = canonical_encode(&capsule).expect("veto capsule encode");
    h.cas
        .write()
        .expect("cas write")
        .put(
            &bytes,
            ObjectType::Generic,
            "constitution_fc3_closure",
            h.seq.next_logical_t_peek(),
            Some(VETO_DECISION_SCHEMA_ID.to_string()),
        )
        .expect("put veto capsule")
}

fn put_commit_capsule_with_artifact(
    h: &Harness,
    veto: &VetoDecisionTx,
    applied_artifact_cid: Option<Cid>,
    target_path: Option<&str>,
) -> Cid {
    let capsule = ArchitectCommitCapsule {
        schema_version: ARCHITECT_COMMIT_SCHEMA_ID.to_string(),
        proposal_tx_id: veto.proposal_tx_id.clone(),
        veto_tx_id: veto.tx_id.clone(),
        decision_root: veto.decision_root,
        constitution_hash: constitution_source_hash(),
        applied_artifact_cid,
        target_path: target_path.map(str::to_string),
        public_summary: "runtime ArchitectAI approved commit".to_string(),
    };
    let bytes = canonical_encode(&capsule).expect("commit capsule encode");
    h.cas
        .write()
        .expect("cas write")
        .put(
            &bytes,
            ObjectType::Generic,
            "constitution_fc3_closure",
            h.seq.next_logical_t_peek(),
            Some(ARCHITECT_COMMIT_SCHEMA_ID.to_string()),
        )
        .expect("put commit capsule")
}

fn put_reinit_capsule(h: &Harness, trigger_entry: u64, reason: ReinitReason) -> Cid {
    let capsule = ReinitReasonCapsule {
        schema_version: REINIT_REASON_SCHEMA_ID.to_string(),
        trigger_entry,
        reason,
        public_summary: "error halt triggers re-init".to_string(),
        private_detail_cid: None,
    };
    let bytes = canonical_encode(&capsule).expect("reinit capsule encode");
    h.cas
        .write()
        .expect("cas write")
        .put(
            &bytes,
            ObjectType::Generic,
            "constitution_fc3_closure",
            h.seq.next_logical_t_peek(),
            Some(REINIT_REASON_SCHEMA_ID.to_string()),
        )
        .expect("put reinit capsule")
}

#[test]
fn fc3_tx_kind_discriminants_are_tail_only() {
    assert_eq!(TxKind::MapReduceTick as u8, 20);
    assert_eq!(TxKind::LogFeedbackArchive as u8, 21);
    assert_eq!(TxKind::ReinitRequest as u8, 22);
    assert_eq!(TxKind::ReinitBoot as u8, 23);
    assert_eq!(TxKind::ArchitectProposal as u8, 24);
    assert_eq!(TxKind::VetoDecision as u8, 25);
    assert_eq!(TxKind::ArchitectCommit as u8, 26);

    assert_eq!(
        TypedTx::LogFeedbackArchive(LogFeedbackArchiveTx::default()).tx_kind(),
        TxKind::LogFeedbackArchive
    );
    assert_eq!(
        TypedTx::ReinitRequest(ReinitRequestTx::default()).tx_kind(),
        TxKind::ReinitRequest
    );
    assert_eq!(
        TypedTx::ReinitBoot(ReinitBootTx::default()).tx_kind(),
        TxKind::ReinitBoot
    );
    assert_eq!(
        TypedTx::ArchitectProposal(ArchitectProposalTx::default()).tx_kind(),
        TxKind::ArchitectProposal
    );
    assert_eq!(
        TypedTx::VetoDecision(VetoDecisionTx::default()).tx_kind(),
        TxKind::VetoDecision
    );
    assert_eq!(
        TypedTx::ArchitectCommit(ArchitectCommitTx::default()).tx_kind(),
        TxKind::ArchitectCommit
    );

    let typed_tx_src = include_str!("../src/state/typed_tx.rs");
    assert!(
        !typed_tx_src.contains("ExternalOnly"),
        "FC3 runtime closure must not keep the old external-only role marker"
    );
}

#[tokio::test]
async fn fc3_logs_feedback_to_architect_ai_is_tape_cas_bound() {
    let mut h = harness();

    h.seq
        .submit_agent_tx(stale_parent_work_tx())
        .await
        .expect("submit stale work");
    let err = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("apply stale work")
        .expect_err("stale work rejects");
    assert!(matches!(
        err,
        ApplyError::Transition(TransitionError::StaleParent)
    ));
    assert_eq!(h.rejections.read().expect("l4e read").len(), 1);

    let feedback_capsule_cid = put_feedback_capsule(&h);
    h.seq
        .emit_system_tx(SystemEmitCommand::LogFeedbackArchive {
            feedback_capsule_cid,
            veto_verdict: VetoVerdict::Pass,
        })
        .await
        .expect("emit feedback");
    let entry = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("apply feedback")
        .expect("feedback accepts");
    assert_eq!(entry.tx_kind, TxKind::LogFeedbackArchive);

    let TypedTx::LogFeedbackArchive(tx) = decode_entry_tx(&h, &entry) else {
        panic!("expected LogFeedbackArchive tx")
    };
    assert_eq!(tx.source_l4_len, 0);
    assert_eq!(tx.source_l4e_len, 1);
    assert_eq!(
        tx.source_l4e_root,
        h.rejections.read().expect("l4e").last_hash()
    );
    assert_eq!(tx.role_mode, MetaRoleMode::Runtime);
    assert_eq!(tx.veto_verdict, VetoVerdict::Pass);

    let cas = h.cas.read().expect("cas read");
    let capsule_bytes = cas.get(&feedback_capsule_cid).expect("capsule bytes");
    let meta = cas
        .metadata(&feedback_capsule_cid)
        .expect("capsule metadata");
    assert_eq!(meta.object_type, ObjectType::Generic);
    assert_eq!(
        meta.schema_id.as_deref(),
        Some(ARCHITECT_FEEDBACK_SCHEMA_ID)
    );
    assert_eq!(
        tx.feedback_root,
        fc3_feedback_root(
            &capsule_bytes,
            tx.source_ledger_root,
            tx.source_l4_len,
            tx.source_l4e_root,
            tx.source_l4e_len,
            tx.cas_metadata_root,
            tx.constitution_hash,
        )
    );
    drop(cas);

    let replayed = replay(&h, &[entry]);
    assert_eq!(
        replayed.state_root_t,
        h.seq.q_snapshot().expect("q").state_root_t
    );
}

#[tokio::test]
async fn fc3_agent_ingress_rejects_meta_txs() {
    let h = harness();
    let before_l4 = h.writer.read().expect("writer").len();
    let before_l4e = h.rejections.read().expect("l4e").len();

    let cases = [
        TypedTx::LogFeedbackArchive(LogFeedbackArchiveTx::default()),
        TypedTx::ArchitectProposal(ArchitectProposalTx::default()),
        TypedTx::VetoDecision(VetoDecisionTx::default()),
        TypedTx::ArchitectCommit(ArchitectCommitTx::default()),
        TypedTx::ReinitRequest(ReinitRequestTx::default()),
        TypedTx::ReinitBoot(ReinitBootTx::default()),
    ];
    for tx in cases {
        let err = h
            .seq
            .submit_agent_tx(tx)
            .await
            .expect_err("FC3 system txs are forbidden on agent ingress");
        assert!(matches!(err, SubmitError::SystemTxForbiddenOnAgentIngress));
    }

    assert_eq!(h.writer.read().expect("writer").len(), before_l4);
    assert_eq!(h.rejections.read().expect("l4e").len(), before_l4e);
}

#[tokio::test]
async fn fc3_meta_feedback_replay_recomputes_source_log_root() {
    let mut h = harness();
    let feedback_capsule_cid = put_feedback_capsule(&h);
    h.seq
        .emit_system_tx(SystemEmitCommand::LogFeedbackArchive {
            feedback_capsule_cid,
            veto_verdict: VetoVerdict::Pass,
        })
        .await
        .expect("emit feedback");
    let entry = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("apply feedback")
        .expect("feedback accepts");
    let TypedTx::LogFeedbackArchive(tx) = decode_entry_tx(&h, &entry) else {
        panic!("expected feedback tx")
    };
    let capsule_bytes = h
        .cas
        .read()
        .expect("cas")
        .get(&tx.feedback_capsule_cid)
        .expect("capsule bytes");
    let recomputed = fc3_feedback_root(
        &capsule_bytes,
        tx.source_ledger_root,
        tx.source_l4_len,
        tx.source_l4e_root,
        tx.source_l4e_len,
        tx.cas_metadata_root,
        tx.constitution_hash,
    );
    assert_eq!(tx.feedback_root, recomputed);

    let replayed = replay(&h, &[entry]);
    assert_eq!(
        replayed.state_root_t,
        h.seq.q_snapshot().expect("q").state_root_t
    );
}

#[tokio::test]
async fn fc3_runtime_architect_veto_pass_allows_approved_commit() {
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
        panic!("expected feedback")
    };

    let artifact_cid = put_artifact(&h, "tool registry patch bytes");
    let proposal_cid = put_proposal_capsule(
        &h,
        &feedback_tx,
        ArchitectProposalKind::ToolRegistryPatch,
        Some("src/bottom_white/tools/runtime_meta_tool.rs"),
        Some(artifact_cid),
        vec!["sandboxed_exec".to_string()],
    );
    h.seq
        .emit_system_tx(SystemEmitCommand::ArchitectProposal {
            feedback_tx_id: feedback_tx.tx_id.clone(),
            proposal_capsule_cid: proposal_cid,
        })
        .await
        .expect("emit architect proposal");
    let proposal_entry = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("apply proposal")
        .expect("proposal accepts");
    assert_eq!(proposal_entry.tx_kind, TxKind::ArchitectProposal);
    let TypedTx::ArchitectProposal(proposal_tx) = decode_entry_tx(&h, &proposal_entry) else {
        panic!("expected proposal")
    };
    assert_eq!(proposal_tx.role_mode, MetaRoleMode::Runtime);
    let proposal_bytes = h.cas.read().expect("cas").get(&proposal_cid).unwrap();
    assert_eq!(
        proposal_tx.proposal_root,
        fc3_architect_proposal_root(
            &proposal_bytes,
            &feedback_tx.tx_id,
            feedback_tx.feedback_root,
            proposal_tx.constitution_hash,
            proposal_tx.tool_registry_root,
        )
    );

    let veto_cid = put_veto_capsule(
        &h,
        &proposal_tx,
        VetoVerdict::Pass,
        VetoReasonCode::ConstitutionCompliant,
    );
    h.seq
        .emit_system_tx(SystemEmitCommand::VetoDecision {
            proposal_tx_id: proposal_tx.tx_id.clone(),
            decision_capsule_cid: veto_cid,
        })
        .await
        .expect("emit veto decision");
    let veto_entry = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("apply veto")
        .expect("veto accepts");
    assert_eq!(veto_entry.tx_kind, TxKind::VetoDecision);
    let TypedTx::VetoDecision(veto_tx) = decode_entry_tx(&h, &veto_entry) else {
        panic!("expected veto")
    };
    assert_eq!(veto_tx.role_mode, MetaRoleMode::Runtime);
    assert_eq!(veto_tx.verdict, VetoVerdict::Pass);
    let veto_bytes = h.cas.read().expect("cas").get(&veto_cid).unwrap();
    assert_eq!(
        veto_tx.decision_root,
        fc3_veto_decision_root(
            &veto_bytes,
            &proposal_tx.tx_id,
            proposal_tx.proposal_root,
            veto_tx.constitution_hash,
        )
    );

    let commit_cid = put_commit_capsule_with_artifact(
        &h,
        &veto_tx,
        Some(artifact_cid),
        Some("src/bottom_white/tools/runtime_meta_tool.rs"),
    );
    h.seq
        .emit_system_tx(SystemEmitCommand::ArchitectCommit {
            veto_tx_id: veto_tx.tx_id.clone(),
            commit_capsule_cid: commit_cid,
        })
        .await
        .expect("emit architect commit");
    let commit_entry = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("apply commit")
        .expect("commit accepts");
    assert_eq!(commit_entry.tx_kind, TxKind::ArchitectCommit);
    let TypedTx::ArchitectCommit(commit_tx) = decode_entry_tx(&h, &commit_entry) else {
        panic!("expected commit")
    };
    assert_eq!(commit_tx.role_mode, MetaRoleMode::Runtime);
    let commit_bytes = h.cas.read().expect("cas").get(&commit_cid).unwrap();
    assert_eq!(
        commit_tx.commit_root,
        fc3_architect_commit_root(
            &commit_bytes,
            &proposal_tx.tx_id,
            &veto_tx.tx_id,
            veto_tx.decision_root,
            commit_tx.constitution_hash,
        )
    );

    let replayed = replay(&h, &entries(&h));
    assert_eq!(
        replayed.state_root_t,
        h.seq.q_snapshot().expect("q").state_root_t
    );
}

#[tokio::test]
async fn fc3_runtime_veto_blocks_constitution_mutation_commit() {
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
        panic!("expected feedback")
    };

    let artifact_cid = put_artifact(&h, "forbidden constitution patch bytes");
    let proposal_cid = put_proposal_capsule(
        &h,
        &feedback_tx,
        ArchitectProposalKind::TrustRootManifestPatch,
        Some("constitution.md"),
        Some(artifact_cid),
        vec!["sandboxed_exec".to_string()],
    );
    h.seq
        .emit_system_tx(SystemEmitCommand::ArchitectProposal {
            feedback_tx_id: feedback_tx.tx_id.clone(),
            proposal_capsule_cid: proposal_cid,
        })
        .await
        .expect("emit forbidden proposal");
    let proposal_entry = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("apply proposal")
        .expect("proposal accepts");
    let TypedTx::ArchitectProposal(proposal_tx) = decode_entry_tx(&h, &proposal_entry) else {
        panic!("expected proposal")
    };

    let veto_cid = put_veto_capsule(
        &h,
        &proposal_tx,
        VetoVerdict::Veto,
        VetoReasonCode::ConstitutionMutationForbidden,
    );
    h.seq
        .emit_system_tx(SystemEmitCommand::VetoDecision {
            proposal_tx_id: proposal_tx.tx_id.clone(),
            decision_capsule_cid: veto_cid,
        })
        .await
        .expect("emit veto");
    let veto_entry = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("apply veto")
        .expect("veto accepts");
    let TypedTx::VetoDecision(veto_tx) = decode_entry_tx(&h, &veto_entry) else {
        panic!("expected veto")
    };
    assert_eq!(veto_tx.verdict, VetoVerdict::Veto);

    let before_l4 = h.writer.read().expect("writer").len();
    let commit_cid =
        put_commit_capsule_with_artifact(&h, &veto_tx, Some(artifact_cid), Some("constitution.md"));
    let err = h
        .seq
        .emit_system_tx(SystemEmitCommand::ArchitectCommit {
            veto_tx_id: veto_tx.tx_id.clone(),
            commit_capsule_cid: commit_cid,
        })
        .await
        .expect_err("Veto-AI VETO must block ArchitectAI commit construction");
    assert!(
        matches!(
            err,
            turingosv4::state::sequencer::EmitSystemError::Fc3CapsuleInvalid
        ),
        "unexpected error: {err}"
    );
    assert_eq!(
        h.writer.read().expect("writer").len(),
        before_l4,
        "blocked commit must not append L4"
    );

    let replayed = replay(&h, &entries(&h));
    assert_eq!(
        replayed.state_root_t,
        h.seq.q_snapshot().expect("q").state_root_t
    );
}

#[tokio::test]
async fn fc3_runtime_passed_proposal_cannot_be_retargeted_at_commit() {
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
        panic!("expected feedback")
    };

    let artifact_cid = put_artifact(&h, "approved runtime patch bytes");
    let proposal_cid = put_proposal_capsule(
        &h,
        &feedback_tx,
        ArchitectProposalKind::ToolRegistryPatch,
        Some("src/bottom_white/tools/runtime_meta_tool.rs"),
        Some(artifact_cid),
        vec!["sandboxed_exec".to_string()],
    );
    h.seq
        .emit_system_tx(SystemEmitCommand::ArchitectProposal {
            feedback_tx_id: feedback_tx.tx_id.clone(),
            proposal_capsule_cid: proposal_cid,
        })
        .await
        .expect("emit proposal");
    let proposal_entry = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("apply proposal")
        .expect("proposal accepts");
    let TypedTx::ArchitectProposal(proposal_tx) = decode_entry_tx(&h, &proposal_entry) else {
        panic!("expected proposal")
    };

    let veto_cid = put_veto_capsule(
        &h,
        &proposal_tx,
        VetoVerdict::Pass,
        VetoReasonCode::ConstitutionCompliant,
    );
    h.seq
        .emit_system_tx(SystemEmitCommand::VetoDecision {
            proposal_tx_id: proposal_tx.tx_id.clone(),
            decision_capsule_cid: veto_cid,
        })
        .await
        .expect("emit veto");
    let veto_entry = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("apply veto")
        .expect("veto accepts");
    let TypedTx::VetoDecision(veto_tx) = decode_entry_tx(&h, &veto_entry) else {
        panic!("expected veto")
    };
    assert_eq!(veto_tx.verdict, VetoVerdict::Pass);

    let before_l4 = h.writer.read().expect("writer").len();
    let retargeted_commit_cid =
        put_commit_capsule_with_artifact(&h, &veto_tx, Some(artifact_cid), Some("constitution.md"));
    let err = h
        .seq
        .emit_system_tx(SystemEmitCommand::ArchitectCommit {
            veto_tx_id: veto_tx.tx_id.clone(),
            commit_capsule_cid: retargeted_commit_cid,
        })
        .await
        .expect_err("PASS on safe proposal must not authorize retargeted commit");
    assert!(
        matches!(
            err,
            turingosv4::state::sequencer::EmitSystemError::Fc3CapsuleInvalid
        ),
        "unexpected error: {err}"
    );
    assert_eq!(
        h.writer.read().expect("writer").len(),
        before_l4,
        "retargeted commit must not append L4"
    );
}

#[test]
fn fc3_architect_feedback_is_not_plain_handover_or_latest() {
    let sequencer_src = include_str!("../src/state/sequencer.rs");
    assert!(!sequencer_src.contains("handover/ai-direct/LATEST.md"));
    assert!(!sequencer_src.contains("TB_LOG.tsv"));
    assert!(!sequencer_src.contains("TRACE_FLOWCHART_MATRIX.md"));
    assert!(
        sequencer_src.contains("constitution_source_hash()"),
        "FC3 feedback must anchor constitution.md, not derived handover views"
    );
}

#[test]
fn fc3_architect_feedback_read_view_is_shielded() {
    let h = harness();
    let sentinel = b"PRIVATE-FC3-DETAIL-MUST-NOT-BE-IN-PUBLIC-SUMMARY";
    let private_detail_cid = h
        .cas
        .write()
        .expect("cas")
        .put(
            sentinel,
            ObjectType::Generic,
            "constitution_fc3_closure",
            0,
            Some("fc3.private_detail.audit_only.v1".to_string()),
        )
        .expect("put private detail");
    let q = h.seq.q_snapshot().expect("q");
    let capsule = ArchitectFeedbackCapsule {
        schema_version: ARCHITECT_FEEDBACK_SCHEMA_ID.to_string(),
        source_ledger_root: q.ledger_root_t,
        source_l4e_root: Hash::ZERO,
        cas_metadata_root: cas_metadata_root_before_logical_t(
            &h.cas.read().expect("cas"),
            h.seq.next_logical_t_peek() + 1,
        )
        .expect("cas root"),
        constitution_hash: constitution_source_hash(),
        public_summary: "only this public summary may cross the agent boundary".to_string(),
        private_detail_cid: Some(private_detail_cid),
    };
    let public_projection = format!(
        "{} {:?}",
        capsule.public_summary, capsule.private_detail_cid
    );
    assert!(!public_projection.contains(std::str::from_utf8(sentinel).unwrap()));
    assert_eq!(capsule.private_detail_cid, Some(private_detail_cid));
}

#[tokio::test]
async fn fc3_error_reinit_request_links_errorhalt_to_next_boot() {
    let mut h = harness();

    h.seq
        .emit_system_tx(SystemEmitCommand::TerminalSummary {
            run_id: RunId("fc3-run".to_string()),
            task_id: TaskId("fc3-task".to_string()),
            run_outcome: RunOutcome::ErrorHalt,
            total_attempts: 1,
            failure_class_histogram: BTreeMap::new(),
            last_logical_t: 0,
            solver_agent: None,
            evidence_capsule_cid: None,
        })
        .await
        .expect("emit terminal summary");
    let terminal_entry = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("apply terminal")
        .expect("terminal accepts");
    assert_eq!(terminal_entry.tx_kind, TxKind::TerminalSummary);

    let reason = ReinitReason::TerminalErrorHalt;
    let reason_cid = put_reinit_capsule(&h, terminal_entry.logical_t, reason);
    let boot_profile = BootProfileId("fc3-default-boot".to_string());
    h.seq
        .emit_system_tx(SystemEmitCommand::ReinitRequest {
            trigger_entry: terminal_entry.logical_t,
            error_evidence_cid: reason_cid,
            reason,
            target_boot_profile: boot_profile.clone(),
        })
        .await
        .expect("emit reinit request");
    let request_entry = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("apply request")
        .expect("reinit request accepts");
    assert_eq!(request_entry.tx_kind, TxKind::ReinitRequest);
    let TypedTx::ReinitRequest(request_tx) = decode_entry_tx(&h, &request_entry) else {
        panic!("expected ReinitRequest")
    };
    assert_eq!(request_tx.trigger_entry, terminal_entry.logical_t);
    assert_eq!(request_tx.error_evidence_cid, reason_cid);
    assert_eq!(request_tx.role_mode, MetaRoleMode::Runtime);

    h.seq
        .emit_system_tx(SystemEmitCommand::ReinitBoot {
            request_tx_id: request_tx.tx_id.clone(),
            boot_profile: boot_profile.clone(),
        })
        .await
        .expect("emit reinit boot");
    let boot_entry = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("apply boot")
        .expect("reinit boot accepts");
    assert_eq!(boot_entry.tx_kind, TxKind::ReinitBoot);
    let TypedTx::ReinitBoot(boot_tx) = decode_entry_tx(&h, &boot_entry) else {
        panic!("expected ReinitBoot")
    };
    assert_eq!(boot_tx.request_tx_id, request_tx.tx_id);
    assert_eq!(boot_tx.boot_profile, boot_profile);
    assert_eq!(boot_tx.role_mode, MetaRoleMode::Runtime);

    let replayed = replay(&h, &entries(&h));
    assert_eq!(
        replayed.state_root_t,
        h.seq.q_snapshot().expect("q").state_root_t
    );
}

#[tokio::test]
async fn fc3_reinit_boot_recomputes_replayed_state_root() {
    let mut h = harness();
    h.seq
        .emit_system_tx(SystemEmitCommand::TerminalSummary {
            run_id: RunId("fc3-replay-root".to_string()),
            task_id: TaskId("fc3-task".to_string()),
            run_outcome: RunOutcome::ErrorHalt,
            total_attempts: 1,
            failure_class_histogram: BTreeMap::new(),
            last_logical_t: 0,
            solver_agent: None,
            evidence_capsule_cid: None,
        })
        .await
        .expect("emit terminal summary");
    let terminal_entry = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("apply terminal")
        .expect("terminal accepts");
    let reason = ReinitReason::TerminalErrorHalt;
    let reason_cid = put_reinit_capsule(&h, terminal_entry.logical_t, reason);
    let boot_profile = BootProfileId("fc3-replay-root-boot".to_string());
    h.seq
        .emit_system_tx(SystemEmitCommand::ReinitRequest {
            trigger_entry: terminal_entry.logical_t,
            error_evidence_cid: reason_cid,
            reason,
            target_boot_profile: boot_profile.clone(),
        })
        .await
        .expect("emit request");
    let request_entry = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("apply request")
        .expect("request accepts");
    let state_after_request = h.seq.q_snapshot().expect("q after request").state_root_t;
    let TypedTx::ReinitRequest(request_tx) = decode_entry_tx(&h, &request_entry) else {
        panic!("expected ReinitRequest")
    };
    h.seq
        .emit_system_tx(SystemEmitCommand::ReinitBoot {
            request_tx_id: request_tx.tx_id.clone(),
            boot_profile,
        })
        .await
        .expect("emit boot");
    let boot_entry = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("apply boot")
        .expect("boot accepts");
    let TypedTx::ReinitBoot(boot_tx) = decode_entry_tx(&h, &boot_entry) else {
        panic!("expected ReinitBoot")
    };
    assert_eq!(boot_tx.replayed_state_root, state_after_request);
    let replayed = replay(&h, &entries(&h));
    assert_eq!(
        replayed.state_root_t,
        h.seq.q_snapshot().expect("q").state_root_t
    );
}

#[tokio::test]
async fn fc3_reinit_no_rewrite_old_evidence() {
    let mut h = harness();
    h.seq
        .emit_system_tx(SystemEmitCommand::TerminalSummary {
            run_id: RunId("fc3-no-rewrite".to_string()),
            task_id: TaskId("fc3-task".to_string()),
            run_outcome: RunOutcome::ErrorHalt,
            total_attempts: 1,
            failure_class_histogram: BTreeMap::new(),
            last_logical_t: 0,
            solver_agent: None,
            evidence_capsule_cid: None,
        })
        .await
        .expect("emit terminal summary");
    let terminal_entry = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("apply terminal")
        .expect("terminal accepts");
    let old_payload = {
        let cas = h.cas.read().expect("cas read");
        cas.get(&terminal_entry.tx_payload_cid)
            .expect("old payload")
    };
    let old_l4e_root = h.rejections.read().expect("l4e").last_hash();

    let reason = ReinitReason::TerminalErrorHalt;
    let reason_cid = put_reinit_capsule(&h, terminal_entry.logical_t, reason);
    h.seq
        .emit_system_tx(SystemEmitCommand::ReinitRequest {
            trigger_entry: terminal_entry.logical_t,
            error_evidence_cid: reason_cid,
            reason,
            target_boot_profile: BootProfileId("fc3-no-rewrite-boot".to_string()),
        })
        .await
        .expect("emit reinit request");
    h.seq
        .try_apply_one(&mut h.rx)
        .expect("apply request")
        .expect("request accepts");

    let cas = h.cas.read().expect("cas read");
    assert_eq!(
        cas.get(&terminal_entry.tx_payload_cid)
            .expect("old payload after"),
        old_payload,
        "re-init must not rewrite old L4 payload CAS bytes"
    );
    assert_eq!(
        h.rejections.read().expect("l4e").last_hash(),
        old_l4e_root,
        "accepted re-init must not rewrite or append L4.E"
    );
}
