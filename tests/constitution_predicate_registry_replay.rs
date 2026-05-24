use std::sync::{Arc, RwLock};

use tempfile::TempDir;
use turingosv4::bottom_white::cas::schema::{Cid, ObjectType};
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::rejection_evidence::RejectionEvidenceWriter;
use turingosv4::bottom_white::ledger::system_keypair::{
    Ed25519Keypair, PinnedSystemPubkeys, SystemEpoch,
};
use turingosv4::bottom_white::ledger::transition_ledger::{
    canonical_encode, replay_full_transition, replay_full_transition_with_predicate_binding,
    InMemoryLedgerWriter, LedgerWriter,
};
use turingosv4::bottom_white::tools::registry::ToolRegistry;
use turingosv4::state::q_state::QState;
use turingosv4::state::sequencer::{Sequencer, SubmissionEnvelope, SystemEmitCommand};
use turingosv4::top_white::predicates::registry::{
    BootPredicateManifest, PredicateRegistry, PredicateRegistrySnapshotCapsule,
};

struct Harness {
    _tmp: TempDir,
    cas: Arc<RwLock<CasStore>>,
    pinned: Arc<PinnedSystemPubkeys>,
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
    let pinned = Arc::new(pinned);
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
        pinned.clone(),
        QState::default(),
        8,
    );
    Harness {
        _tmp: tmp,
        cas,
        pinned,
        seq,
        rx,
        registry,
    }
}

fn put_snapshot(h: &Harness) -> Cid {
    let bytes = canonical_encode(&h.registry.snapshot_capsule()).expect("encode snapshot");
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

#[tokio::test]
async fn replay_reconstructs_activation_with_predicate_cas_view() {
    let mut h = harness();
    let snapshot_cid = put_snapshot(&h);
    h.seq
        .emit_system_tx(SystemEmitCommand::PredicateBindingActivate {
            registry_snapshot_cid: snapshot_cid,
            registry_merkle_root: h.registry.merkle_root_hash(),
        })
        .await
        .expect("emit");
    let entry = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("applied")
        .expect("activation accepts");
    let cas = h.cas.read().expect("cas");
    let replayed = replay_full_transition_with_predicate_binding(
        &QState::default(),
        &[entry],
        &*cas,
        &*cas,
        &h.pinned,
        &h.registry,
        &ToolRegistry::new(),
    )
    .expect("predicate-bound replay");
    assert_eq!(
        replayed.predicate_registry_root_t,
        h.registry.merkle_root_hash()
    );
}

#[tokio::test]
async fn legacy_replay_without_predicate_cas_cannot_reconstruct_activation() {
    let mut h = harness();
    let snapshot_cid = put_snapshot(&h);
    h.seq
        .emit_system_tx(SystemEmitCommand::PredicateBindingActivate {
            registry_snapshot_cid: snapshot_cid,
            registry_merkle_root: h.registry.merkle_root_hash(),
        })
        .await
        .expect("emit");
    let entry = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("applied")
        .expect("activation accepts");
    let cas = h.cas.read().expect("cas");
    let err = replay_full_transition(
        &QState::default(),
        &[entry],
        &*cas,
        &h.pinned,
        &h.registry,
        &ToolRegistry::new(),
    )
    .expect_err("legacy replay has no predicate CAS view");
    assert!(format!("{err:?}").contains("PredicateBindingActivationInvalid"));
}
