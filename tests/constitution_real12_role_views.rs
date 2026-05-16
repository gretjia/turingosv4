//! REAL-12 Atom 2 — Bull/Bear/Solver role-scoped views.

use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::runtime::real5_roles::{
    derive_role_view, derive_role_view_with_context_bytes, AgentRole, DerivedViewInput,
    DerivedViewRequest, PriceSignal,
};
use turingosv4::state::q_state::{AgentId, TaskId};

fn request(role: AgentRole) -> DerivedViewRequest {
    DerivedViewRequest {
        agent_id: AgentId(format!("Agent_{role:?}")),
        role,
        task_id: TaskId("task-real12-view".into()),
        head_t: "HEAD-real12".into(),
    }
}

#[test]
fn bull_and_bear_views_broadcast_role_specific_market_signals() {
    let input = DerivedViewInput {
        read_set: vec![Cid::from_content(b"read-set")],
        price_signals: vec![PriceSignal {
            event_id: "task-market".into(),
            price: "3/5".into(),
            depth: Some(2000),
        }],
        local_errors: Vec::new(),
    };
    let bull = derive_role_view(request(AgentRole::BullTrader), input.clone()).unwrap();
    let bear = derive_role_view(request(AgentRole::BearTrader), input).unwrap();

    let bull_sections = bull.public_sections.join("\n");
    for required in [
        "YES price",
        "TaskOutcome YES market",
        "NodeSurvive YES market",
        "available balance",
        "realized/unrealized PnL",
        "risk cap",
        "liquidity/depth",
        "deadline/budget remaining",
    ] {
        assert!(
            bull_sections.contains(required),
            "bull view missing {required}"
        );
    }

    let bear_sections = bear.public_sections.join("\n");
    for required in [
        "NO price",
        "unsolved-task risk",
        "candidate weakness signals",
        "challenge status",
        "failed attempts",
        "market depth",
        "available balance",
        "PnL",
        "risk cap",
    ] {
        assert!(
            bear_sections.contains(required),
            "bear view missing {required}"
        );
    }

    assert_ne!(bull.derived_view_hash, bear.derived_view_hash);
}

#[test]
fn solver_view_has_limited_market_summary_and_no_raw_material() {
    let solver = derive_role_view(request(AgentRole::Solver), DerivedViewInput::fixture()).unwrap();
    let sections = solver.public_sections.join("\n");
    assert!(sections.contains("Lean goal"));
    assert!(sections.contains("local proof context"));
    assert!(sections.contains("limited market summary only"));
    assert!(!sections.contains("full market dashboard"));

    let rendered = format!("{solver:?}");
    for forbidden in [
        "raw logs",
        "private CoT",
        "raw diagnostics",
        "dashboard-as-truth",
    ] {
        assert!(
            !rendered
                .to_ascii_lowercase()
                .contains(&forbidden.to_ascii_lowercase()),
            "ordinary role view leaked forbidden material: {forbidden}"
        );
    }
}

#[test]
fn prompt_capsule_can_store_view_hash_and_read_set_inputs() {
    let (view, bytes) = derive_role_view_with_context_bytes(
        request(AgentRole::BullTrader),
        DerivedViewInput::fixture(),
    )
    .unwrap();
    assert_eq!(view.visible_context_cid, Cid::from_content(&bytes));
    assert_eq!(view.derived_view_hash, view.visible_context_cid);
    assert!(view
        .hidden_fields_redacted
        .iter()
        .any(|field| field == "raw_diagnostics"));
    assert!(view.read_set.contains(&Cid::default()));
}
