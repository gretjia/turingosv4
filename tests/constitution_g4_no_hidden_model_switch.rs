//! G4.2 no-hidden-model-switch constitutional gate.
//!
//! Authority is the archived 2026-05-13 architect §8 ratification:
//! actual-vs-genesis mismatch is a hidden model switch, blocks audit, and
//! G4.2 introduces no model-switch event.

use serde::{Deserialize, Serialize};
use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::bottom_white::ledger::transition_ledger::canonical_encode;
use turingosv4::runtime::attempt_telemetry::{
    decode_attempt_telemetry_compat, AttemptKind, AttemptOutcome, AttemptTelemetry,
    ATTEMPT_TELEMETRY_SCHEMA_VERSION,
};
use turingosv4::runtime::audit_assertions::{
    audit_model_identity_records, HiddenSwitchCause, ModelIdentityAuditVerdict,
};
use turingosv4::runtime::genesis_report::AgentModelAssignment;
use turingosv4::runtime::proposal_telemetry::TokenCounts;
use turingosv4::state::q_state::{AgentId, Hash, TxId};

fn assignment(
    agent_id: &str,
    model_name: &str,
    family: &str,
    provider: &str,
) -> AgentModelAssignment {
    AgentModelAssignment {
        agent_id: agent_id.into(),
        model_name: model_name.into(),
        model_family: family.into(),
        model_provider: provider.into(),
        model_version: None,
        temperature_milli: 200,
        prompt_template_hash: Some("prompt-template-hash".into()),
    }
}

fn attempt(agent_id: &str, model_name: Option<&str>, family: Option<&str>) -> AttemptTelemetry {
    let mut t = AttemptTelemetry::new_root(
        TxId(format!("attempt-{agent_id}")),
        "run-g4-2".into(),
        "task-g4-2".into(),
        AgentId(agent_id.into()),
        "n0.b0".into(),
        Hash([0xAA; 32]),
        Cid::from_content(format!("candidate-{agent_id}").as_bytes()),
        AttemptKind::ExternalizedLlmCycle,
        AttemptOutcome::LeanFail,
        TokenCounts::default(),
        "step".into(),
    );
    t.model_name = model_name.map(str::to_owned);
    t.model_family = family.map(str::to_owned);
    t.model_provider = Some(
        match family {
            Some("claude") => "anthropic",
            Some("openai") => "openai",
            Some("qwen") => "qwen",
            Some("deepseek") => "deepseek",
            _ => "provider-under-test",
        }
        .into(),
    );
    t.model_version = None;
    t.temperature_milli = Some(200);
    t
}

#[test]
fn matching_actual_model_identity_proceeds() {
    let report = audit_model_identity_records(
        &[assignment(
            "Agent_0",
            "claude-3-7-sonnet",
            "claude",
            "anthropic",
        )],
        &[attempt(
            "Agent_0",
            Some("claude-3-7-sonnet"),
            Some("claude"),
        )],
    );

    assert_eq!(report.verdict, ModelIdentityAuditVerdict::Proceed);
    assert_eq!(report.total_attempts, 1);
    assert!(report.hidden_switches.is_empty());
}

#[test]
fn actual_vs_genesis_mismatch_blocks_as_hidden_switch() {
    let report = audit_model_identity_records(
        &[assignment(
            "Agent_0",
            "claude-3-7-sonnet",
            "claude",
            "anthropic",
        )],
        &[attempt("Agent_0", Some("gpt-5.2"), Some("openai"))],
    );

    assert_eq!(
        report.verdict,
        ModelIdentityAuditVerdict::Block,
        "architect Q4: any actual model vs genesis mismatch fails audit"
    );
    assert_eq!(report.hidden_switches.len(), 1);
    assert_eq!(
        report.hidden_switches[0].cause,
        HiddenSwitchCause::RuntimeProxyReroute
    );
    assert!(
        report
            .render_blocking_report()
            .contains("hidden model switch"),
        "mismatch must emit a blocking report, not delete or rewrite evidence"
    );
}

#[test]
fn missing_actual_attempt_model_identity_blocks() {
    let report = audit_model_identity_records(
        &[assignment(
            "Agent_0",
            "claude-3-7-sonnet",
            "claude",
            "anthropic",
        )],
        &[attempt("Agent_0", None, None)],
    );

    assert_eq!(report.verdict, ModelIdentityAuditVerdict::Block);
    assert_eq!(
        report.hidden_switches[0].cause,
        HiddenSwitchCause::MissingAttemptTelemetry,
        "new LLM attempts must populate actual model identity"
    );
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct AttemptTelemetryV1Wire {
    schema_version: u32,
    attempt_id: TxId,
    run_id: String,
    task_id: String,
    agent_id: AgentId,
    branch_id: String,
    parent_attempt_tx: Option<TxId>,
    attempt_index: u64,
    prompt_context_hash: Hash,
    candidate_payload_cid: Cid,
    lean_result_cid: Option<Cid>,
    attempt_kind: AttemptKind,
    outcome: AttemptOutcome,
    token_counts: TokenCounts,
    tool_name: String,
    proposal_telemetry_cid: Option<Cid>,
    verification_result_cid: Option<Cid>,
    attempt_chain_root: Option<Hash>,
}

#[test]
fn historical_attempt_telemetry_v1_bytes_still_decode() {
    let legacy = AttemptTelemetryV1Wire {
        schema_version: 1,
        attempt_id: TxId("attempt-legacy".into()),
        run_id: "run-legacy".into(),
        task_id: "task-legacy".into(),
        agent_id: AgentId("Agent_0".into()),
        branch_id: "n0.b0".into(),
        parent_attempt_tx: None,
        attempt_index: 0,
        prompt_context_hash: Hash([0xAB; 32]),
        candidate_payload_cid: Cid::from_content(b"candidate"),
        lean_result_cid: None,
        attempt_kind: AttemptKind::ExternalizedLlmCycle,
        outcome: AttemptOutcome::LeanFail,
        token_counts: TokenCounts::default(),
        tool_name: "step".into(),
        proposal_telemetry_cid: None,
        verification_result_cid: None,
        attempt_chain_root: None,
    };
    let bytes = canonical_encode(&legacy).expect("legacy v1 canonical encode");

    let decoded = decode_attempt_telemetry_compat(&bytes)
        .expect("G4.2 must keep old AttemptTelemetry evidence parseable");
    assert_eq!(decoded.attempt_id, TxId("attempt-legacy".into()));
    assert_eq!(decoded.schema_version, 1);
    assert!(decoded.model_name.is_none());
    assert!(decoded.model_family.is_none());
    assert!(
        ATTEMPT_TELEMETRY_SCHEMA_VERSION >= 2,
        "G4.2 adds durable actual-model fields and must use a schema bump or dual reader"
    );
}

#[test]
fn evaluator_success_attempts_record_proxy_reported_actual_model() {
    let evaluator = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("experiments/minif2f_v4/src/bin/evaluator.rs"),
    )
    .expect("read evaluator.rs");

    assert!(
        evaluator.contains("let actual_model_name = response.model.clone();"),
        "successful LLM responses must source actual model identity from proxy-reported response.model"
    );
    assert!(
        evaluator.matches("model_name: actual_model_name.as_str()").count() >= 5,
        "all successful AttemptTelemetry emit paths must write response.model, not requested agent_model"
    );
    assert_eq!(
        evaluator.matches("model_name: agent_model").count(),
        1,
        "only the no-response llm_err path may fall back to the requested model assignment"
    );
    assert!(
        evaluator.contains("No proxy response exists on this path"),
        "the llm_err fallback must be explicit because no proxy-reported actual model exists"
    );
}

#[test]
fn audit_tape_assertion_battery_blocks_hidden_model_switch() {
    let src = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/runtime/audit_assertions.rs"),
    )
    .expect("read audit_assertions.rs");

    assert!(
        src.contains("assert_g_no_hidden_model_switch"),
        "G4.2 hidden-switch detector must be an audit_tape assertion, not dashboard-only text"
    );
    assert!(
        src.contains("r.push(assert_g_no_hidden_model_switch(&tape));"),
        "run_all_assertions must push hidden-switch assertion so audit_tape blocks mismatch"
    );
}

#[test]
fn g4_model_assignment_manifest_write_is_fail_closed_for_new_runs() {
    let src = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("experiments/minif2f_v4/src/chain_runtime.rs"),
    )
    .expect("read chain_runtime.rs");

    assert!(
        src.contains("model assignment manifest CAS write failed")
            && src.contains("std::process::exit(3)"),
        "G4.2 resolver provenance manifest write must fail closed for new model-assignment runs"
    );
}

#[test]
fn g4_genesis_report_write_is_fail_closed_for_model_assignment_runs() {
    let src = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("experiments/minif2f_v4/src/chain_runtime.rs"),
    )
    .expect("read chain_runtime.rs");

    assert!(
        src.contains("genesis_report.json write failed")
            && src.contains("agent_model_assignment.is_empty()")
            && src.contains("model_assignment_manifest_cid.is_some()")
            && src.contains("std::process::exit(3)"),
        "new G4.2 model-assignment runs must fail closed if genesis_report.json cannot be written"
    );
}
