//! REAL-12 Atom 1 — role specialization and role-action gateway.

use turingosv4::bottom_white::ledger::rejection_evidence::RejectionClass;
use turingosv4::runtime::real5_roles::{
    default_allowed_tools, legacy_tool_to_role_action, market_bias, parse_role_action_json,
    route_role_action, AgentRole, MarketBias, MarketSide, RoleAction, RoleActionRoute,
};

#[test]
fn bull_and_bear_roles_are_parseable_and_have_market_only_tools() {
    assert_eq!(
        "BullTrader".parse::<AgentRole>().unwrap(),
        AgentRole::BullTrader
    );
    assert_eq!(
        "BearTrader".parse::<AgentRole>().unwrap(),
        AgentRole::BearTrader
    );
    assert!(AgentRole::ALL.contains(&AgentRole::BullTrader));
    assert!(AgentRole::ALL.contains(&AgentRole::BearTrader));

    assert_eq!(market_bias(AgentRole::BullTrader), MarketBias::Bull);
    assert_eq!(market_bias(AgentRole::BearTrader), MarketBias::Bear);
    assert_eq!(market_bias(AgentRole::Trader), MarketBias::Any);

    let bull_tools = default_allowed_tools(AgentRole::BullTrader);
    assert!(bull_tools.contains(&"buy_yes".to_string()));
    assert!(bull_tools.contains(&"abstain".to_string()));
    assert!(!bull_tools.contains(&"submit_proof".to_string()));
    assert!(!bull_tools.contains(&"verify_peer".to_string()));
    assert!(!bull_tools.contains(&"challenge".to_string()));

    let bear_tools = default_allowed_tools(AgentRole::BearTrader);
    assert!(bear_tools.contains(&"buy_no".to_string()));
    assert!(bear_tools.contains(&"abstain".to_string()));
    assert!(!bear_tools.contains(&"submit_proof".to_string()));
    assert!(!bear_tools.contains(&"verify_peer".to_string()));
    assert!(!bear_tools.contains(&"challenge".to_string()));
}

#[test]
fn bull_and_bear_direction_gates_allow_only_their_side() {
    let bull_yes = parse_role_action_json(
        AgentRole::BullTrader,
        br#"{"tool":"buy_yes","direction":"yes","amount":1000}"#,
    )
    .unwrap();
    assert!(matches!(
        route_role_action(AgentRole::BullTrader, &bull_yes),
        RoleActionRoute::L4 {
            tx_kind: "BuyWithCoinRouterTx"
        }
    ));

    let bull_no = parse_role_action_json(
        AgentRole::BullTrader,
        br#"{"tool":"buy_no","direction":"no","amount":1000}"#,
    )
    .unwrap();
    assert!(matches!(
        route_role_action(AgentRole::BullTrader, &bull_no),
        RoleActionRoute::L4E {
            rejection_class: RejectionClass::PolicyViolation,
            ..
        }
    ));

    let bear_no = parse_role_action_json(
        AgentRole::BearTrader,
        br#"{"tool":"buy_no","direction":"no","amount":1000}"#,
    )
    .unwrap();
    assert!(matches!(
        route_role_action(AgentRole::BearTrader, &bear_no),
        RoleActionRoute::L4 {
            tx_kind: "BuyWithCoinRouterTx"
        }
    ));

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

    assert!(
        matches!(bull_yes, RoleAction::Invest(payload) if payload.side == Some(MarketSide::Yes))
    );
    assert!(matches!(bear_no, RoleAction::Invest(payload) if payload.side == Some(MarketSide::No)));
}

#[test]
fn illegal_role_actions_route_l4e_and_bid_task_never_routes_to_proof() {
    for role in [AgentRole::BullTrader, AgentRole::BearTrader] {
        for tool in ["submit_proof", "verify_peer", "challenge"] {
            let action = legacy_tool_to_role_action(tool).unwrap();
            let route = route_role_action(role, &action);
            assert!(
                matches!(
                    route,
                    RoleActionRoute::L4E {
                        rejection_class: RejectionClass::PolicyViolation,
                        ..
                    }
                ),
                "{role:?} must not route {tool} to L4"
            );
        }
    }

    let solver_buy = parse_role_action_json(
        AgentRole::Solver,
        br#"{"tool":"buy_yes","direction":"yes","amount":1000}"#,
    )
    .unwrap();
    assert!(matches!(
        route_role_action(AgentRole::Solver, &solver_buy),
        RoleActionRoute::L4E {
            rejection_class: RejectionClass::PolicyViolation,
            ..
        }
    ));

    let legacy_bid_task = legacy_tool_to_role_action("bid_task").unwrap();
    assert!(
        matches!(legacy_bid_task, RoleAction::Invest(_)),
        "legacy bid_task remains market-only"
    );
    assert!(!matches!(legacy_bid_task, RoleAction::SubmitProof(_)));
}
