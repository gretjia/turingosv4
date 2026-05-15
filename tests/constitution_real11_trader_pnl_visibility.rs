use turingosv4::economy::money::MicroCoin;
use turingosv4::runtime::agent_pnl::compute_agent_pnl;
use turingosv4::runtime::real5_roles::AgentRole;
use turingosv4::runtime::real6_conviction_budget::{
    conviction_action_allowed, derive_conviction_budget, ConvictionAction,
};
use turingosv4::sdk::your_position::render_your_position;
use turingosv4::state::q_state::{AgentId, QState};

fn agent(name: &str) -> AgentId {
    AgentId(name.into())
}

#[test]
fn trader_position_prompt_contains_balance_pnl_risk_and_autopsy_summary() {
    let trader = agent("Agent_0");
    let mut q = QState::default();
    q.economic_state_t
        .balances_t
        .0
        .insert(trader.clone(), MicroCoin::from_micro_units(700_000));

    let rendered = render_your_position(&q, &trader);

    for required in [
        "Balance:",
        "Realized PnL:",
        "Unrealized PnL:",
        "Open positions:",
        "risk_cap=",
        "source=ChainTape/CAS-derived QState fold",
        "Bankruptcy/autopsy summary:",
    ] {
        assert!(
            rendered.contains(required),
            "Trader PromptCapsule view missing required PnL/risk field {required}:\n{rendered}"
        );
    }
}

#[test]
fn pnl_summary_matches_compute_agent_pnl_and_uses_no_sidecar_source_of_truth() {
    let trader = agent("Agent_0");
    let mut q = QState::default();
    q.economic_state_t
        .balances_t
        .0
        .insert(trader.clone(), MicroCoin::from_micro_units(700_000));
    let view = compute_agent_pnl(&q, &trader, 1_000_000);
    let rendered = render_your_position(&q, &trader);

    assert_eq!(view.balance, 700_000);
    assert_eq!(view.realized_pnl, -300_000);
    assert!(rendered.contains("Balance: 700000"));
    assert!(rendered.contains("Realized PnL: -300000"));
    assert!(!rendered.contains("HashMap sidecar"));
}

#[test]
fn low_balance_trader_is_blocked_from_high_risk_market_action_but_can_read() {
    let trader = agent("Agent_0");
    let mut q = QState::default();
    q.economic_state_t
        .balances_t
        .0
        .insert(trader.clone(), MicroCoin::from_micro_units(50_000));

    let budget = derive_conviction_budget(&q, &trader);
    let risky =
        conviction_action_allowed(&budget, AgentRole::Trader, ConvictionAction::HighRiskMarket);
    let read = conviction_action_allowed(&budget, AgentRole::Trader, ConvictionAction::Read);

    assert!(
        !risky.allowed,
        "below-risk-cap Trader market action must block"
    );
    assert!(
        read.allowed,
        "below-risk-cap Trader must still be able to read/observe"
    );
}

#[test]
fn trader_view_does_not_include_raw_cot_logs_or_lean_stderr() {
    let trader = agent("Agent_0");
    let mut q = QState::default();
    q.economic_state_t
        .balances_t
        .0
        .insert(trader.clone(), MicroCoin::from_micro_units(700_000));
    let rendered = render_your_position(&q, &trader).to_ascii_lowercase();

    for forbidden in ["raw cot", "private cot", "raw log", "lean stderr", "stderr"] {
        assert!(
            !rendered.contains(forbidden),
            "Trader PnL view must not leak {forbidden}"
        );
    }
}
