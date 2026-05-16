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
        "abstain_reason_distribution",
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

#[test]
fn real12_role_turn_trace_uses_task_outcome_market_visibility_for_no_trade_reason() {
    let evaluator = fs::read_to_string("experiments/minif2f_v4/src/bin/evaluator.rs")
        .expect("evaluator source exists");
    assert!(
        evaluator.contains("market_context_visible")
            && evaluator
                .contains("tb_n3_market_block_present || real6_task_outcome_market_present"),
        "REAL-12 RoleTurnTrace must treat TaskOutcomeMarket visibility as market context"
    );
    assert!(
        !evaluator.contains(
            "real5_write_role_turn_trace(\n                        bundle,\n                        &run_id,\n                        agent_id,\n                        real5_prompt_role,\n                        prompt_capsule_cid,\n                        real5_parsed_tool_for_trace.as_deref(),\n                        real5_parse_error_for_trace.as_deref(),\n                        real5_role_policy_rejected_this_turn.as_deref(),\n                        tb_n3_market_block_present,"
        ),
        "RoleTurnTrace must not classify TaskOutcomeMarket-visible abstains as NoPool by passing only tb_n3_market_block_present"
    );
}
