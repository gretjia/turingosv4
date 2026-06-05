use std::fs;

const PARITY_SCHEMA: &str = "handover/directives/tc_prereg_2026-06-04/PARITY_SCHEMA.yaml";

fn parity_schema() -> String {
    fs::read_to_string(PARITY_SCHEMA).expect("TC-020A parity schema exists")
}

fn required_metric_markers() -> [&'static str; 12] {
    [
        "proposal_llm_calls",
        "route_llm_calls",
        "challenge_bear_llm_calls",
        "prompt_tokens",
        "completion_tokens",
        "total_model_tokens",
        "verifier_calls",
        "scheduler_ticks",
        "enumerator_ticks",
        "accepted_commits",
        "rejected_commits",
        "wall_clock_time",
    ]
}

#[test]
fn parity_schema_requires_all_compute_axes() {
    let body = parity_schema();

    assert!(body.contains("schema_id: tc_claim_c_prereg_parity_v1"));
    assert!(body.contains("finite_budget: true"));
    assert!(body.contains("required_metrics:"));

    for metric in required_metric_markers() {
        let marker = format!("- id: {metric}");
        assert!(body.contains(&marker), "missing required metric {metric}");
    }

    assert_eq!(
        body.matches("- id: ").count(),
        required_metric_markers().len()
    );
}

#[test]
fn claim_c_report_is_descriptive_when_parity_fails() {
    let body = parity_schema();
    let sensitive_proven = ["PRO", "VEN"].concat();
    let sensitive_definitive = ["DEFIN", "ITIVE"].concat();

    assert!(body.contains("parity_failure_policy:"));
    for axis in [
        "token_parity",
        "verifier_parity",
        "scheduler_parity",
        "enumerator_parity",
        "wall_clock_parity",
    ] {
        assert!(body.contains(axis), "missing parity failure axis {axis}");
    }
    assert!(body.contains("report_mode_on_failure: descriptive_only"));
    assert!(body.contains("claim_headline_on_failure: forbidden"));
    assert!(body.contains("claim_c_start_condition: all_required_axes_green"));
    assert!(!body.contains(&sensitive_proven));
    assert!(!body.contains(&sensitive_definitive));
}
