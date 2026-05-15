//! Constitution Landing Gate — Runner Invariant Formula
//!
//! Prevents regression of the TB-18R Phase 3 P04/P05 inv1_match=False bug
//! identified 2026-05-07: Phase 3 runner script passed `tx_count` (broader,
//! includes admin scaffold) to `tb_18r_compute_invariant --expected-completed`
//! when binary expects `evaluator_reported_completed_llm_calls` (= LLM-Lean
//! cycle count = `tool_dist.step + tool_dist.parse_fail + tool_dist.llm_err`).
//!
//! This caused false NegativeDelta on mixed-tx problems where evaluator's
//! tx_count includes architect-mandated non-LLM scaffold:
//!   - TB-6 atom-3 synthetic preseed (taskopen-smoke / worktx-smoke-l4e-gate)
//!   - TB-C0 atom A.1 synthetic L4.E gate
//!   - sequencer system-terminal-summary
//!
//! Resolution: handover/alignment/OBS_TB18R_INV1_NONLLM_TX_2026-05-07.md
//! Canonical invariant: CLAUDE.md Report Standard line 80 (clarified 2026-05-07)
//!
//! `FC-trace: FC1-INV1` — protects FC1 hard invariant LHS scope.

use std::path::Path;

const RUNNER_SCRIPT: &str = "handover/tests/scripts/run_tb_18r_phase_3_evidence.sh";

fn read_script() -> String {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let path = Path::new(project_root).join(RUNNER_SCRIPT);
    std::fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!(
            "constitution_runner_invariant_formula: cannot read runner script {}: {e}",
            path.display()
        )
    })
}

/// The runner's `EXTRACTED_JSON` Python block MUST compute `completed_llm_calls`
/// as the sum of `step + parse_fail + llm_err` from `tool_dist`. Any deviation
/// (e.g., reverting to `tx_count`) is the regression we are guarding against.
#[test]
fn runner_extracts_completed_llm_calls_from_step_parse_fail_llm_err() {
    let script = read_script();
    assert!(
        script.contains("completed_llm_calls"),
        "runner script must define `completed_llm_calls` field in extracted_pput.json. \
         Resolution per OBS_TB18R_INV1_NONLLM_TX_2026-05-07.md."
    );
    assert!(
        script.contains("td.get(\"step\", 0)")
            && script.contains("td.get(\"parse_fail\", 0)")
            && script.contains("td.get(\"llm_err\", 0)"),
        "runner script's completed_llm_calls computation must include step + parse_fail + \
         llm_err from tool_dist (each corresponds to an r2_write_attempt_telemetry callsite). \
         If you removed any term, you broke FC1 invariant for problems exhibiting that path."
    );
}

/// `EXPECTED_COMPLETED` (passed to `tb_18r_compute_invariant --expected-completed`)
/// MUST be derived from `completed_llm_calls`, NOT from `tx_count`.
/// This guards against the literal P04/P05 bug.
#[test]
fn runner_passes_completed_llm_calls_to_invariant_binary() {
    let script = read_script();
    assert!(
        script.contains(r#"EXPECTED_COMPLETED="$(echo "$EXTRACTED_JSON" | python3 -c 'import json,sys; print(json.load(sys.stdin)["completed_llm_calls"])')""#),
        "EXPECTED_COMPLETED must be extracted from completed_llm_calls field, not tx_count. \
         The binary's invariant LHS is `evaluator_reported_completed_llm_calls`; passing \
         tx_count produces false NegativeDelta on mixed-tx runs (architect-mandated admin \
         scaffold — TB-6 atom-3 + TB-C0 atom A.1 + system-terminal-summary)."
    );
    assert!(
        !script.contains(r#"EXPECTED_COMPLETED="$(echo "$EXTRACTED_JSON" | python3 -c 'import json,sys; print(json.load(sys.stdin)["tx_count"])')""#),
        "REGRESSION GUARD: runner must NOT extract EXPECTED_COMPLETED from tx_count. \
         This was the P04/P05 bug closed 2026-05-07."
    );
}

/// `architect_inv1_check.json` Python block MUST use `evaluator_reported_completed_llm_calls`
/// as the canonical scope for the invariant (NOT `evaluator_reported_tx_count`).
/// The diagnostic field `evaluator_reported_tx_count_total` may still be emitted for visibility,
/// but the invariant comparison is against completed_llm_calls.
#[test]
fn architect_inv1_uses_completed_llm_calls_scope() {
    let script = read_script();
    assert!(
        script.contains(
            r#""architect_inv_1": "chain_attempt_count == evaluator_reported_completed_llm_calls""#
        ),
        "architect_inv1_check.json must declare invariant scope as completed_llm_calls. \
         CLAUDE.md Report Standard line 80 (clarified 2026-05-07) is the canonical source."
    );
    assert!(
        !script
            .contains(r#""architect_inv_1": "chain_attempt_count == evaluator_reported_tx_count""#),
        "REGRESSION GUARD: architect_inv1 must NOT use evaluator_reported_tx_count as scope. \
         P04/P05 demonstrated the gap (admin scaffold inflates tx_count)."
    );
}

/// CLAUDE.md Report Standard line 80 must reflect the clarified canonical invariant
/// (3-term FC1 line 33 alignment) post-2026-05-07.
#[test]
fn claude_md_report_standard_uses_canonical_invariant() {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let path = Path::new(project_root).join("CLAUDE.md");
    let content =
        std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("cannot read CLAUDE.md: {e}"));
    assert!(
        content.contains("evaluator_reported_completed_llm_calls"),
        "CLAUDE.md Report Standard must reference evaluator_reported_completed_llm_calls. \
         Clarification per OBS_TB18R_INV1_NONLLM_TX_2026-05-07.md."
    );
    assert!(
        content.contains("tool_dist.step + tool_dist.parse_fail + tool_dist.llm_err"),
        "CLAUDE.md must spell out the formula `step + parse_fail + llm_err` so future \
         analysis scripts (and AI coders) compute the right LHS."
    );
}
