# Gemini Paper 1 v2 Adversarial Audit (round 2)
**Date**: 2026-04-24
**Elapsed**: 56.2s
**Prompt size**: 51,195 chars

---

Here is a structured critique of the Paper 1 v2 draft, adhering to the specified role and format.

---

## Adversarial Audit Report: Paper 1 v2

This review assesses the v2 draft's resilience to peer-review scrutiny. While the revisions from v1 represent a significant methodological improvement—particularly the pre-registration, N=4 ablation, and honest disclosure of the proxy-saturation incident—several critical issues remain that would likely prevent acceptance at a top-tier systems/reliability venue.

### 1. Statistical challenges (STAT)

The statistical discipline is much improved, but a key inconsistency in the multiplicity correction remains.

*   **Inconsistent Family-of-Tests Definition**: The pre-registration document (`PREREG_E1V2...`) defines the family for Bonferroni correction as size=4, listing: (1) hard-set A vs B, (2) ablation vs B, (3) easy-set Δ, and (4) per-seed containment. However, the paper's Results (§ 4) reports p-values for three distinct McNemar tests on the hard-set data: (1) B vs A, (2) Ablation vs A, and (3) B vs Ablation. The "Ablation vs A" test, which yields the borderline p=0.0156 result (§ 4.2.1), is not explicitly enumerated in the pre-registered family. A reviewer would flag this as post-hoc hypothesis testing. The author must either (a) state that the pre-registered family was size=3 (A-B, Abl-A, B-Abl) and re-calculate α=0.05/3=0.0167, which would change the interpretation of the Abl-A result, or (b) justify why the Abl-A test was performed but not included in the pre-registered multiplicity plan.
*   **Low Power for Secondary Claims**: The paper correctly concludes that the Meta-Planner's contribution is not statistically distinguishable (p=0.34). However, with only 6 discordant pairs (4 vs 2), the experiment had very low power to detect anything but a large effect. The minimum number of discordant pairs needed to clear α=0.0125 in one direction (e.g., 7 vs 0) was not achieved. This is a limitation of N=40, and while acknowledged, it means the claim that the Meta-Planner is "unresolved" is weak; the experiment was not designed with sufficient power to resolve it in the first place.

### 2. Experimental-design challenges (DESIGN)

The pre-registered sampling is a major strength, but a critical anomaly in the sample itself undermines the core premise of the "hard" problem set.

*   **Sample Contamination / "Hard-Set" Invalidity**: The `hard36` pool was defined as problems that FAILED in two prior baseline seeds (§ 1.3, Appendix A). However, the results (§ 4.5) show that one problem from the `hard10` sample, `mathd_algebra_246`, is now solved by the homogeneous control condition (A) in **100% of seeds (4/4)**. This is a fatal flaw in the sample construction. This problem is no longer "hard" for the control. This means the baseline (A) solve rate on the *actually hard* 9 problems is 0/36, not 4/40. The treatment (B) solves this same "easy" problem in all 4 seeds, meaning its performance on the *actually hard* 9 problems is 8/36. The effect is still present (8 vs 0), but the reported aggregates (12 vs 4) are misleading. This anomaly could be due to a model update at `deepseek.com` between pool construction and the experiment run, or a subtle harness improvement. This **must** be investigated and discussed; it calls the validity of the entire `hard36` pool into question.
*   **Inconsistent Negative Control**: The easy-set negative control data was reused from v1 (§ 4.3) and not re-run with the final `BUILD_SHA: 29ab43a` runtime. While the author argues this is acceptable, a rigorous reliability paper would demand that all reported data, including controls, be generated with the exact same version-pinned code. The pre-registration document also listed the easy-set as an inferential test in the family-of-4, but it's treated as a historical footnote in the paper. This is a minor but noticeable inconsistency.

### 3. Causal-attribution challenges (CAUSE)

The reframing to "portfolio effect" is a major improvement, but the paper still struggles to isolate the cause of the effect due to a key confound.

*   **Meta-Cognitive Confound**: The paper commendably flags the "prompt leakage caveat" (§ 6.3) where the Meta-Planner prompt is meta-cognitive, unlike the other three tactic-list prompts. This is more than a "leakage" issue; it's a fundamental confound. The experiment cannot distinguish between "gains from heterogeneity of 4 peer skills" and "gains from adding one meta-cognitive prompt to a pool of 3 object-level skills." The conclusion that the effect is from "generic prompt heterogeneity" (§ Abstract) is therefore imprecise. A more accurate, though weaker, conclusion would be "gains from a portfolio of prompts including one with meta-cognitive instructions." The current design cannot justify the stronger "generic" claim.
*   **Mechanism of Heterogeneity**: The paper correctly refutes the Meta-Planner as *the* mechanism but offers little insight into the alternative. The claim that heterogeneity "collectively cover[s] more of the tactic space" (§ 1.2) is asserted but not strongly evidenced. The tactic-composition analysis (§ 5.2) is descriptive but shallow. A stronger analysis would show which specific tactics from which specific skill prompts were essential for the B-unique solves.

### 4. Prompt-leakage / symmetry challenges (LEAKAGE)

This category overlaps with CAUSE, but focuses on the fairness of the comparison between conditions.

*   **Abstraction-Level Asymmetry**: As noted in CAUSE, the Meta-Planner prompt (§ 3.3) operates at a higher level of abstraction ("propose a high-level TACTIC FAMILY SHIFT") than the other prompts ("Focus on algebraic simplification..."). This breaks the symmetry of the comparison. Condition B is not just "4 skills vs 1 skill," it's "3 skills + 1 manager vs 1 skill." This asymmetry is a significant weakness that is noted but not resolved.
*   **Token Budget / Information Content**: The Meta-Planner prompt is significantly longer and more complex than the `skill_0` prompt. A skeptical reviewer could argue that condition B simply provides the model with more total information or a richer context. While this is part of the treatment, it complicates the "portfolio effect" interpretation. A better control would have been four distinct but equally complex, object-level skill prompts.

### 5. Reproducibility challenges (REPRO)

This is the paper's strongest area, but minor issues persist.

*   **Honest but Unplanned Deviation**: The proxy-saturation incident (§ 3.5) was handled with impressive transparency. However, it remains a deviation from the (unspecified) execution plan in the pre-registration. The pre-reg did not specify a concurrency policy, which was an omission. While the serial re-run was the correct scientific choice, it highlights that the pre-registration was incomplete.
*   **Deferred Analysis**: Key analyses are "deferred to Appendix C" which is incomplete (§ 4.5, § 4.6). Specifically, the node-count distribution and winning-agent distribution are critical for interpreting the "multi-agent" aspect of the work. A complete paper submitted for review must include this data, as it directly impacts the interpretation of whether solves are single-agent discoveries or multi-agent collaborations.

### 6. Claim-strength challenges (CLAIM)

The claims have been appropriately scaled back from v1, but the sample contamination issue (§2 DESIGN) weakens the headline numbers.

*   **Misleading Effect Size in Abstract**: The abstract reports "heterogeneous solves **12/40** vs homogeneous **4/40**". As established in DESIGN-1, this is numerically correct but interpretively misleading due to the `mathd_algebra_246` anomaly. 4 of the solves on *both* sides come from this single, no-longer-hard problem. The effect on the remaining 9 hard problems is 8/36 vs 0/36. The claim should be rephrased to reflect this, for example by focusing on the number of *distinct hard problems* solved (B solved 4 distinct hard problems that A never solved).
*   **"Portfolio Effect" Overreach**: While a good framing, calling it a "portfolio effect" (§ Abstract, § 1.2) implies a known mechanism from finance (diversification reducing risk/increasing returns). The paper demonstrates a performance gain from diversity but doesn't provide enough evidence to confirm the mechanism is analogous to a financial portfolio (e.g., by showing that different skills excel on different problem types, reducing the risk of failure on any single type). The term is suggestive but not fully justified by the analysis provided.

---

### VERDICT

**CHALLENGE**

The paper is a model of methodological improvement and transparency, but the discovery of the `mathd_algebra_246` anomaly fundamentally compromises the "hard-set" evaluation and inflates the headline result. This, combined with the unresolved multiplicity definition and the meta-cognitive confound, means the work is not yet ready for publication.

### Required Changes Before Submission

| Priority | Category | Change | Rationale |
|---|---|---|---|
| **P0** | DESIGN | Investigate, report on, and re-frame results around the `mathd_algebra_246` anomaly. | This problem's 100% solve rate in the control condition invalidates its status as "hard" and misleads the reader about the baseline and the magnitude of the effect. |
| **P0** | STAT | Clarify the pre-registered family of hypotheses and apply the Bonferroni correction consistently. | The current paper tests a hypothesis (Abl vs A) not listed in the pre-reg's family, creating ambiguity about the validity of the p-value and correction. |
| **P1** | CAUSE | Re-scope the primary claim from "generic heterogeneity" to "heterogeneity including a meta-cognitive prompt". | The current design cannot disentangle the effect of diversity from the effect of adding a higher-level instruction, so the claim must be more precise. |
| **P1** | DESIGN | Re-run the easy-set negative control with the final `BUILD_SHA` code. | Ensures full experimental consistency and adherence to the spirit of a high-reliability, pre-registered study. |
| **P2** | REPRO | Complete the deferred analyses (node-count, winning-agent distribution) in Appendix C. | This data is required for readers to assess the "multi-agent" nature of the solutions and is not optional detail. |
| **P2** | CLAIM | Add the problem-level clustered sensitivity analysis mentioned as a v2 fix (P1-9). | This was promised as a fix to address problem-level effects and would strengthen the statistical claims by accounting for non-independence. |

### Top 3 Must-Fix Items

1.  **Address the `mathd_algebra_246` Anomaly**: The paper must explain why a problem from the "hard" pool was solved 100% of the time by the control. All results must be re-analyzed and re-presented separating this problem from the 9 truly hard ones.
2.  **Reconcile Pre-Registration with Analysis**: The author must provide a single, consistent definition for the family of hypotheses under multiplicity correction and justify it based on the pre-registration document. The p-value for the "Ablation vs A" test must be interpreted under this corrected framework.
3.  **Refine Causal Claim Regarding Meta-Cognition**: The abstract and conclusion must be rewritten to state that the observed effect comes from a portfolio containing mixed-abstraction prompts, acknowledging that the experiment does not isolate the effect of "generic" same-level skill heterogeneity.

### Specific Claim to Cut

Cut this sentence from the abstract and §4.1: **"Effect size: tripled absolute solve count (3× from 4 to 12)..."**

This claim is the most directly compromised by the `mathd_algebra_246` anomaly. Since that problem accounts for all 4 of the control's solves, the "tripling" is an artifact of a flawed baseline. A more honest framing would be: "The heterogeneous condition solved 5 distinct problems, four of which were never solved by the homogeneous condition, which only ever solved one." This focuses on the breadth of problems solved, which is the core of the "portfolio" argument and is robust to the anomaly.