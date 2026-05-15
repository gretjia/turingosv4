use turingosv4::economy::money::MicroCoin;
use turingosv4::runtime::market_decision_trace::NoTradeReason;
use turingosv4::runtime::market_opportunity_trace::{
    derive_market_opportunity_trace, verify_one_trace_per_trader_turn, MarketOpportunityRequest,
};
use turingosv4::runtime::real5_roles::AgentRole;
use turingosv4::state::q_state::{AgentId, CpmmPool, LpShareAmount, PoolStatus, QState, TaskId};
use turingosv4::state::typed_tx::{EventId, ShareAmount};

fn agent(name: &str) -> AgentId {
    AgentId(name.into())
}

fn active_pool(event: EventId) -> CpmmPool {
    CpmmPool {
        event_id: event,
        pool_yes: ShareAmount::from_units(1_000_000),
        pool_no: ShareAmount::from_units(1_000_000),
        lp_total_shares: LpShareAmount::from_units(1_000_000),
        status: PoolStatus::Active,
    }
}

fn request(agent_id: AgentId, task: &str) -> MarketOpportunityRequest {
    MarketOpportunityRequest {
        agent_id,
        role: AgentRole::Trader,
        task_id: TaskId(task.into()),
        head_t: "HEAD_t(test)".into(),
        router_available: true,
        market_prompt_budget_elided: false,
        prompt_capsule_cid: None,
    }
}

#[test]
fn trader_turn_with_active_pool_and_balance_has_actionable_market() {
    let mut q = QState::default();
    let trader = agent("Trader_0");
    q.economic_state_t
        .balances_t
        .0
        .insert(trader.clone(), MicroCoin::from_micro_units(1_000_000));
    let event = EventId(TaskId("task-open".into()));
    q.economic_state_t
        .cpmm_pools_t
        .0
        .insert(event.clone(), active_pool(event.clone()));

    let trace = derive_market_opportunity_trace(&q, request(trader, "task-open"));

    assert_eq!(trace.visible_markets, vec![event.clone()]);
    assert_eq!(trace.actionable_markets, vec![event]);
    assert!(trace.reason_if_no_actionable_market.is_none());
    assert_eq!(trace.available_balance.micro_units(), 1_000_000);
    assert!(trace.router_available);
}

#[test]
fn trader_turn_with_no_pool_is_specific_no_pool_not_unknown() {
    let mut q = QState::default();
    let trader = agent("Trader_0");
    q.economic_state_t
        .balances_t
        .0
        .insert(trader.clone(), MicroCoin::from_micro_units(1_000_000));

    let trace = derive_market_opportunity_trace(&q, request(trader, "task-open"));

    assert!(trace.visible_markets.is_empty());
    assert!(trace.actionable_markets.is_empty());
    assert_eq!(
        trace.reason_if_no_actionable_market,
        Some(NoTradeReason::NoPool)
    );
}

#[test]
fn trader_turn_with_pool_but_zero_balance_is_specific_balance_reason() {
    let mut q = QState::default();
    let trader = agent("Trader_0");
    q.economic_state_t
        .balances_t
        .0
        .insert(trader.clone(), MicroCoin::zero());
    let event = EventId(TaskId("task-open".into()));
    q.economic_state_t
        .cpmm_pools_t
        .0
        .insert(event.clone(), active_pool(event));

    let trace = derive_market_opportunity_trace(&q, request(trader, "task-open"));

    assert!(!trace.visible_markets.is_empty());
    assert!(trace.actionable_markets.is_empty());
    assert_eq!(
        trace.reason_if_no_actionable_market,
        Some(NoTradeReason::AmountExceedsBalance)
    );
}

#[test]
fn prompt_budget_elision_is_specific_prompt_budget_reason() {
    let mut q = QState::default();
    let trader = agent("Trader_0");
    q.economic_state_t
        .balances_t
        .0
        .insert(trader.clone(), MicroCoin::from_micro_units(1_000_000));
    let event = EventId(TaskId("task-open".into()));
    q.economic_state_t
        .cpmm_pools_t
        .0
        .insert(event, active_pool(EventId(TaskId("task-open".into()))));
    let mut req = request(trader, "task-open");
    req.market_prompt_budget_elided = true;

    let trace = derive_market_opportunity_trace(&q, req);

    assert!(!trace.visible_markets.is_empty());
    assert!(trace.actionable_markets.is_empty());
    assert_eq!(
        trace.reason_if_no_actionable_market,
        Some(NoTradeReason::PromptBudgetExceeded)
    );
}

#[test]
fn every_trader_turn_has_exactly_one_market_opportunity_trace() {
    let traces = vec![
        ("turn-1".to_string(), "cid:aaa".to_string()),
        ("turn-2".to_string(), "cid:bbb".to_string()),
    ];
    verify_one_trace_per_trader_turn(2, &traces).expect("one trace per trader turn");
    assert!(verify_one_trace_per_trader_turn(2, &traces[..1]).is_err());
}

#[test]
fn trace_serialization_contains_no_raw_prompt_completion_cot_or_logs() {
    let q = QState::default();
    let trace = derive_market_opportunity_trace(&q, request(agent("Trader_0"), "task-open"));
    let json = serde_json::to_string(&trace).expect("serializes");
    for forbidden in [
        "raw_prompt",
        "raw_completion",
        "private_cot",
        "stderr",
        "raw_log",
    ] {
        assert!(
            !json.to_ascii_lowercase().contains(forbidden),
            "MarketOpportunityTrace must not expose {forbidden}: {json}"
        );
    }
}
