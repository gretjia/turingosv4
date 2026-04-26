// FC alignment conformance battery (minif2f_v4 sub-crate slice).
//
// Companion to /home/zephryj/projects/turingosv4/tests/fc_alignment_conformance.rs
// (which covers turingosv4 root crate symbols). This file covers FC anchors
// implemented in the minif2f_v4 sub-crate: Lean4Oracle (FC1-N12 ground-truth
// predicate), the B1-B4 PPUT accounting layer, and B7-extra rollback_sim.
//
// FC-trace: same as root file — FC1 + FC2 + FC3 + CLAUDE.md Alignment Standard.
// Source of mappings: handover/alignment/TRACE_MATRIX_v2_2026-04-25.md.
//
// Added 2026-04-25 in A0e-fix per Codex finding 4: the root file's
// fc1_n12_lean4_oracle_ground_truth_predicate stub claimed coverage in
// "experiments/minif2f_v4/tests/fc_alignment_conformance.rs" but that file
// did not exist. This file closes that gap.

#![allow(dead_code)]

use minif2f_v4::lean4_oracle::Lean4Oracle;
use minif2f_v4::rollback_sim::{
    rollback_simulation_enabled, should_simulate_rollback,
    ROLLBACK_ENV_VAR, ROLLBACK_TX_THRESHOLD,
};

// ─── FC1-N12: Lean4Oracle ground-truth predicate (the THESIS-V2 anchor) ───

#[test]
fn fc1_n12_lean4_oracle_constructible() {
    // FC1-N12 ground-truth oracle (the thesis claim 7 + 8 anchor).
    // Lean4Oracle is the white-box predicate that settles state on
    // ground-truth feedback (Lean compiler verdict). Witness: type
    // exists + is constructible.
    let _oracle = Lean4Oracle::new(
        "example : 1 + 1 = 2 := by norm_num".to_string(),
        "example".to_string(),
        "/usr/bin/lean-stub".to_string(),
    );
    let _ = std::any::type_name::<Lean4Oracle>();
}

// ─── B7-extra: rollback_sim (PREREG § 5.5 calibration toggle) ───
// FC-trace: FC1-E18 (∏p=0 → Q_t) repeated · FC2-N22 HALT MaxTxExhausted
// (per TRACE_MATRIX_v1 § 7.2 + v2 § 1).

#[test]
fn rollback_sim_threshold_constant_matches_prereg() {
    // PREREG § 5.5 frozen constant — must equal 50.
    assert_eq!(ROLLBACK_TX_THRESHOLD, 50);
}

#[test]
fn rollback_sim_env_var_canonical_name() {
    // Env-var name mirrors PREREG § 5.5 `--simulate-rollback-at-tx-50`.
    assert_eq!(ROLLBACK_ENV_VAR, "SIMULATE_ROLLBACK_AT_TX_50");
}

#[test]
fn rollback_sim_predicate_logic_at_threshold() {
    // FC1-E18 anchor: synthetic ∏p=0 fires exactly at tx == threshold,
    // exactly when env-driven enabled flag is true.
    assert!(should_simulate_rollback(50, true));
    assert!(!should_simulate_rollback(49, true));
    assert!(!should_simulate_rollback(51, true));
    assert!(!should_simulate_rollback(50, false));
    assert!(!should_simulate_rollback(0, true));
    assert!(!should_simulate_rollback(199, true));
}

#[test]
fn rollback_sim_env_check_function_present() {
    // Witness: function exists + returns bool. (Not testing env-driven
    // behavior here because process-global env is shared across parallel
    // cargo tests — see memory feedback_env_var_test_lock.)
    let _flag: bool = rollback_simulation_enabled();
}

// ─── B1-B4 PPUT accounting layer (TRACE_MATRIX_v1 § 3 readonly extension) ───

#[test]
fn b1_jsonl_schema_run_record_dispatcher_present() {
    // B1 — RunRecord::from_json schema_version dispatcher.
    // Witness: type exists + dispatch function callable on a v1 legacy line.
    use minif2f_v4::jsonl_schema::RunRecord;
    let legacy_v1 = r#"{"problem":"/tmp/foo.lean","condition":"n8","model":"deepseek-v4-flash","has_golden_path":false,"time_secs":1.0,"pput":0.0,"gp_token_count":0,"gp_node_count":0,"tx_count":1}"#;
    let parsed = RunRecord::from_json(legacy_v1);
    assert!(parsed.is_ok(), "B1: legacy v1 line must parse");
    match parsed.unwrap() {
        RunRecord::Legacy(_) => {}
        RunRecord::V2(_) => panic!("B1: legacy v1 line should dispatch to Legacy, got V2"),
    }
}

#[test]
fn b2_cost_aggregator_construct_and_record() {
    // B2 — RunCostAccumulator: full-cost C_i over all proposals.
    // Witness: type exists, basic record + total flow works.
    use minif2f_v4::cost_aggregator::RunCostAccumulator;
    let mut acc = RunCostAccumulator::new();
    acc.record_llm_call(100, 50);
    acc.record_proposal(false);
    let total = acc.total_run_token_count();
    assert!(total >= 150, "B2: total_run_token_count must include LLM call tokens");
}

#[test]
fn b3_wall_clock_first_read_to_final_accept() {
    // B3 — RunWallClock: T_i bracket from first prompt → final Lean accept.
    // Witness: type exists + idempotent first-mark + last-call-wins final-mark.
    use minif2f_v4::wall_clock::RunWallClock;
    let mut wc = RunWallClock::new();
    wc.mark_first_read();
    let _ = wc.elapsed_ms(); // valid even before final_accept (no panic)
    wc.mark_final_accept();
    let final_elapsed = wc.elapsed_ms();
    assert!(final_elapsed.is_some(), "B3: elapsed_ms must return Some after both marks");
}

#[test]
fn b4_post_hoc_verifier_progress_zero_on_runtime_reject() {
    // B4 — post-hoc verifier separates runtime claim from verified verdict.
    // Witness: progress_verified = 0 when runtime_accepted is false.
    use minif2f_v4::post_hoc_verifier::{compute_progress_runtime, compute_progress_verified};
    assert_eq!(compute_progress_runtime(false), 0);
    assert_eq!(compute_progress_runtime(true), 1);
    assert_eq!(compute_progress_verified(false, false), 0);
    assert_eq!(compute_progress_verified(true, false), 0); // runtime accept but post_hoc reject → 0
    assert_eq!(compute_progress_verified(true, true), 1);
}
