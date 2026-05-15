//! REAL-5 Atom 2 — role-scoped rtool / derive_view gates.

use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::runtime::real5_roles::{
    derive_role_view, derive_role_view_with_context_bytes, AgentRole, DerivedViewInput,
    DerivedViewRequest, PriceSignal,
};
use turingosv4::state::q_state::{AgentId, TaskId};

#[test]
fn sg_r5_2_role_views_are_scoped_and_hashable() {
    let solver = derive_role_view(
        DerivedViewRequest {
            agent_id: AgentId("Agent_solver".into()),
            role: AgentRole::Solver,
            task_id: TaskId("task".into()),
            head_t: "HEAD1".into(),
        },
        DerivedViewInput::fixture(),
    )
    .unwrap();
    assert!(solver
        .hidden_fields_redacted
        .contains(&"raw_diagnostics".to_string()));
    assert!(solver
        .hidden_fields_redacted
        .contains(&"private_market_internals".to_string()));
    assert!(solver
        .public_sections
        .iter()
        .any(|s| s.contains("Lean goal")));
    assert!(!solver.public_sections.iter().any(|s| s.contains("raw CoT")));

    let trader = derive_role_view(
        DerivedViewRequest {
            agent_id: AgentId("Agent_trader".into()),
            role: AgentRole::Trader,
            task_id: TaskId("task".into()),
            head_t: "HEAD1".into(),
        },
        DerivedViewInput {
            price_signals: vec![PriceSignal {
                event_id: "event-1".into(),
                price: "2/3".into(),
                depth: Some(100),
            }],
            ..DerivedViewInput::fixture()
        },
    )
    .unwrap();
    assert_eq!(trader.price_signals.len(), 1);
    assert!(trader.public_sections.iter().any(|s| s.contains("PnL")));
    assert!(trader.read_set.contains(&Cid::default()));
    assert_ne!(solver.derived_view_hash, trader.derived_view_hash);
}

#[test]
fn sg_r5_2_live_view_context_bytes_match_visible_context_cid() {
    let (view, visible_context_bytes) = derive_role_view_with_context_bytes(
        DerivedViewRequest {
            agent_id: AgentId("Agent_trader".into()),
            role: AgentRole::Trader,
            task_id: TaskId("task".into()),
            head_t: "HEAD2".into(),
        },
        DerivedViewInput::fixture(),
    )
    .unwrap();
    assert_eq!(
        view.visible_context_cid,
        Cid::from_content(&visible_context_bytes)
    );
    assert_eq!(view.derived_view_hash, view.visible_context_cid);
}

#[test]
fn sg_r5_2_no_get_all_context_string_in_role_module() {
    let src = include_str!("../src/runtime/real5_roles.rs");
    assert!(
        !src.contains("get_all_context"),
        "role run must not use get_all_context"
    );
}
