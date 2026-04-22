# Phase 2 A/B + Phase 9 Research Pivot — External Audit Brief

**Date**: 2026-04-22
**Status**: Gate 8→9 mechanically FAIL; user pivoting research direction
**Ask**: 2 questions — (A) is A/B verdict sound? (B) is Hayek-market pivot sound?

---

## § 0. Context (for audit)

TuringOS v4 Phase 8 (R1-α final) completed 3 rounds of dual audit (your
prior reports: EXT_CODEX_PHASE_8_R1ALPHA_2026-04-22.md,
EXT_GEMINI_PHASE_8_R1ALPHA_2026-04-22.md — both PASS on R1-α). Phase 2 A/B
N=20 oneshot just finished; Gate 8→9 decision pending.

---

## § 1. Phase 2 A/B results (clean-audit verified)

**Sample**: `sample_N20_S74677.txt` (fingerprint `8d390ee4eef82dbb`), seed 74677
**Conditions**: oneshot, deepseek-reasoner (NOTE: pre-registration said chat; deviation)
**Execution**: parallel batches on same machine

| | Baseline (main, pre-Phase-8) | Experiment (Phase 8 R1-α) |
|---|---|---|
| Solved | **8/20** (Wilson CI 0.22–0.61) | **7/20** (0.18–0.57) |
| ΣPPUT | **9.163** | **6.917** (Δ -24.5%) |
| Mean PPUT (solved) | 1.145 (CI 0.47–1.83) | 0.988 (CI 0.58–1.40) |
| Max depth | 1 | 1 |
| Σdepth≥10 PPUT | 0 | 0 |

**Paired view (same-problem Δ PPUT)**:
- 7 both solved, 1 only baseline (imo_1981_p6), 0 only experiment, 12 neither
- **Paired Δ PPUT mean = -0.112, CI95 = [-0.300, +0.075] → crosses 0**
- 83% of -2.246 total gap driven by **2 outliers**:
  - mathd_algebra_359: main 2.94 → exp 1.08 (Δ -1.86); timing 34s vs 92s
  - mathd_algebra_160: main 2.29 → exp 1.89 (Δ -0.39)
- Other 5 both-solved: exp faster on 4, slower on 1 (mathd_algebra_332)
- The 1 only-baseline (imo_1981_p6): main 470s solve, exp 517s timeout (47s gap, near cliff)

**Gate criterion (pre-registered in PLAN_FINAL § 4 Gate 8→9)**:
- `ΣPPUT ≥ 90% baseline` (9.16 × 0.9 = 8.25) → exp 6.92 < 8.25 ❌
- `Mean PPUT CI lower ≥ 90% baseline mean` (1.03) → 0.58 < 1.03 ❌

**Strict verdict: FAIL.** But paired CI crosses 0 → **not statistically significant**.

Full raw jsonl + per-problem table: `handover/ai-direct/EXT_AUDIT_PHASE_2_AB_2026-04-22/per_problem.tsv`

---

## § 2. User's research-direction pivot

**User (2026-04-22)**:
> "在完全匹配了宪法和宪法约定的 topology 都齐全的情况下，更多是通过市场机制的对齐（哈耶克、奥地利经济学派为主，区块链经济为参考）来激发智能涌现，更高的 PPUT"

Proposal: **Phase 9 shifts from multi-seed baseline → Market Mechanism Bake-off**.

Rationale:
1. Phase 7 tried tape-economy v1 (fee=500) and v2 (fee=2000) both failed (F-2026-04-20-04): `append: 0` despite fee. Post-hoc (2026-04-22 audit): C-049 `bus.snapshot()` hardcoded empty balances → **agent never saw Coin signals**. Hypothesis: **v1/v2 failed because IO bug severed signal, not because mechanism was wrong**. Post-Phase-8 (C-049 fixed), the same mechanism might work.
2. Phase 8 completes the **constitutional substrate**: topology (Art. IV), capability (Ed25519), signals visible (C-049), governance skeleton (Art. V). Next frontier: incentive structure.
3. Paper 1 thesis upgrades from "we built the constitution" to **"constitutional topology enables Hayekian market mechanisms that drive emergent faster proof discovery"**.

Proposed 9.M agenda:
- **9.M.1** rerun tape-economy v1 (fee=500) in post-Phase-8 — test C-049 root-cause hypothesis ($60, ~4h)
- **9.M.2** A/B 4 market mechanisms each N=20 paired vs Phase 8 baseline:
  - M1 dynamic founder grant (γ tied to tape productivity)
  - M4 Satoshi citation rebate (Phase 3B branch exists, not merged)
  - M7 append staking (small ε Coin stake → refund on GP inclusion)
  - M8 bonding-curve LP (agent as liquidity provider role)
  - ($240, ~16h)
- **9.M.3** best-mechanism combination N=50 × 3 seeds ($90, ~24h)
- **Total budget**: ~$400, same as original Phase 9 baseline plan

---

## § 3. 必答 2 题

### Q1. Phase 2 A/B 判决

Given:
- Strict Gate criteria FAIL (ΣPPUT -24%, mean PPUT CI lower below threshold)
- Paired Δ CI crosses 0 (not statistically significant at N=20)
- 2 problem-level outliers drive 83% of the gap
- Parallel batch execution on same machine (cross-contamination risk)
- Model deviation from pre-registration (reasoner vs chat; consistent both sides)

**Should we**:
- **a.** Accept strict FAIL → block merge, re-debug Phase 8 (but no stat-significant regression to debug)
- **b.** Accept statistical "inconclusive" → re-run different seed to test variance (another $30, 4h)
- **c.** Declare A/B scope-inappropriate (oneshot doesn't exercise Phase 8 changes; swarm is where Phase 8 matters) → run swarm A/B instead ($50, 4-6h)
- **d.** Accept that oneshot regression is noise + pivot directly to Phase 9.M market bake-off (skips explicit gate closure but reallocates budget to research program)

Pick one + justify. Special attention: if you VETO a path, explain the scientific failure mode.

### Q2. Hayek / Austrian market pivot soundness

Evaluate:
- Is the C-049 root-cause hypothesis for F-20-04 (tape-economy v1/v2 failure) defensible?
- Are Hayek/Austrian principles actually operationalizable as mechanism changes in Rust code, or is this theoretical hand-waving?
- Is the 9.M mechanism menu well-chosen? Missing any canonical blockchain mechanism (e.g., quadratic funding, conviction voting, retroactive funding)?
- Does the paper-1 thesis upgrade ("constitutional topology enables market mechanism discovery") over-claim given only single paper's data?

---

## § 4. Files to consult

- `handover/ai-direct/EXT_AUDIT_PHASE_2_AB_2026-04-22/per_problem.tsv` (full paired view)
- `handover/ai-direct/PLAN_FINAL_PHASE_8_TO_PAPER_2026-04-22.md` (current plan, Phase 9 pre-pivot)
- `handover/ai-direct/REGISTRATION_PHASE_9_2026-04-22.md` (pre-reg to compare pivot against)
- `handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md` (F-2026-04-20-04 tape economy v1/v2 failure + C-049 root cause)
- `handover/audits/PPUT_RAW_DATA_2026-04-22.md` (historical baselines)
- `constitution.md` Art. II.2, Art. II.2.1, Art. I.2 (Hayek-relevant articles)
- Raw jsonls:
  - `experiments/minif2f_v4/logs/phase8_baseline_main_oneshot_20260422T122117.jsonl`
  - `.claude/worktrees/phase-8a-snapshot/experiments/minif2f_v4/logs/phase8_experiment_oneshot_20260422T122119.jsonl`

---

## § 5. Output

**Length**: short, direct. Verdict per question + 2-sentence justification.

**Save to**:
- Codex: `/home/zephryj/projects/turingosv4/handover/audits/EXT_CODEX_PHASE_2_AB_2026-04-22.md`
- Gemini: `/home/zephryj/projects/turingosv4/handover/audits/EXT_GEMINI_PHASE_2_AB_2026-04-22.md`

C-066: `ls -la` verify. Codex sandbox block → paste inline.
