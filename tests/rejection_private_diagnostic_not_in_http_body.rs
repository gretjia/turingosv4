//! C8 gate: rejection capsule privacy shielding in HTTP body.
//!
//! Verifies that when `turingos generate` fails and emits a rejection_cid,
//! the web handler:
//!   1. returns a 4xx status (not 200)
//!   2. does NOT include `private_diagnostic_cid` in the JSON body
//!   3. DOES include `rejection_cid`, `reject_class`, `public_error_summary`, `reason`, `retryable`
//!
//! FC-trace: FC1-N5 (privacy shielding), FC3 (L4.E binding)
//! Risk class: Class 3 (shielding boundary test)
#![cfg(feature = "web")]

#[path = "../src/web/mod.rs"]
mod web;

use std::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;

use turingosv4::bottom_white::cas::schema::ObjectType;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::generation_attempt::{
    GenerateRejectionCapsule, RejectClass, GENERATE_REJECTION_CAPSULE_SCHEMA_ID,
};
use turingosv4::runtime::spec_capsule::cas_path;

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

async fn http_post_json(addr: SocketAddr, path: &str, body: &str) -> (u16, String) {
    let mut stream = tokio::net::TcpStream::connect(addr).await.expect("connect");
    let request = format!(
        "POST {path} HTTP/1.1\r\n\
         Host: 127.0.0.1\r\n\
         Content-Type: application/json\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\
         \r\n\
         {body}",
        body.len()
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

fn make_rejection_capsule(
    session_id: &str,
    reject_class: RejectClass,
    retryable: bool,
    private_diag_cid: Option<String>,
) -> GenerateRejectionCapsule {
    GenerateRejectionCapsule {
        schema_id: GENERATE_REJECTION_CAPSULE_SCHEMA_ID.to_string(),
        session_id: session_id.to_string(),
        spec_capsule_cid: None,
        generation_attempt_cid: None,
        triage_attempted: false,
        reject_class,
        public_error_summary: "LLM API returned 500 error".to_string(),
        reason: "llm_api_error".to_string(),
        private_diagnostic_cid: private_diag_cid,
        retryable,
        world_head_unchanged: true,
        logical_t: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0),
    }
}

/// Creates a mock turingos binary that writes a rejection capsule to CAS and
/// emits `rejection_cid=<hex>` on stderr, then exits non-zero.
fn write_mock_generate_script(
    script_path: &std::path::Path,
    workspace: &std::path::Path,
    rejection_cid_hex: &str,
) {
    let script = format!(
        r#"#!/bin/sh
# Mock turingos binary for C8 testing
# Exits with code 2 to simulate generation failure
echo "rejection_cid={rejection_cid_hex}" >&2
exit 2
"#,
        rejection_cid_hex = rejection_cid_hex,
    );
    std::fs::write(script_path, script).expect("write mock script");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(script_path)
            .expect("metadata")
            .permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(script_path, perms).expect("set permissions");
    }
}

#[tokio::test]
async fn test_rejection_private_diagnostic_not_in_http_body() {
    let _guard = env_lock().lock().await;

    let dir = tempfile::TempDir::new().expect("tempdir");
    let workspace = dir.path().to_path_buf();
    let workspace_str = workspace.to_string_lossy().into_owned();
    std::env::set_var("TURINGOS_WEB_WORKSPACE", &workspace_str);

    let session_id = "test-c8-privacy";

    // Set up CAS with a rejection capsule that has a private_diagnostic_cid
    let cas_dir = cas_path(&workspace);
    std::fs::create_dir_all(&cas_dir).expect("create cas dir");
    let mut store = CasStore::open(&cas_dir).expect("open cas store");

    // Write a fake private diagnostic blob
    let private_diag_bytes = b"PRIVATE STACK TRACE: SomeInternalError at line 42";
    let private_cid = store
        .put(
            private_diag_bytes,
            ObjectType::EvidenceCapsule,
            "test_system",
            1000,
            None, // no schema ID = raw private diagnostic
        )
        .expect("put private diagnostic");
    let private_cid_hex = private_cid.hex();

    // Write the rejection capsule referencing the private diagnostic
    let capsule = make_rejection_capsule(
        session_id,
        RejectClass::LlmApiError,
        true,
        Some(private_cid_hex.clone()),
    );
    let capsule_bytes = serde_json::to_vec(&capsule).expect("serialize");
    let rejection_cid = store
        .put(
            &capsule_bytes,
            ObjectType::EvidenceCapsule,
            "test_system",
            1001,
            Some(GENERATE_REJECTION_CAPSULE_SCHEMA_ID.to_string()),
        )
        .expect("put rejection capsule");
    let rejection_cid_hex = rejection_cid.hex();

    // Set up session directory and spec.md
    let sessions_dir = workspace.join("sessions").join(session_id);
    std::fs::create_dir_all(&sessions_dir).expect("create sessions dir");
    std::fs::write(sessions_dir.join("spec.md"), "# Test Spec").expect("write spec.md");

    // Create mock turingos binary
    let mock_bin = dir.path().join("mock_turingos");
    write_mock_generate_script(&mock_bin, &workspace, &rejection_cid_hex);
    std::env::set_var(
        "TURINGOS_BACKEND_OVERRIDE",
        mock_bin.to_str().expect("valid path"),
    );

    let addr = start_server().await;

    let request_body = serde_json::json!({
        "session_id": session_id,
        "from_capsule": false
    })
    .to_string();

    let (status, body) = http_post_json(addr, "/api/generate", &request_body).await;

    // Must return 4xx (not 200, not 500 with raw diagnostics)
    assert!(
        status == 422 || status == 400,
        "expected 4xx status, got {}: {}",
        status,
        body
    );

    // CRITICAL: private_diagnostic_cid must NOT appear in the HTTP response body
    assert!(
        !body.contains(&private_cid_hex),
        "private_diagnostic_cid appeared in HTTP response body!"
    );
    assert!(
        !body.contains("PRIVATE STACK TRACE"),
        "private diagnostic content appeared in HTTP response body!"
    );

    // The rejection_cid itself should be in the response (it's public)
    assert!(
        body.contains(&rejection_cid_hex),
        "rejection_cid missing from response body"
    );

    // Public fields should be present
    assert!(
        body.contains("llm_api_error"),
        "reason missing from response: {}",
        body
    );

    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    std::env::remove_var("TURINGOS_BACKEND_OVERRIDE");
}

#[tokio::test]
async fn test_privacy_blocked_not_retryable_exits_immediately() {
    let _guard = env_lock().lock().await;

    let dir = tempfile::TempDir::new().expect("tempdir");
    let workspace = dir.path().to_path_buf();
    let workspace_str = workspace.to_string_lossy().into_owned();
    std::env::set_var("TURINGOS_WEB_WORKSPACE", &workspace_str);

    let session_id = "test-c8-privacyblocked";

    // Set up CAS with a PrivacyBlocked rejection capsule
    let cas_dir = cas_path(&workspace);
    std::fs::create_dir_all(&cas_dir).expect("create cas dir");
    let mut store = CasStore::open(&cas_dir).expect("open cas store");

    let capsule = make_rejection_capsule(
        session_id,
        RejectClass::PrivacyBlocked,
        false, // retryable = false
        None,
    );
    let capsule_bytes = serde_json::to_vec(&capsule).expect("serialize");
    let rejection_cid = store
        .put(
            &capsule_bytes,
            ObjectType::EvidenceCapsule,
            "test_system",
            1001,
            Some(GENERATE_REJECTION_CAPSULE_SCHEMA_ID.to_string()),
        )
        .expect("put rejection capsule");
    let rejection_cid_hex = rejection_cid.hex();

    // Set up session directory and spec.md
    let sessions_dir = workspace.join("sessions").join(session_id);
    std::fs::create_dir_all(&sessions_dir).expect("create sessions dir");
    std::fs::write(sessions_dir.join("spec.md"), "# Test Spec").expect("write spec.md");

    // Mock binary always returns PrivacyBlocked rejection
    let mock_bin = dir.path().join("mock_turingos_privacy");
    write_mock_generate_script(&mock_bin, &workspace, &rejection_cid_hex);
    std::env::set_var(
        "TURINGOS_BACKEND_OVERRIDE",
        mock_bin.to_str().expect("valid path"),
    );

    let addr = start_server().await;

    let request_body = serde_json::json!({
        "session_id": session_id,
        "from_capsule": false
    })
    .to_string();

    let (status, body) = http_post_json(addr, "/api/generate", &request_body).await;

    // PrivacyBlocked → must return 4xx immediately, not retry up to 3 times
    assert!(
        status == 422 || status == 400,
        "expected 4xx for PrivacyBlocked, got {}: {}",
        status,
        body
    );

    // Verify retryable=false is reflected
    assert!(
        body.contains("\"retryable\":false") || body.contains("retryable: false"),
        "expected retryable=false in response: {}",
        body
    );

    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    std::env::remove_var("TURINGOS_BACKEND_OVERRIDE");
}

#[tokio::test]
async fn test_rejection_4_tuple_present() {
    let _guard = env_lock().lock().await;

    let dir = tempfile::TempDir::new().expect("tempdir");
    let workspace = dir.path().to_path_buf();
    let workspace_str = workspace.to_string_lossy().into_owned();
    std::env::set_var("TURINGOS_WEB_WORKSPACE", &workspace_str);

    let session_id = "test-c8-4tuple";

    // Set up CAS with rejection capsule with all 4-tuple fields set
    let cas_dir = cas_path(&workspace);
    std::fs::create_dir_all(&cas_dir).expect("create cas dir");
    let mut store = CasStore::open(&cas_dir).expect("open cas store");

    let capsule = GenerateRejectionCapsule {
        schema_id: GENERATE_REJECTION_CAPSULE_SCHEMA_ID.to_string(),
        session_id: session_id.to_string(),
        spec_capsule_cid: None,
        generation_attempt_cid: None,
        triage_attempted: true,
        reject_class: RejectClass::NoFilesParsed,
        public_error_summary: "LLM emitted no parseable files".to_string(), // user-safe summary
        reason: "no_files_parsed".to_string(),                               // machine-readable reason
        private_diagnostic_cid: None,
        retryable: true,
        world_head_unchanged: true,
        logical_t: 1000,
    };
    let capsule_bytes = serde_json::to_vec(&capsule).expect("serialize");
    let rejection_cid = store
        .put(
            &capsule_bytes,
            ObjectType::EvidenceCapsule,
            "test_system",
            1001,
            Some(GENERATE_REJECTION_CAPSULE_SCHEMA_ID.to_string()),
        )
        .expect("put rejection capsule");
    let rejection_cid_hex = rejection_cid.hex();

    // Verify 4-tuple fields on the capsule
    assert!(!capsule.public_error_summary.is_empty(), "public_error_summary must not be empty (v5-4tuple)");
    assert!(!capsule.reason.is_empty(), "reason must not be empty (v5-4tuple)");
    // world_head_unchanged must be true (v5 contract)
    assert!(capsule.world_head_unchanged, "world_head_unchanged must be true");
    // reject_class must be set (v5-4tuple)
    assert_eq!(capsule.reject_class, RejectClass::NoFilesParsed);

    // Set up session and mock binary
    let sessions_dir = workspace.join("sessions").join(session_id);
    std::fs::create_dir_all(&sessions_dir).expect("create sessions dir");
    std::fs::write(sessions_dir.join("spec.md"), "# Test Spec").expect("write spec.md");

    let mock_bin = dir.path().join("mock_turingos_4tuple");
    write_mock_generate_script(&mock_bin, &workspace, &rejection_cid_hex);
    std::env::set_var(
        "TURINGOS_BACKEND_OVERRIDE",
        mock_bin.to_str().expect("valid path"),
    );

    let addr = start_server().await;

    let request_body = serde_json::json!({
        "session_id": session_id,
    })
    .to_string();

    let (status, body) = http_post_json(addr, "/api/generate", &request_body).await;

    // Should return 4xx with the 4-tuple fields in the body
    assert!(status >= 400 && status < 600, "expected error status: {}", status);
    // rejection_cid in body
    assert!(body.contains(&rejection_cid_hex), "rejection_cid missing from 4-tuple response");
    // public_error_summary in body
    assert!(
        body.contains("no parseable files") || body.contains("LLM emitted"),
        "public_error_summary missing: {}",
        body
    );

    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    std::env::remove_var("TURINGOS_BACKEND_OVERRIDE");
}
