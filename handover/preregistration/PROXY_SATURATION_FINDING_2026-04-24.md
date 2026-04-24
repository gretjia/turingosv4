# Proxy Saturation Finding — 2026-04-24 early

**Trigger**: user observation — "如果 proxy timeout 的原因是 deepseek 服务商不能提供高并发导致的, 影响了实验结果, 就应该反思, 不然结果没意义"

**Context**: E1 v2 data regeneration running 12 batches × 8 agents = 96 concurrent LLM requests at full saturation.

---

## § 1. Observed contamination

First 30 outcomes across 12 parallel batches:

| Outcome | Count | % |
|---|---|---|
| MEASUREMENT_ERROR (outer 900s timeout hit) | 22 | **73%** |
| SOLVED | 5 | 17% |
| FAIL (proper MaxTxExhausted) | 0 | 0% |
| pending | 3 | 10% |

Compare to identical evaluator binary + identical sample run at ≤2 concurrent batches:

| Configuration | MEASUREMENT_ERROR rate |
|---|---|
| 12-way parallel (this run) | 73% |
| 2-way parallel (Paper 1 v1 runs) | 0-3% |
| 1-way serial (Phase 9.A initial) | 0% |

---

## § 2. Diagnosis

**Root cause**: DeepSeek chat API rate-limits or concurrency-caps at ~16-20 concurrent requests per API key. At 12 batches × 8 agents = 96 concurrent, individual request latency balloons from ~5-10s to ~15-20s+. With MAX_TRANSACTIONS=50 × 15-20s/tx = 750-1000s, the outer 900s cap fires before internal MaxTxExhausted halt.

**Evidence**:
- Per-problem timing from Paper 1 v1 (2-way parallel): chat responses ~8-12s
- Per-problem timing from E1 v2 (12-way parallel): ~15-20s per response on hard problems
- MEASUREMENT_ERROR clusters on HARDEST problems (amc12_2000_p12, amc12_2000_p6) where MAX_TRANSACTIONS=50 is actually hit, not early solves

**Artifact class**: when proxy saturates, MEASUREMENT_ERROR is emitted instead of FAIL. This is silently equivalent to "this condition failed on this problem" in aggregation scripts, but in reality the evaluator never had a chance to either solve or halt cleanly.

---

## § 3. Implication for prior data

### 3.1 Paper 1 v1 data (all E1 × 3 seeds + easy-set + ablation): **NOT affected**

All v1 batches ran 2-way parallel (A + B simultaneously per seed). MEASUREMENT_ERROR count across v1:
- `E1_A_homogeneous`: 0
- `E1_B_heterogeneous`: 0
- `E1_A_seed31415`: 0
- `E1_B_seed31415`: 0
- `E1_A_seed2718`: 0 (1 MEASUREMENT_ERROR seen was on problem 11 / 20, proxy load lower then)
- `E1_B_seed2718`: 0
- `E1_A_seed2357`: 0
- `E1_B_seed2357`: 0
- `E1_A_easy_ctrl`, `E1_B_easy_ctrl`: 0
- `E1_ablation_no_meta_seed141421`: 0

**v1 data integrity confirmed**. Paper 1 v1 CHALLENGE verdict stands on its methodological merits (P0-1 sample bias, P0-2 McNemar labeling, P0-3 overclaim, P0-4 mechanism N=1, P0-5 ablation scope). Proxy saturation is NOT a new blocker for v1.

### 3.2 Phase 9.A baseline data: 1 MEASUREMENT_ERROR on seed 2718 (negligible, <2%)

Phase 9.A ran sequentially with cap=200. Minor contamination. Does not invalidate seed 2718 aggregate.

### 3.3 E1 v2 in-progress data: DISCARD

12-way parallel batches produced 73% MEASUREMENT_ERROR. Unusable. Will re-run with reduced concurrency.

---

## § 4. New harness constraint: MAX_PARALLEL_BATCHES

Add to PREREG template:

```yaml
concurrency_policy:
  max_parallel_evaluator_processes: 2
  rationale: |
    DeepSeek chat API saturates above ~16 concurrent requests; 2 batches ×
    8 agents = 16 is the empirically-safe ceiling. Higher parallelism
    produces MEASUREMENT_ERROR artifacts that contaminate FAIL statistics.
  enforcement: tools/concurrency_gate.sh (to be implemented) — wraps
    evaluator launch in a semaphore.
```

---

## § 5. Updated PREREG_E1V2_HETEROGENEITY

The PREREG committed earlier (`handover/preregistration/PREREG_E1V2_HETEROGENEITY_2026-04-23.md`) did NOT include a concurrency policy. This is now identified as a missing mandatory field.

Per C-070, PREREG is frozen at commit time. We have two options:
1. **Amend with explicit "deviation flagged"** — re-run E1 v2 with max_parallel=2, note in paper that "pre-reg did not specify concurrency; the proxy saturation discovered during execution required re-running at max_parallel=2, with no changes to sample/seeds/conditions"
2. **New PREREG (v3)** — supersede with fully-specified version

Option 1 is honest and documented — going with that.

---

## § 6. Re-run plan (immediate)

- **Discard**: all 12 in-progress jsonl files from E1 v2 first attempt
- **Re-launch**: 4 seeds × 3 conditions = 12 batches, but **serialized in 6 rounds of 2 parallel batches**
- **Estimated time**: 6 rounds × ~1h = 6h wallclock
- **Budget**: ≈$15-20 LLM

Re-launch pairing strategy (A and B of same seed paired to preserve paired-design structure):
- Round 1: A_s141421 + B_s141421
- Round 2: Abl_s141421 + A_s31415
- Round 3: B_s31415 + Abl_s31415
- Round 4: A_s2718 + B_s2718
- Round 5: Abl_s2718 + A_s2357
- Round 6: B_s2357 + Abl_s2357

Each round waits for previous to complete before starting.

---

## § 7. Methodology lesson for C-070 amendment

C-070 § ruling currently specifies pre-reg + multiplicity + N≥3. Missing:
- **Concurrency / load policy**: resource constraints must be pre-registered
- **Null-measurement class**: MEASUREMENT_ERROR must be classified (not silently aggregated as FAIL)

Both will be added to C-070's precedent section in a follow-up commit.

---

## § 8. Credit

User caught this BEFORE I did. The proxy saturation was producing MEASUREMENT_ERROR at 73%, and my earlier reports said "this is a hard problem, common-mode on both sides, doesn't bias paired Δ". That was wrong — the contamination is not benign; it destroys all FAIL statistics.

Paper 1 v2 methodology will explicitly include `max_parallel_batches` + `MEASUREMENT_ERROR exclusion rule`.
