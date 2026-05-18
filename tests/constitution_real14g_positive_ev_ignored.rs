//! REAL-14G — PositiveEVIgnored / action-conversion gates.
//!
//! These tests pin the next bottleneck after REAL-14F: public EV basis is
//! present, PolicyTrader sees positive EV, but the live trader abstains. The
//! summary must be reconstructed from CAS PolicyTraderTrace + EVDecisionTrace
//! sidecars, not dashboard text, and must preserve the no-forced-trade boundary.

use tempfile::TempDir;
use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::economy::money::MicroCoin;
use turingosv4::runtime::ev_decision_trace::{
    write_ev_decision_trace_to_cas, EVAction, EVDecisionTrace, EVReason,
    EV_DECISION_TRACE_SCHEMA_ID,
};
use turingosv4::runtime::policy_trader_trace::{
    write_policy_trader_trace_to_cas, PolicyTraderComparison, PolicyTraderTrace,
    POLICY_TRADER_TRACE_SCHEMA_ID,
};
use turingosv4::runtime::positive_ev_ignored::{
    summarize_positive_ev_ignored_from_cas, PositiveEVIgnoredBucket,
};
use turingosv4::runtime::real5_roles::{AgentRole, MarketSide, RationalPrice};
use turingosv4::state::q_state::{AgentId, TaskId};
use turingosv4::state::typed_tx::EventId;

fn ev_trace() -> EVDecisionTrace {
    EVDecisionTrace {
        schema_version: EV_DECISION_TRACE_SCHEMA_ID.to_string(),
        review_window_id: "real14g-window".into(),
        review_response_id: "real14g-response".into(),
        run_id: "real14g-run".into(),
        batch_id: "real14g-batch".into(),
        agent_id: AgentId("Agent_bull".into()),
        role: AgentRole::BullTrader,
        task_id: TaskId("task-real14g".into()),
        event_id: EventId(TaskId("task-real14g".into())),
        side: MarketSide::Yes,
        quoted_price: Some(RationalPrice::new(4, 10).unwrap()),
        implied_probability_bps: Some(4000),
        agent_probability_bps: Some(6500),
        edge_bps: Some(2500),
        expected_value_micro: Some(250),
        amount: Some(MicroCoin::from_micro_units(1_000)),
        max_risk: MicroCoin::from_micro_units(1_000),
        available_balance: MicroCoin::from_micro_units(100_000),
        risk_cap: MicroCoin::from_micro_units(10_000),
        liquidity_depth: Some(MicroCoin::from_micro_units(1_000_000)),
        slippage_bps: Some(0),
        risk_cap_triggered: false,
        action: EVAction::Abstain,
        reason: EVReason::PositiveEVIgnored,
        prompt_capsule_cid: Cid::from_content(b"prompt-capsule-real14g"),
        market_snapshot_cid: Cid::from_content(b"market-snapshot-real14g"),
        model_assignment_cid: None,
        model_family: Some("gpt-5.5".into()),
        private_alpha_cid: None,
        tool_result_cid: None,
        parent_state_root: "root-real14g".into(),
        created_at_head_t: "HEAD-real14g".into(),
        public_summary: "public positive EV abstain with complete basis".into(),
    }
}

fn policy_trace(source_ev_decision_trace_cid: Cid) -> PolicyTraderTrace {
    PolicyTraderTrace {
        schema_version: POLICY_TRADER_TRACE_SCHEMA_ID.to_string(),
        source_ev_decision_trace_cid,
        prompt_capsule_cid: Cid::from_content(b"prompt-capsule-real14g"),
        market_snapshot_cid: Cid::from_content(b"market-snapshot-real14g"),
        policy_probability_bps: Some(6500),
        implied_probability_bps: Some(4000),
        policy_edge_bps: Some(2500),
        policy_expected_value_micro: Some(250),
        counterfactual_only: true,
        counts_for_e2: false,
        comparison: PolicyTraderComparison::PolicyPositiveEV_LLMAbstained,
        gateway_blocked: false,
        policy_public_summary:
            "PolicyTrader counterfactual comparison=PolicyPositiveEV_LLMAbstained threshold_bps=0"
                .into(),
    }
}

#[test]
fn positive_ev_ignored_summary_reconstructs_rows_from_policy_and_ev_cas() {
    let tmp = TempDir::new().unwrap();
    let mut cas = CasStore::open(tmp.path()).unwrap();
    let ev = ev_trace();
    let ev_cid = write_ev_decision_trace_to_cas(&mut cas, &ev, "ignored", 1).unwrap();
    let policy = policy_trace(ev_cid.clone());
    let policy_cid = write_policy_trader_trace_to_cas(&mut cas, &policy, "ignored", 2).unwrap();

    let summary = summarize_positive_ev_ignored_from_cas(&cas).unwrap();

    assert_eq!(summary.policy_positive_ev_count, 1);
    assert_eq!(summary.ignored_count, 1);
    assert_eq!(summary.executed_positive_ev_count, 0);
    assert_eq!(summary.action_conversion_rate_bps, 0);
    assert_eq!(summary.unknown_count, 0);
    assert_eq!(
        summary
            .by_bucket
            .get(&PositiveEVIgnoredBucket::ModelAbstentionDespiteClearBasis),
        Some(&1)
    );
    assert_eq!(summary.rows.len(), 1);
    let row = &summary.rows[0];
    assert_eq!(row.source_policy_trader_trace_cid, policy_cid.to_string());
    assert_eq!(row.source_ev_decision_trace_cid, ev_cid.to_string());
    assert_eq!(row.agent_id, "Agent_bull");
    assert_eq!(row.role, "BullTrader");
    assert_eq!(row.side, "Yes");
    assert_eq!(row.action, "Abstain");
    assert_eq!(row.ev_reason, "PositiveEVIgnored");
    assert_eq!(row.amount_micro, Some(1_000));
    assert_eq!(row.implied_probability_bps, Some(4000));
    assert_eq!(row.policy_probability_bps, Some(6500));
    assert_eq!(row.policy_edge_bps, Some(2500));
    assert_eq!(
        row.bucket,
        PositiveEVIgnoredBucket::ModelAbstentionDespiteClearBasis
    );
}

#[test]
fn positive_ev_ignored_summary_computes_integer_action_conversion_rate() {
    let tmp = TempDir::new().unwrap();
    let mut cas = CasStore::open(tmp.path()).unwrap();

    let ev_ignored = ev_trace();
    let ev_ignored_cid =
        write_ev_decision_trace_to_cas(&mut cas, &ev_ignored, "ignored", 1).unwrap();
    write_policy_trader_trace_to_cas(&mut cas, &policy_trace(ev_ignored_cid), "ignored", 2)
        .unwrap();

    let mut ev_bought = ev_trace();
    ev_bought.action = EVAction::BuyYes;
    ev_bought.reason = EVReason::PositiveEV;
    ev_bought.review_window_id = "real14g-window-buy".into();
    ev_bought.review_response_id = "real14g-response-buy".into();
    let ev_bought_cid = write_ev_decision_trace_to_cas(&mut cas, &ev_bought, "bought", 3).unwrap();
    let mut policy_bought = policy_trace(ev_bought_cid);
    policy_bought.comparison = PolicyTraderComparison::BothBuy;
    write_policy_trader_trace_to_cas(&mut cas, &policy_bought, "bought", 4).unwrap();

    let summary = summarize_positive_ev_ignored_from_cas(&cas).unwrap();

    assert_eq!(summary.policy_positive_ev_count, 2);
    assert_eq!(summary.executed_positive_ev_count, 1);
    assert_eq!(summary.ignored_count, 1);
    assert_eq!(summary.action_conversion_rate_bps, 5_000);
}

#[test]
fn positive_ev_ignored_summary_fails_closed_when_source_ev_is_missing() {
    let tmp = TempDir::new().unwrap();
    let mut cas = CasStore::open(tmp.path()).unwrap();
    let missing_ev_cid = Cid::from_content(b"missing-ev-decision-trace");
    let policy = policy_trace(missing_ev_cid);
    write_policy_trader_trace_to_cas(&mut cas, &policy, "missing-ev", 1).unwrap();

    let err = summarize_positive_ev_ignored_from_cas(&cas).unwrap_err();

    assert!(
        format!("{err:?}").contains("PositiveEVIgnored source EVDecisionTrace"),
        "missing source EV must fail closed, got {err:?}"
    );
}

#[test]
fn positive_ev_ignored_bucket_taxonomy_is_exhaustive_in_summary() {
    let tmp = TempDir::new().unwrap();
    let cas = CasStore::open(tmp.path()).unwrap();

    let summary = summarize_positive_ev_ignored_from_cas(&cas).unwrap();

    for bucket in PositiveEVIgnoredBucket::all() {
        assert!(
            summary.by_bucket.contains_key(&bucket),
            "bucket {:?} must be rendered even at zero count",
            bucket
        );
    }
    assert_eq!(summary.ignored_count, 0);
}

#[test]
fn positive_ev_ignored_summary_excludes_policy_from_e2_and_private_material() {
    let tmp = TempDir::new().unwrap();
    let mut cas = CasStore::open(tmp.path()).unwrap();
    let ev = ev_trace();
    let ev_cid = write_ev_decision_trace_to_cas(&mut cas, &ev, "ignored", 1).unwrap();
    let mut policy = policy_trace(ev_cid);
    policy.policy_public_summary = "private CoT saw raw_prompt".into();

    let err = write_policy_trader_trace_to_cas(&mut cas, &policy, "bad-private", 2).unwrap_err();
    assert!(
        format!("{err:?}").contains("private/raw"),
        "PolicyTrader summary must reject raw/private material: {err:?}"
    );
}

#[test]
fn trader_view_action_conversion_text_is_optional_not_forced() {
    let evaluator = std::fs::read_to_string("experiments/minif2f_v4/src/bin/evaluator.rs").unwrap();

    for required in [
        "=== REAL-14G Action Conversion View ===",
        "You may buy when public positive EV is clear and risk checks pass.",
        "You may abstain with a public reason; abstain remains valid.",
        "missed_positive_ev_count",
        "action_conversion_rate_bps",
        "positive_ev_ignored_bucket_top",
    ] {
        assert!(
            evaluator.contains(required),
            "TraderView must expose non-forcing action-conversion guidance: {required}"
        );
    }
    for forbidden in [
        "must buy",
        "must short",
        "must trade",
        "required to buy",
        "required to short",
        "bet every turn",
    ] {
        assert!(
            !evaluator.to_ascii_lowercase().contains(forbidden),
            "TraderView must not force action: {forbidden}"
        );
    }
}

#[test]
fn dashboard_renders_positive_ev_ignored_action_conversion_metrics() {
    let dashboard = std::fs::read_to_string("src/bin/audit_dashboard.rs").unwrap();

    for required in [
        "positive_ev_ignored_total_cas",
        "positive_ev_action_conversion_rate_bps",
        "positive_ev_ignored_unknown_count",
        "positive_ev_ignored_bucket_",
        "PositiveEVIgnored Action Conversion",
    ] {
        assert!(
            dashboard.contains(required),
            "dashboard must render REAL-14G CAS-derived action-conversion metric: {required}"
        );
    }
}
