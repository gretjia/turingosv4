//! True-suite FC3 governance/re-init runner contract.
//!
//! This gate executes the current-kernel FC3 helper in a temp directory, then
//! verifies the resulting ChainTape via the public `turingos verify chaintape`
//! wrapper. It proves FC3 meta roles are runtime typed txs, not an external PR
//! ceremony or dashboard-only proof.

use std::path::Path;
use std::process::Command;

use serde_json::Value;
use tempfile::TempDir;

fn bin(name: &str) -> &'static str {
    match name {
        "turingos" => env!("CARGO_BIN_EXE_turingos"),
        "verify_chaintape" => env!("CARGO_BIN_EXE_verify_chaintape"),
        "fc3_governance_reinit_current_kernel" => {
            env!("CARGO_BIN_EXE_fc3_governance_reinit_current_kernel")
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

#[test]
fn fc3_governance_runner_executes_typed_meta_roles_and_replays() {
    let tmp = TempDir::new().expect("tempdir");
    let run_dir = tmp.path().join("fc3");

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

    let helper = Command::new(bin("fc3_governance_reinit_current_kernel"))
        .args([
            "--runtime-repo",
            run_dir.join("runtime_repo").to_str().expect("utf8 path"),
            "--cas",
            run_dir.join("cas").to_str().expect("utf8 path"),
            "--run-id",
            "constitution-true-suite-fc3",
            "--constitution",
            "constitution.md",
            "--out-dir",
            run_dir.to_str().expect("utf8 path"),
        ])
        .output()
        .expect("run fc3 helper");
    assert!(
        helper.status.success(),
        "fc3 helper failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&helper.stdout),
        String::from_utf8_lossy(&helper.stderr)
    );

    let replay_report = run_dir.join("fc3_replay_report.json");
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
            "constitution-true-suite-fc3",
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

    let replay = read_json(&replay_report);
    assert_eq!(replay.get("l4_entries").and_then(Value::as_u64), Some(8));
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

    let index = read_json(&run_dir.join("governance_capsule_index.json"));
    let tx_rows = index
        .get("tx_sequence")
        .and_then(Value::as_array)
        .expect("tx_sequence array");
    let tx_kinds: Vec<&str> = tx_rows
        .iter()
        .map(|row| row.get("tx_kind").and_then(Value::as_str).expect("tx_kind"))
        .collect();
    for expected in [
        "MapReduceTick",
        "LogFeedbackArchive",
        "ArchitectProposal",
        "VetoDecision",
        "ArchitectCommit",
        "TerminalSummary",
        "ReinitRequest",
        "ReinitBoot",
    ] {
        assert!(
            tx_kinds.contains(&expected),
            "missing typed FC3 tx kind {expected}: {tx_kinds:?}"
        );
    }
    for key in [
        "all_fc3_typed_transactions_present",
        "architectai_commit_after_vetoai_pass",
        "terminal_errorhalt_reinit_request_and_boot_present",
        "handover_files_not_source_of_truth",
        "dashboard_stdout_not_evidence",
    ] {
        assert_eq!(
            index
                .get("checks")
                .and_then(|v| v.get(key))
                .and_then(Value::as_bool),
            Some(true),
            "governance index check `{key}` must pass: {index}"
        );
    }

    let chaintape_rows =
        std::fs::read_to_string(run_dir.join("chaintape.jsonl")).expect("read chaintape jsonl");
    assert_eq!(chaintape_rows.lines().count(), 8);
    assert!(run_dir
        .join("runtime_repo")
        .join("genesis_report.json")
        .is_file());
    let copied_genesis = run_dir.join("genesis_report.json");
    std::fs::copy(
        run_dir.join("runtime_repo").join("genesis_report.json"),
        &copied_genesis,
    )
    .expect("copy genesis report");
    assert!(run_dir
        .join("runtime_repo")
        .join("pinned_pubkeys.json")
        .is_file());
    assert!(run_dir
        .join("runtime_repo")
        .join("initial_q_state.json")
        .is_file());

    let participation_report = run_dir.join("full_system_participation.json");
    let participation = Command::new(bin("full_system_participation_current_kernel"))
        .args([
            "--run-id",
            "constitution-true-suite-fc3",
            "--family-id",
            "memory_feedback_reinit",
            "--entrypoint",
            "tests/constitution_true_suite_fc3_governance_runner.rs",
            "--runtime-repo",
            run_dir.join("runtime_repo").to_str().expect("utf8 path"),
            "--cas",
            run_dir.join("cas").to_str().expect("utf8 path"),
            "--replay-report",
            replay_report.to_str().expect("utf8 path"),
            "--genesis-report",
            copied_genesis.to_str().expect("utf8 path"),
            "--fc3-index",
            run_dir
                .join("governance_capsule_index.json")
                .to_str()
                .expect("utf8 path"),
            "--out",
            participation_report.to_str().expect("utf8 path"),
        ])
        .output()
        .expect("run full-system participation helper");
    assert!(
        participation.status.success(),
        "participation helper failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&participation.stdout),
        String::from_utf8_lossy(&participation.stderr)
    );
    let participation = read_json(&participation_report);
    assert_eq!(
        participation
            .get("fc3")
            .and_then(|v| v.get("typed_meta_roles_present"))
            .and_then(Value::as_bool),
        Some(true)
    );
    assert_eq!(
        participation
            .get("fc3")
            .and_then(|v| v.get("reinit_semantics_present"))
            .and_then(Value::as_bool),
        Some(true)
    );
    assert_eq!(
        participation
            .get("market")
            .and_then(|v| v.get("present"))
            .and_then(Value::as_bool),
        Some(false),
        "FC3-only run must honestly report absent market/economy participation"
    );
    assert_eq!(
        participation
            .get("verdict")
            .and_then(|v| v.get("full_system_participation"))
            .and_then(Value::as_bool),
        Some(false)
    );
}

#[test]
fn fc3_governance_runner_script_is_current_kernel_not_external_ceremony() {
    let script =
        std::fs::read_to_string("scripts/run_true_suite_fc3_governance_reinit_current_kernel.sh")
            .expect("read runner script");
    assert!(script.contains("turingos init"));
    assert!(script.contains("fc3_governance_reinit_current_kernel"));
    assert!(script.contains("verify chaintape"));
    assert!(script.contains("handover/evidence/true_suite"));
    assert!(script.contains("governance_capsule_index.json"));
    assert!(script.contains("fc3_replay_report.json"));
    assert!(script.contains("full_system_participation_current_kernel"));
    assert!(script.contains("full_system_participation.json"));
    assert!(
        !script.contains("handover/ai-direct/LATEST.md")
            && !script.contains("TB_LOG.tsv")
            && !script.contains("audit_dashboard_run_report"),
        "FC3 true-suite runner must not substitute handover/dashboard evidence for ChainTape/CAS"
    );
    assert!(
        !script.contains("LLM_PROXY_URL") && !script.contains("raw_response"),
        "FC3 governance runner is a typed runtime path and must not persist raw provider payloads"
    );
}
