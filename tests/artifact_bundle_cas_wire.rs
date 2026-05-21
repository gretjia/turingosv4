use turingosv4::runtime::artifact_bundle::{
    write_artifact_bundle, ArtifactBundleManifest, ArtifactFileEntry, ArtifactFileRole
};
use turingosv4::bottom_white::cas::store::CasStore;

fn parse_cid_hex(s: &str) -> turingosv4::bottom_white::cas::schema::Cid {
    let mut out = [0u8; 32];
    for (i, byte) in out.iter_mut().enumerate() {
        let chunk = &s[i * 2..i * 2 + 2];
        *byte = u8::from_str_radix(chunk, 16).unwrap();
    }
    turingosv4::bottom_white::cas::schema::Cid(out)
}

#[test]
fn test_artifact_bundle_cas_wire_round_trip() {
    let tmp = tempfile::tempdir().expect("create temp workspace");
    let manifest = ArtifactBundleManifest {
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

    let cid = write_artifact_bundle(tmp.path(), &manifest).expect("write bundle");
    assert_eq!(cid.len(), 64);

    let cas_dir = tmp.path().join("cas");
    let store = CasStore::open(&cas_dir).expect("open store");
    let cid_obj = parse_cid_hex(&cid);
    let bytes = store.get(&cid_obj).expect("get bytes");
    let read_manifest: ArtifactBundleManifest = serde_json::from_slice(&bytes).expect("deserialize");
    assert_eq!(read_manifest, manifest);
}
