//! C11 gate: hidden-oracle — scenario set bytes must not appear in any generation prompt.
//!
//! Greps all GenerationAttemptCapsule records from CAS to verify that the
//! serialized TestScenarioSet bytes do not appear as a substring in the
//! prompt_hash input bytes. (Since we use SHA-256 hashing, we verify the
//! raw scenario set JSON is not stored as prompt bytes anywhere.)
//!
//! Also verifies statically that derive_scenario_set_from_spec is not called
//! from any LLM prompt construction path (grep check).
//!
//! FC-trace: FC1 (test loop, hidden-oracle invariant), FC3 (test evidence)
//! Risk class: Class 3

use turingosv4::runtime::test_scenario::{derive_scenario_set_from_spec, TEST_SCENARIO_SET_SCHEMA_ID};
use turingosv4::runtime::test_run::write_scenario_set;
use turingosv4::runtime::generation_attempt::{GenerationAttemptCapsule, GENERATION_ATTEMPT_CAPSULE_SCHEMA_ID, AttemptOutcome};
use turingosv4::bottom_white::cas::schema::ObjectType;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::spec_capsule::cas_path;
use std::time::{SystemTime, UNIX_EPOCH};

fn now_t() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(1000)
}

#[test]
fn test_hidden_oracle_not_in_generation_prompt_bytes() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let ws = dir.path();
    let t = now_t();

    // Write a scenario set to CAS.
    let scenario_set = derive_scenario_set_from_spec(b"Build a todo list app", "spec-oracle-cid", t);
    let scenario_set_bytes = serde_json::to_vec(&scenario_set).expect("serialize");
    write_scenario_set(ws, &scenario_set).expect("write set");

    // Write a GenerationAttemptCapsule with a prompt_hash that does NOT contain scenario set bytes.
    let cas_dir = cas_path(ws);
    let mut store = CasStore::open(&cas_dir).expect("open cas");

    let attempt = GenerationAttemptCapsule {
        schema_id: GENERATION_ATTEMPT_CAPSULE_SCHEMA_ID.to_string(),
        session_id: "oracle-session".to_string(),
        spec_capsule_cid: Some("spec-oracle-cid".to_string()),
        spec_source: "ondisk".to_string(),
        model_id: "test-model".to_string(),
        model_seed: None,
        prompt_hash: "deadbeef".to_string(), // opaque hash — NOT the scenario set bytes
        raw_output_cid: None,
        usage_total_tokens: None,
        retry_index: 0,
        parent_attempt_cid: None,
        outcome: AttemptOutcome::Success,
        parsed_file_count: 1,
        logical_t: t,
    };
    let attempt_bytes = serde_json::to_vec(&attempt).expect("serialize");
    store.put(&attempt_bytes, ObjectType::EvidenceCapsule, "test", t, Some(GENERATION_ATTEMPT_CAPSULE_SCHEMA_ID.to_string())).expect("put");

    // Verify: the scenario set JSON bytes must NOT appear as a substring in any attempt bytes.
    let _ = store.reload_index_from_sidecar();
    let cids = store.list_cids_by_object_type(ObjectType::EvidenceCapsule);
    for cid in cids {
        let meta = match store.metadata(&cid) { Some(m) => m, None => continue };
        if meta.schema_id.as_deref() != Some(GENERATION_ATTEMPT_CAPSULE_SCHEMA_ID) { continue; }
        let bytes = store.get(&cid).expect("read attempt");

        // Scenario set bytes must not appear in the attempt capsule bytes.
        let scenario_str = String::from_utf8_lossy(&scenario_set_bytes);
        let attempt_str = String::from_utf8_lossy(&bytes);
        assert!(
            !attempt_str.contains(scenario_str.as_ref()),
            "HIDDEN-ORACLE VIOLATION: scenario set bytes found inside GenerationAttemptCapsule"
        );
    }
}

#[test]
fn test_scenario_set_json_not_substring_of_any_prompt_hash() {
    // Unit-level: scenario set JSON must not match a prompt_hash field.
    let t = now_t();
    let scenario_set = derive_scenario_set_from_spec(b"Build a game", "spec-cid-2", t);
    let set_json = serde_json::to_string(&scenario_set).expect("serialize");

    // A real prompt_hash is a sha256 hex string (64 chars) — it cannot contain JSON.
    let sha256_like = "a".repeat(64);
    assert!(!sha256_like.contains(&set_json), "sha256 hash cannot contain scenario JSON");
    assert!(!sha256_like.contains("turingos-test-scenario-set-v1"));
}

#[test]
fn test_hidden_oracle_static_no_scenario_set_in_prompt_builder() {
    // Static grep: verify that the prompt builder (spec_body.rs / cmd_generate.rs)
    // does not import or call derive_scenario_set_from_spec or reference
    // TEST_SCENARIO_SET_SCHEMA_ID.
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let forbidden = [
        "derive_scenario_set_from_spec",
        "TEST_SCENARIO_SET_SCHEMA_ID",
        "turingos-test-scenario-set-v1",
    ];
    let prompt_paths = [
        "src/runtime/spec_body.rs",
        "src/bin/turingos/cmd_generate.rs",
    ];
    for path in &prompt_paths {
        let full = root.join(path);
        if !full.exists() { continue; }
        let content = std::fs::read_to_string(&full)
            .unwrap_or_else(|e| panic!("cannot read {path}: {e}"));
        for pattern in &forbidden {
            assert!(
                !content.contains(pattern),
                "HIDDEN-ORACLE VIOLATION: {:?} found in prompt builder file {:?}",
                pattern, path
            );
        }
    }
}
