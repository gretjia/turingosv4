//! REAL-13A — EVDecisionTrace gate.
//!
//! These tests pin the architect's requirement that Bull/Bear no-trade is
//! decomposed into a CAS-backed expected-value decision trace without floats,
//! private CoT, or sequencer/TypedTx schema changes.

use tempfile::TempDir;
use turingosv4::bottom_white::cas::schema::{Cid, ObjectType};
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::economy::money::MicroCoin;
use turingosv4::runtime::ev_decision_trace::{
    ev_decision_trace_cids, read_ev_decision_trace_from_cas, validate_ev_decision_trace,
    write_ev_decision_trace_to_cas, EVAction, EVDecisionTrace, EVDecisionTraceSummary, EVReason,
    EV_DECISION_TRACE_SCHEMA_ID,
};
use turingosv4::runtime::real5_roles::{AgentRole, MarketSide, RationalPrice};
use turingosv4::state::q_state::{AgentId, TaskId};
use turingosv4::state::typed_tx::EventId;

fn trace(action: EVAction, reason: EVReason) -> EVDecisionTrace {
    EVDecisionTrace {
        schema_version: EV_DECISION_TRACE_SCHEMA_ID.to_string(),
        review_window_id: "window-1".into(),
        review_response_id: "response-1".into(),
        run_id: "real13-run".into(),
        batch_id: "real13-batch".into(),
        agent_id: AgentId("Agent_bull".into()),
        role: AgentRole::BullTrader,
        task_id: TaskId("task-real13".into()),
        event_id: EventId(TaskId("task-real13".into())),
        side: MarketSide::Yes,
        quoted_price: RationalPrice::new(5, 8).unwrap(),
        implied_probability_bps: 6250,
        agent_probability_bps: 7000,
        edge_bps: 750,
        expected_value_micro: 12_500,
        amount: MicroCoin::from_micro_units(100_000),
        max_risk: MicroCoin::from_micro_units(200_000),
        available_balance: MicroCoin::from_micro_units(1_000_000),
        risk_cap: MicroCoin::from_micro_units(100_000),
        liquidity_depth: MicroCoin::from_micro_units(500_000),
        slippage_bps: 25,
        risk_cap_triggered: false,
        action,
        reason,
        prompt_capsule_cid: Cid::from_content(b"prompt-capsule"),
        market_snapshot_cid: Cid::from_content(b"market-snapshot"),
        model_assignment_cid: Some(Cid::from_content(b"model-assignment")),
        model_family: Some("gpt".into()),
        private_alpha_cid: None,
        tool_result_cid: None,
        parent_state_root: "root-before-review".into(),
        created_at_head_t: "HEAD-real13".into(),
        public_summary: "public EV decision trace from market review window".into(),
    }
}

#[test]
fn ev_decision_trace_is_generic_cas_backed_and_round_trips() {
    let tmp = TempDir::new().unwrap();
    let mut cas = CasStore::open(tmp.path()).unwrap();
    let original = trace(EVAction::BuyYes, EVReason::PositiveEV);

    validate_ev_decision_trace(&original).unwrap();
    let cid = write_ev_decision_trace_to_cas(&mut cas, &original, "roundtrip", 13).unwrap();

    let meta = cas.metadata(&cid).expect("metadata");
    assert_eq!(meta.object_type, ObjectType::Generic);
    assert_eq!(meta.schema_id.as_deref(), Some(EV_DECISION_TRACE_SCHEMA_ID));
    assert_eq!(ev_decision_trace_cids(&cas), vec![cid.clone()]);
    assert_eq!(
        read_ev_decision_trace_from_cas(&cas, &cid).unwrap(),
        original
    );
}

#[test]
fn ev_decision_trace_summary_is_cas_derived() {
    let tmp = TempDir::new().unwrap();
    let mut cas = CasStore::open(tmp.path()).unwrap();

    let bull = trace(EVAction::BuyYes, EVReason::PositiveEV);
    let bull_cid = write_ev_decision_trace_to_cas(&mut cas, &bull, "bull", 1).unwrap();

    let mut bear = trace(EVAction::Abstain, EVReason::EdgeBelowThreshold);
    bear.role = AgentRole::BearTrader;
    bear.side = MarketSide::No;
    bear.reason = EVReason::EdgeBelowThreshold;
    let bear_cid = write_ev_decision_trace_to_cas(&mut cas, &bear, "bear", 2).unwrap();

    let summary = EVDecisionTraceSummary::from_cas(&cas).unwrap();
    assert_eq!(summary.total, 2);
    assert_eq!(summary.bull_count, 1);
    assert_eq!(summary.bear_count, 1);
    assert_eq!(summary.buy_yes_count, 1);
    assert_eq!(summary.buy_no_count, 0);
    assert_eq!(summary.abstain_count, 1);
    assert_eq!(summary.by_reason.get(&EVReason::PositiveEV), Some(&1));
    assert_eq!(
        summary.by_reason.get(&EVReason::EdgeBelowThreshold),
        Some(&1)
    );
    let cids = ev_decision_trace_cids(&cas);
    assert_eq!(cids.len(), 2);
    assert!(cids.contains(&bull_cid));
    assert!(cids.contains(&bear_cid));
}

#[test]
fn ev_decision_trace_rejects_out_of_range_bps_and_float_markers() {
    let mut invalid = trace(EVAction::Abstain, EVReason::EdgeBelowThreshold);
    invalid.implied_probability_bps = 10_001;
    assert!(validate_ev_decision_trace(&invalid)
        .unwrap_err()
        .contains("implied_probability_bps"));

    let mut invalid = trace(EVAction::Abstain, EVReason::EdgeBelowThreshold);
    invalid.agent_probability_bps = -1;
    assert!(validate_ev_decision_trace(&invalid)
        .unwrap_err()
        .contains("agent_probability_bps"));

    let mut invalid = trace(EVAction::Abstain, EVReason::EdgeBelowThreshold);
    invalid.slippage_bps = 10_001;
    assert!(validate_ev_decision_trace(&invalid)
        .unwrap_err()
        .contains("slippage_bps"));

    let json = serde_json::to_string(&trace(EVAction::Abstain, EVReason::NegativeEV)).unwrap();
    assert!(
        !json.contains("0.") && !json.contains("f64") && !json.contains("f32"),
        "EVDecisionTrace must persist integer/rational fields only: {json}"
    );
}

#[test]
fn ev_decision_trace_enforces_role_side_and_abstain_reason() {
    let mut bull_no = trace(EVAction::BuyNo, EVReason::PositiveEV);
    bull_no.side = MarketSide::No;
    assert!(validate_ev_decision_trace(&bull_no)
        .unwrap_err()
        .contains("BullTrader"));

    let mut bear_yes = trace(EVAction::BuyYes, EVReason::PositiveEV);
    bear_yes.role = AgentRole::BearTrader;
    bear_yes.side = MarketSide::Yes;
    assert!(validate_ev_decision_trace(&bear_yes)
        .unwrap_err()
        .contains("BearTrader"));

    let mut abstain = trace(EVAction::Abstain, EVReason::Unknown);
    assert!(validate_ev_decision_trace(&abstain)
        .unwrap_err()
        .contains("structured"));

    abstain.reason = EVReason::NoActionableMarket;
    abstain.amount = MicroCoin::zero();
    validate_ev_decision_trace(&abstain).unwrap();
}

#[test]
fn ev_decision_trace_rejects_private_or_raw_material() {
    let mut invalid = trace(EVAction::Abstain, EVReason::NegativeEV);
    invalid.public_summary = "private CoT says buy because raw_log showed it".into();
    assert!(validate_ev_decision_trace(&invalid)
        .unwrap_err()
        .contains("private/raw"));
}
