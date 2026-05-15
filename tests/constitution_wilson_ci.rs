//! Constitution gate: Art. I.2 — Statistical signal (PPUT / 95% CI report discipline).
//!
//! Closes the AMBER row "Art. I.2 Statistical signal" in
//! `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` §B.
//!
//! Kill condition (per matrix): "report missing ΣPPUT + Mean-PPUT(solved) +
//! Wilson 95% CI" — i.e. the report cannot omit the 95% CI band when
//! reporting aggregate solve rate.
//!
//! `FC-trace: Art.I.2 — PPUT report standard (CLAUDE.md §17)`.

use turingosv4::runtime::wilson_ci::WilsonCi;

#[test]
fn wilson_ci_helper_exists_and_returns_some_for_nonzero_trials() {
    // Kill: report aggregate solve rate without 95% CI capability.
    // This test asserts the helper exists and produces an interval for any
    // batch with at least one trial.
    let ci = WilsonCi::new_95(13, 50).expect("aggregate report MUST produce CI");
    assert!(ci.lower < ci.point && ci.point < ci.upper);
    assert!(ci.lower >= 0.0 && ci.upper <= 1.0);
}

#[test]
fn wilson_ci_handles_zero_solved_without_panic() {
    // Kill: aggregate report panics or omits CI when batch has zero solves.
    // Wilson must produce a finite upper bound (Wald would not).
    let ci = WilsonCi::new_95(0, 50).expect("trials > 0 must produce CI");
    assert_eq!(ci.point, 0.0);
    assert!(
        ci.upper > 0.0,
        "upper bound must be strictly positive at k=0"
    );
    assert!(ci.upper < 0.1, "upper bound must be plausible at k=0/50");
}

#[test]
fn wilson_ci_handles_full_solved_without_panic() {
    // Kill: aggregate report panics or omits CI when batch has all solves.
    let ci = WilsonCi::new_95(50, 50).expect("trials > 0 must produce CI");
    assert_eq!(ci.point, 1.0);
    assert!(
        ci.lower < 1.0,
        "lower bound must be strictly below 1 at k=n"
    );
}

#[test]
fn wilson_ci_format_includes_ci_band() {
    // Kill: aggregate report renders solve rate without CI band suffix.
    let ci = WilsonCi::new_95(13, 50).expect("trials > 0");
    let s = ci.format_percent();
    assert!(
        s.contains("CI "),
        "format string MUST include CI band: got {}",
        s,
    );
    assert!(s.contains("%"));
}

#[test]
fn wilson_ci_zero_trials_returns_none_not_panic() {
    // Kill: helper panics on degenerate input (would force callers to add
    // their own zero-check at every call site).
    assert!(WilsonCi::new_95(0, 0).is_none());
}
