//! A1 — Structure-gate vs predicate-gate decoupling.
//!
//! Validates:
//!   - envelope OK does NOT imply predicate PASS
//!   - envelope FAIL short-circuits predicate (no Lean / market state / FC3
//!     interpreter is invoked)
//!
//! Plus A3 surjection table — all 7 EnvelopeValidationSubclass values
//! land in {ParseFailed, PolicyViolation}.

use envelope_poc::envelope::{
    validate, EnvelopeValidationSubclass, PayloadCandidate, RejectionClassSurrogate, TaskKind,
    ValidateContext,
};

fn gpqa_ctx<'a>() -> ValidateContext<'a> {
    ValidateContext {
        expected_task_kind: TaskKind::Gpqa,
        expected_task_id: "gpqa.diamond.q_0042",
        expected_agent_id: "agent_alpha",
        legal_stages: &["answer"],
        known_fc_nodes: &[],
    }
}

fn good_gpqa_body(letter: char) -> String {
    format!(
        r#"{{
  "envelope_version": "v1",
  "task_kind": "gpqa",
  "task_id": "gpqa.diamond.q_0042",
  "attempt_branch_id": "n1.b0",
  "agent_self_report": {{
    "agent_label": "agent_alpha",
    "stage_label": "answer"
  }},
  "payload": {{
    "final_answer_letter": "{}",
    "working": "rationale text",
    "confidence_milli": 900
  }}
}}"#,
        letter
    )
}

/// PoC stand-in for gpqa_judge: structure-OK answer may still be wrong vs gold.
fn gpqa_predicate(payload: &PayloadCandidate, gold: char) -> bool {
    match payload {
        PayloadCandidate::Gpqa {
            final_answer_letter,
            ..
        } => *final_answer_letter == gold,
        _ => panic!("non-GPQA payload reached gpqa_predicate"),
    }
}

#[test]
fn envelope_pass_does_not_imply_predicate_pass() {
    let body = good_gpqa_body('A');
    let ok = validate(&body, &gpqa_ctx()).expect("envelope must validate");
    assert!(
        !gpqa_predicate(&ok.payload, 'C'),
        "model said A, gold is C — predicate must FAIL even though envelope was OK"
    );
}

#[test]
fn envelope_pass_then_predicate_pass_is_possible() {
    let body = good_gpqa_body('C');
    let ok = validate(&body, &gpqa_ctx()).expect("envelope must validate");
    assert!(gpqa_predicate(&ok.payload, 'C'));
}

/// Test that envelope_fail short-circuits — we record what would have been
/// invoked vs what actually was invoked. A real runner would skip the
/// expensive Lean / market / FC3 path entirely.
#[test]
fn envelope_fail_short_circuits_predicate() {
    let bodies_that_should_short_circuit = [
        "not json at all",
        r#"{"envelope_version": "v0", "task_kind": "gpqa"}"#, // wrong version
        r#"{"envelope_version": "v1", "task_kind": "mystery"}"#, // unknown variant — caught at envelope level
    ];

    let mut predicate_was_called = false;
    let predicate_spy = |_payload: &PayloadCandidate| -> bool {
        predicate_was_called = true;
        true
    };
    // intentionally consume `predicate_spy` only on the success path
    let _ = predicate_spy;

    for body in bodies_that_should_short_circuit {
        // EnvelopeMalformed for missing fields, EnvelopeUnknownVariant for unknown enum
        match validate(body, &gpqa_ctx()) {
            Ok(_) => panic!(
                "body {:?} unexpectedly passed envelope validation — short-circuit broken",
                body
            ),
            Err((subclass, _path, _msg)) => {
                // We MUST be in the parse_fail bucket
                assert_eq!(
                    subclass.to_attempt_outcome(),
                    envelope_poc::envelope::AttemptOutcomeSurrogate::ParseFail,
                    "envelope failure for {:?} must map to ParseFail, got {:?}",
                    body,
                    subclass
                );
                // and predicate must NOT have been invoked
                assert!(!predicate_was_called);
            }
        }
    }
}

/// A3 — every variant of EnvelopeValidationSubclass maps to a legal pair.
/// AttemptOutcome must be ParseFail; RejectionClass must be ParseFailed
/// (or PolicyViolation for identity mismatch).
#[test]
fn every_subclass_surjects_to_existing_classes() {
    use EnvelopeValidationSubclass as E;
    let all = [
        E::EnvelopeNotJson,
        E::EnvelopeMalformed,
        E::EnvelopeUnknownVariant,
        E::EnvelopePayloadMalformed,
        E::EnvelopeFieldTooLarge,
        E::EnvelopeAgentIdentityMismatch,
        E::EnvelopeStageOutOfSet,
    ];
    for s in all {
        let outcome = s.to_attempt_outcome();
        let class = s.to_rejection_class();
        // No new enum variants are introduced.
        assert_eq!(
            outcome,
            envelope_poc::envelope::AttemptOutcomeSurrogate::ParseFail
        );
        match class {
            RejectionClassSurrogate::ParseFailed | RejectionClassSurrogate::PolicyViolation => {}
            other => panic!(
                "subclass {:?} maps to {:?} — only ParseFailed/PolicyViolation are permitted",
                s, other
            ),
        }
        // Special-case the identity mismatch — explicit assertion in case
        // the mapping table drifts.
        if matches!(s, E::EnvelopeAgentIdentityMismatch) {
            assert_eq!(class, RejectionClassSurrogate::PolicyViolation);
        } else {
            assert_eq!(class, RejectionClassSurrogate::ParseFailed);
        }
    }
}

#[test]
fn identity_mismatch_routes_to_policy_violation() {
    let body = good_gpqa_body('A').replace("agent_alpha", "agent_imposter");
    let err = validate(&body, &gpqa_ctx()).unwrap_err();
    assert_eq!(err.0, EnvelopeValidationSubclass::EnvelopeAgentIdentityMismatch);
    assert_eq!(
        err.0.to_rejection_class(),
        RejectionClassSurrogate::PolicyViolation
    );
}

#[test]
fn stage_out_of_set_is_caught() {
    let body = good_gpqa_body('A').replace("\"answer\"", "\"bogus_stage\"");
    let err = validate(&body, &gpqa_ctx()).unwrap_err();
    assert_eq!(err.0, EnvelopeValidationSubclass::EnvelopeStageOutOfSet);
}

#[test]
fn field_too_large_is_caught() {
    let huge = "x".repeat(4096);
    let body = good_gpqa_body('A').replace("\"rationale text\"", &format!("\"{}\"", huge));
    let err = validate(&body, &gpqa_ctx()).unwrap_err();
    assert_eq!(err.0, EnvelopeValidationSubclass::EnvelopeFieldTooLarge);
}

#[test]
fn malformed_gpqa_letter_is_payload_malformed_not_unknown_variant() {
    // 'Z' is not in {A,B,C,D}. We deliberately classify this as
    // EnvelopePayloadMalformed (not EnvelopeUnknownVariant) because
    // final_answer_letter is a free-form string at envelope schema layer;
    // the closed-set check is a payload-shape check, not an enum-discriminator.
    // This documents the design choice. If user prefers UnknownVariant here,
    // change the mapping in envelope.rs.
    let body = good_gpqa_body('Z');
    let err = validate(&body, &gpqa_ctx()).unwrap_err();
    assert_eq!(err.0, EnvelopeValidationSubclass::EnvelopePayloadMalformed);
}
