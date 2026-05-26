//! True-suite ToolBench API tool-use runner contract.
//!
//! CI uses a local mock OpenAI-compatible proxy so it does not spend provider
//! tokens. The production runner script uses the same helper against a real
//! DeepSeek/SiliconFlow-backed proxy and public ToolBench benchmark input.

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
        "toolbench_api_tool_use_current_kernel" => {
            env!("CARGO_BIN_EXE_toolbench_api_tool_use_current_kernel")
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
            request.contains("Input capsule cid:") && request.contains("Available API ids:"),
            "prompt should bind CAS input capsule and available APIs: {request}"
        );
        assert!(
            !request.contains("relevant_apis"),
            "prompt must not leak hidden ToolBench relevant_apis labels: {request}"
        );
        let rationale = "The user asks for a weather forecast, so the forecast endpoint is the direct API that satisfies the request. The current-conditions endpoint is useful for present weather, but it does not answer a future forecast request. Selecting only the forecast API keeps the tool plan scoped and avoids unnecessary side effects.";
        let content = serde_json::json!({
            "selected_apis": ["WeatherAPI::Forecast"],
            "rationale": rationale
        })
        .to_string();
        let body = serde_json::json!({
            "model": "mock-toolbench-agent",
            "choices": [
                {"message": {"content": content}}
            ],
            "usage": {"prompt_tokens": 53, "completion_tokens": 47, "total_tokens": 100}
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
fn toolbench_runner_calls_proxy_records_tool_calls_and_replays_worktx() {
    let tmp = TempDir::new().expect("tempdir");
    let run_dir = tmp.path().join("toolbench");
    let proxy_url = start_mock_llm_proxy();
    let sample = tmp.path().join("toolbench_sample.json");
    std::fs::write(
        &sample,
        r#"{
          "schema_version": "turingosv4.true_suite.toolbench_sample.v1",
          "query_id": "mock-toolbench-weather-001",
          "source_family": "ToolBench/ToolLLM",
          "public_source": "https://huggingface.co/datasets/tuandunghcmut/toolbench-v1",
          "source_split": "benchmark/mock",
          "query": "I am planning a picnic tomorrow. Which API should I use to fetch the weather forecast for a city?",
          "api_list": [
            {
              "category_name": "Weather",
              "tool_name": "WeatherAPI",
              "api_name": "CurrentConditions",
              "api_description": "Fetches the current weather for a city.",
              "required_parameters": [{"name":"city","type":"STRING"}],
              "method": "GET"
            },
            {
              "category_name": "Weather",
              "tool_name": "WeatherAPI",
              "api_name": "Forecast",
              "api_description": "Fetches the forecast for a city and date range.",
              "required_parameters": [{"name":"city","type":"STRING"},{"name":"date","type":"STRING"}],
              "method": "GET"
            }
          ],
          "relevant_apis": [["WeatherAPI", "Forecast"]]
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
    std::fs::create_dir_all(run_dir.join("tool_capsules")).expect("tool capsule dir");
    std::fs::copy(
        &sample,
        run_dir.join("tool_capsules").join("toolbench_sample.json"),
    )
    .expect("copy sample evidence");

    let helper = Command::new(bin("toolbench_api_tool_use_current_kernel"))
        .args([
            "--runtime-repo",
            run_dir.join("runtime_repo").to_str().expect("utf8 path"),
            "--cas",
            run_dir.join("cas").to_str().expect("utf8 path"),
            "--run-id",
            "constitution-true-suite-toolbench",
            "--constitution",
            "constitution.md",
            "--sample-json",
            sample.to_str().expect("utf8 path"),
            "--llm-proxy-url",
            &proxy_url,
            "--model",
            "mock-toolbench-agent",
            "--out-dir",
            run_dir.to_str().expect("utf8 path"),
        ])
        .output()
        .expect("run toolbench helper");
    assert!(
        helper.status.success(),
        "toolbench helper failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&helper.stdout),
        String::from_utf8_lossy(&helper.stderr)
    );
    full_system::run_full_system_augment(
        &run_dir,
        "constitution-true-suite-toolbench",
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
            "constitution-true-suite-toolbench",
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
        "constitution-true-suite-toolbench",
        "toolbench_api_tool_use",
        "tests/constitution_true_suite_toolbench_runner.rs",
        "toolbench_api_tool_use_manifest.json",
        &replay_report,
        bin("full_system_participation_current_kernel"),
    );

    let manifest = read_json(&run_dir.join("toolbench_api_tool_use_manifest.json"));
    assert_eq!(
        manifest.get("schema_version").and_then(Value::as_str),
        Some("turingosv4.true_suite.toolbench_api_tool_use.v1")
    );
    assert_eq!(
        manifest
            .get("selected_apis_available")
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
        fc_trace.get("tool_call_count").and_then(Value::as_u64),
        Some(1)
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
        "input_capsule_cid",
        "answer_claim_capsule_cid",
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
    assert_eq!(telemetry.tool_calls[0].tool_id, "WeatherAPI::Forecast");

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
fn toolbench_runner_script_uses_public_dataset_and_preserves_external_boundary() {
    let script = std::fs::read_to_string("scripts/run_true_suite_toolbench_current_kernel.sh")
        .expect("read ToolBench runner script");
    assert!(script.contains("tuandunghcmut/toolbench-v1"));
    assert!(script.contains("benchmark/g1_instruction"));
    assert!(script.contains("TOOLBENCH_SAMPLE_JSON"));
    assert!(script.contains("LLM_PROXY_URL"));
    assert!(script.contains("toolbench_api_tool_use_current_kernel"));
    assert!(script.contains("full_system_augment_current_kernel"));
    assert!(script.contains("full_system_participation_current_kernel"));
    assert!(script.contains("turingos verify chaintape"));
    assert!(script.contains("--require-full-system"));
    assert!(script.contains("governance_capsule_index.json"));
    assert!(script.contains("full_system_augmentation_manifest.json"));
    assert!(script.contains("raw provider prompt and response are not written"));
    assert!(
        script.contains("parquet_path.unlink"),
        "downloaded parquet must not become committed evidence"
    );
    assert!(
        !script.contains("stage_phase7_real_e2e")
            && !script.contains("real8x_market_ab_clean")
            && !script.contains("old_15_question"),
        "ToolBench runner must not inherit historical product evidence as final input"
    );
}
