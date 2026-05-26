//! True-suite GPQA science-reasoning runner contract.
//!
//! CI uses a local mock OpenAI-compatible proxy so it does not spend provider
//! tokens. The production runner script uses the same helper against a real
//! DeepSeek/SiliconFlow-backed proxy and public GPQA dataset input.

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
        "gpqa_science_reasoning_current_kernel" => {
            env!("CARGO_BIN_EXE_gpqa_science_reasoning_current_kernel")
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
            request.contains("Question capsule cid:") && request.contains("Choices:"),
            "prompt should bind the CAS question capsule and choices: {request}"
        );
        assert!(
            !request.contains("correct_answer") && !request.contains("correct_choice"),
            "prompt must not leak hidden answer key: {request}"
        );
        let rationale = "The stem describes a thermodynamic setup where equilibrium shifts after a controlled perturbation. The relevant comparison is between the sign of the free-energy change and the reaction quotient, so the option that names the spontaneous direction under those constraints is selected. The distractors confuse kinetic rate or catalyst effects with the equilibrium criterion, which does not decide the final state here.";
        let content = serde_json::json!({
            "answer_choice": "B",
            "rationale": rationale
        })
        .to_string();
        let body = serde_json::json!({
            "model": "mock-gpqa-agent",
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
fn gpqa_runner_calls_proxy_writes_cas_claim_and_replays_worktx() {
    let tmp = TempDir::new().expect("tempdir");
    let run_dir = tmp.path().join("gpqa");
    let proxy_url = start_mock_llm_proxy();
    let sample = tmp.path().join("gpqa_sample.json");
    std::fs::write(
        &sample,
        r#"{
          "schema_version": "turingosv4.true_suite.gpqa_sample.v1",
          "sample_id": "mock-gpqa-thermo-001",
          "source_family": "GPQA",
          "public_source": "https://github.com/idavidrein/gpqa",
          "source_file": "mock-gpqa-compatible.json",
          "high_level_domain": "Chemistry",
          "subdomain": "Physical chemistry",
          "question": "A reversible reaction mixture is perturbed at fixed temperature. Which criterion identifies the spontaneous direction before the system re-equilibrates?",
          "choices": {
            "A": "The catalyst concentration alone determines the final direction.",
            "B": "Compare the reaction quotient with the equilibrium constant through the free-energy change.",
            "C": "Select the direction with the larger forward rate constant regardless of concentrations.",
            "D": "The direction is unknowable once the temperature is fixed."
          },
          "correct_choice": "B",
          "correct_answer": "Compare the reaction quotient with the equilibrium constant through the free-energy change.",
          "canary_string": "gpqa:mock-canary-not-official"
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
        run_dir.join("input_capsules").join("gpqa_sample.json"),
    )
    .expect("copy sample evidence");

    let helper = Command::new(bin("gpqa_science_reasoning_current_kernel"))
        .args([
            "--runtime-repo",
            run_dir.join("runtime_repo").to_str().expect("utf8 path"),
            "--cas",
            run_dir.join("cas").to_str().expect("utf8 path"),
            "--run-id",
            "constitution-true-suite-gpqa",
            "--constitution",
            "constitution.md",
            "--sample-json",
            sample.to_str().expect("utf8 path"),
            "--llm-proxy-url",
            &proxy_url,
            "--model",
            "mock-gpqa-agent",
            "--out-dir",
            run_dir.to_str().expect("utf8 path"),
        ])
        .output()
        .expect("run gpqa helper");
    assert!(
        helper.status.success(),
        "gpqa helper failed\nstdout:\n{}\nstderr:\n{}",
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
            "constitution-true-suite-gpqa",
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

    let manifest = read_json(&run_dir.join("gpqa_science_reasoning_manifest.json"));
    assert_eq!(
        manifest.get("schema_version").and_then(Value::as_str),
        Some("turingosv4.true_suite.gpqa_science_reasoning.v1")
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
        "question_capsule_cid",
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
    let question_cid = cid_from_hex(
        manifest
            .get("question_capsule_cid")
            .and_then(Value::as_str)
            .expect("question cid"),
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
        cas.metadata(&question_cid).map(|m| m.object_type),
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
fn gpqa_runner_script_uses_public_dataset_proxy_and_no_raw_provider_evidence() {
    let script =
        std::fs::read_to_string("scripts/run_true_suite_gpqa_science_reasoning_current_kernel.sh")
            .expect("read gpqa runner script");
    assert!(script.contains("idavidrein/gpqa"));
    assert!(script.contains("LLM_PROXY_URL"));
    assert!(script.contains("gpqa_science_reasoning_current_kernel"));
    assert!(script.contains("verify chaintape"));
    assert!(script.contains("input_capsules"));
    assert!(script.contains("failure_taxonomy.json"));
    assert!(script.contains("benchmark accuracy is not treated as liveness closure"));
    assert!(
        script.contains("rm -f \"$RUN_DIR/input_capsules/gpqa_dataset.zip\"")
            && script.contains("rm -rf \"$RUN_DIR/input_capsules/gpqa_dataset\""),
        "runner should remove temporary full-dataset download/extract artifacts after materializing the sample"
    );
    assert!(
        !script.contains("old_15_question") && !script.contains("stage_phase7_real_e2e"),
        "GPQA runner must not inherit old-15 or historical product evidence as final input"
    );
    assert!(
        !script.contains("raw_response") && !script.contains("raw_prompt"),
        "runner script must not persist raw provider prompt/response artifacts"
    );
}
