# Preregistration Directory — Pre-Reg Discipline per C-070

**Purpose**: every experiment whose results may appear in a public preprint / paper / external claim must open a PREREG file BEFORE any data-collection run.

**Enforcement**: `tools/prereg_check.py` validates any `PAPER_*_DRAFT*.md` by requiring each numbered claim to reference a PREREG file + matching sample file + matching jsonl batch.

## Naming convention

`PREREG_<experiment_id>_<date>.md` — example: `PREREG_E1_HETEROGENEITY_2026-04-22.md`.

## Template

```markdown
# PREREG_<EXPERIMENT_ID>_<DATE>

experiment_id: <string, e.g. E1_V2_HETEROGENEITY>
date_created: <YYYY-MM-DD>
author: <name / handle>
committed_commit_sha: <git rev-parse HEAD at time of pre-reg, locks design>

primary_endpoint:
  statistic: <e.g. McNemar exact binomial one-sided>
  sample: <name + fingerprint, e.g. sample_E1v2_hard10_S20260423.txt>
  threshold: <e.g. p < 0.05 one-sided>

directional_hypothesis: <one-sided / two-sided + direction>

secondary_endpoints:
  - name: <e.g. easy-set Δ>
    statistic: <descriptive / inferential with Bonferroni threshold>
  - ...

sample_construction:
  source_pool: <file, e.g. hard36_pool.txt>
  pool_construction_rule: <e.g. "problems FAIL in both Phase 9.A baseline seeds 31415 + 2718 at N=50 cap">
  selection_rule: <e.g. "random.sample(seed=20260423, n=10)">
  selection_script: <path to deterministic script>
  fingerprint: <hex hash of resulting sample>

stopping_rule:
  seeds: <e.g. 4 Boltzmann seeds: 141421, 31415, 2718, 2357>
  wallclock_cap_per_problem: <secs>
  max_transactions_per_problem: <int>
  total_trials_target: <e.g. 40 paired>

multiplicity_family:
  family_size: <int, number of inferential tests>
  correction: <e.g. Bonferroni α=0.0125 for family of 4>

what_would_falsify: |
  Explicit condition under which we abandon the primary claim.
  Example: "A-unique ≥ 2 across 4 seeds" or "easy-set Δ > 0"

prior_iterations: <list of any previous PREREG for same experiment, e.g. "E1 v1 PREREG did not exist; this v2 re-registers per C-070">
```

## Files in this directory

- `PREREG_E1V2_HETEROGENEITY_2026-04-23.md` — Paper 1 v2 re-registration (addresses Paper 1 v1 dual-audit P0-1, P0-2, P0-5)
- `hard36_pool.txt` — frozen pool of 36 hard problems from which v2 samples are drawn
- `sample_E1v2_hard10_S20260423.txt` — deterministic random draw of 10 from pool

## Check scripts

- `../../tools/prereg_check.py` — validates PREREG file structure + fingerprint consistency
- `../../tools/ablation_gate.py` — requires ≥ 3 seeds for any causal claim
- `../../tools/claim_lint.py` — greps draft for strong-claim language without PREREG backing
