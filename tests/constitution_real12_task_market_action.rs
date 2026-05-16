//! REAL-12 — task-market economic-action probes.
//!
//! These gates target the REAL-11 weak point: TaskOutcomeMarket can be visible
//! and actionable, but the advertised `bid_task` action was not routed as an
//! economic action. REAL-12 keeps the action optional and non-forced; it only
//! ensures an agent following the advertised schema reaches the same typed
//! economic gateway as an `invest` action.

use std::fs;
use std::sync::{Mutex, OnceLock};

use turingosv4::bottom_white::ledger::rejection_evidence::RejectionClass;
use turingosv4::runtime::real5_roles::{
    default_allowed_tools, legacy_tool_to_role_action, route_role_action, AgentRole, RoleAction,
    RoleActionRoute,
};
use turingosv4::sdk::prompt::build_agent_prompt;
use turingosv4::sdk::protocol::parse_agent_output;

fn env_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

#[test]
fn real12_bid_task_maps_to_trader_invest_not_abstain() {
    let trader_tools = default_allowed_tools(AgentRole::Trader);
    assert!(
        trader_tools.iter().any(|tool| tool == "bid_task"),
        "Trader role assignment must advertise bid_task as an allowed task-market action"
    );

    let action = legacy_tool_to_role_action("bid_task")
        .expect("advertised bid_task schema must enter REAL-5 gateway");
    assert!(
        matches!(action, RoleAction::Invest(_)),
        "bid_task should be a task-market economic action, not {action:?}"
    );

    let route = route_role_action(AgentRole::Trader, &action);
    assert!(
        matches!(
            route,
            RoleActionRoute::L4 {
                tx_kind: "BuyWithCoinRouterTx"
            }
        ),
        "Trader bid_task must route to the economic action gateway; got {route:?}"
    );

    let solver_route = route_role_action(AgentRole::Solver, &action);
    assert!(
        matches!(
            solver_route,
            RoleActionRoute::L4E {
                rejection_class: RejectionClass::PolicyViolation,
                ..
            }
        ),
        "Solver must not gain market-action permission via bid_task"
    );
}

#[test]
fn real12_bid_task_parser_remains_integer_only_and_non_forcing() {
    let action = parse_agent_output(
        r#"<action>{"tool":"bid_task","amount":2500,"direction":"long"}</action>"#,
    )
    .expect("integer bid_task should parse");
    assert_eq!(action.tool, "bid_task");
    assert_eq!(action.amount, Some(2500));
    assert_eq!(action.direction.as_deref(), Some("long"));

    let float = parse_agent_output(
        r#"<action>{"tool":"bid_task","amount":2500.5,"direction":"long"}</action>"#,
    );
    assert!(
        float.is_err(),
        "task-market bids must reject decimal/floating amounts"
    );
}

#[test]
fn real12_task_market_prompt_affordance_is_opt_in_and_not_forced() {
    let _guard = env_lock().lock().expect("env lock");
    std::env::set_var("TURINGOS_REAL12_TASK_MARKET_AFFORDANCE", "1");
    let prompt = build_agent_prompt(
        "",
        "",
        "task-run-abc: YES=1/2 depth=100000",
        &[],
        &[],
        "Balance: 1000000 μCoin",
        "append, complete, invest, search",
        "",
        "",
        "available_balance_micro: 1000000\nrealized_pnl_micro: 0\n",
    );
    std::env::remove_var("TURINGOS_REAL12_TASK_MARKET_AFFORDANCE");

    assert!(
        prompt.contains("TaskOutcomeMarket"),
        "prompt must name task-level market affordance when enabled"
    );
    assert!(
        prompt.contains("\"tool\":\"bid_task\""),
        "prompt must preserve the advertised bid_task schema"
    );
    assert!(
        prompt.contains("optional") || prompt.contains("OPTIONAL"),
        "REAL-12 must not force trading"
    );
    assert!(
        prompt.contains("price is signal, not truth"),
        "price boundary must stay explicit"
    );
    assert!(
        !prompt.contains("must trade") && !prompt.contains("must buy"),
        "prompt must not coerce economic action"
    );
}

#[test]
fn real12_evaluator_has_bid_task_execution_arm_for_task_outcome_market() {
    let evaluator = fs::read_to_string("experiments/minif2f_v4/src/bin/evaluator.rs")
        .expect("evaluator source");
    assert!(
        evaluator.contains("\"invest\" | \"bid_task\"") || evaluator.contains("\"bid_task\" =>"),
        "advertised bid_task must have an evaluator execution arm"
    );
    assert!(
        evaluator.contains("tb_real6a_invest_task_outcome_to_router_tx"),
        "bid_task must route through the existing TaskOutcomeMarket router helper"
    );
    assert!(
        evaluator.contains("bid_task_action_emitted_this_turn"),
        "end-of-turn NoTrade classifier must not double-count bid_task as no-trade"
    );
    assert!(
        evaluator.contains("Some(\"invest\") | Some(\"bid_task\")")
            || evaluator
                .contains("parsed_tool == Some(\"invest\") || parsed_tool == Some(\"bid_task\")"),
        "REAL-5 role turn traces must classify bid_task as a market decision, not no-trade"
    );
    assert!(
        evaluator.contains("REAL-12 Role Action Boundary"),
        "Trader role prompts must include an explicit non-forcing action boundary"
    );
    assert!(
        evaluator.contains("Do not emit `step`, `append`, `complete`, `verify_peer`, or `challenge_node` while assigned Trader"),
        "Trader role boundary must block proof-style leakage in the prompt"
    );
    assert!(
        evaluator.contains("TURINGOS_REAL12_TRADER_OBJECTIVE"),
        "Trader objective must be opt-in after the no-E2 prompt objective probe"
    );
    assert!(
        evaluator.contains("=== REAL-12 Trader Objective ==="),
        "Trader prompt must state the economic objective separately from proof solving"
    );
    assert!(
        evaluator.contains("If no edge is visible, abstain with a reason"),
        "Trader objective must preserve no-forced-trade semantics"
    );
    assert!(
        evaluator.contains("Price is signal, not truth"),
        "Trader objective must preserve the price boundary"
    );
}

#[test]
fn real12_task_market_probe_runner_records_bid_task_attempts_without_forcing_trade() {
    let script = fs::read_to_string("scripts/run_real12_task_market_probe.sh")
        .expect("REAL-12 task-market probe runner exists");
    for required in [
        "TURINGOS_REAL12_TASK_MARKET_AFFORDANCE=1",
        "TURINGOS_REAL6_TASK_OUTCOME_MARKET=1",
        "TURINGOS_REAL11_MARKET_OPPORTUNITY_TRACE=1",
        "TURINGOS_REAL11_NO_SCRIPTED_BUYS=1",
        "TURINGOS_REAL7_SCRIPTED_ATTEMPT_PREDICTION_FIXTURE=0",
        "TURINGOS_REAL6B_LIVE_ATTEMPT_PREDICTION=0",
        "TURINGOS_REAL12_TRADER_OBJECTIVE",
        "bid_task_attempted",
        "invest_attempted",
        "live_non_scripted_router_tx_count",
        "No forced trade",
        "No price-as-truth",
    ] {
        assert!(script.contains(required), "runner missing {required}");
    }
}
