//! REAL-13B — Market Review Turn sidecar gates.

use tempfile::TempDir;
use turingosv4::bottom_white::cas::schema::{Cid, ObjectType};
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::market_review::{
    deterministic_response_order, validate_market_review_response, validate_market_review_summary,
    write_market_review_response_to_cas, write_market_review_summary_to_cas,
    write_market_review_window_to_cas, MarketReviewMode, MarketReviewResponse, MarketReviewSummary,
    MarketReviewWindow, MARKET_REVIEW_RESPONSE_SCHEMA_ID, MARKET_REVIEW_SUMMARY_SCHEMA_ID,
    MARKET_REVIEW_WINDOW_SCHEMA_ID,
};
use turingosv4::runtime::real5_roles::AgentRole;
use turingosv4::state::q_state::{AgentId, TaskId, TxId};
use turingosv4::state::typed_tx::EventId;

fn response(agent: &str, cid: Option<&str>) -> MarketReviewResponse {
    MarketReviewResponse {
        window_id: TxId("window-1".into()),
        response_id: format!("response-{agent}"),
        agent_id: AgentId(agent.into()),
        role: AgentRole::BullTrader,
        ev_decision_trace_cid: cid.map(|s| Cid::from_content(s.as_bytes())),
        no_response_trace_cid: None,
        action: "Abstain".into(),
        submitted_tx_id: None,
    }
}

#[test]
fn sequential_market_review_window_is_default_and_deterministic() {
    let window = MarketReviewWindow {
        window_id: TxId("window-1".into()),
        event_id: EventId(TaskId("task-1".into())),
        opened_at_head_t: "HEAD-1".into(),
        market_snapshot_cid: Cid::from_content(b"market-snapshot"),
        eligible_agents: vec![AgentId("Agent_2".into()), AgentId("Agent_1".into())],
        deadline_logical_t: 42,
        mode: MarketReviewMode::SequentialRound,
    };
    assert_eq!(window.mode, MarketReviewMode::SequentialRound);

    let ordered = deterministic_response_order(vec![
        response("Agent_2", Some("cid-2")),
        response("Agent_1", Some("cid-1")),
    ]);
    assert_eq!(ordered[0].agent_id.0, "Agent_1");
    assert_eq!(ordered[1].agent_id.0, "Agent_2");
}

#[test]
fn every_market_review_response_needs_ev_or_no_response_trace() {
    let ok = response("Agent_1", Some("ev-cid"));
    validate_market_review_response(&ok).unwrap();

    let missing = response("Agent_1", None);
    assert!(validate_market_review_response(&missing)
        .unwrap_err()
        .contains("EVDecisionTrace"));
}

#[test]
fn market_review_summary_counts_must_match_response_cids() {
    let summary = MarketReviewSummary {
        window_id: TxId("window-1".into()),
        event_id: EventId(TaskId("task-1".into())),
        response_count: 2,
        buy_count: 0,
        short_count: 0,
        abstain_count: 2,
        missing_count: 0,
        response_cids: vec![
            Cid::from_content(b"response-1"),
            Cid::from_content(b"response-2"),
        ],
        committed_tx_ids: vec![],
    };
    validate_market_review_summary(&summary).unwrap();

    let mut invalid = summary;
    invalid.response_count = 3;
    assert!(validate_market_review_summary(&invalid)
        .unwrap_err()
        .contains("response_count"));
}

#[test]
fn full_async_mode_is_explicitly_unsafe_only() {
    assert!(MarketReviewMode::FullAsyncExperimental.requires_unsafe_research());
    assert!(!MarketReviewMode::SequentialRound.requires_unsafe_research());
    assert!(!MarketReviewMode::BarrieredAsync.requires_unsafe_research());
}

#[test]
fn market_review_sidecars_are_generic_cas_backed() {
    let tmp = TempDir::new().unwrap();
    let mut cas = CasStore::open(tmp.path()).unwrap();
    let window = MarketReviewWindow {
        window_id: TxId("window-cas".into()),
        event_id: EventId(TaskId("task-cas".into())),
        opened_at_head_t: "HEAD-cas".into(),
        market_snapshot_cid: Cid::from_content(b"snapshot"),
        eligible_agents: vec![AgentId("Agent_1".into())],
        deadline_logical_t: 7,
        mode: MarketReviewMode::SequentialRound,
    };
    let response = response("Agent_1", Some("ev-cid"));
    let summary = MarketReviewSummary {
        window_id: TxId("window-cas".into()),
        event_id: EventId(TaskId("task-cas".into())),
        response_count: 1,
        buy_count: 0,
        short_count: 0,
        abstain_count: 1,
        missing_count: 0,
        response_cids: vec![Cid::from_content(b"response-cid")],
        committed_tx_ids: vec![],
    };

    let window_cid = write_market_review_window_to_cas(&mut cas, &window, "window", 1).unwrap();
    let response_cid =
        write_market_review_response_to_cas(&mut cas, &response, "response", 2).unwrap();
    let summary_cid = write_market_review_summary_to_cas(&mut cas, &summary, "summary", 3).unwrap();

    for (cid, schema) in [
        (window_cid, MARKET_REVIEW_WINDOW_SCHEMA_ID),
        (response_cid, MARKET_REVIEW_RESPONSE_SCHEMA_ID),
        (summary_cid, MARKET_REVIEW_SUMMARY_SCHEMA_ID),
    ] {
        let meta = cas.metadata(&cid).expect("metadata");
        assert_eq!(meta.object_type, ObjectType::Generic);
        assert_eq!(meta.schema_id.as_deref(), Some(schema));
    }
}
