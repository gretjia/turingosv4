//! REAL-17P21 — voluntary MarketOrderTicket gates.
//!
//! The ticket is CAS evidence for a trader market-review turn. It can require
//! structure, but it cannot require a non-zero trade.

use turingosv4::bottom_white::cas::schema::ObjectType;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::ev_decision_trace::{
    write_ev_decision_trace_to_cas, EVAction, EVDecisionTrace, EVReason,
    EV_DECISION_TRACE_SCHEMA_ID,
};
use turingosv4::runtime::market_order_ticket::{
    read_market_order_ticket_from_cas, write_market_order_ticket_to_cas, MarketOrderTicket,
    MarketOrderTicketChoice, MARKET_ORDER_TICKET_SCHEMA_ID,
};
use turingosv4::runtime::prompt_capsule::{write_prompt_capsule_v2_to_cas, PromptCapsuleV2};
use turingosv4::runtime::real5_roles::{AgentRole, MarketSide, RationalPrice};
use turingosv4::state::q_state::{AgentId, Hash, TaskId, TxId};
use turingosv4::state::typed_tx::EventId;

fn prompt_capsule(
    cas: &mut CasStore,
    role: AgentRole,
) -> turingosv4::bottom_white::cas::schema::Cid {
    let visible_context_cid = cas
        .put(
            br#"{"visible":"p21 ticket context"}"#,
            ObjectType::Generic,
            "real17p21-visible-context",
            1,
            Some("real17p21.visible_context.v1".to_string()),
        )
        .expect("visible context");
    let capsule = PromptCapsuleV2 {
        prompt_context_hash: Hash([0x21; 32]),
        agent_id: AgentId("Agent_0".to_string()),
        role,
        view_policy_id: "real17p21-ticket-policy".to_string(),
        visible_context_cid,
        read_set: vec![visible_context_cid],
        hidden_fields_redacted: vec!["private_diagnostics".to_string()],
        system_prompt_template_hash: Hash([0x22; 32]),
        model_assignment_cid: None,
    };
    write_prompt_capsule_v2_to_cas(cas, &capsule, "real17p21", 2).expect("prompt capsule")
}

fn ev_trace(
    prompt_capsule_cid: turingosv4::bottom_white::cas::schema::Cid,
    action: EVAction,
) -> EVDecisionTrace {
    EVDecisionTrace {
        schema_version: EV_DECISION_TRACE_SCHEMA_ID.to_string(),
        review_window_id: "real17p21-window".to_string(),
        review_response_id: "real17p21-response".to_string(),
        run_id: "real17p21-run".to_string(),
        batch_id: "real17p21-batch".to_string(),
        agent_id: AgentId("Agent_0".to_string()),
        role: AgentRole::BearTrader,
        task_id: TaskId("task-real17p21".to_string()),
        event_id: EventId(TaskId("task-real17p21".to_string())),
        side: MarketSide::No,
        quoted_price: Some(RationalPrice::new(1, 2).unwrap()),
        implied_probability_bps: Some(5000),
        agent_probability_bps: Some(6500),
        edge_bps: Some(1500),
        expected_value_micro: Some(15_000),
        amount: Some(turingosv4::economy::money::MicroCoin::from_micro_units(
            100_000,
        )),
        max_risk: turingosv4::economy::money::MicroCoin::from_micro_units(100_000),
        available_balance: turingosv4::economy::money::MicroCoin::from_micro_units(1_000_000),
        risk_cap: turingosv4::economy::money::MicroCoin::from_micro_units(100_000),
        liquidity_depth: Some(turingosv4::economy::money::MicroCoin::from_micro_units(
            500_000,
        )),
        slippage_bps: Some(0),
        risk_cap_triggered: false,
        action,
        reason: match action {
            EVAction::BuyNo => EVReason::PositiveEV,
            EVAction::Abstain => EVReason::PositiveEVIgnored,
            EVAction::BuyYes => EVReason::PositiveEV,
        },
        prompt_capsule_cid,
        market_snapshot_cid: prompt_capsule_cid,
        model_assignment_cid: None,
        model_family: None,
        private_alpha_cid: None,
        tool_result_cid: None,
        parent_state_root: "state-root".to_string(),
        created_at_head_t: "HEAD_t(real17p21)".to_string(),
        public_summary: "public EV trace".to_string(),
    }
}

#[test]
fn market_order_ticket_round_trips_zero_amount_voluntary_abstain() {
    let cas_dir = tempfile::tempdir().expect("cas dir");
    let mut cas = CasStore::open(cas_dir.path()).expect("cas open");
    let prompt_capsule_cid = prompt_capsule(&mut cas, AgentRole::BearTrader);
    let ev = ev_trace(prompt_capsule_cid, EVAction::Abstain);
    let ev_cid = write_ev_decision_trace_to_cas(&mut cas, &ev, "real17p21", 3).expect("ev trace");

    let ticket = MarketOrderTicket {
        schema_version: MarketOrderTicket::SCHEMA_VERSION.to_string(),
        review_window_id: TxId("real17p21-window".to_string()),
        review_response_id: "real17p21-response".to_string(),
        agent_id: AgentId("Agent_0".to_string()),
        role: AgentRole::BearTrader,
        event_id: EventId(TaskId("task-real17p21".to_string())),
        role_allowed_action: "buy_no".to_string(),
        side: MarketSide::No,
        candidate_amount_micro: 100_000,
        final_amount_micro: 0,
        choice: MarketOrderTicketChoice::Abstain,
        quote_direction: "buy_no".to_string(),
        quoted_out_shares_micro: Some(50_000),
        quoted_get_shares_micro: Some(150_000),
        quoted_effective_price_num: Some(100_000),
        quoted_effective_price_den: Some(150_000),
        router_liquidity_warning: "low_liquidity".to_string(),
        balance_ok: true,
        risk_ok: true,
        liquidity_ok: true,
        slippage_ok: Some(true),
        edge_bps: Some(1500),
        expected_value_micro: Some(15_000),
        blocking_constraints: vec!["confidence".to_string()],
        prompt_capsule_cid,
        ev_decision_trace_cid: ev_cid,
        submitted_router_tx_id: None,
        forced_nonzero_trade: false,
        public_summary: "voluntary abstain with zero amount ticket".to_string(),
    };

    let ticket_cid =
        write_market_order_ticket_to_cas(&mut cas, &ticket, "real17p21", 4).expect("ticket write");
    let metadata = cas.metadata(&ticket_cid).expect("metadata");

    assert_eq!(metadata.object_type, ObjectType::Generic);
    assert_eq!(
        metadata.schema_id.as_deref(),
        Some(MARKET_ORDER_TICKET_SCHEMA_ID)
    );
    assert_eq!(
        read_market_order_ticket_from_cas(&cas, &ticket_cid).expect("ticket read"),
        ticket
    );
}

#[test]
fn market_order_ticket_rejects_forced_nonzero_trade_flag() {
    let cas_dir = tempfile::tempdir().expect("cas dir");
    let mut cas = CasStore::open(cas_dir.path()).expect("cas open");
    let prompt_capsule_cid = prompt_capsule(&mut cas, AgentRole::BearTrader);
    let ev = ev_trace(prompt_capsule_cid, EVAction::BuyNo);
    let ev_cid = write_ev_decision_trace_to_cas(&mut cas, &ev, "real17p21", 3).expect("ev trace");

    let ticket = MarketOrderTicket {
        schema_version: MarketOrderTicket::SCHEMA_VERSION.to_string(),
        review_window_id: TxId("real17p21-window".to_string()),
        review_response_id: "real17p21-response".to_string(),
        agent_id: AgentId("Agent_0".to_string()),
        role: AgentRole::BearTrader,
        event_id: EventId(TaskId("task-real17p21".to_string())),
        role_allowed_action: "buy_no".to_string(),
        side: MarketSide::No,
        candidate_amount_micro: 100_000,
        final_amount_micro: 100_000,
        choice: MarketOrderTicketChoice::BuyNo,
        quote_direction: "buy_no".to_string(),
        quoted_out_shares_micro: Some(50_000),
        quoted_get_shares_micro: Some(150_000),
        quoted_effective_price_num: Some(100_000),
        quoted_effective_price_den: Some(150_000),
        router_liquidity_warning: "low_liquidity".to_string(),
        balance_ok: true,
        risk_ok: true,
        liquidity_ok: true,
        slippage_ok: Some(true),
        edge_bps: Some(1500),
        expected_value_micro: Some(15_000),
        blocking_constraints: vec![],
        prompt_capsule_cid,
        ev_decision_trace_cid: ev_cid,
        submitted_router_tx_id: Some(TxId("router-real17p21".to_string())),
        forced_nonzero_trade: true,
        public_summary: "nonzero trade cannot be forced in constitutional track".to_string(),
    };

    let err = write_market_order_ticket_to_cas(&mut cas, &ticket, "real17p21", 4)
        .expect_err("forced nonzero ticket must fail closed");
    assert!(
        format!("{err:?}").contains("forced_nonzero_trade"),
        "unexpected error: {err:?}"
    );
}
