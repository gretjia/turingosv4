//! REAL-8 Formal Market A/B Benchmark gates.
//!
//! These tests pin the runner contract before any benchmark evidence can be
//! claimed. The runner itself performs real ChainTape/CAS runs.

use std::fs;

#[test]
fn real8_runner_preserves_architect_ab_arms() {
    let script = fs::read_to_string("scripts/run_real8_market_ab_benchmark.sh")
        .expect("REAL-8 runner exists");

    for expected in [
        "A: market disabled",
        "B: market visible, no TaskOutcomeMarket",
        "C: TaskOutcomeMarket enabled",
        "D: TaskOutcomeMarket + scripted AttemptPrediction fixture",
    ] {
        assert!(
            script.contains(expected),
            "REAL-8 runner must preserve architect arm text: {expected}"
        );
    }
}

#[test]
fn real8_runner_pins_same_problem_model_and_budget_inputs() {
    let script = fs::read_to_string("scripts/run_real8_market_ab_benchmark.sh")
        .expect("REAL-8 runner exists");

    for expected in [
        "--problems <same_problem_set_manifest>",
        "--models <same_model_assignment_manifest>",
        "--budgets <same_budget_manifest>",
        "same problem set",
        "same model assignment",
        "same budgets",
        "PROBLEMS_HASH",
        "MODELS_HASH",
        "BUDGETS_HASH",
    ] {
        assert!(
            script.contains(expected),
            "REAL-8 runner must pin shared benchmark input: {expected}"
        );
    }
}

#[test]
fn real8_runner_reports_required_metrics() {
    let script = fs::read_to_string("scripts/run_real8_market_ab_benchmark.sh")
        .expect("REAL-8 runner exists");

    for expected in [
        "solve_rate",
        "verified_pput_mean",
        "false_accept_rate_mean",
        "cost_per_verified_proof_tokens",
        "market_tx_count",
        "no_trade_reason_distribution",
        "pnl_dispersion_micro",
        "role_diversity_index",
        "audit_failure_rate",
    ] {
        assert!(
            script.contains(expected),
            "REAL-8 runner must report architect metric: {expected}"
        );
    }
}

#[test]
fn real8_runner_keeps_negative_results_and_forbids_causal_overclaim() {
    let script = fs::read_to_string("scripts/run_real8_market_ab_benchmark.sh")
        .expect("REAL-8 runner exists");

    assert!(
        script.contains(
            "This report is descriptive benchmark evidence only. It does not claim causality."
        ),
        "REAL-8 report must explicitly avoid causal overclaim"
    );
    assert!(
        script.contains("Negative result is valid and documented."),
        "REAL-8 report must preserve negative results as valid evidence"
    );
    assert!(
        script.contains("undefined_no_verified_proof"),
        "REAL-8 report must retain no-verified-proof outcomes instead of fabricating cost metrics"
    );
}

#[test]
fn real8_runner_preserves_forbidden_ship_claims() {
    let script = fs::read_to_string("scripts/run_real8_market_ab_benchmark.sh")
        .expect("REAL-8 runner exists");

    for expected in [
        "no forced trades",
        "no price-as-truth",
        "no ghost liquidity",
        "no f64 economy",
        "no off-tape WAL as truth",
        "no private CoT recording",
        "no raw-log broadcast",
    ] {
        assert!(
            script.contains(expected),
            "REAL-8 report must preserve forbidden claim: {expected}"
        );
    }
}

#[test]
fn real8_task_outcome_arm_refreshes_verify_parent_after_auto_market() {
    let source = fs::read_to_string("experiments/minif2f_v4/src/bin/evaluator.rs")
        .expect("evaluator source exists");

    assert!(
        source.contains("real6_verify_parent_root_after_optional_market"),
        "REAL-8 SG-8.4 regression: TaskOutcomeMarket arms must not build VerifyTx \
         with the stale post-Work root after node-market creation mutates state"
    );
    assert!(
        source
            .matches("real6_verify_parent_root_after_optional_market")
            .count()
            >= 3,
        "both full-proof and per-tactic OMEGA paths must refresh VerifyTx parent roots"
    );
}
