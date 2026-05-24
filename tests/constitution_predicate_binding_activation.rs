use std::sync::{Arc, RwLock};

use tempfile::TempDir;
use turingosv4::bottom_white::cas::schema::{Cid, ObjectType};
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::rejection_evidence::RejectionEvidenceWriter;
use turingosv4::bottom_white::ledger::system_keypair::{
    Ed25519Keypair, PinnedSystemPubkeys, SystemEpoch, SystemSignature,
};
use turingosv4::bottom_white::ledger::transition_ledger::{
    canonical_encode, InMemoryLedgerWriter, LedgerWriter,
};
use turingosv4::bottom_white::tools::registry::ToolRegistry;
use turingosv4::state::q_state::{Hash, QState, TxId};
use turingosv4::state::sequencer::{
    ApplyError, EmitSystemError, Sequencer, SubmissionEnvelope, SubmitError, SystemEmitCommand,
};
use turingosv4::state::typed_tx::{PredicateBindingActivateTx, TransitionError, TypedTx};
use turingosv4::top_white::predicates::registry::SafetyOrCreation;
use turingosv4::top_white::predicates::registry::{
    BootPredicateManifest, PredicateBundleMap, PredicateMetadata, PredicateRegistry,
    PredicateRegistrySnapshotCapsule, PredicateSnapshotEntry,
};
use turingosv4::top_white::predicates::visibility::Visibility;

struct Harness {
    _tmp: TempDir,
    cas: Arc<RwLock<CasStore>>,
    seq: Sequencer,
    rx: tokio::sync::mpsc::Receiver<SubmissionEnvelope>,
    registry: PredicateRegistry,
}

fn harness() -> Harness {
    let tmp = TempDir::new().expect("tempdir");
    let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas")));
    let keypair = Arc::new(Ed25519Keypair::generate_with_secure_entropy().expect("keypair"));
    let epoch = SystemEpoch::new(1);
    let mut pinned = PinnedSystemPubkeys::new();
    pinned.insert(epoch, keypair.public_key());
    let registry = PredicateRegistry::from_boot_manifest(BootPredicateManifest::empty())
        .expect("empty manifest");
    let writer: Arc<RwLock<dyn LedgerWriter>> = Arc::new(RwLock::new(InMemoryLedgerWriter::new()));
    let (seq, rx) = Sequencer::new(
        cas.clone(),
        keypair,
        epoch,
        writer,
        Arc::new(RwLock::new(RejectionEvidenceWriter::default())),
        Arc::new(registry.clone()),
        Arc::new(ToolRegistry::new()),
        Arc::new(pinned),
        QState::default(),
        8,
    );
    Harness {
        _tmp: tmp,
        cas,
        seq,
        rx,
        registry,
    }
}

fn put_snapshot(h: &Harness) -> Cid {
    let snapshot = h.registry.snapshot_capsule();
    let bytes = canonical_encode(&snapshot).expect("encode snapshot");
    h.cas
        .write()
        .expect("cas")
        .put(
            &bytes,
            ObjectType::PredicateRegistrySnapshotCapsule,
            "system",
            0,
            Some(PredicateRegistrySnapshotCapsule::SCHEMA_ID.to_string()),
        )
        .expect("put snapshot")
}

fn put_snapshot_with_lied_root(h: &Harness) -> Cid {
    let mut snapshot = h.registry.snapshot_capsule();
    snapshot.entries.push(PredicateSnapshotEntry {
        metadata: PredicateMetadata {
            predicate_id: "ghost_predicate_not_in_binary".to_string(),
            version: 1,
            code_hash: [7u8; 32],
            input_schema: "PredicateContext.v1".to_string(),
            output_schema: "BoolWithProof.v1".to_string(),
            visibility: Visibility::Public,
            owner: "system".to_string(),
            test_suite_hash: [8u8; 32],
            safety_class: SafetyOrCreation::Safety,
        },
        required_in: [PredicateBundleMap::Acceptance].into_iter().collect(),
    });
    snapshot.merkle_root = h.registry.merkle_root_hash();
    let bytes = canonical_encode(&snapshot).expect("encode malformed snapshot");
    h.cas
        .write()
        .expect("cas")
        .put(
            &bytes,
            ObjectType::PredicateRegistrySnapshotCapsule,
            "system",
            0,
            Some(PredicateRegistrySnapshotCapsule::SCHEMA_ID.to_string()),
        )
        .expect("put malformed snapshot")
}

#[tokio::test]
async fn agent_ingress_rejects_predicate_binding_activate_tx() {
    let mut h = harness();
    let tx = TypedTx::PredicateBindingActivate(PredicateBindingActivateTx {
        tx_id: TxId("forged-activation".to_string()),
        parent_state_root: Hash::ZERO,
        registry_snapshot_cid: Cid::from_content(b"missing"),
        registry_merkle_root: h.registry.merkle_root_hash(),
        epoch: SystemEpoch::new(1),
        timestamp_logical: 1,
        system_signature: SystemSignature::default(),
    });
    let err = h.seq.submit_agent_tx(tx).await.expect_err("agent ingress");
    assert!(matches!(err, SubmitError::SystemTxForbiddenOnAgentIngress));
    assert!(h.seq.try_apply_one(&mut h.rx).is_none());
}

#[tokio::test]
async fn emit_rejects_missing_registry_snapshot() {
    let h = harness();
    let err = h
        .seq
        .emit_system_tx(SystemEmitCommand::PredicateBindingActivate {
            registry_snapshot_cid: Cid::from_content(b"missing"),
            registry_merkle_root: h.registry.merkle_root_hash(),
        })
        .await
        .expect_err("missing snapshot");
    assert!(matches!(
        err,
        EmitSystemError::PredicateRegistrySnapshotInvalid
    ));
}

#[tokio::test]
async fn emit_rejects_registry_snapshot_with_lied_root() {
    let h = harness();
    let snapshot_cid = put_snapshot_with_lied_root(&h);
    let err = h
        .seq
        .emit_system_tx(SystemEmitCommand::PredicateBindingActivate {
            registry_snapshot_cid: snapshot_cid,
            registry_merkle_root: h.registry.merkle_root_hash(),
        })
        .await
        .expect_err("malformed snapshot with lied root");
    assert!(matches!(
        err,
        EmitSystemError::PredicateRegistrySnapshotInvalid
    ));
}

#[tokio::test]
async fn activation_is_single_use() {
    let mut h = harness();
    let snapshot_cid = put_snapshot(&h);
    h.seq
        .emit_system_tx(SystemEmitCommand::PredicateBindingActivate {
            registry_snapshot_cid: snapshot_cid,
            registry_merkle_root: h.registry.merkle_root_hash(),
        })
        .await
        .expect("emit activation");
    h.seq
        .try_apply_one(&mut h.rx)
        .expect("first applied")
        .expect("first activation accepts");
    h.seq
        .emit_system_tx(SystemEmitCommand::PredicateBindingActivate {
            registry_snapshot_cid: snapshot_cid,
            registry_merkle_root: h.registry.merkle_root_hash(),
        })
        .await
        .expect("emit second activation");
    let err = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("second applied")
        .expect_err("second activation rejects");
    match err {
        ApplyError::Transition(TransitionError::PredicateBindingAlreadyActivated) => {}
        other => panic!("expected PredicateBindingAlreadyActivated, got {other:?}"),
    }
}
