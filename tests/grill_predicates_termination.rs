//! W3 atom — session-aggregate termination predicate unit tests.
//! Phase 6.3.x R2 §A3 FC1-N9 session-aggregate variant.

use turingosv4::runtime::grill_envelope::TurnPayload;
use turingosv4::runtime::grill_predicates::{
    termination_predicate, PredicateFailureClass, PredicateVerdict,
};

fn make_terminal(turn: u32, covered: &[&str], confidence: f64) -> TurnPayload {
    TurnPayload {
        turn,
        question: None,
        covered_slots: covered.iter().map(|s| s.to_string()).collect(),
        open_slots: vec![],
        confidence,
        done: true,
        rationale: "ready".into(),
        playback: Some("playback content".into()),
    }
}

const SEVEN_REQUIRED: &[&str] = &[
    "job",
    "anchor",
    "memory",
    "first_run",
    "robustness",
    "scope",
    "acceptance",
];

#[test]
fn termination_pass_when_seven_required_and_confidence_high() {
    let p = make_terminal(8, SEVEN_REQUIRED, 0.85);
    assert!(termination_predicate(&p).is_pass());
}

#[test]
fn termination_pass_with_8_slots_including_mirror() {
    let mut slots = SEVEN_REQUIRED.to_vec();
    slots.push("mirror");
    let p = make_terminal(8, &slots, 0.85);
    assert!(termination_predicate(&p).is_pass());
}

#[test]
fn termination_fail_when_missing_acceptance_slot() {
    let missing_acceptance: Vec<&str> = SEVEN_REQUIRED
        .iter()
        .copied()
        .filter(|s| *s != "acceptance")
        .collect();
    let p = make_terminal(8, &missing_acceptance, 0.85);
    assert!(!termination_predicate(&p).is_pass());
}

#[test]
fn termination_fail_when_missing_job_slot() {
    let missing_job: Vec<&str> = SEVEN_REQUIRED
        .iter()
        .copied()
        .filter(|s| *s != "job")
        .collect();
    let p = make_terminal(8, &missing_job, 0.85);
    assert!(!termination_predicate(&p).is_pass());
}

#[test]
fn termination_fail_when_confidence_below_0_8() {
    let p = make_terminal(8, SEVEN_REQUIRED, 0.5);
    let v = termination_predicate(&p);
    assert_eq!(
        v.failure_class(),
        Some(PredicateFailureClass::ConfidenceOutOfRange)
    );
}

#[test]
fn termination_fail_when_turn_below_4() {
    let p = make_terminal(3, SEVEN_REQUIRED, 0.9);
    let v = termination_predicate(&p);
    assert_eq!(
        v.failure_class(),
        Some(PredicateFailureClass::TurnOutOfRange)
    );
}

#[test]
fn termination_pass_at_turn_4_minimum_floor() {
    let p = make_terminal(4, SEVEN_REQUIRED, 0.85);
    assert!(termination_predicate(&p).is_pass());
}

#[test]
fn termination_pass_at_turn_15_ceiling() {
    let p = make_terminal(15, SEVEN_REQUIRED, 0.85);
    assert!(termination_predicate(&p).is_pass());
}

#[test]
fn termination_fail_when_confidence_above_1_0() {
    let p = make_terminal(8, SEVEN_REQUIRED, 1.5);
    let v = termination_predicate(&p);
    assert!(matches!(v, PredicateVerdict::Fail(_)));
}
