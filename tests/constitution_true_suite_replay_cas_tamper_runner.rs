//! True-suite replay/CAS tamper runner contract.
//!
//! This gate creates a current-kernel ChainTape/CAS run in a temp directory,
//! verifies it via the public `turingos verify chaintape` wrapper, then runs
//! `audit_tape_tamper` over temp forks and verifies the original tape again.

use std::path::Path;
use std::process::Command;

use serde_json::Value;
use sha2::{Digest, Sha256};
use tempfile::TempDir;

fn bin(name: &str) -> &'static str {
    match name {
        "turingos" => env!("CARGO_BIN_EXE_turingos"),
        "verify_chaintape" => env!("CARGO_BIN_EXE_verify_chaintape"),
        "boot_cli_current_kernel_fresh" => env!("CARGO_BIN_EXE_boot_cli_current_kernel_fresh"),
        "audit_tape_tamper" => env!("CARGO_BIN_EXE_audit_tape_tamper"),
        _ => panic!("unknown bin {name}"),
    }
}

fn bin_dir(path: &str) -> &Path {
    Path::new(path).parent().expect("bin has parent")
}

fn read_json(path: &Path) -> Value {
    serde_json::from_str(&std::fs::read_to_string(path).expect("read json")).expect("parse json")
}

fn assert_replay_green(report: &Value, label: &str) {
    assert!(
        report
            .get("l4_entries")
            .and_then(Value::as_u64)
            .unwrap_or(0)
            >= 3,
        "{label}: expected at least 3 L4 entries: {report}"
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
            report.get(key).and_then(Value::as_bool),
            Some(true),
            "{label}: replay indicator `{key}` must pass: {report}"
        );
    }
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

#[test]
fn replay_cas_tamper_runner_verifies_current_kernel_and_detects_tamper() {
    let tmp = TempDir::new().expect("tempdir");
    let run_dir = tmp.path().join("replay_cas");

    let init = Command::new(bin("turingos"))
        .args([
            "init",
            "--project",
            run_dir.to_str().expect("utf8 path"),
            "--template",
            "proof",
            "--provider",
            "siliconflow",
        ])
        .output()
        .expect("run turingos init");
    assert!(
        init.status.success(),
        "turingos init failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&init.stdout),
        String::from_utf8_lossy(&init.stderr)
    );

    let helper = Command::new(bin("boot_cli_current_kernel_fresh"))
        .args([
            "--runtime-repo",
            run_dir.join("runtime_repo").to_str().expect("utf8 path"),
            "--cas",
            run_dir.join("cas").to_str().expect("utf8 path"),
            "--run-id",
            "constitution-true-suite-replay-cas",
            "--constitution",
            "constitution.md",
        ])
        .output()
        .expect("run boot helper");
    assert!(
        helper.status.success(),
        "boot helper failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&helper.stdout),
        String::from_utf8_lossy(&helper.stderr)
    );

    std::fs::write(
        run_dir.join("runtime_repo").join("agent_pubkeys.json"),
        "{\"agents\":{}}\n",
    )
    .expect("write explicit empty agent manifest");

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
            "constitution-true-suite-replay-cas",
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

    let tamper_report = run_dir.join("tamper_report.json");
    let tamper = Command::new(bin("audit_tape_tamper"))
        .args([
            "--runtime-repo",
            run_dir.join("runtime_repo").to_str().expect("utf8 path"),
            "--cas-dir",
            run_dir.join("cas").to_str().expect("utf8 path"),
            "--agent-pubkeys",
            run_dir
                .join("runtime_repo")
                .join("agent_pubkeys.json")
                .to_str()
                .expect("utf8 path"),
            "--pinned-pubkeys",
            run_dir
                .join("runtime_repo")
                .join("pinned_pubkeys.json")
                .to_str()
                .expect("utf8 path"),
            "--genesis",
            run_dir
                .join("runtime_repo")
                .join("genesis_report.json")
                .to_str()
                .expect("utf8 path"),
            "--constitution",
            "constitution.md",
            "--alignment-dir",
            "handover/alignment",
            "--tamper-dir",
            run_dir.join("tamper_work").to_str().expect("utf8 path"),
            "--out",
            tamper_report.to_str().expect("utf8 path"),
        ])
        .output()
        .expect("run audit_tape_tamper");
    assert!(
        tamper.status.success(),
        "audit_tape_tamper failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&tamper.stdout),
        String::from_utf8_lossy(&tamper.stderr)
    );

    let post_tamper_replay_report = run_dir.join("post_tamper_replay_report.json");
    let verify_after = Command::new(bin("turingos"))
        .env("TURINGOS_BIN_DIR", bin_dir(bin("verify_chaintape")))
        .args([
            "verify",
            "chaintape",
            "--repo",
            run_dir.join("runtime_repo").to_str().expect("utf8 path"),
            "--cas",
            run_dir.join("cas").to_str().expect("utf8 path"),
            "--run-id",
            "constitution-true-suite-replay-cas",
            "--out",
            post_tamper_replay_report.to_str().expect("utf8 path"),
        ])
        .output()
        .expect("rerun turingos verify chaintape");
    assert!(
        verify_after.status.success(),
        "post-tamper original verify failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&verify_after.stdout),
        String::from_utf8_lossy(&verify_after.stderr)
    );

    assert_replay_green(&read_json(&replay_report), "replay");
    assert_replay_green(&read_json(&post_tamper_replay_report), "post_tamper_replay");

    let tamper_json = read_json(&tamper_report);
    assert_eq!(
        tamper_json.get("detected_count").and_then(Value::as_u64),
        Some(3)
    );
    assert_eq!(tamper_json.get("expected").and_then(Value::as_u64), Some(3));
    assert_eq!(
        tamper_json.get("all_detected").and_then(Value::as_bool),
        Some(true)
    );
    let rows = tamper_json
        .get("tamper_results")
        .and_then(Value::as_array)
        .expect("tamper_results array");
    assert_eq!(rows.len(), 3);
    assert!(
        rows.iter()
            .all(|row| row.get("detected").and_then(Value::as_bool) == Some(true)),
        "all tamper rows must be detected: {tamper_json}"
    );

    let genesis_report = read_json(&run_dir.join("runtime_repo").join("genesis_report.json"));
    let constitution_hash =
        sha256_hex(&std::fs::read("constitution.md").expect("read constitution"));
    assert_eq!(
        genesis_report
            .get("constitution_hash")
            .and_then(Value::as_str),
        Some(constitution_hash.as_str())
    );
}

#[test]
fn replay_cas_tamper_runner_script_uses_current_kernel_not_historical_fixtures() {
    let script =
        std::fs::read_to_string("scripts/run_true_suite_replay_cas_tamper_current_kernel.sh")
            .expect("read runner script");
    assert!(script.contains("turingos init"));
    assert!(script.contains("boot_cli_current_kernel_fresh"));
    assert!(script.contains("verify chaintape"));
    assert!(script.contains("audit_tape_tamper"));
    assert!(script.contains("\"agents\":{}"));
    assert!(script.contains("tamper_report.json"));
    assert!(script.contains("post_tamper_replay_report.json"));
    assert!(script.contains("handover/evidence/true_suite"));
    assert!(
        !script.contains("tb_13_real_llm_smoke"),
        "true-suite replay/CAS runner must not use historical smoke fixture"
    );
    assert!(
        !script.contains("cas_git_repair_challenge_final_20260517T095728Z"),
        "true-suite replay/CAS runner must not reuse historical repair evidence"
    );
}
