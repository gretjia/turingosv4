//! Market Autonomy Lab — PolicyTrader baseline red gates.
//!
//! PolicyTrader is a deterministic, counterfactual-only baseline. It must
//! answer whether positive EV exists under public-at-turn information, while
//! never becoming E2 or live agent action evidence.

use std::path::Path;

fn policy_source() -> String {
    std::fs::read_to_string("src/runtime/policy_trader_trace.rs")
        .expect("PolicyTraderTrace module must exist")
}

#[test]
fn constitution_real13_policy_trader_trace() {
    assert!(
        Path::new("src/runtime/policy_trader_trace.rs").exists(),
        "PolicyTraderTrace must be implemented as a dedicated runtime sidecar module"
    );
    let src = policy_source();
    for required in [
        "real13.policy_trader_trace.v1",
        "PolicyTraderTrace",
        "write_policy_trader_trace_to_cas",
        "read_policy_trader_trace_from_cas",
        "ObjectType::Generic",
        "source_ev_decision_trace_cid",
        "prompt_capsule_cid",
        "market_snapshot_cid",
    ] {
        assert!(
            src.contains(required),
            "PolicyTraderTrace source must contain {required}"
        );
    }
}


#[test]
fn constitution_real13_policy_trader_integer_only() {
    let src = policy_source();
    for forbidden in ["f64", "f32", "0.", "as f"] {
        assert!(
            !src.contains(forbidden),
            "PolicyTraderTrace money/probability path must be integer-only; found {forbidden}"
        );
    }
    for required in [
        "policy_probability_bps",
        "implied_probability_bps",
        "policy_edge_bps",
        "policy_expected_value_micro",
        "/ 10_000",
    ] {
        assert!(
            src.contains(required),
            "missing integer EV field {required}"
        );
    }
}

#[test]
fn constitution_real13_policy_trader_counterfactual_not_e2() {
    let src = policy_source();
    for required in [
        "counterfactual_only",
        "counts_for_e2",
        "counts_for_e2: false",
        "policy_counts_for_e2=false",
    ] {
        assert!(
            src.contains(required),
            "PolicyTraderTrace must explicitly exclude baseline action from E2: {required}"
        );
    }

    let dashboard = std::fs::read_to_string("src/bin/audit_dashboard.rs").unwrap();
    assert!(
        !dashboard.contains("agent_economic_action_tx_count += policy")
            && !dashboard.contains("live_non_scripted_router_tx_count += policy"),
        "dashboard/report must not add PolicyTrader counts to live action metrics"
    );
}

#[test]
fn constitution_real13_policy_trader_compares_llm_ev() {
    let src = policy_source();
    for required in [
        "PolicyPositiveEV_LLMAbstained",
        "PolicyNoPositiveEV",
        "BothBuy",
        "LLMBuyPolicyNoBuy",
        "GatewayBlocked",
        "policy_positive_ev_llm_abstained_count",
        "policy_no_positive_ev_count",
    ] {
        assert!(
            src.contains(required),
            "PolicyTrader baseline must expose comparison {required}"
        );
    }
}

#[test]
fn constitution_real13_policy_trader_dashboard_report() {
    let dashboard = std::fs::read_to_string("src/bin/audit_dashboard.rs").unwrap();
    for required in [
        "policy_trader_trace_total_cas",
        "policy_positive_ev_count",
        "policy_positive_ev_llm_abstained_count",
        "policy_no_positive_ev_count",
        "policy_counts_for_e2=false",
    ] {
        assert!(
            dashboard.contains(required),
            "audit_dashboard --run-report must derive {required} from CAS"
        );
    }
}
