# Gemini PPUT-CCL PREREG Adversarial Audit (Phase A4 ROUND 2)
**Date**: 2026-04-26
**Target**: handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md (round 2)
**Elapsed**: 28.8s
**Prompt size**: 109,752 chars

---

Here is the round-2 verdict.

---

### P0-stat (your axis 2, family-of-5 omits per-artifact guardrails)

**closed** — The new § 9.1 explicitly defines the family as `4 + 4k` tests and § 9.2 pre-commits to the Holm-Bonferroni procedure, fully addressing the multiplicity gap.

### P0-gate-h (your axis 5, RR=0 brittleness)

**closed** — The revised Gate H in § 7 replaces the brittle `RR=0` rule with a one-sided exact binomial test against a baseline `p_0` that § 5.2 requires to be pre-calibrated and frozen before Phase E.

### P0-spof (your axis 6, human meta-predicate single-point-of-failure)

**closed** — The new contingency plan in § 6 D5 is complete, specifying a 48h SLA, a deferred queue, a non-pausing clock, and a hard abort condition, fully mitigating the single-point-of-failure risk.

### P0-leak (Codex axis 3, operational vs cryptographic sealing)

**closed** — § 2.3 is completely reframed to honestly describe "operational sealing," correctly repositioning SHA-256 as an integrity check and adding strong, testable controls like a context-blacklist to prevent leakage.

### P0-measure (Codex axis 5, Gate H per-artifact measurement)

**closed** — The revised § 6 E2 specifies a clear and statistically valid leave-one-out protocol within a single evaluation campaign, resolving the ambiguity of how per-artifact effects are measured while preserving single-touch discipline.

---

- **VERDICT**: PASS
- My conviction is high; the authors have comprehensively addressed every P0 finding from both auditors with high-quality, rigorous revisions that significantly strengthen the experimental design.
- I am not concerned about new issues; the Holm-Bonferroni procedure is algorithmically specified, and the leave-one-out protocol appears to be correctly handled with paired statistical tests, avoiding over-counting of sample independence.