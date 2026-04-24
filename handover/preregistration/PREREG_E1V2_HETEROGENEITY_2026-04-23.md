# PREREG_E1V2_HETEROGENEITY_2026-04-23

experiment_id: E1V2_HETEROGENEITY_RERUN
date_created: 2026-04-23
author: gretjia (with Claude Opus 4.7 collaborator)
committed_commit_sha: PENDING — this file committed before any data-collection run; commit hash recorded by git post-commit hook

rationale: |
  Paper 1 v1 dual-audit (Codex + Gemini) returned CHALLENGE citing:
  - P0-1: hard10 sample selection was stratified-manual without pre-registration (p-hacking risk)
  - P0-5: ablation was single-seed (N=1 insufficient for causal mechanism)

  This v2 re-registers the experiment with:
  - Deterministic random-sample of 10 problems from full 36-problem hard pool, with pre-committed seed
  - Explicit multiplicity family declaration
  - Full 4-seed ablation (matching the 4-seed A/B)

## primary_endpoint

statistic: McNemar exact binomial **one-sided** test on paired-by-problem A vs B discordant cells across 4 Boltzmann routing seeds

sample: sample_E1v2_hard10_S20260423.txt (fingerprint a94c0ae30f728e6c)

threshold: p < 0.0125 (Bonferroni-corrected α for family of 4 inferential tests)

## directional_hypothesis

**One-sided**: B (4-role heterogeneous swarm, including Meta-Planner) > A (8× skill_0 homogeneous swarm) on paired solve count.

Rationale for one-sided: the treatment (heterogeneous) is directly motivated by the theoretical prediction that diverse tactic priors should solve a broader problem distribution. Two-sided testing would be non-directional and weaker given the same data; we commit to the directional alternative BEFORE data collection.

## secondary_endpoints

Declared BEFORE data collection. All inferentially tested with Bonferroni α = 0.0125.

1. **Easy-set control**: N=10 problems SOLVED in all 3 Phase 9.A seeds. Prediction: A = B = 10/10, Δ = 0. **Passes iff Δ ≤ 1** (accepts noise of one trial). Counts as separate inferential test.

2. **Ablation (EXCLUDE_META_PLANNER=1)**: 3 skills (no skill_3 Meta-Planner) × 4 Boltzmann seeds on the SAME sample_E1v2_hard10_S20260423.txt. Prediction: Ablation-solve count < B-solve count by ≥ 2. **Passes iff McNemar one-sided p < 0.0125 (Ablation < B)**.

3. **Per-seed solve-set containment**: count seeds where solve(A) ⊆ solve(B). Prediction: ≥ 3/4. Exploratory, NOT inferentially tested.

4. **Tactic-composition evidence** (payload-level, descriptive): unique solves should show multi-skill tactic composition in gp_payload. Exploratory, NOT inferentially tested.

## sample_construction

source_pool: handover/preregistration/hard36_pool.txt (36 problems)

pool_construction_rule: |
  Problems that FAILed in BOTH Phase 9.A baseline seeds (31415 AND 2718) at the N=50 n8 dual-mode run. This pool is FROZEN at commit time of this PREREG. Any future E1 variant must draw from this pool (or explicitly re-register with a new pool).

selection_rule: |
  Deterministic random sample of 10 problems from hard36 pool, using
  `rng = random.Random(20260423)` (today's date as seed).
  seed=20260423 chosen BEFORE seeing the draw. Publicly verifiable in the git history.

selection_script: |
  rng = random.Random(20260423)
  sample = sorted(rng.sample(pool, 10))

fingerprint: a94c0ae30f728e6c (SHA-256 hex[0:16] of newline-joined sorted problem names)

## stopping_rule

seeds: 4 Boltzmann routing seeds, FIXED a priori:
  - 141421 (√2 × 10^5)
  - 31415 (π × 10^4)
  - 2718 (e × 10^3)
  - 2357 (4th-prime concatenation)

wallclock_cap_per_problem: 900 seconds (external timeout in run_list.sh)
max_transactions_per_problem: 50 (MAX_TRANSACTIONS=50)
total_trials_target: 4 seeds × 10 problems = 40 paired A/B + 40 ablation = 80 solves

abort_condition: if any batch returns MEASUREMENT_ERROR > 2/10, investigate before interpreting; do NOT pool.

## multiplicity_family

family_size: 2 primary inferential tests (hard-set A vs B, ablation vs B)
  + 2 secondary inferential tests (easy-set Δ, per-seed containment)
  = **family_size = 4 Bonferroni-adjusted**

correction: Bonferroni α_family = 0.05; per-test α = **0.0125**

alternative_rule: Holm step-down acceptable if user prefers less conservative; currently declaring Bonferroni for strict conservatism.

## what_would_falsify

The primary claim is FALSIFIED if ANY of:
- A-unique solves ≥ 3 across 4 seeds (symmetry reversal)
- Easy-set Δ > 2 (the effect isn't specific to compositional problems)
- Ablation solves ≥ B solves on at least 2 seeds (Meta-Planner adds nothing)
- Pooled McNemar one-sided p > 0.0125 (Bonferroni threshold)

If any of these, we report the experiment as a negative finding and do NOT proceed with Paper 1 emergence claim.

## claim_language_constraints

Per C-070 ruling item 2, the following claims are FORBIDDEN unless data supports them at threshold:

| Claim | Threshold |
|---|---|
| "strictly dominates" | 4/4 seeds solve(A) ⊆ solve(B) |
| "emergence" / "swarm intelligence" | Requires additional evidence beyond solve-rate gain; demoted otherwise |
| "Meta-Planner is THE mechanism" | Ablation effect size > heterogeneity effect size across ≥ 3 seeds |
| "first application of X" | Prior-art search (Google Scholar / arXiv) with documented search query + null result |

Default claim language for this experiment (pre-reg'd):
- "4-role heterogeneous swarm solves more problems than 1-role swarm on hard MiniF2F sample"
- Qualify all effect claims with seed count and McNemar exact one-sided p
- Qualify all "mechanism" claims with ablation N seeds and effect size

## execution_plan

1. **Run A (HOMOGENEOUS_AGENTS=1) × 4 seeds × 10 problems** on sample_E1v2_hard10_S20260423.txt
2. **Run B (default 4 skills) × 4 seeds × 10 problems** on same sample
3. **Run Ablation (EXCLUDE_META_PLANNER=1) × 4 seeds × 10 problems** on same sample
4. **Easy-set A and B × 1 seed** (reuse existing; no rerun needed for control)
5. Aggregate + Bonferroni-correct + write v2 draft

## prior_iterations

- Paper 1 v1 draft: commit `2687882` on main, status CHALLENGE per dual-audit. Retained as historical record.
- v1 evidence: handover/evidence/e1_jsonl/E1_*.jsonl (8 batches, N=80 trials). Will be retained for comparison in v2 draft but NOT used as primary evidence (replaced by v2 random-sampled data).

## author signature

Pre-registered AT commit time (before any run). Any deviation from this pre-reg in the resulting paper MUST be explicitly flagged in the paper's methods section.

Commit this file FIRST, then execute the runs. Do not modify after commit.
