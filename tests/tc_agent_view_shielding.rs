use turingosv4::judges::lean_micro_state::{GoalState, GoalView, LeanStepError};
use turingosv4::runtime::tc_agent_view::{
    guard_tc_prompt, render_tc_goal_view, TcAgentViewRequest,
};

fn goal_state() -> GoalState {
    GoalState {
        theorem_id: "hidden_thm_internal".to_string(),
        state_id: "state-1".to_string(),
        parent_state_id: Some("state-0".to_string()),
        goals: vec![GoalView {
            goal_index: 0,
            case_label: Some("base".to_string()),
            hypotheses: vec![
                "h : P".to_string(),
                "secret_internal_helper : hidden_payload".to_string(),
            ],
            target: "P".to_string(),
            mvar_id: Some("?m.1".to_string()),
        }],
        imports_hash: "imports".to_string(),
        preamble_hash: "preamble".to_string(),
        lean_version: "Lean 4.24.0".to_string(),
        mathlib_rev: Some("mathlib-private-rev".to_string()),
    }
}

#[test]
fn tc_goal_view_hides_theorem_body_and_full_landscape() {
    let private_body = "theorem hidden_thm_internal : P := by exact secret".to_string();
    let full_bank = "FULL_THEOREM_BANK: hidden_thm_internal, hidden_other".to_string();
    let diagnostic = "private diagnostic cid=abc body=secret".to_string();
    let request = TcAgentViewRequest {
        state: goal_state(),
        theorem_id_is_public: false,
        feedback_class: None,
        feedback_summary: None,
        hidden_theorem_body: Some(private_body.clone()),
        full_theorem_bank: Some(full_bank.clone()),
        private_diagnostic: Some(diagnostic.clone()),
    };

    let view = render_tc_goal_view(request).expect("view renders");
    let json = serde_json::to_string(&view).unwrap();

    assert!(json.contains("state-1"));
    assert!(!json.contains("hidden_thm_internal"));
    assert!(!json.contains(&private_body));
    assert!(!json.contains(&full_bank));
    assert!(!json.contains(&diagnostic));
    assert!(!json.contains("mathlib-private-rev"));
    assert!(!json.contains("?m.1"));
}

#[test]
fn tc_goal_view_exposes_only_state_id_goals_and_bounded_feedback() {
    let request = TcAgentViewRequest {
        state: goal_state(),
        theorem_id_is_public: true,
        feedback_class: Some(LeanStepError::Tactic),
        feedback_summary: Some("intro made progress; remaining target summarized".to_string()),
        hidden_theorem_body: None,
        full_theorem_bank: None,
        private_diagnostic: None,
    };

    let view = render_tc_goal_view(request).expect("view renders");

    assert_eq!(view.state_id, "state-1");
    assert_eq!(view.theorem_id.as_deref(), Some("hidden_thm_internal"));
    assert_eq!(view.goals.len(), 1);
    assert_eq!(view.goals[0].goal_index, 0);
    assert_eq!(view.goals[0].case_label.as_deref(), Some("base"));
    assert_eq!(view.goals[0].public_hypotheses_summary, "2 hypotheses");
    assert_eq!(view.goals[0].public_target_summary, "P");
    assert_eq!(view.feedback.unwrap().class, "tactic");
}

#[test]
fn tc_prompt_guard_blocks_sensitive_verifier_and_private_content() {
    let marker_a = ["raw", " std", "err"].concat();
    let marker_b = ["Lean", " std", "err"].concat();
    let blocked = [
        marker_a,
        marker_b,
        "hidden theorem body".to_string(),
        "private diagnostic".to_string(),
        "FULL_THEOREM_BANK".to_string(),
    ];

    for marker in blocked {
        let prompt = format!("scoped prompt includes {marker}");
        assert!(guard_tc_prompt(&prompt).is_err(), "marker must be blocked");
    }
}

#[test]
fn tc_prompt_guard_allows_bounded_error_class() {
    let prompt = "state_id=state-1 goal[0]=P feedback_class=tactic summary=bounded";

    guard_tc_prompt(prompt).expect("bounded prompt view allowed");
}
