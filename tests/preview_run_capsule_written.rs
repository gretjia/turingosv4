//! Verification that a PreviewRunCapsule is correctly written to the CAS store
//! on each preview request.
#![cfg(feature = "web")]

#[path = "../src/web/mod.rs"]
mod web;

use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use turingosv4::runtime::artifact_bundle::{
    write_artifact_bundle, ArtifactBundleManifest, ArtifactFileEntry, ArtifactFileRole,
};
use turingosv4::runtime::preview_run::{PreviewRunCapsule, SandboxPolicy, PREVIEW_RUN_CAPSULE_SCHEMA_ID};

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
async fn test_preview_run_capsule_written() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let workspace = dir.path().to_path_buf();
    let session_id = "test-session-preview-capsule";

    // Setup CAS store and write file bytes into CAS
    let cas_dir = turingosv4::runtime::spec_capsule::cas_path(&workspace);
    std::fs::create_dir_all(&cas_dir).expect("create cas dir");
    let mut store = turingosv4::bottom_white::cas::store::CasStore::open(&cas_dir).expect("open cas");

    let file_content = b"<html><body>Hello from CAS!</body></html>";
    let file_cid = store
        .put(
            file_content,
            turingosv4::bottom_white::cas::schema::ObjectType::ProposalPayload,
            "test_user",
            0,
            None,
        )
        .expect("put file in CAS");

    let expected_file_cid_hex = file_cid.hex();
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
                cid: expected_file_cid_hex.clone(),
                mime: "text/html".to_string(),
                sha256: expected_sha256.clone(),
                size_bytes: file_content.len() as u64,
                role: ArtifactFileRole::Entrypoint,
            }
        ],
        entrypoint: "index.html".to_string(),
        bundle_size_bytes_total: file_content.len() as u64,
        created_at_logical_t: 100,
    };

    let actual_written_bundle_cid_hex = write_artifact_bundle(&workspace, &manifest).expect("write manifest");

    let workspace_str = workspace.to_string_lossy().into_owned();

    let _guard = env_lock().lock().await;
    std::env::set_var("TURINGOS_WEB_WORKSPACE", &workspace_str);

    let addr = start_server().await;

    // 1. Trigger successful preview request
    let path_uri = format!(
        "/api/preview/{}/file?path=index.html&session_id={}&sandbox_policy=allowscriptsallowsameorigin",
        actual_written_bundle_cid_hex, session_id
    );
    let (status, _) = http_get(addr, &path_uri).await;
    assert_eq!(status, 200);

    // Let's inspect the CAS store to find the PreviewRunCapsule
    let mut store2 = turingosv4::bottom_white::cas::store::CasStore::open(&cas_dir).expect("open cas");
    let mut found_success_capsule = false;
    let mut found_fail_capsule = false;

    // Iterate through all objects in the CAS store
    for entry in store2.list().expect("list CAS") {
        if entry.schema_id.as_deref() == Some(PREVIEW_RUN_CAPSULE_SCHEMA_ID) {
            let bytes = store2.get(&entry.cid).expect("get capsule bytes");
            let capsule: PreviewRunCapsule = serde_json::from_slice(&bytes).expect("parse capsule");
            
            if capsule.serve_success {
                assert_eq!(capsule.artifact_bundle_cid, actual_written_bundle_cid_hex);
                assert_eq!(capsule.session_id, session_id);
                assert_eq!(capsule.entrypoint_path, "index.html");
                assert_eq!(capsule.sandbox_policy, SandboxPolicy::AllowScriptsAllowSameOrigin);
                assert!(capsule.logical_t > 0);
                found_success_capsule = true;
            }
        }
    }
    assert!(found_success_capsule, "expected to find a serve_success=true preview capsule in CAS");

    // 2. Trigger unsuccessful preview request
    let path_uri_fail = format!(
        "/api/preview/{}/file?path=nonexistent.html&session_id={}&sandbox_policy=allowscripts",
        actual_written_bundle_cid_hex, session_id
    );
    let (status_fail, _) = http_get(addr, &path_uri_fail).await;
    assert_eq!(status_fail, 404);

    // Re-list and verify fail capsule is written
    let mut store3 = turingosv4::bottom_white::cas::store::CasStore::open(&cas_dir).expect("open cas");
    for entry in store3.list().expect("list CAS") {
        if entry.schema_id.as_deref() == Some(PREVIEW_RUN_CAPSULE_SCHEMA_ID) {
            let bytes = store3.get(&entry.cid).expect("get capsule bytes");
            let capsule: PreviewRunCapsule = serde_json::from_slice(&bytes).expect("parse capsule");
            
            if !capsule.serve_success {
                assert_eq!(capsule.artifact_bundle_cid, actual_written_bundle_cid_hex);
                assert_eq!(capsule.session_id, session_id);
                assert_eq!(capsule.entrypoint_path, "nonexistent.html");
                assert_eq!(capsule.sandbox_policy, SandboxPolicy::AllowScripts);
                assert!(capsule.logical_t > 0);
                found_fail_capsule = true;
            }
        }
    }
    assert!(found_fail_capsule, "expected to find a serve_success=false preview capsule in CAS");

    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    drop(_guard);
}
