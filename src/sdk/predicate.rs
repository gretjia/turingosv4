//! Predicate trait — the ∏p top-management of Art. IV mermaid.
//!
//! Constitutional basis:
//! - Art. I.1   boolean predicate (Paper 1 MiniF2F, Paper 2 zeta_sum_proof)
//! - Art. I.1.1 PCP predicate     (Paper 3 omegav4, statistical + OOS + audit)
//! - Art. IV    `∏ p(output | Q_t)` — predicates evaluated as a product.
//!   When ∏p = 1 → wtool writes; when ∏p = 0 → Q_{t+1} = Q_t.
//!
//! Phase Z (2026-04-22): Predicate is now wired into the bus as a live
//! evaluation chain — `TuringBus::evaluate_predicates(ctx, payload)` runs
//! the full registered list as a conjunction (AND). Predicates can be
//! domain-specific (ForbiddenPattern for all payloads, WalletBalance only
//! for invest) via `applies_to(ctx)`.

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

impl Verdict {
    /// True iff verdict is Complete or PartialOk (i.e., not rejecting).
    pub fn is_pass(&self) -> bool {
        !matches!(self, Verdict::Reject(_))
    }
    /// Product operator on two verdicts (AND-semantics):
    /// - Any Reject → Reject (short-circuit with first reason)
    /// - Both Complete → Complete
    /// - PartialOk × anything-non-Reject → PartialOk with min confidence
    pub fn product(&self, other: &Verdict) -> Verdict {
        match (self, other) {
            (Verdict::Reject(r), _) | (_, Verdict::Reject(r)) => Verdict::Reject(r.clone()),
            (Verdict::PartialOk { confidence: a }, Verdict::PartialOk { confidence: b }) => {
                Verdict::PartialOk { confidence: a.min(*b) }
            }
            (Verdict::PartialOk { confidence: c }, Verdict::Complete)
            | (Verdict::Complete, Verdict::PartialOk { confidence: c }) => {
                Verdict::PartialOk { confidence: *c }
            }
            (Verdict::Complete, Verdict::Complete) => Verdict::Complete,
        }
    }
}

/// Which predicate domain emitted this verdict.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PredicateKind {
    /// Boolean Lean4 oracle (Paper 1 & 2).
    Lean4Boolean = 0,
    /// PCP predicate — statistical + OOS + external audit (Paper 3, omegav4).
    StatisticalPCP = 1,
    /// External agent challenge audit (Phase 11+).
    ExternalAudit = 2,
    /// Pre-Lean forbidden-pattern check (C-011, F-2026-04-20-05).
    ForbiddenPattern = 3,
    /// Wallet solvency / Law 2 balance check on invest.
    WalletBalance = 4,
    /// Payload size / line-count bounds (V3L-21 / BusConfig).
    PayloadSize = 5,
}

/// Evaluation context passed to `Predicate::applies_to` so predicates can
/// opt-in or opt-out based on the tool / author / tape depth. This is the
/// `q_i` projection of Q_t used to decide *which* predicates form ∏p.
#[derive(Debug, Clone)]
pub struct PredicateContext<'a> {
    pub tool: &'a str,
    pub author: &'a str,
    pub tape_depth: usize,
}

/// A ∏p verifier. Phase Z: this is now the canonical interface for all
/// bus-level checks. Existing checks (forbidden patterns, sorry, Lean
/// oracle) are migrated to default-registered `Predicate` impls.
pub trait Predicate: Send + Sync {
    /// Human-readable name, used in rejection reasons + telemetry.
    fn name(&self) -> &str;
    /// Predicate family (for aggregation / reporting).
    fn kind(&self) -> PredicateKind;
    /// Whether this predicate applies to a given (tool, author, depth) context.
    /// Default = always applies (covers the generic case). Override for
    /// tool-specific predicates (e.g., WalletBalance only on invest).
    fn applies_to(&self, _ctx: &PredicateContext) -> bool { true }
    /// Evaluate the payload, return the verdict.
    fn verify(&self, payload: &str) -> Verdict;
}

// ── Default predicate implementations (migrated from scattered checks) ──

/// Rejects any payload containing a forbidden pattern (Art. I.1, C-011, F-2026-04-20-05).
/// Forbidden list comes from `BusConfig::forbidden_patterns`; typical entries:
/// `native_decide`, `sorryAx`, bare `decide`, `Classical.choice`, IO bypasses.
pub struct ForbiddenPatternPredicate {
    pub patterns: Vec<String>,
}

impl Predicate for ForbiddenPatternPredicate {
    fn name(&self) -> &str { "forbidden_pattern" }
    fn kind(&self) -> PredicateKind { PredicateKind::ForbiddenPattern }
    fn verify(&self, payload: &str) -> Verdict {
        for pat in &self.patterns {
            if payload.contains(pat) {
                return Verdict::Reject(format!("Forbidden pattern: '{}'", pat));
            }
        }
        Verdict::Complete
    }
}

/// Rejects sorry / sorryAx (Lean bypass).
pub struct SorryPredicate;

impl Predicate for SorryPredicate {
    fn name(&self) -> &str { "sorry_guard" }
    fn kind(&self) -> PredicateKind { PredicateKind::ForbiddenPattern }
    fn verify(&self, payload: &str) -> Verdict {
        if payload.contains("sorry") || payload.contains("sorryAx") {
            Verdict::Reject("sorry/sorryAx in proof".into())
        } else {
            Verdict::Complete
        }
    }
}

/// Enforces BusConfig::max_payload_chars / max_payload_lines (V3L-21).
pub struct PayloadSizePredicate {
    pub max_chars: usize,
    pub max_lines: usize,
}

impl Predicate for PayloadSizePredicate {
    fn name(&self) -> &str { "payload_size" }
    fn kind(&self) -> PredicateKind { PredicateKind::PayloadSize }
    fn verify(&self, payload: &str) -> Verdict {
        if payload.chars().count() > self.max_chars {
            return Verdict::Reject(format!(
                "payload exceeds {} chars", self.max_chars));
        }
        if payload.lines().count() > self.max_lines {
            return Verdict::Reject(format!(
                "payload exceeds {} lines", self.max_lines));
        }
        Verdict::Complete
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn product_reject_short_circuits() {
        let a = Verdict::Complete;
        let b = Verdict::Reject("x".into());
        match a.product(&b) {
            Verdict::Reject(r) => assert_eq!(r, "x"),
            v => panic!("expected reject, got {:?}", v),
        }
    }

    #[test]
    fn product_two_complete_is_complete() {
        assert!(matches!(
            Verdict::Complete.product(&Verdict::Complete),
            Verdict::Complete
        ));
    }

    #[test]
    fn product_partial_takes_min_confidence() {
        let a = Verdict::PartialOk { confidence: 0.8 };
        let b = Verdict::PartialOk { confidence: 0.3 };
        match a.product(&b) {
            Verdict::PartialOk { confidence } => assert!((confidence - 0.3).abs() < 1e-9),
            _ => panic!("expected partial"),
        }
    }

    #[test]
    fn forbidden_pattern_rejects_listed() {
        let p = ForbiddenPatternPredicate { patterns: vec!["native_decide".into()] };
        match p.verify("apply native_decide") {
            Verdict::Reject(r) => assert!(r.contains("native_decide")),
            _ => panic!(),
        }
    }

    #[test]
    fn forbidden_pattern_passes_clean() {
        let p = ForbiddenPatternPredicate { patterns: vec!["native_decide".into()] };
        assert!(matches!(p.verify("linarith"), Verdict::Complete));
    }

    #[test]
    fn sorry_predicate_rejects_sorry() {
        let p = SorryPredicate;
        assert!(matches!(p.verify("exact sorry"), Verdict::Reject(_)));
    }

    #[test]
    fn payload_size_rejects_over_chars() {
        let p = PayloadSizePredicate { max_chars: 10, max_lines: 100 };
        assert!(matches!(p.verify("this is way longer than ten"), Verdict::Reject(_)));
    }

    #[test]
    fn payload_size_passes_under_limit() {
        let p = PayloadSizePredicate { max_chars: 100, max_lines: 10 };
        assert!(matches!(p.verify("ok"), Verdict::Complete));
    }
}
