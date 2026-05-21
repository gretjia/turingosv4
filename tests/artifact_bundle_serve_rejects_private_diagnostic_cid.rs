//! Verification that GET /api/bundle/:artifact_bundle_cid/file?path=<relative-path>
//! returns 404 when requested CID points to a private diagnostic capsule.
#![cfg(feature = "web")]

#[path = "../src/web/mod.rs"]
mod web;

use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;

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
async fn test_artifact_bundle_serve_rejects_private_diagnostic_cid() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let workspace = dir.path().to_path_buf();

    // Setup CAS store and write a private diagnostic capsule (e.g. AutopsyPrivateDetail) into CAS
    let cas_dir = turingosv4::runtime::spec_capsule::cas_path(&workspace);
    std::fs::create_dir_all(&cas_dir).expect("create cas dir");
    let mut store = turingosv4::bottom_white::cas::store::CasStore::open(&cas_dir).expect("open cas");

    let private_detail = b"{\"private_logs\":\"confidential_information\"}";
    let private_cid = store
        .put(
            private_detail,
            turingosv4::bottom_white::cas::schema::ObjectType::AutopsyPrivateDetail,
            "test_user",
            0,
            None,
        )
        .expect("put private detail in CAS");

    let private_cid_hex = private_cid.hex();
    let workspace_str = workspace.to_string_lossy().into_owned();

    let _guard = env_lock().lock().await;
    std::env::set_var("TURINGOS_WEB_WORKSPACE", &workspace_str);

    let addr = start_server().await;

    // Request this CID, expecting a 404
    let path_uri = format!(
        "/api/bundle/{}/file?path=index.html",
        private_cid_hex
    );
    let (status, resp_body, _) = http_get(addr, &path_uri).await;

    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    drop(_guard);

    assert_eq!(status, 404, "GET for private diagnostic CID must return 404; body={resp_body}");
}
