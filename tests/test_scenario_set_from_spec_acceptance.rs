//! C11 gate: TestScenarioSet derived from spec and scenarios are replayable from CAS.
//!
//! FC-trace: FC1 (test loop), FC3 (test evidence)
//! Risk class: Class 3

use std::time::{SystemTime, UNIX_EPOCH};
use turingosv4::runtime::test_scenario::{
    derive_scenario_set_from_spec, TestScenario, TEST_SCENARIO_SET_SCHEMA_ID,
};
use turingosv4::runtime::test_run::{write_scenario_set, TEST_RUN_CAPSULE_SCHEMA_ID};

fn now_t() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(1000)
}

#[test]
fn test_derive_scenario_set_includes_basic_scenarios() {
    let set = derive_scenario_set_from_spec(b"Build a todo list app", "cid1", now_t());
    assert_eq!(set.schema_id, TEST_SCENARIO_SET_SCHEMA_ID);
    assert_eq!(set.spec_capsule_cid, "cid1");
    assert!(set.scenarios.iter().any(|s| matches!(s, TestScenario::EntrypointExists)));
    assert!(set.scenarios.iter().any(|s| matches!(s, TestScenario::HtmlParses)));
    assert_eq!(set.scenarios.len(), 2, "no sandbox keyword → 2 scenarios");
}

#[test]
fn test_derive_scenario_set_adds_sandbox_from_spec() {
    let set = derive_scenario_set_from_spec(b"Build a todo list with sandbox policy", "cid2", now_t());
    assert_eq!(set.scenarios.len(), 3, "sandbox keyword → 3 scenarios");
    assert!(set.scenarios.iter().any(|s| matches!(s, TestScenario::SandboxPolicyPreserved { .. })));
}

#[test]
fn test_scenario_set_written_to_cas_and_readable() {
    use turingosv4::runtime::spec_capsule::cas_path;
    use turingosv4::bottom_white::cas::schema::ObjectType;
    use turingosv4::bottom_white::cas::store::CasStore;
    use turingosv4::runtime::test_scenario::TestScenarioSet;

    let dir = tempfile::TempDir::new().expect("tempdir");
    let ws = dir.path();

    let set = derive_scenario_set_from_spec(b"Build a web app", "spec-cid-1", now_t());
    let cid_hex = write_scenario_set(ws, &set).expect("write scenario set");
    assert!(!cid_hex.is_empty(), "scenario set CID must not be empty");

    // Read back from CAS
    let cas_dir = cas_path(ws);
    let mut store = CasStore::open(&cas_dir).expect("open cas");
    let _ = store.reload_index_from_sidecar();

    let cids = store.list_cids_by_object_type(ObjectType::EvidenceCapsule);
    let mut found = false;
    for cid in cids {
        let meta = match store.metadata(&cid) { Some(m) => m, None => continue };
        if meta.schema_id.as_deref() != Some(TEST_SCENARIO_SET_SCHEMA_ID) { continue; }
        let bytes = store.get(&cid).expect("read");
        let back: TestScenarioSet = serde_json::from_slice(&bytes).expect("deserialize");
        assert_eq!(back.spec_capsule_cid, "spec-cid-1");
        assert!(back.scenarios.iter().any(|s| matches!(s, TestScenario::EntrypointExists)));
        found = true;
    }
    assert!(found, "TestScenarioSet not found in CAS");
}

#[test]
fn test_test_run_capsule_schema_id_correct() {
    use turingosv4::runtime::test_run::TestRunCapsule;
    use turingosv4::runtime::test_run::TestScenarioResult;

    let cap = TestRunCapsule {
        schema_id: TEST_RUN_CAPSULE_SCHEMA_ID.to_string(),
        artifact_bundle_cid: "a".repeat(64),
        test_scenario_set_cid: "b".repeat(64),
        results: vec![TestScenarioResult {
            scenario: TestScenario::EntrypointExists,
            pass: true,
            detail: "ok".to_string(),
        }],
        overall_pass: true,
        logical_t: 1000,
    };
    assert_eq!(cap.schema_id, "turingos-test-run-v1");
    assert_eq!(cap.overall_pass, cap.results.iter().all(|r| r.pass));
}
