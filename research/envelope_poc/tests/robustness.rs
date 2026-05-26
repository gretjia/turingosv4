//! A5 — envelope rejection diagnostics carry more information than ad-hoc
//! parser. Also exercises A6 (privacy invariant — raw bytes never inside
//! EnvelopeRejectionPayload) and A7 (per-task-kind positive/negative).

use envelope_poc::envelope::{
    validate, AgentOutputEnvelope, AgentSelfReport, EnvelopeRejectionPayload,
    EnvelopeValidationSubclass, TaskKind, ValidateContext,
};
use serde_json::json;

// ── Synthetic fixture generator (deterministic) ─────────────────────────────

fn good_envelope(kind: TaskKind, payload: serde_json::Value) -> String {
    let env = AgentOutputEnvelope {
        envelope_version: "v1".to_string(),
        task_kind: kind.wire_label().to_string(),
        task_id: match kind {
            TaskKind::LeanStep => "lean.demo.t1".to_string(),
            TaskKind::Math500 => "math500.algebra.q1".to_string(),
            TaskKind::Gpqa => "gpqa.diamond.q_0001".to_string(),
            TaskKind::MarketSignal => "polymarket.event.abc".to_string(),
            TaskKind::Fc3Directive => "fc3.feedback.run_0001".to_string(),
        },
        attempt_branch_id: "n1.b0".to_string(),
        agent_self_report: AgentSelfReport {
            agent_label: "agent_alpha".to_string(),
            stage_label: "answer".to_string(),
            model_provider_hint: None,
        },
        payload,
    };
    serde_json::to_string_pretty(&env).unwrap()
}

fn ctx_for<'a>(kind: TaskKind, fc_nodes: &'a [&'a str]) -> ValidateContext<'a> {
    ValidateContext {
        expected_task_kind: kind,
        expected_task_id: match kind {
            TaskKind::LeanStep => "lean.demo.t1",
            TaskKind::Math500 => "math500.algebra.q1",
            TaskKind::Gpqa => "gpqa.diamond.q_0001",
            TaskKind::MarketSignal => "polymarket.event.abc",
            TaskKind::Fc3Directive => "fc3.feedback.run_0001",
        },
        expected_agent_id: "agent_alpha",
        legal_stages: &["answer"],
        known_fc_nodes: fc_nodes,
    }
}

// ── A7 positives — one good payload per task_kind ───────────────────────────

#[test]
fn lean_step_good_validates() {
    let body = good_envelope(
        TaskKind::LeanStep,
        json!({"lean_tactic_block": "exact rfl", "narration": "trivial", "claims_omega_complete": false}),
    );
    validate(&body, &ctx_for(TaskKind::LeanStep, &[])).expect("good lean_step must validate");
}

#[test]
fn math500_good_validates() {
    let body = good_envelope(
        TaskKind::Math500,
        json!({"final_answer_boxed": "\\boxed{42}", "working": "by inspection"}),
    );
    validate(&body, &ctx_for(TaskKind::Math500, &[])).expect("good math500 must validate");
}

#[test]
fn gpqa_good_validates() {
    let body = good_envelope(
        TaskKind::Gpqa,
        json!({"final_answer_letter": "B", "working": "ok"}),
    );
    validate(&body, &ctx_for(TaskKind::Gpqa, &[])).expect("good gpqa must validate");
}

#[test]
fn market_good_validates() {
    let body = good_envelope(
        TaskKind::MarketSignal,
        json!({
            "event_id": "polymarket.event.abc",
            "side": "YES",
            "size_lots": 5,
            "rationale": "trend",
            "claimed_evidence_cids": ["cid1", "cid2"]
        }),
    );
    validate(&body, &ctx_for(TaskKind::MarketSignal, &[])).expect("good market must validate");
}

#[test]
fn fc3_good_validates() {
    let body = good_envelope(
        TaskKind::Fc3Directive,
        json!({
            "directive_kind": "PROPOSE",
            "target_fc_node": "FC1-N42",
            "rationale": "tighten failure routing",
            "constitution_section_ref": "constitution.md §455"
        }),
    );
    validate(&body, &ctx_for(TaskKind::Fc3Directive, &["FC1-N42", "FC1-N41"]))
        .expect("good fc3 must validate");
}

// ── A7 negatives — each task_kind rejects the right way ─────────────────────

#[test]
fn math500_missing_boxed_marker_is_payload_malformed() {
    let body = good_envelope(
        TaskKind::Math500,
        json!({"final_answer_boxed": "42", "working": "by inspection"}),
    );
    let err = validate(&body, &ctx_for(TaskKind::Math500, &[])).unwrap_err();
    assert_eq!(err.0, EnvelopeValidationSubclass::EnvelopePayloadMalformed);
    assert!(err.1.contains("final_answer_boxed"));
}

#[test]
fn market_event_id_drift_is_caught() {
    let body = good_envelope(
        TaskKind::MarketSignal,
        json!({
            "event_id": "polymarket.event.someone_else",
            "side": "YES",
            "size_lots": 5,
            "rationale": "trend",
            "claimed_evidence_cids": []
        }),
    );
    let err = validate(&body, &ctx_for(TaskKind::MarketSignal, &[])).unwrap_err();
    assert_eq!(err.0, EnvelopeValidationSubclass::EnvelopePayloadMalformed);
    assert!(err.1.contains("event_id"));
}

#[test]
fn market_negative_size_is_caught() {
    let body = good_envelope(
        TaskKind::MarketSignal,
        json!({
            "event_id": "polymarket.event.abc",
            "side": "YES",
            "size_lots": -1,
            "rationale": "x",
            "claimed_evidence_cids": []
        }),
    );
    let err = validate(&body, &ctx_for(TaskKind::MarketSignal, &[])).unwrap_err();
    assert_eq!(err.0, EnvelopeValidationSubclass::EnvelopePayloadMalformed);
}

#[test]
fn market_unknown_side_is_unknown_variant() {
    let body = good_envelope(
        TaskKind::MarketSignal,
        json!({
            "event_id": "polymarket.event.abc",
            "side": "MAYBE",
            "size_lots": 1,
            "rationale": "x",
            "claimed_evidence_cids": []
        }),
    );
    let err = validate(&body, &ctx_for(TaskKind::MarketSignal, &[])).unwrap_err();
    assert_eq!(err.0, EnvelopeValidationSubclass::EnvelopeUnknownVariant);
}

#[test]
fn fc3_target_fc_node_not_in_known_set_is_payload_malformed() {
    let body = good_envelope(
        TaskKind::Fc3Directive,
        json!({
            "directive_kind": "PROPOSE",
            "target_fc_node": "FC-FAKE-999",
            "rationale": "x",
            "constitution_section_ref": "constitution.md §???"
        }),
    );
    let err = validate(
        &body,
        &ctx_for(TaskKind::Fc3Directive, &["FC1-N42", "FC1-N41"]),
    )
    .unwrap_err();
    assert_eq!(err.0, EnvelopeValidationSubclass::EnvelopePayloadMalformed);
    assert!(err.1.contains("target_fc_node"));
}

#[test]
fn fc3_unknown_directive_kind_is_unknown_variant() {
    let body = good_envelope(
        TaskKind::Fc3Directive,
        json!({
            "directive_kind": "DEMAND",
            "target_fc_node": "FC1-N42",
            "rationale": "x",
            "constitution_section_ref": "constitution.md §455"
        }),
    );
    let err = validate(
        &body,
        &ctx_for(TaskKind::Fc3Directive, &["FC1-N42"]),
    )
    .unwrap_err();
    assert_eq!(err.0, EnvelopeValidationSubclass::EnvelopeUnknownVariant);
}

// ── A6 privacy invariant — EnvelopeRejectionPayload never carries raw body ──

#[test]
fn rejection_payload_carries_only_hash_prefix() {
    let raw = "highly sensitive raw LLM response with private CoT here";
    let payload = EnvelopeRejectionPayload::from(
        EnvelopeValidationSubclass::EnvelopeMalformed,
        "$",
        "shape mismatch",
        raw,
        TaskKind::Gpqa,
    );
    let serialized = serde_json::to_string(&payload).expect("serializable");
    // privacy invariant: full raw body MUST NOT appear in serialized bytes
    assert!(
        !serialized.contains("highly sensitive raw LLM response"),
        "CR-18R.4 v2: raw body must never appear in EnvelopeRejectionPayload"
    );
    assert!(!serialized.contains("private CoT"));
    // hash prefix shape: 8 hex chars
    assert_eq!(payload.raw_body_sha256_prefix_8_hex.len(), 8);
    assert!(payload
        .raw_body_sha256_prefix_8_hex
        .chars()
        .all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn rejection_payload_hash_is_deterministic_correlatable() {
    let raw = "same body";
    let a = EnvelopeRejectionPayload::from(
        EnvelopeValidationSubclass::EnvelopeNotJson,
        "$",
        "lex fail",
        raw,
        TaskKind::Gpqa,
    );
    let b = EnvelopeRejectionPayload::from(
        EnvelopeValidationSubclass::EnvelopeMalformed,
        "$",
        "shape fail",
        raw,
        TaskKind::Gpqa,
    );
    // Two different subclass classifications, same raw body — hash prefix
    // SAME, enables audit-side correlation without leaking content.
    assert_eq!(a.raw_body_sha256_prefix_8_hex, b.raw_body_sha256_prefix_8_hex);
}

// ── A5 robustness — synthetic batch (50 items, deterministic) ───────────────

fn make_batch() -> Vec<(&'static str, String, Option<EnvelopeValidationSubclass>)> {
    // (label, body, expected_subclass_or_none_if_ok)
    let mut v: Vec<(&'static str, String, Option<EnvelopeValidationSubclass>)> = Vec::new();

    // 10 well-formed gpqa
    for letter in ['A', 'B', 'C', 'D'].iter().cycle().take(10) {
        let body = good_envelope(TaskKind::Gpqa, json!({"final_answer_letter": letter.to_string(), "working": "x"}));
        v.push(("gpqa_good", body, None));
    }
    // 10 truncated
    for i in 0..10 {
        v.push(("truncated", format!("{{ \"envelope_version\": \"v1\"   {}", "...".repeat(i)),
                Some(EnvelopeValidationSubclass::EnvelopeNotJson)));
    }
    // 10 wrong version
    for _ in 0..10 {
        let body = good_envelope(TaskKind::Gpqa, json!({"final_answer_letter": "A", "working": "x"}))
            .replace("\"v1\"", "\"v9\"");
        v.push(("wrong_version", body, Some(EnvelopeValidationSubclass::EnvelopeUnknownVariant)));
    }
    // 10 agent identity drift
    for _ in 0..10 {
        let body = good_envelope(TaskKind::Gpqa, json!({"final_answer_letter": "A", "working": "x"}))
            .replace("\"agent_alpha\"", "\"agent_imposter\"");
        v.push(("identity_drift", body, Some(EnvelopeValidationSubclass::EnvelopeAgentIdentityMismatch)));
    }
    // 10 oversize fields
    for _ in 0..10 {
        let huge = "X".repeat(4096);
        let body = good_envelope(TaskKind::Gpqa, json!({"final_answer_letter": "A", "working": huge}));
        v.push(("oversize", body, Some(EnvelopeValidationSubclass::EnvelopeFieldTooLarge)));
    }
    v
}

#[test]
fn robustness_batch_classification_matches_oracle() {
    let batch = make_batch();
    let mut ok_count = 0u64;
    let mut by_subclass = std::collections::BTreeMap::<String, u64>::new();
    for (label, body, expected) in &batch {
        match validate(body, &ctx_for(TaskKind::Gpqa, &[])) {
            Ok(_) => {
                assert!(expected.is_none(), "label {} unexpectedly validated", label);
                ok_count += 1;
            }
            Err((subclass, _, _)) => {
                assert!(
                    expected.is_some(),
                    "label {} expected to validate but got {:?}",
                    label,
                    subclass
                );
                assert_eq!(
                    Some(subclass),
                    *expected,
                    "label {} wrong subclass",
                    label
                );
                *by_subclass.entry(format!("{:?}", subclass)).or_insert(0) += 1;
            }
        }
    }
    assert_eq!(ok_count, 10);
    assert_eq!(
        by_subclass.get("EnvelopeNotJson").copied().unwrap_or(0),
        10
    );
    assert_eq!(
        by_subclass.get("EnvelopeUnknownVariant").copied().unwrap_or(0),
        10
    );
    assert_eq!(
        by_subclass
            .get("EnvelopeAgentIdentityMismatch")
            .copied()
            .unwrap_or(0),
        10
    );
    assert_eq!(
        by_subclass.get("EnvelopeFieldTooLarge").copied().unwrap_or(0),
        10
    );
    // diagnostic granularity: 50-attempt batch resolves into 5 distinct
    // categories (1 OK + 4 failure subclasses), all driven from the same
    // single validator entry. The old "everything → parse_fail" path
    // could only have produced 2 categories (OK + ParseFail).
    eprintln!(
        "robustness_batch ok={} subclasses={:?}",
        ok_count, by_subclass
    );
}
