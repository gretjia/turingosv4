//! TB-G G7 — structural run6-equivalent smoke gates.

use turingosv4::runtime::g7_structural_smoke::{evaluate_g7_structural_smoke, G7SmokeInput};

#[test]
fn sg_g7_minimum_tier_passes_with_market_visible_action() {
    let report = evaluate_g7_structural_smoke(G7SmokeInput {
        one_runtime_repo: true,
        multi_agent: true,
        persistent_state: true,
        agent_count: 5,
        active_role_count: 3,
        task_count: 3,
        task_outcome_market_count: 1,
        scripted_attempt_prediction_market_count: 1,
        buy_yes_router_count: 1,
        buy_no_or_short_count: 1,
        verify_tx_count: 1,
        challenge_tx_or_no_challenge_reason_count: 1,
        event_resolve_count: 1,
        pnl_delta_count: 1,
        loss_occurred: false,
        autopsy_capsule_count: 0,
        forced_live_investment: false,
        market_actions_chain_visible: true,
        no_ghost_liquidity: true,
        clean_v3_comparison: true,
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
        agent_count: 5,
        active_role_count: 3,
        task_count: 3,
        task_outcome_market_count: 1,
        scripted_attempt_prediction_market_count: 1,
        buy_yes_router_count: 0,
        buy_no_or_short_count: 0,
        verify_tx_count: 1,
        challenge_tx_or_no_challenge_reason_count: 1,
        event_resolve_count: 1,
        pnl_delta_count: 0,
        loss_occurred: false,
        autopsy_capsule_count: 0,
        forced_live_investment: false,
        market_actions_chain_visible: true,
        no_ghost_liquidity: true,
        clean_v3_comparison: true,
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
        agent_count: 5,
        active_role_count: 3,
        task_count: 3,
        task_outcome_market_count: 1,
        scripted_attempt_prediction_market_count: 1,
        buy_yes_router_count: 1,
        buy_no_or_short_count: 1,
        verify_tx_count: 1,
        challenge_tx_or_no_challenge_reason_count: 1,
        event_resolve_count: 1,
        pnl_delta_count: 1,
        loss_occurred: false,
        autopsy_capsule_count: 0,
        forced_live_investment: false,
        market_actions_chain_visible: true,
        no_ghost_liquidity: true,
        clean_v3_comparison: true,
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

#[test]
fn sg_7_new_structural_minimum_requires_v3_pressure_shape() {
    let report = evaluate_g7_structural_smoke(G7SmokeInput {
        one_runtime_repo: true,
        multi_agent: true,
        persistent_state: true,
        agent_count: 5,
        active_role_count: 3,
        task_count: 3,
        task_outcome_market_count: 1,
        scripted_attempt_prediction_market_count: 1,
        buy_yes_router_count: 1,
        buy_no_or_short_count: 1,
        verify_tx_count: 1,
        challenge_tx_or_no_challenge_reason_count: 1,
        event_resolve_count: 1,
        pnl_delta_count: 1,
        loss_occurred: true,
        autopsy_capsule_count: 1,
        forced_live_investment: false,
        market_actions_chain_visible: true,
        no_ghost_liquidity: true,
        clean_v3_comparison: true,
        proof_related_actions: 1,
        market_visible_actions: 2,
        no_trade_reason_count: 0,
        role_classifier_output: true,
        price_observe_only: true,
        no_price_as_truth: true,
        dashboard_regenerated: true,
    });
    assert!(report.minimum_tier_green, "{}", report.render_section_k());
    let out = report.render_section_k();
    for marker in [
        "agent_count: 5",
        "active_role_count: 3",
        "task_count: 3",
        "task_outcome_market_count: 1",
        "scripted_attempt_prediction_market_count: 1",
        "buy_yes_router_count: 1",
        "buy_no_or_short_count: 1",
        "verify_tx_count: 1",
        "challenge_tx_or_no_challenge_reason_count: 1",
        "event_resolve_count: 1",
        "pnl_delta_count: 1",
        "autopsy_if_loss_satisfied: true",
        "clean_v3_comparison: true",
        "does_not_claim_identical_v3_equivalence",
    ] {
        assert!(out.contains(marker), "missing marker {marker:?} in {out}");
    }
}

#[test]
fn sg_7_rejects_forced_live_investment_and_missing_buy_no() {
    let report = evaluate_g7_structural_smoke(G7SmokeInput {
        one_runtime_repo: true,
        multi_agent: true,
        persistent_state: true,
        agent_count: 5,
        active_role_count: 3,
        task_count: 3,
        task_outcome_market_count: 1,
        scripted_attempt_prediction_market_count: 1,
        buy_yes_router_count: 1,
        buy_no_or_short_count: 0,
        verify_tx_count: 1,
        challenge_tx_or_no_challenge_reason_count: 1,
        event_resolve_count: 1,
        pnl_delta_count: 1,
        loss_occurred: false,
        autopsy_capsule_count: 0,
        forced_live_investment: true,
        market_actions_chain_visible: true,
        no_ghost_liquidity: true,
        clean_v3_comparison: true,
        proof_related_actions: 1,
        market_visible_actions: 1,
        no_trade_reason_count: 0,
        role_classifier_output: true,
        price_observe_only: true,
        no_price_as_truth: true,
        dashboard_regenerated: true,
    });
    assert!(!report.minimum_tier_green);
    let out = report.render_section_k();
    assert!(out.contains("no_forced_live_investment: false"));
    assert!(out.contains("buy_no_or_short_count: 0"));
}

#[test]
fn sg_7_dashboard_safety_flags_are_not_hardcoded() {
    let dashboard = std::fs::read_to_string("src/bin/audit_dashboard.rs")
        .expect("audit_dashboard.rs is readable");
    for forbidden in [
        "forced_live_investment: false",
        "no_ghost_liquidity: true",
        "clean_v3_comparison: true",
        "role_classifier_output: true",
        "price_observe_only: true",
        "no_price_as_truth: true",
        "dashboard_regenerated: true",
    ] {
        assert!(
            !dashboard.contains(forbidden),
            "REAL-7 §K safety/equivalence guard must be derived from evidence, not hardcoded: {forbidden}"
        );
    }
}
