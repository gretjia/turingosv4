//! A07 AgentView shielding gates.

use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::runtime::tape_event::{TapeEventEnvelope, TapeEventKind, TapeEventRef};
use turingosv4::runtime::tc_agent_view::{derive_agent_view, AgentViewPolicy, AgentViewRequest};
use turingosv4::state::q_state::AgentId;

const HEAD_A: &str = "1111111111111111111111111111111111111111";
const HEAD_B: &str = "2222222222222222222222222222222222222222";

fn event(logical_t: u64, head_oid: &str, payload: Cid) -> TapeEventEnvelope {
    TapeEventEnvelope {
        logical_t,
        tape_ref: TapeEventRef::L4Accepted {
            head_oid: head_oid.to_string(),
        },
        kind: TapeEventKind::AcceptedTransition,
        payload_cid: Some(payload),
        source_tx_kind: Some(turingosv4::bottom_white::ledger::transition_ledger::TxKind::Work),
    }
}

#[test]
fn agent_view_is_bound_to_allowed_tape_prefix_and_hashable() {
    let visible = Cid([1; 32]);
    let future = Cid([2; 32]);
    let view = derive_agent_view(
        AgentViewRequest {
            agent_id: AgentId("agent-a".into()),
            role_label: Some("Solver".into()),
            view_policy_id: "a07/solver/v1".into(),
            allowed_tape_prefix_head: HEAD_A.into(),
        },
        AgentViewPolicy {
            redacted_fields: vec!["raw_stderr".into(), "hidden_oracle".into()],
            denied_cids: vec![future],
        },
        &[event(1, HEAD_A, visible), event(2, HEAD_B, future)],
    )
    .expect("agent view derives from allowed prefix");

    assert_eq!(view.allowed_tape_prefix_head, HEAD_A);
    assert_eq!(view.visible_event_cids, vec![visible]);
    assert!(view.redacted_fields.contains(&"raw_stderr".to_string()));
    assert_eq!(
        view.visible_context_cid,
        Cid::from_content(&view.visible_context_bytes)
    );
    assert_eq!(view.visible_context_hash, view.visible_context_cid);
}

#[test]
fn future_events_and_denied_cids_do_not_serialize_into_agent_view() {
    let visible = Cid([3; 32]);
    let private = Cid([4; 32]);
    let view = derive_agent_view(
        AgentViewRequest {
            agent_id: AgentId("agent-b".into()),
            role_label: Some("Trader".into()),
            view_policy_id: "a07/trader/v1".into(),
            allowed_tape_prefix_head: HEAD_A.into(),
        },
        AgentViewPolicy {
            redacted_fields: vec!["private_diagnostic_cid".into()],
            denied_cids: vec![private],
        },
        &[event(1, HEAD_A, visible), event(2, HEAD_B, private)],
    )
    .expect("agent view derives");

    let json = serde_json::to_string(&view).expect("view serializes");
    assert!(json.contains(&visible.hex()));
    assert!(!json.contains(&private.hex()));
    assert!(!json.contains("denied_cids"));
    assert!(!json.contains("private_diagnostic_cid"));
    assert!(!json.contains(HEAD_B));
}

#[test]
fn derived_view_refs_cannot_enter_agent_view_projection() {
    let result = derive_agent_view(
        AgentViewRequest {
            agent_id: AgentId("agent-c".into()),
            role_label: None,
            view_policy_id: "a07/observer/v1".into(),
            allowed_tape_prefix_head: HEAD_A.into(),
        },
        AgentViewPolicy::default(),
        &[TapeEventEnvelope {
            logical_t: 1,
            tape_ref: TapeEventRef::DerivedView {
                source: "dashboard".into(),
            },
            kind: TapeEventKind::AcceptedTransition,
            payload_cid: Some(Cid([5; 32])),
            source_tx_kind: Some(turingosv4::bottom_white::ledger::transition_ledger::TxKind::Work),
        }],
    );

    assert!(result.is_err());
}
