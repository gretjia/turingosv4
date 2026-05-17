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
        "E2 candidate achieved",
        "CANDIDATE_ACHIEVED_REQUIRES_AUDIT",
    ] {
        assert!(
            !script.contains(forbidden),
            "REAL-13 probe must not ship forbidden sentinel: {forbidden}"
        );
    }
    assert!(
        script.contains("E2 candidate pending audit"),
        "R14 audit CHALLENGE: candidate wording must remain pending-audit only"
    );
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

#[test]
fn dashboard_counts_e2_candidate_router_actions_by_exact_submitted_trace_join() {
    let dashboard =
        fs::read_to_string("src/bin/audit_dashboard.rs").expect("audit dashboard source exists");
    assert!(
        !dashboard.contains("router_total.min(summary.submitted_count)"),
        "R14 audit CHALLENGE: dashboard must not classify router tx via count min without exact tx_id join"
    );
    for required in [
        "submitted_market_decision_router_tx_ids",
        "matched_submitted_router_tx_id_count",
        "duplicate_router_tx_id_count",
        "duplicate_submitted_router_tx_id_count",
        "agent_economic_action_tx_count: {}",
    ] {
        assert!(
            dashboard.contains(required),
            "dashboard must expose exact provenance join field {required}"
        );
    }
}

#[test]
fn dashboard_marks_absent_g7_structural_guards_as_non_sentinel() {
    let dashboard =
        fs::read_to_string("src/bin/audit_dashboard.rs").expect("audit dashboard source exists");
    assert!(
        dashboard.contains("g7_guard_absent_interpretation"),
        "R14 audit CHALLENGE: absent G7 guard CAS must render an explicit non-sentinel/N/A annotation"
    );
}

#[test]
fn dashboard_derives_scripted_fixture_count_from_cas_not_constant() {
    let dashboard =
        fs::read_to_string("src/bin/audit_dashboard.rs").expect("audit dashboard source exists");
    assert!(
        !dashboard.contains("scripted_fixture_tx_count: {}\\n\", 0"),
        "R16 audit CHALLENGE: scripted_fixture_tx_count must not be hard-coded to zero"
    );
    assert!(
        dashboard.contains("scripted_attempt_prediction_market_count"),
        "dashboard must derive scripted fixture count from CAS-backed scripted fixture records"
    );
    assert!(
        dashboard.contains("scripted_fixture_tx_count: {}\\n\",\n        scripted_attempt_prediction_market_count"),
        "dashboard must render scripted_fixture_tx_count from the CAS-derived scripted_attempt_prediction_market_count"
    );
}
