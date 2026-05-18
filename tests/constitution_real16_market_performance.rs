//! REAL-16 -- Market Performance / E4 candidate gates.
//!
//! These tests pin the candidate-only E4 evidence contract before any
//! implementation: pinned A/B inputs, ChainTape/CAS-derived metrics, uncertainty
//! reporting, and no market_tx_count-only overclaim.

use turingosv4::runtime::market_performance_e4::{
    evaluate_market_performance_e4, E4ArmInput, E4MetricSource, E4Verdict,
};

fn arm(id: &str, solved: u32, exact_join_count: u64, source: E4MetricSource) -> E4ArmInput {
    E4ArmInput {
        arm_id: id.to_string(),
        evidence_dir: format!("handover/evidence/real16/arm_{id}"),
        problem_set_hash: "problem-hash-hard10".to_string(),
        model_assignment_hash: "model-hash".to_string(),
        budget_hash: "budget-hash".to_string(),
        prompt_template_hash: "prompt-hash".to_string(),
        runtime_config_hash: "runtime-hash-shared".to_string(),
        audit_tape_verdict: "PROCEED".to_string(),
        e2_verifier_verdict: "PROCEED".to_string(),
        e2_verifier_failure_reasons: Vec::new(),
        metric_source: source,
        market_pressure_enabled: id != "A",
        task_count: 10,
        solved_count: solved,
        verified_pput_micro: solved as u64 * 100,
        wasted_attempt_count: 80_u64.saturating_sub(solved as u64 * 10),
        verification_latency_ms_total: 10_000_u64.saturating_sub(solved as u64 * 1_000),
        failed_branch_count: 50_u64.saturating_sub(solved as u64 * 4),
        cost_per_solved_proof_tokens: if solved == 0 {
            None
        } else {
            Some(20_000_u64.saturating_sub(solved as u64 * 1_000))
        },
        exact_join_count,
        ev_to_action_conversion_bps: exact_join_count as u32 * 250,
        role_diversity_bps: if id == "A" { 2_000 } else { 6_000 },
        market_tx_count: exact_join_count + 50,
    }
}

#[test]
fn e4_candidate_rejects_runtime_pput_sidecar_as_claim_source() {
    let inputs = vec![
        arm("A", 2, 0, E4MetricSource::ChainTapeCasVerifier),
        arm("D", 5, 8, E4MetricSource::ChainTapeCasPlusRuntimePputResult),
    ];

    let report = evaluate_market_performance_e4(&inputs);

    assert_eq!(report.verdict, E4Verdict::Veto);
    assert!(!report.e4_candidate);
    assert!(report
        .failure_reasons
        .contains(&"runtime_pput_sidecar_not_claim_bearing".to_string()));
}

#[test]
fn e4_candidate_rejects_market_arm_e2_verifier_veto() {
    let mut d = arm("D", 5, 8, E4MetricSource::ChainTapeCasVerifier);
    d.e2_verifier_verdict = "VETO".to_string();
    d.e2_verifier_failure_reasons = vec!["scripted fixture CAS records present = 1".to_string()];
    let inputs = vec![arm("A", 2, 0, E4MetricSource::ChainTapeCasVerifier), d];

    let report = evaluate_market_performance_e4(&inputs);

    assert_eq!(report.verdict, E4Verdict::Veto);
    assert!(!report.e4_candidate);
    assert!(report
        .failure_reasons
        .contains(&"e2_verifier_not_proceed".to_string()));
    assert!(report
        .failure_reasons
        .contains(&"e2_verifier_failure_reasons_present".to_string()));
}

#[test]
fn e4_candidate_tolerates_control_arm_without_e2_candidate() {
    let mut a = arm("A", 2, 1, E4MetricSource::ChainTapeCasVerifier);
    a.e2_verifier_verdict = "VETO".to_string();
    a.e2_verifier_failure_reasons =
        vec!["matched tx actor is not a live trader-like agent role".to_string()];
    let d = arm("D", 5, 8, E4MetricSource::ChainTapeCasVerifier);

    let report = evaluate_market_performance_e4(&[a, d]);

    assert_eq!(report.verdict, E4Verdict::Proceed);
    assert!(report.e4_candidate);
    assert!(!report
        .failure_reasons
        .contains(&"e2_verifier_not_proceed".to_string()));
    assert!(!report
        .failure_reasons
        .contains(&"e2_verifier_failure_reasons_present".to_string()));
    let control = report
        .arms
        .iter()
        .find(|row| row.arm_id == "A")
        .expect("control arm row exists");
    assert_eq!(
        control.exact_join_count, 0,
        "invalid control-arm router matches must not be reported as claimable agent economic action"
    );
    assert_eq!(
        control.ev_to_action_conversion_bps, 0,
        "invalid control-arm router matches must not contribute to conversion metrics"
    );
}

#[test]
fn e4_candidate_requires_pinned_ab_hashes_and_audit_proceed() {
    let mut inputs = vec![
        arm("A", 2, 0, E4MetricSource::ChainTapeCasVerifier),
        arm("D", 5, 8, E4MetricSource::ChainTapeCasVerifier),
    ];
    inputs[1].budget_hash = "drifted-budget".to_string();

    let report = evaluate_market_performance_e4(&inputs);

    assert_eq!(report.verdict, E4Verdict::Veto);
    assert!(!report.e4_candidate);
    assert!(report
        .failure_reasons
        .contains(&"non_pinned_ab_hashes".to_string()));
}

#[test]
fn e4_candidate_rejects_dashboard_only_metrics() {
    let inputs = vec![
        arm("A", 2, 0, E4MetricSource::ChainTapeCasVerifier),
        arm("D", 5, 8, E4MetricSource::DashboardOnly),
    ];

    let report = evaluate_market_performance_e4(&inputs);

    assert_eq!(report.verdict, E4Verdict::Veto);
    assert!(!report.e4_candidate);
    assert!(report
        .failure_reasons
        .contains(&"dashboard_only_metric_source".to_string()));
}

#[test]
fn e4_candidate_rejects_market_tx_count_only_improvement() {
    let mut a = arm("A", 3, 0, E4MetricSource::ChainTapeCasVerifier);
    let mut d = arm("D", 3, 8, E4MetricSource::ChainTapeCasVerifier);
    d.verified_pput_micro = a.verified_pput_micro;
    d.wasted_attempt_count = a.wasted_attempt_count;
    d.verification_latency_ms_total = a.verification_latency_ms_total;
    d.failed_branch_count = a.failed_branch_count;
    d.cost_per_solved_proof_tokens = a.cost_per_solved_proof_tokens;
    d.role_diversity_bps = a.role_diversity_bps;
    d.market_tx_count = 10_000;
    a.market_tx_count = 0;

    let report = evaluate_market_performance_e4(&[a, d]);

    assert_eq!(report.verdict, E4Verdict::CleanNegative);
    assert!(!report.e4_candidate);
    assert!(report
        .failure_reasons
        .contains(&"market_tx_count_only_not_e4".to_string()));
}

#[test]
fn e4_candidate_detects_market_arm_behavior_improvement_with_uncertainty() {
    let inputs = vec![
        arm("A", 2, 0, E4MetricSource::ChainTapeCasVerifier),
        arm("B", 3, 2, E4MetricSource::ChainTapeCasVerifier),
        arm("C", 4, 5, E4MetricSource::ChainTapeCasVerifier),
        arm("D", 6, 9, E4MetricSource::ChainTapeCasVerifier),
    ];

    let report = evaluate_market_performance_e4(&inputs);

    assert_eq!(report.verdict, E4Verdict::Proceed);
    assert!(report.e4_candidate);
    assert_eq!(report.best_arm_id.as_deref(), Some("D"));
    assert!(report.improved_metrics.contains(&"solve_rate".to_string()));
    assert!(report
        .improved_metrics
        .contains(&"verified_pput".to_string()));
    assert!(report
        .improved_metrics
        .contains(&"wasted_attempts".to_string()));
    assert!(report
        .improved_metrics
        .contains(&"verification_latency".to_string()));
    assert!(report
        .improved_metrics
        .contains(&"role_diversity".to_string()));
    assert!(report.solve_rate_wilson_95_ci_by_arm.contains_key("A"));
    assert!(report.solve_rate_wilson_95_ci_by_arm.contains_key("D"));
}

#[test]
fn e4_markdown_is_candidate_only_and_forbids_achieved_claims() {
    let inputs = vec![
        arm("A", 2, 0, E4MetricSource::ChainTapeCasVerifier),
        arm("D", 5, 8, E4MetricSource::ChainTapeCasVerifier),
    ];
    let report = evaluate_market_performance_e4(&inputs);
    let md = report.render_markdown();

    assert!(md.contains("E4 candidate pending audit"));
    assert!(md.contains("candidate-only; not E4 achieved"));
    assert!(!md.contains("market emergence proven"));
    assert!(!md.contains("market mechanism shipped"));
}

#[test]
fn real16_cli_is_source_separated_from_dashboard_truth() {
    let src = std::fs::read_to_string("src/bin/real16_market_performance_verifier.rs")
        .unwrap_or_default();

    assert!(
        src.contains("--input-json"),
        "REAL-16 CLI must consume structured verifier input, not dashboard prose"
    );
    assert!(
        src.contains("--json-out") && src.contains("--md-out"),
        "REAL-16 CLI must emit machine and human reports"
    );
    assert!(
        !src.contains("audit_dashboard_run_report.txt"),
        "REAL-16 CLI must not parse dashboard text as source of truth"
    );
}

#[test]
fn real16_runner_pins_market_emergence_ab_arms() {
    let script = std::fs::read_to_string("scripts/run_real16_market_performance_benchmark.sh")
        .expect("REAL-16 runner exists");

    for expected in [
        "A: baseline market-visible",
        "B: EV scaffold",
        "C: EV + BCAST",
        "D: EV + BCAST + PnL + role-specialized action-conversion view",
        "sample_E1v2_hard10_S20260423.txt",
        "138f75bc3a776df11df45fed77c01565e128662ca03e3d863e68dcf8ee22d8fc",
        "REAL16_VERIFIER_INPUT.json",
        "real16_market_performance_verifier",
        "--derive-arm-json",
        "e2_verifier_verdict",
        "arm_diff_allowlist.txt",
        "TURINGOS_REAL13_TRADER_EV_SCAFFOLD",
        "TURINGOS_REAL11_MARKET_OPPORTUNITY_TRACE",
        "TURINGOS_REAL_BCAST_LIBRARIAN",
        "TURINGOS_REAL14G_ACTION_CONVERSION_VIEW",
        "LLM_PROXY_URL",
    ] {
        assert!(
            script.contains(expected),
            "REAL-16 runner must preserve contract marker: {expected}"
        );
    }

    assert!(
        !script.contains("PPUT_RESULT"),
        "REAL-16 runner must not parse evaluator stdout PPUT_RESULT for claim-bearing E4 metrics"
    );
    assert!(
        !script.contains("|| true"),
        "REAL-16 runner must not swallow verifier VETOs"
    );
}

#[test]
fn real16_runner_enables_market_opportunity_trace_for_e2_provenance() {
    let script = std::fs::read_to_string("scripts/run_real16_market_performance_benchmark.sh")
        .expect("REAL-16 runner exists");

    assert!(
        script.contains("export TURINGOS_REAL11_MARKET_OPPORTUNITY_TRACE=1"),
        "REAL-16 runner must enable MarketOpportunityTrace for every arm so exact-join buys can satisfy E2 provenance"
    );
    assert!(
        script.contains("printf 'TURINGOS_REAL11_MARKET_OPPORTUNITY_TRACE=%s\\n'"),
        "REAL-16 runner must record the MarketOpportunityTrace switch in per-arm manifests"
    );
    assert!(
        script.contains(
            "TURINGOS_REAL13_TRADER_EV_SCAFFOLD\nTURINGOS_REAL11_MARKET_OPPORTUNITY_TRACE\nTURINGOS_REAL_BCAST_LIBRARIAN"
        ),
        "REAL-16 runner must allowlist MarketOpportunityTrace in arm drift manifests"
    );
}

#[test]
fn real16_runner_forbids_unsafe_or_scripted_e4_inputs() {
    let script = std::fs::read_to_string("scripts/run_real16_market_performance_benchmark.sh")
        .expect("REAL-16 runner exists");

    for forbidden_guard in [
        "TURINGOS_REAL6B_LIVE_ATTEMPT_PREDICTION",
        "TURINGOS_REAL7_SCRIPTED_ATTEMPT_PREDICTION_FIXTURE",
        "TURINGOS_REAL7_SCRIPTED_TASK_OUTCOME_BUYS",
        "forced trade",
        "price-as-truth",
        "ghost liquidity",
        "PolicyTrader action counted as E4",
    ] {
        assert!(
            script.contains(forbidden_guard),
            "REAL-16 runner must guard forbidden mechanism: {forbidden_guard}"
        );
    }
}

#[test]
fn real16_load_bearing_files_are_trust_root_pinned() {
    let genesis =
        std::fs::read_to_string("genesis_payload.toml").expect("genesis_payload.toml exists");

    for path in [
        "src/runtime/market_performance_e4.rs",
        "src/bin/real16_market_performance_verifier.rs",
        "tests/constitution_real16_market_performance.rs",
        "scripts/run_real16_market_performance_benchmark.sh",
    ] {
        assert!(
            genesis.contains(path),
            "REAL-16 load-bearing file must be pinned in Trust Root: {path}"
        );
    }
}
