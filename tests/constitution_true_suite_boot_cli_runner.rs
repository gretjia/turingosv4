//! True-suite boot/CLI runner contract.
//!
//! This gate executes the boot helper in a temp directory, then verifies the
//! resulting ChainTape via the public `turingos verify chaintape` wrapper.

use std::path::Path;
use std::process::Command;

use serde_json::Value;
use tempfile::TempDir;

fn bin(name: &str) -> &'static str {
    match name {
        "turingos" => env!("CARGO_BIN_EXE_turingos"),
        "verify_chaintape" => env!("CARGO_BIN_EXE_verify_chaintape"),
        "boot_cli_current_kernel_fresh" => env!("CARGO_BIN_EXE_boot_cli_current_kernel_fresh"),
        _ => panic!("unknown bin {name}"),
    }
}

fn bin_dir(path: &str) -> &Path {
    Path::new(path).parent().expect("bin has parent")
}

#[test]
fn boot_cli_runner_executes_current_kernel_and_replays_via_cli() {
    let tmp = TempDir::new().expect("tempdir");
    let run_dir = tmp.path().join("boot_cli");

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
            "constitution-true-suite-boot-cli",
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
            "constitution-true-suite-boot-cli",
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

    let genesis_report = run_dir.join("runtime_repo").join("genesis_report.json");
    assert!(genesis_report.is_file(), "genesis_report.json missing");
    assert!(run_dir
        .join("runtime_repo")
        .join("initial_q_state.json")
        .is_file());
    assert!(run_dir
        .join("runtime_repo")
        .join("pinned_pubkeys.json")
        .is_file());
    assert!(run_dir.join("cas").is_dir());

    let replay: Value = serde_json::from_str(
        &std::fs::read_to_string(&replay_report).expect("read replay_report.json"),
    )
    .expect("parse replay report");
    assert_eq!(
        replay.get("l4_entries").and_then(Value::as_u64),
        Some(3),
        "fresh boot emits boot tick and resume emits one additional tick"
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

    let genesis: Value = serde_json::from_str(
        &std::fs::read_to_string(genesis_report).expect("read genesis report"),
    )
    .expect("parse genesis report");
    assert_eq!(
        genesis
            .get("constitution_hash")
            .and_then(Value::as_str)
            .map(str::len),
        Some(64)
    );
    assert_eq!(
        genesis.get("agent_pubkeys_path").and_then(Value::as_str),
        Some("agent_pubkeys.json")
    );
}

#[test]
fn boot_cli_runner_script_stays_external_to_kernel_simulation() {
    let script = std::fs::read_to_string("scripts/run_true_suite_boot_cli_current_kernel.sh")
        .expect("read runner script");
    assert!(script.contains("turingos init"));
    assert!(script.contains("boot_cli_current_kernel_fresh"));
    assert!(script.contains("verify chaintape"));
    assert!(script.contains("handover/evidence/true_suite"));
    assert!(
        !script.contains("TURINGOS_REAL7_SCRIPTED_TASK_OUTCOME_BUYS"),
        "boot evidence runner must not smuggle market scripted-buy fixtures"
    );
}
