//! TRACE_MATRIX FC1a-predicate_pi: Nesbitt's-inequality step judge for Atom 10
//! real-world stress test.
//!
//! Implements an IneqMath-style multi-category judge for the 8-step canonical
//! proof of Nesbitt's inequality:
//!
//!     For a, b, c > 0:  a/(b+c) + b/(a+c) + c/(a+b) ≥ 3/2
//!
//! Five failure categories mirror the IneqMath paper's step-level judges
//! (arxiv.org/abs/2506.07927):
//!
//!   1. DirectionReversal       — applied AM-GM with ≤ instead of ≥ (most
//!                                 common LLM mistake; ~30% of failures in
//!                                 IneqMath data)
//!   2. BadSubstitution         — set x=a+b inconsistently with later use
//!   3. AlgebraError            — wrong expansion or cancellation
//!   4. LogicalGap              — step doesn't follow from prior accepted
//!                                 steps (cited or implied)
//!   5. MissingEqualityCase     — concluded ≥3/2 without showing equality
//!                                 at a=b=c (only triggers at final step)
//!
//! Plus a sixth implicit category for the proof skeleton:
//!
//!   6. OffStage                — step text doesn't match ANY recognizable
//!                                 stage of the canonical proof
//!
//! On-disk §8: handover/directives/2026-05-22_TDMA_BOUNDED_RC1_DIRECTIVE_AND_§8.md

use super::math_step_judge::{JudgeVerdict, MathStepJudge};

/// Eight canonical stages of the Nesbitt AM-GM proof.
/// TRACE_MATRIX FC1a-predicate_pi: Tracks where the worker is in the proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NesbittStage {
    Step1Substitute,    // x = b+c, y = a+c, z = a+b   (or analog)
    Step2Rewrite,       // each a/(b+c) → (combination of x,y,z)
    Step3Expand,        // expand into 6 ratio terms
    Step4Group,         // group into (x/y + y/x), (y/z + z/y), (x/z + z/x)
    Step5ApplyAmGm,     // each pair x/y + y/x ≥ 2  (3 pairs)
    Step6Sum,           // total of 6 pair-terms ≥ 6
    Step7Subtract,      // subtract 3 to recover original LHS form
    Step8ConcludeAndEq, // LHS ≥ 3/2; equality at x=y=z ⟺ a=b=c
}

impl NesbittStage {
    /// TRACE_MATRIX FC1a-predicate_pi: Next stage in the canonical proof.
    pub fn next(self) -> Option<Self> {
        Some(match self {
            NesbittStage::Step1Substitute => NesbittStage::Step2Rewrite,
            NesbittStage::Step2Rewrite => NesbittStage::Step3Expand,
            NesbittStage::Step3Expand => NesbittStage::Step4Group,
            NesbittStage::Step4Group => NesbittStage::Step5ApplyAmGm,
            NesbittStage::Step5ApplyAmGm => NesbittStage::Step6Sum,
            NesbittStage::Step6Sum => NesbittStage::Step7Subtract,
            NesbittStage::Step7Subtract => NesbittStage::Step8ConcludeAndEq,
            NesbittStage::Step8ConcludeAndEq => return None,
        })
    }

    /// Human-readable label.
    /// TRACE_MATRIX FC1a-predicate_pi: For evidence reporting.
    pub fn label(self) -> &'static str {
        match self {
            NesbittStage::Step1Substitute => "Step1-Substitute",
            NesbittStage::Step2Rewrite => "Step2-Rewrite",
            NesbittStage::Step3Expand => "Step3-Expand",
            NesbittStage::Step4Group => "Step4-Group",
            NesbittStage::Step5ApplyAmGm => "Step5-ApplyAMGM",
            NesbittStage::Step6Sum => "Step6-Sum",
            NesbittStage::Step7Subtract => "Step7-Subtract",
            NesbittStage::Step8ConcludeAndEq => "Step8-Conclude+Eq",
        }
    }
}

/// Reject classes — mirror IneqMath's five-judge taxonomy + an off-stage class.
/// TRACE_MATRIX FC1a-predicate_pi: Five orthogonal failure signatures.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NesbittRejectClass {
    DirectionReversal,
    BadSubstitution,
    AlgebraError,
    LogicalGap,
    MissingEqualityCase,
    OffStage,
}

impl NesbittRejectClass {
    /// TRACE_MATRIX FC1a-predicate_pi: Canonical reject_class string for
    /// the tape's `RetryConstraint.id` and TapeNode.reject_class field.
    pub fn reject_class_str(self) -> &'static str {
        match self {
            NesbittRejectClass::DirectionReversal => "direction-reversal",
            NesbittRejectClass::BadSubstitution => "bad-substitution",
            NesbittRejectClass::AlgebraError => "algebra-error",
            NesbittRejectClass::LogicalGap => "logical-gap",
            NesbittRejectClass::MissingEqualityCase => "missing-equality-case",
            NesbittRejectClass::OffStage => "off-stage",
        }
    }

    /// TRACE_MATRIX FC1a-predicate_pi: Canonical failed_predicate string for
    /// the StateUpdate header's `failed_predicate` field.
    pub fn failed_predicate_str(self) -> &'static str {
        match self {
            NesbittRejectClass::DirectionReversal => "amgm.direction",
            NesbittRejectClass::BadSubstitution => "vars.consistency",
            NesbittRejectClass::AlgebraError => "simplify.rule",
            NesbittRejectClass::LogicalGap => "step.cites_prior",
            NesbittRejectClass::MissingEqualityCase => "equality.case",
            NesbittRejectClass::OffStage => "stage.unrecognized",
        }
    }
}

/// Judge for one step. Internal state advances through `NesbittStage` as the
/// worker accumulates accepted steps.
/// TRACE_MATRIX FC1a-predicate_pi: Multi-category sequential judge.
pub struct NesbittStepJudge {
    /// Current expected stage. After `verdict()` returns Pass, the caller
    /// should call `advance()` to move to the next stage. (We keep advance
    /// explicit so the kernel's success-detection path is the source of
    /// truth, not the judge's internal counter.)
    pub current_stage: std::cell::Cell<NesbittStage>,
}

impl Default for NesbittStepJudge {
    fn default() -> Self {
        Self::new()
    }
}

impl NesbittStepJudge {
    /// TRACE_MATRIX FC1a-predicate_pi: Construct judge starting at Step 1.
    pub fn new() -> Self {
        Self {
            current_stage: std::cell::Cell::new(NesbittStage::Step1Substitute),
        }
    }

    /// TRACE_MATRIX FC1a-predicate_pi: Promote stage after a successful step.
    pub fn advance(&self) -> bool {
        match self.current_stage.get().next() {
            Some(next) => {
                self.current_stage.set(next);
                true
            }
            None => false,
        }
    }

    /// TRACE_MATRIX FC1a-predicate_pi: Convert verdict into a `JudgeVerdict`
    /// for the kernel's success bit.
    pub fn verdict_for_stage(
        &self,
        candidate: &str,
        stage: NesbittStage,
        prior_accepted: &[String],
    ) -> (JudgeVerdict, Option<NesbittRejectClass>) {
        let c = candidate.to_lowercase();

        // 1. DirectionReversal: any step that says "≤" / "<=" / "less than"
        //    when applying AM-GM is reversed.
        let mentions_amgm = c.contains("am-gm")
            || c.contains("amgm")
            || c.contains("arithmetic mean")
            || c.contains("geometric mean");
        if mentions_amgm && (c.contains("<=") || c.contains("≤") || c.contains("less than"))
        {
            return (
                JudgeVerdict::Fail {
                    reason: format!(
                        "Stage {}: AM-GM applied in wrong direction (≤ instead of ≥)",
                        stage.label()
                    ),
                },
                Some(NesbittRejectClass::DirectionReversal),
            );
        }

        // 2. MissingEqualityCase: final step needs to say equality at a=b=c
        if stage == NesbittStage::Step8ConcludeAndEq {
            let mentions_eq = c.contains("equality")
                || c.contains("a=b=c")
                || c.contains("a = b = c");
            if !mentions_eq {
                return (
                    JudgeVerdict::Fail {
                        reason: "Final step omits the equality condition (a=b=c)".into(),
                    },
                    Some(NesbittRejectClass::MissingEqualityCase),
                );
            }
            // Must also conclude 3/2
            if !c.contains("3/2") && !c.contains("1.5") && !c.contains("three halves") {
                return (
                    JudgeVerdict::Fail {
                        reason: "Final step does not conclude the bound 3/2".into(),
                    },
                    Some(NesbittRejectClass::AlgebraError),
                );
            }
        }

        // 3. BadSubstitution: stages 2-4 reference substitution; if step
        //    text uses inconsistent variable bindings (e.g., introduces
        //    x = a+b after declaring x = b+c), flag it.
        if matches!(
            stage,
            NesbittStage::Step1Substitute
                | NesbittStage::Step2Rewrite
                | NesbittStage::Step3Expand
        ) {
            let bindings = [
                ("x = a+b", "x = b+c"),
                ("x = b+c", "x = a+b"),
                ("y = a+c", "y = b+c"),
                ("y = b+c", "y = a+c"),
            ];
            for (b1, b2) in bindings {
                if c.contains(b1) && c.contains(b2) {
                    return (
                        JudgeVerdict::Fail {
                            reason: format!(
                                "Stage {}: inconsistent variable binding ({} vs {})",
                                stage.label(),
                                b1,
                                b2
                            ),
                        },
                        Some(NesbittRejectClass::BadSubstitution),
                    );
                }
            }
        }

        // 4a. AlgebraError at Step 2: canonical form has (x+z-y)/(2y).
        //     Wrong-sign forms ((x+z+y), (x-z-y), etc.) are algebra errors.
        if stage == NesbittStage::Step2Rewrite {
            // Look for sign patterns in numerator. Canonical: x+z-y, y+z-x, x+y-z.
            // Wrongs: x+z+y, x-z-y, x+y+z, etc.
            let has_wrong_sign = c.contains("(x+z+y)")
                || c.contains("(x-z-y)")
                || c.contains("(x+y+z)")
                || c.contains("x + z + y")
                || c.contains("x - z - y");
            let has_canonical = c.contains("(x+z-y)")
                || c.contains("(y+z-x)")
                || c.contains("(x+y-z)")
                || c.contains("x+z-y");
            if has_wrong_sign && !has_canonical {
                return (
                    JudgeVerdict::Fail {
                        reason: format!(
                            "Stage {}: numerator sign error — canonical form is (x+z-y)/(2y), not (x+z+y)/(2y)",
                            stage.label()
                        ),
                    },
                    Some(NesbittRejectClass::AlgebraError),
                );
            }
        }

        // 4b. AlgebraError at Step 3: canonical splits into SIX fractions.
        //     Wrong forms: "single fraction", "one big fraction", "denominator xyz".
        if stage == NesbittStage::Step3Expand {
            let has_wrong_combination = c.contains("single fraction")
                || c.contains("one big fraction")
                || c.contains("denominator xyz")
                || c.contains("combined fraction");
            let has_canonical = c.contains("six")
                || c.contains("6 fractions")
                || c.contains("split")
                || c.contains("(x+z)/y");
            if has_wrong_combination && !has_canonical {
                return (
                    JudgeVerdict::Fail {
                        reason: format!(
                            "Stage {}: should expand into SIX separate fractions, not a single combined fraction",
                            stage.label()
                        ),
                    },
                    Some(NesbittRejectClass::AlgebraError),
                );
            }
        }

        // 4c. AlgebraError at Step 6: "≥ 6/2" instead of "≥ 6", or computing
        //     six pair sum as 5 / 8 / etc.
        if stage == NesbittStage::Step6Sum {
            // The correct claim at Step 6 is "sum of 6 pair-terms ≥ 6"
            let has_six_bound =
                c.contains("≥ 6") || c.contains(">= 6") || c.contains("at least 6");
            let has_wrong_bound = c.contains("≥ 4")
                || c.contains(">= 4")
                || c.contains("≥ 5")
                || c.contains(">= 5")
                || c.contains("≥ 8")
                || c.contains(">= 8");
            if !has_six_bound && has_wrong_bound {
                return (
                    JudgeVerdict::Fail {
                        reason: "Step 6 wrong bound: 3 pairs each ≥ 2 means sum ≥ 6, not 4/5/8"
                            .into(),
                    },
                    Some(NesbittRejectClass::AlgebraError),
                );
            }
        }

        // 5. LogicalGap: skipping forward (e.g., trying to conclude 3/2 at
        //    Step 3 before Steps 4-7 are established)
        if matches!(stage, NesbittStage::Step1Substitute | NesbittStage::Step2Rewrite)
            && (c.contains("3/2") || c.contains("≥ 3/2"))
        {
            return (
                JudgeVerdict::Fail {
                    reason: format!(
                        "Stage {}: skipping ahead to the 3/2 bound without intermediate steps",
                        stage.label()
                    ),
                },
                Some(NesbittRejectClass::LogicalGap),
            );
        }

        // 6. OffStage: step doesn't match any expected keyword for this stage
        let keywords: &[&str] = match stage {
            NesbittStage::Step1Substitute => &["substitute", "let x", "set x", "x = b+c", "x=b+c"],
            NesbittStage::Step2Rewrite => &["rewrite", "(x+z-y)", "in terms of x, y, z", "/y", "/x"],
            NesbittStage::Step3Expand => &["expand", "six", "6 fractions", "separate fractions", "split"],
            NesbittStage::Step4Group => &["group", "pair", "(x/y + y/x)", "x/y + y/x"],
            NesbittStage::Step5ApplyAmGm => &["am-gm", "amgm", "≥ 2", ">= 2", "geometric mean"],
            NesbittStage::Step6Sum => &["sum", "≥ 6", ">= 6", "total", "six pair"],
            NesbittStage::Step7Subtract => &["subtract", "minus 3", "- 3", "-3"],
            NesbittStage::Step8ConcludeAndEq => &["3/2", "1.5", "three halves", "conclude"],
        };
        let has_keyword = keywords.iter().any(|kw| c.contains(kw));
        if !has_keyword {
            return (
                JudgeVerdict::Fail {
                    reason: format!(
                        "Stage {}: step text doesn't recognize any expected keyword. Hint: {:?}",
                        stage.label(),
                        keywords
                    ),
                },
                Some(NesbittRejectClass::OffStage),
            );
        }

        // 7. LogicalGap secondary check: if at Stage N>1, the step should
        //    reference a result from a prior stage (or the kernel should
        //    see prior_accepted populated). For OFFLINE simulation we
        //    accept if prior_accepted.len() >= stage_index - 1.
        let stage_idx = match stage {
            NesbittStage::Step1Substitute => 1,
            NesbittStage::Step2Rewrite => 2,
            NesbittStage::Step3Expand => 3,
            NesbittStage::Step4Group => 4,
            NesbittStage::Step5ApplyAmGm => 5,
            NesbittStage::Step6Sum => 6,
            NesbittStage::Step7Subtract => 7,
            NesbittStage::Step8ConcludeAndEq => 8,
        };
        if prior_accepted.len() < stage_idx - 1 {
            return (
                JudgeVerdict::Fail {
                    reason: format!(
                        "Stage {}: only {} prior steps accepted, expected ≥ {}",
                        stage.label(),
                        prior_accepted.len(),
                        stage_idx - 1
                    ),
                },
                Some(NesbittRejectClass::LogicalGap),
            );
        }

        (JudgeVerdict::Pass, None)
    }
}

impl MathStepJudge for NesbittStepJudge {
    fn verdict(&self, prior_steps: &[String], candidate_step: &str) -> JudgeVerdict {
        let stage = self.current_stage.get();
        let (v, _class) = self.verdict_for_stage(candidate_step, stage, prior_steps);
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn canonical_step(stage: NesbittStage) -> &'static str {
        match stage {
            NesbittStage::Step1Substitute => "Let x = b+c, y = a+c, z = a+b. Substitute.",
            NesbittStage::Step2Rewrite => {
                "Rewrite a/(b+c) = (x+z-y)/(2y) in terms of x, y, z."
            }
            NesbittStage::Step3Expand => {
                "Expand into six separate fractions: (x+z)/y + (y+z)/x + (x+y)/z - 3"
            }
            NesbittStage::Step4Group => {
                "Group into three pairs: (x/y + y/x) + (y/z + z/y) + (x/z + z/x)"
            }
            NesbittStage::Step5ApplyAmGm => {
                "By AM-GM, each pair x/y + y/x ≥ 2 since arithmetic mean ≥ geometric mean = 1"
            }
            NesbittStage::Step6Sum => "Sum the three pairs: total ≥ 6",
            NesbittStage::Step7Subtract => "Subtract 3 from both sides: LHS ≥ 6 - 3 = 3",
            NesbittStage::Step8ConcludeAndEq => {
                "Divide by 2: a/(b+c) + b/(a+c) + c/(a+b) ≥ 3/2. Equality iff x=y=z ⟺ a=b=c."
            }
        }
    }

    #[test]
    fn judge_accepts_canonical_step_1() {
        let j = NesbittStepJudge::new();
        let (v, _) = j.verdict_for_stage(
            canonical_step(NesbittStage::Step1Substitute),
            NesbittStage::Step1Substitute,
            &[],
        );
        assert!(v.is_pass(), "got {:?}", v);
    }

    #[test]
    fn judge_rejects_direction_reversal() {
        let j = NesbittStepJudge::new();
        let bad = "By AM-GM, each pair x/y + y/x ≤ 2 (mean ≤ geometric)";
        let (v, c) = j.verdict_for_stage(bad, NesbittStage::Step5ApplyAmGm, &["s1".into(), "s2".into(), "s3".into(), "s4".into()]);
        assert!(matches!(v, JudgeVerdict::Fail { .. }));
        assert_eq!(c, Some(NesbittRejectClass::DirectionReversal));
    }

    #[test]
    fn judge_rejects_bad_substitution() {
        let j = NesbittStepJudge::new();
        let bad = "Let x = b+c and also x = a+b. Substitute.";
        let (v, c) = j.verdict_for_stage(bad, NesbittStage::Step1Substitute, &[]);
        assert!(matches!(v, JudgeVerdict::Fail { .. }));
        assert_eq!(c, Some(NesbittRejectClass::BadSubstitution));
    }

    #[test]
    fn judge_rejects_logical_gap_skipping_ahead() {
        let j = NesbittStepJudge::new();
        let bad = "Substitute and conclude ≥ 3/2 directly";
        let (v, c) = j.verdict_for_stage(bad, NesbittStage::Step1Substitute, &[]);
        assert!(matches!(v, JudgeVerdict::Fail { .. }));
        assert_eq!(c, Some(NesbittRejectClass::LogicalGap));
    }

    #[test]
    fn judge_rejects_algebra_error_on_step_6() {
        let j = NesbittStepJudge::new();
        let bad = "Sum the three pairs: total ≥ 4";
        let prior: Vec<String> = (0..5).map(|i| format!("s{}", i)).collect();
        let (v, c) = j.verdict_for_stage(bad, NesbittStage::Step6Sum, &prior);
        assert!(matches!(v, JudgeVerdict::Fail { .. }));
        assert_eq!(c, Some(NesbittRejectClass::AlgebraError));
    }

    #[test]
    fn judge_rejects_missing_equality_on_final() {
        let j = NesbittStepJudge::new();
        let bad = "Divide by 2: a/(b+c) + b/(a+c) + c/(a+b) ≥ 3/2.";
        let prior: Vec<String> = (0..7).map(|i| format!("s{}", i)).collect();
        let (v, c) = j.verdict_for_stage(bad, NesbittStage::Step8ConcludeAndEq, &prior);
        assert!(matches!(v, JudgeVerdict::Fail { .. }));
        assert_eq!(c, Some(NesbittRejectClass::MissingEqualityCase));
    }

    #[test]
    fn judge_rejects_off_stage() {
        let j = NesbittStepJudge::new();
        let bad = "consider the polynomial x^5 - 1 and its roots";
        let prior: Vec<String> = (0..4).map(|i| format!("s{}", i)).collect();
        let (v, c) = j.verdict_for_stage(bad, NesbittStage::Step5ApplyAmGm, &prior);
        assert!(matches!(v, JudgeVerdict::Fail { .. }));
        assert_eq!(c, Some(NesbittRejectClass::OffStage));
    }

    #[test]
    fn judge_advances_through_full_proof() {
        let j = NesbittStepJudge::new();
        let stages = [
            NesbittStage::Step1Substitute,
            NesbittStage::Step2Rewrite,
            NesbittStage::Step3Expand,
            NesbittStage::Step4Group,
            NesbittStage::Step5ApplyAmGm,
            NesbittStage::Step6Sum,
            NesbittStage::Step7Subtract,
            NesbittStage::Step8ConcludeAndEq,
        ];
        let mut prior: Vec<String> = Vec::new();
        for stage in stages {
            assert_eq!(j.current_stage.get(), stage);
            let (v, _) = j.verdict_for_stage(canonical_step(stage), stage, &prior);
            assert!(v.is_pass(), "stage {:?} should pass canonical step", stage);
            prior.push(canonical_step(stage).into());
            j.advance();
        }
        // After Step 8 advance, the judge has no next stage.
        assert!(!j.advance() || j.current_stage.get() == NesbittStage::Step8ConcludeAndEq);
    }
}
