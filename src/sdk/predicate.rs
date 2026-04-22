//! Predicate trait — abstracts ∏p verification across domains.
//!
//! Constitutional basis:
//! - Art. I.1   boolean predicate (Paper 1 MiniF2F, Paper 2 zeta_sum_proof)
//! - Art. I.1.1 PCP predicate     (Paper 3 omegav4, statistical + OOS + audit)
//!
//! Preservation contract (M-1, GENERALIZATION_ROADMAP §3):
//! This trait is NOT yet wired into the Lean4Oracle at runtime for Paper 1.
//! The goal is to keep the seam open so that Paper 2/3 can add new
//! `Predicate` implementations (StatisticalPCP, ExternalAudit) without
//! touching the bus or ledger.

use serde::{Deserialize, Serialize};

/// Three-way verdict on a payload.
///
/// Maps to `experiments/minif2f_v4/src/lean4_oracle.rs::PartialVerdict`
/// plus a `confidence` field for PCP predicates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Verdict {
    /// Payload fully satisfies the predicate (∏p = 1, terminal).
    Complete,
    /// Payload advances toward satisfaction but is not terminal.
    /// Boolean domain: partial proof with unsolved goals; confidence = 1.0.
    /// PCP domain: soundness proximity, confidence ∈ (0, 1].
    PartialOk { confidence: f64 },
    /// Payload violates the predicate (∏p = 0). Reason for error broadcast.
    Reject(String),
}

/// Which predicate domain emitted this verdict.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PredicateKind {
    /// Boolean Lean4 oracle (Paper 1 & 2).
    Lean4Boolean,
    /// PCP predicate — statistical + OOS + external audit (Paper 3, omegav4).
    StatisticalPCP,
    /// External agent challenge audit (Phase 11+).
    ExternalAudit,
}

/// A ∏p verifier. Currently not implemented by `Lean4Oracle`; preserved for
/// GENERALIZATION_ROADMAP Paper 2/3 migration. Runtime verify path for
/// Paper 1 still goes through `Lean4Oracle::verify_omega_detailed`.
///
/// `#[allow(dead_code)]` because this trait is M-1 preservation (forward
/// compatibility) — Paper 1 scope does not instantiate it. Removing the
/// attribute will produce a justified compiler warning once `Lean4Oracle`
/// implements `Predicate` (Phase 2 / Paper 2 work).
#[allow(dead_code)]
pub trait Predicate: Send + Sync {
    fn verify(&self, payload: &str) -> Verdict;
    fn kind(&self) -> PredicateKind;
}
