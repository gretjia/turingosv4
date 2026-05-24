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
    canonical_decode, canonical_encode, InMemoryLedgerWriter, LedgerWriter, TxKind,
};
use turingosv4::bottom_white::tools::registry::ToolRegistry;
use turingosv4::economy::money::{MicroCoin, StakeMicroCoin};
use turingosv4::state::q_state::{
    AgentId, EscrowEntry, Hash, QState, TaskId, TaskMarketEntry, TxId,
};
use turingosv4::state::sequencer::{ApplyError, Sequencer, SubmissionEnvelope, SystemEmitCommand};
use turingosv4::state::typed_tx::{
    AgentSignature, BoolWithProof, PredicateBindingActivateTx, PredicateId, PredicateResultsBundle,
    TransitionError, TypedTx, WorkTx,
};
use turingosv4::top_white::predicates::registry::{
    BootPredicateKind, BootPredicateManifest, BootPredicateSpec, PredicateBundleMap,
    PredicateMetadata, PredicateRegistry, PredicateRegistrySnapshotCapsule, SafetyOrCreation,
};
use turingosv4::top_white::predicates::visibility::Visibility;

struct Harness {
    _tmp: TempDir,
    cas: Arc<RwLock<CasStore>>,
    seq: Sequencer,
    rx: tokio::sync::mpsc::Receiver<SubmissionEnvelope>,
}

fn registry_with_static_acceptance(result: bool) -> PredicateRegistry {
    let mut required_in = BTreeSet::new();
    required_in.insert(PredicateBundleMap::Acceptance);
    PredicateRegistry::from_boot_manifest(BootPredicateManifest {
        entries: vec![BootPredicateSpec {
            metadata: PredicateMetadata {
                predicate_id: "p.static".to_string(),
                version: 1,
                code_hash: [0x42; 32],
                input_schema: "PredicateWorkView.v1".to_string(),
                output_schema: "bool".to_string(),
                visibility: Visibility::Public,
                owner: "system".to_string(),
                test_suite_hash: [0x24; 32],
                safety_class: SafetyOrCreation::Safety,
            },
            required_in,
            kind: BootPredicateKind::StaticBool(result),
        }],
    })
    .expect("registry")
}

fn bound_q(registry: &PredicateRegistry) -> QState {
    let mut q = QState::default();
    q.predicate_registry_root_t = registry.merkle_root_hash();
    let agent = AgentId("predicate-test-agent".to_string());
    let sponsor = AgentId("predicate-test-sponsor".to_string());
    let task = TaskId("predicate-test-task".to_string());
    let escrow_tx = TxId("predicate-test-escrow".to_string());
    q.economic_state_t
        .balances_t
        .0
        .insert(agent, MicroCoin::from_micro_units(1_000));
    q.economic_state_t.escrows_t.0.insert(
        escrow_tx.clone(),
        EscrowEntry {
            amount: MicroCoin::from_micro_units(1_000),
            depositor: sponsor.clone(),
            task_id: task.clone(),
        },
    );
    let mut market = TaskMarketEntry {
        publisher: sponsor,
        total_escrow: MicroCoin::from_micro_units(1_000),
        ..TaskMarketEntry::default()
    };
    market.escrow_lock_tx_ids.insert(escrow_tx);
    q.economic_state_t.task_markets_t.0.insert(task, market);
    q
}

fn harness(initial_q: QState, registry: PredicateRegistry) -> Harness {
    let tmp = TempDir::new().expect("tempdir");
    let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas")));
    let keypair = Arc::new(Ed25519Keypair::generate_with_secure_entropy().expect("keypair"));
    let epoch = SystemEpoch::new(1);
    let mut pinned = PinnedSystemPubkeys::new();
    pinned.insert(epoch, keypair.public_key());
    let writer: Arc<RwLock<dyn LedgerWriter>> = Arc::new(RwLock::new(InMemoryLedgerWriter::new()));
    let (seq, rx) = Sequencer::new(
        cas.clone(),
        keypair,
        epoch,
        writer,
        Arc::new(RwLock::new(RejectionEvidenceWriter::default())),
        Arc::new(registry),
        Arc::new(ToolRegistry::new()),
        Arc::new(pinned),
        initial_q,
        8,
    );
    Harness {
        _tmp: tmp,
        cas,
        seq,
        rx,
    }
}

fn work_tx(value: bool) -> TypedTx {
    let pid = PredicateId("p.static".to_string());
    let mut acceptance = BTreeMap::new();
    acceptance.insert(
        pid,
        BoolWithProof {
            value,
            proof_cid: None,
        },
    );
    TypedTx::Work(WorkTx {
        tx_id: TxId("work-1".to_string()),
        task_id: TaskId("predicate-test-task".to_string()),
        parent_state_root: Hash::ZERO,
        agent_id: AgentId("predicate-test-agent".to_string()),
        read_set: BTreeSet::new(),
        write_set: BTreeSet::new(),
        proposal_cid: Cid::from_content(b"proposal"),
        predicate_results: PredicateResultsBundle {
            acceptance,
            settlement: BTreeMap::new(),
            safety_class: turingosv4::state::typed_tx::SafetyOrCreation::Safety,
        },
        stake: StakeMicroCoin::from_micro_units(10),
        signature: AgentSignature::default(),
        timestamp_logical: 1,
    })
}

#[tokio::test]
async fn forged_value_true_is_rejected_when_registry_predicate_recomputes_false() {
    let registry = registry_with_static_acceptance(false);
    let q = bound_q(&registry);
    let mut h = harness(q, registry);
    h.seq
        .submit_agent_tx(work_tx(true))
        .await
        .expect("submit work");
    let err = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("applied")
        .expect_err("predicate mismatch must reject");
    assert!(matches!(
        err,
        ApplyError::Transition(TransitionError::AcceptancePredicateProofMismatch(pid))
            if pid.0 == "p.static"
    ));
}

#[tokio::test]
async fn registry_pass_stamp_with_true_value_enters_l4() {
    let registry = registry_with_static_acceptance(true);
    let q = bound_q(&registry);
    let mut h = harness(q, registry.clone());
    h.seq
        .submit_agent_tx(work_tx(true))
        .await
        .expect("submit work");
    let entry = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("applied")
        .expect("predicate pass must accept");
    assert_eq!(entry.tx_kind, TxKind::Work);
    assert_eq!(
        h.seq.q_snapshot().unwrap().predicate_registry_root_t,
        registry.merkle_root_hash()
    );
    assert_ne!(entry.resulting_state_root, Hash::ZERO);
}

#[tokio::test]
async fn predicate_binding_activation_records_snapshot_root_on_l4_payload() {
    let registry = registry_with_static_acceptance(true);
    let q = QState::default();
    let mut h = harness(q, registry.clone());
    let snapshot = registry.snapshot_capsule();
    let snapshot_bytes = canonical_encode(&snapshot).expect("snapshot encode");
    let snapshot_cid = {
        let mut cas = h.cas.write().expect("cas");
        cas.put(
            &snapshot_bytes,
            ObjectType::PredicateRegistrySnapshotCapsule,
            "system",
            0,
            Some(PredicateRegistrySnapshotCapsule::SCHEMA_ID.to_string()),
        )
        .expect("put snapshot")
    };
    h.seq
        .emit_system_tx(SystemEmitCommand::PredicateBindingActivate {
            registry_snapshot_cid: snapshot_cid,
            registry_merkle_root: registry.merkle_root_hash(),
        })
        .await
        .expect("emit activation");
    let entry = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("applied")
        .expect("activation accepts");
    assert_eq!(entry.tx_kind, TxKind::PredicateBindingActivate);
    assert_eq!(
        h.seq.q_snapshot().unwrap().predicate_registry_root_t,
        registry.merkle_root_hash()
    );
    let payload = h
        .cas
        .read()
        .expect("cas")
        .get(&entry.tx_payload_cid)
        .expect("payload");
    let decoded: TypedTx = canonical_decode(&payload).expect("decode payload");
    match decoded {
        TypedTx::PredicateBindingActivate(PredicateBindingActivateTx {
            registry_snapshot_cid,
            registry_merkle_root,
            ..
        }) => {
            assert_eq!(registry_snapshot_cid, snapshot_cid);
            assert_eq!(registry_merkle_root, registry.merkle_root_hash());
        }
        other => panic!("expected activation payload, got {other:?}"),
    }
}
