//! TRACE_MATRIX FC1a-judge_pi: Math-step JudgeAI for the
//! Atom 7.5 real-evidence run.
//!
//! Two backends behind the same `MathStepJudge` API:
//!
//! 1. **`OfflineHeuristic`** — deterministic verdict logic that checks for a
//!    canonical sequence of derivation phrases consistent with the
//!    Tao/Ramanujan regularized-sum proof of `Σ n = -1/12` via the smoothing
//!    `m·exp(-m/N)·cos(m/N)`. Used in CI + integration tests; no network.
//!
//! 2. **`LlmClient`** — a slot for the production LLM judge. Atom 7.5 ships
//!    the trait + offline default; the LLM-backed variant lands when the
//!    user runs the manual real-LLM evidence step (Atom 7.5 acceptance
//!    criterion 5 in the orchestrator plan §10).
//!
//! Discipline: this judge ALWAYS returns one of three verdicts. The kernel
//! treats `Pass` as `env_result.success = true` + `header.status = Proceed`
//! and `Fail` / `NeedsClarification` as `success = false`.
//!
//! On-disk §8: handover/directives/2026-05-22_TDMA_BOUNDED_RC1_DIRECTIVE_AND_§8.md

use serde::{Deserialize, Serialize};

/// Judgement of a single proof step.
/// TRACE_MATRIX FC1a-judge_pi: Ternary verdict from JudgeAI.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum JudgeVerdict {
    Pass,
    Fail { reason: String },
    NeedsClarification { question: String },
}

impl JudgeVerdict {
    /// TRACE_MATRIX FC1a-judge_pi: Coerce to a kernel-friendly success bit.
    pub fn is_pass(&self) -> bool {
        matches!(self, JudgeVerdict::Pass)
    }
}

/// Contract for a math-step judge.
/// TRACE_MATRIX FC1a-judge_pi: Pluggable JudgeAI backend.
pub trait MathStepJudge {
    /// Judge a single candidate step in the context of prior accepted steps.
    fn verdict(&self, prior_steps: &[String], candidate_step: &str) -> JudgeVerdict;
}

// ── Offline heuristic backend ──────────────────────────────────

/// Deterministic JudgeAI that recognizes the canonical Ramanujan/Tao
/// regularized-sum derivation steps. Used when no LLM is wired up.
///
/// Recognition rules (loose; intended as a structural check, NOT a math proof
/// verifier):
///   * Step 1 must mention the regularizer `m·exp(-m/N)` (or `m * exp(-m/N)`)
///     AND `cos(m/N)`.
///   * Step 2+ must reference the partial sum `S(N)` OR the regularizer's
///     asymptotic.
///   * Final step starts with `[COMPLETE]` AND mentions `-1/12`.
///
/// TRACE_MATRIX FC1a-judge_pi: Offline judge for CI + integration tests.
#[derive(Debug, Default, Clone)]
pub struct OfflineHeuristicJudge;

impl OfflineHeuristicJudge {
    /// TRACE_MATRIX FC1a-judge_pi: Constructor (default offline judge).
    pub fn new() -> Self {
        Self
    }
}

impl MathStepJudge for OfflineHeuristicJudge {
    fn verdict(&self, prior_steps: &[String], candidate_step: &str) -> JudgeVerdict {
        let step = candidate_step.trim();
        if step.is_empty() {
            return JudgeVerdict::Fail {
                reason: "empty step".into(),
            };
        }

        // Terminal step
        if step.starts_with("[COMPLETE]") {
            if step.contains("-1/12") {
                return JudgeVerdict::Pass;
            }
            return JudgeVerdict::Fail {
                reason: "[COMPLETE] step must derive -1/12".into(),
            };
        }

        // Step 1 expectations
        if prior_steps.is_empty() {
            let has_regularizer =
                (step.contains("m·exp(-m/N)") || step.contains("m * exp(-m/N)") ||
                 step.contains("m*exp(-m/N)"))
                    && step.contains("cos(m/N)");
            if has_regularizer {
                return JudgeVerdict::Pass;
            }
            return JudgeVerdict::Fail {
                reason: "step 1 must introduce the regularizer m·exp(-m/N)·cos(m/N)".into(),
            };
        }

        // Subsequent steps: must build on prior. We accept any of:
        //   - mentions the partial sum S(N)
        //   - mentions the Abel/Cesàro/Euler-Maclaurin / asymptotic expansion
        //   - mentions integration by parts / smoothing
        //   - mentions zeta-regularization or analytic continuation
        let lower = step.to_lowercase();
        let signals = [
            "s(n)",
            "abel",
            "cesaro",
            "cesàro",
            "euler-maclaurin",
            "asymptotic",
            "integration by parts",
            "smoothing",
            "zeta",
            "analytic continuation",
            "regulariz",
            "expansion",
            "differentiate",
        ];
        if signals.iter().any(|sig| lower.contains(sig)) {
            return JudgeVerdict::Pass;
        }

        JudgeVerdict::NeedsClarification {
            question: "Which technique are you applying? Mention S(N), Abel/Cesàro, \
                       Euler-Maclaurin, asymptotic expansion, or analytic continuation."
                .into(),
        }
    }
}

// ── Tests ──────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offline_judge_accepts_canonical_step_1() {
        let j = OfflineHeuristicJudge::new();
        let v = j.verdict(
            &[],
            "Define S(N) = sum over m >= 1 of m·exp(-m/N)·cos(m/N).",
        );
        assert!(v.is_pass(), "got {:?}", v);
    }

    #[test]
    fn offline_judge_rejects_step_1_without_regularizer() {
        let j = OfflineHeuristicJudge::new();
        let v = j.verdict(&[], "Just write -1/12 directly.");
        assert!(matches!(v, JudgeVerdict::Fail { .. }));
    }

    #[test]
    fn offline_judge_accepts_subsequent_step_with_signal_word() {
        let j = OfflineHeuristicJudge::new();
        let prior = vec!["Step 1 defined regularizer.".to_string()];
        let v = j.verdict(
            &prior,
            "Expand S(N) using Euler-Maclaurin asymptotic expansion.",
        );
        assert!(v.is_pass());
    }

    #[test]
    fn offline_judge_clarifies_subsequent_step_without_signal() {
        let j = OfflineHeuristicJudge::new();
        let prior = vec!["Step 1".to_string()];
        let v = j.verdict(&prior, "Then magic happens.");
        assert!(matches!(v, JudgeVerdict::NeedsClarification { .. }));
    }

    #[test]
    fn offline_judge_accepts_complete_with_minus_1_over_12() {
        let j = OfflineHeuristicJudge::new();
        let v = j.verdict(
            &["s1".into(), "s2".into()],
            "[COMPLETE] Therefore the regularized sum equals -1/12.",
        );
        assert!(v.is_pass());
    }

    #[test]
    fn offline_judge_rejects_complete_without_minus_1_over_12() {
        let j = OfflineHeuristicJudge::new();
        let v = j.verdict(
            &["s1".into()],
            "[COMPLETE] The sum equals 0.",
        );
        assert!(matches!(v, JudgeVerdict::Fail { .. }));
    }
}
