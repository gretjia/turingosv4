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
fn ev_trace_does_not_invent_50_50_or_zero_liquidity_for_missing_basis() {
    let evaluator = std::fs::read_to_string("experiments/minif2f_v4/src/bin/evaluator.rs").unwrap();

    for forbidden in [
        "RationalPrice::new(5_000, 10_000)",
        "real13_midpoint_probability_bps(\n        judgment.estimated_probability_band,\n        implied_probability_bps,\n    )",
        ".unwrap_or_else(|| MicroCoin::from_micro_units(0))",
    ] {
        assert!(
            !evaluator.contains(forbidden),
            "EVDecisionTrace must mark missing basis unavailable instead of inventing defaults: {forbidden}"
        );
    }
}

#[test]
fn positive_ev_abstain_with_constraints_pass_is_positive_ev_ignored() {
    let evaluator = std::fs::read_to_string("experiments/minif2f_v4/src/bin/evaluator.rs").unwrap();

    for required in [
        "public_positive_ev_constraints_pass(",
        "real13_policy_trader_threshold_bps()",
        "if action == EVAction::Abstain && public_positive_ev_detected",
        "reason = EVReason::PositiveEVIgnored",
    ] {
        assert!(
            evaluator.contains(required),
            "evaluator classifier must classify public positive EV abstain as PositiveEVIgnored \
             without depending on the agent's self-declared expected_value_sign: {required}"
        );
    }
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
fn trader_view_requires_public_ev_fields_even_for_abstain() {
    let evaluator = std::fs::read_to_string("experiments/minif2f_v4/src/bin/evaluator.rs").unwrap();

    for required in [
        "For every BullTrader/BearTrader `buy_yes`, `buy_no`, or `abstain` action, include public EV fields",
        "observed_price_num",
        "observed_price_den",
        "estimated_probability_lower_bps",
        "estimated_probability_upper_bps",
        "expected_value_sign",
        "liquidity_depth_micro",
        "candidate amount",
    ] {
        assert!(
            evaluator.contains(required),
            "TraderView must ask Bull/Bear abstains to externalize EV basis field: {required}"
        );
    }
}

#[test]
fn trader_ev_scaffold_exposes_side_specific_json_schemas() {
    let evaluator = std::fs::read_to_string("experiments/minif2f_v4/src/bin/evaluator.rs").unwrap();

    for required in [
        "TURINGOS_REAL13_TRADER_EV_SCAFFOLD",
        "=== REAL-13 Public EV Scaffold ===",
    ] {
        assert!(
            evaluator.contains(required),
            "Trader EV scaffold must expose side-specific schema token: {required}"
        );
    }
    for required in [
        "\\\"tool\\\":\\\"buy_yes\\\"",
        "\\\"tool\\\":\\\"buy_no\\\"",
        "\\\"tool\\\":\\\"abstain\\\"",
        "\\\"observed_price_num\\\":",
        "\\\"observed_price_den\\\":",
        "\\\"estimated_probability_lower_bps\\\":",
        "\\\"estimated_probability_upper_bps\\\":",
        "\\\"expected_value_sign\\\":",
        "\\\"liquidity_depth_micro\\\":",
    ] {
        assert!(
            evaluator.contains(required),
            "Trader EV scaffold must expose JSON schema token in Rust source: {required}"
        );
    }
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
fn trader_ev_scaffold_is_public_quote_only_not_price_as_truth() {
    let evaluator = std::fs::read_to_string("experiments/minif2f_v4/src/bin/evaluator.rs").unwrap();

    for required in [
        "pool.pool_yes.units",
        "pool.pool_no.units",
        "price is signal, not truth",
        "Do not copy price as probability",
        "No forced trade",
        "estimated_probability_* must be your public confidence interval",
    ] {
        assert!(
            evaluator.contains(required),
            "Trader EV scaffold must preserve public-quote/no-price-truth boundary: {required}"
        );
    }
}

#[test]
fn trader_ev_scaffold_rejects_zero_zero_probability_as_placeholder() {
    let evaluator = std::fs::read_to_string("experiments/minif2f_v4/src/bin/evaluator.rs").unwrap();

    for required in [
        "0/0 probability bands are uncalibrated, not NegativeEV",
        "Use 0/0 only when your public rationale says the outcome probability is literally zero",
        "If you cannot estimate, abstain with expected_value_sign=\\\"unknown\\\" and a public reason",
    ] {
        assert!(
            evaluator.contains(required),
            "Trader EV scaffold must prevent zero/zero probability from becoming a silent placeholder: {required}"
        );
    }
}

#[test]
fn trader_ev_scaffold_includes_probability_calibration_ladder_without_forcing_trade() {
    let evaluator = std::fs::read_to_string("experiments/minif2f_v4/src/bin/evaluator.rs").unwrap();

    for required in [
        "=== REAL-13 Probability Calibration Ladder ===",
        "Use one coarse integer bps band before deciding EV sign",
        "0-0 is allowed only for literal impossibility, not uncertainty",
        "0-1000 near-impossible",
        "1000-3000 low-confidence",
        "3000-5000 below-even",
        "5000-7000 above-even",
        "7000-9000 strong-confidence",
        "9000-10000 near-certain",
        "Calibrated abstain is allowed; do not trade unless EV is positive and risk checks pass",
        "estimated_probability_lower_bps:<calibrated_lower_bps>",
        "estimated_probability_upper_bps:<calibrated_upper_bps>",
    ] {
        assert!(
            evaluator.contains(required),
            "Trader EV scaffold must include a non-forcing probability calibration ladder: {required}"
        );
    }

    assert!(
        !evaluator.contains("<your_lower_bps_or_0>")
            && !evaluator.contains("<your_upper_bps_or_0>"),
        "Trader EV scaffold must not invite zero-zero probability as the abstain placeholder"
    );
}

#[test]
fn trader_ev_scaffold_includes_non_forcing_positive_ev_action_check() {
    let evaluator = std::fs::read_to_string("experiments/minif2f_v4/src/bin/evaluator.rs").unwrap();

    for required in [
        "=== REAL-13 Voluntary Positive EV Action Check ===",
        "midpoint_bps = (estimated_probability_lower_bps + estimated_probability_upper_bps) / 2",
        "If midpoint_bps > implied_probability_bps",
        "buy remains voluntary",
        "If you abstain despite public positive EV",
        "positive_ev_override:",
    ] {
        assert!(
            evaluator.contains(required),
            "Trader EV scaffold must make the positive-EV action handoff explicit without forcing trade: {required}"
        );
    }

    for forbidden in [
        "must buy",
        "must short",
        "required to buy",
        "required to short",
    ] {
        assert!(
            !evaluator.to_ascii_lowercase().contains(forbidden),
            "Trader EV scaffold must not force market action: {forbidden}"
        );
    }
}

#[test]
fn trader_ev_scaffold_preserves_positive_ev_sign_for_voluntary_abstain() {
    let evaluator = std::fs::read_to_string("experiments/minif2f_v4/src/bin/evaluator.rs").unwrap();

    for required in [
        "expected_value_sign is the public arithmetic EV sign, not a commitment to trade",
        "If midpoint_bps > implied_probability_bps, keep expected_value_sign=\\\"positive\\\" even when choosing voluntary abstain",
        "\\\"expected_value_sign\\\":\\\"positive|negative|zero|unknown\\\"",
        "positive_ev_override:",
    ] {
        assert!(
            evaluator.contains(required),
            "Trader EV scaffold must preserve positive EV sign even when abstain remains voluntary: {required}"
        );
    }

    for forbidden in [
        "must buy",
        "must short",
        "required to buy",
        "required to short",
        "every turn",
    ] {
        assert!(
            !evaluator.to_ascii_lowercase().contains(forbidden),
            "Positive-EV sign preservation must not force market action: {forbidden}"
        );
    }
}

#[test]
fn trader_ev_scaffold_explains_prediction_market_payoff_without_forcing_trade() {
    let evaluator = std::fs::read_to_string("experiments/minif2f_v4/src/bin/evaluator.rs").unwrap();

    for required in [
        "=== REAL-17P13 Prediction-Market Payoff Primer ===",
        "Polymarket-style analogy only; TuringOS ChainTape/CAS remains the source of truth",
        "You buy outcome shares below the full settlement value and later profit only if the selected outcome wins",
        "If your side wins, approximate profit_micro = shares_redeemed_micro - amount_spent_micro",
        "If your side loses, the spent amount is at risk",
        "Price is market-implied odds signal, not truth",
        "A clear positive EV may justify a voluntary buy_yes or buy_no, but abstain remains valid",
    ] {
        assert!(
            evaluator.contains(required),
            "Trader EV scaffold must explain prediction-market payoff mechanics without forcing trade: {required}"
        );
    }

    for forbidden in [
        "must buy",
        "must short",
        "required to buy",
        "required to short",
        "every turn",
        "price is truth",
        "price determines truth",
    ] {
        assert!(
            !evaluator.to_ascii_lowercase().contains(forbidden),
            "Prediction-market payoff primer must not force action or promote price-as-truth: {forbidden}"
        );
    }
}

#[test]
fn trader_ev_scaffold_separates_ev_arithmetic_from_voluntary_action_choice() {
    let evaluator = std::fs::read_to_string("experiments/minif2f_v4/src/bin/evaluator.rs").unwrap();

    for required in [
        "=== REAL-17P14 Two-Stage Voluntary Action Selector ===",
        "Stage 1 EVArithmeticReview: first compute public midpoint_bps, edge_bps, expected_value_micro, and risk_ok",
        "Stage 2 VoluntaryMarketActionChoice: then choose buy_yes, buy_no, or abstain according to your role",
        "Do not collapse positive EV into abstain without naming the blocking constraint",
        "If risk_ok=true and expected_value_micro>0, a role-allowed buy is the salient action, but still voluntary",
        "If you abstain when risk_ok=true and expected_value_micro>0, payload must start with positive_ev_override:",
    ] {
        assert!(
            evaluator.contains(required),
            "Trader EV scaffold must separate arithmetic review from voluntary action choice without forcing trade: {required}"
        );
    }

    for forbidden in [
        "must buy",
        "must short",
        "required to buy",
        "required to short",
        "every turn",
        "forced buy",
        "forced short",
    ] {
        assert!(
            !evaluator.to_ascii_lowercase().contains(forbidden),
            "Two-stage action selector must not force market action: {forbidden}"
        );
    }
}

#[test]
fn trader_view_includes_bounded_cross_market_opportunity_board_without_price_as_truth() {
    let evaluator = std::fs::read_to_string("experiments/minif2f_v4/src/bin/evaluator.rs").unwrap();

    for required in [
        "=== REAL-17P16 Cross-Market Opportunity Board ===",
        "Bounded QState-derived board of active market prices and depths",
        "Use this board to compare opportunity salience across visible markets before choosing a role-allowed action",
        "Board rows are price/depth signals only; ChainTape/CAS evidence remains the source of truth",
        "For BearTrader, scan NO-side rows for overpriced-success or underpriced-failure opportunities",
        "For BullTrader, scan YES-side rows for underpriced-success opportunities",
        "Board renders only when at least two active markets are visible",
        "cross_market_board_top_k",
    ] {
        assert!(
            evaluator.contains(required),
            "TraderView must expose a bounded cross-market opportunity board without forcing trades: {required}"
        );
    }

    for forbidden in [
        "must buy",
        "must short",
        "required to buy",
        "required to short",
        "price is truth",
        "price determines truth",
        "every turn",
    ] {
        assert!(
            !evaluator.to_ascii_lowercase().contains(forbidden),
            "Cross-market opportunity board must remain non-forcing and not price-as-truth: {forbidden}"
        );
    }
}

#[test]
fn trader_view_uses_hayekian_price_discovery_under_constitutional_truth_boundary() {
    let evaluator = std::fs::read_to_string("experiments/minif2f_v4/src/bin/evaluator.rs").unwrap();

    for required in [
        "=== REAL-17P18 Hayekian Price-Discovery Boundary ===",
        "Prices compress dispersed public evidence into discovery signals; they do not decide truth",
        "Constitutional boundary: ChainTape/CAS/Lean predicates remain authoritative",
        "Use price and salience to decide where to look, not what is true",
        "recommendation_observe_only=true",
        "salience_basis=ChainTape/CAS",
        "recent_positive_ev_ignored_count",
        "recent_rejection_count",
        "recent_partial_progress_count",
        "role_relevant_salience_bps",
        "salience_unknown",
    ] {
        assert!(
            evaluator.contains(required),
            "TraderView must use Hayekian price discovery only inside constitutional truth boundaries: {required}"
        );
    }

    for forbidden in [
        "must buy",
        "must short",
        "required to buy",
        "required to short",
        "price is truth",
        "price determines truth",
        "every turn",
    ] {
        assert!(
            !evaluator.to_ascii_lowercase().contains(forbidden),
            "Hayekian board guidance must not become forced trade or price-as-truth: {forbidden}"
        );
    }
}

#[test]
fn opportunity_board_reads_cas_derived_non_price_salience() {
    let evaluator = std::fs::read_to_string("experiments/minif2f_v4/src/bin/evaluator.rs").unwrap();

    for required in [
        "struct Real17P18EventSalience",
        "fn real17p18_salience_by_event",
        "ev_decision_trace_cids(&cas)",
        "ATTEMPT_TELEMETRY_SCHEMA_ID",
        "read_attempt_telemetry_from_cas",
        "recent_market_action_count",
        "recent_ev_positive_count",
        "recent_verified_success_count",
        "provenance_cid_count",
        "salience_unknown=false",
    ] {
        assert!(
            evaluator.contains(required),
            "P18 board must use CAS-derived non-price salience rather than price/depth only: {required}"
        );
    }

    for forbidden in [
        "dashboard",
        "stdout",
        "raw prompt",
        "raw completion",
        "raw CoT",
        "raw log",
    ] {
        assert!(
            !evaluator
                .to_ascii_lowercase()
                .contains(&format!("p18 {forbidden}")),
            "P18 board salience must not depend on non-authoritative or raw material: {forbidden}"
        );
    }
}

#[test]
fn opportunity_board_exposes_row_level_voluntary_action_affordance() {
    let evaluator = std::fs::read_to_string("experiments/minif2f_v4/src/bin/evaluator.rs").unwrap();

    for required in [
        "=== REAL-17P19 Row-Level Voluntary Action Affordance ===",
        "action_affordance_basis=ChainTape/CAS+QState",
        "row_action_affordance=voluntary",
        "role_allowed_action",
        "candidate_amount_micro",
        "liquidity_ok",
        "balance_ok",
        "risk_ok",
        "slippage_ok=quote_preview_available",
        "row_edge_bps=<compute_from_your_probability_band>",
        "row_expected_value_micro=<compute_integer_ev>",
        "prior_positive_ev_ignored_count",
        "row_blocking_constraints",
        "Do not trade from row salience alone",
    ] {
        assert!(
            evaluator.contains(required),
            "P19 board rows must expose voluntary row-level action affordance without forcing trade: {required}"
        );
    }

    for forbidden in [
        "must buy",
        "must short",
        "required to buy",
        "required to short",
        "price is truth",
        "price determines truth",
        "every turn",
    ] {
        assert!(
            !evaluator.to_ascii_lowercase().contains(forbidden),
            "P19 row-level action affordance must stay voluntary and non-price-as-truth: {forbidden}"
        );
    }
}

#[test]
fn opportunity_board_includes_integer_router_quote_preview_without_admission_claim() {
    let evaluator = std::fs::read_to_string("experiments/minif2f_v4/src/bin/evaluator.rs").unwrap();

    for required in [
        "=== REAL-17P20 Integer Router Quote Preview ===",
        "router_quote_basis=QState+integer_cpmm_router_quote",
        "quote preview is a signal, not an admission guarantee",
        "quote_buy_with_coin_router(",
        "QuoteDirection::BuyYes",
        "QuoteDirection::BuyNo",
        "quote_direction",
        "quoted_out_shares_micro",
        "quoted_get_shares_micro",
        "quoted_effective_price_num",
        "quoted_effective_price_den",
        "router_liquidity_warning",
        "slippage_ok=quote_preview_available",
    ] {
        assert!(
            evaluator.contains(required),
            "P20 board rows must expose integer router quote previews without admission or truth claims: {required}"
        );
    }

    for forbidden in [
        "must buy",
        "must short",
        "required to buy",
        "required to short",
        "price is truth",
        "price determines truth",
        "every turn",
    ] {
        assert!(
            !evaluator.to_ascii_lowercase().contains(forbidden),
            "P20 quote preview must stay voluntary and non-price-as-truth: {forbidden}"
        );
    }
}

#[test]
fn trader_view_requires_voluntary_market_order_ticket_without_minimum_nonzero_trade() {
    let evaluator = std::fs::read_to_string("experiments/minif2f_v4/src/bin/evaluator.rs").unwrap();

    for required in [
        "=== REAL-17P21 Voluntary MarketOrderTicket ===",
        "Each Trader turn must externalize a structured market_order_ticket",
        "amount_micro=0 is a valid voluntary abstain",
        "No minimum non-zero trade exists in the constitutional track",
        "market_order_ticket.choice",
        "market_order_ticket.selected_board_row",
        "market_order_ticket.quote_preview",
        "market_order_ticket.blocking_constraints",
        "write_market_order_ticket_to_cas_or_exit",
    ] {
        assert!(
            evaluator.contains(required),
            "P21 TraderView/wiring must require structured voluntary tickets without forced nonzero trades: {required}"
        );
    }

    for forbidden in [
        "must buy",
        "must short",
        "required to buy",
        "required to short",
        "price is truth",
        "price determines truth",
        "every turn",
    ] {
        assert!(
            !evaluator.to_ascii_lowercase().contains(forbidden),
            "P21 ticket wording must stay voluntary and non-price-as-truth: {forbidden}"
        );
    }
}

#[test]
fn bear_trader_positive_no_edge_has_symmetric_non_forcing_action_salience() {
    let evaluator = std::fs::read_to_string("experiments/minif2f_v4/src/bin/evaluator.rs").unwrap();

    for required in [
        "=== REAL-17 BearTrader NO-Side Action Conversion ===",
        "For BearTrader, positive EV on the NO side is symmetric with BullTrader YES-side positive EV",
        "`buy_no` is the candidate economic action for clear public positive EV on NO",
        "`abstain` remains valid when confidence, liquidity, balance, or risk checks do not pass",
        "Do not convert a NO-side positive EV edge into a YES-side action",
    ] {
        assert!(
            evaluator.contains(required),
            "BearTrader action-conversion scaffold must make NO-side positive EV salient without forcing a trade: {required}"
        );
    }

    for forbidden in [
        "must buy",
        "must short",
        "required to buy",
        "required to short",
        "every turn",
    ] {
        assert!(
            !evaluator.to_ascii_lowercase().contains(forbidden),
            "BearTrader action-conversion scaffold must not force market action: {forbidden}"
        );
    }
}

#[test]
fn bear_trader_no_side_prompt_defines_no_as_task_outcome_not_theorem_false() {
    let evaluator = std::fs::read_to_string("experiments/minif2f_v4/src/bin/evaluator.rs").unwrap();

    for required in [
        "BuyNo means buying the TaskOutcomeMarket NO outcome",
        "NO outcome means no accepted valid proof before the market deadline",
        "NO outcome is not a claim that the mathematical statement is false",
        "Do not require theorem falsehood before considering `buy_no`",
        "`abstain` remains valid for weak confidence, liquidity, balance, or risk checks",
    ] {
        assert!(
            evaluator.contains(required),
            "BearTrader NO-side scaffold must define NO as task-outcome risk, not theorem falsity: {required}"
        );
    }

    for forbidden in [
        "must buy",
        "must short",
        "required to buy",
        "required to short",
        "every turn",
    ] {
        assert!(
            !evaluator.to_ascii_lowercase().contains(forbidden),
            "BearTrader NO-side semantic scaffold must remain non-forcing: {forbidden}"
        );
    }
}

#[test]
fn bear_trader_no_side_prompt_calibrates_failure_probability_from_public_progress() {
    let evaluator = std::fs::read_to_string("experiments/minif2f_v4/src/bin/evaluator.rs").unwrap();

    for required in [
        "=== REAL-17P9 BearTrader NO Probability Calibration ===",
        "Use public progress signals, not theorem falsehood, when estimating NO probability",
        "repeated rejected attempts, no accepted proof, shrinking time budget, or weak solver progress may justify a NO probability band above even",
        "Do not default to 5000 bps when the visible public evidence is asymmetric",
        "`buy_no` remains voluntary; `abstain` remains valid when the edge is weak or risk checks fail",
    ] {
        assert!(
            evaluator.contains(required),
            "BearTrader NO-side calibration must make public failure-probability evidence salient without forcing a trade: {required}"
        );
    }

    for forbidden in [
        "must buy",
        "must short",
        "required to buy",
        "required to short",
        "every turn",
    ] {
        assert!(
            !evaluator.to_ascii_lowercase().contains(forbidden),
            "BearTrader NO-side calibration must remain non-forcing: {forbidden}"
        );
    }
}

#[test]
fn evaluator_classifies_zero_zero_unknown_probability_as_uncalibrated() {
    let evaluator = std::fs::read_to_string("experiments/minif2f_v4/src/bin/evaluator.rs").unwrap();

    for required in [
        "real13_probability_band_is_degenerate_zero",
        "EVReason::ProbabilityUncalibrated",
        "judgment.expected_value_sign == ExpectedValueSign::Unknown",
    ] {
        assert!(
            evaluator.contains(required),
            "Evaluator must classify 0/0 probability with unknown EV as ProbabilityUncalibrated, not NegativeEV: {required}"
        );
    }
}

#[test]
fn evaluator_emits_ev_trace_when_public_basis_is_unavailable() {
    let evaluator = std::fs::read_to_string("experiments/minif2f_v4/src/bin/evaluator.rs").unwrap();

    for forbidden in [
        "let quoted_price = judgment.observed_price?;",
        "real13_probability_midpoint_bps(judgment.estimated_probability_band)?",
        "let amount = judgment.intended_amount?;",
        "let liquidity_depth = judgment.liquidity_depth?;",
    ] {
        assert!(
            !evaluator.contains(forbidden),
            "EVDecisionTrace must not silently disappear when public EV basis is unavailable: {forbidden}"
        );
    }
    assert!(
        evaluator.contains("EVReason::InsufficientConfidence"),
        "missing public EV basis should become an explicit EVDecisionTrace reason"
    );
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
