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
    ev_decision_trace_cids, public_positive_ev_constraints_pass, read_ev_decision_trace_from_cas,
    validate_ev_decision_trace, write_ev_decision_trace_to_cas, EVAction, EVDecisionTrace,
    EVDecisionTraceSummary, EVReason, EV_DECISION_TRACE_SCHEMA_ID,
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
        quoted_price: Some(RationalPrice::new(5, 8).unwrap()),
        implied_probability_bps: Some(6250),
        agent_probability_bps: Some(7000),
        edge_bps: Some(750),
        expected_value_micro: Some(12_500),
        amount: Some(MicroCoin::from_micro_units(100_000)),
        max_risk: MicroCoin::from_micro_units(200_000),
        available_balance: MicroCoin::from_micro_units(1_000_000),
        risk_cap: MicroCoin::from_micro_units(100_000),
        liquidity_depth: Some(MicroCoin::from_micro_units(500_000)),
        slippage_bps: Some(25),
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
fn ev_decision_trace_summary_reports_public_basis_delivery() {
    let tmp = TempDir::new().unwrap();
    let mut cas = CasStore::open(tmp.path()).unwrap();

    let complete = trace(EVAction::Abstain, EVReason::NegativeEV);
    write_ev_decision_trace_to_cas(&mut cas, &complete, "complete", 1).unwrap();

    let mut zero_amount = trace(EVAction::Abstain, EVReason::LiquidityTooLow);
    zero_amount.amount = Some(MicroCoin::from_micro_units(0));
    write_ev_decision_trace_to_cas(&mut cas, &zero_amount, "zero-amount", 2).unwrap();

    let mut zero_liquidity = trace(EVAction::Abstain, EVReason::LiquidityTooLow);
    zero_liquidity.liquidity_depth = Some(MicroCoin::from_micro_units(0));
    write_ev_decision_trace_to_cas(&mut cas, &zero_liquidity, "zero-liquidity", 3).unwrap();

    let mut missing = trace(EVAction::Abstain, EVReason::InsufficientConfidence);
    missing.quoted_price = None;
    missing.implied_probability_bps = None;
    missing.agent_probability_bps = None;
    missing.edge_bps = None;
    missing.expected_value_micro = None;
    missing.liquidity_depth = None;
    write_ev_decision_trace_to_cas(&mut cas, &missing, "missing", 4).unwrap();

    let summary = EVDecisionTraceSummary::from_cas(&cas).unwrap();
    assert_eq!(summary.public_basis_available_count, 1);
    assert_eq!(summary.public_basis_missing_count, 3);
    assert_eq!(summary.public_basis_delivery_rate_bps, 2_500);
}

#[test]
fn ev_decision_trace_rejects_buy_with_zero_amount_or_zero_liquidity() {
    let mut zero_amount = trace(EVAction::BuyYes, EVReason::PositiveEV);
    zero_amount.amount = Some(MicroCoin::from_micro_units(0));
    assert!(validate_ev_decision_trace(&zero_amount)
        .unwrap_err()
        .contains("complete public EV basis"));

    let mut zero_liquidity = trace(EVAction::BuyYes, EVReason::PositiveEV);
    zero_liquidity.liquidity_depth = Some(MicroCoin::from_micro_units(0));
    assert!(validate_ev_decision_trace(&zero_liquidity)
        .unwrap_err()
        .contains("complete public EV basis"));
}

#[test]
fn ev_decision_trace_rejects_out_of_range_bps_and_float_markers() {
    let mut invalid = trace(EVAction::Abstain, EVReason::EdgeBelowThreshold);
    invalid.implied_probability_bps = Some(10_001);
    assert!(validate_ev_decision_trace(&invalid)
        .unwrap_err()
        .contains("implied_probability_bps"));

    let mut invalid = trace(EVAction::Abstain, EVReason::EdgeBelowThreshold);
    invalid.agent_probability_bps = Some(-1);
    assert!(validate_ev_decision_trace(&invalid)
        .unwrap_err()
        .contains("agent_probability_bps"));

    let mut invalid = trace(EVAction::Abstain, EVReason::EdgeBelowThreshold);
    invalid.slippage_bps = Some(10_001);
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
    abstain.amount = None;
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


#[test]
fn public_positive_ev_constraints_pass_requires_edge_over_threshold() {
    let trace = trace(EVAction::Abstain, EVReason::NegativeEV);

    assert!(public_positive_ev_constraints_pass(
        trace.edge_bps,
        trace.expected_value_micro,
        trace.amount,
        trace.available_balance,
        trace.risk_cap,
        trace.liquidity_depth,
        trace.risk_cap_triggered,
        100,
    ));
    assert!(!public_positive_ev_constraints_pass(
        Some(100),
        Some(1),
        trace.amount,
        trace.available_balance,
        trace.risk_cap,
        trace.liquidity_depth,
        trace.risk_cap_triggered,
        100,
    ));
}

#[test]
fn public_positive_ev_constraints_pass_rejects_missing_or_blocked_basis() {
    let trace = trace(EVAction::Abstain, EVReason::NegativeEV);
    let zero = Some(MicroCoin::from_micro_units(0));
    let amount = trace.amount;

    for candidate in [
        (
            None,
            trace.available_balance,
            trace.risk_cap,
            trace.liquidity_depth,
            false,
        ),
        (
            zero,
            trace.available_balance,
            trace.risk_cap,
            trace.liquidity_depth,
            false,
        ),
        (
            amount,
            MicroCoin::from_micro_units(99_999),
            trace.risk_cap,
            trace.liquidity_depth,
            false,
        ),
        (
            amount,
            trace.available_balance,
            MicroCoin::from_micro_units(99_999),
            trace.liquidity_depth,
            false,
        ),
        (
            amount,
            trace.available_balance,
            trace.risk_cap,
            Some(MicroCoin::from_micro_units(99_999)),
            false,
        ),
        (
            amount,
            trace.available_balance,
            trace.risk_cap,
            trace.liquidity_depth,
            true,
        ),
    ] {
        assert!(!public_positive_ev_constraints_pass(
            trace.edge_bps,
            trace.expected_value_micro,
            candidate.0,
            candidate.1,
            candidate.2,
            candidate.3,
            candidate.4,
            0,
        ));
    }
}

#[test]
fn positive_ev_abstain_classifier_does_not_take_declared_ev_sign() {
    let trace = trace(EVAction::Abstain, EVReason::NegativeEV);

    assert!(public_positive_ev_constraints_pass(
        trace.edge_bps,
        trace.expected_value_micro,
        trace.amount,
        trace.available_balance,
        trace.risk_cap,
        trace.liquidity_depth,
        trace.risk_cap_triggered,
        0,
    ));
}


#[test]
fn real13_runner_enables_public_ev_scaffold_by_default() {
    let runner = std::fs::read_to_string("scripts/run_real13_market_pressure_probe.sh").unwrap();

    assert!(
        runner.contains(
            "export TURINGOS_REAL13_TRADER_EV_SCAFFOLD=\"${TURINGOS_REAL13_TRADER_EV_SCAFFOLD:-1}\""
        ),
        "REAL-14F hard evidence runner must enable the public EV scaffold by default while still allowing explicit override"
    );
    assert!(
        runner.contains("TURINGOS_REAL13_TRADER_EV_SCAFFOLD=$TURINGOS_REAL13_TRADER_EV_SCAFFOLD"),
        "REAL-13 report sentinels must record whether the public EV scaffold was enabled"
    );
}

#[test]
fn dashboard_reports_public_ev_basis_delivery_metrics() {
    let dashboard = std::fs::read_to_string("src/bin/audit_dashboard.rs").unwrap();

    for required in [
        "ev_public_basis_available_count",
        "ev_public_basis_missing_count",
        "ev_public_basis_delivery_rate_bps",
    ] {
        assert!(
            dashboard.contains(required),
            "audit dashboard must report CAS-derived EV basis delivery metric: {required}"
        );
    }
}

#[test]
fn real13_runner_report_surfaces_ev_basis_and_policy_metrics() {
    let runner = std::fs::read_to_string("scripts/run_real13_market_pressure_probe.sh").unwrap();

    for required in [
        "ev_public_basis_available_count",
        "ev_public_basis_missing_count",
        "ev_public_basis_delivery_rate_bps",
        "policy_trader_trace_total_cas",
        "policy_positive_ev_count",
        "policy_positive_ev_llm_abstained_count",
        "policy_insufficient_public_basis_count",
        "policy_counts_for_e2",
    ] {
        assert!(
            runner.contains(required),
            "REAL-13/14F runner report must surface dashboard-derived EV basis and PolicyTrader metric: {required}"
        );
    }
}

#[test]
fn real13_runner_records_replay_config_hashes() {
    let runner = std::fs::read_to_string("scripts/run_real13_market_pressure_probe.sh").unwrap();

    for required in [
        "REAL14F_RUNTIME_CONFIG.json",
        "REAL14F_RUNTIME_CONFIG.sha256",
        "problem_set_hash",
        "model_assignment_hash",
        "budget_config_hash",
        "prompt_template_hash",
        "config_hash",
    ] {
        assert!(
            runner.contains(required),
            "REAL-14F runner must record replay/config hash field for true-problem evidence: {required}"
        );
    }
}


#[test]
fn ev_reason_taxonomy_is_exhaustive_in_summary_and_dashboard() {
    let tmp = TempDir::new().unwrap();
    let cas = CasStore::open(tmp.path()).unwrap();
    let summary = EVDecisionTraceSummary::from_cas(&cas).unwrap();

    for reason in [
        EVReason::PositiveEV,
        EVReason::NegativeEV,
        EVReason::EdgeBelowThreshold,
        EVReason::RiskCapBlocked,
        EVReason::BalanceBlocked,
        EVReason::LiquidityTooLow,
        EVReason::SlippageTooHigh,
        EVReason::ParserOrGatewayFailed,
        EVReason::WindowClosed,
        EVReason::PositiveEVIgnored,
        EVReason::InsufficientConfidence,
        EVReason::ProbabilityUncalibrated,
        EVReason::NoActionableMarket,
        EVReason::Unknown,
    ] {
        assert!(
            summary.by_reason.contains_key(&reason),
            "EVDecisionTraceSummary must include zero-count row for {reason:?}"
        );
    }

    let dashboard = std::fs::read_to_string("src/bin/audit_dashboard.rs").unwrap();
    assert!(
        dashboard.contains("PositiveEVIgnored"),
        "dashboard must render PositiveEVIgnored even when count is zero"
    );
    assert!(
        dashboard.contains("ProbabilityUncalibrated"),
        "dashboard must render ProbabilityUncalibrated even when count is zero"
    );
}
