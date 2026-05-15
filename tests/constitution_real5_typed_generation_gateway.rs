//! REAL-5 Atom 4 — typed generation gateway gates.

use turingosv4::bottom_white::ledger::rejection_evidence::RejectionClass;
use turingosv4::runtime::real5_roles::{
    legacy_tool_to_role_action, parse_role_action_json, route_role_action, AbstainPayload,
    AgentRole, RoleAction, RoleActionRoute,
};

#[test]
fn sg_r5_4_malformed_role_action_routes_parsefailed() {
    let err = parse_role_action_json(AgentRole::Trader, br#"{"tool":"invest","#)
        .expect_err("malformed JSON must reject");
    assert_eq!(err.rejection_class, RejectionClass::ParseFailed);
}

#[test]
fn sg_r5_4_role_allowed_tools_are_enforced() {
    let solver_liquidity = RoleAction::ProvideLiquidity(Default::default());
    let route = route_role_action(AgentRole::Solver, &solver_liquidity);
    assert!(matches!(route, RoleActionRoute::L4E { .. }));
    assert!(format!("{route:?}").contains("solver cannot directly emit MarketSeedTx"));

    let trader_proof = RoleAction::SubmitProof(Default::default());
    let route = route_role_action(AgentRole::Trader, &trader_proof);
    assert!(matches!(route, RoleActionRoute::L4E { .. }));

    let abstain = RoleAction::Abstain(AbstainPayload {
        reason: "no perceived edge".into(),
    });
    let route = route_role_action(AgentRole::Trader, &abstain);
    assert!(matches!(route, RoleActionRoute::CasOnly { .. }));
}

#[test]
fn sg_r5_4_legacy_proof_and_verify_tools_are_blocked_for_trader() {
    for legacy_tool in ["append", "complete", "step", "verify_peer"] {
        let action = legacy_tool_to_role_action(legacy_tool)
            .expect("legacy evaluator tool maps into REAL-5 typed action gateway");
        let route = route_role_action(AgentRole::Trader, &action);
        assert!(
            matches!(
                route,
                RoleActionRoute::L4E {
                    rejection_class: RejectionClass::PolicyViolation,
                    ..
                }
            ),
            "Trader must not route legacy tool {legacy_tool:?} to production"
        );
    }
}

#[test]
fn sg_r5_4_evaluator_wires_role_gateway_before_legacy_actions() {
    let evaluator = include_str!("../experiments/minif2f_v4/src/bin/evaluator.rs");
    assert!(evaluator.contains("real5_gate_parsed_action_for_role"));
    assert!(evaluator.contains("real5_role_policy_rejected_this_turn"));
    assert!(evaluator.contains("real5_role_policy_reject"));
}
