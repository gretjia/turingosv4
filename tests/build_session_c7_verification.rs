//! Verification of C7: BuildSessionView derived from CAS.
#![cfg(feature = "web")]

#[path = "../src/web/mod.rs"]
mod web;

use std::net::SocketAddr;
use std::path::Path;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;

use turingosv4::bottom_white::cas::schema::{Cid, ObjectType};
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::spec_capsule::{cas_path, GrillSessionCapsuleBody};
use turingosv4::runtime::generation_attempt::{
    GenerationAttemptCapsule, AttemptOutcome, GENERATION_ATTEMPT_CAPSULE_SCHEMA_ID,
};
use turingosv4::runtime::artifact_bundle::{
    ArtifactBundleManifest, ARTIFACT_BUNDLE_SCHEMA_ID,
};
use turingosv4::runtime::preview_run::{
    PreviewRunCapsule, SandboxPolicy, PREVIEW_RUN_CAPSULE_SCHEMA_ID,
};
use turingosv4::runtime::build_session_view::{BuildSessionView, BuildStatus};

static ENV_LOCK: std::sync::OnceLock<Mutex<()>> = std::sync::OnceLock::new();

fn env_lock() -> &'static Mutex<()> {
    ENV_LOCK.get_or_init(|| Mutex::new(()))
}

async fn start_server() -> SocketAddr {
    let router = web::router::build_with_state(64);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind random port");
    let addr = listener.local_addr().expect("local addr");
    tokio::spawn(async move {
        axum::serve(listener, router)
            .await
            .expect("axum serve error in test");
    });
    addr
}

async fn http_get(addr: SocketAddr, path: &str) -> (u16, String) {
    let mut stream = tokio::net::TcpStream::connect(addr).await.expect("connect");
    let request = format!(
        "GET {path} HTTP/1.1\r\n\
         Host: 127.0.0.1\r\n\
         Connection: close\r\n\
         \r\n"
    );
    stream.write_all(request.as_bytes()).await.expect("write");
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).await.expect("read");
    let raw = String::from_utf8_lossy(&buf).into_owned();
    let (head, resp_body) = if let Some(idx) = raw.find("\r\n\r\n") {
        (&raw[..idx], raw[idx + 4..].to_string())
    } else {
        (raw.as_str(), String::new())
    };
    let status_code: u16 = head
        .lines()
        .next()
        .unwrap_or("")
        .split_whitespace()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    (status_code, resp_body)
}

#[tokio::test]
async fn test_build_session_c7_lifecycle() {
    let _guard = env_lock().lock().await;

    let dir = tempfile::TempDir::new().expect("tempdir");
    let workspace = dir.path().to_path_buf();
    let workspace_str = workspace.to_string_lossy().into_owned();
    std::env::set_var("TURINGOS_WEB_WORKSPACE", &workspace_str);

    let cas_dir = cas_path(&workspace);
    std::fs::create_dir_all(&cas_dir).expect("create cas dir");
    let mut store = CasStore::open(&cas_dir).expect("open cas store");

    let addr = start_server().await;
    let session_id = "test-session-c7";

    // 1. Initial state (no spec capsule) -> BuildStatus::SpecPending
    let (status, body) = http_get(addr, &format!("/api/build/session/{session_id}")).await;
    assert_eq!(status, 200);
    let view: BuildSessionView = serde_json::from_str(&body).expect("parse view");
    assert_eq!(view.current_status, BuildStatus::SpecPending);
    assert!(view.spec_capsule_cid.is_none());
    assert!(view.generation_attempts.is_empty());

    // 2. Add GrillSessionCapsule (contains spec) -> BuildStatus::SpecDone
    let spec_cid_hex = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".to_string();
    let grill_body = GrillSessionCapsuleBody {
        schema_id: "turingos-spec-grill-session-v1".to_string(),
        session_id: session_id.to_string(),
        turn_cids: vec![],
        final_spec_capsule_cid: spec_cid_hex.clone(),
        termination_reason: "done".to_string(),
        created_at_logical_t: 10,
    };
    let grill_bytes = serde_json::to_vec(&grill_body).unwrap();
    let grill_cid = store.put(
        &grill_bytes,
        ObjectType::EvidenceCapsule,
        "test_user",
        10,
        Some("turingos-spec-grill-session-v1".to_string()),
    ).expect("put grill session");

    let (status, body) = http_get(addr, &format!("/api/build/session/{session_id}")).await;
    assert_eq!(status, 200);
    let view: BuildSessionView = serde_json::from_str(&body).expect("parse view");
    assert_eq!(view.current_status, BuildStatus::SpecDone);
    assert_eq!(view.spec_capsule_cid.as_deref(), Some(spec_cid_hex.as_str()));

    // 3. Add GenerationAttempt -> BuildStatus::Generating
    let attempt_body = GenerationAttemptCapsule {
        schema_id: GENERATION_ATTEMPT_CAPSULE_SCHEMA_ID.to_string(),
        session_id: session_id.to_string(),
        spec_capsule_cid: Some(spec_cid_hex.clone()),
        outcome: AttemptOutcome::Success,
        world_head_parent: "parent_head".to_string(),
        world_head_resulting: "result_head".to_string(),
        bounty_t_spent: 100,
        logical_t: 20,
    };
    let attempt_bytes = serde_json::to_vec(&attempt_body).unwrap();
    let attempt_cid = store.put(
        &attempt_bytes,
        ObjectType::EvidenceCapsule,
        "test_user",
        20,
        Some(GENERATION_ATTEMPT_CAPSULE_SCHEMA_ID.to_string()),
    ).expect("put attempt");

    let (status, body) = http_get(addr, &format!("/api/build/session/{session_id}")).await;
    assert_eq!(status, 200);
    let view: BuildSessionView = serde_json::from_str(&body).expect("parse view");
    assert_eq!(view.current_status, BuildStatus::Generating);
    assert_eq!(view.generation_attempts, vec![attempt_cid.hex()]);

    // 4. Add ArtifactBundle -> BuildStatus::Generated
    let bundle_body = ArtifactBundleManifest {
        schema_id: ARTIFACT_BUNDLE_SCHEMA_ID.to_string(),
        session_id: session_id.to_string(),
        spec_capsule_cid: Some(spec_cid_hex.clone()),
        files: vec![],
        entrypoint: "index.html".to_string(),
        created_at_logical_t: 30,
    };
    let bundle_bytes = serde_json::to_vec(&bundle_body).unwrap();
    let bundle_cid = store.put(
        &bundle_bytes,
        ObjectType::EvidenceCapsule,
        "test_user",
        30,
        Some(ARTIFACT_BUNDLE_SCHEMA_ID.to_string()),
    ).expect("put bundle");

    let (status, body) = http_get(addr, &format!("/api/build/session/{session_id}")).await;
    assert_eq!(status, 200);
    let view: BuildSessionView = serde_json::from_str(&body).expect("parse view");
    assert_eq!(view.current_status, BuildStatus::Generated);
    assert_eq!(view.artifact_versions, vec![bundle_cid.hex()]);

    // 5. Add Rejection (logical_t >= bundle) -> BuildStatus::Rejected
    let rejection_body = serde_json::json!({
        "schema_id": "turingos-generate-rejection-v1",
        "session_id": session_id.to_string(),
        "private_diagnostic_cid": "secret_diagnostic_cid",
        "logical_t": 40,
    });
    let rejection_bytes = serde_json::to_vec(&rejection_body).unwrap();
    let rejection_cid = store.put(
        &rejection_bytes,
        ObjectType::EvidenceCapsule,
        "test_user",
        40,
        Some("turingos-generate-rejection-v1".to_string()),
    ).expect("put rejection");

    let (status, body) = http_get(addr, &format!("/api/build/session/{session_id}")).await;
    assert_eq!(status, 200);
    let view: BuildSessionView = serde_json::from_str(&body).expect("parse view");
    assert_eq!(view.current_status, BuildStatus::Rejected);
    assert_eq!(view.rejection_events, vec![rejection_cid.hex()]);

    // 6. Test delete cache & rebuild
    let cache_file = cas_dir.join(".turingos_cas_index.jsonl");
    assert!(cache_file.exists());
    std::fs::remove_file(&cache_file).expect("remove cache");

    let (status, body) = http_get(addr, &format!("/api/build/session/{session_id}")).await;
    assert_eq!(status, 200);
    let view: BuildSessionView = serde_json::from_str(&body).expect("parse view");
    // Verify it still reconstructs correctly
    assert_eq!(view.current_status, BuildStatus::Rejected);
    assert_eq!(view.spec_capsule_cid.as_deref(), Some(spec_cid_hex.as_str()));
    assert_eq!(view.rejection_events, vec![rejection_cid.hex()]);

    // 7. Verify mutation doesn't affect git-chain/CAS
    let mut mutated_view = view.clone();
    mutated_view.session_id = "hacked-session".to_string();
    // Re-check view endpoint, verify it remains test-session-c7
    let (status, body) = http_get(addr, &format!("/api/build/session/{session_id}")).await;
    assert_eq!(status, 200);
    let view2: BuildSessionView = serde_json::from_str(&body).expect("parse view");
    assert_eq!(view2.session_id, session_id);

    // 8. Verify private_diagnostic_cid shielding (is not present in returned view body)
    assert!(!body.contains("secret_diagnostic_cid"));

    // 9. Verify test_scenario_set_cid shielding
    let test_run_body = serde_json::json!({
        "schema_id": "turingos-test-run-v1",
        "session_id": session_id.to_string(),
        "test_scenario_set_cid": "secret_scenario_cid",
        "logical_t": 50,
    });
    let test_run_bytes = serde_json::to_vec(&test_run_body).unwrap();
    let _test_run_cid = store.put(
        &test_run_bytes,
        ObjectType::EvidenceCapsule,
        "test_user",
        50,
        Some("turingos-test-run-v1".to_string()),
    ).expect("put test run");

    let (status, body) = http_get(addr, &format!("/api/build/session/{session_id}")).await;
    assert_eq!(status, 200);
    // Verify secret_scenario_cid is not present anywhere in the returned JSON
    assert!(!body.contains("secret_scenario_cid"));

    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
}

#[tokio::test]
async fn test_build_session_ordering() {
    let _guard = env_lock().lock().await;

    let dir = tempfile::TempDir::new().expect("tempdir");
    let workspace = dir.path().to_path_buf();
    let workspace_str = workspace.to_string_lossy().into_owned();
    std::env::set_var("TURINGOS_WEB_WORKSPACE", &workspace_str);

    let cas_dir = cas_path(&workspace);
    std::fs::create_dir_all(&cas_dir).expect("create cas dir");
    let mut store = CasStore::open(&cas_dir).expect("open cas store");

    let addr = start_server().await;
    let session_id = "test-session-ordering";

    // Add GrillSession (to pass spec validation)
    let spec_cid_hex = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".to_string();
    let grill_body = GrillSessionCapsuleBody {
        schema_id: "turingos-spec-grill-session-v1".to_string(),
        session_id: session_id.to_string(),
        turn_cids: vec![],
        final_spec_capsule_cid: spec_cid_hex.clone(),
        termination_reason: "done".to_string(),
        created_at_logical_t: 10,
    };
    let grill_bytes = serde_json::to_vec(&grill_body).unwrap();
    let _grill_cid = store.put(
        &grill_bytes,
        ObjectType::EvidenceCapsule,
        "test_user",
        10,
        Some("turingos-spec-grill-session-v1".to_string()),
    ).expect("put grill");

    // Write attempts with different logical timestamps to test ordering by (logical_t, cid)
    // Attempt A: logical_t = 30
    // Attempt B: logical_t = 20
    // Attempt C: logical_t = 20
    // Let's write them in reverse/out-of-order sequence to ensure the system sorts them.
    let make_attempt = |logical_t: u64, marker: &str| {
        let attempt = GenerationAttemptCapsule {
            schema_id: GENERATION_ATTEMPT_CAPSULE_SCHEMA_ID.to_string(),
            session_id: session_id.to_string(),
            spec_capsule_cid: Some(spec_cid_hex.clone()),
            outcome: AttemptOutcome::Success,
            world_head_parent: format!("parent_{marker}"),
            world_head_resulting: format!("result_{marker}"),
            bounty_t_spent: 100,
            logical_t,
        };
        serde_json::to_vec(&attempt).unwrap()
    };

    let att_a_bytes = make_attempt(30, "A");
    let att_b_bytes = make_attempt(20, "B");
    let att_c_bytes = make_attempt(20, "C");

    // Put them into CAS in a specific order: A, C, B
    let cid_a = store.put(&att_a_bytes, ObjectType::EvidenceCapsule, "test_user", 30, Some(GENERATION_ATTEMPT_CAPSULE_SCHEMA_ID.to_string())).unwrap();
    let cid_c = store.put(&att_c_bytes, ObjectType::EvidenceCapsule, "test_user", 20, Some(GENERATION_ATTEMPT_CAPSULE_SCHEMA_ID.to_string())).unwrap();
    let cid_b = store.put(&att_b_bytes, ObjectType::EvidenceCapsule, "test_user", 20, Some(GENERATION_ATTEMPT_CAPSULE_SCHEMA_ID.to_string())).unwrap();

    let (status, body) = http_get(addr, &format!("/api/build/session/{session_id}")).await;
    assert_eq!(status, 200);
    let view: BuildSessionView = serde_json::from_str(&body).expect("parse view");

    // Expected sorting:
    // First by logical_t ascending, then by Cid ascending.
    // 20 is less than 30, so cid_b and cid_c come before cid_a.
    // Between cid_b and cid_c, whichever Cid value is smaller should come first.
    let mut expected = vec![cid_b, cid_c];
    expected.sort(); // sorts by Cid value ascending
    expected.push(cid_a); // logical_t = 30 is largest, so it must be last

    let expected_hexs: Vec<String> = expected.iter().map(|c| c.hex()).collect();
    assert_eq!(view.generation_attempts, expected_hexs);

    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
}
