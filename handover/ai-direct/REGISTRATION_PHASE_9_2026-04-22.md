# Phase 9 Pre-Registration — 2026-04-22

**Purpose**: lock all seeds / configs / metrics / Gate criteria **before** the batch runs, so the final results are defensibly pre-registered (paper-level scientific rigor, C-023 Generator ≠ Evaluator applied to ourselves).

**Status**: registered (2026-04-22). No changes to this document without a new registration entry + explicit user sign-off.

---

## § 1. Scope

Phase 9 establishes the **论文级统计基线** for Paper 1 post-Phase-8 binary.

**Prerequisite**: Phase 2 A/B (Gate 8→9) PASS — experiment ΣPPUT ≥ 90% baseline.
If Phase 2 A/B FAILs, Phase 9 does not run until Phase 8 regressions are resolved.

Four sub-experiments:
- **9.A** 6 seeds × N=50 dual-mode (primary PPUT baseline)
- **9.B** 6 seeds × N=50 step-only (depth emergence verification)
- **9.C** Law 2 property-based test (10K random tx)
- **9.D** Karpathy TOP-3 micro-bench (perf claims)

Phase 8 diversity metric (pairwise payload) runs alongside 9.A/B; no separate batch.

---

## § 2. Pre-registered seeds

**Locked**: `{74677, 31415, 2718, 141421, 2357, 5772}`

Rationale:
- `74677`: v3.1 baseline (F-2026-04-15-06; re-used for historical anchor)
- `31415`: variance run (F-2026-04-20-02; π × 10^4)
- `2718`: e × 10^3 (new, independent)
- `141421`: √2 × 10^5 (new, independent)
- `2357`: 4th prime concatenation (new)
- `5772`: Euler-Mascheroni × 10^4 (new)

Total samples: 6 × 50 = 300 per condition; 600 total (dual + step).

**Power analysis** (Gemini Q6 methodology): with 300 problems and historical
Phase 7 rate ~45% step-only / ~85% monolithic, Wilson 95% CI half-width ≈ 0.056.
Sufficient to detect Δ ≥ 5pp between conditions.

---

## § 3. Pre-registered configs

### 9.A Dual-mode (step + complete both available)
```
TURING_STEP_ONLY=0       (default — not set)
TEMP_LADDER=1
HAYEK_BOUNTY=1
TAPE_ECONOMY_V2=1
CONDITION=n8
SAMPLE=experiments/minif2f_v4/analysis/sample_N50_S74677.txt (fp: 796ead6c40351ae9)
MODEL=deepseek-chat      (Paper 1 default; matches Phase 7)
MAX_TRANSACTIONS=200     (default)
```

### 9.B Step-only
Identical to 9.A except: `TURING_STEP_ONLY=1`

### 9.C Law 2 proptest
New test `tests/law2_proptest.rs`:
- `proptest` crate
- 10,000 random tx sequences
- Each tx: append / invest / halt_and_settle / receipt submission
- Invariant: `Σ wallet.balances + Σ market.lp_reserves == initial_total_coin`
- Covers: invest refund path, Hayek bounty payout, settle_portfolios

### 9.D Karpathy TOP-3 micro-bench
Criterion.rs benches:
- `trace_ancestors` HashSet → Vec<&str>
- `author + payload to_string` → Arc<str>
- `graveyard + TopKClasses` dedup

Acceptance per candidate: >5% wall-clock improvement → implement; <5% → archive decision as "not worth" + rationale.

---

## § 4. Pre-registered Gate criteria

### Gate 9 → 10

**Main (必过)**:
- Mean PPUT (solved-only, dual-mode, all 6 seeds combined) Wilson 95% CI lower bound ≥ **5.0**

**Auxiliary (全部必过)**:
- Σdepth≥10 PPUT > 0.5 across 6 step-only seeds **AND** depth≥10 solves ≥ 2
- pairwise_payload_diversity_mean ≥ 0.25 across 6 seeds (each seed's mean)
- reputation p50 > 0 (per seed — agent citations observed)
- Law 2 proptest (10K tx) 100% pass
- halt_reason_distribution公开 (at least 3 distinct reasons seen across 6 seeds)

**Rationale for 5.0 threshold**: historical top 3 Mean PPUT (solved) = 6.158 / 5.561 / 5.354 (Phase 7). 5.0 is "not significantly regressed" lower bound (see `PPUT_RAW_DATA_2026-04-22.md § 4.2`).

**Rationale for depth≥10 threshold**: Phase 7 pioneered 3 depth-17/20/23 solves. 0.5 ΣPPUT threshold ≈ Phase 7 baseline (0.65) − 0.15 tolerance. Prevents regression to pre-Phase-7 (where depth≥10 was zero).

---

## § 5. Pre-registered metrics (C-052 + Report Standard)

Every CHECKPOINT_PHASE_9_SEED_X must report:
1. **ΣPPUT** (all problems)
2. **Mean PPUT (solved-only)** + 95% CI (Wilson)
3. **Mean PPUT (all)** 
4. **Max depth** reached
5. **depth≥10 solves count** + Σ PPUT on depth≥10
6. **gp_path histogram** (alone / per_tactic / tape+payload)
7. **halt_reason_distribution**: OmegaAccepted / MaxTxExhausted / WallClockCap / ComputeCapViolated / ErrorHalt
8. **reputation_distribution** p50 / p90 / max
9. **pairwise_payload_diversity_mean + min** (multi-agent only)
10. **tool_dist aggregate**: complete / step / step_partial_ok / step_reject / append / omega_wtool / invest / search
11. **parent_selection_entropy** (Art. II.2.1, multi-agent)

Reports without these fields → violation of C-052, blocks Gate.

---

## § 6. Pre-registered analysis scripts

- `experiments/minif2f_v4/analysis/phase2_ab_analyze.py` (existing, Phase 2 Gate)
- `experiments/minif2f_v4/analysis/phase9_aggregate.py` (TBD, aggregates 6 seeds → table)
- `experiments/minif2f_v4/analysis/pput_scan.py` (existing, raw data)
- `handover/audits/PPUT_RAW_DATA_2026-04-22.md` (authoritative historical reference)

---

## § 7. Pre-registered failure modes

If a seed's run fails to complete:
- `MEASUREMENT_ERROR oneshot/swarm WAL` → retry once, then abandon seed (substitute with NEXT pre-registered backup seed: `{31, 1618, 1729}` in order)
- Oracle timeout > 300s consecutive on >3 problems → kill batch, investigate
- Disk full mid-batch → retry after cleanup, same seed
- Any code change to `src/` or `experiments/` between seed runs → **invalidates all already-run seeds**; batch must restart from seed 1

---

## § 8. Pre-registered budget

| Sub | Seeds | N | Condition × 2 | Est cost | Est time |
|---|---|---|---|---|---|
| 9.A dual | 6 | 50 | dual | $180 | ~18h |
| 9.B step | 6 | 50 | step_only | $180 | ~24h (slower per tactic) |
| 9.C proptest | — | — | — | <$1 | 1h |
| 9.D bench | — | — | — | <$5 | 2h |
| **Total** | — | — | — | **$370** | **~45h sequential; 15-24h with proxy parallelism** |

Parallelism: 3 seeds dual + 3 seeds step_only simultaneously via proxy (deepseek rate 60 req/min sufficient).

---

## § 9. Pre-registered post-conditions

Post-Phase-9 on PASS:
1. Merge experiment branch to main (if not already)
2. `handover/ai-direct/CHECKPOINT_PHASE_9_2026-04-XX.md` file
3. Update `LATEST.md` with PPUT baselines
4. Update `PPUT_RAW_DATA` with 12 new run entries (6 × 2)
5. Trigger Phase 10 Gate Wave A planning

On FAIL:
1. Do NOT advance to Phase 10
2. Root-cause analysis — which sub-task of Phase 8 caused the regression?
3. Hotfix cycle (new Step-B branch) + re-audit + re-Gate
4. `handover/audits/PHASE9_REGRESSION_2026-04-XX.md`

---

## § 10. Sign-off

| Role | Identity | Date |
|---|---|---|
| Registrar | Claude Opus 4.7 (this session) | 2026-04-22 |
| Authorizer | Human architect (user) | pending A/B PASS first |

**Any modification**: append new `§ 10.N revision` + explicit justification. This doc is append-only post sign-off.
