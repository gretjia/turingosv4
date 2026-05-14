//! TB-G G7 — structural run6-equivalent smoke gates.

use turingosv4::runtime::g7_structural_smoke::{evaluate_g7_structural_smoke, G7SmokeInput};

#[test]
fn sg_g7_minimum_tier_passes_with_market_visible_action() {
    let report = evaluate_g7_structural_smoke(G7SmokeInput {
        one_runtime_repo: true,
        multi_agent: true,
        persistent_state: true,
        proof_related_actions: 2,
        market_visible_actions: 1,
        no_trade_reason_count: 0,
        role_classifier_output: true,
        price_observe_only: true,
        no_price_as_truth: true,
        dashboard_regenerated: true,
    });
    assert!(
        report.minimum_tier_green,
        "report should pass: {}",
        report.render_section_k()
    );
}

#[test]
fn sg_g7_clean_negative_passes_without_market_action_when_explained() {
    let report = evaluate_g7_structural_smoke(G7SmokeInput {
        one_runtime_repo: true,
        multi_agent: true,
        persistent_state: true,
        proof_related_actions: 1,
        market_visible_actions: 0,
        no_trade_reason_count: 3,
        role_classifier_output: true,
        price_observe_only: true,
        no_price_as_truth: true,
        dashboard_regenerated: true,
    });
    assert!(report.minimum_tier_green);
    let out = report.render_section_k();
    assert!(out.contains("## §K G7 structural smoke"));
    assert!(out.contains("clean_negative: true"));
}

#[test]
fn sg_g7_missing_minimum_gate_requires_forward_stub() {
    let report = evaluate_g7_structural_smoke(G7SmokeInput {
        one_runtime_repo: true,
        multi_agent: true,
        persistent_state: false,
        proof_related_actions: 1,
        market_visible_actions: 0,
        no_trade_reason_count: 0,
        role_classifier_output: true,
        price_observe_only: true,
        no_price_as_truth: true,
        dashboard_regenerated: true,
    });
    assert!(!report.minimum_tier_green);
    let out = report.render_section_k();
    assert!(out.contains("forward_tb_stub_required: true"));
    let causes = out
        .lines()
        .filter(|line| line.trim_start().starts_with("- "))
        .count();
    assert!(causes >= 3, "§K bottleneck must list >=3 causes: {out}");
}
