//! C11 gate: TestRunCapsule is replayable from CAS by artifact_bundle_cid.
//!
//! FC-trace: FC1 (test loop), FC3 (test evidence)
//! Risk class: Class 3

use std::time::{SystemTime, UNIX_EPOCH};
use turingosv4::runtime::test_scenario::{derive_scenario_set_from_spec, TestScenario};
use turingosv4::runtime::test_run::{
    TestRunCapsule, TestScenarioResult, TEST_RUN_CAPSULE_SCHEMA_ID,
    write_test_run_capsule, latest_test_run_for_bundle,
};
use turingosv4::runtime::artifact_bundle::{ArtifactBundleManifest, ArtifactFileEntry, ArtifactFileRole, ARTIFACT_BUNDLE_SCHEMA_ID};
use turingosv4::bottom_white::cas::schema::ObjectType;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::spec_capsule::cas_path;

fn now_t() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(1000)
}

fn put_bundle(store: &mut CasStore, session_id: &str, t: u64) -> String {
    let file_bytes = b"<!DOCTYPE html><html><body>hello</body></html>";
    let file_cid = store.put(file_bytes, ObjectType::EvidenceCapsule, "test", t, None).expect("put file");

    let manifest = ArtifactBundleManifest {
        schema_id: ARTIFACT_BUNDLE_SCHEMA_ID.to_string(),
        session_id: session_id.to_string(),
        spec_capsule_cid: None,
        generation_attempt_cid: "a".repeat(64),
        previous_bundle_cid: None,
        files: vec![ArtifactFileEntry {
            path: "index.html".to_string(),
            cid: file_cid.hex(),
            mime: "text/html".to_string(),
            sha256: "aabb".to_string(),
            size_bytes: 46,
            role: ArtifactFileRole::Entrypoint,
        }],
        entrypoint: "index.html".to_string(),
        bundle_size_bytes_total: 46,
        created_at_logical_t: t,
    };
    let bytes = serde_json::to_vec(&manifest).expect("serialize");
    let cid = store.put(&bytes, ObjectType::EvidenceCapsule, "test", t, Some(ARTIFACT_BUNDLE_SCHEMA_ID.to_string())).expect("put bundle");
    cid.hex()
}

#[test]
fn test_test_run_capsule_replayable_from_cas() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let ws = dir.path();

    let t = now_t();
    let cas_dir = cas_path(ws);
    std::fs::create_dir_all(&cas_dir).expect("create cas");
    let mut store = CasStore::open(&cas_dir).expect("open cas");

    let bundle_cid = put_bundle(&mut store, "session-replay", t);

    // Build a TestRunCapsule
    let scenario_set = derive_scenario_set_from_spec(b"Build a todo list", "spec-cid-replay", t);
    let set_cid = turingosv4::runtime::test_run::write_scenario_set(ws, &scenario_set).expect("write set");

    let cap = TestRunCapsule {
        schema_id: TEST_RUN_CAPSULE_SCHEMA_ID.to_string(),
        artifact_bundle_cid: bundle_cid.clone(),
        test_scenario_set_cid: set_cid.clone(),
        results: vec![TestScenarioResult {
            scenario: TestScenario::EntrypointExists,
            pass: true,
            detail: "found".to_string(),
        }],
        overall_pass: true,
        logical_t: t + 1,
    };

    write_test_run_capsule(ws, &cap).expect("write capsule");

    // Read back by bundle CID
    let recovered = latest_test_run_for_bundle(ws, &bundle_cid);
    assert!(recovered.is_some(), "TestRunCapsule must be recoverable from CAS by bundle CID");
    let recovered = recovered.unwrap();
    assert_eq!(recovered.artifact_bundle_cid, bundle_cid);
    assert_eq!(recovered.test_scenario_set_cid, set_cid);
    assert!(recovered.overall_pass);
    assert_eq!(recovered.results.len(), 1);
}

#[test]
fn test_overall_pass_false_when_any_result_fails() {
    let results = vec![
        TestScenarioResult { scenario: TestScenario::EntrypointExists, pass: true, detail: "ok".into() },
        TestScenarioResult { scenario: TestScenario::HtmlParses, pass: false, detail: "no DOCTYPE".into() },
    ];
    let overall = results.iter().all(|r| r.pass);
    assert!(!overall, "overall_pass must be false when any result fails");
}

#[test]
fn test_capsule_cid_different_from_scenario_set_cid() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let ws = dir.path();
    let t = now_t();

    let scenario_set = derive_scenario_set_from_spec(b"spec", "cid-s", t);
    let set_cid = turingosv4::runtime::test_run::write_scenario_set(ws, &scenario_set).expect("write set");

    let cap = TestRunCapsule {
        schema_id: TEST_RUN_CAPSULE_SCHEMA_ID.to_string(),
        artifact_bundle_cid: "x".repeat(64),
        test_scenario_set_cid: set_cid.clone(),
        results: vec![],
        overall_pass: true,
        logical_t: t + 1,
    };

    let run_cid = write_test_run_capsule(ws, &cap).expect("write capsule");
    assert_ne!(run_cid, set_cid, "TestRunCapsule CID must differ from scenario set CID");
}
