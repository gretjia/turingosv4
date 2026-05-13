use std::fs;
use std::path::Path;

use tempfile::tempdir;
use turingosv4::runtime::dev_harness::{
    close_run, open_run, record_audit, record_command, record_diff_text, validate_run,
    DevHarnessError, DevOpenRequest,
};

fn base_request(root: &Path) -> DevOpenRequest {
    DevOpenRequest {
        evidence_root: root.to_path_buf(),
        title: "docs harness bootstrap".to_string(),
        module: "Harness".to_string(),
        molecule_or_atom: "molecule".to_string(),
        requested_risk_class: 1,
        fc_nodes: vec!["FC3-N33".to_string()],
        allowed_paths: vec!["HARNESS.md".to_string()],
        acceptance_commands: vec!["true".to_string()],
        human_intent: Some("unify cross-agent harness".to_string()),
        ratification: None,
        git_head: Some("test-head".to_string()),
    }
}

#[test]
fn dev_harness_open_writes_manifest_and_fc_witness() {
    let tmp = tempdir().unwrap();
    let run = open_run(base_request(tmp.path())).unwrap();

    let manifest = fs::read_to_string(run.run_dir.join("DevTaskManifest.json")).unwrap();
    assert!(manifest.contains("\"title\": \"docs harness bootstrap\""));
    assert!(manifest.contains("\"risk_class\": 1"));
    assert!(manifest.contains("\"audit_required\": false"));

    let fc_witness = fs::read_to_string(run.run_dir.join("FCWitnessManifest.json")).unwrap();
    assert!(fc_witness.contains("FC3-N33"));
    assert!(run.run_dir.join("events.jsonl").exists());
    assert!(run.run_dir.join("events_hash_chain.json").exists());
}

#[test]
fn dev_harness_restricted_diff_forces_class4_and_close_requires_audit() {
    let tmp = tempdir().unwrap();
    let run = open_run(base_request(tmp.path())).unwrap();

    record_diff_text(
        &run.run_dir,
        "diff --git a/src/state/typed_tx.rs b/src/state/typed_tx.rs\n",
    )
    .unwrap();

    let validation = validate_run(&run.run_dir).unwrap();
    assert_eq!(validation.effective_risk_class, 4);
    assert!(validation.audit_required);
    assert!(validation
        .restricted_surface_hits
        .iter()
        .any(|hit| hit.contains("src/state/typed_tx.rs")));

    let err = close_run(&run.run_dir).unwrap_err();
    assert!(matches!(err, DevHarnessError::AuditRequired));
}

#[test]
fn dev_harness_doc_mentions_of_restricted_paths_do_not_escalate_risk() {
    let tmp = tempdir().unwrap();
    let run = open_run(base_request(tmp.path())).unwrap();

    record_diff_text(
        &run.run_dir,
        "diff --git a/HARNESS.md b/HARNESS.md\n+Mention src/state/typed_tx.rs as a restricted path.\n",
    )
    .unwrap();

    let validation = validate_run(&run.run_dir).unwrap();
    assert_eq!(validation.effective_risk_class, 1);
    assert!(validation.restricted_surface_hits.is_empty());
}

#[test]
fn dev_harness_records_failed_command_as_evidence_and_blocks_close() {
    let tmp = tempdir().unwrap();
    let run = open_run(base_request(tmp.path())).unwrap();

    let command = record_command(
        &run.run_dir,
        &["sh", "-c", "echo before; echo err >&2; exit 7"],
    )
    .unwrap();

    assert_eq!(command.exit_code, 7);
    assert!(fs::read_to_string(run.run_dir.join(&command.stdout.path))
        .unwrap()
        .contains("before"));
    assert!(fs::read_to_string(run.run_dir.join(&command.stderr.path))
        .unwrap()
        .contains("err"));

    let validation = validate_run(&run.run_dir).unwrap();
    assert!(!validation.acceptance_passed);

    let err = close_run(&run.run_dir).unwrap_err();
    assert!(matches!(err, DevHarnessError::AcceptanceFailed));
}

#[test]
fn dev_harness_close_requires_at_least_one_command_evidence() {
    let tmp = tempdir().unwrap();
    let mut req = base_request(tmp.path());
    req.acceptance_commands.clear();
    let run = open_run(req).unwrap();

    let err = close_run(&run.run_dir).unwrap_err();
    assert!(matches!(err, DevHarnessError::AcceptanceFailed));
}

#[test]
fn dev_harness_hash_chain_detects_tampering() {
    let tmp = tempdir().unwrap();
    let run = open_run(base_request(tmp.path())).unwrap();
    record_diff_text(&run.run_dir, "diff --git a/HARNESS.md b/HARNESS.md\n").unwrap();

    fs::write(run.run_dir.join("events.jsonl"), "{\"tampered\":true}\n").unwrap();

    let err = validate_run(&run.run_dir).unwrap_err();
    assert!(matches!(err, DevHarnessError::HashChainBroken { .. }));
}

#[test]
fn dev_harness_closes_with_passing_evidence_and_proceed_audit_when_required() {
    let tmp = tempdir().unwrap();
    let mut req = base_request(tmp.path());
    req.requested_risk_class = 3;
    req.allowed_paths = vec!["src/runtime/dev_harness.rs".to_string()];
    let run = open_run(req).unwrap();

    record_command(&run.run_dir, &["sh", "-c", "true"]).unwrap();
    let audit_file = run.run_dir.join("audit.md");
    fs::write(&audit_file, "Findings: none\nVerdict: PROCEED\n").unwrap();
    record_audit(
        &run.run_dir,
        "clean-context-codex",
        "PROCEED",
        &audit_file,
        "no findings",
    )
    .unwrap();

    let summary = close_run(&run.run_dir).unwrap();
    assert_eq!(summary.close_status, "closed");
    assert!(summary.acceptance_passed);
    assert_eq!(summary.audit_verdict.as_deref(), Some("PROCEED"));
    assert!(run.run_dir.join("DevRunSummary.json").exists());
}

#[test]
fn dev_harness_close_summary_records_final_event_chain_head() {
    let tmp = tempdir().unwrap();
    let run = open_run(base_request(tmp.path())).unwrap();

    record_command(&run.run_dir, &["sh", "-c", "true"]).unwrap();
    let summary = close_run(&run.run_dir).unwrap();
    let chain: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(run.run_dir.join("events_hash_chain.json")).unwrap(),
    )
    .unwrap();

    assert_eq!(
        summary.event_chain_head_hash,
        chain["head_hash"].as_str().unwrap()
    );
}
