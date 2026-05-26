//! True-suite SWE-bench coding-repair runner contract.
//!
//! CI uses a local mock OpenAI-compatible proxy so it does not spend provider
//! tokens. The production runner script uses the same helper against a real
//! DeepSeek/SiliconFlow-backed proxy and public SWE-bench Lite input.

use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::Path;
use std::process::Command;
use std::thread;

use serde_json::Value;
use tempfile::TempDir;
use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::bottom_white::cas::store::CasStore;

fn bin(name: &str) -> &'static str {
    match name {
        "turingos" => env!("CARGO_BIN_EXE_turingos"),
        "verify_chaintape" => env!("CARGO_BIN_EXE_verify_chaintape"),
        "swebench_live_coding_repair_current_kernel" => {
            env!("CARGO_BIN_EXE_swebench_live_coding_repair_current_kernel")
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
            request.contains("Issue capsule cid:")
                && request.contains("Repository: astropy/astropy")
                && request.contains("astropy/modeling/tests/test_separable.py::test_separable"),
            "prompt should bind the CAS issue capsule and public SWE-bench test metadata: {request}"
        );
        assert!(
            !request.contains("cright[-right.shape[0]:, -right.shape[1]:] = right")
                && !request.contains("gold_patch")
                && !request.contains("test_patch"),
            "prompt must not leak hidden SWE-bench gold/test patch: {request}"
        );
        let rationale = "The issue is isolated to the separability stacking helper. When the right side is already a coordinate matrix, filling the lower-right block with ones discards the actual separability information computed earlier. The repair preserves that matrix by assigning the right-hand matrix into the lower-right block. This keeps independent right-hand coordinates independent while still allowing coupled coordinates to remain coupled. The change is intentionally scoped to the stacking helper and leaves unrelated arithmetic and coordinate helpers untouched.";
        let patch = "diff --git a/astropy/modeling/separable.py b/astropy/modeling/separable.py\n--- a/astropy/modeling/separable.py\n+++ b/astropy/modeling/separable.py\n@@ -242,7 +242,7 @@ def _cstack(left, right):\n         cright = _coord_matrix(right, 'right', noutp)\n     else:\n         cright = np.zeros((noutp, right.shape[1]))\n-        cright[-right.shape[0]:, -right.shape[1]:] = 1\n+        cright[-right.shape[0]:, -right.shape[1]:] = right\n \n     return np.hstack([cleft, cright])\n";
        let content = serde_json::json!({
            "target_files": ["astropy/modeling/separable.py"],
            "patch": patch,
            "rationale": rationale
        })
        .to_string();
        let body = serde_json::json!({
            "model": "mock-swebench-agent",
            "choices": [
                {"message": {"content": content}}
            ],
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
    format!("http://{addr}")
}

#[test]
fn swebench_runner_calls_proxy_writes_cas_patch_and_replays_worktx() {
    let tmp = TempDir::new().expect("tempdir");
    let run_dir = tmp.path().join("swebench");
    let proxy_url = start_mock_llm_proxy();
    let sample = tmp.path().join("swebench_sample.json");
    std::fs::write(
        &sample,
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
          "gold_patch": "diff --git a/astropy/modeling/separable.py b/astropy/modeling/separable.py\n--- a/astropy/modeling/separable.py\n+++ b/astropy/modeling/separable.py\n@@ -242,7 +242,7 @@ def _cstack(left, right):\n         cright = _coord_matrix(right, 'right', noutp)\n     else:\n         cright = np.zeros((noutp, right.shape[1]))\n-        cright[-right.shape[0]:, -right.shape[1]:] = 1\n+        cright[-right.shape[0]:, -right.shape[1]:] = right\n \n     return np.hstack([cleft, cright])\n",
          "test_patch": "diff --git a/astropy/modeling/tests/test_separable.py b/astropy/modeling/tests/test_separable.py\n--- a/astropy/modeling/tests/test_separable.py\n+++ b/astropy/modeling/tests/test_separable.py\n@@ -28,6 +28,7 @@\n+def test_nested_compound_models():\n+    assert True\n",
          "fail_to_pass": ["astropy/modeling/tests/test_separable.py::test_separable[compound_model6-result6]"],
          "pass_to_pass": ["astropy/modeling/tests/test_separable.py::test_cstack"],
          "created_at": "2022-03-03T15:14:54Z",
          "version": "4.3",
          "environment_setup_commit": "298ccb478e6bf092953bca67a3d29dc6c35f6752"
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
    std::fs::create_dir_all(run_dir.join("repo_snapshots")).expect("snapshot dir");
    std::fs::copy(
        &sample,
        run_dir.join("repo_snapshots").join("swebench_sample.json"),
    )
    .expect("copy sample evidence");

    let helper = Command::new(bin("swebench_live_coding_repair_current_kernel"))
        .args([
            "--runtime-repo",
            run_dir.join("runtime_repo").to_str().expect("utf8 path"),
            "--cas",
            run_dir.join("cas").to_str().expect("utf8 path"),
            "--run-id",
            "constitution-true-suite-swebench",
            "--constitution",
            "constitution.md",
            "--sample-json",
            sample.to_str().expect("utf8 path"),
            "--llm-proxy-url",
            &proxy_url,
            "--model",
            "mock-swebench-agent",
            "--out-dir",
            run_dir.to_str().expect("utf8 path"),
        ])
        .output()
        .expect("run swebench helper");
    assert!(
        helper.status.success(),
        "swebench helper failed\nstdout:\n{}\nstderr:\n{}",
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
            "constitution-true-suite-swebench",
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

    let manifest = read_json(&run_dir.join("swebench_live_coding_repair_manifest.json"));
    assert_eq!(
        manifest.get("schema_version").and_then(Value::as_str),
        Some("turingosv4.true_suite.swebench_live_coding_repair.v1")
    );
    assert_eq!(
        manifest
            .get("patch_structurally_plausible")
            .and_then(Value::as_bool),
        Some(true)
    );
    assert_eq!(
        manifest.get("target_file_overlap").and_then(Value::as_bool),
        Some(true)
    );
    assert_eq!(
        manifest.get("has_unified_diff").and_then(Value::as_bool),
        Some(true)
    );
    assert_eq!(
        manifest
            .get("rationale_guard_passed")
            .and_then(Value::as_bool),
        Some(true)
    );
    assert_eq!(
        manifest.get("work_tx_landed").and_then(Value::as_bool),
        Some(true)
    );
    assert!(manifest.get("raw_response").is_none());
    assert!(manifest.get("raw_prompt").is_none());

    for key in [
        "issue_capsule_cid",
        "patch_claim_capsule_cid",
        "evaluation_capsule_cid",
        "proposal_telemetry_cid",
    ] {
        let cid = cid_from_hex(manifest.get(key).and_then(Value::as_str).expect(key));
        let cas = CasStore::open(&run_dir.join("cas")).expect("open cas");
        assert!(!cas.get(&cid).expect("read cas object").is_empty(), "{key}");
    }

    let evaluation_cid = cid_from_hex(
        manifest
            .get("evaluation_capsule_cid")
            .and_then(Value::as_str)
            .expect("evaluation cid"),
    );
    let cas = CasStore::open(&run_dir.join("cas")).expect("open cas");
    let evaluation = cas.get(&evaluation_cid).expect("cas get");
    let evaluation_json: Value =
        serde_json::from_slice(&evaluation).expect("evaluation json parse");
    assert_eq!(
        evaluation_json
            .get("benchmark_verdict")
            .and_then(Value::as_str),
        Some("repair_patch_structurally_plausible")
    );

    let replay = read_json(&replay_report);
    assert_eq!(
        replay.get("ledger_root_verified").and_then(Value::as_bool),
        Some(true)
    );
    assert_eq!(
        replay
            .get("system_signatures_verified")
            .and_then(Value::as_bool),
        Some(true)
    );
    assert_eq!(
        replay
            .get("agent_signatures_verified")
            .and_then(Value::as_bool),
        Some(true)
    );
    assert_eq!(
        replay.get("state_reconstructed").and_then(Value::as_bool),
        Some(true)
    );
}

#[test]
fn swebench_runner_script_uses_public_dataset_proxy_and_no_raw_provider_evidence() {
    let script = std::fs::read_to_string("scripts/run_true_suite_swebench_current_kernel.sh")
        .expect("read swebench runner script");
    assert!(script.contains("princeton-nlp/SWE-bench_Lite"));
    assert!(script.contains("datasets-server.huggingface.co/rows"));
    assert!(script.contains("\"config\": config"));
    assert!(script.contains("\"gold_patch\": str(row[\"patch\"])"));
    assert!(script.contains("\"test_patch\": str(row[\"test_patch\"])"));
    assert!(
        script.contains("LLM_PROXY_URL") && script.contains("/health"),
        "runner must use an external local proxy boundary"
    );
    assert!(
        !script.contains("stage_phase7_real_e2e") && !script.contains("old_15_question"),
        "SWE-bench runner must not inherit old-15 or historical product evidence as final input"
    );
    assert!(
        !script.contains("raw_response") && !script.contains("raw_prompt"),
        "runner script must not persist raw provider prompt/response artifacts"
    );

    let helper = std::fs::read_to_string("src/bin/swebench_live_coding_repair_current_kernel.rs")
        .expect("read helper");
    assert!(helper.contains("gold patch and test patch remain benchmark-side evaluation data"));
    assert!(
        helper.contains("Do not include or quote any hidden benchmark patch, test patch, or hints")
    );
    assert!(helper.contains("raw_provider_response_persisted: false"));
    assert!(!helper.contains("Command::new(\"git\")"));
    assert!(!helper.contains("Command::new(\"docker\")"));
}
