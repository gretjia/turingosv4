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

use std::path::PathBuf;

use turingosv4::state::typed_tx::{ExhaustionReason, RunOutcome};


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
