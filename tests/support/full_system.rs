use std::path::Path;
use std::process::Command;

use serde_json::Value;

fn current_source_commit() -> String {
    let repo = git2::Repository::discover(env!("CARGO_MANIFEST_DIR"))
        .expect("discover source tree git repo");
    let commit = repo
        .head()
        .expect("source tree HEAD")
        .peel_to_commit()
        .expect("source tree HEAD commit")
        .id()
        .to_string();
    commit
}

pub fn assert_source_tree_fingerprint_current(report: &Value) {
    let source_tree = report
        .get("source_tree")
        .and_then(Value::as_object)
        .expect("full-system receipt must carry source_tree object");
    assert_eq!(
        source_tree.get("commit").and_then(Value::as_str),
        Some(current_source_commit().as_str()),
        "full-system receipt must bind to the current TuringOS source tree HEAD"
    );
    assert!(
        matches!(
            source_tree.get("status").and_then(Value::as_str),
            Some("clean") | Some("dirty_allowed_recorded")
        ),
        "receipt must explicitly classify source tree cleanliness"
    );
}

pub fn run_full_system_augment(run_dir: &Path, run_id: &str, augment_bin: &str) {
    let augment = Command::new(augment_bin)
        .args([
            "--runtime-repo",
            run_dir.join("runtime_repo").to_str().expect("utf8 path"),
            "--cas",
            run_dir.join("cas").to_str().expect("utf8 path"),
            "--run-id",
            run_id,
            "--constitution",
            "constitution.md",
            "--out-dir",
            run_dir.to_str().expect("utf8 path"),
        ])
        .output()
        .expect("run full-system augment helper");
    assert!(
        augment.status.success(),
        "full-system augment failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&augment.stdout),
        String::from_utf8_lossy(&augment.stderr)
    );
    std::fs::copy(
        run_dir.join("runtime_repo").join("genesis_report.json"),
        run_dir.join("genesis_report.json"),
    )
    .expect("copy refreshed genesis report");
}

pub fn assert_full_system_lit(
    run_dir: &Path,
    run_id: &str,
    family_id: &str,
    entrypoint: &str,
    domain_manifest_name: &str,
    replay_report: &Path,
    participation_bin: &str,
) -> Value {
    let participation_report = run_dir.join("full_system_participation.json");
    let participation = Command::new(participation_bin)
        .args([
            "--run-id",
            run_id,
            "--family-id",
            family_id,
            "--entrypoint",
            entrypoint,
            "--runtime-repo",
            run_dir.join("runtime_repo").to_str().expect("utf8 path"),
            "--cas",
            run_dir.join("cas").to_str().expect("utf8 path"),
            "--replay-report",
            replay_report.to_str().expect("utf8 path"),
            "--genesis-report",
            run_dir
                .join("genesis_report.json")
                .to_str()
                .expect("utf8 path"),
            "--domain-manifest",
            run_dir
                .join(domain_manifest_name)
                .to_str()
                .expect("utf8 path"),
            "--fc3-index",
            run_dir
                .join("governance_capsule_index.json")
                .to_str()
                .expect("utf8 path"),
            "--require-full-system",
            "--out",
            participation_report.to_str().expect("utf8 path"),
        ])
        .output()
        .expect("run full-system participation helper");
    assert!(
        participation.status.success(),
        "full-system participation failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&participation.stdout),
        String::from_utf8_lossy(&participation.stderr)
    );
    let report: Value = serde_json::from_str(
        &std::fs::read_to_string(&participation_report).expect("read participation report"),
    )
    .expect("parse participation report");
    assert_eq!(
        report
            .get("verdict")
            .and_then(|v| v.get("full_system_participation"))
            .and_then(Value::as_bool),
        Some(true)
    );
    assert_eq!(
        report
            .get("verdict")
            .and_then(|v| v.get("full_system_verdict"))
            .and_then(Value::as_str),
        Some("FULL_SYSTEM_LIT")
    );
    assert_eq!(
        report
            .get("fc1")
            .and_then(|v| v.get("present"))
            .and_then(Value::as_bool),
        Some(true)
    );
    assert_eq!(
        report
            .get("fc2")
            .and_then(|v| v.get("present"))
            .and_then(Value::as_bool),
        Some(true)
    );
    assert_eq!(
        report
            .get("fc3")
            .and_then(|v| v.get("typed_meta_roles_present"))
            .and_then(Value::as_bool),
        Some(true)
    );
    assert_eq!(
        report
            .get("fc3")
            .and_then(|v| v.get("reinit_semantics_present"))
            .and_then(Value::as_bool),
        Some(true)
    );
    assert_eq!(
        report
            .get("market")
            .and_then(|v| v.get("present"))
            .and_then(Value::as_bool),
        Some(true)
    );
    assert!(
        report
            .get("market")
            .and_then(|v| v.get("agent_market_action_txs"))
            .and_then(Value::as_u64)
            .unwrap_or(0)
            > 0,
        "full-system sample must include an agent market action"
    );
    assert!(
        report
            .get("market")
            .and_then(|v| v.get("market_decision_submitted_count"))
            .and_then(Value::as_u64)
            .unwrap_or(0)
            > 0,
        "full-system sample must write a submitted market decision trace"
    );
    assert_source_tree_fingerprint_current(&report);
    report
}
