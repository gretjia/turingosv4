//! REAL-BCAST-1 — LibrarianDigest gates.

use tempfile::TempDir;
use turingosv4::bottom_white::cas::schema::{Cid, ObjectType};
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::librarian_broadcast::{
    build_librarian_digest, librarian_digest_cids, read_librarian_digest_from_cas,
    validate_broadcast_epoch, validate_librarian_digest, write_librarian_digest_to_cas,
    BroadcastEpoch, LibrarianEvidenceEvent, LibrarianEvidenceKind, LibrarianSourceScope,
    LIBRARIAN_DIGEST_SCHEMA_ID,
};

fn event(label: &str) -> LibrarianEvidenceEvent {
    LibrarianEvidenceEvent {
        cid: Cid::from_content(label.as_bytes()),
        kind: LibrarianEvidenceKind::LeanError,
        class_label: "err:type_mismatch".into(),
        task_id: Some("task-algebra".into()),
        public_summary: "Repeated Lean type mismatch in externalized tactic".into(),
        head_t: 7,
    }
}

#[test]
fn digest_clusters_repeated_errors_and_is_deterministic() {
    let scope = LibrarianSourceScope {
        current_run_cas_root: Cid::from_content(b"root"),
        prior_capsule_cids: vec![],
        max_prior_batches: 0,
        task_tags: vec!["algebra".into()],
    };
    let a = build_librarian_digest(scope.clone(), 10, vec![event("a"), event("b")]).unwrap();
    let b = build_librarian_digest(scope, 10, vec![event("b"), event("a")]).unwrap();

    assert_eq!(
        a, b,
        "same source scope/events must build deterministic digest"
    );
    assert_eq!(a.typical_error_clusters.len(), 1);
    assert_eq!(a.typical_error_clusters[0].count, 2);
    validate_librarian_digest(&a).unwrap();
}

#[test]
fn digest_is_generic_cas_backed_and_readable() {
    let tmp = TempDir::new().unwrap();
    let mut cas = CasStore::open(tmp.path()).unwrap();
    let scope = LibrarianSourceScope {
        current_run_cas_root: Cid::from_content(b"root"),
        prior_capsule_cids: vec![],
        max_prior_batches: 0,
        task_tags: vec!["algebra".into()],
    };
    let digest = build_librarian_digest(scope, 10, vec![event("a"), event("b")]).unwrap();
    let cid = write_librarian_digest_to_cas(&mut cas, &digest, "digest", 10).unwrap();
    let meta = cas.metadata(&cid).unwrap();
    assert_eq!(meta.object_type, ObjectType::Generic);
    assert_eq!(meta.schema_id.as_deref(), Some(LIBRARIAN_DIGEST_SCHEMA_ID));
    assert_eq!(librarian_digest_cids(&cas), vec![cid]);
    assert_eq!(read_librarian_digest_from_cas(&cas, &cid).unwrap(), digest);
}

#[test]
fn broadcast_epoch_rejects_future_or_out_of_window_digest() {
    let epoch = BroadcastEpoch {
        epoch_id: "epoch-1".into(),
        source_head_t: 10,
        digest_cid: Cid::from_content(b"digest"),
        valid_from: 11,
        valid_until: 20,
        task_tags: vec!["algebra".into()],
    };
    validate_broadcast_epoch(&epoch, 15).unwrap();
    assert!(validate_broadcast_epoch(&epoch, 9)
        .unwrap_err()
        .contains("future digest"));
    assert!(validate_broadcast_epoch(&epoch, 21)
        .unwrap_err()
        .contains("expired"));
}
