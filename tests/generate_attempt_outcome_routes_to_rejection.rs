use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use turingosv4::runtime::generation_attempt::{GenerationAttemptCapsule, AttemptOutcome};
use turingosv4::runtime::rejection_capsule::{GenerateRejectionCapsule, RejectClass};
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::cas::schema::ObjectType;

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

fn start_mock_llm_error_server() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    thread::spawn(move || {
        if let Ok((mut stream, _)) = listener.accept() {
            let mut buf = [0; 4096];
            let _ = stream.read(&mut buf);
            let response = "HTTP/1.1 500 Internal Server Error\r\nConnection: close\r\nContent-Length: 0\r\n\r\n";
            let _ = stream.write_all(response.as_bytes());
            let _ = stream.flush();
        }
    });
    format!("http://127.0.0.1:{}", port)
}

#[test]
fn test_generate_attempt_outcome_routes_to_rejection_llm_error() {
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

    let endpoint = start_mock_llm_error_server();

    // Run generate command (should fail)
    let output = Command::new(turingos_bin())
        .arg("generate")
        .arg("--workspace")
        .arg(&ws)
        .env("TURINGOS_SILICONFLOW_ENDPOINT", &endpoint)
        .env("SILICONFLOW_API_KEY", "mock-key")
        .output()
        .expect("run generate");

    assert!(!output.status.success());

    // Verify GenerationAttemptCapsule and GenerateRejectionCapsule in CAS
    let cas_dir = ws.join("cas");
    let store = CasStore::open(&cas_dir).expect("open cas store");
    let cids = store.list_cids_by_object_type(ObjectType::EvidenceCapsule);
    
    let mut attempt_capsule: Option<(GenerationAttemptCapsule, String)> = None;
    let mut rejection_capsule: Option<GenerateRejectionCapsule> = None;
    
    for cid in cids {
        if let Some(meta) = store.metadata(&cid) {
            if meta.schema_id.as_deref() == Some("turingos-generation-attempt-v1") {
                let bytes = store.get(&cid).expect("read capsule");
                let cap: GenerationAttemptCapsule = serde_json::from_slice(&bytes).expect("deserialize");
                attempt_capsule = Some((cap, cid.hex()));
            } else if meta.schema_id.as_deref() == Some("turingos-generate-rejection-v1") {
                let bytes = store.get(&cid).expect("read capsule");
                let cap: GenerateRejectionCapsule = serde_json::from_slice(&bytes).expect("deserialize");
                rejection_capsule = Some(cap);
            }
        }
    }

    let (attempt, attempt_cid) = attempt_capsule.expect("GenerationAttemptCapsule not found");
    assert_eq!(attempt.outcome, AttemptOutcome::LlmApiError);

    let rejection = rejection_capsule.expect("GenerateRejectionCapsule not found");
    assert_eq!(rejection.reject_class, RejectClass::LlmApiError);
    assert_eq!(rejection.generation_attempt_cid, Some(attempt_cid));
    assert!(rejection.retryable);
    assert!(rejection.world_head_unchanged);
}

#[test]
fn test_generate_attempt_outcome_routes_to_rejection_no_files() {
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

    // LLM response with no files parsed
    let raw_response = r#"{
      "choices": [
        {
          "message": {
            "role": "assistant",
            "content": "Here is no code files."
          },
          "finish_reason": "stop"
        }
      ],
      "usage": {
        "prompt_tokens": 5,
        "completion_tokens": 5,
        "total_tokens": 10
      }
    }"#.to_string();

    let endpoint = start_mock_llm_server(raw_response);

    // Run generate command (should fail)
    let output = Command::new(turingos_bin())
        .arg("generate")
        .arg("--workspace")
        .arg(&ws)
        .env("TURINGOS_SILICONFLOW_ENDPOINT", &endpoint)
        .env("SILICONFLOW_API_KEY", "mock-key")
        .output()
        .expect("run generate");

    assert!(!output.status.success());

    // Verify GenerationAttemptCapsule and GenerateRejectionCapsule in CAS
    let cas_dir = ws.join("cas");
    let store = CasStore::open(&cas_dir).expect("open cas store");
    let cids = store.list_cids_by_object_type(ObjectType::EvidenceCapsule);
    
    let mut attempt_capsule: Option<(GenerationAttemptCapsule, String)> = None;
    let mut rejection_capsule: Option<GenerateRejectionCapsule> = None;
    
    for cid in cids {
        if let Some(meta) = store.metadata(&cid) {
            if meta.schema_id.as_deref() == Some("turingos-generation-attempt-v1") {
                let bytes = store.get(&cid).expect("read capsule");
                let cap: GenerationAttemptCapsule = serde_json::from_slice(&bytes).expect("deserialize");
                attempt_capsule = Some((cap, cid.hex()));
            } else if meta.schema_id.as_deref() == Some("turingos-generate-rejection-v1") {
                let bytes = store.get(&cid).expect("read capsule");
                let cap: GenerateRejectionCapsule = serde_json::from_slice(&bytes).expect("deserialize");
                rejection_capsule = Some(cap);
            }
        }
    }

    let (attempt, attempt_cid) = attempt_capsule.expect("GenerationAttemptCapsule not found");
    assert_eq!(attempt.outcome, AttemptOutcome::NoFilesParsed);

    let rejection = rejection_capsule.expect("GenerateRejectionCapsule not found");
    assert_eq!(rejection.reject_class, RejectClass::NoFilesParsed);
    assert_eq!(rejection.generation_attempt_cid, Some(attempt_cid));
    assert!(rejection.retryable);
    assert!(rejection.world_head_unchanged);
}
