//! True-suite MATH competition-reasoning runner contract.
//!
//! CI uses a local mock OpenAI-compatible proxy so it does not spend provider
//! tokens. The production runner script uses the same helper against a real
//! DeepSeek/SiliconFlow-backed proxy and public MATH dataset input.

use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::Path;
use std::process::Command;
use std::thread;

use serde_json::Value;
use tempfile::TempDir;
use turingosv4::bottom_white::cas::schema::{Cid, ObjectType};
use turingosv4::bottom_white::cas::store::CasStore;

fn bin(name: &str) -> &'static str {
    match name {
        "turingos" => env!("CARGO_BIN_EXE_turingos"),
        "verify_chaintape" => env!("CARGO_BIN_EXE_verify_chaintape"),
        "math_competition_reasoning_current_kernel" => {
            env!("CARGO_BIN_EXE_math_competition_reasoning_current_kernel")
        }
        _ => panic!("unknown bin {name}"),
    }
}

fn bin_dir(path: &str) -> &Path {
    Path::new(path).parent().expect("bin has parent")
}

fn read_json(path: &Path) -> Value {
    serde_json::from_str(&std::fs::read_to_string(path).expect("read json")).expect("parse json")
}

fn cid_from_hex(hex: &str) -> Cid {
    assert_eq!(hex.len(), 64, "cid must be 64 hex chars: {hex}");
    let mut bytes = [0u8; 32];
    for i in 0..32 {
        bytes[i] = u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).expect("cid hex byte");
    }
    Cid(bytes)
}

fn start_mock_llm_proxy() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind mock proxy");
    let addr = listener.local_addr().expect("local addr");
    thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("accept request");
        let mut buf = [0u8; 32768];
        let n = stream.read(&mut buf).expect("read request");
        let request = String::from_utf8_lossy(&buf[..n]);
        assert!(
            request.starts_with("POST /v1/chat/completions"),
            "unexpected mock proxy request: {request}"
        );
        assert!(
            request.contains("Problem capsule cid:")
                && request.contains("Subject: Algebra")
                && request.contains("Level: Level 3"),
            "prompt should bind the CAS problem capsule and public problem metadata: {request}"
        );
        assert!(
            !request.contains("expected_answer") && !request.contains("\\boxed{11}"),
            "prompt must not leak hidden answer key: {request}"
        );
        let rationale = "Substitute the two given values directly into the expression. The fourth power of two is sixteen, and five squared is twenty-five, so the doubled square term contributes fifty. Adding the numerator terms gives sixty-six, and dividing by six gives eleven. This is the requested exact value, so the final answer is 11.";
        let content = serde_json::json!({
            "final_answer": "11",
            "rationale": rationale
        })
        .to_string();
        let body = serde_json::json!({
            "model": "mock-math-agent",
            "choices": [
                {"message": {"content": content}}
            ],
            "usage": {"prompt_tokens": 41, "completion_tokens": 59, "total_tokens": 100}
        })
        .to_string();
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(),
            body
        );
        stream
            .write_all(response.as_bytes())
            .expect("write response");
    });
    format!("http://{addr}")
}

#[test]
fn math_runner_calls_proxy_writes_cas_claim_and_replays_worktx() {
    let tmp = TempDir::new().expect("tempdir");
    let run_dir = tmp.path().join("math");
    let proxy_url = start_mock_llm_proxy();
    let sample = tmp.path().join("math_sample.json");
    std::fs::write(
        &sample,
        r#"{
          "schema_version": "turingosv4.true_suite.math_sample.v1",
          "sample_id": "mock-math-algebra-001",
          "source_family": "MATH",
          "public_source": "https://huggingface.co/datasets/EleutherAI/hendrycks_math",
          "source_file": "mock-math-compatible.json",
          "subject": "Algebra",
          "level": "Level 3",
          "problem": "If $x = 2$ and $y = 5$, then what is the value of $\\frac{x^4+2y^2}{6}$ ?",
          "solution": "We have $\\frac{x^4 + 2y^2}{6} = \\frac{2^4 + 2(5^2)}{6} = \\frac{16+50}{6} = \\boxed{11}.$",
          "expected_answer": "11",
          "canary_string": "math:mock-canary-not-official"
        }"#,
    )
    .expect("write sample");

    let init = Command::new(bin("turingos"))
        .args([
            "init",
            "--project",
            run_dir.to_str().expect("utf8 path"),
            "--template",
            "proof",
            "--provider",
            "deepseek",
        ])
        .output()
        .expect("run turingos init");
    assert!(
        init.status.success(),
        "turingos init failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&init.stdout),
        String::from_utf8_lossy(&init.stderr)
    );
    std::fs::create_dir_all(run_dir.join("input_capsules")).expect("input dir");
    std::fs::copy(
        &sample,
        run_dir.join("input_capsules").join("math_sample.json"),
    )
    .expect("copy sample evidence");

    let helper = Command::new(bin("math_competition_reasoning_current_kernel"))
        .args([
            "--runtime-repo",
            run_dir.join("runtime_repo").to_str().expect("utf8 path"),
            "--cas",
            run_dir.join("cas").to_str().expect("utf8 path"),
            "--run-id",
            "constitution-true-suite-math",
            "--constitution",
            "constitution.md",
            "--sample-json",
            sample.to_str().expect("utf8 path"),
            "--llm-proxy-url",
            &proxy_url,
            "--model",
            "mock-math-agent",
            "--out-dir",
            run_dir.to_str().expect("utf8 path"),
        ])
        .output()
        .expect("run math helper");
    assert!(
        helper.status.success(),
        "math helper failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&helper.stdout),
        String::from_utf8_lossy(&helper.stderr)
    );

    let replay_report = run_dir.join("replay_report.json");
    let verify = Command::new(bin("turingos"))
        .env("TURINGOS_BIN_DIR", bin_dir(bin("verify_chaintape")))
        .args([
            "verify",
            "chaintape",
            "--repo",
            run_dir.join("runtime_repo").to_str().expect("utf8 path"),
            "--cas",
            run_dir.join("cas").to_str().expect("utf8 path"),
            "--run-id",
            "constitution-true-suite-math",
            "--out",
            replay_report.to_str().expect("utf8 path"),
        ])
        .output()
        .expect("run turingos verify chaintape");
    assert!(
        verify.status.success(),
        "turingos verify chaintape failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&verify.stdout),
        String::from_utf8_lossy(&verify.stderr)
    );

    let manifest = read_json(&run_dir.join("math_competition_reasoning_manifest.json"));
    assert_eq!(
        manifest.get("schema_version").and_then(Value::as_str),
        Some("turingosv4.true_suite.math_competition_reasoning.v1")
    );
    assert_eq!(
        manifest
            .get("rationale_guard_passed")
            .and_then(Value::as_bool),
        Some(true)
    );
    assert_eq!(
        manifest.get("answer_correct").and_then(Value::as_bool),
        Some(true)
    );
    assert_eq!(
        manifest.get("work_tx_landed").and_then(Value::as_bool),
        Some(true)
    );
    assert_eq!(
        manifest.get("closure_scope").and_then(Value::as_str),
        Some("domain_adapter_smoke_only")
    );
    assert_eq!(
        manifest
            .get("full_system_participation_required")
            .and_then(Value::as_bool),
        Some(true)
    );
    assert_eq!(
        manifest
            .get("final_closure_possible")
            .and_then(Value::as_bool),
        Some(false)
    );
    assert!(manifest.get("raw_response").is_none());
    assert!(manifest.get("raw_prompt").is_none());
    for key in [
        "problem_capsule_cid",
        "answer_claim_capsule_cid",
        "evaluation_capsule_cid",
        "proposal_telemetry_cid",
        "prompt_sha256",
        "provider_response_sha256",
    ] {
        assert_eq!(
            manifest.get(key).and_then(Value::as_str).map(str::len),
            Some(64),
            "{key} should be a 64-char hex digest/cid"
        );
    }

    let cas = CasStore::open(&run_dir.join("cas")).expect("open cas");
    let problem_cid = cid_from_hex(
        manifest
            .get("problem_capsule_cid")
            .and_then(Value::as_str)
            .expect("problem cid"),
    );
    let answer_cid = cid_from_hex(
        manifest
            .get("answer_claim_capsule_cid")
            .and_then(Value::as_str)
            .expect("answer cid"),
    );
    let evaluation_cid = cid_from_hex(
        manifest
            .get("evaluation_capsule_cid")
            .and_then(Value::as_str)
            .expect("evaluation cid"),
    );
    let telemetry_cid = cid_from_hex(
        manifest
            .get("proposal_telemetry_cid")
            .and_then(Value::as_str)
            .expect("proposal telemetry cid"),
    );
    assert_eq!(
        cas.metadata(&problem_cid).map(|m| m.object_type),
        Some(ObjectType::EvidenceCapsule)
    );
    assert_eq!(
        cas.metadata(&answer_cid).map(|m| m.object_type),
        Some(ObjectType::EvidenceCapsule)
    );
    assert_eq!(
        cas.metadata(&evaluation_cid).map(|m| m.object_type),
        Some(ObjectType::ProposalPayload)
    );
    assert_eq!(
        cas.metadata(&telemetry_cid).map(|m| m.object_type),
        Some(ObjectType::Generic)
    );

    let taxonomy = read_json(&run_dir.join("failure_taxonomy.json"));
    assert_eq!(
        taxonomy.get("model_task_failure").and_then(Value::as_bool),
        Some(false)
    );
    let replay = read_json(&replay_report);
    assert_eq!(
        replay.get("state_reconstructed").and_then(Value::as_bool),
        Some(true)
    );
    assert_eq!(
        replay
            .get("cas_payloads_retrievable")
            .and_then(Value::as_bool),
        Some(true)
    );
}

#[test]
fn math_runner_script_uses_public_dataset_proxy_and_no_raw_provider_evidence() {
    let script =
        std::fs::read_to_string("scripts/run_true_suite_math_competition_current_kernel.sh")
            .expect("read math runner script");
    assert!(script.contains("EleutherAI/hendrycks_math"));
    assert!(script.contains("datasets-server.huggingface.co/rows"));
    assert!(script.contains("LLM_PROXY_URL"));
    assert!(script.contains("math_competition_reasoning_current_kernel"));
    assert!(script.contains("verify chaintape"));
    assert!(script.contains("input_capsules"));
    assert!(script.contains("failure_taxonomy.json"));
    assert!(script.contains("benchmark accuracy is not treated as liveness closure"));
    assert!(
        !script.contains("pyarrow") && !script.contains("pandas"),
        "runner should use the single-row public API instead of dataframe/parquet dependencies"
    );
    assert!(
        script.contains(r#"marker = r"\boxed{""#)
            && script.contains(r#"re.search(r"\\boxed\s*\{([^{}]+)\}", solution)"#),
        "runner should parse single-backslash LaTeX boxed answers from decoded JSON"
    );
    assert!(
        !script.contains("old_15_question") && !script.contains("stage_phase7_real_e2e"),
        "Math runner must not inherit old-15 or historical product evidence as final input"
    );
    assert!(
        !script.contains("raw_response") && !script.contains("raw_prompt"),
        "runner script must not persist raw provider prompt/response artifacts"
    );
}
