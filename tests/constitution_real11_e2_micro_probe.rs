use std::fs;

#[test]
fn real11_e2_micro_probe_runner_has_required_constraints_and_outputs() {
    let script = fs::read_to_string("scripts/run_real11_e2_micro_probe.sh")
        .expect("REAL-11 E2 micro-probe runner exists");

    for required in [
        "TURINGOS_REAL6_TASK_OUTCOME_MARKET=1",
        "TURINGOS_REAL5_ROLE_VIEWS=1",
        "TURINGOS_REAL11_MARKET_OPPORTUNITY_TRACE=1",
        "TURINGOS_REAL11_TRADER_PNL_VIEW=1",
        "TURINGOS_REAL11_NO_SCRIPTED_BUYS=1",
        "TURINGOS_REAL11_E2_MICRO_PROBE=1",
        "audit_dashboard_run_report.txt",
        "REAL11_E2_MICRO_PROBE_REPORT.md",
        "buy_with_coin_router",
        "live_non_scripted_router_tx_count",
        "agent_economic_action_tx_count",
        "MarketOpportunityTrace count",
        "Trader turn count source",
        "MarketOpportunityTrace summary",
        "NoTradeReason distribution",
    ] {
        assert!(
            script.contains(required),
            "REAL-11 micro-probe runner missing required contract text: {required}"
        );
    }
}

#[test]
fn real11_e2_micro_probe_report_uses_allowed_e2_verdict_rule() {
    let report =
        fs::read_to_string("handover/reports/REAL11_E2_MICRO_PROBE_REPORT.md").unwrap_or_default();
    if report.is_empty() {
        return;
    }

    assert!(report.contains("E2 achieved only if live_non_scripted_router_tx_count >= 1"));
    assert!(report.contains("No forced trade"));
    assert!(report.contains("No price-as-truth"));
    assert!(report.contains("MarketOpportunityTrace summary"));
    assert!(!report.contains("scripted positive-control as E2"));
}
