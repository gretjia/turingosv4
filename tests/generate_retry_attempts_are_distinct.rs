use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::PathBuf;
use std::process::Command;
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

fn start_mock_llm_server(response_body: String, max_requests: usize) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    thread::spawn(move || {
        for _ in 0..max_requests {
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
        }
    });
    format!("http://127.0.0.1:{}", port)
}

#[test]
fn test_generate_retry_attempts_are_distinct() {
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

    // Setup mock LLM response
    let raw_response = "{\n  \"choices\": [\n    {\n      \"message\": {\n        \"role\": \"assistant\",\n        \"content\": \"### File: index.html\\n```html\\n<!DOCTYPE html><html><body><h1>Hello</h1></body></html>\\n```\"\n      },\n      \"finish_reason\": \"stop\"\n    }\n  ],\n  \"usage\": {\n    \"prompt_tokens\": 10,\n    \"completion_tokens\": 20,\n    \"total_tokens\": 30\n  }\n}".to_string();

    let endpoint = start_mock_llm_server(raw_response, 2);

    // First generate run
    let output1 = Command::new(turingos_bin())
        .arg("generate")
        .arg("--workspace")
        .arg(&ws)
        .env("TURINGOS_SILICONFLOW_ENDPOINT", &endpoint)
        .env("SILICONFLOW_API_KEY", "mock-key")
        .output()
        .expect("run generate 1");
    assert!(output1.status.success());

    // Second generate run
    let output2 = Command::new(turingos_bin())
        .arg("generate")
        .arg("--workspace")
        .arg(&ws)
        .env("TURINGOS_SILICONFLOW_ENDPOINT", &endpoint)
        .env("SILICONFLOW_API_KEY", "mock-key")
        .output()
        .expect("run generate 2");
    assert!(output2.status.success());

    // Check CAS attempts
    let cas_dir = ws.join("cas");
    let store = CasStore::open(&cas_dir).expect("open cas store");
    let cids = store.list_cids_by_object_type(ObjectType::EvidenceCapsule);

    let mut attempts = Vec::new();
    for cid in cids {
        if let Some(meta) = store.metadata(&cid) {
            if meta.schema_id.as_deref() == Some("turingos-generation-attempt-v1") {
                let bytes = store.get(&cid).expect("read capsule");
                let cap: GenerationAttemptCapsule =
                    serde_json::from_slice(&bytes).expect("deserialize");
                attempts.push((cap, cid.hex()));
            }
        }
    }

    // Sort by retry_index
    attempts.sort_by_key(|x| x.0.retry_index);
    assert_eq!(
        attempts.len(),
        2,
        "Expected 2 generation attempts, found {}",
        attempts.len()
    );

    let (cap0, cid0) = &attempts[0];
    let (cap1, _) = &attempts[1];

    assert_eq!(cap0.retry_index, 0);
    assert_eq!(cap0.parent_attempt_cid, None);

    assert_eq!(cap1.retry_index, 1);
    assert_eq!(cap1.parent_attempt_cid, Some(cid0.clone()));
}
