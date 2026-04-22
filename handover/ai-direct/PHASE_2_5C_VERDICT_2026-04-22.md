# Phase 2.5c chat A/B verdict — 2026-04-22

**Data sources** (canonical):
- `experiments/minif2f_v4/logs/phase2_5c_chat_baseline_main_oneshot_20260422T144829.jsonl`
- `.claude/worktrees/phase-8a-snapshot/experiments/minif2f_v4/logs/phase2_5c_chat_experiment_oneshot_20260422T144831.jsonl`
- Sample: `experiments/minif2f_v4/analysis/sample_N20_S74677.txt` (N=20, seed 74677, fingerprint `8d390ee4eef82dbb`)
- Model: `deepseek-chat` (both conditions), `temperature=0.2`, `max_tokens=8000`
- Prompt: hardened "DO NOT wrap in markdown code fences" (commits `5499a01` main / `e86e712` exp)

## § 1. Aggregate metrics

| Metric | Main (Phase-7 HEAD) | Exp (Phase-8 branch) |
|---|---|---|
| Solved | 5 / 20 | **6 / 20** |
| Σ PPUT | **41.00** | 38.61 |
| Mean PPUT (solved) | **8.200** (CI [5.59, 10.81]) | 6.435 (CI [3.62, 9.26]) |
| Mean time to solve | 15.0 s | 27.8 s |
| Max depth | 1 | 1 |
| Σ depth ≥ 10 PPUT | 0.00 | 0.00 |

## § 2. Solve-set analysis

```
main ∩ exp = {imo_1962_p2, mathd_algebra_160, mathd_algebra_171, mathd_algebra_359, mathd_algebra_44}
main only  = ∅
exp only   = {imo_1964_p2}
neither    = 14 problems
```

**Exp strictly dominates** main's solve set (main ⊂ exp). Phase 8 changes do not reduce solve coverage.

## § 3. Paired Δ (same-problem N=20)

- Σ PPUT Δ (exp − main) = **−2.39**
- Mean PPUT Δ = **−0.119** (CI **[−0.540, +0.301]**)
- Notable per-problem:
  - `imo_1964_p2`: +2.97 (exp solved, main didn't)
  - `imo_1962_p2`: −1.95 (both solved; exp slower)
  - `mathd_algebra_359`: −2.10 (both solved; exp slower)

Interpretation: exp is slightly slower on shared easy algebra (chat-model timing noise) but picks up 1 extra hard problem (imo_1964_p2). Paired Δ CI crosses 0 by a wide margin — statistically indistinguishable at N=20.

## § 4. Gate verdict

### § 4.1 Applying DECISION_TREE § 4.1 criteria (pre-registered)

**PASS criteria** (both required):
1. Paired ΔPPUT CI does NOT fully lie below −0.05 ✓ (CI upper bound +0.301 > −0.05)
2. ΣPPUT_exp ≥ 0.90 × ΣPPUT_main ✓ (38.61 / 41.00 = 94.2% ≥ 90%)

**VERDICT per pre-reg**: **PASS**. Merge path unblocked, Phase 9.A baseline can proceed with Phase-8 binary.

### § 4.2 Analyzer script discrepancy

`phase2_ab_analyze.py` printed `FAIL: Mean PPUT CI lower 3.61 < 90% of baseline mean 7.38`. This criterion (Mean PPUT CI lower ≥ 90% baseline mean) is **NOT in DECISION_TREE § 4.1** — it's a legacy hardcoded rule from before the tree was formally revised. Pre-reg criteria in the tree take precedence.

**Action**: file C-068-b as doc-level follow-up — align analyzer script with DECISION_TREE before seed 2 run.

## § 5. Measurement caveat (corrected 2026-04-22): native_decide forbidden-pattern rejection

**Initial (wrong) diagnosis**: "fence leak" based on sub-3s FAILs with no oracle reject warn.

**Actual root cause**: chat's go-to tactic for numeric problems is `native_decide` (Lean bytecode brute-force). Per F-2026-04-20-05 + C-011 + C-050, `native_decide` and bare `decide` are **correctly forbidden** (they trivially discharge arithmetic goals without mathematical content). Oracle's `check_payload` catches this pre-Lean and emits `[oracle] payload rejected pre-Lean: Forbidden pattern: 'native_decide'` warn. Earlier grep missed this because I searched for "oracle reject" string literal.

**Actual failure-mode breakdown**:

| Outcome | Main (baseline) | Exp (Phase 8) |
|---|---|---|
| OMEGA ACCEPTED | 5 | **6** |
| Oracle reject (real Lean error) | 9 | 6 |
| Pre-Lean `native_decide`/`decide` | 6 | 8 |
| **Σ** | **20** | **20** |

Across 40 attempts: 12× `native_decide`, 2× `decide` bare. All correctly forbidden.

**Solve rate among judge-able (non-forbidden) attempts**:
- Main: 5/14 = 35.7%
- Exp: **6/12 = 50.0%**

Exp's ratio is notably higher, suggesting Phase 8 changes either produce better tactic choices OR chat coincidentally tried `native_decide` more often on exp (low N, hard to say).

**Constitutional implication**: this is not harness noise — it's chat's preferred oneshot tactic being correctly blocked. Options:
- **Accept as reality**: oneshot chat on numeric problems has a ~35% floor-loss to forbidden patterns. Swarm (n1/n3) mode likely recovers because agents can try alternate tactics.
- **Soft prompt hint**: "Use norm_num/omega/linarith/Decidable.decide; do NOT use bare `decide` or `native_decide` — they are forbidden per constitutional rule." **Risk**: C-031/C-034 hierarchy (机制 > 参数 > 提示); also chat might over-avoid legitimate `decide` uses.
- **Relax forbidden list**: NO — this is a Paper 1 bypass; F-2026-04-20-05 was explicit about preventing bytecode free-wins.

**Decision**: accept for now. Swarm-mode tests (Phase 9.B-D) should demonstrate recovery.

## § 6. Option B executed (revised post-correction)

**User approved Option B 2026-04-22**. Revised plan after § 5 correction:

- ~~Fix fence leak~~ **SKIP** — not a real issue; silent rejects were forbidden-pattern rejections with proper warn logs
- **Align analyzer script** to DECISION_TREE § 4.1 criteria (legacy Mean-PPUT-CI check removed)
- **Defer soft prompt hint** about `native_decide` — C-031 constraint + likely hurts swarm mode
- **Run seed 2** (31415) to confirm gate PASS is not seed-dependent

**Prep checklist**:
- [x] Correct verdict diagnosis (this doc revision)
- [ ] Align `phase2_ab_analyze.py` gate criteria to DECISION_TREE § 4.1
- [ ] Launch seed 31415 chat A/B on same sample (no binary rebuild needed; same prompt)
- [ ] Run aligned analyzer + write Phase 2.5c-seed2 verdict
