//! TB-18R R2 Integration Test — per-LLM-call AttemptTelemetry shape per path.
//!
//! Maps to charter v2 SG-18R.1 ("Every externalized LLM-Lean cycle produces a
//! CAS AttemptTelemetry object. Test: `tb_18r_attempt_telemetry_per_llm_call.rs`
//! covering all 6 paths (step / step_partial_ok / step_reject / parse_fail /
//! llm_err / omega_wtool)").
//!
//! The 6 production paths in `experiments/minif2f_v4/src/bin/evaluator.rs`:
//!   1. omega-full          (line ~2317 of evaluator.rs; LeanPass, full payload)
//!   2. omega-pertactic     (line ~2861; LeanPass, per-tactic step)
//!   3. step_partial_ok     (line ~3236; LeanPass, intermediate, no proof artifact)
//!   4. step_reject         (line ~3263; LeanFail or SorryBlock)
//!   5. parse_fail          (line ~3275; ParseFail, no Lean invocation, sentinel candidate)
//!   6. llm_err             (line ~3289; LlmErr, no Lean invocation, sentinel candidate)
//!
//! These tests reconstruct the shape that `r2_write_attempt_telemetry` produces
//! and assert end-to-end CAS byte-identity for each path. They do NOT spin up
//! the full evaluator binary (which would require LLM API + Lean toolchain);
//! the actual evaluator wire-up is verified by R6/R7 evidence runs.
//!
//! TRACE_MATRIX FC1-N41 (TB-18R R2 NEW witness on the externalization paths).

use tempfile::TempDir;

use turingosv4::bottom_white::cas::schema::{Cid, ObjectType};
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::attempt_telemetry::{
    read_attempt_telemetry_from_cas, read_lean_result_from_cas, write_attempt_telemetry_to_cas,
    write_lean_result_to_cas, AttemptKind, AttemptOutcome, AttemptTelemetry, LeanErrorClass,
    LeanResult,
};
use turingosv4::runtime::proposal_telemetry::TokenCounts;
use turingosv4::state::q_state::{AgentId, Hash, TxId};

/// Mirror of the SHA-256 prompt_context_hash trick used in evaluator.rs §3.1
/// (Cid::from_content + cast to Hash; avoids adding sha2 to evaluator deps).
fn ctx_hash(prompt: &[u8]) -> Hash {
    Hash(Cid::from_content(prompt).0)
}

/// Helper that mirrors the production `r2_write_attempt_telemetry` flow
/// minus the FAIL-CLOSED exit handling — the test asserts on `Result`s
/// instead.
fn write_path(
    cas: &mut CasStore,
    attempt_id: &str,
    candidate_bytes: &[u8],
    tool_name: &str,
    outcome: AttemptOutcome,
    error_class: Option<LeanErrorClass>,
    lean_result: Option<(i32, bool)>,
    is_omega_success: bool,
) -> (Cid, AttemptTelemetry) {
    let attempt_id = TxId(attempt_id.to_string());
    let candidate_cid = cas
        .put(candidate_bytes, ObjectType::ProposalPayload, "test", 0, None)
        .expect("put candidate");
    let lean_result_cid = if let Some((exit_code, verified)) = lean_result {
        let proof_artifact_cid = if verified { Some(candidate_cid) } else { None };
        let verdict_kind =
            LeanResult::derive_verdict_kind_from_legacy_fields(exit_code, verified, error_class)
                .unwrap_or_default();
        let lr = LeanResult {
            attempt_id: attempt_id.clone(),
            exit_code,
            verified,
            stderr_cid: None,
            stdout_cid: None,
            proof_artifact_cid,
            error_class,
            verdict_kind,
        };
        Some(write_lean_result_to_cas(cas, &lr, "test", 0).expect("write lean result"))
    } else {
        None
    };
    let _ = is_omega_success; // attempt_id naming is the test's problem to choose
    let mut attempt = AttemptTelemetry::new_root(
        attempt_id,
        "test-run-r2".into(),
        "task-test-run-r2".into(),
        AgentId("agent_0".into()),
        "n0.b0".into(),
        ctx_hash(b"test prompt body"),
        candidate_cid,
        AttemptKind::ExternalizedLlmCycle,
        outcome,
        TokenCounts {
            prompt_tokens: 100,
            completion_tokens: 30,
            tool_tokens: 0,
        },
        tool_name.into(),
    );
    attempt.lean_result_cid = lean_result_cid;
    let cid = write_attempt_telemetry_to_cas(cas, &attempt, "test", 0).expect("write attempt");
    (cid, attempt)
}

#[test]
fn r2_path_1_omega_wtool_full_attempt_telemetry_shape() {
    let dir = TempDir::new().expect("tempdir");
    let mut cas = CasStore::open(dir.path()).expect("open cas");
    let parsed_candidate = b"theorem foo : 1 + 1 = 2 := by rfl";
    let (cid, original) = write_path(
        &mut cas,
        "worktx-task-test-run-r2-omega-full-0",
        parsed_candidate,
        "omega_wtool",
        AttemptOutcome::LeanPass,
        None,
        Some((0, true)),
        true,
    );
    let recovered = read_attempt_telemetry_from_cas(&cas, &cid).expect("read");
    assert_eq!(recovered, original);
    assert_eq!(recovered.outcome, AttemptOutcome::LeanPass);
    assert_eq!(recovered.tool_name, "omega_wtool");
    assert_eq!(recovered.attempt_chain_root, None);
    let lr = read_lean_result_from_cas(&cas, recovered.lean_result_cid.as_ref().unwrap())
        .expect("read lean result");
    assert_eq!(lr.exit_code, 0);
    assert!(lr.verified);
    assert!(lr.proof_artifact_cid.is_some(), "omega-full must carry a proof artifact CID");
    assert!(lr.error_class.is_none());
    assert_eq!(
        cas.get(&recovered.candidate_payload_cid).expect("get candidate"),
        parsed_candidate.to_vec(),
        "candidate_payload bytes must equal the parsed external candidate (NOT raw LLM response)"
    );
}

#[test]
fn r2_path_2_omega_wtool_pertactic_attempt_telemetry_shape() {
    let dir = TempDir::new().expect("tempdir");
    let mut cas = CasStore::open(dir.path()).expect("open cas");
    let tactic_bytes = b"exact rfl";
    let (cid, original) = write_path(
        &mut cas,
        "worktx-task-test-run-r2-omega-pertactic-3",
        tactic_bytes,
        "omega_wtool_pertactic",
        AttemptOutcome::LeanPass,
        None,
        Some((0, true)),
        true,
    );
    let recovered = read_attempt_telemetry_from_cas(&cas, &cid).expect("read");
    assert_eq!(recovered, original);
    assert_eq!(recovered.outcome, AttemptOutcome::LeanPass);
    assert_eq!(
        recovered.tool_name, "omega_wtool_pertactic",
        "path 2 disambiguates from path 1 via tool_name (preflight §3.7)"
    );
    let lr = read_lean_result_from_cas(&cas, recovered.lean_result_cid.as_ref().unwrap())
        .expect("read lean result");
    assert_eq!(lr.exit_code, 0);
    assert!(lr.verified);
    assert!(lr.proof_artifact_cid.is_some());
}

#[test]
fn r2_path_3_step_partial_ok_attempt_telemetry_shape() {
    let dir = TempDir::new().expect("tempdir");
    let mut cas = CasStore::open(dir.path()).expect("open cas");
    let tactic_bytes = b"intro x";
    let (cid, _) = write_path(
        &mut cas,
        "att-test-run-r2-agent_0-7-step_partial_ok",
        tactic_bytes,
        "step_partial_ok",
        AttemptOutcome::LeanPass,
        None,
        Some((0, false)),
        false,
    );
    let recovered = read_attempt_telemetry_from_cas(&cas, &cid).expect("read");
    assert_eq!(
        recovered.outcome,
        AttemptOutcome::LeanPass,
        "intermediate partial-accept maps to LeanPass per R1 mapping"
    );
    assert_eq!(recovered.tool_name, "step_partial_ok");
    let lr = read_lean_result_from_cas(&cas, recovered.lean_result_cid.as_ref().unwrap())
        .expect("read lean result");
    assert_eq!(lr.exit_code, 0);
    assert!(!lr.verified, "step_partial_ok did not produce a Complete verdict");
    assert!(
        lr.proof_artifact_cid.is_none(),
        "step_partial_ok is intermediate; no proof artifact"
    );
    assert!(lr.error_class.is_none());
}

#[test]
fn r2_path_4a_step_reject_lean_failed_attempt_telemetry_shape() {
    let dir = TempDir::new().expect("tempdir");
    let mut cas = CasStore::open(dir.path()).expect("open cas");
    let tactic_bytes = b"linarith";
    let (cid, _) = write_path(
        &mut cas,
        "att-test-run-r2-agent_0-9-step_reject",
        tactic_bytes,
        "step_reject",
        AttemptOutcome::LeanFail,
        Some(LeanErrorClass::LeanFailed),
        Some((1, false)),
        false,
    );
    let recovered = read_attempt_telemetry_from_cas(&cas, &cid).expect("read");
    assert_eq!(recovered.outcome, AttemptOutcome::LeanFail);
    let lr = read_lean_result_from_cas(&cas, recovered.lean_result_cid.as_ref().unwrap())
        .expect("read lean result");
    assert_eq!(lr.exit_code, 1);
    assert!(!lr.verified);
    assert!(lr.proof_artifact_cid.is_none());
    assert_eq!(
        lr.error_class,
        Some(LeanErrorClass::LeanFailed),
        "non-sorry rejection must classify as LeanFailed (mirrors R3 RejectionClass=6)"
    );
}

#[test]
fn r2_path_4b_step_reject_sorry_block_attempt_telemetry_shape() {
    let dir = TempDir::new().expect("tempdir");
    let mut cas = CasStore::open(dir.path()).expect("open cas");
    let tactic_bytes = b"sorry";
    let (cid, _) = write_path(
        &mut cas,
        "att-test-run-r2-agent_0-11-step_reject",
        tactic_bytes,
        "step_reject",
        AttemptOutcome::SorryBlock,
        Some(LeanErrorClass::SorryBlocked),
        Some((1, false)),
        false,
    );
    let recovered = read_attempt_telemetry_from_cas(&cas, &cid).expect("read");
    assert_eq!(
        recovered.outcome,
        AttemptOutcome::SorryBlock,
        "sorry / forbidden_payload rejection maps to SorryBlock (preflight §3.5)"
    );
    let lr = read_lean_result_from_cas(&cas, recovered.lean_result_cid.as_ref().unwrap())
        .expect("read lean result");
    assert_eq!(
        lr.error_class,
        Some(LeanErrorClass::SorryBlocked),
        "sorry rejection must classify as SorryBlocked (mirrors R3 RejectionClass=8)"
    );
}

#[test]
fn r2_path_5_parse_fail_attempt_telemetry_shape() {
    let dir = TempDir::new().expect("tempdir");
    let mut cas = CasStore::open(dir.path()).expect("open cas");
    // Per preflight §3.2: parse_fail uses a fixed sentinel — NEVER raw LLM
    // response. The sentinel byte string is a structural marker; R5 audit
    // fence may rely on this exact value to recognize the path.
    const PARSE_FAIL_SENTINEL: &[u8] = b"tb-18r-parse-fail-no-candidate";
    let (cid, _) = write_path(
        &mut cas,
        "att-test-run-r2-agent_0-13-parse_fail",
        PARSE_FAIL_SENTINEL,
        "parse_fail",
        AttemptOutcome::ParseFail,
        None,
        None,
        false,
    );
    let recovered = read_attempt_telemetry_from_cas(&cas, &cid).expect("read");
    assert_eq!(recovered.outcome, AttemptOutcome::ParseFail);
    assert!(
        recovered.lean_result_cid.is_none(),
        "Lean was not invoked on parse_fail; no LeanResult should exist"
    );
    let payload_bytes = cas.get(&recovered.candidate_payload_cid).expect("get sentinel");
    assert_eq!(
        payload_bytes,
        PARSE_FAIL_SENTINEL.to_vec(),
        "parse_fail candidate_payload must be the fixed sentinel marker (NOT raw LLM response)"
    );
}

#[test]
fn r2_path_6_llm_err_attempt_telemetry_shape() {
    let dir = TempDir::new().expect("tempdir");
    let mut cas = CasStore::open(dir.path()).expect("open cas");
    const LLM_ERR_SENTINEL: &[u8] = b"tb-18r-llm-err-no-candidate";
    let (cid, _) = write_path(
        &mut cas,
        "att-test-run-r2-agent_0-15-llm_err",
        LLM_ERR_SENTINEL,
        "llm_err",
        AttemptOutcome::LlmErr,
        None,
        None,
        false,
    );
    let recovered = read_attempt_telemetry_from_cas(&cas, &cid).expect("read");
    assert_eq!(recovered.outcome, AttemptOutcome::LlmErr);
    assert!(
        recovered.lean_result_cid.is_none(),
        "Lean was not invoked on llm_err; no LeanResult should exist"
    );
    let payload_bytes = cas.get(&recovered.candidate_payload_cid).expect("get sentinel");
    assert_eq!(
        payload_bytes,
        LLM_ERR_SENTINEL.to_vec(),
        "llm_err candidate_payload must be the fixed sentinel marker (privacy invariant CR-18R.4 v2)"
    );
}

#[test]
fn r2_six_paths_have_distinct_outcome_values() {
    // Sanity that the six R2 outcome values are all distinct discriminators
    // — guards against accidental enum collapse in a future R-series revision.
    use std::collections::HashSet;
    let outcomes: HashSet<u8> = [
        AttemptOutcome::LeanPass as u8,    // path 1
        AttemptOutcome::LeanPass as u8,    // path 2 (same kind, distinguished by tool_name)
        AttemptOutcome::LeanPass as u8,    // path 3 (intermediate; distinguished by proof_artifact_cid=None)
        AttemptOutcome::LeanFail as u8,    // path 4a
        AttemptOutcome::SorryBlock as u8,  // path 4b
        AttemptOutcome::ParseFail as u8,   // path 5
        AttemptOutcome::LlmErr as u8,      // path 6
    ]
    .iter()
    .copied()
    .collect();
    // Distinct discriminators: LeanPass, LeanFail, SorryBlock, ParseFail, LlmErr → 5 values
    // (paths 1/2/3 share LeanPass; path 4 splits into 4a/4b).
    assert_eq!(
        outcomes.len(),
        5,
        "five distinct AttemptOutcome discriminators across the six R2 paths"
    );
}
