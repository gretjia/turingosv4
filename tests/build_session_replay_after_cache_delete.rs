//! C9 gate: offline replay is byte-stable after CAS sidecar cache delete.
//!
//! Confirms that `reconstruct_session` produces an identical result on
//! re-run after the `.turingos_cas_index.jsonl` sidecar (the CasStore
//! reload cache) has been deleted from disk. This is the "delete cache and
//! re-derive" replay property required by master plan §C9.
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

#[test]
fn test_build_session_replay_after_cache_delete() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let workspace = dir.path();
    let session_id = "test-c9-cache-delete";

    let cas_dir = cas_path(workspace);
    std::fs::create_dir_all(&cas_dir).expect("create cas dir");
    let mut store = CasStore::open(&cas_dir).expect("open cas store");

    let t = now_t();

    // Write a small chain: attempt + bundle
    let attempt = GenerationAttemptCapsule {
        schema_id: GENERATION_ATTEMPT_CAPSULE_SCHEMA_ID.to_string(),
        session_id: session_id.to_string(),
        spec_capsule_cid: None,
        spec_source: "ondisk_spec_md".to_string(),
        model_id: "test-model".to_string(),
        model_seed: None,
        prompt_hash: "abc123".to_string(),
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

    let file_content = b"<html>cache-delete</html>";
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
    let _manifest_cid = store
        .put(
            &serde_json::to_vec(&manifest).unwrap(),
            ObjectType::EvidenceCapsule,
            "test",
            t + 1,
            Some(ARTIFACT_BUNDLE_SCHEMA_ID.to_string()),
        )
        .expect("put manifest");

    // First reconstruction (populates any in-memory CasStore cache).
    let first = reconstruct_session(workspace, session_id).expect("first replay");
    let first_step_count = first.steps.len();
    let first_view_attempts = first.view.generation_attempts.clone();
    let first_view_artifacts = first.view.artifact_versions.clone();

    // Delete the CAS sidecar index file (the .turingos_cas_index.jsonl reload cache).
    // Replay should still succeed because the git-backed CAS chain is authoritative.
    let sidecar = cas_dir.join(".turingos_cas_index.jsonl");
    if sidecar.exists() {
        std::fs::remove_file(&sidecar).expect("remove sidecar");
    }

    // Second reconstruction after cache delete.
    let second = reconstruct_session(workspace, session_id).expect("second replay");
    let second_step_count = second.steps.len();
    let second_view_attempts = second.view.generation_attempts.clone();
    let second_view_artifacts = second.view.artifact_versions.clone();

    assert_eq!(
        first_step_count, second_step_count,
        "step counts must match before/after cache delete"
    );
    assert_eq!(
        first_view_attempts, second_view_attempts,
        "view.generation_attempts must be byte-stable"
    );
    assert_eq!(
        first_view_artifacts, second_view_artifacts,
        "view.artifact_versions must be byte-stable"
    );

    // Both reconstructions must have no dangling refs.
    assert!(first.dangling_cid_errors.is_empty());
    assert!(second.dangling_cid_errors.is_empty());
}
