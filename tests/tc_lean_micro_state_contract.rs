use turingosv4::judges::lean_micro_state::{
    deterministic_state_id, recertify_complete, AxiomCleanliness, GoalState, GoalView,
    LeanFixtureStepper, LeanRecertError, LeanStepError, LeanStepOutcome, ProofArtifact,
    TacticAttempt,
};

fn goal_state(state_id: &str, parent_state_id: Option<&str>, target: &str) -> GoalState {
    GoalState {
        theorem_id: "thm_core_intro".to_string(),
        state_id: state_id.to_string(),
        parent_state_id: parent_state_id.map(str::to_string),
        goals: vec![GoalView {
            goal_index: 0,
            case_label: None,
            hypotheses: vec!["h : P".to_string()],
            target: target.to_string(),
            mvar_id: Some("?m.1".to_string()),
        }],
        imports_hash: "imports".to_string(),
        preamble_hash: "preamble".to_string(),
        lean_version: "Lean 4.24.0".to_string(),
        mathlib_rev: None,
    }
}

fn attempt(parent_state_id: &str, tactic: &str) -> TacticAttempt {
    TacticAttempt {
        attempt_id: format!("att-{tactic}"),
        parent_state_id: parent_state_id.to_string(),
        tactic: tactic.to_string(),
        timeout_ms: 5000,
        input_goal_hash: "goal-hash".to_string(),
    }
}

fn hashed_goal_state(target: &str) -> GoalState {
    let mut state = goal_state("placeholder", None, target);
    state.state_id = deterministic_state_id(&state);
    state
}

fn clean_axioms() -> AxiomCleanliness {
    AxiomCleanliness {
        checked_by_print_axioms: true,
        axioms: vec!["propext".to_string()],
        whitelist: vec![
            "propext".to_string(),
            "Classical.choice".to_string(),
            "Quot.sound".to_string(),
        ],
        clean: true,
    }
}

#[test]
fn micro_state_ids_are_content_hashes() {
    let initial = hashed_goal_state("P -> P");
    let mut stepper = LeanFixtureStepper::new(initial.clone());

    let outcome = stepper.step(attempt(&initial.state_id, "intro h"));

    match outcome {
        LeanStepOutcome::Advanced { next } => {
            assert_eq!(next.state_id, deterministic_state_id(&next));
            assert!(next.state_id.starts_with("lean-state-sha256:"));
            assert!(!next.state_id.contains('/'));
            assert_ne!(
                next.state_id,
                format!("{}/{}", initial.state_id, "att-intro h")
            );
        }
        other => panic!("intro should advance, got {other:?}"),
    }
}

#[test]
fn micro_step_json_never_contains_verified_or_verdict_kind() {
    let outcome = LeanStepOutcome::Failed {
        class: LeanStepError::Tactic,
        feedback: "bounded public feedback".to_string(),
    };
    let json = serde_json::to_string(&outcome)
        .unwrap()
        .to_ascii_lowercase();

    assert!(!json.contains("verified"));
    assert!(!json.contains("verdict"));
}

#[test]
fn lean_step_outcome_has_no_verified_state_before_final_judge() {
    let state = goal_state("ps-1", None, "P");
    let outcome = LeanStepOutcome::Advanced {
        next: state.clone(),
    };
    let json = serde_json::to_string(&outcome).unwrap();

    assert!(json.contains("advanced"));
    assert!(!json.contains("Verified"));
    assert!(!json.contains("verified"));
    let attempt = attempt(&state.state_id, "intro h");
    assert_eq!(attempt.tactic, "intro h");
}

#[test]
fn proof_artifact_keeps_axiom_cleanliness_explicit_and_fail_closed() {
    let artifact = ProofArtifact {
        theorem_id: "thm".to_string(),
        proof_script: "by\n  exact h".to_string(),
        assembled_source_cid: "cid-source".to_string(),
        final_lean_result_cid: "cid-lean-result".to_string(),
        axiom_report: clean_axioms(),
    };
    let encoded = serde_json::to_string(&artifact).unwrap();
    let decoded: ProofArtifact = serde_json::from_str(&encoded).unwrap();
    assert!(decoded.axiom_report.clean);

    let failed = LeanStepOutcome::Failed {
        class: LeanStepError::Tactic,
        feedback: "tactic failed".to_string(),
    };
    assert!(serde_json::to_string(&failed).unwrap().contains("tactic"));
}

#[test]
fn lean_step_intro_advances() {
    let initial = hashed_goal_state("P -> P");
    let mut stepper = LeanFixtureStepper::new(initial.clone());

    let outcome = stepper.step(attempt(&initial.state_id, "intro h"));

    match outcome {
        LeanStepOutcome::Advanced { next } => {
            assert_eq!(next.state_id, deterministic_state_id(&next));
            assert_eq!(
                next.parent_state_id.as_deref(),
                Some(initial.state_id.as_str())
            );
            assert_eq!(next.goals[0].target, "P");
        }
        other => panic!("intro should advance, got {other:?}"),
    }
}

#[test]
fn lean_step_simp_completes() {
    let initial = hashed_goal_state("Nat.succ n = n + 1");
    let mut stepper = LeanFixtureStepper::new(initial.clone());

    let outcome = stepper.step(attempt(&initial.state_id, "simp"));

    match outcome {
        LeanStepOutcome::Complete { proof_script } => {
            assert_eq!(proof_script, "by\n  simp");
        }
        other => panic!("simp should assemble a proof candidate, got {other:?}"),
    }
}

#[test]
fn lean_step_backtracks_from_parent_state() {
    let initial = hashed_goal_state("P -> P");
    let mut stepper = LeanFixtureStepper::new(initial.clone());
    let advanced = stepper.step(attempt(&initial.state_id, "intro h"));
    let child_id = match advanced {
        LeanStepOutcome::Advanced { next } => next.state_id,
        other => panic!("intro should advance, got {other:?}"),
    };

    assert_eq!(stepper.backtrack(&initial.state_id), Some(initial.clone()));
    assert_eq!(
        stepper
            .backtrack(&child_id)
            .unwrap()
            .parent_state_id
            .as_deref(),
        Some(initial.state_id.as_str())
    );
    assert!(stepper.backtrack("missing").is_none());
}

#[test]
fn lean_step_feedback_is_bounded_public_text() {
    let initial = hashed_goal_state("P");
    let mut stepper = LeanFixtureStepper::new(initial.clone());

    let outcome = stepper.step(attempt(&initial.state_id, "exact missing_identifier"));

    match outcome {
        LeanStepOutcome::Failed { class, feedback } => {
            assert_eq!(class, LeanStepError::Tactic);
            assert!(feedback.chars().count() <= 160);
            assert!(!feedback.to_ascii_lowercase().contains("stderr"));
            assert!(!feedback.contains("unknown identifier 'missing_identifier'"));
        }
        other => panic!("unknown tactic should fail with bounded feedback, got {other:?}"),
    }
}

#[test]
fn complete_outcome_requires_final_lean_judge_verified() {
    let err = recertify_complete("thm", "by\n  simp", false, clean_axioms()).unwrap_err();

    assert_eq!(err, LeanRecertError::FinalJudgeRejected);
}

#[test]
fn proof_artifact_requires_checked_axiom_report() {
    let unchecked = AxiomCleanliness {
        checked_by_print_axioms: false,
        axioms: Vec::new(),
        whitelist: Vec::new(),
        clean: true,
    };

    let err = recertify_complete("thm", "by\n  simp", true, unchecked).unwrap_err();

    assert_eq!(err, LeanRecertError::AxiomReportUnchecked);
}

#[test]
fn unclean_axiom_report_fails_closed() {
    let unclean = AxiomCleanliness {
        checked_by_print_axioms: true,
        axioms: vec!["sorryAx".to_string()],
        whitelist: vec!["propext".to_string()],
        clean: false,
    };

    let err = recertify_complete("thm", "by\n  simp", true, unclean).unwrap_err();

    assert_eq!(err, LeanRecertError::AxiomReportUnclean);

    let artifact = recertify_complete("thm", "by\n  simp", true, clean_axioms()).unwrap();
    assert_eq!(artifact.theorem_id, "thm");
    assert_eq!(artifact.proof_script, "by\n  simp");
}
