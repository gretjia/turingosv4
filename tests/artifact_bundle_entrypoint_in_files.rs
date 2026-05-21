use turingosv4::runtime::artifact_bundle::{
    write_artifact_bundle, ArtifactBundleManifest, ArtifactFileEntry, ArtifactFileRole
};

#[test]
fn test_artifact_bundle_entrypoint_must_be_in_files() {
    let tmp = tempfile::tempdir().expect("create temp workspace");
    
    let mut manifest = ArtifactBundleManifest {
        schema_id: "turingos-artifact-bundle-v1".to_string(),
        session_id: "test_session_123".to_string(),
        spec_capsule_cid: Some("a".repeat(64)),
        generation_attempt_cid: "b".repeat(64),
        previous_bundle_cid: None,
        files: vec![
            ArtifactFileEntry {
                path: "index.html".to_string(),
                cid: "c".repeat(64),
                mime: "text/html".to_string(),
                sha256: "d".repeat(64),
                size_bytes: 120,
                role: ArtifactFileRole::Entrypoint,
            }
        ],
        entrypoint: "other.html".to_string(),
        bundle_size_bytes_total: 120,
        created_at_logical_t: 12345,
    };

    assert!(write_artifact_bundle(tmp.path(), &manifest).is_err());

    // Fix it
    manifest.entrypoint = "index.html".to_string();
    assert!(write_artifact_bundle(tmp.path(), &manifest).is_ok());
}
