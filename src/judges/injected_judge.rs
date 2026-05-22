//! TRACE_MATRIX FC1a-predicate_pi: Scripted-verdict judge for stress testing.
//!
//! `InjectedJudge` consumes a pre-specified `Vec<JudgeVerdict>` and returns
//! the next verdict each time `verdict()` is called. After the list is
//! exhausted, it falls back to a default verdict (configurable). This makes
//! kernel stress testing fully deterministic — the test author controls
//! exactly when the kernel sees a failure vs. success.
//!
//! Use cases: Atom 9 distiller compression stress test scenarios (forced
//! distinct-signature retries, forced same-signature retries to trigger
//! zero_gain, long failure chains).
//!
//! On-disk §8: handover/directives/2026-05-22_TDMA_BOUNDED_RC1_DIRECTIVE_AND_§8.md

use std::cell::Cell;

use super::math_step_judge::{JudgeVerdict, MathStepJudge};

/// Scripted-verdict judge.
/// TRACE_MATRIX FC1a-predicate_pi: Deterministic backend for stress tests.
pub struct InjectedJudge {
    verdicts: Vec<JudgeVerdict>,
    cursor: Cell<usize>,
    fallback: JudgeVerdict,
}

impl InjectedJudge {
    /// TRACE_MATRIX FC1a-predicate_pi: Build from a verdict script + fallback.
    pub fn new(verdicts: Vec<JudgeVerdict>, fallback: JudgeVerdict) -> Self {
        Self {
            verdicts,
            cursor: Cell::new(0),
            fallback,
        }
    }

    /// TRACE_MATRIX FC1a-predicate_pi: How many verdicts have been served.
    pub fn served(&self) -> usize {
        self.cursor.get()
    }
}

impl MathStepJudge for InjectedJudge {
    fn verdict(&self, _prior_steps: &[String], _candidate_step: &str) -> JudgeVerdict {
        let i = self.cursor.get();
        self.cursor.set(i + 1);
        self.verdicts.get(i).cloned().unwrap_or_else(|| self.fallback.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn injected_judge_serves_in_order() {
        let j = InjectedJudge::new(
            vec![
                JudgeVerdict::Fail { reason: "v1".into() },
                JudgeVerdict::Fail { reason: "v2".into() },
                JudgeVerdict::Pass,
            ],
            JudgeVerdict::Pass,
        );
        assert!(matches!(j.verdict(&[], ""), JudgeVerdict::Fail { reason } if reason == "v1"));
        assert!(matches!(j.verdict(&[], ""), JudgeVerdict::Fail { reason } if reason == "v2"));
        assert!(j.verdict(&[], "").is_pass());
        assert_eq!(j.served(), 3);
    }

    #[test]
    fn injected_judge_falls_back_after_exhaustion() {
        let j = InjectedJudge::new(
            vec![JudgeVerdict::Fail { reason: "only".into() }],
            JudgeVerdict::Pass,
        );
        let _ = j.verdict(&[], "");
        // Past the end → fallback
        assert!(j.verdict(&[], "").is_pass());
        assert!(j.verdict(&[], "").is_pass());
    }
}
