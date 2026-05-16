//! REAL-12 Atom 4 — Bull/Bear scripted positive control boundaries.

use std::fs;

use turingosv4::bottom_white::ledger::rejection_evidence::RejectionClass;
use turingosv4::runtime::real5_roles::{
    parse_role_action_json, route_role_action, AgentRole, RoleActionRoute,
};

#[test]
fn bull_bear_scripted_positive_control_requires_yes_and_no_l4_routes() {
    let bull_yes = parse_role_action_json(
        AgentRole::BullTrader,
        br#"{"tool":"buy_yes","direction":"yes","amount":1000}"#,
    )
    .unwrap();
    let bear_no = parse_role_action_json(
        AgentRole::BearTrader,
        br#"{"tool":"buy_no","direction":"no","amount":1000}"#,
    )
    .unwrap();

    assert!(matches!(
        route_role_action(AgentRole::BullTrader, &bull_yes),
        RoleActionRoute::L4 {
            tx_kind: "BuyWithCoinRouterTx"
        }
    ));
    assert!(
        matches!(
            route_role_action(AgentRole::BearTrader, &bear_no),
            RoleActionRoute::L4 {
                tx_kind: "BuyWithCoinRouterTx"
            }
        ),
        "Bear BuyNo success is mandatory in REAL-12 positive-control"
    );
}

#[test]
fn unsupported_or_cross_side_short_paths_are_explicit_l4e_not_silent() {
    let bear_yes = parse_role_action_json(
        AgentRole::BearTrader,
        br#"{"tool":"buy_yes","direction":"yes","amount":1000}"#,
    )
    .unwrap();
    assert!(matches!(
        route_role_action(AgentRole::BearTrader, &bear_yes),
        RoleActionRoute::L4E {
            rejection_class: RejectionClass::PolicyViolation,
            ..
        }
    ));
}

#[test]
fn positive_control_runner_marks_scripted_buys_as_not_e2() {
    let script = fs::read_to_string("scripts/run_real12_task_market_probe.sh")
        .expect("REAL-12 runner exists");
    for required in [
        "scripted_positive_control_is_not_e2",
        "buy_yes_router_count",
        "buy_no_router_count",
        "No ghost liquidity",
        "No f64/f32 money path",
    ] {
        assert!(script.contains(required), "runner missing {required}");
    }
}
