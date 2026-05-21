use turingosv4::runtime::generation_attempt::{
    write_generation_attempt_capsule, GenerationAttemptCapsule, AttemptOutcome
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
fn test_generation_attempt_capsule_cas_wire_round_trip() {
    let tmp = tempfile::tempdir().expect("create temp workspace");
    let capsule = GenerationAttemptCapsule {
        schema_id: "turingos-generation-attempt-v1".to_string(),
        session_id: "test_session_123".to_string(),
        spec_capsule_cid: Some("a".repeat(64)),
        spec_source: "cas_capsule".to_string(),
        model_id: "test-model".to_string(),
        model_seed: None,
        prompt_hash: "b".repeat(64),
        raw_output_cid: Some("c".repeat(64)),
        usage_total_tokens: Some(100),
        retry_index: 0,
        parent_attempt_cid: None,
        outcome: AttemptOutcome::Success,
        parsed_file_count: 1,
        logical_t: 12345,
    };

    let cid = write_generation_attempt_capsule(tmp.path(), &capsule).expect("write capsule");
    assert_eq!(cid.len(), 64);

    let cas_dir = tmp.path().join("cas");
    let store = CasStore::open(&cas_dir).expect("open store");
    let cid_obj = parse_cid_hex(&cid);
    let bytes = store.get(&cid_obj).expect("get bytes");
    let read_capsule: GenerationAttemptCapsule = serde_json::from_slice(&bytes).expect("deserialize");
    assert_eq!(read_capsule, capsule);
}
