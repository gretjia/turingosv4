//! A08 LeanJudge axiom-gate tests.

use std::collections::BTreeSet;
use std::path::PathBuf;

use turingosv4::judges::lean_judge::{
    classify_axiom_report, AxiomCheckStatus, LeanJudge, DEFAULT_ALLOWED_AXIOMS, AXIOM_WHITELIST,
    KERNEL_BYPASS_TOKENS,
};
use turingosv4::runtime::attempt_telemetry::{VerifierErrorClass, VerifierVerdictKind};

#[test]
fn axiom_report_with_no_axioms_passes_whitelist_gate() {
    let allowed = BTreeSet::new();
    let report = classify_axiom_report("'t' does not depend on any axioms\n", "", &allowed);

    assert_eq!(report.status, AxiomCheckStatus::PassedWhitelisted);
    assert!(report.rejected_axioms.is_empty());
}

#[test]
fn axiom_report_with_non_whitelisted_axiom_is_rejected() {
    let allowed = BTreeSet::from(["propext".to_string()]);
    let report = classify_axiom_report(
        "t depends on axioms: [propext, Classical.choice]\n",
        "",
        &allowed,
    );

    assert_eq!(report.status, AxiomCheckStatus::RejectedNonWhitelisted);
    assert_eq!(report.rejected_axioms, vec!["Classical.choice".to_string()]);
}

#[test]
fn default_axiom_whitelist_matches_banked_classical_base() {
    // DEFAULT_ALLOWED_AXIOMS must equal AXIOM_WHITELIST (the banked classical base).
    // Classical.choice is a BANKED axiom required by het det-family proofs; excluding
    // it from the default caused valid banked-classical proofs to be wrongly rejected.
    // Non-banked axioms (sorryAx, Lean.ofReduceBool, Lean.trustCompiler, hand-declared)
    // must remain absent — this test pins the exact set, not just a subset.
    assert_eq!(
        DEFAULT_ALLOWED_AXIOMS,
        &["propext", "Classical.choice", "Quot.sound"],
        "DEFAULT_ALLOWED_AXIOMS must equal the banked classical base (AXIOM_WHITELIST)"
    );
}

#[test]
fn axiom_probe_failure_is_fail_closed() {
    let allowed = BTreeSet::new();
    let report = classify_axiom_report("", "unknown declaration 'missing_theorem'", &allowed);

    assert_eq!(report.status, AxiomCheckStatus::AxiomProbeFailed);
    assert!(report.rejected_axioms.is_empty());
}

// ── §11.5 regression: banked-classical proofs must pass the DEFAULT gate ─────────────────

/// Classical.choice is BANKED (§11.5, AXIOM_WHITELIST). A proof whose transitive axiom set
/// is exactly {Classical.choice} must be accepted by the DEFAULT LeanJudge allowed_axioms
/// (i.e., classify_axiom_report with DEFAULT_ALLOWED_AXIOMS yields PassedWhitelisted).
/// Prior bug: DEFAULT_ALLOWED_AXIOMS excluded Classical.choice → valid het det-family proofs
/// were wrongly rejected by verify_axioms_after_success.
#[test]
fn banked_classical_choice_axiom_passes_default_gate() {
    let allowed: BTreeSet<String> = DEFAULT_ALLOWED_AXIOMS
        .iter()
        .map(|s| s.to_string())
        .collect();
    // Simulate a proof that depends ONLY on Classical.choice (a het det-family typical footprint).
    let report = classify_axiom_report(
        "'lm_det_zero' depends on axioms: [Classical.choice]\n",
        "",
        &allowed,
    );
    assert_eq!(
        report.status,
        AxiomCheckStatus::PassedWhitelisted,
        "Classical.choice is banked and must pass DEFAULT_ALLOWED_AXIOMS gate; got rejected_axioms={:?}",
        report.rejected_axioms
    );
    assert!(
        report.rejected_axioms.is_empty(),
        "no axiom should be rejected for a banked-classical-only proof"
    );
}

/// The full banked classical base {propext, Classical.choice, Quot.sound} (all three AXIOM_WHITELIST
/// members) must all pass the DEFAULT gate — i.e., DEFAULT_ALLOWED_AXIOMS == AXIOM_WHITELIST.
#[test]
fn full_banked_classical_base_passes_default_gate() {
    let allowed: BTreeSet<String> = DEFAULT_ALLOWED_AXIOMS
        .iter()
        .map(|s| s.to_string())
        .collect();
    // Simulate a proof depending on all three banked axioms.
    let report = classify_axiom_report(
        "'thm' depends on axioms: [propext, Classical.choice, Quot.sound]\n",
        "",
        &allowed,
    );
    assert_eq!(
        report.status,
        AxiomCheckStatus::PassedWhitelisted,
        "all three AXIOM_WHITELIST axioms must pass DEFAULT gate; rejected={:?}",
        report.rejected_axioms
    );
}

/// Non-banked axiom Lean.ofReduceBool (the footprint of native_decide) MUST still be rejected
/// by both DEFAULT_ALLOWED_AXIOMS and AXIOM_WHITELIST — i.e., the fix does NOT admit
/// native_decide-trust into the default gate.
#[test]
fn native_decide_trust_axiom_rejected_by_default_and_whitelist() {
    // Test against DEFAULT_ALLOWED_AXIOMS.
    let allowed_default: BTreeSet<String> = DEFAULT_ALLOWED_AXIOMS
        .iter()
        .map(|s| s.to_string())
        .collect();
    let report_default = classify_axiom_report(
        "'thm' depends on axioms: [Lean.ofReduceBool, Lean.trustCompiler]\n",
        "",
        &allowed_default,
    );
    assert_eq!(
        report_default.status,
        AxiomCheckStatus::RejectedNonWhitelisted,
        "Lean.ofReduceBool (native_decide trust) must be rejected by DEFAULT_ALLOWED_AXIOMS"
    );
    assert!(
        report_default.rejected_axioms.contains(&"Lean.ofReduceBool".to_string()),
        "Lean.ofReduceBool must be in rejected_axioms: {:?}",
        report_default.rejected_axioms
    );

    // Test against AXIOM_WHITELIST (the #print-axioms gate in axiom_gate / lean_market_agent).
    let allowed_whitelist: BTreeSet<String> = AXIOM_WHITELIST
        .iter()
        .map(|s| s.to_string())
        .collect();
    let report_whitelist = classify_axiom_report(
        "'thm' depends on axioms: [Lean.ofReduceBool, Lean.trustCompiler]\n",
        "",
        &allowed_whitelist,
    );
    assert_eq!(
        report_whitelist.status,
        AxiomCheckStatus::RejectedNonWhitelisted,
        "Lean.ofReduceBool (native_decide trust) must be rejected by AXIOM_WHITELIST"
    );
}

/// sorryAx (the footprint of sorry/admit) MUST still be rejected by the default gate.
#[test]
fn sorry_ax_rejected_by_default_gate() {
    let allowed: BTreeSet<String> = DEFAULT_ALLOWED_AXIOMS
        .iter()
        .map(|s| s.to_string())
        .collect();
    let report = classify_axiom_report(
        "'thm' depends on axioms: [sorryAx]\n",
        "",
        &allowed,
    );
    assert_eq!(
        report.status,
        AxiomCheckStatus::RejectedNonWhitelisted,
        "sorryAx must be rejected by DEFAULT_ALLOWED_AXIOMS"
    );
    assert!(
        report.rejected_axioms.contains(&"sorryAx".to_string()),
        "sorryAx must appear in rejected_axioms: {:?}",
        report.rejected_axioms
    );
}

#[test]
fn unsafe_shortcut_is_source_rejected_before_lean_runs() {
    assert!(
        KERNEL_BYPASS_TOKENS.contains(&"unsafe"),
        "A08 requires unsafe source shortcuts to be forbidden"
    );

    let mut judge = LeanJudge::new("theorem t : True := by");
    judge.lean_bin = PathBuf::from("/definitely/not/a/lean/binary");
    let outcome = judge.verify("unsafe exact trivial");

    assert_eq!(
        outcome.verdict_kind,
        VerifierVerdictKind::IncompleteProofBlocked
    );
    assert_eq!(
        outcome.error_class,
        Some(VerifierErrorClass::IncompleteProofBlocked)
    );
    assert_eq!(
        outcome.axiom_check_status,
        AxiomCheckStatus::SourceForbiddenPattern
    );
    assert_eq!(outcome.rejected_axioms, Vec::<String>::new());
}
