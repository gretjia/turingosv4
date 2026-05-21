//! C9 gate: offline replay reads artifact bundle (and its referenced files) from CAS.
//!
//! Confirms that `reconstruct_session` enumerates the ArtifactBundle step,
//! that the bundle's file CIDs resolve in CAS, and that the read-back manifest
//! cross-references the GenerationAttempt CID correctly.
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
use turingosv4::runtime::replay::{reconstruct_session, ReplayStep};
use turingosv4::runtime::spec_capsule::cas_path;

fn now_t() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(1000)
}

#[test]
fn test_artifact_bundle_replay_reads_cas() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let workspace = dir.path();
    let session_id = "test-c9-bundle-replay";

    let cas_dir = cas_path(workspace);
    std::fs::create_dir_all(&cas_dir).expect("create cas dir");
    let mut store = CasStore::open(&cas_dir).expect("open cas store");

    let t = now_t();

    // 1. Write a GenerationAttemptCapsule
    let attempt = GenerationAttemptCapsule {
        schema_id: GENERATION_ATTEMPT_CAPSULE_SCHEMA_ID.to_string(),
        session_id: session_id.to_string(),
        spec_capsule_cid: None,
        spec_source: "ondisk_spec_md".to_string(),
        model_id: "test-model".to_string(),
        model_seed: None,
        prompt_hash: "deadbeef".to_string(),
        raw_output_cid: None,
        usage_total_tokens: None,
        retry_index: 0,
        parent_attempt_cid: None,
        outcome: AttemptOutcome::Success,
        parsed_file_count: 1,
        logical_t: t,
    };
    let attempt_bytes = serde_json::to_vec(&attempt).expect("serialize");
    let attempt_cid = store
        .put(
            &attempt_bytes,
            ObjectType::EvidenceCapsule,
            "test",
            t,
            Some(GENERATION_ATTEMPT_CAPSULE_SCHEMA_ID.to_string()),
        )
        .expect("put attempt");

    // 2. Write a file blob (the actual artifact content)
    let file_content = b"<html><body>Hello from artifact bundle replay test</body></html>";
    let file_cid = store
        .put(file_content, ObjectType::EvidenceCapsule, "test", t, None)
        .expect("put file blob");

    // 3. Write an ArtifactBundleManifest referencing the attempt + file
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
    let manifest_bytes = serde_json::to_vec(&manifest).expect("serialize manifest");
    let manifest_cid = store
        .put(
            &manifest_bytes,
            ObjectType::EvidenceCapsule,
            "test",
            t + 1,
            Some(ARTIFACT_BUNDLE_SCHEMA_ID.to_string()),
        )
        .expect("put manifest");

    // 4. Reconstruct session via offline replay
    let result = reconstruct_session(workspace, session_id).expect("reconstruct ok");

    // 5. ArtifactBundle step must be present and reference the manifest CID
    let bundle_step_found = result.steps.iter().any(|s| match s {
        ReplayStep::ArtifactBundle { cid, .. } => cid == &manifest_cid.hex(),
        _ => false,
    });
    assert!(
        bundle_step_found,
        "ArtifactBundle step with cid={} not found in steps={:?}",
        manifest_cid.hex(),
        result.steps
    );

    // 6. No dangling refs — all CIDs resolved in CAS
    assert!(
        result.dangling_cid_errors.is_empty(),
        "expected no dangling refs, got: {:?}",
        result.dangling_cid_errors
    );
}
