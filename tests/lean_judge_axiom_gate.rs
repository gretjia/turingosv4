//! A08 LeanJudge axiom-gate tests.

use std::collections::BTreeSet;
use std::path::PathBuf;

use turingosv4::judges::lean_judge::{
    classify_axiom_report, AxiomCheckStatus, LeanJudge, DEFAULT_ALLOWED_AXIOMS,
    KERNEL_BYPASS_TOKENS,
};
use turingosv4::runtime::attempt_telemetry::{LeanErrorClass, LeanVerdictKind};

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
fn default_axiom_whitelist_is_explicit_and_minimal() {
    assert_eq!(DEFAULT_ALLOWED_AXIOMS, &["propext", "Quot.sound"]);
}

#[test]
fn axiom_probe_failure_is_fail_closed() {
    let allowed = BTreeSet::new();
    let report = classify_axiom_report("", "unknown declaration 'missing_theorem'", &allowed);

    assert_eq!(report.status, AxiomCheckStatus::AxiomProbeFailed);
    assert!(report.rejected_axioms.is_empty());
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

    assert_eq!(outcome.verdict_kind, LeanVerdictKind::SorryBlocked);
    assert_eq!(outcome.error_class, Some(LeanErrorClass::SorryBlocked));
    assert_eq!(
        outcome.axiom_check_status,
        AxiomCheckStatus::SourceForbiddenPattern
    );
    assert_eq!(outcome.rejected_axioms, Vec::<String>::new());
}
