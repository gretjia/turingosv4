//! REAL-12 Atom 3 — mandatory EconomicJudgment.

use tempfile::TempDir;
use turingosv4::bottom_white::cas::schema::ObjectType;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::economy::money::MicroCoin;
use turingosv4::runtime::economic_judgment::{
    economic_judgment_cids, read_economic_judgment_from_cas, validate_economic_judgment,
    verify_bull_bear_turn_judgment_coverage, write_economic_judgment_to_cas, EconomicJudgment,
    EconomicJudgmentAction, EconomicJudgmentReasonSummary, EconomicReason, ExpectedValueSign,
    ProbabilityBand, ECONOMIC_JUDGMENT_SCHEMA_ID,
};
use turingosv4::runtime::real5_roles::{
    write_role_turn_trace_to_cas, AgentRole, MarketSide, RationalPrice, RoleTurnOutcome,
    RoleTurnTrace,
};
use turingosv4::state::q_state::{AgentId, TaskId};
use turingosv4::state::typed_tx::EventId;

fn judgment(
    role: AgentRole,
    action: EconomicJudgmentAction,
    side: Option<MarketSide>,
) -> EconomicJudgment {
    EconomicJudgment {
        schema_version: ECONOMIC_JUDGMENT_SCHEMA_ID.to_string(),
        agent_id: AgentId("Agent_econ".into()),
        role,
        task_id: TaskId("task-econ".into()),
        head_t: "HEAD-econ".into(),
        visible_markets: vec![EventId(TaskId("task-econ".into()))],
        chosen_market: Some(EventId(TaskId("task-econ".into()))),
        intended_side: side,
        intended_amount: Some(MicroCoin::from_micro_units(1000)),
        action,
        reason: EconomicReason::NoPerceivedEdge,
        observed_price: Some(RationalPrice::new(2, 5).unwrap()),
        estimated_probability_band: Some(ProbabilityBand {
            lower_bps: 6100,
            upper_bps: 7000,
        }),
        expected_value_sign: ExpectedValueSign::Positive,
        liquidity_depth: Some(MicroCoin::from_micro_units(5000)),
        balance_available: MicroCoin::from_micro_units(10000),
        risk_cap: MicroCoin::from_micro_units(100),
        oracle_or_deadline_risk: Some("deadline_round:10".into()),
        prompt_capsule_cid: Default::default(),
        public_summary: "positive edge from public task outcome market".into(),
    }
}

#[test]
fn economic_judgment_schema_is_pinned_and_generic_cas_backed() {
    let tmp = TempDir::new().unwrap();
    let mut cas = CasStore::open(tmp.path()).unwrap();
    let original = judgment(
        AgentRole::BullTrader,
        EconomicJudgmentAction::Buy,
        Some(MarketSide::Yes),
    );
    validate_economic_judgment(&original).unwrap();
    let cid = write_economic_judgment_to_cas(&mut cas, &original, "test", 12).unwrap();
    let meta = cas.metadata(&cid).expect("metadata");
    assert_eq!(meta.object_type, ObjectType::Generic);
    assert_eq!(meta.schema_id.as_deref(), Some(ECONOMIC_JUDGMENT_SCHEMA_ID));
    assert_eq!(economic_judgment_cids(&cas), vec![cid]);
    assert_eq!(
        read_economic_judgment_from_cas(&cas, &cid).unwrap(),
        original
    );
}

#[test]
fn buy_or_short_requires_positive_integer_ev_basis_and_role_side_match() {
    validate_economic_judgment(&judgment(
        AgentRole::BullTrader,
        EconomicJudgmentAction::Buy,
        Some(MarketSide::Yes),
    ))
    .expect("Bull Buy YES with positive EV basis is valid");

    validate_economic_judgment(&judgment(
        AgentRole::BearTrader,
        EconomicJudgmentAction::Short,
        Some(MarketSide::No),
    ))
    .expect("Bear Short/Buy NO with positive EV basis is valid");

    let mut no_ev = judgment(
        AgentRole::BullTrader,
        EconomicJudgmentAction::Buy,
        Some(MarketSide::Yes),
    );
    no_ev.expected_value_sign = ExpectedValueSign::Negative;
    assert!(validate_economic_judgment(&no_ev)
        .unwrap_err()
        .contains("positive EV"));

    let bull_no = judgment(
        AgentRole::BullTrader,
        EconomicJudgmentAction::Buy,
        Some(MarketSide::No),
    );
    assert!(validate_economic_judgment(&bull_no)
        .unwrap_err()
        .contains("BullTrader cannot choose NO"));

    let bear_yes = judgment(
        AgentRole::BearTrader,
        EconomicJudgmentAction::Short,
        Some(MarketSide::Yes),
    );
    assert!(validate_economic_judgment(&bear_yes)
        .unwrap_err()
        .contains("BearTrader cannot choose YES"));
}

#[test]
fn abstain_requires_structured_reason_and_no_private_cot() {
    let mut abstain = judgment(
        AgentRole::BearTrader,
        EconomicJudgmentAction::Abstain,
        Some(MarketSide::No),
    );
    abstain.chosen_market = None;
    abstain.intended_amount = None;
    abstain.expected_value_sign = ExpectedValueSign::Negative;
    abstain.reason = EconomicReason::ExpectedValueNegative;
    validate_economic_judgment(&abstain).unwrap();

    let mut unknown = abstain.clone();
    unknown.reason = EconomicReason::Unknown;
    assert!(validate_economic_judgment(&unknown)
        .unwrap_err()
        .contains("structured reason"));

    let json = serde_json::to_string(&abstain).unwrap();
    for forbidden in ["raw_prompt", "raw_completion", "private CoT", "raw_log"] {
        assert!(
            !json.contains(forbidden),
            "EconomicJudgment must not persist forbidden material: {forbidden}"
        );
    }
}

#[test]
fn bull_bear_role_turns_must_link_to_cas_backed_economic_judgments() {
    let tmp = TempDir::new().unwrap();
    let mut cas = CasStore::open(tmp.path()).unwrap();

    let mut bull = judgment(
        AgentRole::BullTrader,
        EconomicJudgmentAction::Abstain,
        Some(MarketSide::Yes),
    );
    bull.chosen_market = None;
    bull.intended_amount = None;
    bull.observed_price = None;
    bull.estimated_probability_band = None;
    bull.expected_value_sign = ExpectedValueSign::Negative;
    bull.reason = EconomicReason::NoPerceivedEdge;
    let judgment_cid = write_economic_judgment_to_cas(&mut cas, &bull, "linked", 1).unwrap();

    let trace = RoleTurnTrace::new(
        bull.agent_id.clone(),
        bull.role,
        bull.task_id.clone(),
        bull.prompt_capsule_cid,
        Some("abstain".into()),
        RoleTurnOutcome::NoTrade {
            reason: turingosv4::runtime::market_decision_trace::NoTradeReason::NoPerceivedEdge,
            public_summary: "no positive edge".into(),
        },
    )
    .with_economic_judgment_cid(judgment_cid);
    write_role_turn_trace_to_cas(&mut cas, &trace, "linked-turn", 2).unwrap();

    let coverage = verify_bull_bear_turn_judgment_coverage(&cas)
        .expect("linked Bull/Bear turn must satisfy judgment coverage");
    assert_eq!(coverage.required_trader_turns, 1);
    assert_eq!(coverage.linked_trader_turns, 1);

    let unlinked = RoleTurnTrace::new(
        AgentId("Agent_unlinked".into()),
        AgentRole::BearTrader,
        TaskId("task-econ".into()),
        Default::default(),
        Some("abstain".into()),
        RoleTurnOutcome::NoTrade {
            reason: turingosv4::runtime::market_decision_trace::NoTradeReason::NoPerceivedEdge,
            public_summary: "missing linked EconomicJudgment".into(),
        },
    );
    write_role_turn_trace_to_cas(&mut cas, &unlinked, "unlinked-turn", 3).unwrap();

    let err = verify_bull_bear_turn_judgment_coverage(&cas).unwrap_err();
    assert!(
        err.contains("missing EconomicJudgment link"),
        "coverage error should name the missing link, got: {err}"
    );
}

#[test]
fn economic_judgment_summary_is_derived_from_cas_objects_not_stdout() {
    let tmp = TempDir::new().unwrap();
    let mut cas = CasStore::open(tmp.path()).unwrap();
    let mut bull = judgment(
        AgentRole::BullTrader,
        EconomicJudgmentAction::Abstain,
        Some(MarketSide::Yes),
    );
    bull.chosen_market = None;
    bull.intended_amount = None;
    bull.observed_price = None;
    bull.estimated_probability_band = None;
    bull.expected_value_sign = ExpectedValueSign::Negative;
    bull.reason = EconomicReason::NoPerceivedEdge;
    let mut bear = bull.clone();
    bear.agent_id = AgentId("Agent_bear".into());
    bear.role = AgentRole::BearTrader;
    bear.intended_side = Some(MarketSide::No);
    bear.reason = EconomicReason::UnresolvedOracleRisk;

    write_economic_judgment_to_cas(&mut cas, &bull, "bull", 1).unwrap();
    write_economic_judgment_to_cas(&mut cas, &bear, "bear", 2).unwrap();

    let summary = EconomicJudgmentReasonSummary::from_cas(&cas).unwrap();
    assert_eq!(summary.total, 2);
    assert_eq!(summary.bull_judgment_count, 1);
    assert_eq!(summary.bear_judgment_count, 1);
    assert_eq!(summary.abstain_structured_reason_count, 2);
    assert_eq!(
        summary
            .by_reason
            .get(&EconomicReason::NoPerceivedEdge)
            .copied()
            .unwrap_or_default(),
        1
    );
    assert_eq!(
        summary
            .by_reason
            .get(&EconomicReason::UnresolvedOracleRisk)
            .copied()
            .unwrap_or_default(),
        1
    );
}

#[test]
fn evaluator_must_not_fabricate_positive_ev_basis_for_live_buys() {
    let evaluator = std::fs::read_to_string("experiments/minif2f_v4/src/bin/evaluator.rs").unwrap();
    for forbidden in [
        "RationalPrice::new(1, 2)",
        "lower_bps: 6001",
        "upper_bps: 7000",
        "ExpectedValueSign::Positive\n        } else",
        "positive-EV basis\".into()",
    ] {
        assert!(
            !evaluator.contains(forbidden),
            "REAL-12 evaluator must not fabricate buy/short EV basis with {forbidden}"
        );
    }
}
