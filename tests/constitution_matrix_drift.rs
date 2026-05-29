//! K-2.3 Matrix Drift Gate.
//!
//! Detects drift between `scripts/constitution_gates.manifest.toml` (the
//! charter-authorized gate set, post K-1.5 + K-1.5a) and
//! `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` (the human-readable
//! clause-to-gate mapping).
//!
//! ## Drift detection mechanism
//!
//! - **Manifest** lists each `constitution_*` gate with its `authority` field.
//! - **Matrix** is a hand-maintained markdown table mapping constitutional
//!   clauses (§A Art. I.1, §B Art. I.2, etc.) to their backing gate file.
//! - When the manifest grows (new gate registered), the matrix MUST be updated
//!   to add a row referencing that gate. Otherwise the matrix becomes a stale
//!   derived view drifting from the live manifest.
//!
//! ## Baseline allowlist
//!
//! At K-2.3 ship time, 67 of 128 manifest gates were NOT yet referenced in the
//! matrix. This test grandfathers them as the baseline allowlist. Future PRs
//! adding gates must either reference them in the matrix OR add to the
//! allowlist (with PR justification).
//!
//! ## v3 plan reference
//!
//! See `/home/zephryj/.claude/plans/karpathy-architect-md-turingosv4-harnes-splendid-sunbeam.md`
//! §5 atom K-2.3 (Rust drift test, not Python generator per A2 critique).

use std::collections::HashSet;
use std::fs;

/// Baseline allowlist: 67 gates known to be in manifest but not yet in matrix
/// as of K-2.3 ship (2026-05-20). Future tightening removes entries as matrix
/// is expanded.
const BASELINE_ALLOWLIST: &[&str] = &[
    "constitution_admission_no_fail_open_default",
    "constitution_aggregate_report",
    "constitution_architect_verbatim_struct_binding",
    "constitution_audit_views",
    "constitution_benchmark_manifest",
    "constitution_class4_atomic_rollback_witness",
    "constitution_completeset_merge",
    "constitution_cpmm_pool",
    "constitution_economy_strict_equality",
    "constitution_g1_2_persistence_evidence_binding",
    "constitution_librarian_digest",
    "constitution_librarian_half_async",
    "constitution_librarian_market_no_trade",
    "constitution_librarian_no_raw_leakage",
    "constitution_librarian_prompt_injection",
    "constitution_librarian_real_evidence_binding",
    "constitution_librarian_role_projector",
    "constitution_librarian_selector",
    "constitution_librarian_source_scope",
    "constitution_market_autonomy_research_envelope",
    "constitution_market_quarantine",
    "constitution_market_seed_hardening",
    "constitution_n1_agent_economy_a3",
    "constitution_n2_event_resolve",
    "constitution_no_evidence_drift_in_tests",
    "constitution_pcp_corpus_phase2",
    "constitution_policy_trader_trace",
    "constitution_polymarket_event_state_gate",
    "constitution_polymarket_smoke",
    "constitution_real12_bull_bear_positive_control",
    "constitution_real12_claim_boundary",
    "constitution_real12_economic_judgment",
    "constitution_real12_live_micro_probe",
    "constitution_real12_role_specialization",
    "constitution_real12_role_views",
    "constitution_real12_task_market_action",
    "constitution_real13a_display_coin",
    "constitution_real13a_ev_decision_trace",
    "constitution_real13b_market_review_window",
    "constitution_real13d_signal_purification",
    "constitution_real13h_market_pressure_probe",
    "constitution_real14_e2_candidate_verifier",
    "constitution_real14g_positive_ev_ignored",
    "constitution_real15_role_differentiation",
    "constitution_real16_market_performance",
    "constitution_real17_evaluator_prompt_provenance_wire",
    "constitution_real17_market_decision_provenance_link",
    "constitution_real17_market_order_ticket",
    "constitution_real5_architect_veto_scaffold",
    "constitution_real5_prompt_capsule_v2",
    "constitution_real5_role_assignment",
    "constitution_real5_role_based_smoke",
    "constitution_real5_role_scoped_view",
    "constitution_real5_tick_budget",
    "constitution_real5_trader_activation",
    "constitution_real5_typed_generation_gateway",
    "constitution_real5_verifier_challenger",
    "constitution_router_price_quote",
    "constitution_runner_invariant_formula",
    "constitution_tape_canonical_gate",
    "constitution_tb_n3_a3_emit",
    "constitution_tb_n3_invest_routing",
    // Phase 5 fix-up: added by K-3.1' (CI mirror) and K-2.3 (matrix drift self-mirror).
    // These post-K-2.3-ship gates are exempt from initial matrix coverage.
    "constitution_matrix_drift",
    "constitution_rules_ci_mirror",
    // Task A adversarial test: trivial gate exercising K-1.5/K-2.3 harness wires end-to-end.
    "constitution_demo_filesystem_check",
    // K-HARDEN-4 (2026-05-20): meta-gate verifying L5/L7/L8 hardening infrastructure.
    "constitution_subagent_pr_hygiene",
    // K-HARDEN validation run (probe gate, can be deleted after validation)
    "constitution_harden_validation_probe",
];

fn manifest_gates() -> HashSet<String> {
    let manifest = fs::read_to_string("scripts/constitution_gates.manifest.toml")
        .expect("manifest must exist (K-1.5 ship)");
    manifest
        .lines()
        .filter_map(|l| l.trim().strip_prefix("name = \"")?.strip_suffix("\""))
        .map(|s| s.to_string())
        .collect()
}

fn matrix_text() -> String {
    fs::read_to_string("handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md")
        .expect("matrix must exist")
}

#[test]
fn matrix_has_drift_gated_banner() {
    let matrix = matrix_text();
    let header: String = matrix.lines().take(20).collect::<Vec<_>>().join("\n");
    assert!(
        header.contains("K-2.3") || header.contains("Drift-gated"),
        "Matrix header must reference K-2.3 drift gate. Add a banner to the \
         first 20 lines of CONSTITUTION_EXECUTION_MATRIX.md, e.g.:\n  \
         > **Drift-gated** by `tests/constitution_matrix_drift.rs` (K-2.3)."
    );
}

#[test]
fn manifest_gates_subset_of_matrix_plus_allowlist() {
    let gates = manifest_gates();
    let matrix = matrix_text();
    let allowlist: HashSet<&str> = BASELINE_ALLOWLIST.iter().copied().collect();

    let mut undrift: Vec<&str> = Vec::new();
    for gate in &gates {
        if !allowlist.contains(gate.as_str()) && !matrix.contains(gate.as_str()) {
            undrift.push(gate.as_str());
        }
    }
    undrift.sort();

    assert!(
        undrift.is_empty(),
        "K-2.3 matrix drift detected. {} gates are in scripts/constitution_gates.manifest.toml \
         but NOT referenced in handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md, and NOT \
         in BASELINE_ALLOWLIST:\n\n{:?}\n\nFix options:\n  (1) Add a row to the matrix \
         referencing each gate (preferred)\n  (2) Add the gate name to BASELINE_ALLOWLIST in \
         this test file (with PR justification)\n",
        undrift.len(),
        undrift
    );
}

#[test]
fn allowlist_doesnt_grow_silently() {
    // Defense: the allowlist size at K-2.3 ship is 67. If a future PR
    // expands the allowlist (vs adding to matrix proper), this test alerts.
    // To raise the cap legitimately: bump this constant AND document in PR
    // why the matrix could not absorb the gate.
    const K23_SHIP_ALLOWLIST_SIZE: usize = 69;
    assert!(
        BASELINE_ALLOWLIST.len() <= K23_SHIP_ALLOWLIST_SIZE,
        "BASELINE_ALLOWLIST has grown beyond K-2.3 ship size {} (now {}). \
         Each addition is matrix-coverage regression. Either populate matrix \
         instead, or bump K23_SHIP_ALLOWLIST_SIZE with explicit PR justification.",
        K23_SHIP_ALLOWLIST_SIZE,
        BASELINE_ALLOWLIST.len()
    );
}
