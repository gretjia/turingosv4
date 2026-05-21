use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::PathBuf;
use std::process::Command;
use std::thread;
use turingosv4::bottom_white::cas::schema::ObjectType;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::artifact_bundle::{ArtifactBundleManifest, ArtifactFileRole};

fn turingos_bin() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let candidates = [
        format!("{manifest_dir}/target/debug/turingos"),
        format!("{manifest_dir}/target/release/turingos"),
    ];
    for candidate in candidates.iter() {
        let path = PathBuf::from(candidate);
        if path.exists() {
            return path;
        }
    }
    panic!("turingos binary not found");
}

fn start_mock_llm_server(response_body: String) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    thread::spawn(move || {
        if let Ok((mut stream, _)) = listener.accept() {
            let mut buf = [0; 4096];
            let _ = stream.read(&mut buf);
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                response_body.len(),
                response_body
            );
            let _ = stream.write_all(response.as_bytes());
            let _ = stream.flush();
        }
    });
    format!("http://127.0.0.1:{}", port)
}

#[test]
fn test_generated_artifact_has_bundle_manifest() {
    let tmp = tempfile::tempdir().expect("create temp workspace");
    let ws = tmp.path().join("my_workspace");

    // Init workspace
    let status = Command::new(turingos_bin())
        .arg("init")
        .arg("--project")
        .arg(&ws)
        .status()
        .expect("run init");
    assert!(status.success());

    // Write spec.md
    fs::write(ws.join("spec.md"), "# Test Spec\nGenerate some code.").expect("write spec.md");

    // Setup mock LLM response emitting index.html and main.js
    let raw_response = "{\n  \"choices\": [\n    {\n      \"message\": {\n        \"role\": \"assistant\",\n        \"content\": \"### File: index.html\\n```html\\n<!DOCTYPE html><html><body><h1>Hello</h1></body></html>\\n```\\n\\n### File: main.js\\n```javascript\\nconsole.log('hi');\\n```\"\n      },\n      \"finish_reason\": \"stop\"\n    }\n  ],\n  \"usage\": {\n    \"prompt_tokens\": 10,\n    \"completion_tokens\": 20,\n    \"total_tokens\": 30\n  }\n}".to_string();

    let endpoint = start_mock_llm_server(raw_response);

    // Run generate command
    let output = Command::new(turingos_bin())
        .arg("generate")
        .arg("--workspace")
        .arg(&ws)
        .env("TURINGOS_SILICONFLOW_ENDPOINT", &endpoint)
        .env("SILICONFLOW_API_KEY", "mock-key")
        .output()
        .expect("run generate");

    assert!(
        output.status.success(),
        "generate failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Verify stdout contains artifact_bundle_cid=
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("artifact_bundle_cid="),
        "stdout must print bundle cid"
    );

    let cid_line = stdout
        .lines()
        .find(|l| l.contains("artifact_bundle_cid="))
        .unwrap();
    let bundle_cid = cid_line.split('=').nth(1).unwrap().trim().to_string();
    assert_eq!(bundle_cid.len(), 64);

    // Verify ArtifactBundleManifest is recorded in CAS
    let cas_dir = ws.join("cas");
    let store = CasStore::open(&cas_dir).expect("open cas store");
    let cids = store.list_cids_by_object_type(ObjectType::EvidenceCapsule);

    let mut manifest_opt: Option<ArtifactBundleManifest> = None;
    for cid in cids {
        if let Some(meta) = store.metadata(&cid) {
            if meta.schema_id.as_deref() == Some("turingos-artifact-bundle-v1") {
                let bytes = store.get(&cid).expect("read manifest");
                let m: ArtifactBundleManifest =
                    serde_json::from_slice(&bytes).expect("deserialize");
                manifest_opt = Some(m);
                break;
            }
        }
    }

    let manifest = manifest_opt.expect("ArtifactBundleManifest not found in CAS");
    assert_eq!(manifest.files.len(), 2);
    assert_eq!(manifest.entrypoint, "index.html");

    // Check classification roles
    let index_file = manifest
        .files
        .iter()
        .find(|f| f.path == "index.html")
        .unwrap();
    assert_eq!(index_file.role, ArtifactFileRole::Entrypoint);
    assert_eq!(index_file.mime, "text/html");

    let js_file = manifest.files.iter().find(|f| f.path == "main.js").unwrap();
    assert_eq!(js_file.role, ArtifactFileRole::Source);
    assert_eq!(js_file.mime, "text/javascript");
}
