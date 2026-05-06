//! TB-18R R4 — chain-derived attempt-count invariant ship-gate equation.
//!
//! Asserts `attempt_count_invariant()` Ok/Err semantics across all halt
//! classes per charter §1.2 FR-18R.3 v2 + FR-18R.4 v2 (G1-ratified
//! canonical contract; Codex Q1+Q4 remediation).
//!
//! See `handover/ai-direct/TB-18R_R4_STEP_B_invariant.md` §5 test plan.

use turingosv4::runtime::chain_derived_run_facts::{
    attempt_count_invariant, AttemptCountInvariantViolation, ChainDerivedRunFacts,
};
use turingosv4::state::typed_tx::RunOutcome;

fn facts(
    halt: RunOutcome,
    expected: u64,
    l4: u64,
    l4e: u64,
    aborted: u64,
) -> ChainDerivedRunFacts {
    let delta = (l4 as i64) + (l4e as i64) - (expected as i64);
    ChainDerivedRunFacts {
        expected_completed_attempts: expected,
        l4_work_attempt_count: l4,
        l4e_work_attempt_count: l4e,
        attempt_aborted_count: aborted,
        delta,
        terminal_halt_class: halt,
        ..ChainDerivedRunFacts::default()
    }
}

/// SG-18R.3 + FR-18R.3 v2: clean halt (`OmegaAccepted`) with delta=0 and
/// no aborted attempts → Ok.
#[test]
fn clean_halt_omega_accepted_invariant_passes() {
    // P49-shape: 32 LLM calls; 1 L4 omega + 31 L4.E failure = 32.
    let f = facts(RunOutcome::OmegaAccepted, 32, 1, 31, 0);
    assert_eq!(f.delta, 0);
    attempt_count_invariant(&f).expect("clean omega + delta=0 + aborted=0 must pass");
}

/// SG-18R.3 + FR-18R.3 v2: clean halt (`MaxTxExhausted`) with delta=0 and
/// no aborted attempts → Ok.
#[test]
fn clean_halt_max_tx_exhausted_invariant_passes() {
    // 50 LLM calls; 0 omega (no proof found) + 50 L4.E failures = 50.
    let f = facts(RunOutcome::MaxTxExhausted, 50, 0, 50, 0);
    assert_eq!(f.delta, 0);
    attempt_count_invariant(&f).expect("clean max-tx + delta=0 + aborted=0 must pass");
}

/// FR-18R.3 v2 violation: clean halt with non-zero delta → Err.
#[test]
fn clean_halt_with_nonzero_delta_invariant_fails() {
    // Evaluator reported 32 completed; chain has 33 (extra phantom attempt).
    let f = facts(RunOutcome::OmegaAccepted, 32, 1, 32, 0);
    assert_eq!(f.delta, 1);
    let err = attempt_count_invariant(&f).expect_err("non-zero delta on clean halt must fail");
    assert!(matches!(
        err,
        AttemptCountInvariantViolation::CleanHaltDeltaNonZero { delta: 1, .. }
    ));
}

/// FR-18R.4 v2 violation: clean halt with non-zero attempt_aborted_count → Err.
#[test]
fn clean_halt_with_nonzero_aborted_invariant_fails() {
    let f = facts(RunOutcome::MaxTxExhausted, 30, 0, 30, 2);
    assert_eq!(f.delta, 0);
    let err =
        attempt_count_invariant(&f).expect_err("non-zero aborted on clean halt must fail");
    assert!(matches!(
        err,
        AttemptCountInvariantViolation::CleanHaltAbortedNonZero {
            attempt_aborted_count: 2,
            ..
        }
    ));
}

/// FR-18R.3 v2 auxiliary equation: WallClockCap halt with
/// `expected + aborted == l4 + l4e` → Ok.
#[test]
fn wall_clock_cap_with_aborted_attempts_invariant_passes_when_balanced() {
    // 18 completed + 4 aborted = 22 starts; chain has 18 work entries
    // (aborted attempts didn't reach chain) and 4 TerminalAbortRecord CAS
    // objects. Auxiliary equation: 18 + 4 == 18 + 4 ✅ (the 4 aborted
    // attempts each got submitted to L4 or L4.E too, since the abort
    // happened mid-Lean after the L4/L4.E record was written).
    //
    // Concretely: 14 L4 omega-style + 4 L4.E failure-path = 18 work
    // entries on chain matching the 18 completed; the 4 aborted are the
    // ones the evaluator killed mid-Lean before they reached either pass
    // or fail outcome — chain-side accounting via the auxiliary equation:
    // expected + aborted == l4 + l4e+aborted_split.
    //
    // For this unit test: expected=18, aborted=4, l4=14, l4e=8 (the 8
    // includes 4 mid-abort failure-records + 4 'pure' parse/sorry rejects).
    // 18 + 4 == 14 + 8 ✅.
    let f = facts(RunOutcome::WallClockCap, 18, 14, 8, 4);
    attempt_count_invariant(&f).expect("balanced wall-clock-cap must pass");
}

/// FR-18R.3 v2 auxiliary equation: abort halt with mismatch → Err.
#[test]
fn wall_clock_cap_with_aborted_attempts_invariant_fails_when_unbalanced() {
    // expected=20, aborted=3, but chain has only 21 entries — missing 2.
    let f = facts(RunOutcome::WallClockCap, 20, 10, 11, 3);
    let err = attempt_count_invariant(&f)
        .expect_err("unbalanced abort halt must fail auxiliary equation");
    assert!(matches!(
        err,
        AttemptCountInvariantViolation::AbortHaltUnbalanced { .. }
    ));
}

/// FR-18R.3 v2: negative delta is forbidden under any halt class —
/// strictly an attempt-vanished-pre-chain ship-gate violation.
#[test]
fn negative_delta_always_fails_clean_halt() {
    // Evaluator reported 32 completed; chain has only 30 (2 vanished).
    let f = facts(RunOutcome::OmegaAccepted, 32, 1, 29, 0);
    assert_eq!(f.delta, -2);
    let err = attempt_count_invariant(&f).expect_err("negative delta must fail");
    assert!(matches!(
        err,
        AttemptCountInvariantViolation::NegativeDelta { delta: -2, .. }
    ));
}

#[test]
fn negative_delta_always_fails_abort_halt() {
    let f = facts(RunOutcome::ErrorHalt, 20, 5, 10, 5);
    assert_eq!(f.delta, -5);
    let err = attempt_count_invariant(&f).expect_err("negative delta must fail under abort halt too");
    assert!(matches!(
        err,
        AttemptCountInvariantViolation::NegativeDelta { delta: -5, .. }
    ));
}

/// Genesis edge: empty chain with zero expected — vacuously Ok.
#[test]
fn empty_chain_zero_expected_invariant_passes() {
    let f = facts(RunOutcome::OmegaAccepted, 0, 0, 0, 0);
    attempt_count_invariant(&f).expect("empty chain vacuous Ok");
}

/// DegradedLLM (TB-18 Atom A halt class) is an abort-class for invariant
/// purposes: balanced auxiliary equation → Ok.
#[test]
fn degraded_llm_with_aborted_attempts_invariant_passes_when_balanced() {
    let f = facts(RunOutcome::DegradedLLM, 5, 0, 5, 3);
    // expected + aborted = 5 + 3 = 8; l4 + l4e = 0 + 5 = 5 — UNBALANCED.
    let err = attempt_count_invariant(&f).expect_err("DegradedLLM with mismatch fails");
    assert!(matches!(
        err,
        AttemptCountInvariantViolation::AbortHaltUnbalanced { .. }
    ));

    // Now balanced: expected=2, aborted=3, l4=2, l4e=3 → 2+3 == 2+3 ✅.
    let f2 = facts(RunOutcome::DegradedLLM, 2, 2, 3, 3);
    attempt_count_invariant(&f2).expect("balanced DegradedLLM must pass");
}
