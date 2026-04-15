# TuringOS v4 — Handover State

**Updated**: 2026-04-15  
**Session Summary**: v3.1 experiment completed — **n1 STRICT WIN over oneshot** (+7 solves/50, SolveRate 60% vs 46%). n3 aborted due to Art. II.1 broadcast mechanism break (F-2026-04-15-02, confirmed non-speculation).

**Batch timestamp for audit**: `20260415T013559`

## v3.1 Official Results (N=50 stratified, seed=74677, fp=796ead6c40351ae9)

### Primary metric (SolveRate, aborts = fail)

| Condition | Solves / N | SolveRate | Aggregate_PPUT | Notes |
|---|---|---|---|---|
| oneshot | 23/50 | **46.0%** | 0.171 | 0 timeouts |
| **n1** | **30/50** | **60.0%** | 0.126 | **+7 STRICT WIN vs oneshot**; 20 timeouts |
| n3 | 7/50 | 14.0% | 0.018 | abort gate triggered @problem 10; 43 subsequent skipped |

### Paired-subset (7 problems where all 3 conditions completed pre-abort)

| Condition | Solves / 7 | Rate |
|---|---|---|
| oneshot | 2 | 29% |
| n1 | 7 | 100% |
| n3 | 7 | 100% |

**Pairwise**: n1 vs oneshot STRICT WIN (+5); n3 vs n1 EQUIVALENT; n3 vs oneshot STRICT WIN (+5).

### Key signal (narrow, evidence-anchored)

- **Primary**: n1 strict win over oneshot (+7 / +5 paired). Reproducibility of this +7 gap across seeds not tested.
- **Paired descriptive**: n1 = n3 = 7/7 on the 7 problems where all 3 completed pre-n3-abort. This is a **descriptive** equivalence on a very small N=7.
- **Causal claims deliberately NOT made here**:
  - Not claiming "multi-agent gives no marginal gain" — N=7 and broken Art. II.1 (F-2026-04-15-02) disallow that inference (C-033).
  - Not claiming "reasoner-redundancy confirmed" — that's the v3.2 hypothesis, not a v3.1 result.
  - Safe characterization per external audit: n3 ≈ "three independent n3-style attempts with broken coordination" (different from "3 independent oneshots"; oneshot had 0 timeouts, n3 had 3/10).
- **Open**: whether a fixed Art. II.1 (broadcast graveyard) would recover multi-agent benefit is a Step-B / v3.3 test.

## Files
- Results: `experiments/minif2f_v4/logs/v31_{oneshot,n1,n3}_20260415T013559.jsonl`
- Stderr: `experiments/minif2f_v4/logs/v31_20260415T013559.err`
- Analysis: `experiments/minif2f_v4/logs/frozen_analysis_20260415T013559.txt`
- Diagnosis: `handover/ai-direct/N3_DIAGNOSIS_2026-04-15.md`
- Notepad: `handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md`

## Next (post-M4 audit PASS)

1. v3.2 chat-model comparison (same seed, deepseek-chat) — test "TuringOS replaces CoT" thesis
2. Step-B protocol queued for `bus.rs recent_rejections` global-scope fix (post v3.2)

## Audit request

Auditors: read `AUTO_RESEARCH_NOTEPAD.md` §9 checklist first. Compare claims in this file against:
- Raw jsonl counts (grep `"has_golden_path":true`)
- frozen_analysis output
- N3_DIAGNOSIS causal chain
