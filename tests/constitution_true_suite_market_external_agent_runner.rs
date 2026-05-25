//! True-suite market/economy external-agent runner contract.
//!
//! The integration test uses a local mock OpenAI-compatible proxy so CI does
//! not spend provider tokens. The production runner script uses the same
//! helper against a real DeepSeek/SiliconFlow-backed proxy.

use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::Path;
use std::process::Command;
use std::thread;

use serde_json::Value;
use tempfile::TempDir;

fn bin(name: &str) -> &'static str {
    match name {
        "turingos" => env!("CARGO_BIN_EXE_turingos"),
        "verify_chaintape" => env!("CARGO_BIN_EXE_verify_chaintape"),
        "market_external_agent_current_kernel" => {
            env!("CARGO_BIN_EXE_market_external_agent_current_kernel")
        }
        _ => panic!("unknown bin {name}"),
    }
}

fn bin_dir(path: &str) -> &Path {
    Path::new(path).parent().expect("bin has parent")
}

fn start_mock_llm_proxy() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind mock proxy");
    let addr = listener.local_addr().expect("local addr");
    thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("accept request");
        let mut buf = [0u8; 8192];
        let n = stream.read(&mut buf).expect("read request");
        let request = String::from_utf8_lossy(&buf[..n]);
        assert!(
            request.starts_with("POST /v1/chat/completions"),
            "unexpected mock proxy request: {request}"
        );
        assert!(
            request.contains("true-suite-market-constitution-true-suite-market-agent"),
            "prompt should name the market event without using kernel fixtures: {request}"
        );
        let body = r#"{
          "model": "mock-market-agent",
          "choices": [
            {"message": {"content": "{\"direction\":\"yes\",\"amount_micro\":1000}"}}
          ],
          "usage": {"prompt_tokens": 11, "completion_tokens": 7, "total_tokens": 18}
        }"#;
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
fn market_external_agent_runner_calls_proxy_and_replays_signed_router_tx() {
    let tmp = TempDir::new().expect("tempdir");
    let run_dir = tmp.path().join("market_action");
    let proxy_url = start_mock_llm_proxy();

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

    let manifest_path = run_dir.join("external_agent_market_manifest.json");
    let helper = Command::new(bin("market_external_agent_current_kernel"))
        .args([
            "--runtime-repo",
            run_dir.join("runtime_repo").to_str().expect("utf8 path"),
            "--cas",
            run_dir.join("cas").to_str().expect("utf8 path"),
            "--run-id",
            "constitution-true-suite-market-agent",
            "--constitution",
            "constitution.md",
            "--llm-proxy-url",
            &proxy_url,
            "--model",
            "mock-market-agent",
            "--out",
            manifest_path.to_str().expect("utf8 path"),
        ])
        .output()
        .expect("run market helper");
    assert!(
        helper.status.success(),
        "market helper failed\nstdout:\n{}\nstderr:\n{}",
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
            "constitution-true-suite-market-agent",
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

    let manifest: Value = serde_json::from_str(
        &std::fs::read_to_string(&manifest_path).expect("read market manifest"),
    )
    .expect("parse market manifest");
    assert_eq!(
        manifest.get("schema_version").and_then(Value::as_str),
        Some("turingosv4.true_suite.market_external_agent.v1")
    );
    assert_eq!(
        manifest.get("direction").and_then(Value::as_str),
        Some("yes")
    );
    assert_eq!(
        manifest.get("amount_micro").and_then(Value::as_i64),
        Some(1000)
    );
    assert_eq!(
        manifest.get("router_landed").and_then(Value::as_bool),
        Some(true)
    );
    assert_eq!(
        manifest.get("pool_active").and_then(Value::as_bool),
        Some(true)
    );
    assert_eq!(
        manifest
            .get("agent_response_sha256")
            .and_then(Value::as_str)
            .map(str::len),
        Some(64),
        "raw LLM response should not be persisted; only its hash is kept"
    );
    assert!(
        manifest.get("raw_response").is_none(),
        "runner evidence must not store raw provider response"
    );

    let replay: Value = serde_json::from_str(
        &std::fs::read_to_string(&replay_report).expect("read replay_report.json"),
    )
    .expect("parse replay report");
    assert_eq!(
        replay.get("l4_entries").and_then(Value::as_u64),
        Some(6),
        "boot emits PredicateBindingActivate+MapReduceTick; market helper adds TaskOpen+MarketSeed+CpmmPool+BuyWithCoinRouter"
    );
    for key in [
        "ledger_root_verified",
        "system_signatures_verified",
        "state_reconstructed",
        "economic_state_reconstructed",
        "cas_payloads_retrievable",
        "agent_signatures_verified",
        "proposal_telemetry_cas_retrievable",
    ] {
        assert_eq!(
            replay.get(key).and_then(Value::as_bool),
            Some(true),
            "replay indicator `{key}` must pass: {replay}"
        );
    }

    let genesis_report = run_dir.join("runtime_repo").join("genesis_report.json");
    let genesis: Value = serde_json::from_str(
        &std::fs::read_to_string(genesis_report).expect("read genesis report"),
    )
    .expect("parse genesis report");
    assert!(
        genesis
            .get("initial_balances")
            .and_then(Value::as_array)
            .map(|v| v.len() >= 13)
            .unwrap_or(false),
        "market evidence must record the boot preseed that funds provider/trader wallets"
    );
}

#[test]
fn market_external_agent_runner_script_uses_provider_proxy_not_kernel_fixtures() {
    let script = std::fs::read_to_string("scripts/run_true_suite_market_external_agent.sh")
        .expect("read market runner script");
    assert!(script.contains("LLM_PROXY_URL"));
    assert!(script.contains("market_external_agent_current_kernel"));
    assert!(script.contains("verify chaintape"));
    assert!(script.contains("handover/evidence/true_suite"));
    assert!(
        script.contains("src/drivers/llm_proxy.py"),
        "script should tell operators to use the external provider proxy"
    );
    for forbidden in [
        "TURINGOS_REAL7_SCRIPTED_TASK_OUTCOME_BUYS",
        "TURINGOS_REAL7_SCRIPTED_ATTEMPT_PREDICTION_FIXTURE",
        "TURINGOS_REAL6B_LIVE_ATTEMPT_PREDICTION",
    ] {
        assert!(
            !script.contains(forbidden),
            "true-suite market runner must not inherit old scripted REAL fixtures: {forbidden}"
        );
    }
}
