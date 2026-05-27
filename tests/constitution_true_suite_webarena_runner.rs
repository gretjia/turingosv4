//! True-suite WebArena web-agent runner contract.
//!
//! CI uses a local mock OpenAI-compatible proxy so it does not spend provider
//! tokens. The production runner script uses the same helper against a real
//! DeepSeek/SiliconFlow-backed proxy and public WebArena task configuration.

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
        "webarena_web_agent_current_kernel" => {
            env!("CARGO_BIN_EXE_webarena_web_agent_current_kernel")
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
        let mut buf = [0u8; 65536];
        let n = stream.read(&mut buf).expect("read request");
        let request = String::from_utf8_lossy(&buf[..n]);
        assert!(
            request.starts_with("POST /v1/chat/completions"),
            "unexpected mock proxy request: {request}"
        );
        assert!(
            request.contains("Task capsule cid:")
                && request.contains("Observation capsule cid:")
                && request.contains("Observation blob cid:")
                && request.contains("Start URL: __SHOPPING_ADMIN__"),
            "prompt should bind task, observation, and start URL: {request}"
        );
        assert!(
            request.contains("Quest Lumaflex"),
            "visible observation should be available to the model: {request}"
        );
        assert!(
            !request.contains("reference_answer")
                && !request.contains("reference_answer_sha256")
                && !request.contains("reference_answer_raw_annotation"),
            "prompt must not leak hidden WebArena evaluation fields: {request}"
        );
        let rationale = "The visible offline observation lists the shopping admin top seller table for the requested year. I only use the provided snapshot rather than live account access, then read the first-ranked row and preserve the browser action as an observation-based answer trace.";
        let content = serde_json::json!({
            "final_answer": "Quest Lumaflex Band",
            "browser_action": "read visible shopping_admin top-sellers table row 1",
            "rationale": rationale
        })
        .to_string();
        let body = serde_json::json!({
            "model": "mock-webarena-agent",
            "choices": [
                {"message": {"content": content}}
            ],
            "usage": {"prompt_tokens": 91, "completion_tokens": 44, "total_tokens": 135}
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

fn write_mock_sample(sample: &Path) {
    std::fs::write(
        sample,
        r#"{
          "schema_version": "turingosv4.true_suite.webarena_sample.v1",
          "sample_id": "mock-webarena-shopping-admin-000",
          "source_family": "WebArena",
          "public_source": "https://github.com/web-arena-x/webarena/blob/main/config_files/test.raw.json",
          "source_file": "config_files/test.raw.json",
          "task_id": "0",
          "intent": "What is the top-1 best-selling product in 2022",
          "start_url": "__SHOPPING_ADMIN__",
          "sites": ["shopping_admin"],
          "allowed_tools": ["browser_sandbox"],
          "observation_html": "<html><body><h1>Sales dashboard</h1><table><tr><th>Rank</th><th>Product</th><th>Year</th></tr><tr><td>1</td><td>Quest Lumaflex Band</td><td>2022</td></tr><tr><td>2</td><td>Sprite Stasis Ball</td><td>2022</td></tr></table></body></html>",
          "reference_answer": "Quest Lumaflex Band"
        }"#,
    )
    .expect("write sample");
}

fn init_webarena_workspace(run_dir: &Path, sample: &Path) {
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
        run_dir.join("input_capsules").join("webarena_sample.json"),
    )
    .expect("copy sample evidence");
}

#[test]
fn webarena_runner_calls_proxy_records_browser_action_and_replays_worktx() {
    let tmp = TempDir::new().expect("tempdir");
    let run_dir = tmp.path().join("webarena");
    let proxy_url = start_mock_llm_proxy();
    let sample = tmp.path().join("webarena_sample.json");
    write_mock_sample(&sample);

    init_webarena_workspace(&run_dir, &sample);

    let helper = Command::new(bin("webarena_web_agent_current_kernel"))
        .args([
            "--runtime-repo",
            run_dir.join("runtime_repo").to_str().expect("utf8 path"),
            "--cas",
            run_dir.join("cas").to_str().expect("utf8 path"),
            "--run-id",
            "constitution-true-suite-webarena",
            "--constitution",
            "constitution.md",
            "--sample-json",
            sample.to_str().expect("utf8 path"),
            "--llm-proxy-url",
            &proxy_url,
            "--model",
            "mock-webarena-agent",
            "--out-dir",
            run_dir.to_str().expect("utf8 path"),
        ])
        .output()
        .expect("run webarena helper");
    assert!(
        helper.status.success(),
        "webarena helper failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&helper.stdout),
        String::from_utf8_lossy(&helper.stderr)
    );
    full_system::run_full_system_augment(
        &run_dir,
        "constitution-true-suite-webarena",
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
            "constitution-true-suite-webarena",
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
        "constitution-true-suite-webarena",
        "webarena_web_agent",
        "tests/constitution_true_suite_webarena_runner.rs",
        "webarena_web_agent_manifest.json",
        &replay_report,
        bin("full_system_participation_current_kernel"),
    );

    let manifest = read_json(&run_dir.join("webarena_web_agent_manifest.json"));
    assert_eq!(
        manifest.get("schema_version").and_then(Value::as_str),
        Some("turingosv4.true_suite.webarena_web_agent.v1")
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

    let fc_trace = read_json(&run_dir.join("fc_trace_report.json"));
    assert_eq!(
        fc_trace.get("family_id").and_then(Value::as_str),
        Some("webarena_web_agent")
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

    let mut cas = CasStore::open(&run_dir.join("cas")).expect("open CAS");
    for key in [
        "observation_capsule_cid",
        "task_capsule_cid",
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
    assert_eq!(
        telemetry.tool_calls[0].tool_id,
        "browser_sandbox::answer_from_visible_observation"
    );

    let taxonomy = read_json(&run_dir.join("failure_taxonomy.json"));
    assert_eq!(
        taxonomy.get("model_task_failure").and_then(Value::as_bool),
        Some(false)
    );
    assert_eq!(
        taxonomy.get("browser_state_drift").and_then(Value::as_bool),
        Some(false)
    );

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
fn webarena_runner_script_uses_official_config_proxy_and_no_live_side_effects() {
    let script = std::fs::read_to_string("scripts/run_true_suite_webarena_current_kernel.sh")
        .expect("read WebArena runner script");
    assert!(script.contains("web-arena-x/webarena"));
    assert!(script.contains("config_files/test.raw.json"));
    assert!(script.contains("WEBARENA_SAMPLE_JSON"));
    assert!(script.contains("WEBARENA_OBSERVATION_HTML"));
    assert!(script.contains("LLM_PROXY_URL"));
    assert!(script.contains("webarena_web_agent_current_kernel"));
    assert!(script.contains("full_system_augment_current_kernel"));
    assert!(script.contains("full_system_participation_current_kernel"));
    assert!(script.contains("turingos verify chaintape"));
    assert!(script.contains("--require-full-system"));
    assert!(script.contains("governance_capsule_index.json"));
    assert!(script.contains("full_system_augmentation_manifest.json"));
    assert!(script.contains("raw provider prompt and response are not written"));
    assert!(script.contains("no live website or account side effects"));
    assert!(
        !script.contains("stage_phase7_real_e2e")
            && !script.contains("real8x_market_ab_clean")
            && !script.contains("old_15_question"),
        "WebArena runner must not inherit historical product evidence as final input"
    );
}
