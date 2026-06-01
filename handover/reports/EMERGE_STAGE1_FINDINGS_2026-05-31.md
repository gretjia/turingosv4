# EMERGE Stage-1 Findings — heterogeneous strong-model capability on real Lean (44-pool)

**Date:** 2026-05-31 · **Run:** `/tmp/emerge_stage1.json` · wall 12925s (3.6h), 999 LLM calls, 995 Lean verifications
**Prereg:** `handover/preregistration/EMERGE_STAGE1_PREREG_2026-05-31.json` (decision rules locked before any result read)

## Verdict against the locked prereg

| thesis | verdict |
|---|---|
| **A — verifier coverage** (sampling + Lean rescues low-p) | **VALIDATED** |
| **B — heterogeneous combination** (union > best single) | **VALIDATED literally (+8)** — but the *cross-family* mechanism is **REFUTED**; one confound unresolved |
| → Stage-2 gate (combination-target ≥ 3) | **PROCEED** (8 targets) |

## Setup

3 strong models across 2 API channels via the de-branded gateway: `deepseek-v4-pro` (DeepSeek **official**
channel), `deepseek-ai/DeepSeek-V3.2` + `Qwen/Qwen3-32B` (SiliconFlow). Full **unbiased** 44 real-Mathlib pool
(not pre-filtered by any model's difficulty), k=4, temp 0.8+0.05·s, max_rounds 2, in-harness `#print axioms`
whitelist gate {propext, Classical.choice, Quot.sound}.

## A — Verifier coverage: VALIDATED (cleanly)

| model | pass@1 (mean p̂) | pass@4 (union over 4 draws) | lift |
|---|---|---|---|
| deepseek-v4-pro | 0.256 | 0.409 | **+0.153** |
| deepseek-ai/DeepSeek-V3.2 | 0.182 | 0.364 | **+0.182** |
| Qwen/Qwen3-32B | 0.034 | 0.045 | +0.011 |

Drawing k samples and letting the Lean verifier pick any correct one ~**1.5–2× single-shot** for the strong
models. This directly answers the architect's high-temperature "lucky-guess / 蒙对" question from first
principles: for a theorem with single-shot p>0, sampling-and-verifying is rational — pass@k = 1−(1−p)^k climbs,
and the verifier (not a vote) means there is no ½ ceiling. For p≈0 theorems no k rescues (18/44 ceiling below).

## B — Combination: literal +8, but the cross-family mechanism is REFUTED

- union(3 models) = **26/44** vs best single (v4-pro) = **18/44** → **+8 (+44% relative)**. 8 theorems the
  *flagship* cannot reach in 4 draws but the combination can.
- **The diversity, however, is not cross-family.** Complementary-pair matrix:

  | pair | a_only | b_only |
  |---|---|---|
  | Qwen vs V3.2 | **0** | 14 |
  | Qwen vs v4-pro | **0** | 16 |
  | V3.2 vs v4-pro | 8 | 10 |

  Qwen/Qwen3-32B is **strictly dominated** — `a_only = 0` against BOTH stronger models; its 2 solves
  (lm_B1, lm_poly_eval_factor) are a subset of both. The genuinely cross-**family** model (DeepSeek vs Qwen)
  contributed **zero** complementarity. Hong-Page diversity D ≈ 0 here.
- The real, substantial complementarity is **v4-pro ⟷ V3.2** — same DeepSeek family, different *generation*:
  symmetric 8 V3.2-only + 10 v4-pro-only.

**Honest implication.** "Many small / cross-family models → emergent capability ≥ a big model" is **not
supported at this scale**: the smaller cross-family model (Qwen3-32B) was dominated and added nothing. What IS
supported: **two strong models of different generations have complementary coverage**, and a verifier turns
that into +44% solve-rate. The value is strong-model combination + verifier coverage, not small-model swarm
emergence — which is consistent with the architect's own stated intuition that small models have a capability
ceiling.

## The unresolved confound (do not oversell)

At k=4 *each*, union(2 models) draws **8 samples/theorem vs 4**. Part of the +8 could be pure extra sampling,
not distinct capability. Evidence it is *partly* genuine: the complementarity is **bidirectional** (v4-pro also
solves 10 that V3.2 misses) — pure variance would not produce a stable symmetric split. But the clean control
is **v4-pro @ k8 vs union(v4-pro@k4, V3.2@k4)** at equal total draws — exactly Stage-2's homogeneous arm. Until
that runs, "combination beats the best single model" is literally true at equal *per-model* budget but NOT yet
proven at equal *total* budget.

## Data quality (真题真跑 checks passed)

- Qwen's low score is **real, not a parse artifact**: it shares solves with the others, used 181K tokens
  (comparable to v4-pro 198K) → full generations, not truncated. 999 LLM / 995 Lean calls ⇒ ~4 parse failures
  total — the extract_tactic robustness fix holds across all 3 model output formats at scale.
- The two prior harness bugs (JSON drop, theorem-name corruption) did **not** recur.

## Limitations to close in Stage-2

1. **Proofs not persisted** — the harness records hit counts, not winning proof bodies, so the 26 solves are
   verified only by the inline axiom gate (audited rigorous: source-grep + positive `#print axioms` whitelist),
   not *independently* re-verified. Stage-2 must persist winning proofs and re-run `bench_axiom_reverify.py` on
   them for a defensible headline.
2. **Qwen3-32B is too weak to fairly test cross-family diversity among _strong_ models.** A fair arm = a strong
   non-DeepSeek model (e.g. Qwen2.5-72B-Instruct), to ask whether cross-family helps when the family-mate is
   actually competitive.

## Stage-2 (reframed by these findings)

Primary control: **union(v4-pro@k4, V3.2@k4) vs v4-pro@k8** (equal total draws) — is combination *capability* or
just *more sampling*? Then: TuringOS price routing over the 8 combination-target theorems
`[lm_commute_pow, lm_det_3x3, lm_det_mul, lm_f, lm_ineq2, lm_monotone_glue, lm_sum_cubes, lm_trace_prod]`; a
fairer cross-family strong arm; persist + independently re-verify all winning proofs.

## Constitution posture

FC1/2/3 untouched, no §6 surface, integer money, de-branded multi-channel calling, prereg locked before read,
PR-only. The 18/44 none-solved set is the real hard-ceiling frontier proxy (no model solved in 4 draws).
