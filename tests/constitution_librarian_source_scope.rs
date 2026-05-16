//! REAL-BCAST-1 — LibrarianSourceScope gates.

use tempfile::TempDir;
use turingosv4::bottom_white::cas::schema::{Cid, ObjectType};
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::librarian_broadcast::{
    validate_librarian_source_scope, LibrarianSourceScope,
};

fn cid(label: &str) -> Cid {
    Cid::from_content(label.as_bytes())
}

#[test]
fn source_scope_requires_current_run_cas_root() {
    let tmp = TempDir::new().unwrap();
    let cas = CasStore::open(tmp.path()).unwrap();
    let scope = LibrarianSourceScope {
        current_run_cas_root: Cid::default(),
        prior_capsule_cids: vec![],
        max_prior_batches: 0,
        task_tags: vec!["minif2f".into()],
    };

    assert!(validate_librarian_source_scope(&scope, &cas)
        .unwrap_err()
        .contains("current_run_cas_root"));
}

#[test]
fn source_scope_resolves_prior_capsules_or_fails_closed() {
    let tmp = TempDir::new().unwrap();
    let mut cas = CasStore::open(tmp.path()).unwrap();
    let prior = cas
        .put(
            b"prior capsule",
            ObjectType::Generic,
            "test-prior",
            1,
            Some("prior.capsule.v1".into()),
        )
        .unwrap();
    let scope = LibrarianSourceScope {
        current_run_cas_root: cid("root"),
        prior_capsule_cids: vec![prior],
        max_prior_batches: 1,
        task_tags: vec!["algebra".into()],
    };
    validate_librarian_source_scope(&scope, &cas).unwrap();

    let missing = LibrarianSourceScope {
        prior_capsule_cids: vec![cid("missing")],
        ..scope
    };
    assert!(validate_librarian_source_scope(&missing, &cas)
        .unwrap_err()
        .contains("unresolved prior_capsule_cid"));
}

#[test]
fn source_scope_rejects_global_pointer_inputs() {
    let tmp = TempDir::new().unwrap();
    let cas = CasStore::open(tmp.path()).unwrap();
    let scope = LibrarianSourceScope {
        current_run_cas_root: cid("root"),
        prior_capsule_cids: vec![],
        max_prior_batches: 0,
        task_tags: vec!["LATEST_MARKOV_CAPSULE.txt".into()],
    };

    assert!(validate_librarian_source_scope(&scope, &cas)
        .unwrap_err()
        .contains("global pointer"));
}
