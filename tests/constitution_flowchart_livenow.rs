//! Narrow LiveNow flowchart probes.
//!
//! These tests intentionally stay narrow. They exercise the FC1/FC2
//! production paths that are live today; FC3 feedback/re-init closure lives in
//! `tests/constitution_fc3_closure.rs`.
//! - FC1 typed sequencer wtool to L4 and L4.E
//! - FC1 real WorkTx provenance is CAS-bound, not synthetic fixture placeholders
//! - FC1 rtool/input snapshot is derived from ChainTape + CAS-backed telemetry
//! - FC2 boot, replay verification, and resume bootstrap
//! - FC2 system-emitted map-reduce tick to ChainTape
//! - FC2 terminal summary / halt-summary typing

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, RwLock};

use tempfile::TempDir;

use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::rejection_evidence::{
    RejectionClass as L4ERejectionClass, RejectionEvidenceWriter,
};
use turingosv4::bottom_white::ledger::system_keypair::{
    Ed25519Keypair, PinnedSystemPubkeys, SystemEpoch, SystemSignature,
};
use turingosv4::bottom_white::ledger::transition_ledger::{
    InMemoryLedgerWriter, LedgerWriter, TxKind,
};
use turingosv4::bottom_white::tools::registry::ToolRegistry;
use turingosv4::bus::{BusConfig, TuringBus};
use turingosv4::economy::money::{MicroCoin, StakeMicroCoin};
use turingosv4::kernel::Kernel;
use turingosv4::runtime::adapter::{make_real_worktx_signed_by, make_synthetic_task_open};
use turingosv4::runtime::agent_keypairs::AgentKeypairRegistry;
use turingosv4::runtime::proposal_telemetry::{
    write_to_cas as write_proposal_telemetry, ProposalTelemetry, TokenCounts,
};
use turingosv4::runtime::verify::{verify_chaintape, VerifyOptions};
use turingosv4::runtime::{build_chaintape_sequencer, RuntimeChaintapeConfig};
use turingosv4::state::q_state::{
    AgentId, EscrowEntry, Hash, QState, TaskId, TaskMarketEntry, TxId,
};
use turingosv4::state::sequencer::{
    ApplyError, Sequencer, SubmissionEnvelope, SubmitError, SystemEmitCommand,
};
use turingosv4::state::typed_tx::{
    AgentSignature, BoolWithProof, MapReduceTickTx, PredicateId, PredicateResultsBundle,
    RejectionClass, RunId, RunOutcome, SafetyOrCreation, TickKind, TransitionError, TypedTx,
    WorkTx,
};
use turingosv4::top_white::predicates::registry::{
    BootPredicateKind, BootPredicateManifest, BootPredicateSpec, PredicateBundleMap,
    PredicateMetadata, PredicateRegistry, SafetyOrCreation as RegistrySafetyOrCreation,
};
use turingosv4::top_white::predicates::visibility::Visibility;

struct MemoryHarness {
    _tmp: TempDir,
    cas: Arc<RwLock<CasStore>>,
    seq: Arc<Sequencer>,
    rx: tokio::sync::mpsc::Receiver<SubmissionEnvelope>,
    rejection_writer: Arc<RwLock<RejectionEvidenceWriter>>,
}

fn static_registry(result: bool) -> PredicateRegistry {
    let mut required_in = BTreeSet::new();
    required_in.insert(PredicateBundleMap::Acceptance);
    PredicateRegistry::from_boot_manifest(BootPredicateManifest {
        entries: vec![BootPredicateSpec {
            metadata: PredicateMetadata {
                predicate_id: "flow.live.static".to_string(),
                version: 1,
                code_hash: [0x41; 32],
                input_schema: "PredicateWorkView.v1".to_string(),
                output_schema: "bool".to_string(),
                visibility: Visibility::Public,
                owner: "system".to_string(),
                test_suite_hash: [0x14; 32],
                safety_class: RegistrySafetyOrCreation::Safety,
            },
            required_in,
            kind: BootPredicateKind::StaticBool(result),
        }],
    })
    .expect("static predicate registry")
}

fn q_for_bound_work(registry: &PredicateRegistry) -> QState {
    let mut q = QState::default();
    q.predicate_registry_root_t = registry.merkle_root_hash();
    let agent = AgentId("flow-live-agent".to_string());
    let sponsor = AgentId("flow-live-sponsor".to_string());
    let task = TaskId("flow-live-task".to_string());
    let escrow_tx = TxId("flow-live-escrow".to_string());

    q.economic_state_t
        .balances_t
        .0
        .insert(agent, MicroCoin::from_micro_units(10_000));
    q.economic_state_t.escrows_t.0.insert(
        escrow_tx.clone(),
        EscrowEntry {
            amount: MicroCoin::from_micro_units(10_000),
            depositor: sponsor.clone(),
            task_id: task.clone(),
        },
    );
    let mut market = TaskMarketEntry {
        publisher: sponsor,
        total_escrow: MicroCoin::from_micro_units(10_000),
        ..TaskMarketEntry::default()
    };
    market.escrow_lock_tx_ids.insert(escrow_tx);
    q.economic_state_t.task_markets_t.0.insert(task, market);
    q
}

fn memory_harness(initial_q: QState, registry: PredicateRegistry) -> MemoryHarness {
    let tmp = TempDir::new().expect("tempdir");
    let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas")));
    let cas_for_harness = Arc::clone(&cas);
    let keypair = Arc::new(Ed25519Keypair::generate_with_secure_entropy().expect("keypair"));
    let epoch = SystemEpoch::new(1);
    let writer: Arc<RwLock<dyn LedgerWriter>> = Arc::new(RwLock::new(InMemoryLedgerWriter::new()));
    let rejection_writer = Arc::new(RwLock::new(RejectionEvidenceWriter::default()));
    let mut pinned = PinnedSystemPubkeys::new();
    pinned.insert(epoch, keypair.public_key());
    let (seq, rx) = Sequencer::new(
        cas,
        keypair,
        epoch,
        writer,
        rejection_writer.clone(),
        Arc::new(registry),
        Arc::new(ToolRegistry::new()),
        Arc::new(pinned),
        initial_q,
        16,
    );
    MemoryHarness {
        _tmp: tmp,
        cas: cas_for_harness,
        seq: Arc::new(seq),
        rx,
        rejection_writer,
    }
}

fn work_tx(tx_id: &str, value: bool) -> TypedTx {
    work_tx_with_parent_and_proposal(
        tx_id,
        value,
        Hash::ZERO,
        Cid::from_content(tx_id.as_bytes()),
    )
}

fn work_tx_with_parent_and_proposal(
    tx_id: &str,
    value: bool,
    parent_state_root: Hash,
    proposal_cid: Cid,
) -> TypedTx {
    let pid = PredicateId("flow.live.static".to_string());
    let mut acceptance = BTreeMap::new();
    acceptance.insert(
        pid,
        BoolWithProof {
            value,
            proof_cid: None,
        },
    );
    TypedTx::Work(WorkTx {
        tx_id: TxId(tx_id.to_string()),
        task_id: TaskId("flow-live-task".to_string()),
        parent_state_root,
        agent_id: AgentId("flow-live-agent".to_string()),
        read_set: BTreeSet::new(),
        write_set: BTreeSet::new(),
        proposal_cid,
        predicate_results: PredicateResultsBundle {
            acceptance,
            settlement: BTreeMap::new(),
            safety_class: SafetyOrCreation::Safety,
        },
        stake: StakeMicroCoin::from_micro_units(10),
        signature: AgentSignature::default(),
        timestamp_logical: 1,
    })
}

fn write_flow_proposal_telemetry(
    h: &MemoryHarness,
    label: &str,
    logical_t: u64,
    parent_tx: Option<TxId>,
) -> Cid {
    let mut cas = h.cas.write().expect("cas write");
    let record = ProposalTelemetry::build_for_evaluator_append_with_parent(
        &mut cas,
        "flowchart-livenow",
        "flow-live-agent",
        logical_t,
        format!("payload-{label}").as_bytes(),
        label,
        TokenCounts {
            prompt_tokens: 10 + logical_t,
            completion_tokens: 2,
            tool_tokens: 1,
        },
        "constitution_flowchart_livenow",
        logical_t,
        parent_tx,
    )
    .expect("build ProposalTelemetry");
    write_proposal_telemetry(
        &mut cas,
        &record,
        "constitution_flowchart_livenow",
        logical_t,
    )
    .expect("write ProposalTelemetry")
}

fn runtime_cfg(tmp: &TempDir, run_id: &str, resume: bool) -> RuntimeChaintapeConfig {
    RuntimeChaintapeConfig {
        runtime_repo_path: tmp.path().join("runtime_repo"),
        cas_path: tmp.path().join("cas"),
        run_id: run_id.to_string(),
        queue_capacity: 16,
        resume_existing_chain: resume,
    }
}

#[tokio::test]
async fn fc1_typed_worktx_routes_to_l4_or_l4e() {
    let accept_registry = static_registry(true);
    let mut accepted = memory_harness(q_for_bound_work(&accept_registry), accept_registry);
    accepted
        .seq
        .submit_agent_tx(work_tx("flow-live-accepted", true))
        .await
        .expect("submit accepted work");
    let entry = accepted
        .seq
        .try_apply_one(&mut accepted.rx)
        .expect("apply accepted envelope")
        .expect("accepted WorkTx enters L4");
    assert_eq!(entry.tx_kind, TxKind::Work);
    assert_eq!(
        accepted.rejection_writer.read().expect("l4e read").len(),
        0,
        "accepted WorkTx must not create L4.E rejection"
    );

    let reject_registry = static_registry(false);
    let mut rejected = memory_harness(q_for_bound_work(&reject_registry), reject_registry);
    rejected
        .seq
        .submit_agent_tx(work_tx("flow-live-rejected", true))
        .await
        .expect("submit rejected work");
    let err = rejected
        .seq
        .try_apply_one(&mut rejected.rx)
        .expect("apply rejected envelope")
        .expect_err("forged true claim must reject");
    assert!(matches!(
        err,
        ApplyError::Transition(TransitionError::AcceptancePredicateProofMismatch(pid))
            if pid.0 == "flow.live.static"
    ));
    let guard = rejected.rejection_writer.read().expect("l4e read");
    let records = guard.records();
    assert_eq!(records.len(), 1, "rejected WorkTx must create one L4.E row");
    assert_eq!(records[0].tx_kind, TxKind::Work);
    assert_eq!(
        records[0].rejection_class,
        L4ERejectionClass::PredicateFailed
    );
    assert_eq!(
        rejected.seq.next_logical_t_peek(),
        0,
        "L4.E rejection must not consume L4 logical_t"
    );
}

#[test]
fn fc1_real_worktx_provenance_is_cas_bound_not_fixture_placeholder() {
    let tmp = TempDir::new().expect("tempdir");
    let mut keypairs = AgentKeypairRegistry::open(tmp.path()).expect("agent keypairs");
    let proposal_cid = Cid::from_content(b"flowchart-livenow-proposal-telemetry");
    let tx = make_real_worktx_signed_by(
        &mut keypairs,
        "flowchart-livenow-task",
        "flow-live-agent",
        Hash::ZERO,
        10,
        "flow-live",
        proposal_cid,
        true,
        1,
    )
    .expect("real WorkTx");
    let TypedTx::Work(work) = tx else {
        panic!("make_real_worktx_signed_by must return TypedTx::Work");
    };

    assert!(
        work.read_set
            .iter()
            .any(|key| key.0 == format!("cas.proposal_telemetry:{}", proposal_cid.hex())),
        "real WorkTx.read_set must bind ProposalTelemetry CAS, got {:?}",
        work.read_set
    );
    assert!(
        work.write_set
            .iter()
            .any(|key| key.0 == "task_output:flowchart-livenow-task:flow-live-agent:flow-live"),
        "real WorkTx.write_set must bind task/agent output, got {:?}",
        work.write_set
    );
    assert!(
        work.read_set.iter().all(|key| key.0 != "k.read")
            && work.write_set.iter().all(|key| key.0 != "k.write"),
        "real WorkTx must not reuse synthetic fixture placeholders"
    );
}

#[tokio::test]
async fn fc1_rtool_input_snapshot_is_chain_cas_derived() {
    let registry = static_registry(true);
    let mut h = memory_harness(q_for_bound_work(&registry), registry);

    let parent_tx = TxId("flow-live-parent".to_string());
    let parent_proposal_cid = write_flow_proposal_telemetry(&h, "parent", 1, None);
    let parent_root = h.seq.q_snapshot().expect("parent q").state_root_t;
    h.seq
        .submit_agent_tx(work_tx_with_parent_and_proposal(
            &parent_tx.0,
            true,
            parent_root,
            parent_proposal_cid,
        ))
        .await
        .expect("submit parent WorkTx");
    h.seq
        .try_apply_one(&mut h.rx)
        .expect("apply parent envelope")
        .expect("parent WorkTx enters L4");

    let child_tx = TxId("flow-live-child".to_string());
    let child_proposal_cid = write_flow_proposal_telemetry(&h, "child", 2, Some(parent_tx.clone()));
    let child_root = h.seq.q_snapshot().expect("child q").state_root_t;
    h.seq
        .submit_agent_tx(work_tx_with_parent_and_proposal(
            &child_tx.0,
            true,
            child_root,
            child_proposal_cid,
        ))
        .await
        .expect("submit child WorkTx");
    h.seq
        .try_apply_one(&mut h.rx)
        .expect("apply child envelope")
        .expect("child WorkTx enters L4");

    let canonical_edges = h.seq.compute_canonical_edges_at_head();
    assert_eq!(
        canonical_edges
            .get(&parent_tx)
            .expect("parent edge from CAS telemetry")
            .iter()
            .cloned()
            .collect::<Vec<_>>(),
        vec![child_tx.clone()],
        "FC1 rtool/input must reconstruct parent -> child edges from L4 WorkTx + ProposalTelemetry CAS"
    );

    let bus = TuringBus::with_sequencer(Kernel::new(), BusConfig::default(), Arc::clone(&h.seq));
    let snapshot = bus.snapshot();
    assert!(
        snapshot.sequencer_wired,
        "FC1 input snapshot must be wired to the typed sequencer read path"
    );
    assert!(
        snapshot.tape.is_empty(),
        "legacy shadow Tape is not the source of this FC1 read-view proof"
    );
    assert!(
        snapshot.price_index.contains_key(&parent_tx)
            && snapshot.price_index.contains_key(&child_tx),
        "FC1 input snapshot must expose ChainTape-derived node market entries, got {:?}",
        snapshot.price_index.keys().collect::<Vec<_>>()
    );
}

#[tokio::test]
async fn fc2_boot_replay_and_resume_are_live() {
    let tmp = TempDir::new().expect("tempdir");
    let fresh_cfg = runtime_cfg(&tmp, "flowchart-livenow-fresh", false);
    let bundle = build_chaintape_sequencer(&fresh_cfg).expect("fresh boot");
    let post_boot_q = bundle.sequencer.q_snapshot().expect("q after boot");
    let bus = TuringBus::with_sequencer(
        Kernel::new(),
        BusConfig::default(),
        bundle.sequencer.clone(),
    );
    let open = make_synthetic_task_open(
        "flowchart-livenow-task",
        "flowchart-livenow-sponsor",
        post_boot_q.state_root_t,
        "flowchart-livenow-open",
    );
    bus.submit_typed_tx(open).await.expect("submit TaskOpen");
    bundle
        .shutdown()
        .await
        .expect("fresh shutdown drains queue");
    drop(bus);

    let report = verify_chaintape(
        &fresh_cfg.runtime_repo_path,
        &fresh_cfg.cas_path,
        &VerifyOptions::default(),
    )
    .expect("verify chaintape");
    assert!(report.all_indicators_pass(), "replay report: {report:?}");
    assert_eq!(
        report.l4_entries, 3,
        "fresh boot must produce activation + boot MapReduceTick + TaskOpen L4 entries"
    );
    assert_eq!(report.l4e_entries, 0, "LiveNow boot probe expects no L4.E");
    assert!(
        report.detail.initial_q_state_loaded_from_disk,
        "replay must load the boot-time initial_q_state"
    );

    let resume_cfg = runtime_cfg(&tmp, "flowchart-livenow-resume", true);
    let resumed = build_chaintape_sequencer(&resume_cfg).expect("resume boot");
    assert_eq!(
        resumed.sequencer.next_logical_t_peek(),
        3,
        "resume must set next_logical_t to existing L4 length"
    );
    resumed.shutdown().await.expect("resume shutdown");
}

#[tokio::test]
async fn fc2_map_reduce_tick_is_tape_visible_and_replay_verified() {
    let tmp = TempDir::new().expect("tempdir");
    let fresh_cfg = runtime_cfg(&tmp, "flowchart-livenow-map-reduce-tick", false);
    let bundle = build_chaintape_sequencer(&fresh_cfg).expect("fresh boot");
    let writer = bundle.transition_writer.clone();
    let seq = bundle.sequencer.clone();

    bundle
        .shutdown()
        .await
        .expect("shutdown after boot map-reduce tick");

    let guard = writer.read().expect("ledger read");
    assert_eq!(
        guard.len(),
        2,
        "fresh boot must produce activation + boot MapReduceTick L4 rows"
    );
    let tick_entry = guard.read_at(2).expect("tick ledger entry");
    assert_eq!(tick_entry.tx_kind, TxKind::MapReduceTick);
    drop(guard);

    let q_after = seq.q_snapshot().expect("q after tick");
    assert_eq!(
        q_after.q_t.current_round, 1,
        "FC2 tick clock must advance the runtime round exactly once"
    );

    let report = verify_chaintape(
        &fresh_cfg.runtime_repo_path,
        &fresh_cfg.cas_path,
        &VerifyOptions::default(),
    )
    .expect("verify chaintape with MapReduceTick");
    assert!(report.all_indicators_pass(), "replay report: {report:?}");
    assert_eq!(report.l4_entries, 2);
    assert_eq!(report.l4e_entries, 0);
}

#[tokio::test]
async fn fc2_map_reduce_tick_agent_ingress_is_forbidden() {
    let h = memory_harness(
        QState::genesis(),
        PredicateRegistry::from_boot_manifest(BootPredicateManifest::empty())
            .expect("empty predicate registry"),
    );
    let tx = TypedTx::MapReduceTick(MapReduceTickTx {
        tx_id: TxId("forged-map-reduce-tick".to_string()),
        parent_state_root: Hash::ZERO,
        tape0_root: Hash::ZERO,
        tape0_len: 0,
        clock_t: 1,
        map_root: Hash::from_bytes([0x11; 32]),
        reduce_root: Hash::from_bytes([0x22; 32]),
        tick_kind: TickKind::Scheduled,
        epoch: SystemEpoch::new(1),
        timestamp_logical: 1,
        system_signature: SystemSignature::from_bytes([0u8; 64]),
    });
    let err = h
        .seq
        .submit_agent_tx(tx)
        .await
        .expect_err("agent ingress must reject system tick");
    assert!(matches!(err, SubmitError::SystemTxForbiddenOnAgentIngress));
}

#[tokio::test]
async fn fc2_terminal_summary_anchors_run_outcome() {
    let q = QState::genesis();
    let mut h = memory_harness(
        q,
        PredicateRegistry::from_boot_manifest(BootPredicateManifest::empty())
            .expect("empty predicate registry"),
    );
    let mut hist = BTreeMap::new();
    hist.insert(RejectionClass::Opaque, 3);
    h.seq
        .emit_system_tx(SystemEmitCommand::TerminalSummary {
            run_id: RunId("flowchart-livenow-run".to_string()),
            task_id: TaskId("flowchart-livenow-task".to_string()),
            run_outcome: RunOutcome::MaxTxExhausted,
            total_attempts: 3,
            failure_class_histogram: hist,
            last_logical_t: 2,
            solver_agent: Some(AgentId("flow-live-agent".to_string())),
            evidence_capsule_cid: Some(Cid::from_content(b"flow-livenow-evidence")),
        })
        .await
        .expect("emit terminal summary");
    let entry = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("apply terminal summary envelope")
        .expect("TerminalSummary enters L4");
    assert_eq!(entry.tx_kind, TxKind::TerminalSummary);

    let q_after = h.seq.q_snapshot().expect("q after terminal summary");
    let run = q_after
        .economic_state_t
        .runs_t
        .0
        .get(&RunId("flowchart-livenow-run".to_string()))
        .expect("runs_t entry");
    assert_eq!(run.run_outcome, RunOutcome::MaxTxExhausted);
    assert_eq!(run.attempt_count, 3);
    assert_eq!(run.last_logical_t, 2);
    assert_eq!(
        run.solver_agent,
        Some(AgentId("flow-live-agent".to_string()))
    );
}
