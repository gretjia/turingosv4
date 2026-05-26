//! A4 — FC1 LHS equality preserved when envelope sub-classes live inside
//! the parse_fail bucket.
//!
//! Canonical FC1 invariant (CLAUDE.md §4):
//!   evaluator_reported_completed_llm_calls
//!     = tool_dist.step + tool_dist.parse_fail + tool_dist.llm_err
//!
//! This test simulates a mixed-outcome batch and asserts that envelope
//! sub-classes ALL increment the parse_fail bucket — neither the step nor
//! the llm_err bucket changes, and the per-attempt-count LHS stays exact.

use envelope_poc::envelope::{
    validate, AttemptOutcomeSurrogate, EnvelopeValidationSubclass, TaskKind, ValidateContext,
};

#[derive(Default, Debug)]
struct ToolDist {
    step: u64,
    step_partial_ok: u64,
    step_reject: u64,
    parse_fail: u64,
    llm_err: u64,
    aborted: u64,
}

impl ToolDist {
    /// LHS bucket — per FC1 invariant.
    fn lhs(&self) -> u64 {
        self.step + self.parse_fail + self.llm_err
    }
}

/// Simulate a single attempt: classify its outcome and increment the
/// matching tool_dist bucket. Aborts are routed to `aborted` and NOT
/// counted in LHS, per the canonical scope.
fn record_attempt(dist: &mut ToolDist, outcome: AttemptOutcomeSurrogate) {
    match outcome {
        AttemptOutcomeSurrogate::LeanPass => dist.step += 1,
        AttemptOutcomeSurrogate::PartialAccepted => dist.step_partial_ok += 1,
        AttemptOutcomeSurrogate::LeanFail => dist.step_reject += 1,
        AttemptOutcomeSurrogate::ParseFail => dist.parse_fail += 1,
        AttemptOutcomeSurrogate::SorryBlock => dist.parse_fail += 1, // mirrors evaluator.rs:3275 logic
        AttemptOutcomeSurrogate::LlmErr => dist.llm_err += 1,
        AttemptOutcomeSurrogate::Aborted => dist.aborted += 1,
    }
}

fn gpqa_ctx<'a>() -> ValidateContext<'a> {
    ValidateContext {
        expected_task_kind: TaskKind::Gpqa,
        expected_task_id: "gpqa.diamond.q_0001",
        expected_agent_id: "agent_alpha",
        legal_stages: &["answer"],
        known_fc_nodes: &[],
    }
}

fn good_body() -> String {
    r#"{"envelope_version":"v1","task_kind":"gpqa","task_id":"gpqa.diamond.q_0001","attempt_branch_id":"n1.b0","agent_self_report":{"agent_label":"agent_alpha","stage_label":"answer"},"payload":{"final_answer_letter":"A","working":"x"}}"#.to_string()
}

fn drive_attempt(body_or_synthetic: AttemptInput) -> AttemptOutcomeSurrogate {
    match body_or_synthetic {
        AttemptInput::EnvelopeAndPredicate { body, predicate_pass } => {
            match validate(&body, &gpqa_ctx()) {
                Ok(_ok) => {
                    if predicate_pass {
                        AttemptOutcomeSurrogate::LeanPass
                    } else {
                        AttemptOutcomeSurrogate::LeanFail
                    }
                }
                Err((subclass, _, _)) => subclass.to_attempt_outcome(),
            }
        }
        AttemptInput::SyntheticLlmError => AttemptOutcomeSurrogate::LlmErr,
        AttemptInput::SyntheticAbort => AttemptOutcomeSurrogate::Aborted,
    }
}

enum AttemptInput {
    EnvelopeAndPredicate { body: String, predicate_pass: bool },
    SyntheticLlmError,
    SyntheticAbort,
}

#[test]
fn fc1_lhs_invariant_holds_under_mixed_batch() {
    // 12 attempts with carefully chosen mix:
    //   4 envelope OK + predicate PASS  -> step
    //   2 envelope OK + predicate FAIL  -> step_reject (NOT in LHS)
    //   3 envelope FAIL (various subclasses) -> parse_fail
    //   2 LLM error (synthetic) -> llm_err
    //   1 abort (synthetic, NOT in LHS)
    //
    // LHS expected: 4 (step) + 3 (parse_fail) + 2 (llm_err) = 9.
    // step_reject and aborted are intentionally NOT in LHS per CLAUDE.md §4.
    let inputs = vec![
        AttemptInput::EnvelopeAndPredicate { body: good_body(), predicate_pass: true },
        AttemptInput::EnvelopeAndPredicate { body: good_body(), predicate_pass: true },
        AttemptInput::EnvelopeAndPredicate { body: good_body(), predicate_pass: true },
        AttemptInput::EnvelopeAndPredicate { body: good_body(), predicate_pass: true },
        AttemptInput::EnvelopeAndPredicate { body: good_body(), predicate_pass: false },
        AttemptInput::EnvelopeAndPredicate { body: good_body(), predicate_pass: false },
        AttemptInput::EnvelopeAndPredicate {
            body: "garbage".to_string(),
            predicate_pass: false,
        },
        AttemptInput::EnvelopeAndPredicate {
            body: good_body().replace("\"v1\"", "\"v9\""),
            predicate_pass: false,
        },
        AttemptInput::EnvelopeAndPredicate {
            body: good_body().replace("agent_alpha", "agent_imposter"),
            predicate_pass: false,
        },
        AttemptInput::SyntheticLlmError,
        AttemptInput::SyntheticLlmError,
        AttemptInput::SyntheticAbort,
    ];
    let total_attempts = inputs.len() as u64;

    let mut dist = ToolDist::default();
    for inp in inputs {
        let outcome = drive_attempt(inp);
        record_attempt(&mut dist, outcome);
    }

    // Per-bucket assertions
    assert_eq!(dist.step, 4, "4 envelope OK + predicate PASS expected");
    assert_eq!(
        dist.step_reject, 2,
        "2 envelope OK + predicate FAIL expected — not in LHS"
    );
    assert_eq!(dist.parse_fail, 3, "3 envelope-fail attempts expected");
    assert_eq!(dist.llm_err, 2);
    assert_eq!(dist.aborted, 1);

    // Canonical LHS:
    assert_eq!(dist.lhs(), 9, "FC1 LHS = step + parse_fail + llm_err");

    // Sanity: aborted is OUT of LHS scope (per attempt_telemetry.rs:172-178)
    assert_eq!(dist.lhs() + dist.step_reject + dist.step_partial_ok + dist.aborted,
               total_attempts,
               "every attempt should land in exactly one bucket");
}

#[test]
fn envelope_subclass_diversity_within_parse_fail_bucket() {
    use EnvelopeValidationSubclass as E;
    // The 7 subclasses must all route to parse_fail bucket — i.e. they
    // increase the SAME counter, so they can be distinguished only by
    // the (AttemptTelemetry.tool_name dotted label, optional sub-class field).
    let all_subclasses = [
        E::EnvelopeNotJson,
        E::EnvelopeMalformed,
        E::EnvelopeUnknownVariant,
        E::EnvelopePayloadMalformed,
        E::EnvelopeFieldTooLarge,
        E::EnvelopeAgentIdentityMismatch,
        E::EnvelopeStageOutOfSet,
    ];
    let mut dist = ToolDist::default();
    for s in all_subclasses {
        record_attempt(&mut dist, s.to_attempt_outcome());
    }
    assert_eq!(dist.parse_fail, 7);
    assert_eq!(dist.step, 0);
    assert_eq!(dist.llm_err, 0);
    assert_eq!(dist.lhs(), 7);
}
