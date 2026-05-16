//! REAL-BCAST-1 — half-async / barriered async replay contract.

use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::runtime::librarian_broadcast::BroadcastEpoch;
use turingosv4::runtime::market_review::{
    deterministic_response_order, validate_market_review_broadcast_contract, MarketReviewMode,
    MarketReviewResponse, MarketReviewSummary, MarketReviewWindow,
};
use turingosv4::runtime::real5_roles::AgentRole;
use turingosv4::state::q_state::{AgentId, TaskId, TxId};
use turingosv4::state::typed_tx::EventId;

fn cid(label: &str) -> Cid {
    Cid::from_content(label.as_bytes())
}

fn response(agent: &str, digest: Cid) -> MarketReviewResponse {
    MarketReviewResponse {
        window_id: TxId("window".into()),
        response_id: format!("response-{agent}"),
        agent_id: AgentId(agent.into()),
        role: AgentRole::BullTrader,
        ev_decision_trace_cid: Some(cid(&format!("ev-{agent}"))),
        no_response_trace_cid: None,
        action: "Abstain".into(),
        submitted_tx_id: None,
        librarian_digest_cid: Some(digest),
        broadcast_epoch_id: Some("epoch-1".into()),
    }
}

#[test]
fn barriered_async_freezes_digest_for_all_responses() {
    let digest = cid("digest");
    let window = MarketReviewWindow {
        window_id: TxId("window".into()),
        event_id: EventId(TaskId("task".into())),
        opened_at_head_t: "HEAD".into(),
        market_snapshot_cid: cid("market"),
        eligible_agents: vec![AgentId("Agent_2".into()), AgentId("Agent_1".into())],
        deadline_logical_t: 20,
        mode: MarketReviewMode::BarrieredAsync,
        librarian_digest_cid: Some(digest),
        broadcast_epoch_id: Some("epoch-1".into()),
    };
    let responses = deterministic_response_order(vec![
        response("Agent_2", digest),
        response("Agent_1", digest),
    ]);
    let summary = MarketReviewSummary {
        window_id: TxId("window".into()),
        event_id: EventId(TaskId("task".into())),
        response_count: 2,
        buy_count: 0,
        short_count: 0,
        abstain_count: 2,
        missing_count: 0,
        response_cids: vec![cid("r1"), cid("r2")],
        committed_tx_ids: vec![],
        digest_set: vec![digest],
    };
    let epoch = BroadcastEpoch {
        epoch_id: "epoch-1".into(),
        source_head_t: 10,
        digest_cid: digest,
        valid_from: 10,
        valid_until: 20,
        task_tags: vec!["task".into()],
    };
    validate_market_review_broadcast_contract(&window, &responses, &summary, &epoch, 15).unwrap();
    assert_eq!(responses[0].agent_id.0, "Agent_1");
}

#[test]
fn barriered_async_rejects_mismatched_or_future_digest() {
    let digest = cid("digest");
    let other = cid("other");
    let mut window = MarketReviewWindow {
        window_id: TxId("window".into()),
        event_id: EventId(TaskId("task".into())),
        opened_at_head_t: "HEAD".into(),
        market_snapshot_cid: cid("market"),
        eligible_agents: vec![AgentId("Agent_1".into())],
        deadline_logical_t: 20,
        mode: MarketReviewMode::BarrieredAsync,
        librarian_digest_cid: Some(digest),
        broadcast_epoch_id: Some("epoch-1".into()),
    };
    let responses = vec![response("Agent_1", other)];
    let summary = MarketReviewSummary {
        window_id: TxId("window".into()),
        event_id: EventId(TaskId("task".into())),
        response_count: 1,
        buy_count: 0,
        short_count: 0,
        abstain_count: 1,
        missing_count: 0,
        response_cids: vec![cid("r1")],
        committed_tx_ids: vec![],
        digest_set: vec![digest],
    };
    let epoch = BroadcastEpoch {
        epoch_id: "epoch-1".into(),
        source_head_t: 10,
        digest_cid: digest,
        valid_from: 10,
        valid_until: 20,
        task_tags: vec!["task".into()],
    };
    assert!(
        validate_market_review_broadcast_contract(&window, &responses, &summary, &epoch, 15)
            .unwrap_err()
            .contains("digest mismatch")
    );

    window.librarian_digest_cid = Some(digest);
    let responses = vec![response("Agent_1", digest)];
    assert!(
        validate_market_review_broadcast_contract(&window, &responses, &summary, &epoch, 9)
            .unwrap_err()
            .contains("future digest")
    );
}
