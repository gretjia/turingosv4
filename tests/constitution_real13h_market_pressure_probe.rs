//! REAL-13H — integrated market-pressure probe wiring.

use std::fs;

#[test]
fn real13_probe_runner_enables_ev_review_without_live_real6b_or_scripted_buys() {
    let script = fs::read_to_string("scripts/run_real13_market_pressure_probe.sh")
        .expect("REAL-13 probe runner exists");
    for required in [
        "TURINGOS_REAL13_EV_DECISION_TRACE=1",
        "TURINGOS_MARKET_REVIEW_MODE=sequential",
        "BullTrader,BearTrader,Solver,Verifier,Challenger",
        "TURINGOS_REAL12_TASK_MARKET_AFFORDANCE=1",
        "TURINGOS_REAL12_TRADER_OBJECTIVE",
        "TURINGOS_REAL6B_LIVE_ATTEMPT_PREDICTION=0",
        "TURINGOS_REAL11_NO_SCRIPTED_BUYS=1",
        "live_non_scripted_router_tx_count",
        "ev_decision_trace_total_cas",
        "market_review_summary_cas_count",
    ] {
        assert!(script.contains(required), "runner missing {required}");
    }
    for forbidden in [
        "TURINGOS_MARKET_REVIEW_MODE=full_async_experimental",
        "TURINGOS_REAL7_SCRIPTED_TASK_OUTCOME_BUYS=1",
        "live_non_scripted_router_tx_count=\"$buy_with_coin_router\"",
    ] {
        assert!(
            !script.contains(forbidden),
            "REAL-13 probe must not ship forbidden sentinel: {forbidden}"
        );
    }
}

#[test]
fn evaluator_writes_ev_decision_trace_and_market_review_sidecars_from_cas_path() {
    let evaluator = fs::read_to_string("experiments/minif2f_v4/src/bin/evaluator.rs")
        .expect("evaluator source exists");
    for required in [
        "TURINGOS_REAL13_EV_DECISION_TRACE",
        "real13_build_ev_decision_trace",
        "write_ev_decision_trace_to_cas_or_exit",
        "write_market_review_window_to_cas_or_exit",
        "write_market_review_response_to_cas_or_exit",
        "write_market_review_summary_to_cas_or_exit",
        "ev_decision_trace_total",
        "market_review_summary_total",
    ] {
        assert!(evaluator.contains(required), "evaluator missing {required}");
    }
    let real13_block = evaluator
        .split("fn real13_ev_decision_trace_enabled")
        .nth(1)
        .and_then(|tail| {
            tail.split("fn write_scheduler_decision_trace_to_cas_or_exit")
                .next()
        })
        .expect("REAL-13 helper block exists");
    for forbidden in ["std::thread::sleep", "tokio::time::sleep"] {
        assert!(
            !real13_block.contains(forbidden),
            "REAL-13 market review helper block must not use sleep timing: {forbidden}"
        );
    }
}
