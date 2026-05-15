//! REAL-5 Atom 6 — Trader Role Activation gates.

use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::economy::money::MicroCoin;
use turingosv4::runtime::market_decision_trace::NoTradeReason;
use turingosv4::runtime::real5_roles::{
    scripted_positive_edge_trade, verify_trader_turns, AgentRole, NoTradeReasonTrace,
    RationalPrice, TraderTurnWitness,
};
use turingosv4::state::q_state::{AgentId, TaskId};
use turingosv4::state::typed_tx::EventId;

#[test]
fn sg_r5_6_every_trader_turn_has_market_decision_or_no_trade_reason() {
    let trace = NoTradeReasonTrace {
        agent_id: AgentId("Agent_trader".into()),
        role: AgentRole::Trader,
        task_id: TaskId("task".into()),
        visible_markets: vec![EventId(TaskId("event".into()))],
        reason: NoTradeReason::NoPerceivedEdge,
        observed_price: Some(RationalPrice::new(1, 2).unwrap()),
        liquidity_depth: Some(MicroCoin::from_micro_units(10)),
        balance_available: MicroCoin::from_micro_units(100),
        prompt_capsule_cid: Cid([7; 32]),
    };
    verify_trader_turns(&[TraderTurnWitness::NoTrade(trace)]).unwrap();
}

#[test]
fn sg_r5_6_scripted_positive_edge_trader_can_produce_router_tx() {
    let route = scripted_positive_edge_trade(
        AgentId("Agent_trader".into()),
        EventId(TaskId("event".into())),
        MicroCoin::from_micro_units(100),
    )
    .unwrap();
    assert_eq!(route.tx_kind, "BuyWithCoinRouterTx");
    assert!(route.l4_or_l4e_anchor_required);
}
