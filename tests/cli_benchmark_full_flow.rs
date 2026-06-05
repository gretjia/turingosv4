//! Constitutional Full-Flow Benchmark CLI contract.
//!
//! This test intentionally drives the public `turingos` binary. Helper
//! binaries may be resolved by the CLI, but the operator entrypoint must be
//! `turingos benchmark full-flow run`.

use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

use serde_json::Value;
use tempfile::TempDir;

fn bin(name: &str) -> &'static str {
    match name {
        "turingos" => env!("CARGO_BIN_EXE_turingos"),
        "verify_chaintape" => env!("CARGO_BIN_EXE_verify_chaintape"),
        "swebench_live_coding_repair_current_kernel" => {
            env!("CARGO_BIN_EXE_swebench_live_coding_repair_current_kernel")
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

fn start_mock_llm_proxy() -> (String, thread::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind mock proxy");
    listener
        .set_nonblocking(true)
        .expect("set mock proxy nonblocking");
    let addr = listener.local_addr().expect("local addr");
    let handle = thread::spawn(move || {
        let deadline = Instant::now() + Duration::from_secs(180);
        let (mut stream, _) = loop {
            match listener.accept() {
                Ok(pair) => break pair,
                Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                    assert!(Instant::now() < deadline, "mock proxy received no request");
                    thread::sleep(Duration::from_millis(20));
                }
                Err(err) => panic!("accept request: {err}"),
            }
        };
        stream
            .set_read_timeout(Some(Duration::from_secs(3)))
            .expect("set mock proxy read timeout");
        stream
            .set_write_timeout(Some(Duration::from_secs(3)))
            .expect("set mock proxy write timeout");
        let request = read_http_request(&mut stream, deadline);
        assert!(
            request.starts_with("POST /v1/chat/completions"),
            "unexpected mock proxy request: {request}"
        );
        assert!(
            request.contains("Issue capsule cid:")
                && request.contains("Repository: astropy/astropy")
                && request.contains("astropy/modeling/tests/test_separable.py::test_separable"),
            "prompt should bind public SWE-bench issue metadata: {request}"
        );
        assert!(
            !request.contains("gold_patch") && !request.contains("test_patch"),
            "prompt must not leak hidden SWE-bench gold/test patch: {request}"
        );
        let content = serde_json::json!({
            "target_files": ["astropy/modeling/separable.py"],
            "patch": "diff --git a/astropy/modeling/separable.py b/astropy/modeling/separable.py\n--- a/astropy/modeling/separable.py\n+++ b/astropy/modeling/separable.py\n@@ -242,7 +242,7 @@ def _cstack(left, right):\n         cright = _coord_matrix(right, 'right', noutp)\n     else:\n         cright = np.zeros((noutp, right.shape[1]))\n-        cright[-right.shape[0]:, -right.shape[1]:] = 1\n+        cright[-right.shape[0]:, -right.shape[1]:] = right\n \n     return np.hstack([cleft, cright])\n",
            "rationale": "The repair preserves the right-hand separability matrix instead of replacing it with ones."
        })
        .to_string();
        let body = serde_json::json!({
            "model": "mock-swebench-agent",
            "choices": [{"message": {"content": content}}],
            "usage": {"prompt_tokens": 71, "completion_tokens": 81, "total_tokens": 152}
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
    (format!("http://{addr}"), handle)
}

fn read_http_request(stream: &mut std::net::TcpStream, deadline: Instant) -> String {
    let mut bytes = Vec::new();
    let mut buf = [0u8; 8192];
    loop {
        match stream.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                bytes.extend_from_slice(&buf[..n]);
                if bytes.windows(4).any(|w| w == b"\r\n\r\n") {
                    break;
                }
            }
            Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                assert!(
                    Instant::now() < deadline,
                    "mock proxy request read timed out"
                );
                thread::sleep(Duration::from_millis(10));
            }
            Err(err) => panic!("read request: {err}"),
        }
    }
    String::from_utf8_lossy(&bytes).into_owned()
}

fn write_swebench_sample(path: &Path) {
    std::fs::write(
        path,
        r#"{
          "schema_version": "turingosv4.true_suite.swebench_sample.v1",
          "sample_id": "princeton-nlp/SWE-bench_Lite:test:0",
          "source_family": "SWE-bench_Lite",
          "public_source": "https://huggingface.co/datasets/princeton-nlp/SWE-bench_Lite",
          "source_file": "datasets-server:default/test:0",
          "repo": "astropy/astropy",
          "instance_id": "astropy__astropy-12907",
          "base_commit": "d16bfe05a744909de4b27f5875fe0d4ed41ce607",
          "problem_statement": "Modeling's separability_matrix does not compute separability correctly for nested CompoundModels.",
          "hints_text": "private-benchmark-hint-not-for-prompt",
          "gold_patch": "hidden-gold-patch",
          "test_patch": "hidden-test-patch",
          "fail_to_pass": ["astropy/modeling/tests/test_separable.py::test_separable[compound_model6-result6]"],
          "pass_to_pass": ["astropy/modeling/tests/test_separable.py::test_cstack"],
          "created_at": "2022-03-03T15:14:54Z",
          "version": "4.3",
          "environment_setup_commit": "298ccb478e6bf092953bca67a3d29dc6c35f6752"
        }"#,
    )
    .expect("write sample");
}

fn write_tdma_task_evidence(evidence_dir: &Path, stages_completed: u64, stages_total: u64) {
    std::fs::create_dir_all(evidence_dir).expect("create tdma evidence dir");
    let manifest = serde_json::json!({
        "run_id": "turingos-tdma-swebench",
        "problem_label": "turingos tdma --judge swebench pallets__flask-5063",
        "stages_completed": stages_completed,
        "stages_total": stages_total,
        "total_attempts": 3,
        "total_failed_attempts": if stages_completed == stages_total { 0 } else { 3 },
        "distinct_judge_classes": if stages_completed == stages_total {
            serde_json::json!([])
        } else {
            serde_json::json!(["hidden_test_failure"])
        },
        "leak_in_any_prompt": false,
        "total_wall_clock_ms": 123456,
        "probes_sha256": "probe-sha",
        "chaintape_sha256": "tdma-tape-sha"
    });
    std::fs::write(
        evidence_dir.join("manifest.json"),
        serde_json::to_string_pretty(&manifest).expect("manifest json"),
    )
    .expect("write tdma manifest");
    std::fs::write(evidence_dir.join("per_attempt_probes.jsonl"), "{}\n").expect("write probes");
}

#[test]
fn benchmark_full_flow_help_declares_verdict_boundaries() {
    let help = Command::new(bin("turingos"))
        .args(["benchmark", "full-flow", "--help"])
        .output()
        .expect("run turingos benchmark full-flow help");
    assert!(
        help.status.success(),
        "help failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&help.stdout),
        String::from_utf8_lossy(&help.stderr)
    );
    let stdout = String::from_utf8_lossy(&help.stdout);
    assert!(stdout.contains("turingos benchmark full-flow run"));
    assert!(stdout.contains("FC1") && stdout.contains("FC2") && stdout.contains("FC3"));
    assert!(stdout.contains("market"));
    assert!(stdout.contains("FLOW-PASS"));
    assert!(stdout.contains("SYSTEM-PASS"));
    assert!(stdout.contains("TASK-PASS"));
}

#[test]
fn benchmark_full_flow_cli_writes_packet_without_promoting_smoke_to_task_pass() {
    let tmp = TempDir::new().expect("tempdir");
    let run_dir = tmp.path().join("full_flow");
    let sample = tmp.path().join("swebench_sample.json");
    write_swebench_sample(&sample);
    let (proxy_url, proxy_thread) = start_mock_llm_proxy();

    let out = Command::new(bin("turingos"))
        .env("TURINGOS_BIN_DIR", bin_dir(bin("turingos")))
        .args([
            "benchmark",
            "full-flow",
            "run",
            "--run-dir",
            run_dir.to_str().expect("utf8 path"),
            "--run-id",
            "cli-full-flow-smoke",
            "--constitution",
            "constitution.md",
            "--sample-json",
            sample.to_str().expect("utf8 path"),
            "--llm-proxy-url",
            &proxy_url,
            "--model",
            "mock-swebench-agent",
        ])
        .output()
        .expect("run turingos benchmark full-flow");
    let proxy_join = proxy_thread.join();
    let proxy_join_ok = proxy_join.is_ok();
    assert!(
        out.status.success(),
        "benchmark failed\nproxy_join_ok={}\nstdout:\n{}\nstderr:\n{}",
        proxy_join_ok,
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    proxy_join.expect("mock proxy thread completed");

    let packet = read_json(&run_dir.join("constitutional_full_flow_benchmark_packet.json"));
    assert_eq!(
        packet["schema_version"],
        "turingosv4.benchmark.constitutional_full_flow.v1"
    );
    assert_eq!(packet["entrypoint"], "turingos benchmark full-flow run");
    assert_eq!(packet["flow_verdict"], "FLOW-PASS");
    assert_eq!(packet["system_verdict"], "SYSTEM-PASS");
    assert_ne!(
        packet["task_verdict"]["kind"], "TASK-PASS",
        "structural SWE-bench smoke must not be promoted to hidden-test TASK-PASS"
    );
    assert_eq!(
        packet["task_verdict"]["source"],
        "swebench_current_kernel_structural_smoke"
    );

    let participation = read_json(&run_dir.join("full_system_participation.json"));
    assert_eq!(
        participation["flowchart_node_receipts"]["verdict"],
        "FLOW-PASS"
    );
    let missing = participation["flowchart_node_receipts"]["missing"]
        .as_array()
        .expect("missing array");
    assert!(missing.is_empty(), "missing receipts: {missing:?}");
    let fc1 = participation["flowchart_node_receipts"]["fc1"]
        .as_array()
        .expect("fc1 receipts");
    assert!(
        fc1.iter().any(|r| {
            r.get("node_id").and_then(Value::as_str) == Some("FC1-N15")
                && r.get("status").and_then(Value::as_str) == Some("present")
        }),
        "FC1 reject branch must be lit by an L4.E receipt"
    );
    assert!(
        participation["market"]["present"].as_bool() == Some(true),
        "market must participate in the same full-flow run"
    );
}

#[test]
fn benchmark_full_flow_packet_can_use_real_tdma_task_evidence() {
    let tmp = TempDir::new().expect("tempdir");
    let run_dir = tmp.path().join("full_flow");
    let sample = tmp.path().join("swebench_sample.json");
    let task_evidence = tmp.path().join("tdma_evidence");
    write_swebench_sample(&sample);
    write_tdma_task_evidence(&task_evidence, 1, 1);
    let (proxy_url, proxy_thread) = start_mock_llm_proxy();

    let out = Command::new(bin("turingos"))
        .env("TURINGOS_BIN_DIR", bin_dir(bin("turingos")))
        .args([
            "benchmark",
            "full-flow",
            "run",
            "--run-dir",
            run_dir.to_str().expect("utf8 path"),
            "--run-id",
            "cli-full-flow-with-tdma",
            "--constitution",
            "constitution.md",
            "--sample-json",
            sample.to_str().expect("utf8 path"),
            "--llm-proxy-url",
            &proxy_url,
            "--model",
            "mock-swebench-agent",
            "--task-evidence-dir",
            task_evidence.to_str().expect("utf8 path"),
            "--require-task-pass",
        ])
        .output()
        .expect("run turingos benchmark full-flow with task evidence");
    proxy_thread.join().expect("mock proxy thread completed");
    assert!(
        out.status.success(),
        "benchmark failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let packet = read_json(&run_dir.join("constitutional_full_flow_benchmark_packet.json"));
    assert_eq!(packet["task_verdict"]["kind"], "TASK-PASS");
    assert_eq!(
        packet["task_verdict"]["source"],
        "swebench_tdma_hidden_test_verifier"
    );
    assert_eq!(
        packet["evidence_paths"]["task_evidence_dir"],
        task_evidence.to_str().expect("utf8 path")
    );
}
