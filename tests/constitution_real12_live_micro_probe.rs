//! REAL-12 Atom 5/6 — live role-specialized micro-probe reporting.

use std::fs;

#[test]
fn real12_live_micro_probe_runner_uses_bull_bear_roles_and_no_scripted_buys() {
    let script = fs::read_to_string("scripts/run_real12_task_market_probe.sh")
        .expect("REAL-12 live micro-probe runner exists");
    for required in [
        "Solver,BullTrader,BearTrader,Verifier,Challenger",
        "TURINGOS_REAL5_ROLE_VIEWS=1",
        "TURINGOS_REAL6_TASK_OUTCOME_MARKET=1",
        "TURINGOS_REAL11_MARKET_OPPORTUNITY_TRACE=1",
        "TURINGOS_REAL6B_LIVE_ATTEMPT_PREDICTION=0",
        "TURINGOS_REAL11_NO_SCRIPTED_BUYS=1",
        "TURINGOS_REAL7_SCRIPTED_TASK_OUTCOME_BUYS",
        "live_non_scripted_router_tx_count",
    ] {
        assert!(script.contains(required), "runner missing {required}");
    }
}

#[test]
fn real12_live_micro_probe_records_required_bull_bear_metrics() {
    let script = fs::read_to_string("scripts/run_real12_task_market_probe.sh")
        .expect("REAL-12 live micro-probe runner exists");
    for required in [
        "real12.economic_judgment.v1",
        "real5.role_turn_trace.v1",
        "economic_judgment_coverage_ok",
        "economic_judgment_total_cas",
        "bull_judgment_count",
        "bear_judgment_count",
        "buy_yes_router_count",
        "buy_no_router_count",
        "economic_judgment_reason_distribution",
        "economic_judgment_total",
        "E2 candidate",
        "E2 NOT ACHIEVED",
    ] {
        assert!(
            script.contains(required),
            "runner missing metric {required}"
        );
    }
    for forbidden in [
        "economic_judgment_total=\"$(sum_tool economic_judgment_total)\"",
        "bull_judgment_count=\"$(sum_tool bull_judgment_count)\"",
        "bear_judgment_count=\"$(sum_tool bear_judgment_count)\"",
        "live_non_scripted_router_tx_count=\"$buy_with_coin_router\"",
    ] {
        assert!(
            !script.contains(forbidden),
            "runner must not derive canonical REAL-12 metrics from stdout/raw tx counts: {forbidden}"
        );
    }
}

