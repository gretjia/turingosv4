//! TRACE_MATRIX FC1a-predicate_pi: Putnam 2024 A1 step-level judge.
//!
//! Problem (Putnam 2024 A1):
//!   Determine all positive integers n such that there exist positive
//!   integers a, b, c satisfying 2·a^n + 3·b^n = 4·c^n.
//!
//! Answer: n = 1, witnessed by (a, b, c) = (1, 2, 2). For n ≥ 2 no
//! solution exists; the official proof uses two cases:
//!   - n = 2: mod-3 analysis forces all of a, b, c to be multiples of 3,
//!            contradicting WLOG gcd(a, b, c) = 1.
//!   - n ≥ 3: a 2-adic infinite descent argument shows b, then a, then c
//!            are all even, again contradicting gcd = 1.
//!
//! Source: 85th Putnam (Bhargava–Kedlaya–Ng official solutions,
//! kskedlaya.org/putnam-archive/2024s.pdf).
//!
//! This is "extreme stress" because (a) DeepSeek-chat is not the math
//! DeepSeekMath-V2 model — it is known to hand-wave through infinite
//! descent (arXiv:2509.24827), and (b) the judge is intentionally
//! strict: every stage must mention specific structural markers
//! (mod 3, even, gcd, etc.). Failures should be plentiful.
//!
//! On-disk §8: handover/directives/2026-05-22_TDMA_BOUNDED_RC1_DIRECTIVE_AND_§8.md

use std::cell::Cell;

use super::math_step_judge::{JudgeVerdict, MathStepJudge};

/// Eight canonical stages of the Putnam 2024 A1 proof.
/// TRACE_MATRIX FC1a-predicate_pi: Sequential proof state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PutnamA1Stage {
    Stage1WitnessN1,    // verify (1,2,2) works for n=1: 2+6=8=4·2
    Stage2WlogGcd,      // assume gcd(a,b,c)=1 WLOG
    Stage3N2Mod3,       // for n=2, derive a²+c² ≡ 0 (mod 3) ⇒ 3|a, 3|c
    Stage4N2ContradictB,// show 3|b too, contradicting gcd=1
    Stage5N3DescentB,   // for n≥3, from 3b^n = 4c^n − 2a^n derive b even
    Stage6N3DescentA,   // derive a even
    Stage7N3DescentC,   // derive c even
    Stage8Conclude,     // all-even contradicts gcd=1; therefore n=1 only
}

impl PutnamA1Stage {
    /// TRACE_MATRIX FC1a-predicate_pi: Next stage in the canonical proof.
    pub fn next(self) -> Option<Self> {
        use PutnamA1Stage::*;
        Some(match self {
            Stage1WitnessN1 => Stage2WlogGcd,
            Stage2WlogGcd => Stage3N2Mod3,
            Stage3N2Mod3 => Stage4N2ContradictB,
            Stage4N2ContradictB => Stage5N3DescentB,
            Stage5N3DescentB => Stage6N3DescentA,
            Stage6N3DescentA => Stage7N3DescentC,
            Stage7N3DescentC => Stage8Conclude,
            Stage8Conclude => return None,
        })
    }

    /// TRACE_MATRIX FC1a-predicate_pi: Human-readable stage label.
    pub fn label(self) -> &'static str {
        use PutnamA1Stage::*;
        match self {
            Stage1WitnessN1 => "Stage1-Witness-n=1",
            Stage2WlogGcd => "Stage2-WLOG-gcd=1",
            Stage3N2Mod3 => "Stage3-n=2-mod3",
            Stage4N2ContradictB => "Stage4-n=2-b-mult-of-3",
            Stage5N3DescentB => "Stage5-n>=3-b-even",
            Stage6N3DescentA => "Stage6-n>=3-a-even",
            Stage7N3DescentC => "Stage7-n>=3-c-even",
            Stage8Conclude => "Stage8-Conclude-n=1-only",
        }
    }
}

/// Reject classes mirroring the IneqMath taxonomy, adapted to A1.
/// TRACE_MATRIX FC1a-predicate_pi: Orthogonal failure signatures.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PutnamA1RejectClass {
    MissingWitness,        // Stage 1: didn't verify (1,2,2) works numerically
    MissingGcdAssumption,  // Stage 2: didn't reduce by gcd
    WrongMod,              // Stage 3: didn't use mod 3 / wrong residues
    MissingDescentStep,    // Stage 5/6/7: skipped the "b/a/c is even" derivation
    HandWave,              // Step text too short / lacks reasoning
    WrongFinalAnswer,      // Stage 8: concluded wrong n
    OffStage,              // Doesn't match any expected stage marker
    LogicalGap,            // Jumped ahead without prior stages
}

impl PutnamA1RejectClass {
    /// TRACE_MATRIX FC1a-predicate_pi: Canonical reject_class string.
    pub fn reject_class_str(self) -> &'static str {
        use PutnamA1RejectClass::*;
        match self {
            MissingWitness => "missing-witness",
            MissingGcdAssumption => "missing-gcd-assumption",
            WrongMod => "wrong-mod-analysis",
            MissingDescentStep => "missing-descent-step",
            HandWave => "hand-wave",
            WrongFinalAnswer => "wrong-final-answer",
            OffStage => "off-stage",
            LogicalGap => "logical-gap",
        }
    }

    /// TRACE_MATRIX FC1a-predicate_pi: Canonical failed_predicate string.
    pub fn failed_predicate_str(self) -> &'static str {
        use PutnamA1RejectClass::*;
        match self {
            MissingWitness => "witness.numeric",
            MissingGcdAssumption => "wlog.gcd",
            WrongMod => "modular.arithmetic",
            MissingDescentStep => "descent.parity",
            HandWave => "rigor.detail",
            WrongFinalAnswer => "answer.value",
            OffStage => "stage.unrecognized",
            LogicalGap => "step.cites_prior",
        }
    }
}

/// Judge for one step of the Putnam 2024 A1 proof.
/// TRACE_MATRIX FC1a-predicate_pi: Strict stage-aware verifier.
pub struct PutnamA1Judge {
    pub current_stage: Cell<PutnamA1Stage>,
}

impl Default for PutnamA1Judge {
    fn default() -> Self {
        Self::new()
    }
}

impl PutnamA1Judge {
    /// TRACE_MATRIX FC1a-predicate_pi: Start at Stage 1.
    pub fn new() -> Self {
        Self {
            current_stage: Cell::new(PutnamA1Stage::Stage1WitnessN1),
        }
    }

    /// TRACE_MATRIX FC1a-predicate_pi: Advance after a successful step.
    pub fn advance(&self) -> bool {
        match self.current_stage.get().next() {
            Some(n) => {
                self.current_stage.set(n);
                true
            }
            None => false,
        }
    }

    /// TRACE_MATRIX FC1a-predicate_pi: Sequential strict step verdict.
    pub fn verdict_for_stage(
        &self,
        candidate: &str,
        stage: PutnamA1Stage,
        prior_accepted: &[String],
    ) -> (JudgeVerdict, Option<PutnamA1RejectClass>) {
        use PutnamA1RejectClass::*;
        use PutnamA1Stage::*;
        let c = candidate.to_lowercase();
        let len_chars = candidate.chars().count();

        // Hand-wave check: any step below 80 chars is too thin.
        if len_chars < 80 {
            return (
                JudgeVerdict::Fail {
                    reason: format!(
                        "Stage {} step too short ({} chars) — needs explicit reasoning",
                        stage.label(),
                        len_chars
                    ),
                },
                Some(HandWave),
            );
        }

        // Logical-gap check: prior stages must be accepted before later ones.
        let stage_idx = match stage {
            Stage1WitnessN1 => 1,
            Stage2WlogGcd => 2,
            Stage3N2Mod3 => 3,
            Stage4N2ContradictB => 4,
            Stage5N3DescentB => 5,
            Stage6N3DescentA => 6,
            Stage7N3DescentC => 7,
            Stage8Conclude => 8,
        };
        if prior_accepted.len() < stage_idx - 1 {
            return (
                JudgeVerdict::Fail {
                    reason: format!(
                        "Stage {} requires {} prior steps, got {}",
                        stage.label(),
                        stage_idx - 1,
                        prior_accepted.len()
                    ),
                },
                Some(LogicalGap),
            );
        }

        match stage {
            Stage1WitnessN1 => {
                // Must mention n=1 AND a numeric witness AND verify the equation.
                let has_n1 = c.contains("n = 1") || c.contains("n=1");
                let has_witness =
                    c.contains("(1, 2, 2)") || c.contains("(1,2,2)") || c.contains("a=1") || c.contains("a = 1");
                let verifies = c.contains("2 + 6 = 8")
                    || c.contains("2+6=8")
                    || c.contains("8 = 4")
                    || c.contains("8=4")
                    || (c.contains("=8") && c.contains("4·2"))
                    || c.contains("works");
                if !(has_n1 && has_witness && verifies) {
                    return (
                        JudgeVerdict::Fail {
                            reason: "Stage 1 must (a) state n=1, (b) name witness (a,b,c)=(1,2,2), (c) verify 2+6=8=4·2".into(),
                        },
                        Some(MissingWitness),
                    );
                }
            }
            Stage2WlogGcd => {
                let has_gcd = c.contains("gcd") || c.contains("greatest common") || c.contains("common divisor");
                let has_wlog = c.contains("wlog")
                    || c.contains("without loss")
                    || c.contains("assume")
                    || c.contains("dividing");
                let has_one = c.contains("= 1") || c.contains("=1") || c.contains("coprime");
                if !(has_gcd && has_wlog && has_one) {
                    return (
                        JudgeVerdict::Fail {
                            reason: "Stage 2 must (a) mention gcd, (b) reduce WLOG by dividing or assuming, (c) state gcd=1 (or coprime)".into(),
                        },
                        Some(MissingGcdAssumption),
                    );
                }
            }
            Stage3N2Mod3 => {
                let has_n2 = c.contains("n = 2") || c.contains("n=2");
                let has_mod3 = c.contains("mod 3") || c.contains("mod3") || c.contains("modulo 3");
                let has_squares =
                    c.contains("square") || c.contains("a²") || c.contains("a^2") || c.contains("c²") || c.contains("c^2");
                let has_residue = c.contains("0 or 1") || c.contains("0,1") || c.contains("0 and 1");
                if !(has_n2 && has_mod3 && has_squares && has_residue) {
                    return (
                        JudgeVerdict::Fail {
                            reason: "Stage 3 must establish: (a) case n=2, (b) work mod 3, (c) discuss squares, (d) note only 0 and 1 are squares mod 3".into(),
                        },
                        Some(WrongMod),
                    );
                }
            }
            Stage4N2ContradictB => {
                let mentions_b = c.contains("b") || c.contains("b²") || c.contains("b^2");
                let mentions_3 = c.contains("multiple of 3") || c.contains("divisible by 3") || c.contains("3 | b") || c.contains("3|b");
                let contradicts = c.contains("contradict") || c.contains("contradiction");
                if !(mentions_b && mentions_3 && contradicts) {
                    return (
                        JudgeVerdict::Fail {
                            reason: "Stage 4 must show b is also a multiple of 3 AND state the gcd=1 contradiction".into(),
                        },
                        Some(MissingDescentStep),
                    );
                }
            }
            Stage5N3DescentB => {
                let has_n3 = c.contains("n ≥ 3") || c.contains("n >= 3") || c.contains("n>=3") || c.contains("n>3") || c.contains("n ≥3");
                let mentions_b_even = c.contains("b is even")
                    || c.contains("b must be even")
                    || c.contains("b even")
                    || c.contains("b = 2");
                let mentions_eq = c.contains("3b") || c.contains("3·b") || c.contains("4c") || c.contains("4·c");
                if !(has_n3 && mentions_b_even && mentions_eq) {
                    return (
                        JudgeVerdict::Fail {
                            reason: "Stage 5 must (a) be in case n≥3, (b) derive that b is even, (c) reference the equation 3b^n = 4c^n − 2a^n".into(),
                        },
                        Some(MissingDescentStep),
                    );
                }
            }
            Stage6N3DescentA => {
                let mentions_a_even = c.contains("a is even") || c.contains("a must be even") || c.contains("a even");
                if !mentions_a_even {
                    return (
                        JudgeVerdict::Fail {
                            reason: "Stage 6 must derive that a is even (from the rewritten equation with b/2)".into(),
                        },
                        Some(MissingDescentStep),
                    );
                }
            }
            Stage7N3DescentC => {
                let mentions_c_even = c.contains("c is even") || c.contains("c must be even") || c.contains("c even");
                if !mentions_c_even {
                    return (
                        JudgeVerdict::Fail {
                            reason: "Stage 7 must derive that c is even (closing the descent)".into(),
                        },
                        Some(MissingDescentStep),
                    );
                }
            }
            Stage8Conclude => {
                let has_n1 = c.contains("n = 1") || c.contains("n=1");
                let has_only = c.contains("only")
                    || c.contains("unique")
                    || c.contains("solely")
                    || c.contains("therefore");
                let mentions_gcd = c.contains("gcd") || c.contains("contradict");
                if !(has_n1 && has_only && mentions_gcd) {
                    return (
                        JudgeVerdict::Fail {
                            reason: "Stage 8 must conclude (a) n=1 is the unique/only answer, (b) referencing the gcd=1 contradiction".into(),
                        },
                        Some(WrongFinalAnswer),
                    );
                }
            }
        }

        (JudgeVerdict::Pass, None)
    }
}

impl MathStepJudge for PutnamA1Judge {
    fn verdict(&self, prior_steps: &[String], candidate_step: &str) -> JudgeVerdict {
        let stage = self.current_stage.get();
        let (v, _c) = self.verdict_for_stage(candidate_step, stage, prior_steps);
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_prior(n: usize) -> Vec<String> {
        (0..n).map(|i| format!("step {} accepted with sufficient detail beyond the 80-char hand-wave threshold for the judge", i)).collect()
    }

    #[test]
    fn judge_accepts_canonical_stage1() {
        let j = PutnamA1Judge::new();
        let s = "For n = 1, take the witness (a, b, c) = (1, 2, 2). We verify 2·1 + 3·2 = 8 = 4·2, so this works.";
        let (v, _) = j.verdict_for_stage(s, PutnamA1Stage::Stage1WitnessN1, &[]);
        assert!(v.is_pass(), "got {:?}", v);
    }

    #[test]
    fn judge_rejects_handwave_short_step() {
        let j = PutnamA1Judge::new();
        let s = "n=1 works.";
        let (v, c) = j.verdict_for_stage(s, PutnamA1Stage::Stage1WitnessN1, &[]);
        assert!(matches!(v, JudgeVerdict::Fail { .. }));
        assert_eq!(c, Some(PutnamA1RejectClass::HandWave));
    }

    #[test]
    fn judge_rejects_stage1_missing_witness() {
        let j = PutnamA1Judge::new();
        let s = "For n = 1 the equation has many solutions, this is straightforward and obvious to verify by direct computation.";
        let (v, c) = j.verdict_for_stage(s, PutnamA1Stage::Stage1WitnessN1, &[]);
        assert!(matches!(v, JudgeVerdict::Fail { .. }));
        assert_eq!(c, Some(PutnamA1RejectClass::MissingWitness));
    }

    #[test]
    fn judge_rejects_stage3_missing_mod3() {
        let j = PutnamA1Judge::new();
        let s = "For n = 2 we proceed by considering parities and noting that 2a² + 3b² = 4c² forces structural constraints on a, b, c.";
        let (v, c) = j.verdict_for_stage(s, PutnamA1Stage::Stage3N2Mod3, &make_prior(2));
        assert!(matches!(v, JudgeVerdict::Fail { .. }));
        assert_eq!(c, Some(PutnamA1RejectClass::WrongMod));
    }

    #[test]
    fn judge_rejects_logical_gap_when_priors_insufficient() {
        let j = PutnamA1Judge::new();
        let s = "Working mod 3, we see that a² + c² ≡ 0 (mod 3) and the only squares mod 3 are 0 and 1, so both must be 0.";
        let (v, c) = j.verdict_for_stage(s, PutnamA1Stage::Stage3N2Mod3, &[]);
        assert!(matches!(v, JudgeVerdict::Fail { .. }));
        assert_eq!(c, Some(PutnamA1RejectClass::LogicalGap));
    }

    #[test]
    fn judge_accepts_canonical_stage5_descent_b() {
        let j = PutnamA1Judge::new();
        let s = "For n ≥ 3, consider 3b^n = 4c^n − 2a^n. The right side is even, so 3b^n is even, hence b is even.";
        let (v, _) = j.verdict_for_stage(s, PutnamA1Stage::Stage5N3DescentB, &make_prior(4));
        assert!(v.is_pass(), "got {:?}", v);
    }
}
