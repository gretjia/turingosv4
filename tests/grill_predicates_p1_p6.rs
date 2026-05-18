//! W3 atom — per-turn predicates P1-P6 unit tests.
//! Phase 6.3.x R2 §A8 typed PredicateFailureClass discipline.

use turingosv4::runtime::grill_envelope::TurnPayload;
use turingosv4::runtime::grill_predicates::{
    p2_kind_ok, p3_slots_in_vocab, p4_monotonic, p5_turn_bounded, p6_question_nonempty_lang,
    run_turn_predicates, Lang, PredicateFailureClass, PredicateVerdict,
};

fn make_payload(
    turn: u32,
    question: Option<&str>,
    covered: &[&str],
    open: &[&str],
    confidence: f64,
    done: bool,
    rationale: &str,
    playback: Option<&str>,
) -> TurnPayload {
    TurnPayload {
        turn,
        question: question.map(String::from),
        covered_slots: covered.iter().map(|s| s.to_string()).collect(),
        open_slots: open.iter().map(|s| s.to_string()).collect(),
        confidence,
        done,
        rationale: rationale.to_string(),
        playback: playback.map(String::from),
    }
}

#[test]
fn p2_pass_on_question_when_not_done() {
    let p = make_payload(3, Some("你想做什么？"), &[], &[], 0.3, false, "x", None);
    assert!(p2_kind_ok(&p).is_pass());
}

#[test]
fn p2_pass_on_done_terminal() {
    let p = make_payload(
        8,
        None,
        &["job"],
        &[],
        0.9,
        true,
        "x",
        Some("playback here"),
    );
    assert!(p2_kind_ok(&p).is_pass());
}

#[test]
fn p2_fail_on_done_true_but_no_playback() {
    let p = make_payload(8, None, &["job"], &[], 0.9, true, "x", None);
    let v = p2_kind_ok(&p);
    assert_eq!(
        v.failure_class(),
        Some(PredicateFailureClass::PlaybackMissing)
    );
}

#[test]
fn p2_fail_on_done_false_but_no_question() {
    let p = make_payload(3, None, &[], &[], 0.3, false, "x", None);
    let v = p2_kind_ok(&p);
    assert_eq!(
        v.failure_class(),
        Some(PredicateFailureClass::QuestionMissing)
    );
}

#[test]
fn p3_pass_on_canonical_slots() {
    let p = make_payload(
        2,
        Some("xxxxxxxx"),
        &["job", "anchor"],
        &["memory"],
        0.2,
        false,
        "x",
        None,
    );
    assert!(p3_slots_in_vocab(&p).is_pass());
}

#[test]
fn p3_fail_on_invented_slot() {
    let p = make_payload(
        2,
        Some("xxxxxxxx"),
        &["job", "foo"],
        &[],
        0.2,
        false,
        "x",
        None,
    );
    let v = p3_slots_in_vocab(&p);
    assert_eq!(v.failure_class(), Some(PredicateFailureClass::UnknownSlot));
}

#[test]
fn p4_pass_on_strict_superset() {
    let prev = vec!["job".to_string(), "anchor".to_string()];
    let p = make_payload(
        3,
        Some("xxxxxxxx"),
        &["job", "anchor", "memory"],
        &[],
        0.3,
        false,
        "x",
        None,
    );
    assert!(p4_monotonic(&p, &prev).is_pass());
}

#[test]
fn p4_pass_on_equality() {
    let prev = vec!["job".to_string(), "anchor".to_string()];
    let p = make_payload(
        3,
        Some("xxxxxxxx"),
        &["job", "anchor"],
        &[],
        0.3,
        false,
        "x",
        None,
    );
    assert!(p4_monotonic(&p, &prev).is_pass());
}

#[test]
fn p4_fail_on_shrinking_set() {
    let prev = vec!["job".to_string(), "anchor".to_string()];
    let p = make_payload(3, Some("xxxxxxxx"), &["job"], &[], 0.3, false, "x", None);
    let v = p4_monotonic(&p, &prev);
    assert_eq!(v.failure_class(), Some(PredicateFailureClass::NonMonotonic));
}

#[test]
fn p4_pass_on_turn_1_empty_prev() {
    let p = make_payload(1, Some("xxxxxxxx"), &[], &[], 0.0, false, "x", None);
    assert!(p4_monotonic(&p, &[]).is_pass());
}

#[test]
fn p5_pass_on_turn_15() {
    let p = make_payload(15, Some("xxxxxxxx"), &[], &[], 0.5, false, "x", None);
    assert!(p5_turn_bounded(&p).is_pass());
}

#[test]
fn p5_fail_on_turn_16() {
    let p = make_payload(16, Some("xxxxxxxx"), &[], &[], 0.5, false, "x", None);
    let v = p5_turn_bounded(&p);
    assert_eq!(
        v.failure_class(),
        Some(PredicateFailureClass::TurnOutOfRange)
    );
}

#[test]
fn p5_fail_on_turn_zero() {
    let p = make_payload(0, Some("xxxxxxxx"), &[], &[], 0.5, false, "x", None);
    assert!(matches!(p5_turn_bounded(&p), PredicateVerdict::Fail(_)));
}

#[test]
fn p6_pass_on_chinese_question() {
    let p = make_payload(
        3,
        Some("你最近遇到什么事情让你想要这个工具？"),
        &[],
        &[],
        0.3,
        false,
        "x",
        None,
    );
    assert!(p6_question_nonempty_lang(&p, Lang::Zh).is_pass());
}

#[test]
fn p6_fail_on_english_question_when_zh_requested() {
    let p = make_payload(
        3,
        Some("What do you need a tool for?"),
        &[],
        &[],
        0.3,
        false,
        "x",
        None,
    );
    let v = p6_question_nonempty_lang(&p, Lang::Zh);
    assert_eq!(
        v.failure_class(),
        Some(PredicateFailureClass::LanguageMismatch)
    );
}

#[test]
fn p6_pass_on_english_when_en_requested() {
    let p = make_payload(
        3,
        Some("What do you need a tool for in the morning?"),
        &[],
        &[],
        0.3,
        false,
        "x",
        None,
    );
    assert!(p6_question_nonempty_lang(&p, Lang::En).is_pass());
}

#[test]
fn p6_pass_on_8_char_chinese() {
    let p = make_payload(
        3,
        Some("你想做什么类的工具呢？"),
        &[],
        &[],
        0.3,
        false,
        "x",
        None,
    );
    assert!(p6_question_nonempty_lang(&p, Lang::Zh).is_pass());
}

#[test]
fn p6_fail_on_7_char_chinese() {
    let p = make_payload(3, Some("你想要什么呢"), &[], &[], 0.3, false, "x", None);
    let v = p6_question_nonempty_lang(&p, Lang::Zh);
    assert_eq!(
        v.failure_class(),
        Some(PredicateFailureClass::QuestionTooShort)
    );
}

#[test]
fn p6_pass_when_done_true_skips_check() {
    let p = make_payload(8, None, &["job"], &[], 0.9, true, "x", Some("playback"));
    assert!(p6_question_nonempty_lang(&p, Lang::Zh).is_pass());
}

#[test]
fn predicate_failure_class_discriminant_lock() {
    assert_eq!(PredicateFailureClass::SchemaParseError as u8, 0);
    assert_eq!(PredicateFailureClass::KindMismatch as u8, 1);
    assert_eq!(PredicateFailureClass::UnknownSlot as u8, 2);
    assert_eq!(PredicateFailureClass::NonMonotonic as u8, 3);
    assert_eq!(PredicateFailureClass::TurnOutOfRange as u8, 4);
    assert_eq!(PredicateFailureClass::LanguageMismatch as u8, 5);
    assert_eq!(PredicateFailureClass::QuestionTooShort as u8, 6);
    assert_eq!(PredicateFailureClass::QuestionMissing as u8, 7);
    assert_eq!(PredicateFailureClass::PlaybackMissing as u8, 8);
    assert_eq!(PredicateFailureClass::ConfidenceOutOfRange as u8, 9);
}

#[test]
fn bundle_all_pass_when_each_passes() {
    let p = make_payload(
        3,
        Some("你想做什么呢请说"),
        &["job"],
        &["anchor"],
        0.3,
        false,
        "x",
        None,
    );
    let bundle = run_turn_predicates(&p, &[], Lang::Zh);
    assert!(bundle.all_pass());
    assert!(bundle.first_failure().is_none());
}

#[test]
fn bundle_first_failure_reports_correct_class() {
    let p = make_payload(16, Some("xxxxxxxx"), &[], &[], 0.5, false, "x", None);
    let bundle = run_turn_predicates(&p, &[], Lang::En);
    assert!(!bundle.all_pass());
    assert_eq!(
        bundle.first_failure(),
        Some(PredicateFailureClass::TurnOutOfRange)
    );
}
