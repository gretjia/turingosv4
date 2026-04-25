# Gemini Paper 1 v2.1 Adversarial Audit (round 3)
**Date**: 2026-04-25
**Target commit**: d349a86
**Elapsed**: 32.8s
**Prompt size**: 89,623 chars

---

Excellent. The author has taken the round-2 feedback seriously. I will now conduct my independent round-3 audit of v2.1.

---

## Adversarial Audit Report: Paper 1 v2.1 (Round 3)

This review finds that the author has diligently and transparently addressed all P0 blockers identified in the round-2 merged verdict. The paper's core claims are now more precise, robust to the discovered model drift, and framed with appropriate intellectual honesty. The methodological rigor has been substantially improved to the point where the central finding would survive peer review.

### 1. STAT — power, multiplicity, exact-test assumptions, family closure

-   **Round-2 P0 Blocker (P0-D)**: Reconcile the pre-registered family-of-4 vs. executed-tests inconsistency.
-   **v2.1 Fix**: § 3.6 ("Reconciliation note") directly addresses this. It transparently flags the original ambiguity in the pre-reg, explicitly closes the confirmatory family at 3 hard-set McNemar tests, and demotes the other analyses to descriptive. This is a model of how to handle pre-registration inconsistencies post-hoc.
-   **Verdict**: **Closed by v2.1**. The statistical plan is now unambiguous and defensible.

### 2. DESIGN — sample construction, negative controls, ablation scope

-   **Round-2 P0 Blocker (P0-A)**: The `mathd_algebra_246` model-drift anomaly, which compromised the "hard-set" definition and inflated the baseline.
-   **v2.1 Fix**: § 4.7 provides a superb, dedicated analysis of the drift event. It presents the evidence, correctly notes that the McNemar p-value is unaffected (as it ignores concordant pairs), and provides a "Drift-robust restatement of Table 4.1" over the 9 truly-hard problems. This turns a potential flaw into a documented, interesting finding about live model evaluation.
-   **Verdict**: **Closed by v2.1**. The handling of this critical issue is exemplary.

### 3. CAUSE — is the new "portfolio of prompts including one meta-cognitive" framing defensible

-   **Round-2 P0 Blocker (P0-B)**: Overclaiming "generic prompt heterogeneity" when the experiment had a meta-cognitive confound.
-   **v2.1 Fix**: The claim has been systematically down-scoped. The abstract, § 6.1, and Limitation 12 now precisely frame the finding as a gain from a "portfolio of prompts including one meta-cognitive instruction." The paper explicitly states it does *not* claim to have isolated generic peer-level heterogeneity.
-   **Verdict**: **Closed by v2.1**. The causal claim is now appropriately narrowed to what the data can support.

### 4. LEAKAGE — token budget, content fairness between A/Abl/B

-   **Round-2 Blocker**: The core issue was the meta-cognitive vs. object-level asymmetry, which was part of P0-B.
-   **v2.1 Fix**: As with CAUSE, this is addressed by the honest reframing and the addition of Limitation 12. The paper no longer claims a symmetric comparison, but rather reports the result of an asymmetric one. The deferred P1-D (token-budget table) remains open, but its importance is diminished now that the claim is not about "fair" peer-level skill comparison.
-   **Verdict**: **Partially closed**. The P0-level conceptual issue is closed. The residual P1 (token table) is still open but does not block the paper's core, reframed contribution.
-   **Residual**: P1 (should fix for camera-ready, but does not block arXiv).

### 5. REPRO — pre-reg discipline, artifact stability, BUILD_SHA enforcement

-   **Round-2 P0 Blocker (P0-E)**: Artifacts stored in unstable `.claude/worktrees/` paths.
-   **v2.1 Fix**: § 8.5 now lists all 12 raw jsonl files under the stable `handover/evidence/v2/` path. This is a direct and complete fix.
-   **Verdict**: **Closed by v2.1**. The evidence archive is now stable and suitable for submission.

### 6. CLAIM — does the v2.1 abstract still overclaim

-   **Round-2 P0 Blocker (P0-C)**: Using the drift-inflated "tripled absolute solve count (3× from 4 to 12)" headline.
-   **v2.1 Fix**: This language is completely removed. The abstract and § 4.1 now lead with the drift-robust framing: "B solves 4 distinct hard problems A never solved; A solves 0 hard problems B never solved; 8/8 discordant pairs favor B." This is a much stronger and more honest headline.
-   **Verdict**: **Closed by v2.1**. The claims are now precise and robust.

---

### Final Assessment

The author has successfully navigated a challenging round of revisions. Every P0 blocker identified in the round-2 audit has been addressed with transparency and rigor. The resulting paper is not only stronger but serves as a good case study in responsible reporting of unexpected experimental events like model drift. The remaining open items are correctly classified as P1/P2 and do not undermine the paper's central, now-carefully-scoped contribution. The pre-commitment to CHALLENGE has been overcome by the quality of the revision.

**VERDICT**: **PASS**

**Top 3 must-fix items**: None. The paper is arXiv-ready.

**On promoting deferred items**: The deferred v2.2 items (P1-A cluster analysis, P1-D token table, P1-E Docker transcript, P2-B Appendix C) are correctly prioritized and should **not** be promoted to P0 status for this round; they represent good-to-have polishing for a final conference version, not blockers for the current claims.