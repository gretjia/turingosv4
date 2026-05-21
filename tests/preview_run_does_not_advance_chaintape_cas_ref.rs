//! Verification that preview run does not advance the state refs
//! and only advances the CAS commit chain (refs/chaintape/cas) by exactly one commit.
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
async fn test_preview_run_does_not_advance_chaintape_cas_ref() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let workspace = dir.path().to_path_buf();
    let session_id = "test-session-preview-ref";

    // Initialize the git repo at workspace
    let repo = git2::Repository::init(&workspace).expect("git init");

    // Configure basic git author for the repository
    let mut config = repo.config().expect("git config");
    config.set_str("user.name", "test").expect("user.name");
    config.set_str("user.email", "test@test.com").expect("user.email");

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

    // Check git ref OIDs before preview
    let cas_ref_name = "refs/chaintape/cas";
    let trans_ref_name = "refs/transitions/main";
    let l4_ref_name = "refs/chaintape/l4";
    let l4e_ref_name = "refs/chaintape/l4e";

    let cas_oid_before = repo.find_reference(cas_ref_name).ok().map(|r| r.target().unwrap());
    let trans_oid_before = repo.find_reference(trans_ref_name).ok().map(|r| r.target().unwrap());
    let l4_oid_before = repo.find_reference(l4_ref_name).ok().map(|r| r.target().unwrap());
    let l4e_oid_before = repo.find_reference(l4e_ref_name).ok().map(|r| r.target().unwrap());

    assert!(cas_oid_before.is_some(), "refs/chaintape/cas must exist before preview");

    let workspace_str = workspace.to_string_lossy().into_owned();

    let _guard = env_lock().lock().await;
    std::env::set_var("TURINGOS_WEB_WORKSPACE", &workspace_str);

    let addr = start_server().await;

    // Trigger preview
    let path_uri = format!(
        "/api/preview/{}/file?path=index.html&session_id={}&sandbox_policy=allowscripts",
        actual_written_bundle_cid_hex, session_id
    );
    let (status, _) = http_get(addr, &path_uri).await;
    assert_eq!(status, 200);

    std::env::remove_var("TURINGOS_WEB_WORKSPACE");
    drop(_guard);

    // Check OIDs after preview
    let repo_after = git2::Repository::open(&workspace).expect("git open");
    let cas_oid_after = repo_after.find_reference(cas_ref_name).ok().map(|r| r.target().unwrap());
    let trans_oid_after = repo_after.find_reference(trans_ref_name).ok().map(|r| r.target().unwrap());
    let l4_oid_after = repo_after.find_reference(l4_ref_name).ok().map(|r| r.target().unwrap());
    let l4e_oid_after = repo_after.find_reference(l4e_ref_name).ok().map(|r| r.target().unwrap());

    // State refs must remain identical/untouched
    assert_eq!(trans_oid_before, trans_oid_after);
    assert_eq!(l4_oid_before, l4_oid_after);
    assert_eq!(l4e_oid_before, l4e_oid_after);

    // CAS ref must have advanced
    let new_cas_oid = cas_oid_after.expect("refs/chaintape/cas must exist after preview");
    assert_ne!(cas_oid_before.unwrap(), new_cas_oid);

    // CAS ref must have advanced by EXACTLY one commit
    let new_commit = repo_after.find_commit(new_cas_oid).expect("new CAS commit");
    assert_eq!(new_commit.parent_count(), 1);
    let parent_oid = new_commit.parent_id(0).expect("parent commit ID");
    assert_eq!(parent_oid, cas_oid_before.unwrap(), "must advance by exactly one commit");
}
