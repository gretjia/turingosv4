//! Broad AGI true-suite batch runner contract.
//!
//! This gate is deliberately about evidence accounting, not benchmark scoring.
//! It proves the batch control plane cannot turn manifests, old-15 evidence,
//! leaderboard rows, or pending adapters into OBL-005 closure.

use std::process::Command;

use serde_json::Value;
use tempfile::TempDir;

const SCRIPT: &str = "scripts/run_true_suite_broad_agi_batch.sh";

fn read_json(path: &std::path::Path) -> Value {
    serde_json::from_str(&std::fs::read_to_string(path).expect("read json")).expect("parse json")
}

#[test]
fn broad_agi_batch_plan_only_writes_non_closing_pending_report() {
    let tmp = TempDir::new().expect("tempdir");
    let run_root = tmp.path().join("true_suite_batch");

    let output = Command::new("bash")
        .args([
            SCRIPT,
            "--plan-only",
            "--run-id",
            "constitution-broad-batch-contract",
            "--run-root",
            run_root.to_str().expect("utf8 path"),
        ])
        .output()
        .expect("run broad AGI batch script");
    assert!(
        output.status.success(),
        "broad batch plan-only failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let batch_dir = run_root.join("broad_batch");
    let manifest = read_json(&batch_dir.join("broad_agi_batch_manifest.json"));
    let aggregate = read_json(&batch_dir.join("aggregate_fc_trace_report.json"));
    let results_raw =
        std::fs::read_to_string(batch_dir.join("family_results.jsonl")).expect("read jsonl");
    let results: Vec<Value> = results_raw
        .lines()
        .map(|line| serde_json::from_str(line).expect("parse jsonl row"))
        .collect();

    assert_eq!(
        manifest.get("schema_version").and_then(Value::as_str),
        Some("turingosv4.true_suite.broad_agi_batch.v1")
    );
    assert_eq!(
        manifest.get("mode").and_then(Value::as_str),
        Some("plan-only")
    );
    assert_eq!(
        manifest.get("closure_status").and_then(Value::as_str),
        Some("OPEN_REAL_WORLD_COVERAGE_PENDING")
    );
    assert_eq!(
        manifest
            .get("full_system_required_for_final")
            .and_then(Value::as_bool),
        Some(true)
    );
    assert_eq!(
        manifest
            .get("per_sample_fc_union_is_not_sufficient")
            .and_then(Value::as_bool),
        Some(true)
    );
    assert_eq!(
        manifest
            .get("market_participation_required_for_every_sample")
            .and_then(Value::as_bool),
        Some(true)
    );
    assert_eq!(
        manifest
            .get("full_system_closure_candidate")
            .and_then(Value::as_bool),
        Some(false),
        "plan-only must never become a full-system closure candidate"
    );
    assert_eq!(
        manifest
            .get("all_declared_artifacts_present")
            .and_then(Value::as_bool),
        Some(false),
        "full-system closure also requires all declared replay/CAS/domain artifacts"
    );
    assert_eq!(
        manifest
            .get("closure_decision_source")
            .and_then(Value::as_str),
        Some("OBL-005 witness after per-sample full_system_participation reports")
    );
    assert_eq!(
        manifest
            .get("old_15_is_not_sufficient")
            .and_then(Value::as_bool),
        Some(true)
    );
    assert_eq!(
        manifest
            .get("leaderboard_score_is_not_liveness")
            .and_then(Value::as_bool),
        Some(true)
    );
    let outputs = manifest
        .get("outputs")
        .and_then(Value::as_object)
        .expect("outputs");
    for key in [
        "family_results_jsonl",
        "aggregate_fc_trace_report",
        "evidence_package_manifest",
    ] {
        let path = outputs
            .get(key)
            .and_then(Value::as_str)
            .unwrap_or_else(|| panic!("missing output path {key}"));
        assert!(
            !path.starts_with('/'),
            "batch manifest output path {key} must not leak a machine absolute path: {path}"
        );
        assert!(
            path.starts_with("<run-root>/") || path.starts_with("handover/evidence/true_suite/"),
            "batch manifest output path {key} must be reconstructable relative evidence path, got {path}"
        );
    }

    let guards = manifest
        .get("no_overclaim_guards")
        .and_then(Value::as_array)
        .expect("no_overclaim_guards");
    for required in [
        "plan-only mode cannot emit passed coverage",
        "pending benchmark adapters never count as liveness pass",
        "old 15-question evidence cannot close OBL-005",
        "leaderboard score is capability signal only, not module liveness",
        "TDMA evidence is domain tape evidence, not bottom-white L4 ChainTape",
        "provider raw prompt/response is not a valid final artifact",
        "full_system_participation.json must be parsed and FULL_SYSTEM_LIT, not merely present",
        "domain artifacts without a FULL_SYSTEM_LIT participation report remain partial runner evidence",
        "market/economy must participate even in one-agent runs via invest or tape-visible abstention",
    ] {
        assert!(
            guards.iter().any(|guard| guard.as_str() == Some(required)),
            "missing no-overclaim guard: {required}"
        );
    }

    assert!(
        results.len() >= 18,
        "batch must report real-world domains plus broad AGI families, got {}",
        results.len()
    );
    assert!(
        results.iter().all(|row| {
            row.get("status")
                .and_then(Value::as_str)
                .is_some_and(|status| {
                    !matches!(status, "PASS" | "passed" | "full_system_participation_passed")
                })
        }),
        "plan-only results must stay pending/partial or runner-required, not full-system passed: {results:?}"
    );
    assert!(
        results.iter().all(|row| {
            row.get("full_system_verdict").and_then(Value::as_str) == Some("MISSING")
                && row
                    .get("full_system_report_lit")
                    .and_then(Value::as_bool)
                    == Some(false)
        }),
        "plan-only rows with no report must be explicit missing full-system evidence, not final evidence"
    );
    for required_id in [
        "gaia_general_assistant",
        "gpqa_science_reasoning",
        "math_formal_proof",
        "swebench_live_coding_repair",
        "webarena_web_agent",
        "mind2web_open_web",
        "osworld_computer_use",
        "toolbench_api_tool_use",
        "cybench_security_sandbox",
        "market_economy_polymarket",
        "memory_feedback_reinit",
    ] {
        assert!(
            results
                .iter()
                .any(|row| row.get("id").and_then(Value::as_str) == Some(required_id)),
            "batch results missing required broad AGI family {required_id}"
        );
    }

    assert_eq!(
        aggregate
            .get("full_system_closure_candidate")
            .and_then(Value::as_bool),
        Some(false)
    );
    assert!(
        aggregate
            .get("pending_result_count")
            .and_then(Value::as_u64)
            .unwrap_or(0)
            > 0,
        "broad batch cannot be final while adapters/evidence are pending"
    );
    assert_eq!(
        aggregate
            .get("all_required_fc_blocks_declared")
            .and_then(Value::as_bool),
        Some(true),
        "batch report must still declare FC1/FC2/FC3 trace coverage"
    );
    assert_eq!(
        aggregate
            .get("per_result_required_fc_blocks_declared")
            .and_then(Value::as_bool),
        Some(true),
        "each result must declare FC1/FC2/FC3; FC union across different results is not enough"
    );
    assert_eq!(
        aggregate
            .get("full_system_participation_pass_count")
            .and_then(Value::as_u64),
        Some(0),
        "plan-only cannot produce full-system participation passes"
    );
}

#[test]
fn broad_agi_batch_script_preserves_external_boundary_and_no_overclaim_guards() {
    let script = std::fs::read_to_string(SCRIPT).expect("read broad batch script");
    assert!(script.contains("broad_agi_true_suite_manifest.toml"));
    assert!(script.contains("realworld_liveness_coverage.toml"));
    assert!(script.contains("run_true_suite_boot_cli_current_kernel.sh"));
    assert!(script.contains("run_true_suite_replay_cas_tamper_current_kernel.sh"));
    assert!(script.contains("run_true_suite_market_external_agent.sh"));
    assert!(script.contains("run_true_suite_generate_artifact_current_kernel.sh"));
    assert!(script.contains("run_true_suite_tdma_current_kernel.sh"));
    assert!(script.contains("run_true_suite_fc3_governance_reinit_current_kernel.sh"));
    assert!(script.contains("run_true_suite_gpqa_science_reasoning_current_kernel.sh"));
    assert!(script.contains("run_true_suite_math_competition_current_kernel.sh"));
    assert!(script.contains("run_true_suite_swebench_current_kernel.sh"));
    assert!(script.contains("run_true_suite_toolbench_current_kernel.sh"));
    assert!(script.contains("run_true_suite_mind2web_current_kernel.sh"));
    assert!(script.contains("package_true_suite_evidence.sh"));
    assert!(script.contains("evidence_package_manifest"));
    assert!(script.contains("fc3_governance_reinit_fresh"));
    assert!(script.contains("gpqa_science_reasoning_fresh"));
    assert!(script.contains("math_competition_reasoning_fresh"));
    assert!(script.contains("swebench_live_coding_repair_fresh"));
    assert!(script.contains("toolbench_api_tool_use_fresh"));
    assert!(script.contains("mind2web_open_web_fresh"));
    assert!(script.contains("memory_feedback_reinit"));
    assert!(script.contains("gpqa_science_reasoning"));
    assert!(script.contains("math_formal_proof"));
    assert!(script.contains("swebench_live_coding_repair"));
    assert!(script.contains("toolbench_api_tool_use"));
    assert!(script.contains("mind2web_open_web"));
    assert!(script.contains("benchmark_adapter_pending"));
    assert!(script.contains("OPEN_REAL_WORLD_COVERAGE_PENDING"));
    assert!(script.contains("full_system_participation.json"));
    assert!(script.contains("\"MISSING\""));
    assert!(script.contains("full_system_participation_report_partial"));
    assert!(script.contains("INVALID_FULL_SYSTEM_REPORT"));
    assert!(script.contains("full_system_closure_candidate"));
    assert!(script.contains("and all_declared_artifacts_present"));
    assert!(script.contains("full_system_participation.json must be parsed and FULL_SYSTEM_LIT"));
    assert!(script.contains("old 15-question evidence cannot close OBL-005"));
    assert!(script.contains("leaderboard score is capability signal only"));
    assert!(script.contains("TDMA evidence is domain tape evidence"));
    assert!(script.contains("provider raw prompt/response is not a valid final artifact"));
    assert!(script.contains("market/economy must participate even in one-agent runs"));
    assert!(
        !script.contains("raw_prompt") && !script.contains("raw_response"),
        "batch evidence must not introduce raw provider prompt/response artifacts"
    );
    assert!(
        !script.contains("stage_phase7_real_e2e")
            && !script.contains("real8x_market_ab_clean")
            && !script.contains("old_15_question"),
        "batch runner must not inherit historical candidate evidence as final input"
    );
}

fn write_file(path: &std::path::Path, body: &str) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create parent");
    }
    std::fs::write(path, body).expect("write file");
}

fn seed_fc3_final_artifacts(run_root: &std::path::Path, report_body: &str) {
    let fc3 = run_root.join("fc3");
    std::fs::create_dir_all(fc3.join("cas")).expect("create cas dir");
    write_file(&fc3.join("chaintape.jsonl"), "{}\n");
    write_file(&fc3.join("cas.dotgit.tar.gz"), "");
    write_file(
        &fc3.join("fc3_replay_report.json"),
        r#"{"ledger_root_verified":true}"#,
    );
    write_file(&fc3.join("governance_capsule_index.json"), "{}\n");
    write_file(&fc3.join("fc3_governance_reinit_run_manifest.json"), "{}\n");
    write_file(&fc3.join("full_system_participation.json"), report_body);
}

fn run_batch_plan_only(run_root: &std::path::Path, run_id: &str) -> std::path::PathBuf {
    let output = Command::new("bash")
        .args([
            SCRIPT,
            "--plan-only",
            "--run-id",
            run_id,
            "--run-root",
            run_root.to_str().expect("utf8 path"),
        ])
        .output()
        .expect("run broad AGI batch script");
    assert!(
        output.status.success(),
        "broad batch plan-only failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    run_root.join("broad_batch")
}

#[test]
fn broad_agi_batch_parses_partial_participation_report_content() {
    let tmp = TempDir::new().expect("tempdir");
    let run_root = tmp.path().join("true_suite_batch");
    seed_fc3_final_artifacts(
        &run_root,
        r#"{
  "schema_version": "turingosv4.true_suite.full_system_participation.v1",
  "fc1": {"present": true},
  "fc2": {"present": true},
  "fc3": {"typed_meta_roles_present": true, "reinit_semantics_present": true},
  "market": {"present": false},
  "replay": {"all_indicators_pass": true},
  "verdict": {
    "full_system_participation": false,
    "full_system_verdict": "PARTIAL_RUNNER_ONLY",
    "missing": ["market_economy_invest_or_visible_abstention"],
    "final_closure_possible": false
  }
}
"#,
    );

    let batch_dir = run_batch_plan_only(&run_root, "constitution-broad-batch-partial");
    let aggregate = read_json(&batch_dir.join("aggregate_fc_trace_report.json"));
    let results_raw =
        std::fs::read_to_string(batch_dir.join("family_results.jsonl")).expect("read jsonl");
    let fc3_rows: Vec<Value> = results_raw
        .lines()
        .map(|line| serde_json::from_str(line).expect("parse jsonl row"))
        .filter(|row: &Value| {
            matches!(
                row.get("id").and_then(Value::as_str),
                Some("fc3_governance_reinit_fresh" | "memory_feedback_reinit")
            )
        })
        .collect();

    assert_eq!(
        fc3_rows.len(),
        2,
        "domain and broad-family FC3 rows should both see the same artifacts"
    );
    for row in &fc3_rows {
        assert_eq!(
            row.get("final_artifacts_present").and_then(Value::as_bool),
            Some(true)
        );
        assert_eq!(
            row.get("full_system_report_present")
                .and_then(Value::as_bool),
            Some(true)
        );
        assert_eq!(
            row.get("full_system_report_lit").and_then(Value::as_bool),
            Some(false)
        );
        assert_eq!(
            row.get("status").and_then(Value::as_str),
            Some("full_system_participation_report_partial")
        );
        assert_eq!(
            row.get("full_system_verdict").and_then(Value::as_str),
            Some("PARTIAL_RUNNER_ONLY")
        );
        let missing = row
            .get("full_system_missing")
            .and_then(Value::as_array)
            .expect("missing");
        assert!(missing
            .iter()
            .any(|v| { v.as_str() == Some("market_economy_invest_or_visible_abstention") }));
    }
    assert_eq!(
        aggregate
            .get("full_system_participation_pass_count")
            .and_then(Value::as_u64),
        Some(0),
        "partial full-system report content must not count as a pass"
    );
    assert_eq!(
        aggregate
            .get("full_system_closure_candidate")
            .and_then(Value::as_bool),
        Some(false)
    );
}

#[test]
fn broad_agi_batch_rejects_claimed_full_report_with_missing_market_row() {
    let tmp = TempDir::new().expect("tempdir");
    let run_root = tmp.path().join("true_suite_batch");
    seed_fc3_final_artifacts(
        &run_root,
        r#"{
  "schema_version": "turingosv4.true_suite.full_system_participation.v1",
  "fc1": {"present": true},
  "fc2": {"present": true},
  "fc3": {"typed_meta_roles_present": true, "reinit_semantics_present": true},
  "market": {"present": false},
  "replay": {"all_indicators_pass": true},
  "verdict": {
    "full_system_participation": true,
    "full_system_verdict": "FULL_SYSTEM_LIT",
    "missing": [],
    "final_closure_possible": true
  }
}
"#,
    );

    let batch_dir = run_batch_plan_only(&run_root, "constitution-broad-batch-invalid-full");
    let results_raw =
        std::fs::read_to_string(batch_dir.join("family_results.jsonl")).expect("read jsonl");
    let fc3_row: Value = results_raw
        .lines()
        .map(|line| serde_json::from_str(line).expect("parse jsonl row"))
        .find(|row: &Value| {
            row.get("kind").and_then(Value::as_str) == Some("broad_agi_family")
                && row.get("id").and_then(Value::as_str) == Some("memory_feedback_reinit")
        })
        .expect("memory_feedback_reinit row");

    assert_eq!(
        fc3_row
            .get("full_system_report_lit")
            .and_then(Value::as_bool),
        Some(false)
    );
    assert_eq!(
        fc3_row.get("full_system_verdict").and_then(Value::as_str),
        Some("INVALID_FULL_SYSTEM_REPORT")
    );
    assert_eq!(
        fc3_row.get("status").and_then(Value::as_str),
        Some("full_system_participation_report_partial")
    );
    let missing = fc3_row
        .get("full_system_missing")
        .and_then(Value::as_array)
        .expect("missing");
    assert!(missing
        .iter()
        .any(|v| v.as_str() == Some("market_economy_invest_or_visible_abstention")));
}

#[test]
fn broad_agi_batch_rejects_market_opportunity_without_agent_choice() {
    let tmp = TempDir::new().expect("tempdir");
    let run_root = tmp.path().join("true_suite_batch");
    seed_fc3_final_artifacts(
        &run_root,
        r#"{
  "schema_version": "turingosv4.true_suite.full_system_participation.v1",
  "fc1": {"present": true},
  "fc2": {"present": true},
  "fc3": {"typed_meta_roles_present": true, "reinit_semantics_present": true},
  "market": {
    "present": true,
    "mode": "opportunity_visible_missing_agent_choice",
    "agent_market_action_txs": 0,
    "market_decision_submitted_count": 0,
    "market_decision_no_trade_count": 0,
    "market_decision_declined_count": 0,
    "market_opportunity_trace_count": 1
  },
  "replay": {"all_indicators_pass": true},
  "verdict": {
    "full_system_participation": true,
    "full_system_verdict": "FULL_SYSTEM_LIT",
    "missing": [],
    "final_closure_possible": true
  }
}
"#,
    );

    let batch_dir = run_batch_plan_only(&run_root, "constitution-broad-batch-opportunity-only");
    let results_raw =
        std::fs::read_to_string(batch_dir.join("family_results.jsonl")).expect("read jsonl");
    let fc3_row: Value = results_raw
        .lines()
        .map(|line| serde_json::from_str(line).expect("parse jsonl row"))
        .find(|row: &Value| {
            row.get("kind").and_then(Value::as_str) == Some("broad_agi_family")
                && row.get("id").and_then(Value::as_str) == Some("memory_feedback_reinit")
        })
        .expect("memory_feedback_reinit row");

    assert_eq!(
        fc3_row
            .get("full_system_report_lit")
            .and_then(Value::as_bool),
        Some(false),
        "visible market opportunity alone is not an invest/abstain choice"
    );
    assert_eq!(
        fc3_row.get("full_system_verdict").and_then(Value::as_str),
        Some("INVALID_FULL_SYSTEM_REPORT")
    );
    let required_rows = fc3_row
        .get("full_system_required_rows")
        .and_then(Value::as_object)
        .expect("required rows");
    assert_eq!(
        required_rows
            .get("market_economy_invest_or_visible_abstention")
            .and_then(Value::as_bool),
        Some(false)
    );
}
