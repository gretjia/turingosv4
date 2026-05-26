//! Round-4 structural privacy fence (closes audit finding P1.4).
//!
//! Stronger than `robustness.rs::rejection_payload_carries_only_hash_prefix`
//! which only checked specific substrings. This test plants a unique sentinel
//! inside every agent-controlled envelope field and asserts the sentinel
//! NEVER appears in any rejection material:
//!   - validation error path (err.1) — system-controlled JSON path only
//!   - validation error message (err.2) — must be a content-free category label
//!   - EnvelopeRejectionPayload serialized form — must not echo agent bytes
//!
//! If this test fails, an attacker-controlled agent could exfiltrate PII /
//! secrets / chain-of-thought fragments through the L4.E rejection capsule.

use envelope_poc::envelope::{
    validate, EnvelopeRejectionPayload, EnvelopeValidationSubclass, TaskKind, ValidateContext,
};

const SENTINEL: &str = "SECRET_SENTINEL_8XQZ7K_DO_NOT_LEAK";

fn ctx_gpqa<'a>() -> ValidateContext<'a> {
    ValidateContext {
        expected_task_kind: TaskKind::Gpqa,
        expected_task_id: "gpqa.diamond.q_0042",
        expected_agent_id: "agent_alpha",
        legal_stages: &["answer"],
        known_fc_nodes: &[],
    }
}

fn assert_sentinel_absent(label: &str, body: &str) {
    let result = validate(body, &ctx_gpqa());
    let err = match result {
        Ok(_) => panic!("body {} unexpectedly validated", label),
        Err(e) => e,
    };
    let (subclass, path, message) = &err;
    assert!(
        !path.contains(SENTINEL),
        "[{}] sentinel leaked into err.path: {:?}",
        label,
        path
    );
    assert!(
        !message.contains(SENTINEL),
        "[{}] sentinel leaked into err.message: {:?}",
        label,
        message
    );

    // Build the actual CAS-bound payload and serialize it.
    let rejection = EnvelopeRejectionPayload::from(
        *subclass,
        path.as_str(),
        message.as_str(),
        body,
        TaskKind::Gpqa,
    );
    let serialized = serde_json::to_string(&rejection).expect("serializable");
    assert!(
        !serialized.contains(SENTINEL),
        "[{}] sentinel leaked into serialized EnvelopeRejectionPayload: {}",
        label,
        serialized
    );
}

#[test]
fn sentinel_in_task_id_does_not_leak() {
    let body = format!(
        r#"{{"envelope_version":"v1","task_kind":"gpqa",
            "task_id":"{}","attempt_branch_id":"n1.b0",
            "agent_self_report":{{"agent_label":"agent_alpha","stage_label":"answer"}},
            "payload":{{"final_answer_letter":"A","working":"x"}}}}"#,
        SENTINEL
    );
    assert_sentinel_absent("task_id", &body);
}

#[test]
fn sentinel_in_agent_label_does_not_leak() {
    let body = format!(
        r#"{{"envelope_version":"v1","task_kind":"gpqa",
            "task_id":"gpqa.diamond.q_0042","attempt_branch_id":"n1.b0",
            "agent_self_report":{{"agent_label":"{}","stage_label":"answer"}},
            "payload":{{"final_answer_letter":"A","working":"x"}}}}"#,
        SENTINEL
    );
    assert_sentinel_absent("agent_label", &body);
}

#[test]
fn sentinel_in_stage_label_does_not_leak() {
    let body = format!(
        r#"{{"envelope_version":"v1","task_kind":"gpqa",
            "task_id":"gpqa.diamond.q_0042","attempt_branch_id":"n1.b0",
            "agent_self_report":{{"agent_label":"agent_alpha","stage_label":"{}"}},
            "payload":{{"final_answer_letter":"A","working":"x"}}}}"#,
        SENTINEL
    );
    assert_sentinel_absent("stage_label", &body);
}

#[test]
fn sentinel_in_envelope_version_does_not_leak() {
    let body = format!(
        r#"{{"envelope_version":"{}","task_kind":"gpqa",
            "task_id":"gpqa.diamond.q_0042","attempt_branch_id":"n1.b0",
            "agent_self_report":{{"agent_label":"agent_alpha","stage_label":"answer"}},
            "payload":{{"final_answer_letter":"A","working":"x"}}}}"#,
        SENTINEL
    );
    assert_sentinel_absent("envelope_version", &body);
}

#[test]
fn sentinel_in_task_kind_does_not_leak() {
    let body = format!(
        r#"{{"envelope_version":"v1","task_kind":"{}",
            "task_id":"gpqa.diamond.q_0042","attempt_branch_id":"n1.b0",
            "agent_self_report":{{"agent_label":"agent_alpha","stage_label":"answer"}},
            "payload":{{"final_answer_letter":"A","working":"x"}}}}"#,
        SENTINEL
    );
    assert_sentinel_absent("task_kind", &body);
}

#[test]
fn sentinel_in_market_side_does_not_leak() {
    let body = format!(
        r#"{{"envelope_version":"v1","task_kind":"market_signal",
            "task_id":"polymarket.event.abc","attempt_branch_id":"n1.b0",
            "agent_self_report":{{"agent_label":"agent_alpha","stage_label":"answer"}},
            "payload":{{"event_id":"polymarket.event.abc","side":"{}","size_lots":1,
                        "rationale":"x","claimed_evidence_cids":[]}}}}"#,
        SENTINEL
    );
    let ctx = ValidateContext {
        expected_task_kind: TaskKind::MarketSignal,
        expected_task_id: "polymarket.event.abc",
        expected_agent_id: "agent_alpha",
        legal_stages: &["answer"],
        known_fc_nodes: &[],
    };
    let err = validate(&body, &ctx).unwrap_err();
    assert!(!err.1.contains(SENTINEL), "side sentinel leaked in path");
    assert!(!err.2.contains(SENTINEL), "side sentinel leaked in message");
    let rejection =
        EnvelopeRejectionPayload::from(err.0, err.1.clone(), err.2.clone(), &body, TaskKind::MarketSignal);
    assert!(
        !serde_json::to_string(&rejection).unwrap().contains(SENTINEL),
        "side sentinel leaked in serialized rejection payload"
    );
}

#[test]
fn sentinel_in_fc3_directive_kind_does_not_leak() {
    let body = format!(
        r#"{{"envelope_version":"v1","task_kind":"fc3_directive",
            "task_id":"fc3.feedback.run_0001","attempt_branch_id":"n1.b0",
            "agent_self_report":{{"agent_label":"agent_alpha","stage_label":"answer"}},
            "payload":{{"directive_kind":"{}","target_fc_node":"FC1-N42",
                        "rationale":"x","constitution_section_ref":"x"}}}}"#,
        SENTINEL
    );
    let ctx = ValidateContext {
        expected_task_kind: TaskKind::Fc3Directive,
        expected_task_id: "fc3.feedback.run_0001",
        expected_agent_id: "agent_alpha",
        legal_stages: &["answer"],
        known_fc_nodes: &["FC1-N42"],
    };
    let err = validate(&body, &ctx).unwrap_err();
    assert!(!err.1.contains(SENTINEL));
    assert!(!err.2.contains(SENTINEL));
    let rejection =
        EnvelopeRejectionPayload::from(err.0, err.1.clone(), err.2.clone(), &body, TaskKind::Fc3Directive);
    assert!(!serde_json::to_string(&rejection).unwrap().contains(SENTINEL));
}

#[test]
fn sentinel_in_fc3_target_node_does_not_leak() {
    let body = format!(
        r#"{{"envelope_version":"v1","task_kind":"fc3_directive",
            "task_id":"fc3.feedback.run_0001","attempt_branch_id":"n1.b0",
            "agent_self_report":{{"agent_label":"agent_alpha","stage_label":"answer"}},
            "payload":{{"directive_kind":"PROPOSE","target_fc_node":"{}",
                        "rationale":"x","constitution_section_ref":"x"}}}}"#,
        SENTINEL
    );
    let ctx = ValidateContext {
        expected_task_kind: TaskKind::Fc3Directive,
        expected_task_id: "fc3.feedback.run_0001",
        expected_agent_id: "agent_alpha",
        legal_stages: &["answer"],
        known_fc_nodes: &["FC1-N42"],
    };
    let err = validate(&body, &ctx).unwrap_err();
    assert!(!err.1.contains(SENTINEL));
    assert!(!err.2.contains(SENTINEL));
    let rejection =
        EnvelopeRejectionPayload::from(err.0, err.1.clone(), err.2.clone(), &body, TaskKind::Fc3Directive);
    assert!(!serde_json::to_string(&rejection).unwrap().contains(SENTINEL));
}

#[test]
fn serde_lexer_error_does_not_leak_sentinel() {
    // Body that fails JSON lex but contains the sentinel — historically the
    // serde error message would quote a fragment of the body. This test pins
    // the round-4 hardening that drops serde error text entirely.
    let body = format!("not_json_at_all but contains {}", SENTINEL);
    let err = validate(&body, &ctx_gpqa()).unwrap_err();
    assert_eq!(err.0, EnvelopeValidationSubclass::EnvelopeNotJson);
    assert!(!err.2.contains(SENTINEL));
}

#[test]
fn serde_shape_error_does_not_leak_sentinel() {
    // JSON-valid but envelope-shape-invalid; sentinel embedded in a value.
    // Pre-round-4, the serde::Error display might include "expected struct
    // ... got string \"...\"" — strip it.
    let body = format!(
        r#"{{"envelope_version":"v1","oops_extra_field":"{}"}}"#,
        SENTINEL
    );
    let err = validate(&body, &ctx_gpqa()).unwrap_err();
    assert_eq!(err.0, EnvelopeValidationSubclass::EnvelopeMalformed);
    assert!(!err.2.contains(SENTINEL));
}
