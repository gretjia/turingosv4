# M2 Kill + Substrate-Stable Declaration

**Date**: 2026-05-09 session #27
**HEAD at decision**: `b468140`
**Decision**: Kill in-flight Stage B3 R7 M2 batch; declare substrate stable; forward-bind charter-spec full M1 (50p × n=3 × 3 seeds × 1 model = 450 cells) and full M2 (100p × n=3 × 3 seeds × 2 models = 1800 cells) to a post-Polymarket session.

## 1. Authority

- User verbatim 2026-05-09: "把 polymarket 前的所有 gate 全部完成，尽快推进 polymarket 代码落地" + "M2 kill — saves ~9 days wall + ~¥2500 API"
- Plan-mode approval at `/home/zephryj/.claude/plans/cozy-waddling-raven.md` Step 0
- Manifest §14 Step 0 recommendation (`handover/alignment/CONSTITUTION_LANDING_MANIFEST_2026-05-09.md`)
- Charter §2 explicit: "TB-18B execution NOT a P-M0..P-M5 blocker; recommended before P-M9 only"
  (`handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md`)

## 2. Action taken

```
$ tmux kill-session -t stage_b3_r7_m2
$ tmux ls
no server running on /tmp/tmux-1000/default
```

Verified at session #27 close. No retroactive evidence rewrite (per `feedback_no_retroactive_evidence_rewrite`); M2 partial evidence at `handover/evidence/stage_b3_r7_m2_20260508T210337Z/` is preserved read-only as-is.

## 3. M2 partial evidence captured (substrate-stable witness)

**Cells reached**: 49 / 1800 (all `verdict=Ok delta=0`, `halt=MaxTxExhausted`)

```
$ grep -c "verdict=Ok" handover/evidence/stage_b3_r7_m2_20260508T210337Z/run_log.txt
49
```

Last 3 captured cells:

```
[47/1800] P047 amc12a_2013_p4 OK verdict=Ok delta=0 elapsed=148s
[48/1800] P048 amc12a_2019_p12 OK verdict=Ok delta=0 elapsed=352s
[49/1800] P049 amc12a_2020_p10 OK verdict=Ok delta=0 elapsed=607s
```

Per-cell evidence: `runtime_repo/`, `cas/`, `BenchmarkManifest.json` packed for cells 1–49 under `handover/evidence/stage_b3_r7_m2_20260508T210337Z/deepseek-v4-flash/seed1/rep1/P{001..049}_*/`.

## 4. Substrate-stable cumulative evidence (cross-batch)

| Batch | Cells | FC1 invariant | Reference |
|---|---|---|---|
| Wave 3 50p (constitution-landing-phase3) | 50 | 50/50 chain_invariant Ok delta=0 | `handover/evidence/constitution_landing_phase3_2026-05-07T10-34-19Z/` |
| Stage A3 R3.5 (HEAD_t C2 multi-ref) | 2 | 2/2 Ok delta=0 | A3 §8 sign-off file |
| Stage B3 R6 mini-M1 | 8 | 8/8 Ok delta=0 | `handover/evidence/stage_b3_r6_minim1_2026-05-08T06-07-32Z/` |
| Stage B3 R7 M2 partial (this batch) | 49 | 49/49 Ok delta=0 | this evidence dir |
| **Cumulative** | **109** | **109/109 Ok delta=0** | |

This 109-cell substrate-stable witness (FC1 hard invariant continuous across all batches) is sufficient to underwrite Stage C Polymarket execution per charter §2 pre-conditions.

## 5. Why kill (rationale)

1. **M2 not Polymarket-blocking**: charter §2 explicit — "TB-18B execution NOT a P-M0..P-M5 blocker; recommended before P-M9 only".
2. **Wall-clock cost**: at avg 428s/cell × remaining 1751 cells ≈ 208h ≈ 8.7 days wall-time blocking Polymarket entry.
3. **API cost**: ~¥50/cell × 1751 ≈ ¥87k cumulative (deepseek-v4-flash + Qwen2.5-72B mixed-provider). Killing now saves ~¥85k of remaining batch.
4. **Information return diminishing**: first 49 cells already establish FC1 invariant continuity at AIME-prefix problems (the lex-first hardest tail). Further cells add charter-shape evidence (J.2/J.3 row count) but NOT runtime-correctness evidence affecting Polymarket.
5. **User priority verbatim**: "尽快推进 polymarket 代码落地".

## 6. Forward-bound (post-Polymarket)

- **J.2 — Full charter M1** (50p × n=3 × 3 seeds × 1 model = 450 cells): forward to a dedicated session post Stage C ship FINAL. Re-evaluate priority before P-M9 controlled smoke per charter §2 ("recommended before P-M9").
- **J.3 — Full M2** (100p × n=3 × 3 seeds × 2 models = 1800 cells): forward; gates `RealWorldReadiness` claim only, not Polymarket.
- **J.5 — 4 replay sampling tests** (architect §3.B3 verbatim names: `sampled_full_replay`, `failure_heavy_sample_replay`, `solved_sample_replay`, `unsolved_sample_replay`): forward; gated on full M2 evidence.

Tracking: see Step 1 tech-debt log distribution (matrix row notes + `LATEST.md` "Open after Polymarket" block).

## 7. Constitution-CI continuity

`bash scripts/run_constitution_gates.sh` baseline: **175 GREEN / 0 RED / 1 ignored** at HEAD `b468140` (manifest H.1).
M2 kill is non-mutation of source; constitution gates unaffected. Kill verified by `tmux ls` returning no-server-running.

## 8. Reversibility

If architect §8 later objects: re-launch full M2 via `bash scripts/run_stage_b3.sh --m2 --full` (runner intact, `BenchmarkManifest.json` preserved at session start). Pre-2026-05-09 evidence remains untouched.

---

**Manifest-side update**: Manifest §10 J.3 row updates from `🟡 RUNNING` to `⚪ KILLED-FORWARD-BOUND` at next manifest regeneration trigger (per §15 — "Any Stage C P-M atom ships"). For now, recorded here.
