//! TB-18 Atom E (OBS_R023 closure; architect Q4 deferral cap = TB-18) —
//! `EvidenceCapsule.outcome` MUST be propagated from caller's actual
//! `RunOutcome`, not hardcoded literal.
//!
//! Architect ruling 2026-05-05 TB-18 charter SG-18.3 verbatim:
//!
//! > Hardcoded MaxTxExhausted literal removed; non-MaxTx outcome propagates.
//!
//! This test exercises:
//!
//! 1. **Static literal scan** of `experiments/minif2f_v4/src/bin/evaluator.rs`:
//!    after Atom E refactor, only ONE intentional `ExhaustionReason::MaxTxExhausted`
//!    literal must remain (the function-header default initialization of the
//!    `terminal_exhaustion_reason` variable). The two prior literal uses at
//!    the EvidenceCapsule write site + TerminalSummary emit site are GONE,
//!    replaced by `terminal_exhaustion_reason` + `.to_run_outcome()` projection.
//!    Likewise, `RunOutcome::MaxTxExhausted` literal MUST appear ZERO times in
//!    this binary (it was only at the TerminalSummary emit site, now removed).
//!
//! 2. **`to_run_outcome` projection contract** of
//!    `turingosv4::state::typed_tx::ExhaustionReason` — every variant maps to
//!    the canonical `RunOutcome` discriminant per Art.IV halt_reason taxonomy.
//!    This is the projection invoked by Atom E's refactor and is what makes
//!    propagation deterministic across the ExhaustionReason→RunOutcome
//!    boundary. (Atom A future-binding: `DegradedLLM` will be a NEW
//!    `ExhaustionReason` variant; this test will need to extend then.)
//!
//! Architect §2.5 forward-binding (Atom A): `RunOutcome::DegradedLLM` MUST
//! emit EvidenceCapsule + TerminalSummary + budget counters; that round-trip
//! test (`tb_18_degraded_llm_evidence_emission.rs`) will exercise the
//! propagation end-to-end on a synthetic non-MaxTx exit. Atom E itself
//! cannot construct a non-MaxTx exit (no halt path produces one yet); the
//! projection contract test below is the strongest available structural
//! guard at Atom E ship time.

use std::fs;
use std::path::PathBuf;

use turingosv4::state::typed_tx::{ExhaustionReason, RunOutcome};

const EVALUATOR_SRC: &str = "experiments/minif2f_v4/src/bin/evaluator.rs";

/// SG-18.3 structural guard #1 — `RunOutcome::MaxTxExhausted` literal MUST
/// NOT appear anywhere in `evaluator.rs`. The OBS_R023 refactor replaced
/// the only prior site (TerminalSummary emit) with
/// `terminal_exhaustion_reason.to_run_outcome()`.
#[test]
fn tb_18_e_no_run_outcome_max_tx_literal_in_evaluator() {
    let path = workspace_relative(EVALUATOR_SRC);
    let src = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));

    // Scan every line; any occurrence of `RunOutcome::MaxTxExhausted` as a
    // literal value (not in a doc-comment that was deliberately worded
    // around the literal) is a regression. Doc-comments in this codebase
    // referencing "MaxTxExhausted" use the bare word, not the enum-path
    // form, so the strict path-form scan is sufficient.
    let occurrences: Vec<(usize, &str)> = src
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains("RunOutcome::MaxTxExhausted"))
        .map(|(idx, line)| (idx + 1, line))
        .collect();

    assert!(
        occurrences.is_empty(),
        "TB-18 Atom E (OBS_R023 closure) regression: \
         `RunOutcome::MaxTxExhausted` literal found in {}. \
         Use `terminal_exhaustion_reason.to_run_outcome()` instead. \
         Matches: {:?}",
        path.display(),
        occurrences
    );
}

/// SG-18.3 structural guard #2 — `ExhaustionReason::MaxTxExhausted` literal
/// must appear EXACTLY ONCE in `evaluator.rs`: the function-header default
/// initialization of `terminal_exhaustion_reason`. The only other prior
/// occurrence (EvidenceCapsule write site) is now replaced by the variable.
///
/// Future Atom A may add additional literal occurrences ONLY in mutation
/// sites (e.g. `terminal_exhaustion_reason = ExhaustionReason::DegradedLLM;`)
/// but those are non-default; this test will either tolerate them
/// (≥1 match acceptable) or be re-tightened in the Atom A commit to count
/// exactly the expected default + DegradedLLM mutation site.
#[test]
fn tb_18_e_exhaustion_reason_max_tx_literal_appears_once() {
    let path = workspace_relative(EVALUATOR_SRC);
    let src = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));

    let occurrences: Vec<(usize, &str)> = src
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains("ExhaustionReason::MaxTxExhausted"))
        .map(|(idx, line)| (idx + 1, line))
        .collect();

    assert_eq!(
        occurrences.len(),
        1,
        "TB-18 Atom E (OBS_R023 closure) expects exactly ONE \
         `ExhaustionReason::MaxTxExhausted` literal in {} — the \
         function-header default initialization of \
         `terminal_exhaustion_reason`. Found {}: {:?}. \
         If Atom A added a mutation-site literal, re-tighten this test \
         to the new expected count.",
        path.display(),
        occurrences.len(),
        occurrences
    );
}

/// SG-18.3 projection contract — `ExhaustionReason::to_run_outcome()` is
/// the canonical mapping invoked by Atom E's refactor. Every variant must
/// project to the constitutionally-correct `RunOutcome` per Art.IV
/// halt_reason taxonomy. This test locks the projection so downstream
/// propagation is deterministic.
#[test]
fn tb_18_e_to_run_outcome_projection_contract() {
    assert_eq!(
        ExhaustionReason::MaxTxExhausted.to_run_outcome(),
        RunOutcome::MaxTxExhausted,
        "MaxTxExhausted must project to RunOutcome::MaxTxExhausted"
    );
    assert_eq!(
        ExhaustionReason::WallClockCap.to_run_outcome(),
        RunOutcome::WallClockCap,
        "WallClockCap must project to RunOutcome::WallClockCap"
    );
    assert_eq!(
        ExhaustionReason::ComputeCap.to_run_outcome(),
        RunOutcome::ComputeCap,
        "ComputeCap must project to RunOutcome::ComputeCap"
    );
    assert_eq!(
        ExhaustionReason::ProtocolCollapse.to_run_outcome(),
        RunOutcome::ErrorHalt,
        "ProtocolCollapse must project to RunOutcome::ErrorHalt \
         (RunOutcome is the constitutionally narrower 5-way taxonomy)"
    );
    assert_eq!(
        ExhaustionReason::SolverGiveUp.to_run_outcome(),
        RunOutcome::ErrorHalt,
        "SolverGiveUp must project to RunOutcome::ErrorHalt"
    );
    // TB-18 Atom A added 6th variant; projection is 1:1.
    assert_eq!(
        ExhaustionReason::DegradedLLM.to_run_outcome(),
        RunOutcome::DegradedLLM,
        "DegradedLLM must project to RunOutcome::DegradedLLM"
    );
}

fn workspace_relative(rel: &str) -> PathBuf {
    // Tests run from the workspace root via `cargo test --workspace`.
    // `CARGO_MANIFEST_DIR` for the top-level crate IS the workspace root.
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir).join(rel)
}
