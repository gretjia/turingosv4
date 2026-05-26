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
        "domain artifacts without full_system_participation.json remain partial runner evidence",
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
            row.get("full_system_verdict")
                .and_then(Value::as_str)
                == Some("PARTIAL_RUNNER_ONLY")
        }),
        "plan-only rows must be explicit partial-runner evidence, not final full-system evidence"
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
    assert!(script.contains("PARTIAL_RUNNER_ONLY"));
    assert!(script.contains("full_system_report_present_declared_artifacts_missing"));
    assert!(script.contains("full_system_closure_candidate"));
    assert!(script.contains("and all_declared_artifacts_present"));
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
