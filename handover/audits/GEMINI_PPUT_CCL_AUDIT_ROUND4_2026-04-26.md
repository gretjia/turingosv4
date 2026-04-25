# Gemini PPUT-CCL PREREG Adversarial Audit (Phase A4 ROUND 4)
**Date**: 2026-04-26
**Target**: handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md (round 4)
**Elapsed**: 34.5s
**Prompt size**: 603,865 chars

---

This is a round 4 audit. My previous verdicts were PASS in round 2 and PASS in round 3. The round 4 changes were prompted by Codex's still-open CHALLENGE items from round 3. My task is to determine if these changes hold my PASS or introduce new issues.

---

### R4-1: STAT unit/power inconsistency

-   **Closure check**: **Closed**. Codex's R3 P0 correctly identified that the document was internally inconsistent regarding the statistical unit of analysis. The round 4 "clean rewrite" of § 5 and § 9 resolves this completely by establishing a single, consistent source of truth: the independent unit is **per-problem** for both Phase C (n=10) and Phase E (n=54). This eliminates the ambiguity and makes the statistical plan executable as written.

-   **New-issue check**: **No new issues**. The rewrite does not introduce new problems.
    1.  **Power Calculation Verification**: I have verified the power calculations in § 9.4 and § 9.5.
        -   The family size is now `N_max = 4 + 3*10 = 34`. The smallest Holm threshold is `α = 0.05 / 34 ≈ 0.00147`.
        -   **Phase C (n=10)**: For a one-sided binomial test, the probability of getting 10/10 successes is `0.5^10 ≈ 0.000977`, which is `≤ 0.00147`. The probability of getting 9/10 or more successes is `11 * 0.5^10 ≈ 0.0107`, which is `> 0.00147`. Therefore, the claim in § 9.4 that **Phase C requires 10/10 paired wins** is mathematically correct.
        -   **Phase E (n=54)**: The claim in § 9.5 that `≥39/54` successes are required at the smallest threshold is also correct. The one-sided p-value for 39 successes is `≈ 0.000871` (which is `≤ 0.00147`), while the p-value for 38 successes is `≈ 0.00211` (which is `> 0.00147`).
    -   The power tables are correct. The high bar for significance (10/10 in Phase C) is explicitly acknowledged as a pre-registered tradeoff, which is a sign of rigor, not a new issue.

### R4-2: j-RR mathematically unwinnable

-   **Closure check**: **Closed**. Codex's R3 P0 that the j-RR inferential test was mathematically unwinnable was a critical finding. The round 4 fix in § 5.4 — reclassifying j-RR as a **descriptive guardrail** (a point check `RR_j ≤ p_0`) and removing it from the inferential family — is the correct and standard way to resolve this. It directly closes the "unwinnable test" problem.

-   **New-issue check**: **No new issues**.
    1.  **Certification Logic Soundness**: The logic is sound. The document's justification that j-RR is a *constraint* (a safety check) rather than a *discovery hypothesis* is statistically defensible. A point check is the appropriate tool for a constraint.
    2.  **Internal Consistency**: I have verified that this change was propagated consistently across all relevant sections. The definitions of `WBCG_PPUT` (§ 1.6), `ArtifactState` (§ 1.7), Gate H (§ 7), the Phase E execution plan (§ 6 E3), and the claim language (§ 10.1) all now correctly refer to "three inferential tests" plus the "j-RR point check guardrail". The family size and `N_max` are also correctly updated to 34. The change is well-integrated and introduces no contradictions.

### R4-3: Sealing generalization

-   **Closure check**: **Closed**. Codex's R3 P0 noted that the hash defense was too literal. The round 4 generalizations in § 2.3 are comprehensive and directly address this.
    -   **L3 (Tool calls)** now blocks not just the literal seed string but also substring and concatenation patterns, a wide range of shell hash binaries, and inline Python/Node invocations. This is a massive improvement.
    -   **L4 (Static analysis)** now includes a broad blacklist of hash libraries across Python, Rust, and JS, and even a clever check for SHA-256 IV constants to detect manual implementations.
    -   **L5 (Path enumeration block)** is the most critical addition. By blocking `ls`, `find`, `rg --files`, `glob.glob`, etc., on the MiniF2F/Test directory, it closes the vector where an agent could enumerate the source pool to apply the bucketing rule to.
-   This multi-layered defense now appears robust against all reasonable recomputation attack patterns an agent might attempt during the arc.

-   **New-issue check**: **No new issues**. These are pure improvements to the operational security of the experiment and do not introduce any new statistical or logical flaws. The honest acknowledgement of residual risk (e.g., git history) remains appropriate.

### Internal Consistency

The document hangs together very well. The clean rewrite of § 5 and § 9 has resolved the patch-stacking inconsistencies that Codex flagged. The removal of j-RR from the inferential family has been applied consistently everywhere. I found no stale references or new contradictions introduced by the round 4 changes. The changelog is meticulous and accurately reflects the evolution of the protocol.

---

-   **VERDICT**: **PASS**
-   **Top 3 still-open or new P0**: None. The round 4 revisions have comprehensively and correctly closed all three of the P0s raised by the Codex round 3 audit. My previous PASS verdicts from rounds 2 and 3 are maintained and strengthened.
-   **Conviction**: High. The fixes are not superficial patches; they are principled and robust solutions to the subtle but critical issues identified in the previous round. The statistical plan is now internally consistent, the certification gate is mathematically reachable, and the heldout sealing is operationally strong.
-   **Are 4 revision rounds + 1 clean rewrite a sign of structural instability or normal pre-registration tightening?**: This is a sign of **normal and highly effective pre-registration tightening**. The core research thesis and experimental arc have remained stable. The revisions have progressively hardened the *protocol* against valid adversarial critiques, which is the exact purpose of a multi-round, dual-auditor process. The final document is significantly more rigorous than the initial draft, demonstrating the value of the process, not a flaw in the underlying research.