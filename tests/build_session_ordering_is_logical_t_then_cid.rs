//! Spec-named test file split from build_session_c7_verification.rs on 2026-05-21
//! per master plan §C7 acceptance command requirement.
#![cfg(feature = "web")]

#[path = "../src/web/mod.rs"]
mod web;

use std::net::SocketAddr;
use std::path::Path;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;

use turingosv4::bottom_white::cas::schema::{Cid, ObjectType};
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::spec_capsule::{cas_path, GrillSessionCapsuleBody, GrillAttemptTally};
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
async fn build_session_ordering_is_logical_t_then_cid() {
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
        session_id: session_id.to_string(),
        turn_cids: vec![],
        final_spec_capsule_cid: spec_cid_hex.clone(),
        termination_reason: "done".to_string(),
        total_turns: 0,
        partial_session: false,
        lang: "en".to_string(),
        grill_attempt_tally: GrillAttemptTally::default(),
        logical_t: 10,
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
            spec_source: "ondisk_spec_md".to_string(),
            model_id: format!("test_{marker}"),
            model_seed: None,
            prompt_hash: format!("hash_{marker}"),
            raw_output_cid: None,
            usage_total_tokens: None,
            retry_index: 0,
            parent_attempt_cid: None,
            outcome: AttemptOutcome::Success,
            parsed_file_count: 1,
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
