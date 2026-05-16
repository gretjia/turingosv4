//! REAL-BCAST-1 — LibrarianSelector gates.

use tempfile::TempDir;
use turingosv4::bottom_white::cas::schema::{Cid, ObjectType};
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::economy::money::MicroCoin;
use turingosv4::runtime::ev_decision_trace::{
    write_ev_decision_trace_to_cas, EVAction, EVDecisionTrace, EVReason,
    EV_DECISION_TRACE_SCHEMA_ID,
};
use turingosv4::runtime::librarian_broadcast::{
    decode_librarian_candidate, select_librarian_events, LibrarianEvidenceKind,
};
use turingosv4::runtime::real5_roles::{AgentRole, MarketSide, RationalPrice};
use turingosv4::state::q_state::{AgentId, TaskId};
use turingosv4::state::typed_tx::EventId;

fn ev_trace(reason: EVReason) -> EVDecisionTrace {
    EVDecisionTrace {
        schema_version: EV_DECISION_TRACE_SCHEMA_ID.to_string(),
        review_window_id: "window".into(),
        review_response_id: "response".into(),
        run_id: "run".into(),
        batch_id: "batch".into(),
        agent_id: AgentId("Agent_bull".into()),
        role: AgentRole::BullTrader,
        task_id: TaskId("task".into()),
        event_id: EventId(TaskId("task".into())),
        side: MarketSide::Yes,
        quoted_price: RationalPrice::new(5, 8).unwrap(),
        implied_probability_bps: 6250,
        agent_probability_bps: 6000,
        edge_bps: -250,
        expected_value_micro: -2500,
        amount: MicroCoin::from_micro_units(10_000),
        max_risk: MicroCoin::from_micro_units(10_000),
        available_balance: MicroCoin::from_micro_units(100_000),
        risk_cap: MicroCoin::from_micro_units(50_000),
        liquidity_depth: MicroCoin::from_micro_units(100_000),
        slippage_bps: 0,
        risk_cap_triggered: false,
        action: EVAction::Abstain,
        reason,
        prompt_capsule_cid: Cid::from_content(b"prompt"),
        market_snapshot_cid: Cid::from_content(b"market"),
        model_assignment_cid: None,
        model_family: None,
        private_alpha_cid: None,
        tool_result_cid: None,
        parent_state_root: "root".into(),
        created_at_head_t: "HEAD".into(),
        public_summary: "public EV trace".into(),
    }
}

#[test]
fn selector_reads_known_typed_sidecars() {
    let tmp = TempDir::new().unwrap();
    let mut cas = CasStore::open(tmp.path()).unwrap();
    let ev_cid =
        write_ev_decision_trace_to_cas(&mut cas, &ev_trace(EVReason::NegativeEV), "ev", 1).unwrap();

    let events = select_librarian_events(&cas).unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].cid, ev_cid);
    assert_eq!(events[0].kind, LibrarianEvidenceKind::EVReason);
    assert_eq!(events[0].class_label, "ev:NegativeEV");
}

#[test]
fn selector_fails_closed_on_explicit_unknown_candidate() {
    let tmp = TempDir::new().unwrap();
    let mut cas = CasStore::open(tmp.path()).unwrap();
    let unknown = cas
        .put(
            br#"{"schema_version":"mystery.v1"}"#,
            ObjectType::Generic,
            "unknown",
            1,
            Some("mystery.v1".into()),
        )
        .unwrap();

    assert!(decode_librarian_candidate(&cas, &unknown)
        .unwrap_err()
        .contains("unknown librarian evidence schema"));
}

#[test]
fn selector_does_not_copy_raw_payload_body() {
    let tmp = TempDir::new().unwrap();
    let mut cas = CasStore::open(tmp.path()).unwrap();
    let _cid =
        write_ev_decision_trace_to_cas(&mut cas, &ev_trace(EVReason::NoActionableMarket), "ev", 1)
            .unwrap();
    let events = select_librarian_events(&cas).unwrap();
    let rendered = format!("{events:?}");
    assert!(!rendered.contains("raw prompt"));
    assert!(!rendered.contains("raw completion"));
    assert!(!rendered.contains("private CoT"));
    assert!(!rendered.contains("Lean stderr"));
}
