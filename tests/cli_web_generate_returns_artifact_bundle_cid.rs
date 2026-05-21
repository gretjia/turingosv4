//! Verification that POST /api/generate response includes artifact_bundle_cid
//! and maps individual artifact file CIDs and SHA-256 hashes from CAS.
#![cfg(feature = "web")]

#[path = "../src/web/mod.rs"]
mod web;

use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use turingosv4::runtime::artifact_bundle::{
    write_artifact_bundle, ArtifactBundleManifest, ArtifactFileEntry, ArtifactFileRole,
};

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

fn write_stub_script(dir: &tempfile::TempDir, exit_code: i32) -> String {
    let script_path = dir.path().join("turingos");
    let script_content = format!(
        "#!/bin/sh\nexit {exit_code}\n",
    );
    std::fs::write(&script_path, script_content).expect("write stub");
    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(&script_path).unwrap().permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(&script_path, perms).unwrap();
    script_path.to_string_lossy().into_owned()
}

fn setup_session(workspace: &std::path::Path, session_id: &str) -> std::path::PathBuf {
    let session_dir = workspace.join("sessions").join(session_id);
    std::fs::create_dir_all(&session_dir).expect("create session dir");
    session_dir
}

fn write_spec_md(session_dir: &std::path::Path) {
    let spec_content =
        "# Stub Spec\n\n## One-line Goal\n\nTest spec.\n\n<!-- TURINGOS_SPEC_END -->\n";
    std::fs::write(session_dir.join("spec.md"), spec_content).expect("write spec.md");
}

fn write_stub_artifacts(session_dir: &std::path::Path) {
    let artifacts_dir = session_dir.join("artifacts");
    std::fs::create_dir_all(&artifacts_dir).expect("create artifacts dir");
    let mut html = String::new();
    html.push_str("<!DOCTYPE html><html><head><title>Hello</title></head><body>");
    html.push_str("<canvas id=\"c\" width=\"200\" height=\"400\"></canvas>");
    html.push_str("<script>\n");
    html.push_str("let ctx = document.getElementById('c').getContext('2d');\n");
    html.push_str("let player = { x: 0 };\n");
    html.push_str(
        "function tick() { ctx.fillRect(player.x, 0, 10, 10); requestAnimationFrame(tick); }\n",
    );
    html.push_str("document.addEventListener('keydown', function(e) {\n");
    html.push_str("  if (e.code === 'ArrowLeft') { player.x = player.x - 1; }\n");
    html.push_str("  if (e.code === 'ArrowRight') { player.x = player.x + 1; }\n");
    html.push_str("});\n");
    for _ in 0..40 {
        html.push_str(
            "// padding comment line to clear minimum size heuristic threshold ok ok ok ok\n",
        );
    }
    html.push_str("tick();\n");
    html.push_str("</script></body></html>\n");
    std::fs::write(artifacts_dir.join("index.html"), &html).expect("write index.html");
}

#[tokio::test]
async fn test_generate_returns_artifact_bundle_cid() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let workspace = dir.path().to_path_buf();
    let session_id = "test-session-bundle-cid";
    let session_dir = setup_session(&workspace, session_id);
    write_spec_md(&session_dir);
    write_stub_artifacts(&session_dir);

    // Pre-populate CAS with an artifact bundle manifest for this session
    let expected_file_cid = "34".repeat(32);
    let expected_sha256 = "56".repeat(32);

    let manifest = ArtifactBundleManifest {
        schema_id: "turingos-artifact-bundle-v1".to_string(),
        session_id: session_id.to_string(),
        spec_capsule_cid: Some("aa".repeat(32)),
        generation_attempt_cid: "bb".repeat(32),
        previous_bundle_cid: None,
        files: vec![
            ArtifactFileEntry {
                path: "index.html".to_string(),
                cid: expected_file_cid.clone(),
                mime: "text/html".to_string(),
                sha256: expected_sha256.clone(),
                size_bytes: 120,
                role: ArtifactFileRole::Entrypoint,
            }
        ],
        entrypoint: "index.html".to_string(),
        bundle_size_bytes_total: 120,
        created_at_logical_t: 100,
    };

    let actual_written_cid = write_artifact_bundle(&workspace, &manifest).expect("write manifest");

    let script_path = write_stub_script(&dir, 0);
    let workspace_str = workspace.to_string_lossy().into_owned();

    let _guard = env_lock().lock().await;
    std::env::set_var("TURINGOS_BACKEND_OVERRIDE", &script_path);
    std::env::set_var("TURINGOS_WEB_WORKSPACE", &workspace_str);

    let addr = start_server().await;
    let body = format!(r#"{{"session_id":"{session_id}","from_capsule":false}}"#);
    let (status, resp_body) = http_post_json(addr, "/api/generate", &body).await;

    std::env::remove_var("TURINGOS_BACKEND_OVERRIDE");
    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    drop(_guard);

    assert_eq!(status, 200, "POST must return 200; body={resp_body}");

    let parsed: serde_json::Value = serde_json::from_str(&resp_body).expect("valid JSON");
    
    // Assert GenerateResponse level fields
    let resp_bundle_cid = parsed["artifact_bundle_cid"]
        .as_str()
        .expect("must contain artifact_bundle_cid");
    assert_eq!(resp_bundle_cid, actual_written_cid.as_str());

    let resp_status = parsed["status"]
        .as_str()
        .expect("must contain status");
    assert_eq!(resp_status, "success");

    // Assert ArtifactEntry level fields
    let artifacts = parsed["artifacts"]
        .as_array()
        .expect("must contain artifacts array");
    
    let html_entry = artifacts
        .iter()
        .find(|e| e["path"].as_str() == Some("index.html"))
        .expect("must contain index.html");

    assert_eq!(html_entry["cid"].as_str(), Some(expected_file_cid.as_str()));
    assert_eq!(html_entry["sha256"].as_str(), Some(expected_sha256.as_str()));
}
