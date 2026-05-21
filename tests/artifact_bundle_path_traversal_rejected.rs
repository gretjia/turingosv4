use turingosv4::runtime::artifact_bundle::{
    write_artifact_bundle, ArtifactBundleManifest, ArtifactFileEntry, ArtifactFileRole
};

#[test]
fn test_artifact_bundle_path_traversal_rejected() {
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
        entrypoint: "index.html".to_string(),
        bundle_size_bytes_total: 120,
        created_at_logical_t: 12345,
    };

    // 1. Starts with /
    manifest.files[0].path = "/index.html".to_string();
    manifest.entrypoint = "/index.html".to_string();
    assert!(write_artifact_bundle(tmp.path(), &manifest).is_err());

    // 2. Contains .. at start
    manifest.files[0].path = "../index.html".to_string();
    manifest.entrypoint = "../index.html".to_string();
    assert!(write_artifact_bundle(tmp.path(), &manifest).is_err());

    // 3. Contains .. in middle
    manifest.files[0].path = "foo/../index.html".to_string();
    manifest.entrypoint = "foo/../index.html".to_string();
    assert!(write_artifact_bundle(tmp.path(), &manifest).is_err());

    // 4. Empty path
    manifest.files[0].path = "".to_string();
    manifest.entrypoint = "".to_string();
    assert!(write_artifact_bundle(tmp.path(), &manifest).is_err());
}
