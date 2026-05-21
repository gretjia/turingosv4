//! C9 gate: offline replay verifies all cross-CID references resolve in CAS.
//!
//! Confirms that `reconstruct_session` returns an empty `dangling_cid_errors`
//! when all referenced CIDs resolve, and a non-empty list when a manifest
//! references a CID that does NOT exist in CAS.
//!
//! FC-trace: FC1 (replay loop), FC2 (boot reconstruction)
//! Risk class: Class 2

use std::time::{SystemTime, UNIX_EPOCH};

use turingosv4::bottom_white::cas::schema::ObjectType;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::artifact_bundle::{
    ArtifactBundleManifest, ArtifactFileEntry, ArtifactFileRole, ARTIFACT_BUNDLE_SCHEMA_ID,
};
use turingosv4::runtime::generation_attempt::{
    AttemptOutcome, GenerationAttemptCapsule, GENERATION_ATTEMPT_CAPSULE_SCHEMA_ID,
};
use turingosv4::runtime::replay::reconstruct_session;
use turingosv4::runtime::spec_capsule::cas_path;

fn now_t() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(1000)
}

const FAKE_DANGLING_CID: &str =
    "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";

#[test]
fn test_replay_clean_chain_has_no_dangling_refs() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let workspace = dir.path();
    let session_id = "test-c9-clean-refs";

    let cas_dir = cas_path(workspace);
    std::fs::create_dir_all(&cas_dir).expect("create cas dir");
    let mut store = CasStore::open(&cas_dir).expect("open cas store");

    let t = now_t();

    let attempt = GenerationAttemptCapsule {
        schema_id: GENERATION_ATTEMPT_CAPSULE_SCHEMA_ID.to_string(),
        session_id: session_id.to_string(),
        spec_capsule_cid: None,
        spec_source: "ondisk_spec_md".to_string(),
        model_id: "test".to_string(),
        model_seed: None,
        prompt_hash: "aa".to_string(),
        raw_output_cid: None,
        usage_total_tokens: None,
        retry_index: 0,
        parent_attempt_cid: None,
        outcome: AttemptOutcome::Success,
        parsed_file_count: 1,
        logical_t: t,
    };
    let attempt_cid = store
        .put(
            &serde_json::to_vec(&attempt).unwrap(),
            ObjectType::EvidenceCapsule,
            "test",
            t,
            Some(GENERATION_ATTEMPT_CAPSULE_SCHEMA_ID.to_string()),
        )
        .expect("put attempt");

    let file_content = b"clean";
    let file_cid = store
        .put(file_content, ObjectType::EvidenceCapsule, "test", t, None)
        .expect("put file");

    let manifest = ArtifactBundleManifest {
        schema_id: ARTIFACT_BUNDLE_SCHEMA_ID.to_string(),
        session_id: session_id.to_string(),
        spec_capsule_cid: None,
        generation_attempt_cid: attempt_cid.hex(),
        previous_bundle_cid: None,
        files: vec![ArtifactFileEntry {
            path: "index.html".to_string(),
            cid: file_cid.hex(),
            mime: "text/html".to_string(),
            sha256: file_cid.hex(),
            size_bytes: file_content.len() as u64,
            role: ArtifactFileRole::Entrypoint,
        }],
        entrypoint: "index.html".to_string(),
        bundle_size_bytes_total: file_content.len() as u64,
        created_at_logical_t: t + 1,
    };
    store
        .put(
            &serde_json::to_vec(&manifest).unwrap(),
            ObjectType::EvidenceCapsule,
            "test",
            t + 1,
            Some(ARTIFACT_BUNDLE_SCHEMA_ID.to_string()),
        )
        .expect("put manifest");

    let result = reconstruct_session(workspace, session_id).expect("replay");
    assert!(
        result.dangling_cid_errors.is_empty(),
        "expected empty dangling_cid_errors, got: {:?}",
        result.dangling_cid_errors
    );
}

#[test]
fn test_replay_dangling_ref_is_reported() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let workspace = dir.path();
    let session_id = "test-c9-dangling";

    let cas_dir = cas_path(workspace);
    std::fs::create_dir_all(&cas_dir).expect("create cas dir");
    let mut store = CasStore::open(&cas_dir).expect("open cas store");

    let t = now_t();

    // Write a file blob (real CID) but NOT the generation attempt — the manifest
    // will reference a non-existent generation_attempt_cid.
    let file_content = b"dangling-ref-test";
    let file_cid = store
        .put(file_content, ObjectType::EvidenceCapsule, "test", t, None)
        .expect("put file");

    let manifest = ArtifactBundleManifest {
        schema_id: ARTIFACT_BUNDLE_SCHEMA_ID.to_string(),
        session_id: session_id.to_string(),
        spec_capsule_cid: None,
        generation_attempt_cid: FAKE_DANGLING_CID.to_string(),
        previous_bundle_cid: None,
        files: vec![ArtifactFileEntry {
            path: "index.html".to_string(),
            cid: file_cid.hex(),
            mime: "text/html".to_string(),
            sha256: file_cid.hex(),
            size_bytes: file_content.len() as u64,
            role: ArtifactFileRole::Entrypoint,
        }],
        entrypoint: "index.html".to_string(),
        bundle_size_bytes_total: file_content.len() as u64,
        created_at_logical_t: t,
    };
    store
        .put(
            &serde_json::to_vec(&manifest).unwrap(),
            ObjectType::EvidenceCapsule,
            "test",
            t,
            Some(ARTIFACT_BUNDLE_SCHEMA_ID.to_string()),
        )
        .expect("put manifest");

    let result = reconstruct_session(workspace, session_id).expect("replay");
    assert!(
        !result.dangling_cid_errors.is_empty(),
        "expected non-empty dangling_cid_errors when generation_attempt_cid references a CID not in CAS"
    );
    // At least one error must mention the fake CID we used.
    let mentions_fake = result
        .dangling_cid_errors
        .iter()
        .any(|e| e.contains(FAKE_DANGLING_CID));
    assert!(
        mentions_fake,
        "dangling_cid_errors must mention the fake CID; got: {:?}",
        result.dangling_cid_errors
    );
}
