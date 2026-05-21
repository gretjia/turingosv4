//! C11 gate: accepted_delivery requires a passing TestRunCapsule.
//!
//! Verifies that derive_build_session_view returns accepted_delivery=true
//! only when the latest TestRunCapsule for the session's bundle has overall_pass=true.
//!
//! FC-trace: FC1 (test loop), FC3 (test evidence)
//! Risk class: Class 3

use std::time::{SystemTime, UNIX_EPOCH};
use turingosv4::runtime::build_session_view::{derive_build_session_view, BuildStatus};
use turingosv4::runtime::artifact_bundle::{ArtifactBundleManifest, ArtifactFileEntry, ArtifactFileRole, ARTIFACT_BUNDLE_SCHEMA_ID};
use turingosv4::runtime::generation_attempt::{GenerationAttemptCapsule, GENERATION_ATTEMPT_CAPSULE_SCHEMA_ID, AttemptOutcome};
use turingosv4::runtime::spec_capsule::{SPEC_CAPSULE_SCHEMA_ID, cas_path};
use turingosv4::runtime::test_run::{TestRunCapsule, TestScenarioResult, TEST_RUN_CAPSULE_SCHEMA_ID, write_test_run_capsule};
use turingosv4::runtime::test_scenario::{derive_scenario_set_from_spec, TestScenario};
use turingosv4::runtime::test_run::write_scenario_set;
use turingosv4::bottom_white::cas::schema::ObjectType;
use turingosv4::bottom_white::cas::store::CasStore;

fn now_t() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(1000)
}

fn setup_workspace_with_bundle(ws: &std::path::Path, session_id: &str, t: u64) -> (String, String) {
    let cas_dir = cas_path(ws);
    std::fs::create_dir_all(&cas_dir).expect("create cas");
    let mut store = CasStore::open(&cas_dir).expect("open cas");

    // Write a spec capsule (raw JSON — SpecCapsule struct is not pub)
    let spec_bytes = serde_json::to_vec(&serde_json::json!({
        "schema_id": SPEC_CAPSULE_SCHEMA_ID,
        "session_id": session_id,
        "spec_body": "## My spec\n\nBuild a todo list.",
        "source_questions": [],
        "logical_t": t,
    })).expect("serialize spec");
    let spec_cid = store.put(&spec_bytes, ObjectType::EvidenceCapsule, "test", t, Some(SPEC_CAPSULE_SCHEMA_ID.to_string())).expect("put spec");

    // Write a GenerationAttemptCapsule
    let attempt = GenerationAttemptCapsule {
        schema_id: GENERATION_ATTEMPT_CAPSULE_SCHEMA_ID.to_string(),
        session_id: session_id.to_string(),
        spec_capsule_cid: Some(spec_cid.hex()),
        spec_source: "ondisk".to_string(),
        model_id: "model".to_string(),
        model_seed: None,
        prompt_hash: "aabbccdd".to_string(),
        raw_output_cid: None,
        usage_total_tokens: None,
        retry_index: 0,
        parent_attempt_cid: None,
        outcome: AttemptOutcome::Success,
        parsed_file_count: 1,
        logical_t: t + 1,
    };
    let attempt_bytes = serde_json::to_vec(&attempt).expect("serialize attempt");
    let attempt_cid = store.put(&attempt_bytes, ObjectType::EvidenceCapsule, "test", t + 1, Some(GENERATION_ATTEMPT_CAPSULE_SCHEMA_ID.to_string())).expect("put attempt");

    // Write file content
    let html = b"<!DOCTYPE html><html><body>hello</body></html>";
    let file_cid = store.put(html, ObjectType::EvidenceCapsule, "test", t + 1, None).expect("put file");

    // Write ArtifactBundleManifest
    let bundle = ArtifactBundleManifest {
        schema_id: ARTIFACT_BUNDLE_SCHEMA_ID.to_string(),
        session_id: session_id.to_string(),
        spec_capsule_cid: Some(spec_cid.hex()),
        generation_attempt_cid: attempt_cid.hex(),
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
        created_at_logical_t: t + 2,
    };
    let bundle_bytes = serde_json::to_vec(&bundle).expect("serialize bundle");
    let bundle_cid = store.put(&bundle_bytes, ObjectType::EvidenceCapsule, "test", t + 2, Some(ARTIFACT_BUNDLE_SCHEMA_ID.to_string())).expect("put bundle");

    (spec_cid.hex(), bundle_cid.hex())
}

#[test]
fn test_accepted_delivery_true_when_test_run_passes() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let ws = dir.path();
    let session = "accepted-pass-session";
    let t = now_t();

    let (_spec_cid, bundle_cid) = setup_workspace_with_bundle(ws, session, t);

    // Write a passing TestRunCapsule
    let scenario_set = derive_scenario_set_from_spec(b"Build a todo list", "spec-cid", t);
    let set_cid = write_scenario_set(ws, &scenario_set).expect("write set");

    let cap = TestRunCapsule {
        schema_id: TEST_RUN_CAPSULE_SCHEMA_ID.to_string(),
        artifact_bundle_cid: bundle_cid.clone(),
        test_scenario_set_cid: set_cid,
        results: vec![TestScenarioResult {
            scenario: TestScenario::EntrypointExists,
            pass: true,
            detail: "ok".to_string(),
        }],
        overall_pass: true,
        logical_t: t + 3,
    };
    write_test_run_capsule(ws, &cap).expect("write capsule");

    let view = derive_build_session_view(ws, session).expect("derive view");
    assert!(view.accepted_delivery, "accepted_delivery must be true");
    assert_eq!(view.current_status, BuildStatus::Accepted);
}

#[test]
fn test_accepted_delivery_false_without_test_run() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let ws = dir.path();
    let session = "no-test-run-session";
    let t = now_t();

    setup_workspace_with_bundle(ws, session, t);

    // No TestRunCapsule written
    let view = derive_build_session_view(ws, session).expect("derive view");
    assert!(!view.accepted_delivery, "accepted_delivery must be false without TestRunCapsule");
    assert_ne!(view.current_status, BuildStatus::Accepted);
}

#[test]
fn test_accepted_delivery_false_when_test_run_fails() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let ws = dir.path();
    let session = "failing-test-run-session";
    let t = now_t();

    let (_spec_cid, bundle_cid) = setup_workspace_with_bundle(ws, session, t);

    // Write a FAILING TestRunCapsule
    let scenario_set = derive_scenario_set_from_spec(b"Build a todo list", "spec-cid-3", t);
    let set_cid = write_scenario_set(ws, &scenario_set).expect("write set");

    let cap = TestRunCapsule {
        schema_id: TEST_RUN_CAPSULE_SCHEMA_ID.to_string(),
        artifact_bundle_cid: bundle_cid.clone(),
        test_scenario_set_cid: set_cid,
        results: vec![TestScenarioResult {
            scenario: TestScenario::HtmlParses,
            pass: false,
            detail: "no DOCTYPE".to_string(),
        }],
        overall_pass: false,
        logical_t: t + 3,
    };
    write_test_run_capsule(ws, &cap).expect("write capsule");

    let view = derive_build_session_view(ws, session).expect("derive view");
    assert!(!view.accepted_delivery, "accepted_delivery must be false when test run fails");
    assert_ne!(view.current_status, BuildStatus::Accepted);
}
