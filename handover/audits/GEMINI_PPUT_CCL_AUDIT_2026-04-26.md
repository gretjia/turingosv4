# Gemini PPUT-CCL PREREG Adversarial Audit (Phase A4 round 1)
**Date**: 2026-04-26
**Target**: handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md
**Elapsed**: 51.0s
**Prompt size**: 140,707 chars

---

An independent adversarial audit follows.

---

### 1. DEFINITION
**PASS**. The definitions for V-PPUT, C_i, T_i, ArtifactState, and the Trust Root are exceptionally precise and leave little room for post-hoc reinterpretation. The commitment to count all tokens from all branches (§ 1.2) and the wall-clock definition of T_i (§ 1.3) are strong. The 4-state machine for artifacts (§ 1.7) is unambiguous and correctly separates artifact confidence from the binary ground truth of task progress.

### 2. STAT
**CHALLENGE**. The statistical design has a significant flaw in its definition of the family of tests.
- **Finding**: The pre-registration declares a family-of-5 for H1-H5 (§ 5, § 9). However, the success criterion for any artifact to be counted towards WBCG_PPUT in Gate H (§ 7) includes three additional, implicit, one-sided hypothesis tests on the heldout set: `ΔFAR ≤ 0`, `RR = 0`, and `ΔCPR ≤ 0`. These are inferential tests conducted on the final dataset and are subject to multiplicity.
- **Impact**: Failing to account for these guardrail tests in the family-wise error rate correction understates the probability of a Type I error (i.e., falsely certifying an artifact as beneficial when its perceived gains are due to chance).
- **Required Revision**: The family of tests must be explicitly expanded to include the three guardrail hypotheses for each artifact being tested. The Bonferroni correction should apply to this larger family (e.g., 5 + 3*k tests, where k is the number of artifacts promoted to heldout evaluation), or an alternative, pre-specified procedure like the Holm-Bonferroni method must be declared to handle the full set of planned inferences.

### 3. LEAKAGE
**PASS**. The leakage prevention is state-of-the-art. The three-way split with a frozen seed and hash bucketing is sound. The specific anti-leakage conformance test `docs_do_not_include_exact_adaptation_solution` (§ 3.5), which uses a rolling hash check to prevent verbatim copying of adaptation solutions into artifacts, directly and effectively addresses the most plausible leakage vector.

### 4. GOODHART
**PASS**. The anti-Goodhart battery is comprehensive. The combination of metering-level conformance tests (§ 3) and content-level meta-predicates (§ 3.5) defends both major attack surfaces. The specific risk of a thinly-veiled lookup table is mitigated by the `parametric_templates` predicate and, ultimately, by the fact that the North Star metric (H-VPPUT) is measured on a sealed heldout set, which inherently punishes overfitting and rewards generalization.

### 5. GATE-H REACHABILITY
**CHALLENGE**. The success criterion in Gate H (§ 7) is so stringent that it risks making a null result a near-certainty, regardless of the system's actual quality.
- **Finding**: The condition `RR = 0` (Regression Rate) is exceptionally brittle. It means a single regression on any one of the 54 heldout problems, potentially due to stochastic noise or a minor edge case, would invalidate an artifact that might be strongly beneficial on average across the other 53 problems.
- **Impact**: This sets an almost unattainable bar for capability compilation within a 30-day arc. While intellectually honest, it is poorly calibrated for empirical reality. A system should be able to accept trade-offs, and a single-point failure condition is not a robust measure of progress.
- **Required Revision**: Relax the `RR = 0` condition. Replace it with a statistically more robust criterion, such as `RR ≤ 1` or a one-sided binomial test to ensure the number of regressions is not statistically significant against a pre-defined baseline noise level.

### 6. CCL-HUMAN-LOAD
**CHALLENGE**. The reliance on a single human as a live meta-predicate in Phase D (§ 6, D5) introduces a single-point-of-failure (SPOF) without a documented contingency plan.
- **Finding**: The plan states the user (`gretjia`) will review each `Accepted` artifact. It does not specify what happens if the user is unavailable for one or more days during the critical one-week Phase D window.
- **Impact**: User unavailability would halt the artifact promotion pipeline, potentially wasting valuable days of the fixed 30-day budget and jeopardizing the entire arc.
- **Required Revision**: The PREREG must add a contingency plan. For example: "If the human meta-predicate is unavailable for more than 24 hours, `Accepted` artifacts will be queued. The 30-day arc clock will not be paused. This is an acknowledged operational risk to the throughput of Phase D."

### 7. HETEROGENEITY-TIMING
**PASS**. The decision to introduce heterogeneous LLMs only at Phase D (§ 12.2) is methodologically sound. The rationale—to avoid confounding the constitutional ablation effects in Phase C with model-specific effects—is correct and demonstrates strong experimental design discipline.

### 8. TRUST-ROOT-ENFORCEMENT
**PASS**. The PREREG correctly handles the risk of implementation failure for the syscall-level enforcement. By defining the `EPERM` trap as a mandatory condition of Gate B (§ 7) and stating that failure of any gate A-G leads to a negative finding, the document correctly frames this as a hard blocker for the arc's success. This is the appropriate level of rigor.

### 9. REPRO
**PASS**. The split generation protocol is fully transparent and reproducible. The seed, script, and bucketing rule are all committed. The realized counts (144/46/54) are within the pre-declared `±5` tolerance (§ 2.2), which is an appropriate and principled way to handle the pseudo-randomness of hash bucketing.

### 10. CLAIM-LANG
**PASS**. The claim language constraints in § 10 are exemplary. They are specific, tied to concrete evidentiary thresholds, and include pre-written "default claim language for negative outcomes." This proactively prevents post-hoc narrative shaping and is a model of pre-registration discipline. The specific scenario of partial success (ablations pass, no artifacts certified) is explicitly covered in the discussion of Gate H failure (§ 7).

---

**VERDICT**: **CHALLENGE**

**P0 Must-Fix Items:**

1.  **STAT (§ 2, § 7, § 9):** The family of tests for multiplicity correction is incomplete. You must explicitly include the per-artifact guardrail tests (`ΔFAR ≤ 0`, `RR = 0`, `ΔCPR ≤ 0`) in the family and adjust the Bonferroni correction accordingly.
2.  **GATE-H REACHABILITY (§ 7):** The `RR = 0` condition is too brittle and over-determines a null result. Relax this to a more statistically robust measure (e.g., `RR ≤ 1` or a one-sided binomial test) to ensure the primary success gate is empirically reachable.
3.  **CCL-HUMAN-LOAD (§ 6, D5):** The human-in-the-loop is an undocumented single-point-of-failure. Add a contingency plan to the PREREG specifying what happens if the user is unavailable during Phase D.

**P1 Should-Fix Items:**

None. The three P0 items are the only significant blockers. Addressing them will substantially strengthen the arc's validity.

**Conviction:**
I would have flagged this PREREG as inadequate had Codex independently issued a PASS; the unacknowledged multiplicity issue is a fundamental statistical flaw that would not survive peer review.