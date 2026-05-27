//! True-suite OSWorld computer-use runner contract.
//!
//! CI uses a local mock OpenAI-compatible proxy so it does not spend provider
//! tokens. The production runner script uses the same helper against a real
//! DeepSeek/SiliconFlow-backed proxy and an offline OSWorld-style sandbox
//! snapshot.

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
        "osworld_computer_use_current_kernel" => {
            env!("CARGO_BIN_EXE_osworld_computer_use_current_kernel")
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
                && request.contains("Sandbox snapshot capsule cid:")
                && request.contains("Sandbox snapshot blob cid:")
                && request.contains("Environment: ubuntu_desktop_sandbox"),
            "prompt should bind task, snapshot, and sandbox environment: {request}"
        );
        assert!(
            request.contains("/home/oai/share/notes/draft.txt"),
            "visible sandbox snapshot should be available to the model: {request}"
        );
        assert!(
            !request.contains("expected_action")
                && !request.contains("expected_final_state")
                && !request.contains("final.txt exists and draft.txt is absent"),
            "prompt must not leak hidden OSWorld evaluation fields: {request}"
        );
        let rationale = "The visible offline sandbox snapshot shows the notes directory and the draft.txt file that the task wants renamed. I would only act inside that declared sandbox with network disabled, then verify the file tree reflects the rename rather than claiming any host operating system side effect.";
        let content = serde_json::json!({
            "final_answer": "Renamed draft.txt to final.txt in the sandbox notes directory.",
            "os_action": "mv /home/oai/share/notes/draft.txt /home/oai/share/notes/final.txt",
            "file_state_diff": "- /home/oai/share/notes/draft.txt\n+ /home/oai/share/notes/final.txt",
            "rationale": rationale
        })
        .to_string();
        let body = serde_json::json!({
            "model": "mock-osworld-agent",
            "choices": [
                {"message": {"content": content}}
            ],
            "usage": {"prompt_tokens": 110, "completion_tokens": 52, "total_tokens": 162}
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
          "schema_version": "turingosv4.true_suite.osworld_sample.v1",
          "sample_id": "mock-osworld-rename-note-000",
          "source_family": "OSWorld",
          "public_source": "https://arxiv.org/abs/2404.07972",
          "source_file": "offline_osworld_style_sample.json",
          "task_id": "rename-note-000",
          "instruction": "In the sandboxed desktop, rename the visible draft note to final.txt and report completion.",
          "environment": "ubuntu_desktop_sandbox",
          "network_policy": "offline_no_network",
          "allowed_tools": ["computer_sandbox"],
          "sandbox_snapshot_text": "Visible file tree:\n/home/oai/share/notes/draft.txt\n/home/oai/share/notes/archive/\nNo live host OS operation is allowed.",
          "expected_action": "mv /home/oai/share/notes/draft.txt /home/oai/share/notes/final.txt",
          "expected_final_state": "final.txt exists and draft.txt is absent"
        }"#,
    )
    .expect("write sample");
}

fn init_osworld_workspace(run_dir: &Path, sample: &Path) {
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
        run_dir.join("input_capsules").join("osworld_sample.json"),
    )
    .expect("copy sample evidence");
}

#[test]
fn osworld_runner_calls_proxy_records_sandbox_action_and_replays_worktx() {
    let tmp = TempDir::new().expect("tempdir");
    let run_dir = tmp.path().join("osworld");
    let proxy_url = start_mock_llm_proxy();
    let sample = tmp.path().join("osworld_sample.json");
    write_mock_sample(&sample);

    init_osworld_workspace(&run_dir, &sample);

    let helper = Command::new(bin("osworld_computer_use_current_kernel"))
        .args([
            "--runtime-repo",
            run_dir.join("runtime_repo").to_str().expect("utf8 path"),
            "--cas",
            run_dir.join("cas").to_str().expect("utf8 path"),
            "--run-id",
            "constitution-true-suite-osworld",
            "--constitution",
            "constitution.md",
            "--sample-json",
            sample.to_str().expect("utf8 path"),
            "--llm-proxy-url",
            &proxy_url,
            "--model",
            "mock-osworld-agent",
            "--out-dir",
            run_dir.to_str().expect("utf8 path"),
        ])
        .output()
        .expect("run OSWorld helper");
    assert!(
        helper.status.success(),
        "OSWorld helper failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&helper.stdout),
        String::from_utf8_lossy(&helper.stderr)
    );
    full_system::run_full_system_augment(
        &run_dir,
        "constitution-true-suite-osworld",
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
            "constitution-true-suite-osworld",
            "--out",
            replay_report.to_str().expect("utf8 path"),
        ])
        .output()
        .expect("run turingos verify chaintape");
    assert!(
        verify.status.success(),
        "turingos verify failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&verify.stdout),
        String::from_utf8_lossy(&verify.stderr)
    );

    let manifest = read_json(&run_dir.join("osworld_computer_use_manifest.json"));
    assert_eq!(
        manifest.get("schema_version").and_then(Value::as_str),
        Some("turingosv4.true_suite.osworld_computer_use.v1")
    );
    assert_eq!(
        manifest.get("source_family").and_then(Value::as_str),
        Some("OSWorld")
    );
    assert_eq!(
        manifest.get("work_tx_landed").and_then(Value::as_bool),
        Some(true)
    );
    assert_eq!(
        manifest.get("action_match").and_then(Value::as_bool),
        Some(true)
    );
    assert_eq!(
        manifest
            .get("raw_provider_response_persisted")
            .and_then(Value::as_bool),
        None,
        "top-level manifest should avoid ambiguous raw-provider persistence fields"
    );
    assert_eq!(
        manifest
            .get("final_closure_possible")
            .and_then(Value::as_bool),
        Some(false),
        "OSWorld runner lights one domain only; it cannot close OBL-005 alone"
    );

    let claim = read_json(&run_dir.join("input_capsules").join("answer_claim.json"));
    assert_eq!(
        claim
            .get("raw_provider_response_persisted")
            .and_then(Value::as_bool),
        Some(false)
    );
    assert_eq!(
        claim.get("os_action").and_then(Value::as_str),
        Some("mv /home/oai/share/notes/draft.txt /home/oai/share/notes/final.txt")
    );

    let task = read_json(&run_dir.join("input_capsules").join("task_capsule.json"));
    assert!(
        task.get("expected_action_sha256").is_some(),
        "hidden OSWorld expected action should be hash-only task metadata"
    );
    assert!(
        task.get("expected_final_state_sha256").is_some(),
        "hidden OSWorld final state should be hash-only task metadata"
    );

    let replay = read_json(&replay_report);
    for key in [
        "ledger_root_verified",
        "system_signatures_verified",
        "agent_signatures_verified",
        "state_reconstructed",
        "economic_state_reconstructed",
        "cas_payloads_retrievable",
    ] {
        assert_eq!(
            replay.get(key).and_then(Value::as_bool),
            Some(true),
            "replay report key {key} must be true"
        );
    }

    let fc_trace = read_json(&run_dir.join("fc_trace_report.json"));
    let fc_blocks: Vec<_> = fc_trace
        .get("fc_blocks_seen")
        .and_then(Value::as_array)
        .expect("fc blocks")
        .iter()
        .filter_map(Value::as_str)
        .collect();
    for fc in ["FC1", "FC2", "FC3"] {
        assert!(fc_blocks.contains(&fc), "OSWorld trace missing {fc}");
    }

    let telemetry_cid = manifest
        .get("proposal_telemetry_cid")
        .and_then(Value::as_str)
        .expect("proposal telemetry cid");
    let mut cas = CasStore::open(&run_dir.join("cas")).expect("open cas");
    let telemetry =
        read_proposal_telemetry(&mut cas, &cid_from_hex(telemetry_cid)).expect("read telemetry");
    assert_eq!(telemetry.tool_calls.len(), 1);
    assert_eq!(
        telemetry.tool_calls[0].tool_id,
        "computer_sandbox::action_from_visible_snapshot"
    );

    let participation = full_system::assert_full_system_lit(
        &run_dir,
        "constitution-true-suite-osworld",
        "osworld_computer_use",
        "scripts/run_true_suite_osworld_current_kernel.sh",
        "osworld_computer_use_manifest.json",
        &replay_report,
        bin("full_system_participation_current_kernel"),
    );
    assert_eq!(
        participation
            .get("verdict")
            .and_then(|v| v.get("final_closure_possible"))
            .and_then(Value::as_bool),
        Some(false),
        "OSWorld can light full-system participation for this sample while OBL-005 remains open at suite level"
    );
}

#[test]
fn osworld_runner_script_and_batch_contracts_are_non_closing_and_external() {
    let script = std::fs::read_to_string("scripts/run_true_suite_osworld_current_kernel.sh")
        .expect("read OSWorld script");
    assert!(script.contains("osworld_computer_use_current_kernel"));
    assert!(script.contains("OSWORLD_SAMPLE_JSON"));
    assert!(script.contains("OSWORLD_SNAPSHOT_TEXT"));
    assert!(script.contains("full_system_augment_current_kernel"));
    assert!(script.contains("full_system_participation_current_kernel"));
    assert!(script.contains("turingos verify chaintape"));
    assert!(script.contains("\"final_closure_possible\": false"));
    assert!(script.contains("raw provider prompt and response are not written to evidence"));
    assert!(
        !script.contains("raw_prompt") && !script.contains("raw_response"),
        "OSWorld script must not introduce raw provider transcript artifacts"
    );

    let batch =
        std::fs::read_to_string("scripts/run_true_suite_broad_agi_batch.sh").expect("read batch");
    assert!(batch.contains("osworld_computer_use_fresh"));
    assert!(batch.contains("run_true_suite_osworld_current_kernel.sh"));
    assert!(batch.contains("\"osworld_computer_use\""));
}
