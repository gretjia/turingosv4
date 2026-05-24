//! TRACE_MATRIX FC1a-judge_pi: Putnam 2025 B3 strict step judge.
//!
//! Problem (Putnam 2025 B3, December 6, 2025):
//!   Suppose S is a nonempty set of positive integers with the property
//!   that if n is in S, then every positive divisor of 2025n − 15n is
//!   also in S. Must S contain all positive integers?
//!
//! Answer: NO. Counterexample: take S to be the smallest set closed
//! under "n ∈ S ⇒ all divisors of 2010n in S" starting from n = 1.
//! Since 2025 − 15 = 2010 = 2 · 3 · 5 · 67, the closure under "divisors
//! of 2010n" preserves the property that every member has all its prime
//! factors in {2, 3, 5, 67}. Therefore primes outside that set (e.g. 7)
//! never enter S, so S need not contain all positive integers.
//!
//! Why this is EXTREME for DeepSeek-chat:
//!   - Putnam 2025 was held December 6, 2025 — AFTER DeepSeek-chat's
//!     training cutoff. No memorized solution available.
//!   - Putnam committee chair stated publicly that ≥6/12 problems were
//!     designed to be LLM-resistant.
//!   - The argument requires (a) simplifying 2025n − 15n, (b) factoring
//!     2010, (c) constructing a closure-under-divisors set, (d)
//!     identifying it doesn't contain all primes. Each is a place where
//!     a chat-tier LLM commonly hand-waves.
//!
//! On-disk §8: handover/directives/2026-05-22_TDMA_BOUNDED_RC1_DIRECTIVE_AND_§8.md

use std::cell::Cell;

use super::math_step_judge::{JudgeVerdict, MathStepJudge};

/// Five stages of the Putnam 2025 B3 canonical proof.
/// TRACE_MATRIX FC1a-judge_pi: Sequential proof state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PutnamB3Stage {
    Stage1Simplify,     // 2025n − 15n = 2010n
    Stage2Factor2010,   // 2010 = 2·3·5·67 (four prime factors)
    Stage3Closure,      // observe divisors of 2010n keep primes in {2,3,5,67}
    Stage4Counterex,    // construct counterexample S (closure starting from {1})
    Stage5ConcludeNo,   // conclude NO, S need not contain all positive integers
}

impl PutnamB3Stage {
    /// TRACE_MATRIX FC1a-judge_pi: Next stage in canonical proof.
    pub fn next(self) -> Option<Self> {
        use PutnamB3Stage::*;
        Some(match self {
            Stage1Simplify => Stage2Factor2010,
            Stage2Factor2010 => Stage3Closure,
            Stage3Closure => Stage4Counterex,
            Stage4Counterex => Stage5ConcludeNo,
            Stage5ConcludeNo => return None,
        })
    }

    /// TRACE_MATRIX FC1a-judge_pi: Human-readable stage label.
    pub fn label(self) -> &'static str {
        use PutnamB3Stage::*;
        match self {
            Stage1Simplify => "Stage1-Simplify-2010n",
            Stage2Factor2010 => "Stage2-Factor-2010",
            Stage3Closure => "Stage3-Closure-Prime-Containment",
            Stage4Counterex => "Stage4-Counterexample-Construction",
            Stage5ConcludeNo => "Stage5-Conclude-NO",
        }
    }
}

/// Reject classes. Six orthogonal failure signatures.
/// TRACE_MATRIX FC1a-judge_pi: Failure classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PutnamB3RejectClass {
    MissingSimplification, // Stage 1: didn't compute 2025−15=2010
    MissingFactorization,  // Stage 2: didn't factor 2010=2·3·5·67
    MissingClosureArgument, // Stage 3: didn't argue prime containment
    MissingCounterexample,  // Stage 4: didn't construct/describe a witness
    WrongFinalAnswer,       // Stage 5: said YES or didn't take a stance
    HandWave,               // Step too short / no rigor
    OffStage,               // Step doesn't match stage at all
    LogicalGap,             // Skipped prior stages
}

impl PutnamB3RejectClass {
    /// TRACE_MATRIX FC1a-judge_pi: Canonical reject_class string for tape.
    pub fn reject_class_str(self) -> &'static str {
        use PutnamB3RejectClass::*;
        match self {
            MissingSimplification => "missing-simplification",
            MissingFactorization => "missing-factorization",
            MissingClosureArgument => "missing-closure-argument",
            MissingCounterexample => "missing-counterexample",
            WrongFinalAnswer => "wrong-final-answer",
            HandWave => "hand-wave",
            OffStage => "off-stage",
            LogicalGap => "logical-gap",
        }
    }
    /// TRACE_MATRIX FC1a-judge_pi: Canonical failed_predicate string for header.
    pub fn failed_predicate_str(self) -> &'static str {
        use PutnamB3RejectClass::*;
        match self {
            MissingSimplification => "arithmetic.simplify",
            MissingFactorization => "primes.factor",
            MissingClosureArgument => "closure.primes",
            MissingCounterexample => "witness.construct",
            WrongFinalAnswer => "answer.value",
            HandWave => "rigor.detail",
            OffStage => "stage.unrecognized",
            LogicalGap => "step.cites_prior",
        }
    }
}

/// Sequential strict judge for Putnam 2025 B3.
/// TRACE_MATRIX FC1a-judge_pi: Strict step-aware verifier.
pub struct PutnamB3Judge {
    pub current_stage: Cell<PutnamB3Stage>,
}

impl Default for PutnamB3Judge {
    fn default() -> Self {
        Self::new()
    }
}

impl PutnamB3Judge {
    /// TRACE_MATRIX FC1a-judge_pi: Constructor at Stage 1.
    pub fn new() -> Self {
        Self {
            current_stage: Cell::new(PutnamB3Stage::Stage1Simplify),
        }
    }

    /// TRACE_MATRIX FC1a-judge_pi: Promote stage after successful step.
    pub fn advance(&self) -> bool {
        match self.current_stage.get().next() {
            Some(n) => {
                self.current_stage.set(n);
                true
            }
            None => false,
        }
    }

    /// TRACE_MATRIX FC1a-judge_pi: Strict step verdict.
    pub fn verdict_for_stage(
        &self,
        candidate: &str,
        stage: PutnamB3Stage,
        prior_accepted: &[String],
    ) -> (JudgeVerdict, Option<PutnamB3RejectClass>) {
        use PutnamB3RejectClass::*;
        use PutnamB3Stage::*;
        let c = candidate.to_lowercase();
        let len_chars = candidate.chars().count();

        // Hand-wave floor
        if len_chars < 100 {
            return (
                JudgeVerdict::Fail {
                    reason: format!(
                        "Stage {} step too short ({} chars) — strict verifier requires explicit reasoning",
                        stage.label(),
                        len_chars
                    ),
                },
                Some(HandWave),
            );
        }

        // Logical-gap check
        let stage_idx = match stage {
            Stage1Simplify => 1,
            Stage2Factor2010 => 2,
            Stage3Closure => 3,
            Stage4Counterex => 4,
            Stage5ConcludeNo => 5,
        };
        if prior_accepted.len() < stage_idx - 1 {
            return (
                JudgeVerdict::Fail {
                    reason: format!(
                        "Stage {} requires {} prior steps; got {}",
                        stage.label(),
                        stage_idx - 1,
                        prior_accepted.len()
                    ),
                },
                Some(LogicalGap),
            );
        }

        match stage {
            Stage1Simplify => {
                // Must compute 2025n - 15n = 2010n explicitly
                let has_2025_minus_15 = c.contains("2025n - 15n")
                    || c.contains("2025n−15n")
                    || c.contains("2025n - 15n")
                    || c.contains("(2025 - 15)")
                    || c.contains("(2025-15)")
                    || c.contains("2025 − 15")
                    || c.contains("2025-15");
                let has_2010n = c.contains("2010n")
                    || c.contains("2010 n")
                    || c.contains("= 2010")
                    || c.contains("equals 2010");
                if !(has_2025_minus_15 && has_2010n) {
                    return (
                        JudgeVerdict::Fail {
                            reason: "Stage 1 must explicitly simplify 2025n − 15n = 2010n (showing both the original expression and the result)".into(),
                        },
                        Some(MissingSimplification),
                    );
                }
            }
            Stage2Factor2010 => {
                // Must factor 2010 = 2·3·5·67. We require all four primes to appear.
                let has_2 = c.contains(" 2 ") || c.contains("2·") || c.contains("2 ·") || c.contains("·2") || c.contains("*2") || c.contains("2 *");
                let has_3 = c.contains(" 3 ") || c.contains("3·") || c.contains("·3") || c.contains("*3") || c.contains("3 *");
                let has_5 = c.contains(" 5 ") || c.contains("5·") || c.contains("·5") || c.contains("*5") || c.contains("5 *");
                let has_67 = c.contains("67") && !c.contains("670") && !c.contains("167") && !c.contains("267");
                let has_factor_mention = c.contains("factor")
                    || c.contains("prime")
                    || c.contains("=")
                    || c.contains("2010 = ")
                    || c.contains("2010=")
                    || c.contains("decomposition");
                if !(has_2 && has_3 && has_5 && has_67 && has_factor_mention) {
                    return (
                        JudgeVerdict::Fail {
                            reason: format!(
                                "Stage 2 must factor 2010 = 2 · 3 · 5 · 67 (need all four primes: has_2={} has_3={} has_5={} has_67={} has_factor_mention={})",
                                has_2, has_3, has_5, has_67, has_factor_mention
                            ),
                        },
                        Some(MissingFactorization),
                    );
                }
            }
            Stage3Closure => {
                // Must observe that divisors of 2010n only have primes from {2,3,5,67} ∪ primes(n)
                let has_divisors = c.contains("divisor") || c.contains("divide");
                let has_primes = c.contains("prime");
                let has_containment = c.contains("only")
                    || c.contains("contain")
                    || c.contains("subset")
                    || c.contains("from")
                    || c.contains("among")
                    || c.contains("introduce no new")
                    || c.contains("no new prime");
                if !(has_divisors && has_primes && has_containment) {
                    return (
                        JudgeVerdict::Fail {
                            reason: "Stage 3 must argue: divisors of 2010n introduce only primes from {2,3,5,67}∪primes(n) — need 'divisor', 'prime', and a containment word ('only', 'contain', etc.)".into(),
                        },
                        Some(MissingClosureArgument),
                    );
                }
            }
            Stage4Counterex => {
                // Must construct a counterexample S that does NOT contain all positive ints
                let has_witness = c.contains("counterexample")
                    || c.contains("counter-example")
                    || c.contains("take s")
                    || c.contains("let s")
                    || c.contains("define s")
                    || c.contains("set s")
                    || c.contains("consider s")
                    || c.contains("s =")
                    || c.contains("example");
                let has_excluded_prime = c.contains("7")
                    || c.contains("11")
                    || c.contains("13")
                    || c.contains("not contain")
                    || c.contains("does not include")
                    || c.contains("excludes")
                    || c.contains("never enter");
                if !(has_witness && has_excluded_prime) {
                    return (
                        JudgeVerdict::Fail {
                            reason: "Stage 4 must (a) construct/name the counterexample S, (b) identify a prime that's NOT in S (e.g. 7, 11, or 13)".into(),
                        },
                        Some(MissingCounterexample),
                    );
                }
            }
            Stage5ConcludeNo => {
                let says_no = c.contains(" no ")
                    || c.contains(" no.")
                    || c.contains(" no,")
                    || c.contains("not necessarily")
                    || c.contains("need not")
                    || c.contains("does not need")
                    || c.contains("doesn't need")
                    || c.contains("therefore no")
                    || c.contains("answer is no")
                    || c.contains("conclude no");
                let mentions_pos_ints = c.contains("positive integer")
                    || c.contains("all positive")
                    || c.contains("all of n");
                if !(says_no && mentions_pos_ints) {
                    return (
                        JudgeVerdict::Fail {
                            reason: "Stage 5 must conclude NO — S need NOT contain all positive integers (use 'no'/'need not'/'not necessarily' + 'positive integer')".into(),
                        },
                        Some(WrongFinalAnswer),
                    );
                }
            }
        }

        (JudgeVerdict::Pass, None)
    }
}

impl MathStepJudge for PutnamB3Judge {
    fn verdict(&self, prior_steps: &[String], candidate_step: &str) -> JudgeVerdict {
        let stage = self.current_stage.get();
        let (v, _c) = self.verdict_for_stage(candidate_step, stage, prior_steps);
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn priors(n: usize) -> Vec<String> {
        (0..n).map(|i| format!("step {} with substantial detail beyond the hand-wave floor so the cross-stage logical-gap test does not accidentally kick in", i)).collect()
    }

    #[test]
    fn b3_judge_accepts_stage1_simplification() {
        let j = PutnamB3Judge::new();
        let s = "We first simplify the expression 2025n - 15n by factoring out n, which gives (2025-15)·n = 2010n. So divisors of 2010n is the operative set.";
        let (v, _) = j.verdict_for_stage(s, PutnamB3Stage::Stage1Simplify, &[]);
        assert!(v.is_pass(), "{:?}", v);
    }

    #[test]
    fn b3_judge_rejects_stage1_without_simplification() {
        let j = PutnamB3Judge::new();
        let s = "We must show that S is forced to grow to contain every positive integer. The set-closure conditions imply rapid expansion.";
        let (v, c) = j.verdict_for_stage(s, PutnamB3Stage::Stage1Simplify, &[]);
        assert!(matches!(v, JudgeVerdict::Fail { .. }));
        assert_eq!(c, Some(PutnamB3RejectClass::MissingSimplification));
    }

    #[test]
    fn b3_judge_accepts_stage2_factor() {
        let j = PutnamB3Judge::new();
        let s = "Now we factor 2010 = 2 · 3 · 5 · 67 into its four prime factors. Note that 67 is itself prime and not a factor of any other small candidate.";
        let (v, _) = j.verdict_for_stage(s, PutnamB3Stage::Stage2Factor2010, &priors(1));
        assert!(v.is_pass(), "{:?}", v);
    }

    #[test]
    fn b3_judge_rejects_stage2_missing_67() {
        let j = PutnamB3Judge::new();
        let s = "We factor 2010 = 2 · 3 · 5 · 134 into manageable parts. Note the small primes 2, 3, 5 are present in this decomposition.";
        let (v, c) = j.verdict_for_stage(s, PutnamB3Stage::Stage2Factor2010, &priors(1));
        assert!(matches!(v, JudgeVerdict::Fail { .. }));
        assert_eq!(c, Some(PutnamB3RejectClass::MissingFactorization));
    }

    #[test]
    fn b3_judge_accepts_stage5_no_conclusion() {
        let j = PutnamB3Judge::new();
        let s = "Therefore the answer is NO — S need not contain all positive integers. The constructed set S provides a witness where 7 is never an element.";
        let (v, _) = j.verdict_for_stage(s, PutnamB3Stage::Stage5ConcludeNo, &priors(4));
        assert!(v.is_pass(), "{:?}", v);
    }

    #[test]
    fn b3_judge_rejects_stage5_yes() {
        let j = PutnamB3Judge::new();
        let s = "After careful analysis, we conclude that S must always grow to contain every positive integer. This is forced by the closure conditions imposed.";
        let (v, c) = j.verdict_for_stage(s, PutnamB3Stage::Stage5ConcludeNo, &priors(4));
        assert!(matches!(v, JudgeVerdict::Fail { .. }));
        assert_eq!(c, Some(PutnamB3RejectClass::WrongFinalAnswer));
    }
}
