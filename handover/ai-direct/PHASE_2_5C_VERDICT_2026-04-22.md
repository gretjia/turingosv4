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

## § 5. Measurement caveat: fence-leak common-mode

Both conditions exhibited identical sub-3s silent FAILs on the same 5 problems:
- `mathd_algebra_208` (1-2 s, 1 tx, no oracle reject warn)
- `mathd_numbertheory_235 / _254 / _345 / _447`

Per err log inspection, these entered via the Rule 22 v2 clause 4 silent-reject path — hardened prompt eliminated ~75% of fence responses but ~25% still leak through in this sample. Common-mode pollution **does NOT bias the paired Δ** (both sides affected identically) but reduces effective N from 20 to 15.

**Action** (C-068-a, already filed as F-2026-04-22-08 follow-up):
- Add `warn!` log on fence reject path (fix silent-reject debt)
- Consider 2-shot retry pattern: if first response has fence, resubmit with even stronger instruction (still Rule 22 v2 compliant: reject-only, no byte mod)
- Before seed 2 run, aim for < 5% silent-reject rate

## § 6. Recommended next step

### Option A: Pre-reg strict — proceed now (default)
Per DECISION_TREE § 4.1 PASS, immediately:
1. Update DECISIONS_2026-04-22.md with PASS verdict
2. Launch Phase 9.A baseline (N=50 × 6 seeds on current exp binary)
3. Fix fence-leak + analyzer discrepancy in parallel (before 9.B)

**Pro**: fastest path to Paper 1 preprint
**Con**: N=20 with 25% fence-leak contamination → low statistical power; seed 2 re-run would boost confidence

### Option B: Conservative — one seed 2 confirmation first (recommended)
Per DECISION_TREE Step 2 "optional confidence-building seed":
1. Fix fence-leak ≥ 95% (1 session, ~30 min code + smoke)
2. Fix analyzer script (10 min)
3. Run Phase 2.5c-seed2 (chat A/B on seed 31415, ~60 min compute, ~$2)
4. If seed 2 also PASS → high-confidence gate → Phase 9.A
5. If seed 2 fails or INCONCLUSIVE → file F-finding, decide

**Pro**: confirms not seed-dependent before burning $30 on 9.A baseline
**Con**: +1 hour elapsed

### Option C: Paranoid — re-run same seed post fence-fix
Purely artifact-elimination re-run on seed 74677 after fence-leak fix.

**Pro**: directly isolates fence-leak impact
**Con**: not seed-diverse, adds little info over option B; $2 + 60 min

## § 7. Recommendation

**Option B**. One seed of confirmation + artifact fix is cheap ($2, 1 hour) and addresses two known issues:
- Under-powered N=20 with fence contamination
- Analyzer script misalignment with DECISION_TREE

Both issues would come up again at 9.B onwards. Fixing them now is strictly cheaper than later.

---

**Prep checklist for Option B (if approved)**:
- [ ] Enhance oneshot prompt v3 (stronger no-fence) + add `warn!` on reject path
- [ ] Align `phase2_ab_analyze.py` gate criteria to DECISION_TREE § 4.1
- [ ] Build main + exp binaries with fix
- [ ] Launch seed 31415 chat A/B on same sample
- [ ] Run aligned analyzer + write Phase 2.5c-seed2 verdict
