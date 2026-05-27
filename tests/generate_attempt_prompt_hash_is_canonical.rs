use sha2::{Digest, Sha256};
use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use turingosv4::bottom_white::cas::schema::ObjectType;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::generation_attempt::GenerationAttemptCapsule;

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

/// Starts a mock HTTP server that captures the request body into `capture`,
/// then responds with `response_body`. Reads exactly Content-Length bytes so
/// it works correctly regardless of request size.
fn start_mock_llm_server(response_body: String, capture: Arc<Mutex<Option<Vec<u8>>>>) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    thread::spawn(move || {
        if let Ok((stream, _)) = listener.accept() {
            let mut reader = BufReader::new(stream);

            // Read HTTP request headers line by line until blank line
            let mut content_length: usize = 0;
            loop {
                let mut line = String::new();
                reader.read_line(&mut line).unwrap_or(0);
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    break;
                }
                let lower = trimmed.to_ascii_lowercase();
                if lower.starts_with("content-length:") {
                    if let Some(val) = lower.strip_prefix("content-length:") {
                        content_length = val.trim().parse().unwrap_or(0);
                    }
                }
            }

            // Read exactly content_length bytes as the request body
            let mut body = vec![0u8; content_length];
            reader.read_exact(&mut body).unwrap_or(());

            *capture.lock().unwrap() = Some(body);

            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                response_body.len(),
                response_body
            );
            let mut writer = reader.into_inner();
            let _ = writer.write_all(response.as_bytes());
            let _ = writer.flush();
        }
    });
    format!("http://127.0.0.1:{}", port)
}

#[test]
fn test_generate_attempt_prompt_hash_is_canonical() {
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
    let spec_content = "# Test Spec\nGenerate some code.";
    let spec_path = ws.join("spec.md");
    fs::write(&spec_path, spec_content).expect("write spec.md");

    let raw_response = "{\n  \"choices\": [\n    {\n      \"message\": {\n        \"role\": \"assistant\",\n        \"content\": \"### File: index.html\\n```html\\n<!doctype html>\\n<html><body><main>ok</main></body></html>\\n```\"\n      },\n      \"finish_reason\": \"stop\"\n    }\n  ],\n  \"usage\": {\n    \"prompt_tokens\": 10,\n    \"completion_tokens\": 20,\n    \"total_tokens\": 30\n  }\n}".to_string();

    let captured_body: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));
    let endpoint = start_mock_llm_server(raw_response, Arc::clone(&captured_body));

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
        "generate failed:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    // Verify GenerationAttemptCapsule is recorded in CAS
    let cas_dir = ws.join("cas");
    let store = CasStore::open(&cas_dir).expect("open cas store");
    let cids = store.list_cids_by_object_type(ObjectType::EvidenceCapsule);

    let mut attempt_capsule: Option<GenerationAttemptCapsule> = None;
    for cid in cids {
        if let Some(meta) = store.metadata(&cid) {
            if meta.schema_id.as_deref() == Some("turingos-generation-attempt-v1") {
                let bytes = store.get(&cid).expect("read capsule");
                let cap: GenerationAttemptCapsule =
                    serde_json::from_slice(&bytes).expect("deserialize");
                attempt_capsule = Some(cap);
                break;
            }
        }
    }

    let cap = attempt_capsule.expect("GenerationAttemptCapsule not found in CAS");

    // Hash the actual HTTP request body captured by the mock server.
    // This is the canonical byte sequence that production hashes for prompt_hash.
    let body = captured_body
        .lock()
        .unwrap()
        .take()
        .expect("mock server did not capture a request body");

    let mut hasher = Sha256::new();
    hasher.update(&body);
    let expected_hash = format!("{:x}", hasher.finalize());

    assert_eq!(
        cap.prompt_hash, expected_hash,
        "prompt_hash mismatch: capsule stored {:?} but SHA-256 of actual request body is {:?}",
        cap.prompt_hash, expected_hash
    );
}
