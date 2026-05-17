//! Market Autonomy Lab — BCAST market/no-trade coverage gates.
//!
//! These are red gates for Atom 1. They define the expected CAS-backed
//! Librarian path before any Trust-Root-pinned implementation patch.

use tempfile::TempDir;
use turingosv4::bottom_white::cas::schema::{Cid, ObjectType};
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::librarian_broadcast::{
    build_librarian_digest, select_librarian_events, LibrarianEvidenceKind, LibrarianSourceScope,
};
use turingosv4::runtime::market_decision_trace::{
    write_market_decision_trace_to_cas, MarketDecisionTrace, NoTradeReason,
};
use turingosv4::runtime::market_opportunity_trace::{
    write_market_opportunity_trace_to_cas, MarketOpportunityTrace,
};
use turingosv4::runtime::market_review::{write_market_review_summary_to_cas, MarketReviewSummary};
use turingosv4::state::q_state::{AgentId, TaskId, TxId};
use turingosv4::state::typed_tx::{BuyDirection, EventId};

fn source_scope() -> LibrarianSourceScope {
    LibrarianSourceScope {
        current_run_cas_root: Cid::from_content(b"root"),
        prior_capsule_cids: vec![],
        max_prior_batches: 0,
        task_tags: vec!["market-autonomy".into()],
    }
}

fn no_pool_trace(agent: &str, summary: &str) -> MarketDecisionTrace {
    MarketDecisionTrace::no_trade(
        AgentId(agent.into()),
        Some(TxId("worktx-Agent_1-evt-1".into())),
        Some(BuyDirection::BuyYes),
        Some(10_000),
        NoTradeReason::NoPool,
        summary,
    )
}

#[test]
fn selector_promotes_market_decision_no_trade_into_market_reason_events() {
    let tmp = TempDir::new().unwrap();
    let mut cas = CasStore::open(tmp.path()).unwrap();
    let no_pool_1 = write_market_decision_trace_to_cas(
        &mut cas,
        &no_pool_trace("Agent_bull", "no pool"),
        "a",
        1,
    )
    .unwrap();
    let no_pool_2 = write_market_decision_trace_to_cas(
        &mut cas,
        &no_pool_trace("Agent_bear", "no pool"),
        "b",
        2,
    )
    .unwrap();
    let submitted = MarketDecisionTrace::submitted(
        AgentId("Agent_submit".into()),
        TxId("worktx-Agent_1-evt-1".into()),
        BuyDirection::BuyYes,
        10_000,
        TxId("router-Agent_submit-1".into()),
        "submitted buy yes",
    );
    write_market_decision_trace_to_cas(&mut cas, &submitted, "submitted", 3).unwrap();

    let events = select_librarian_events(&cas).unwrap();
    let market_events: Vec<_> = events
        .iter()
        .filter(|event| event.kind == LibrarianEvidenceKind::MarketReason)
        .collect();

    assert_eq!(market_events.len(), 2);
    assert!(market_events.iter().any(|event| event.cid == no_pool_1));
    assert!(market_events.iter().any(|event| event.cid == no_pool_2));
    assert!(market_events
        .iter()
        .all(|event| event.class_label == "market_no_trade:no_pool"));
}

#[test]
fn digest_clusters_market_decision_no_trade_reasons() {
    let tmp = TempDir::new().unwrap();
    let mut cas = CasStore::open(tmp.path()).unwrap();
    let first = write_market_decision_trace_to_cas(
        &mut cas,
        &no_pool_trace("Agent_bull", "no pool"),
        "a",
        1,
    )
    .unwrap();
    let second = write_market_decision_trace_to_cas(
        &mut cas,
        &no_pool_trace("Agent_bear", "no pool"),
        "b",
        2,
    )
    .unwrap();

    let events = select_librarian_events(&cas).unwrap();
    let digest = build_librarian_digest(source_scope(), 10, events).unwrap();

    assert_eq!(digest.market_reason_clusters.len(), 1);
    let cluster = &digest.market_reason_clusters[0];
    assert_eq!(cluster.reason, "market_no_trade:no_pool");
    assert_eq!(cluster.count, 2);
    assert_eq!(cluster.provenance_cids, vec![first, second]);
}

#[test]
fn selector_promotes_market_review_summary_abstain_missing_into_market_reasons() {
    let tmp = TempDir::new().unwrap();
    let mut cas = CasStore::open(tmp.path()).unwrap();
    let summary = MarketReviewSummary {
        window_id: TxId("window-1".into()),
        event_id: EventId(TaskId("task-1".into())),
        response_count: 3,
        buy_count: 0,
        short_count: 0,
        abstain_count: 2,
        missing_count: 1,
        response_cids: vec![
            Cid::from_content(b"response-1"),
            Cid::from_content(b"response-2"),
            Cid::from_content(b"response-3"),
        ],
        committed_tx_ids: vec![],
        digest_set: vec![],
    };
    write_market_review_summary_to_cas(&mut cas, &summary, "summary", 5).unwrap();

    let events = select_librarian_events(&cas).unwrap();
    let digest = build_librarian_digest(source_scope(), 10, events).unwrap();
    let labels: Vec<_> = digest
        .market_reason_clusters
        .iter()
        .map(|cluster| cluster.reason.as_str())
        .collect();

    assert!(labels.contains(&"market_review:abstain"));
    assert!(labels.contains(&"market_review:missing"));
}

#[test]
fn selector_promotes_market_opportunity_trace_into_market_reason_events() {
    let tmp = TempDir::new().unwrap();
    let mut cas = CasStore::open(tmp.path()).unwrap();
    let trace = MarketOpportunityTrace {
        schema_version: "real11.market_opportunity_trace.v1".into(),
        agent_id: AgentId("Agent_bull".into()),
        role: turingosv4::runtime::real5_roles::AgentRole::BullTrader,
        task_id: TaskId("task-1".into()),
        head_t: "head_t".into(),
        visible_markets: vec![EventId(TaskId("task-1".into()))],
        actionable_markets: vec![],
        available_balance: turingosv4::economy::money::MicroCoin::from_micro_units(10_000),
        router_available: true,
        reason_if_no_actionable_market: Some(NoTradeReason::NoPool),
        prompt_capsule_cid: None,
    };
    let cid = write_market_opportunity_trace_to_cas(&mut cas, &trace, "opportunity", 7).unwrap();

    let events = select_librarian_events(&cas).unwrap();
    assert!(events.iter().any(|event| {
        event.cid == cid
            && event.kind == LibrarianEvidenceKind::MarketReason
            && event.class_label == "market_opportunity:no_pool"
    }));
}

#[test]
fn selector_fails_closed_on_unknown_schema_in_cas_index() {
    let tmp = TempDir::new().unwrap();
    let mut cas = CasStore::open(tmp.path()).unwrap();
    cas.put(
        br#"{"schema_version":"mystery.v1"}"#,
        ObjectType::Generic,
        "unknown",
        1,
        Some("mystery.v1".into()),
    )
    .unwrap();

    assert!(select_librarian_events(&cas)
        .unwrap_err()
        .contains("unknown librarian evidence schema"));
}

#[test]
fn selector_skips_known_safe_non_broadcast_generic_schemas() {
    let tmp = TempDir::new().unwrap();
    let mut cas = CasStore::open(tmp.path()).unwrap();
    for schema in [
        "TransitionError.display.v1",
        "real5.role_assignment_manifest.v1",
        "v1/model_assignment_manifest",
        "turingosv4.agent_proposal_record.v1",
        "real13.policy_trader_trace.v1",
        "turingosv4.librarian_digest.v1",
        "turingosv4.librarian_role_crop.v1",
        "turingosv4.librarian_broadcast_epoch.v1",
        "turingosv4.verification_result.v1",
        "turingosv4.proposal_telemetry.v1",
    ] {
        let payload = format!(r#"{{"schema_version":"{schema}","public":"metadata"}}"#);
        cas.put(
            payload.as_bytes(),
            ObjectType::Generic,
            schema,
            1,
            Some(schema.into()),
        )
        .unwrap();
    }

    assert!(
        select_librarian_events(&cas).unwrap().is_empty(),
        "known safe metadata schemas are not Librarian broadcast candidates"
    );
}

#[test]
fn market_no_trade_librarian_path_rejects_raw_prompt_completion_cot_logs() {
    let tmp = TempDir::new().unwrap();
    let mut cas = CasStore::open(tmp.path()).unwrap();
    let trace = no_pool_trace(
        "Agent_bull",
        "raw prompt and private CoT should never broadcast",
    );
    write_market_decision_trace_to_cas(&mut cas, &trace, "raw", 1).unwrap();

    assert!(select_librarian_events(&cas)
        .unwrap_err()
        .contains("forbidden broadcast material"));
}

#[test]
fn dashboard_bcast_section_reports_market_no_trade_cluster_counts() {
    let dashboard = include_str!("../src/bin/audit_dashboard.rs");

    for required in [
        "librarian_market_reason_cluster_count",
        "librarian_no_trade_reason_cluster_count",
        "librarian_ev_reason_cluster_count",
    ] {
        assert!(
            dashboard.contains(required),
            "REAL-BCAST dashboard section must render {required}"
        );
    }
}
