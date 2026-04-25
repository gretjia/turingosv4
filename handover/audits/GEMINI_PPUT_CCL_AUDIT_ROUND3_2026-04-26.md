# Gemini PPUT-CCL PREREG Adversarial Audit (Phase A4 ROUND 3)
**Date**: 2026-04-26
**Target**: handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md (round 3)
**Elapsed**: 33.5s
**Prompt size**: 354,668 chars

---

This is a round 3 audit. My previous verdict was PASS in round 2. The round 3 changes were prompted by Codex's still-open CHALLENGE items from round 2. My task is to determine if these changes hold my PASS or introduce new issues.

### 1. R3-1 (family timing & independent unit)

The round 2 design had a subtle but critical flaw: the Holm-Bonferroni correction thresholds depended on the final family size `N = 4 + 4k`, but the Phase C decisions had to be made before `k` (the number of artifacts) was known. Furthermore, it treated `(problem, seed)` as an independent unit, which is statistically questionable as seeds within the same problem are correlated.

The round 3 fixes address both points comprehensively and correctly:
- **§ 9.1 (Family Timing):** The introduction of a pre-committed `k_max = 10` to define a frozen `N_max = 44` for all multiplicity corrections is the standard, correct way to handle a data-dependent family size in a pre-registration. It resolves the timing paradox cleanly. The choice to accept a potential loss of power if `k < k_max` is the right trade-off for maintaining statistical validity.
- **§ 9.2 (Independent Unit):** Changing the independent unit to **per-problem** (n=10 for the hard-10 set) and treating seeds as noise-reducing replicates is statistically more honest and robust. The power analysis in § 9.5 is correctly recomputed for this smaller, more defensible sample size.

These changes are statistically valid and represent a significant strengthening of the experimental design. They introduce no new issues.

### 2. R3-2 (p_0 protocol)

The round 2 design introduced `p_0` (baseline regression rate) but left its calibration protocol underspecified, creating a new tuning surface as Codex correctly identified.

The round 3 fix in **§ 5.2** is a textbook example of how to close such an attack surface. It specifies every single parameter of the calibration process:
- The exact `toggle` for the experiment.
- The `sample` (full adaptation set, pre-defined seeds).
- The `estimator` (conservative worst-case).
- The `schedule` (mandatory at Phase B7).
- The `freeze point` and `audit trail` (value and data hash written to the Trust Root).
- A `ceiling` (`p_0 > 0.10` is an abort condition, preventing gaming).

This protocol removes all analyst degrees of freedom. The process is now deterministic, transparent, and auditable. This completely closes the tuning-surface attack.

### 3. R3-3 (sealing 5 layers)

The round 2 operational sealing was an improvement but, as Codex noted, still had a leak vector: an agent could grep the public seed string from non-blacklisted files (like audit reports or architect notes) and run `hashlib` itself to recompute the heldout set.

The round 3 fix in **§ 2.3** is a multi-layered, defense-in-depth solution that appears operationally complete:
- **L2 (Prompt Context Blacklist)** is now expanded to cover any file containing the literal seed string, closing the "grep from other files" vector.
- **L3 (Tool Call Args)** now scans arguments, blocking `echo "SEED" | ...` style attacks.
- **L4 (Hashlib Gate)** blocks the agent from importing or calling the necessary tools for recomputation.
- **L5 (Source Pool Blacklist)** is the final, decisive layer. Even if an agent could somehow bypass L1-L4, it cannot enumerate the source pool of problem IDs to apply the bucketing rule to.

This 5-layer system is exceptionally thorough and closes the recomputation leak vector. The honest acknowledgement of the residual git-history risk is appropriate.

### 4. R3-4 (claim caveat)

The round 2 LOO measurement protocol was sound, but the interpretation of its results could be overstated. The round 3 fix in **§ 10** adds a new row explicitly constraining the claim language for per-artifact PPUT gains to "marginal contribution given Q\{j}", clarifying that it is not a measure of standalone efficacy. This is a precise and necessary clarification that aligns the potential claims with the actual measurement protocol.

### Internal Consistency

I have reviewed the full PREREG document again. The round 3 patches are well-integrated. The new definitions in § 2.3, § 5.2, and § 9 are consistently referenced in the execution plan (§ 6) and claim constraints (§ 10). The increased complexity has not introduced contradictions; rather, it has resolved the ambiguities that Codex correctly identified in round 2. The document now hangs together more tightly than before.

---

- **VERDICT**: PASS
- I confirm that all round-3 changes are sound. They comprehensively address the still-open P0s raised by the Codex round-2 audit without introducing new issues or regressing any of the items that led to my round-2 PASS. The pre-registration is now significantly more rigorous.
- My conviction is high; the fixes are not just patches but are principled, state-of-the-art solutions to subtle pre-registration challenges.
- The three revision rounds are a sign of a healthy and effective adversarial audit process, not structural instability; the core research thesis has remained unchanged while the experimental protocol has been progressively hardened against valid critiques.