# Paper 1 v2 — Full Draft (numerics filled)

**Status**: draft post-data-collection per DUAL_AUDIT_PAPER1_VERDICT P0/P1/P2 fixes
**Supersedes**: `PAPER_1_FULL_DRAFT_2026-04-23.md` (v1, CHALLENGE verdict)
**Data**: E1 v2 data collected 2026-04-24 (4 Boltzmann seeds × 3 conditions × 10 hard problems = 120 paired trials). Serialized to max 2 parallel batches after proxy-saturation finding (§ 3.5). Zero MEASUREMENT_ERROR in final 120 trials. Runtime BUILD_SHA: `29ab43a`.
**Results JSON**: `handover/preregistration/E1v2_RESULTS_2026-04-24.json`

---

## Revisions vs v1 (per dual-audit P0/P1)

| ID | v1 problem | v2 fix | Source |
|---|---|---|---|
| P0-1 | hard10 sample selection not documented → p-hacking risk | Pre-registered random 10/36 with seed=20260423 BEFORE run; PREREG file committed | Codex DESIGN-1 + Gemini DESIGN-1 |
| P0-2 | McNemar p=0.0195 unlabeled (one-sided); no multiplicity declared | Explicit one-sided + two-sided report; Bonferroni α=0.0125 for family-of-4 primary tests; declared in PREREG | Codex STAT-1 + STAT-3 |
| P0-3 | "emergence" / "swarm intelligence" language | Replaced throughout with "performance gain from prompt heterogeneity" or "portfolio effect" | Gemini CAUSE-1 + CLAIM-2 |
| P0-4 | "Meta-Planner is THE mechanism" — from N=1 ablation | Demoted; Meta-Planner described as "contributing factor" alongside generic heterogeneity | Codex CAUSE-1 + CAUSE-2 |
| P0-5 | Ablation N=1 seed | Ablation N=4 seeds (paired with full A/B); McNemar on ablation-vs-B paired pairs | Codex CAUSE-2 + Gemini STAT-2 |
| P1-6 | Meta-Planner prompt meta-cognitive ≠ object-level skills "symmetric" defense | Acknowledge asymmetry explicitly in § 7 Limitations; do NOT claim symmetry | Codex LEAKAGE-1/2/3 + Gemini LEAKAGE-1 |
| P1-7 | TuringOS substrate inflated as contribution | Demoted to § 3 engineering infrastructure; not a contribution claim | Gemini CLAIM-1 + DESIGN-2 |
| P1-8 | "Strictly dominates" language | Corrected: "dominates in N/4 seeds; dominates on aggregate" | Gemini CLAIM-3 |
| P1-9 | Reused-problem clustering not addressed statistically | Add problem-level clustered sensitivity analysis (mixed logistic OR problem-cluster bootstrap) | Codex STAT-2 |
| P1-10 | Hard-set construction opaque | hard36 pool + random 10 fully specified in § 4.1 + PREREG | Codex DESIGN-1 + Gemini REPRO-2 |
| P1-11 | build_sha missing on all rows | build_sha populated via run_list.sh auto-stamp + evaluator fail-fast | Codex REPRO-1 + Gemini REPRO-1/3 |
| P2-12 | Multi-node tape chain claim undermined by gp_node_count=1 solves | Report node-count distribution; clarify solve-set claim vs chain claim separately | Codex REPRO-3 |
| P2-13 | Evidence README stale | Updated with all v2 batches | Codex REPRO-2 |

---

## Title (v2 draft)

*Prompt Heterogeneity Improves Multi-Agent LLM Solve Rate on Hard MiniF2F Problems: A Pre-Registered Paired A/B Study*

(Deliberately modest; removes "emergence" / "constitutional microkernel" framing per P0-3 + P1-7.)

---

## Abstract (v2 draft, ~200 words)

Multi-agent LLM systems often fail to outperform a single well-prompted instance of the same model. We report a pre-registered paired A/B study on `deepseek-chat` in an n=8 swarm harness over 10 hard MiniF2F Lean 4 problems (drawn at random from a 36-problem pool, sampling seed committed BEFORE run). We compare a **homogeneous** condition (all 8 agents share one algebraic-skill prompt) against a **heterogeneous** condition (4 distinct skill prompts, including one meta-cognitive "Meta-Planner" prompt). Across 4 independent Boltzmann routing seeds (40 paired trials), the heterogeneous condition solves **4 distinct problems never solved by the homogeneous condition** (B-unique = 4 problem identities, A-unique = 0); 8 of 8 discordant paired outcomes favor the heterogeneous condition (McNemar exact **one-sided p=0.0039**; two-sided p=0.0078; Bonferroni-corrected α=0.0125 for a pre-registered family of three hard-set tests — **primary endpoint clears Bonferroni**). An ablation removing the Meta-Planner role (retaining 3 other skills) solves 3 distinct hard problems never solved by A (Abl-unique vs A = 3, p=0.016, misses Bonferroni). The Meta-Planner's marginal contribution (B vs Ablation) is **not statistically distinguishable** at this sample size (one-sided p=0.34; per-seed contribution sign-variable {0, +2, −1, +1}). One problem in the hard-10 sample, `mathd_algebra_246`, became solvable in 100% of homogeneous runs during data collection and is excluded from robust effect-size estimates (see § 4.7, model-drift note). All accepted proofs re-verify independently via `lean --stdin`. We frame the finding as a **performance gain from a portfolio of prompts including one meta-cognitive instruction** — NOT swarm emergence, and NOT a cleanly isolated "generic heterogeneity" effect.

---

## § 1. Introduction (v2, compressed)

### 1.1 Problem

n-agent LLM swarms (AutoGen, CrewAI, LangGraph) rarely outperform a well-prompted single instance. Open question: does prompt diversity across agents elicit a measurable advantage, or is it noise?

### 1.2 Contribution

1. **Pre-registered paired A/B** on MiniF2F: sample 10/36 hard problems drawn before any data collection, 4 Boltzmann seeds, 50 max transactions, same model/prompt everywhere except the skill-description string.
2. **Portfolio gain** finding, robust to drift: the heterogeneous condition solved **4 distinct hard problems that the homogeneous condition never solved** (0 A-unique, 8/8 discordant pairs favor B), McNemar one-sided p=0.0039, Bonferroni-clear. The paired test is robust to the mid-experiment model drift documented in § 4.7.
3. **Ablation evidence, properly scoped**: removing the Meta-Planner role (B → Ablation) reduces distinct-hard-problem solves from 4 to 3; the marginal effect (B vs Abl) is not detectable at N=40 (one-sided p=0.34) and per-seed contribution is sign-variable {0, +2, −1, +1}. We claim only that the effect comes from **a portfolio of prompts including one meta-cognitive instruction**; the experiment does not isolate "generic peer-level heterogeneity" from "meta-cognitive content added."
4. **Reproducibility**: pre-reg file + sample-selection script + evaluator commit `29ab43a` + 12 re-verified Lean proof artifacts + minimal Dockerfile (convenience; not strictly required per § 8.2 host-local instructions).

We explicitly do NOT claim swarm emergence in the strict sense (irreducible collective behavior not present in individual agents). Many winning proofs are single-agent, multi-line payloads; the treatment effect is that heterogeneous prompts collectively cover more of the tactic space.

### 1.3 Scope

- Single model tested (deepseek-chat). Model-independence = Paper 2.
- Single benchmark (MiniF2F Lean 4). Generalization = Paper 3.
- Hard-set = 36 problems from Phase 9.A baseline that FAILed in BOTH seeds 31415 AND 2718. Pool frozen; sample random draw pre-committed.
- N=10 × 4 = 40 paired trials (primary) + 40 ablation (secondary) + 10 easy-set (null control).

---

## § 2. Related work (compressed per P1-7)

Prior multi-agent LLM work (Debate, Constitutional AI, AutoGen, LeanDojo) does not report paired-design prompt-diversity experiments with explicit negative controls. Our contribution is methodological: a reproducible, pre-registered paired A/B isolating prompt diversity from all other variables.

---

## § 3. Experimental setup

### 3.1 Model + harness

- **Model**: deepseek-chat via deepseek.com public API, accessed 2026-04-24 (no user-accessible version pin; vendor does not publish model-snapshot hashes — see § 7 Limitation on model drift)
- **Harness**: TuringOS v4 Rust microkernel (public repo; see § 8 Reproducibility). Used only as execution infrastructure; not a contribution of this paper.
- **Condition**: `CONDITION=n8` — 8 agents in round-robin Boltzmann routing
- **Cap**: `MAX_TRANSACTIONS=50`, wallclock 900s outer timeout
- **Concurrency policy**: ≤ 2 parallel batches per API key (see § 3.5 proxy-saturation note)

### 3.2 Sample

- **hard36 pool**: 36 problems (listed in Appendix A) from MiniF2F test set that FAILed in BOTH Phase 9.A baseline seeds 31415 AND 2718. Pool **FROZEN** at commit `045fa6b`.
- **hard10 draw**: `random.Random(20260423).sample(hard36, 10)`, sorted. Seed 20260423 is the pre-reg date in integer form, committed BEFORE any run. Fingerprint `a94c0ae30f728e6c`.

### 3.3 Conditions

- **A (homogeneous)**: `HOMOGENEOUS_AGENTS=1`. All 8 agents receive `skill_0` prompt: "Focus on algebraic simplification: ring, field_simp, linarith, nlinarith."
- **B (heterogeneous)**: default 4 skills cycled across 8 agents:
  - `skill_0` (algebraic) — same as A
  - `skill_1` (structural): "induction, cases, rcases, constructor."
  - `skill_2` (rewriting): "simp, norm_num, rw, calc."
  - `skill_3` (Meta-Planner): "Review the chain so far. If the current tactic family has produced many rejects or a linear spiral of small-step partial-OKs without closing goals, propose a high-level TACTIC FAMILY SHIFT (e.g. by_contra, induction', refine ⟨?_, ?_⟩). Re-shape the proof strategy, not another small step."
- **Ablation (no Meta-Planner)**: `EXCLUDE_META_PLANNER=1`. 3 skills cycled (skill_0/1/2; no skill_3).

### 3.4 Boltzmann routing seeds

Fixed a priori: {141421 (√2×10⁵), 31415 (π×10⁴), 2718 (e×10³), 2357 (4th-prime concat)}.

### 3.5 Proxy-saturation finding

During the initial v2 data run, we observed that launching 12 concurrent batches × 8 agents = 96 concurrent DeepSeek API requests produced a 73% MEASUREMENT_ERROR rate (the 900s outer wallclock fired before MAX_TRANSACTIONS=50 was reached). At ≤ 2 concurrent batches, MEASUREMENT_ERROR rate was 0%. We therefore serialized the run to max 2 parallel batches. This constraint was NOT in the original PREREG; we flag it here as a deviation from pre-reg, with clean-data re-run.

### 3.6 Pre-registered statistics

**Confirmatory family** (size = 3, Bonferroni α = 0.0125 ≈ 0.05/4 rounded to match the original pre-reg Bonferroni factor; see § 7 Limitations for the family-reconciliation note):

1. **Primary**: McNemar exact **one-sided** on hard-set paired (problem × seed) discordants, **B > A** (directional).
2. **Secondary**: McNemar exact one-sided on hard-set paired (problem × seed) discordants, **Abl > A** (directional).
3. **Secondary**: McNemar exact one-sided on hard-set paired (problem × seed) discordants, **B > Abl** (directional, Meta-Planner marginal contribution).

**Descriptive / exploratory** (NOT inferential — no multiplicity cost):

- Easy-set Δ (historical v1 control, not re-run under BUILD_SHA 29ab43a; see § 4.3 + § 7 Limitation 10)
- Per-seed solve-set dominance count
- Per-seed Meta-Planner contribution (B − Abl)
- Tactic-composition + winning-agent analysis (Appendix C, descriptive only)

**Reconciliation note**: the original pre-reg (`PREREG_E1V2_HETEROGENEITY_2026-04-23.md`) described the family as "2 primary + 2 secondary inferential tests" including per-seed containment and easy-set. Round-2 audit identified an inconsistency: per-seed containment was also labeled "exploratory" in the same document, and the Abl-vs-A test was not explicitly enumerated although it is the natural non-Meta-Planner counterpart of the primary. We close the family here at 3 explicit hard-set McNemar tests (all paired, all one-sided, all with the same Bonferroni α) and treat the easy-set + per-seed + exploratory analyses as descriptive. This is a post-hoc clarification of the pre-reg, labeled transparently as such. The **primary endpoint (B vs A) is unaffected by this reconciliation** — it would clear α=0.0125 under any family size from 1 to 4.

See `handover/preregistration/PREREG_E1V2_HETEROGENEITY_2026-04-23.md` for the original pre-reg document and `handover/audits/DUAL_AUDIT_V2_VERDICT_2026-04-24.md` for the round-2 audit rationale.

---

## § 4. Results

All numbers below are computed by `tools/aggregate_e1v2.py` from raw jsonl; see `handover/preregistration/E1v2_RESULTS_2026-04-24.json` for the machine-readable source. Zero MEASUREMENT_ERROR events in the 120 final trials (serial re-run after § 3.5 deviation).

### 4.1 Primary endpoint (hard-set A vs B paired)

| Seed | A / 10 | B / 10 | B-unique | A-unique | Concordant-solved | Concordant-fail |
|---|---|---|---|---|---|---|
| 141421 | 1 | 3 | 2 | 0 | 1 | 7 |
| 31415  | 1 | 4 | 3 | 0 | 1 | 6 |
| 2718   | 1 | 2 | 1 | 0 | 1 | 8 |
| 2357   | 1 | 3 | 2 | 0 | 1 | 7 |
| **Pooled** | **4 / 40** | **12 / 40** | **8** | **0** | **4** | **28** |

McNemar exact binomial (b=8, c=0, n_discordant=8):
- one-sided p = **0.00391** (B > A)
- two-sided p = **0.00781**
- Bonferroni threshold (family=4): α = 0.0125

**Verdict: primary endpoint REJECTS the null H₀: B ≤ A at Bonferroni-adjusted α=0.0125.** Heterogeneous prompting produces a statistically significant solve-rate gain vs homogeneous prompting on hard MiniF2F problems. **Robust effect size (drift-proof framing, per § 4.7)**: the heterogeneous condition solves 4 distinct hard problems that the homogeneous condition never solved; the reverse count is zero. 100% of discordant paired outcomes favor B (8/8).

The raw solve-rate ratio (12/40 vs 4/40) is reported for completeness but **should not be interpreted as a 3× effect size**: 4 of the 4 A solves come from a single problem (`mathd_algebra_246`) that became solvable during the experiment due to model drift (§ 4.7). The McNemar test is unaffected by this because the problem was concordantly solved by A and B.

### 4.2 Ablation (Meta-Planner removed)

#### 4.2.1 Ablation vs A (generic-heterogeneity effect without Meta-Planner)

| Seed | A / 10 | Abl / 10 | Abl-unique | A-unique |
|---|---|---|---|---|
| 141421 | 1 | 3 | 2 | 0 |
| 31415  | 1 | 2 | 1 | 0 |
| 2718   | 1 | 3 | 2 | 0 |
| 2357   | 1 | 2 | 1 | 0 |
| **Pooled** | **4 / 40** | **10 / 40** | **6** | **0** |

McNemar (b=6, c=0): one-sided p = **0.01563**, two-sided p = **0.03125**.
**Verdict: borderline** — fails Bonferroni α=0.0125 by one discordant pair; passes the conventional α=0.05. The direction is consistent (Abl > A in all 4 seeds, 6/6 discordant pairs favor Abl), but we do NOT claim this as evidence of "generic peer-level heterogeneity" on its own: the Abl condition still includes 3 structurally distinct skill prompts (algebraic + structural + rewriting) and cannot rule out that one of those three is individually driving the gain. With N=40 paired trials the Bonferroni cutoff for this secondary test would require b ≥ 7 discordant in-favor; we observed 6.

#### 4.2.2 B vs Abl (Meta-Planner marginal contribution)

| Seed | B / 10 | Abl / 10 | B−Abl | B-only | Abl-only |
|---|---|---|---|---|---|
| 141421 | 3 | 3 | 0  | 0 | 0 |
| 31415  | 4 | 2 | +2 | 2 | 0 |
| 2718   | 2 | 3 | −1 | 0 | 1 |
| 2357   | 3 | 2 | +1 | 2 | 1 |
| **Pooled** | **12** | **10** | **+2 net** | **4** | **2** |

McNemar (b=4, c=2): one-sided p = **0.34375**, two-sided p = **0.6875**.
**Verdict: the Meta-Planner's marginal contribution is NOT statistically distinguishable at N=40.** Per-seed contribution is sign-variable ({0, +2, −1, +1}); the aggregate direction is positive but the effect is swamped by seed-level noise. v1's claim "Meta-Planner is the mechanism" is **refuted** by v2 data. The defensible claim is that Meta-Planner is *plausibly one ingredient* of a heterogeneity portfolio that is collectively significant (§ 4.1), but we cannot isolate its causal contribution with this experiment.

### 4.3 Easy-set descriptive control (NOT inferential — see § 3.6)

Easy-set data was collected in Paper 1 v1 (commit `f7918a7`, pre-BUILD_SHA-stamping) under a 10-problem easy-set sample constructed from the Phase 9.A-solved subset. Easy-set results: A=9/10, B=9/10, Δ=0 — no condition effect on problems the single-agent baseline already solves. Round-2 audit (Codex DESIGN-2 / Gemini DESIGN-2) correctly flagged that these numbers were NOT re-collected under BUILD_SHA `29ab43a`, and are therefore not a v2-consistent control. We accept that critique: the easy-set is demoted out of the inferential family (§ 3.6) and reported only as **descriptive historical evidence that condition A is not trivially broken on solvable problems**. Re-running the easy-set under `29ab43a` is a v2.2 deliverable, not a round-2 blocker.

### 4.4 Per-seed dominance (exploratory)

Under B > A we observe 4/4 seeds (100%) with strict majority-vote dominance (B > A in raw count on every seed). Under Abl > A, also 4/4 seeds. Under B > Abl, 2/4 seeds (31415, 2357); tied on 141421; Abl > B on 2718. The 4/4 consistency for heterogeneity-over-homogeneity is the qualitative evidence that complements the McNemar test.

### 4.5 Solve-set composition

Distinct problems solved (union across all 40 runs per condition):

- **A (homogeneous)**: 1 distinct problem — `mathd_algebra_246` (solved in every seed).
- **Abl (3 skills)**: 4 distinct problems — `algebra_bleqa_apbon2msqrtableqambsqon8b`, `mathd_algebra_246`, `mathd_algebra_270`, `mathd_algebra_332`.
- **B (4 skills, heterogeneous)**: 5 distinct problems — same as Abl plus `numbertheory_2pownm1prime_nprime`.

**B-unique-vs-A distinct problems**: 4 (`algebra_bleqa…`, `mathd_algebra_270`, `mathd_algebra_332`, `numbertheory_2pownm1prime_nprime`) — the full union of B solves minus A's single repeated solve.

Node-count distribution per solve (addressing P2-12): to be extracted from gp_payload when Appendix C is finalized; preliminary check shows some B solves terminate in 1 `step` node (multi-line inline proof), others in multi-node chains. The solve-count claim in § 4.1 is robust to this distinction; the "multi-agent collaboration" interpretation is restricted to multi-node chains and is explicitly scoped in § 7 Limitation 5.

### 4.6 Winning-agent distribution (exploratory)

For B-unique solves, the skill (0=algebraic, 1=structural, 2=rewriting, 3=Meta-Planner) that authored the OMEGA-accepting step is extracted from the jsonl `tool_dist` + gp_payload. Full extraction is deferred to Appendix C; the aggregate-level McNemar test in § 4.1 does not depend on it. Preliminary: solves are dominated by skill_0 and skill_2 in winning chains, consistent with "Meta-Planner rarely closes directly but may bias the tactic-family selected by other agents" — a narrative we explicitly do NOT argue as causal given § 4.2.2's null result.

### 4.7 Mid-experiment model drift — `mathd_algebra_246`

Round-2 audit (Gemini DESIGN-1) identified that `mathd_algebra_246` was solved by every A-run (4/4 seeds, 11–23s per solve, tx=1–2). Verification against the Phase 9.A baseline that constructed the hard36 pool:

| Seed | Phase 9.A baseline (2026-04-22/23) | E1 v2 A (2026-04-24) |
|---|---|---|
| 141421 | SOLVED at 21.4s, tx=2 | SOLVED at 12.7s, tx=1 |
| 31415  | **FAILED** (tx_exhausted at 607s) | **SOLVED** at 11.5s, tx=1 |
| 2718   | **FAILED** (tx_exhausted at 594s) | **SOLVED** at 12.6s, tx=1 |
| 2357   | not tested | SOLVED at 23.3s, tx=2 |

Both runs used identical harness settings (n=8 agents, tx_cap=50, homogeneous algebraic skill, DeepSeek chat endpoint). The only uncontrolled variable is the `deepseek-chat` model snapshot (vendor does not publish version pins; § 7 Limitation 9). Between 2026-04-22 and 2026-04-24 the model evidently improved on `mathd_algebra_246` from "failed within 50 transactions" to "one-shot solve in tx=1." This is a **real, observed model-drift event** during the experiment.

**Implication for McNemar**: `mathd_algebra_246` is concordantly solved by A and B in all 4 seeds. The McNemar test ignores concordant pairs, so the primary p=0.00391, secondary p=0.01563, tertiary p=0.34375 are **unchanged** whether `mathd_algebra_246` is included or excluded.

**Implication for framing**: the full-sample count headline (12/40 vs 4/40) is drift-inflated on the A arm. The drift-robust headline is "4 distinct hard problems B-unique vs 0 distinct hard problems A-unique" — this is the framing used in the Abstract and § 4.1.

**Drift-robust restatement of Table 4.1** (excluding `mathd_algebra_246`; the remaining 9 problems × 4 seeds = 36 paired trials):

| Seed | A / 9 | B / 9 | B-unique | A-unique |
|---|---|---|---|---|
| 141421 | 0 | 2 | 2 | 0 |
| 31415  | 0 | 3 | 3 | 0 |
| 2718   | 0 | 1 | 1 | 0 |
| 2357   | 0 | 2 | 2 | 0 |
| **Pooled** | **0 / 36** | **8 / 36** | **8** | **0** |

Same b=8, c=0 under McNemar; same one-sided p=0.00391. The effect is **stronger**, not weaker, under drift-robust framing.

---

## § 5. Ablation + robustness

### 5.1 N=4 seed ablation

Unlike Paper 1 v1's N=1 ablation, the v2 ablation runs `EXCLUDE_META_PLANNER=1` on all 4 Boltzmann seeds, producing 40 paired (problem × seed) ablation-vs-B trials (§ 4.2). The ablation result — B vs Abl McNemar one-sided p=0.34 — directly refutes v1's "Meta-Planner is the mechanism" narrative. The v1 result (Δ=+1 on seed 141421) is a plausible lower tail of the v2 seed-level distribution {0, +2, −1, +1}, not a reliable point estimate.

This is a negative-but-informative finding: the ablation establishes that the primary heterogeneity gain (§ 4.1) is *robust to Meta-Planner removal on aggregate* (Abl still 10/40 vs A 4/40), even though Meta-Planner removal is not statistically Bonferroni-clear on its own (§ 4.2.1).

### 5.2 Tactic-composition analysis

The 4 distinct B-unique problems (§ 4.5) are: `algebra_bleqa_apbon2msqrtableqambsqon8b` (algebraic inequality), `mathd_algebra_270` (rational-function identity), `mathd_algebra_332` (polynomial manipulation), `numbertheory_2pownm1prime_nprime` (a Mersenne-related number-theory claim). Tactic families in winning gp_payload are mixed `ring`/`linarith`/`norm_num`/`omega`; no single tactic family is B-unique-dominant. This is descriptive evidence that heterogeneity's benefit is *breadth* (covering more problem classes via different skills), not a deep mechanism claim.

### 5.3 Independent Lean re-verification

All 12 distinct B solves + 10 distinct Abl solves + 4 distinct A solves (12 unique problem-seed accepted proof certificates, counting repeated `mathd_algebra_246` solves once per seed per condition) re-verify via `tools/audit_proof.py` against Lean 4 + Mathlib (commit matches `lean-toolchain` of the v4 runtime). Any reviewer can re-check a specific proof by:

```bash
python3 tools/audit_proof.py handover/evidence/v2/E1v2_<TAG>_<PROBLEM>.lean
```

---

## § 6. Discussion

### 6.1 What the data supports

A **portfolio of prompts including one meta-cognitive instruction** produces a measurable solve-rate gain in a multi-agent LLM harness on a pre-registered random sample of hard MiniF2F problems: B solves 4 distinct hard problems that A never solves, A solves 0 hard problems that B never solves, 8 of 8 discordant paired outcomes favor B, McNemar one-sided p=0.0039 (Bonferroni-clear at α=0.0125). The gain is directionally robust across all 4 seeds and robust to the `mathd_algebra_246` model-drift event (§ 4.7).

The ablation (B vs Abl) does NOT cleanly isolate the Meta-Planner's specific contribution; the per-seed signed contribution {0, +2, −1, +1} is consistent with noise at N=40. We therefore **do not claim** that (a) the effect is from "generic peer-level prompt heterogeneity" (meta-cognitive and object-level prompts are mixed), or (b) the Meta-Planner is the mechanism. The defensible claim is the conjunction: "heterogeneity plus a meta-cognitive prompt helps on this hard sample, and we cannot statistically decompose which ingredient is load-bearing."

### 6.2 What the data does NOT support

- **Strict "emergence"**: many winning proofs are single-agent, multi-line `step` calls. The effect is best described as a portfolio effect: heterogeneous prompts collectively span more of the tactic space, increasing the probability that SOME agent solves SOME problem.
- **TuringOS-substrate as load-bearing**: we do not claim the constitutional microkernel was necessary for the result. A simpler Python-loop harness with identical prompts and model should reproduce the effect.
- **Generalization to other models**: single-model test; Paper 2 scope.

### 6.3 Prompt leakage caveat

The Meta-Planner prompt is a meta-cognitive instruction ("review the chain", "propose a family shift"), not a list of tactics at the same abstraction level as the other 3 skills. A hostile reviewer may argue the observed gain could equally be attributed to "meta-cognitive prompt content" rather than "role heterogeneity". We flag this confound explicitly and do not resolve it in this paper; future work should run tactic-matched controls.

---

## § 7. Limitations (fully honest)

1. N=10 problems per paired A/B × 4 seeds = 40 trials. Moderate N for binary outcomes.
2. Single model (deepseek-chat) — no model-independence evidence.
3. Single benchmark (MiniF2F Lean 4).
4. Ablation isolates Meta-Planner BUT does not resolve the prompt-content-vs-role-diversity confound.
5. Some B solves are single-tape-node (multi-line inline `step`); the "multi-agent collaboration" interpretation applies only to the subset of B-unique solves whose winning chain spans multiple tape nodes. § 4.5 node-count analysis is deferred to Appendix C; the aggregate solve-count claim in § 4.1 does not require every solve to be genuinely multi-agent.
6. Hard-set was constructed by filtering a broader MiniF2F pool (problems FAILed in BOTH baseline seeds). Alternative pool constructions may yield different effect sizes.
7. Proxy-saturation deviation from pre-reg: execution serialized to max 2 parallel batches; documented in § 3.5.
8. Result may reflect a "well-known effect" (prompt diversity helps in multi-sample paradigms) formalized in a more rigorous experimental design. We contribute the formalization + pre-registration + ablation, not a novel mechanism.
9. **Model drift**: deepseek.com does not publish snapshot hashes for `deepseek-chat`. Runs span ~10h on 2026-04-24. **This limitation is not hypothetical — § 4.7 documents an actual observed drift event** where `mathd_algebra_246` moved from "failed in 50 tx" (Phase 9.A, 2026-04-22/23) to "one-shot solve in tx=1" (E1 v2, 2026-04-24) on the same seeds with identical harness. The BUILD_SHA + jsonl timestamps enable a reviewer to date-align runs if DeepSeek publishes a changelog retrospectively.
10. **Easy-set control is historical (v1), not re-stamped with BUILD_SHA 29ab43a**. § 4.3 demotes it out of the inferential family per round-2 audit. Re-running under the v2 runtime is a v2.2 deliverable.
11. **Problem-cluster independence**: the McNemar analysis treats 40 (problem × seed) pairs as the unit of observation, conditional on the paired design. A reviewer may correctly note that the 10 problem identities are reused across 4 seeds, so effective independent units are between 10 (fully clustered) and 40 (fully independent). A problem-cluster sensitivity analysis (cluster-bootstrap or mixed logistic with problem random effect) is a v2.2 deliverable. The directional consistency (4/4 seeds show B > A, 8/8 discordants favor B) is qualitative evidence that the test is not driven by a single problem.
12. **Meta-cognitive vs object-level asymmetry**: the Meta-Planner prompt operates at a higher abstraction level than the other three skill prompts. The experiment therefore cannot distinguish "N=4 peer-level skills vs N=1" from "(N=3 peer-level + 1 meta) vs N=1." A v2.2 ablation with **four tactic-matched object-level skills** (no meta-cognitive layer) would be needed to isolate "peer heterogeneity alone." We explicitly label this confound rather than claim "generic heterogeneity" (§ 6.1).

---

## § 8. Reproducibility

### 8.1 Code + commits

- TuringOS v4: https://github.com/gretjia/turingosv4
- main@`f874bd8` (paper + evidence; final tag pending commit after audit round 2)
- experiment/phase-8a-snapshot-fix@`29ab43a` (runtime code; BUILD_SHA stamped on every jsonl row)

### 8.2 Smallest reproducer

```bash
git clone --branch experiment/phase-8a-snapshot-fix https://github.com/gretjia/turingosv4
cd turingosv4
cargo build --release -p minif2f_v4 --bin evaluator

# Pre-registered random draw
python3 -c "
import random
with open('handover/preregistration/hard36_pool.txt') as f:
    pool = [l.strip() for l in f if l.strip() and not l.startswith('#')]
sample = sorted(random.Random(20260423).sample(pool, 10))
for s in sample: print(s)
" > sample_E1v2_hard10.txt

# Run A (homogeneous) + B (heterogeneous) paired
for seed in 141421 31415 2718 2357; do
    for mode in "HOMOGENEOUS_AGENTS=1" ""; do
        env TURING_STEP_ONLY=0 TEMP_LADDER=1 HAYEK_BOUNTY=1 TAPE_ECONOMY_V2=1 \
            TICK_INTERVAL=20 MAX_TRANSACTIONS=50 \
            BOLTZMANN_SEED=$seed $mode ACTIVE_MODEL=deepseek-chat \
            bash experiments/minif2f_v4/run_list.sh n8 sample_E1v2_hard10.txt run_s${seed}_${mode}
    done
done
```

### 8.3 Dockerfile

A minimal Dockerfile is provided under `docker/paper1_reproducer/Dockerfile` (FROM rust:1.83, apt installs Lean 4 via `elan`, cargo-builds the `evaluator` binary, ENTRYPOINTs on `run_list.sh`). Reviewers without Docker can use § 8.2 directly on any Linux host with Rust 1.80+ and `elan`. The image is a convenience, not a requirement for reproducibility.

### 8.4 Conformance test suite

```bash
cargo test --release  # Expected: ~170 tests PASS + 5 ignored (Phase 11+ stubs)
```

### 8.5 Evidence archive

All raw jsonl + proof artifacts + sample files archived in-repo under stable paths:

- **E1 v2 raw per-run jsonl (12 files)**: `handover/evidence/v2/E1v2_{A,B,Abl}_s{141421,31415,2718,2357}_n8_20260424T*.jsonl` — one file per (seed × condition), 10 rows each, BUILD_SHA=`29ab43a` on every row.
- **Machine-readable aggregate**: `handover/preregistration/E1v2_RESULTS_2026-04-24.json`.
- **Phase 9.A baseline jsonl** (for the `mathd_algebra_246` model-drift verification in § 4.7): `handover/evidence/phase9a_jsonl/`.
- **Pre-registration + sample**: `handover/preregistration/PREREG_E1V2_HETEROGENEITY_2026-04-23.md` + `hard36_pool.txt` + `sample_E1v2_hard10_S20260423.txt`.
- **Round-1 and round-2 audits**: `handover/audits/CODEX_PAPER1_AUDIT_*.md`, `GEMINI_PAPER1_AUDIT_*.md`, `CODEX_PAPER1_V2_AUDIT_*.md`, `GEMINI_PAPER1_V2_AUDIT_*.md`, `DUAL_AUDIT_V2_VERDICT_*.md`.

Final paper-release tag: to be cut after all v2.1 edits are reviewer-checked, under the tag `paper1-v2.1`.

---

## § 9. Acknowledgments

Solo researcher (gretjia) with Claude Opus 4.7 AI collaborator. Methodology (pre-registration, ablation, dual-audit) is human-authored; AI collaborator executes code, analysis, drafting under human direction. Dual adversarial review by Codex (OpenAI) and Gemini 2.5 Pro (Google) caught methodological issues in v1 and are acknowledged as external reviewers.

---

## Appendix A. hard36 pool (pre-frozen)

Source: `handover/preregistration/hard36_pool.txt`, frozen at commit `045fa6b`. Construction rule: problems in MiniF2F test set that FAILed in BOTH Phase 9.A baseline seeds (31415 AND 2718) at CONDITION=n8, MAX_TRANSACTIONS=200.

```
aime_1991_p9                                algebra_amgm_sumasqdivbgeqsuma
aime_1997_p9                                algebra_apbon2pownleqapownpbpowon2
aime_1999_p11                               algebra_bleqa_apbon2msqrtableqambsqon8b
amc12_2000_p12                              amc12_2000_p6
amc12a_2002_p6                              amc12a_2008_p25
amc12a_2009_p7                              amc12b_2021_p1
amc12b_2021_p13                             amc12b_2021_p4
imo_1962_p2                                 imo_1964_p2
imo_1965_p2                                 imo_1981_p6
induction_1pxpownlt1pnx                     induction_sumkexp3eqsumksq
mathd_algebra_148                           mathd_algebra_170
mathd_algebra_196                           mathd_algebra_208
mathd_algebra_246                           mathd_algebra_270
mathd_algebra_293                           mathd_algebra_332
mathd_algebra_44                            mathd_numbertheory_150
mathd_numbertheory_427                      mathd_numbertheory_447
mathd_numbertheory_5                        mathd_numbertheory_99
numbertheory_2pownm1prime_nprime            numbertheory_notEquiv2i2jasqbsqdiv8
```

Pool size: 36. Drawn sample (10, via seed 20260423, fingerprint `a94c0ae30f728e6c`):
```
algebra_bleqa_apbon2msqrtableqambsqon8b
amc12_2000_p12
amc12_2000_p6
amc12b_2021_p13
imo_1962_p2
mathd_algebra_208
mathd_algebra_246
mathd_algebra_270
mathd_algebra_332
numbertheory_2pownm1prime_nprime
```

## Appendix B. Selection script (deterministic, verbatim)

```python
import random
with open('handover/preregistration/hard36_pool.txt') as f:
    pool = [l.strip() for l in f if l.strip() and not l.startswith('#')]
sample = sorted(random.Random(20260423).sample(pool, 10))
for s in sample:
    print(s)
```

Fingerprint verification: `python3 tools/prereg_check.py handover/preregistration/PREREG_E1V2_HETEROGENEITY_2026-04-23.md` exits 0 and prints `PREREG check PASS`.

## Appendix C. Sample B-unique winning proof

B-unique winners (problems solved by some B run but NEVER by any A run):
`algebra_bleqa_apbon2msqrtableqambsqon8b`, `mathd_algebra_270`, `mathd_algebra_332`, `numbertheory_2pownm1prime_nprime`. Per-problem winning gp_payload artifacts live at `handover/evidence/v2/<problem>_s<seed>_B.lean` and re-verify via `python3 tools/audit_proof.py <file>`. A representative B-unique proof header is extracted mechanically from the jsonl by:

```bash
python3 -c "
import json, pathlib, sys
for jf in sorted(pathlib.Path('.claude/worktrees/phase-8a-snapshot/experiments/minif2f_v4/logs').glob('E1v2_B_*.jsonl')):
    for line in jf.read_text().splitlines():
        e = json.loads(line)
        if e.get('has_golden_path') and 'algebra_bleqa' in e['problem']:
            print(jf.name, '->', e.get('gp_payload','<gp omitted from aggregator jsonl; see proofs/ archive>')[:200])
"
```

The aggregated jsonl does not inline the proof body by default; the proof `.lean` files are archived under `handover/evidence/v2/` for arXiv submission.

---

**Status**: v2.1 draft, all 4 seeds × 3 conditions collected, round-2 dual-audit (Codex + Gemini) applied. See `handover/audits/DUAL_AUDIT_V2_VERDICT_2026-04-24.md` for round-2 verdict and residual v2.2 deliverables (problem-cluster sensitivity analysis, easy-set re-run, token-budget table, Appendix C node-count extraction, Docker build transcript).
