//! Verification that GET /api/bundle/:artifact_bundle_cid/file?path=<relative-path>
//! rejects paths containing traversal (..) or starting with a slash (/).
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

async fn http_get(addr: SocketAddr, path: &str) -> (u16, String, Option<String>) {
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
    
    let mut content_type = None;
    for line in head.lines() {
        if line.to_ascii_lowercase().starts_with("content-type:") {
            if let Some(val) = line.split(':').nth(1) {
                content_type = Some(val.trim().to_string());
            }
        }
    }
    (status_code, resp_body, content_type)
}

#[tokio::test]
async fn test_artifact_bundle_file_serve_rejects_traversal() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let workspace = dir.path().to_path_buf();
    let session_id = "test-session-traversal";

    // Setup CAS store and write manifest into CAS
    let cas_dir = turingosv4::runtime::spec_capsule::cas_path(&workspace);
    std::fs::create_dir_all(&cas_dir).expect("create cas dir");

    let manifest = ArtifactBundleManifest {
        schema_id: "turingos-artifact-bundle-v1".to_string(),
        session_id: session_id.to_string(),
        spec_capsule_cid: Some("aa".repeat(32)),
        generation_attempt_cid: "bb".repeat(32),
        previous_bundle_cid: None,
        files: vec![
            ArtifactFileEntry {
                path: "index.html".to_string(),
                cid: "34".repeat(32),
                mime: "text/html".to_string(),
                sha256: "56".repeat(32),
                size_bytes: 120,
                role: ArtifactFileRole::Entrypoint,
            }
        ],
        entrypoint: "index.html".to_string(),
        bundle_size_bytes_total: 120,
        created_at_logical_t: 100,
    };

    let actual_written_bundle_cid_hex = write_artifact_bundle(&workspace, &manifest).expect("write manifest");

    let workspace_str = workspace.to_string_lossy().into_owned();

    let _guard = env_lock().lock().await;
    std::env::set_var("TURINGOS_WEB_WORKSPACE", &workspace_str);

    let addr = start_server().await;

    // Test cases for traversal and invalid paths
    let bad_paths = vec![
        "../index.html",
        "index.html/../etc",
        "/etc/passwd",
        "",
    ];

    for bad_path in bad_paths {
        let path_uri = format!(
            "/api/bundle/{}/file?path={}",
            actual_written_bundle_cid_hex,
            bad_path
        );
        let (status, resp_body, _) = http_get(addr, &path_uri).await;
        assert_eq!(
            status,
            400,
            "GET for path {:?} must return 400; body={resp_body}",
            bad_path
        );
    }

    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    drop(_guard);
}
