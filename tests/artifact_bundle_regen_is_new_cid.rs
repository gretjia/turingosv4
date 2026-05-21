use turingosv4::runtime::artifact_bundle::{
    write_artifact_bundle, latest_artifact_bundle_cid_for_session, ArtifactBundleManifest, ArtifactFileEntry, ArtifactFileRole
};

#[test]
fn test_artifact_bundle_regen_is_new_cid() {
    let tmp = tempfile::tempdir().expect("create temp workspace");
    
    let manifest1 = ArtifactBundleManifest {
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

    let cid1 = write_artifact_bundle(tmp.path(), &manifest1).expect("write first bundle");
    
    let manifest2 = ArtifactBundleManifest {
        schema_id: "turingos-artifact-bundle-v1".to_string(),
        session_id: "test_session_123".to_string(),
        spec_capsule_cid: Some("a".repeat(64)),
        generation_attempt_cid: "e".repeat(64),
        previous_bundle_cid: Some(cid1.clone()),
        files: vec![
            ArtifactFileEntry {
                path: "index.html".to_string(),
                cid: "f".repeat(64),
                mime: "text/html".to_string(),
                sha256: "g".repeat(64),
                size_bytes: 150,
                role: ArtifactFileRole::Entrypoint,
            }
        ],
        entrypoint: "index.html".to_string(),
        bundle_size_bytes_total: 150,
        created_at_logical_t: 12346,
    };

    let cid2 = write_artifact_bundle(tmp.path(), &manifest2).expect("write second bundle");
    assert_ne!(cid1, cid2);

    let latest_cid = latest_artifact_bundle_cid_for_session(tmp.path(), "test_session_123")
        .expect("get latest")
        .expect("should find a latest bundle");
    assert_eq!(latest_cid, cid2);
}
