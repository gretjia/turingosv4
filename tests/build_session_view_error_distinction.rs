//! TB-SOFTWARE-3-0 Atom S3 (2026-05-23): three distinct outcomes of
//! `derive_build_session_view` — empty-ok, corrupt-error, bad-capsule-error.
//!
//! FC-trace: FC2-N16 (derived view error taxonomy), FC3 (CAS evidence)
//! Risk class: Class 2.
//!
//! Plan §3 (S3) requires that:
//!   1. Missing CAS / empty session  →  `Ok(BuildStatus::SpecPending)`
//!   2. Unopenable CAS (file at the dir path, damaged repo)
//!                                    →  `Err(BuildSessionViewError::Open)`
//!   3. Schema-id-matched capsule whose bytes won't deserialize
//!                                    →  `Err(BuildSessionViewError::Decode)`

use std::time::{SystemTime, UNIX_EPOCH};
use turingosv4::bottom_white::cas::schema::ObjectType;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::build_session_view::{
    derive_build_session_view, BuildSessionViewError, BuildStatus,
};
use turingosv4::runtime::spec_capsule::cas_path;

fn now_t() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(1000)
}

#[test]
fn empty_workspace_returns_ok_spec_pending() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let ws = tmp.path();

    let view = derive_build_session_view(ws, "sess_empty").expect("empty is Ok");

    assert_eq!(view.session_id, "sess_empty");
    assert_eq!(view.current_status, BuildStatus::SpecPending);
    assert!(view.spec_capsule_cid.is_none());
    assert!(view.generation_attempts.is_empty());
    assert!(view.artifact_versions.is_empty());
    assert!(view.preview_runs.is_empty());
    assert!(view.rejection_events.is_empty());
    assert!(!view.accepted_delivery);
}

#[test]
fn corrupt_cas_returns_open_error() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let ws = tmp.path();

    // Plant a regular file where the cas directory would live. CasStore::open
    // tries `Repository::open(path)`, then `Repository::init(path)` — both
    // fail when the path is a regular file, surfacing the Open variant.
    let cas_path_pb = cas_path(ws);
    std::fs::write(&cas_path_pb, b"not a directory").expect("plant file");

    let err = derive_build_session_view(ws, "sess_corrupt")
        .expect_err("file-at-cas-path must error");

    match err {
        BuildSessionViewError::Open(_) => {} // expected
        other => panic!("expected BuildSessionViewError::Open, got {other:?}"),
    }
}

#[test]
fn bad_capsule_returns_decode_error() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let ws = tmp.path();
    let session_id = "sess_bad_capsule";

    // Bootstrap a real CAS, then write a capsule tagged with the
    // spec-grill-session schema_id but whose body isn't a valid
    // GrillSessionCapsuleBody — decode must fail.
    let cas_dir = cas_path(ws);
    std::fs::create_dir_all(&cas_dir).expect("create cas dir");
    let mut store = CasStore::open(&cas_dir).expect("open cas");
    let bogus = b"{ this is not valid spec-grill-session JSON }";
    let _cid = store
        .put(
            bogus,
            ObjectType::EvidenceCapsule,
            "test",
            now_t(),
            Some("turingos-spec-grill-session-v1".to_string()),
        )
        .expect("put bogus capsule");

    let err = derive_build_session_view(ws, session_id)
        .expect_err("bad capsule must error");

    match err {
        BuildSessionViewError::Decode(_) => {} // expected
        other => panic!("expected BuildSessionViewError::Decode, got {other:?}"),
    }
}
