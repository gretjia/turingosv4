use turingosv4::runtime::grill_envelope::{
    parse_and_validate, parse_turn_payload, EnvelopeParseError, TurnPayload, CANONICAL_SLOTS,
    REQUIRED_SLOTS,
};

#[test]
fn parses_valid_turn_3_question() {
    let raw = r#"{
        "turn": 3,
        "question": "你刚才提到离线玩，那需要联机吗？",
        "covered_slots": ["job", "anchor"],
        "open_slots": ["memory", "first_run", "robustness", "scope", "acceptance"],
        "confidence": 0.4,
        "done": false,
        "rationale": "probe memory slot via offline emphasis"
    }"#;
    let p = parse_and_validate(raw).expect("should parse");
    assert_eq!(p.turn, 3);
    assert_eq!(p.covered_slots.len(), 2);
    assert!(!p.done);
    assert!(p.playback.is_none());
}

#[test]
fn parses_valid_terminal_with_playback() {
    let raw = r#"{
        "turn": 8,
        "question": null,
        "covered_slots": ["job","anchor","memory","first_run","robustness","scope","acceptance","mirror"],
        "open_slots": [],
        "confidence": 0.88,
        "done": true,
        "rationale": "all required slots covered",
        "playback": "1) 你想要... 2) 像谁... 7) 算成功..."
    }"#;
    let p = parse_and_validate(raw).expect("should parse");
    assert!(p.done);
    assert!(p.playback.is_some());
    assert!(p.question.is_none());
}

#[test]
fn rejects_turn_16_out_of_range() {
    let raw = r#"{"turn":16,"question":"x","covered_slots":[],"open_slots":[],"confidence":0.5,"done":false,"rationale":"x"}"#;
    match parse_and_validate(raw) {
        Err(EnvelopeParseError::TurnOutOfRange(16)) => {}
        other => panic!("expected TurnOutOfRange(16), got {:?}", other),
    }
}

#[test]
fn rejects_turn_zero() {
    let raw = r#"{"turn":0,"question":"x","covered_slots":[],"open_slots":[],"confidence":0.5,"done":false,"rationale":"x"}"#;
    assert!(matches!(
        parse_and_validate(raw),
        Err(EnvelopeParseError::TurnOutOfRange(0))
    ));
}

#[test]
fn rejects_confidence_above_1() {
    let raw = r#"{"turn":3,"question":"x","covered_slots":[],"open_slots":[],"confidence":1.5,"done":false,"rationale":"x"}"#;
    assert!(matches!(
        parse_and_validate(raw),
        Err(EnvelopeParseError::ConfidenceOutOfRange(_))
    ));
}

#[test]
fn rejects_confidence_negative() {
    let raw = r#"{"turn":3,"question":"x","covered_slots":[],"open_slots":[],"confidence":-0.1,"done":false,"rationale":"x"}"#;
    assert!(matches!(
        parse_and_validate(raw),
        Err(EnvelopeParseError::ConfidenceOutOfRange(_))
    ));
}

#[test]
fn rejects_empty_question_when_done_false() {
    let raw = r#"{"turn":3,"question":"","covered_slots":[],"open_slots":[],"confidence":0.5,"done":false,"rationale":"x"}"#;
    assert!(matches!(
        parse_and_validate(raw),
        Err(EnvelopeParseError::EmptyQuestionWhenNotDone)
    ));
}

#[test]
fn rejects_null_question_when_done_false() {
    let raw = r#"{"turn":3,"question":null,"covered_slots":[],"open_slots":[],"confidence":0.5,"done":false,"rationale":"x"}"#;
    assert!(matches!(
        parse_and_validate(raw),
        Err(EnvelopeParseError::EmptyQuestionWhenNotDone)
    ));
}

#[test]
fn rejects_missing_playback_when_done_true() {
    let raw = r#"{"turn":8,"question":null,"covered_slots":["job","anchor","memory","first_run","robustness","scope","acceptance"],"open_slots":[],"confidence":0.85,"done":true,"rationale":"x"}"#;
    assert!(matches!(
        parse_and_validate(raw),
        Err(EnvelopeParseError::MissingPlaybackOnDone)
    ));
}

#[test]
fn rejects_invalid_json() {
    let raw = r#"not json at all"#;
    assert!(matches!(
        parse_and_validate(raw),
        Err(EnvelopeParseError::InvalidJson(_))
    ));
}

#[test]
fn canonical_slots_have_eight() {
    assert_eq!(CANONICAL_SLOTS.len(), 8);
    assert!(CANONICAL_SLOTS.contains(&"job"));
    assert!(CANONICAL_SLOTS.contains(&"mirror"));
}

#[test]
fn required_slots_have_seven_and_exclude_mirror() {
    assert_eq!(REQUIRED_SLOTS.len(), 7);
    assert!(!REQUIRED_SLOTS.contains(&"mirror"));
    assert!(REQUIRED_SLOTS.contains(&"job"));
    assert!(REQUIRED_SLOTS.contains(&"acceptance"));
}

#[test]
fn parses_without_validate_does_not_check_ranges() {
    // parse_turn_payload should accept turn=16 (the range check is in parse_and_validate)
    let raw = r#"{"turn":16,"question":"x","covered_slots":[],"open_slots":[],"confidence":0.5,"done":false,"rationale":"x"}"#;
    let p =
        parse_turn_payload(raw).expect("parse_turn_payload should accept; validation is layered");
    assert_eq!(p.turn, 16);
}
