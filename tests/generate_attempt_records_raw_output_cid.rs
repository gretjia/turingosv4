use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::PathBuf;
use std::process::Command;
use std::thread;
use turingosv4::bottom_white::cas::schema::ObjectType;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::generation_attempt::{AttemptOutcome, GenerationAttemptCapsule};

fn parse_cid_hex(s: &str) -> turingosv4::bottom_white::cas::schema::Cid {
    let mut out = [0u8; 32];
    for (i, byte) in out.iter_mut().enumerate() {
        let chunk = &s[i * 2..i * 2 + 2];
        *byte = u8::from_str_radix(chunk, 16).unwrap();
    }
    turingosv4::bottom_white::cas::schema::Cid(out)
}

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
fn test_generate_attempt_records_raw_output_cid() {
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
    let raw_response = "{\n  \"choices\": [\n    {\n      \"message\": {\n        \"role\": \"assistant\",\n        \"content\": \"### File: index.html\\n```html\\n<!doctype html>\\n<html><body><main>ok</main></body></html>\\n```\"\n      },\n      \"finish_reason\": \"stop\"\n    }\n  ],\n  \"usage\": {\n    \"prompt_tokens\": 10,\n    \"completion_tokens\": 20,\n    \"total_tokens\": 30\n  }\n}".to_string();

    let endpoint = start_mock_llm_server(raw_response.clone());

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
    assert_eq!(cap.outcome, AttemptOutcome::Success);
    assert_eq!(cap.parsed_file_count, 1);

    let raw_cid_hex = cap.raw_output_cid.expect("raw_output_cid is missing");
    let raw_cid = parse_cid_hex(&raw_cid_hex);
    let raw_bytes = store.get(&raw_cid).expect("read raw response from CAS");
    let read_raw = String::from_utf8(raw_bytes).expect("convert raw response to string");

    assert_eq!(read_raw, raw_response);
}
