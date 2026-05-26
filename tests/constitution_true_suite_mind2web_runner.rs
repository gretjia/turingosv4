//! True-suite Mind2Web browser-action runner contract.
//!
//! CI uses a local mock OpenAI-compatible proxy so it does not spend provider
//! tokens. The production runner script uses the same helper against a real
//! DeepSeek/SiliconFlow-backed proxy and public Mind2Web offline webpage input.

use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::Path;
use std::process::Command;
use std::thread;

use serde_json::Value;
use tempfile::TempDir;
use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::proposal_telemetry::read_from_cas as read_proposal_telemetry;

#[path = "support/full_system.rs"]
mod full_system;

fn bin(name: &str) -> &'static str {
    match name {
        "turingos" => env!("CARGO_BIN_EXE_turingos"),
        "verify_chaintape" => env!("CARGO_BIN_EXE_verify_chaintape"),
        "mind2web_browser_action_current_kernel" => {
            env!("CARGO_BIN_EXE_mind2web_browser_action_current_kernel")
        }
        "full_system_augment_current_kernel" => {
            env!("CARGO_BIN_EXE_full_system_augment_current_kernel")
        }
        "full_system_participation_current_kernel" => {
            env!("CARGO_BIN_EXE_full_system_participation_current_kernel")
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

fn start_mock_llm_proxy_with_content(content: String) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind mock proxy");
    let addr = listener.local_addr().expect("local addr");
    thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("accept request");
        let mut buf = [0u8; 65536];
        let n = stream.read(&mut buf).expect("read request");
        let request = String::from_utf8_lossy(&buf[..n]);
        assert!(
            request.starts_with("POST /v1/chat/completions"),
            "unexpected mock proxy request: {request}"
        );
        assert!(
            request.contains("Input capsule cid:")
                && request.contains("Page snapshot cid:")
                && request.contains("backend_node_id=136"),
            "prompt should bind CAS input, page snapshot, and candidate ids: {request}"
        );
        assert!(
            !request.contains("action_repr")
                && !request.contains("target_backend_node_ids_sha256")
                && !request.contains("Reservation type -> SELECT: Pickup"),
            "prompt must not leak hidden Mind2Web target/action labels: {request}"
        );
        let body = serde_json::json!({
            "model": "mock-mind2web-agent",
            "choices": [
                {"message": {"content": content}}
            ],
            "usage": {"prompt_tokens": 89, "completion_tokens": 51, "total_tokens": 140}
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

fn start_mock_llm_proxy() -> String {
    let rationale = "The task asks to check pickup availability, so the first required interaction is the reservation-type selector. The candidate with backend node 136 is the select element for the reservation search type, and choosing Pickup aligns the form with the task before entering location and time details.";
    let content = serde_json::json!({
        "backend_node_id": "136",
        "operation": "SELECT",
        "value": "Pickup",
        "rationale": rationale
    })
    .to_string();
    start_mock_llm_proxy_with_content(content)
}

fn write_mock_sample(sample: &Path) {
    std::fs::write(
        sample,
        r#"{
          "schema_version": "turingosv4.true_suite.mind2web_sample.v1",
          "sample_id": "mock-mind2web-reservation-001:0",
          "source_family": "Mind2Web",
          "public_source": "https://huggingface.co/datasets/osunlp/Mind2Web",
          "source_file": "data/train/train_0.json",
          "website": "exploretock",
          "domain": "Travel",
          "subdomain": "Restaurant",
          "annotation_id": "mock-mind2web-reservation-001",
          "confirmed_task": "Check for pickup restaurant available in Boston, NY on March 18, 5pm with just one guest",
          "action_index": 0,
          "action_repr": "[combobox] Reservation type -> SELECT: Pickup",
          "cleaned_html": "<html><body><select backend_node_id=\"136\" id=\"reservations-city-search-type\" name=\"type\"><option>Pickup</option><option>Delivery</option></select><button backend_node_id=\"647\" aria_label=\"Book a reservation\"></button></body></html>",
          "operation": {"op": "SELECT", "original_op": "SELECT", "value": "Pickup"},
          "pos_candidates": [
            {
              "tag": "select",
              "attributes": "{\"backend_node_id\":\"136\",\"id\":\"reservations-city-search-type\",\"name\":\"type\",\"class\":\"MuiSelect-root\"}",
              "backend_node_id": "136",
              "is_original_target": true,
              "is_top_level_target": true
            }
          ],
          "neg_candidates": [
            {
              "tag": "button",
              "attributes": "{\"backend_node_id\":\"647\",\"aria_label\":\"Book a reservation\"}",
              "backend_node_id": "647"
            }
          ]
        }"#,
    )
    .expect("write sample");
}

fn init_mind2web_workspace(run_dir: &Path, sample: &Path) {
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
    std::fs::create_dir_all(run_dir.join("input_capsules")).expect("input capsule dir");
    std::fs::copy(
        sample,
        run_dir.join("input_capsules").join("mind2web_sample.json"),
    )
    .expect("copy sample evidence");
}

#[test]
fn mind2web_runner_calls_proxy_records_browser_action_and_replays_worktx() {
    let tmp = TempDir::new().expect("tempdir");
    let run_dir = tmp.path().join("mind2web");
    let proxy_url = start_mock_llm_proxy();
    let sample = tmp.path().join("mind2web_sample.json");
    write_mock_sample(&sample);

    init_mind2web_workspace(&run_dir, &sample);

    let helper = Command::new(bin("mind2web_browser_action_current_kernel"))
        .args([
            "--runtime-repo",
            run_dir.join("runtime_repo").to_str().expect("utf8 path"),
            "--cas",
            run_dir.join("cas").to_str().expect("utf8 path"),
            "--run-id",
            "constitution-true-suite-mind2web",
            "--constitution",
            "constitution.md",
            "--sample-json",
            sample.to_str().expect("utf8 path"),
            "--llm-proxy-url",
            &proxy_url,
            "--model",
            "mock-mind2web-agent",
            "--out-dir",
            run_dir.to_str().expect("utf8 path"),
        ])
        .output()
        .expect("run mind2web helper");
    assert!(
        helper.status.success(),
        "mind2web helper failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&helper.stdout),
        String::from_utf8_lossy(&helper.stderr)
    );
    full_system::run_full_system_augment(
        &run_dir,
        "constitution-true-suite-mind2web",
        bin("full_system_augment_current_kernel"),
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
            "constitution-true-suite-mind2web",
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
    full_system::assert_full_system_lit(
        &run_dir,
        "constitution-true-suite-mind2web",
        "mind2web_open_web",
        "tests/constitution_true_suite_mind2web_runner.rs",
        "mind2web_browser_action_manifest.json",
        &replay_report,
        bin("full_system_participation_current_kernel"),
    );

    let manifest = read_json(&run_dir.join("mind2web_browser_action_manifest.json"));
    assert_eq!(
        manifest.get("schema_version").and_then(Value::as_str),
        Some("turingosv4.true_suite.mind2web_browser_action.v1")
    );
    assert_eq!(
        manifest
            .get("selected_candidate_available")
            .and_then(Value::as_bool),
        Some(true)
    );
    assert_eq!(
        manifest.get("exact_match").and_then(Value::as_bool),
        Some(true)
    );
    assert_eq!(
        manifest.get("work_tx_landed").and_then(Value::as_bool),
        Some(true)
    );
    assert!(manifest.get("raw_response").is_none());
    assert!(manifest.get("raw_prompt").is_none());

    let fc_trace = read_json(&run_dir.join("fc_trace_report.json"));
    assert_eq!(
        fc_trace
            .get("browser_action_exact_match")
            .and_then(Value::as_bool),
        Some(true)
    );
    assert_eq!(
        fc_trace
            .get("final_closure_possible")
            .and_then(Value::as_bool),
        Some(false)
    );
    assert_eq!(
        fc_trace.get("closure_scope").and_then(Value::as_str),
        Some("domain_adapter_smoke_only")
    );
    assert_eq!(
        fc_trace
            .get("full_system_participation_required")
            .and_then(Value::as_bool),
        Some(true)
    );

    let mut cas = CasStore::open(&run_dir.join("cas")).expect("open CAS");
    for key in [
        "page_snapshot_cid",
        "input_capsule_cid",
        "answer_claim_capsule_cid",
        "browser_action_trace_cid",
        "evaluation_capsule_cid",
    ] {
        let cid = cid_from_hex(manifest.get(key).and_then(Value::as_str).expect(key));
        let bytes = cas.get(&cid).expect("cas get");
        assert!(
            !bytes.is_empty(),
            "CAS object for {key} must be retrievable"
        );
    }
    let telemetry_cid = cid_from_hex(
        manifest
            .get("proposal_telemetry_cid")
            .and_then(Value::as_str)
            .expect("proposal telemetry cid"),
    );
    let telemetry =
        read_proposal_telemetry(&mut cas, &telemetry_cid).expect("proposal telemetry decodes");
    assert_eq!(telemetry.tool_calls.len(), 1);
    assert_eq!(telemetry.tool_calls[0].tool_id, "browser_sandbox::SELECT");

    let replay = read_json(&replay_report);
    assert_eq!(
        replay.get("ledger_root_verified").and_then(Value::as_bool),
        Some(true)
    );
    assert_eq!(
        replay
            .get("agent_signatures_verified")
            .and_then(Value::as_bool),
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
fn mind2web_runner_records_missing_backend_node_as_model_failure_not_infrastructure_failure() {
    let tmp = TempDir::new().expect("tempdir");
    let run_dir = tmp.path().join("mind2web_missing_backend");
    let content = serde_json::json!({
        "operation": "SELECT",
        "value": "Pickup",
        "rationale": "The select element appears to control reservation type, but this intentionally omits backend_node_id to exercise fail-closed evidence."
    })
    .to_string();
    let proxy_url = start_mock_llm_proxy_with_content(content);
    let sample = tmp.path().join("mind2web_sample.json");
    write_mock_sample(&sample);
    init_mind2web_workspace(&run_dir, &sample);

    let helper = Command::new(bin("mind2web_browser_action_current_kernel"))
        .args([
            "--runtime-repo",
            run_dir.join("runtime_repo").to_str().expect("utf8 path"),
            "--cas",
            run_dir.join("cas").to_str().expect("utf8 path"),
            "--run-id",
            "constitution-true-suite-mind2web-missing-backend",
            "--constitution",
            "constitution.md",
            "--sample-json",
            sample.to_str().expect("utf8 path"),
            "--llm-proxy-url",
            &proxy_url,
            "--model",
            "mock-mind2web-agent",
            "--out-dir",
            run_dir.to_str().expect("utf8 path"),
        ])
        .output()
        .expect("run mind2web helper");
    assert!(
        helper.status.success(),
        "missing backend node should be model-task failure evidence, not helper crash\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&helper.stdout),
        String::from_utf8_lossy(&helper.stderr)
    );

    let manifest = read_json(&run_dir.join("mind2web_browser_action_manifest.json"));
    assert_eq!(
        manifest
            .get("selected_candidate_available")
            .and_then(Value::as_bool),
        Some(false)
    );
    assert_eq!(
        manifest.get("exact_match").and_then(Value::as_bool),
        Some(false)
    );
    assert_eq!(
        manifest.get("work_tx_landed").and_then(Value::as_bool),
        Some(true)
    );

    let taxonomy = read_json(&run_dir.join("failure_taxonomy.json"));
    assert_eq!(
        taxonomy.get("model_task_failure").and_then(Value::as_bool),
        Some(true)
    );
    assert_eq!(
        taxonomy
            .get("infrastructure_failure")
            .and_then(Value::as_bool),
        Some(false)
    );

    let answer_claim = read_json(&run_dir.join("input_capsules").join("answer_claim.json"));
    assert_eq!(
        answer_claim
            .get("selected_backend_node_id")
            .and_then(Value::as_str),
        Some("__missing_backend_node_id__")
    );
    assert!(answer_claim.get("raw_response").is_none());
    assert!(answer_claim.get("raw_prompt").is_none());
}

#[test]
fn mind2web_runner_script_uses_public_dataset_and_preserves_external_boundary() {
    let script = std::fs::read_to_string("scripts/run_true_suite_mind2web_current_kernel.sh")
        .expect("read Mind2Web runner script");
    assert!(script.contains("osunlp/Mind2Web"));
    assert!(script.contains("data/train/train_0.json"));
    assert!(script.contains("MIND2WEB_SAMPLE_JSON"));
    assert!(script.contains("LLM_PROXY_URL"));
    assert!(script.contains("mind2web_browser_action_current_kernel"));
    assert!(script.contains("full_system_augment_current_kernel"));
    assert!(script.contains("full_system_participation_current_kernel"));
    assert!(script.contains("turingos verify chaintape"));
    assert!(script.contains("--require-full-system"));
    assert!(script.contains("governance_capsule_index.json"));
    assert!(script.contains("full_system_augmentation_manifest.json"));
    assert!(script.contains("raw provider prompt and response are not written"));
    assert!(script.contains("not live website side effects"));
    assert!(
        !script.contains("stage_phase7_real_e2e")
            && !script.contains("real8x_market_ab_clean")
            && !script.contains("old_15_question"),
        "Mind2Web runner must not inherit historical product evidence as final input"
    );
}
