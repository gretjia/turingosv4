# TuringOS v4 — Handover State

**Updated**: 2026-04-20 01:30 UTC
**Session Summary**: User raised Art. IV violation (tape unused). Strict tape-in-verification over-corrected (52% regression); dual-path revision (payload-alone OR tape+payload) restored one-shots and broke a persistent-fail. New best single-run **43/50 = 86%**.

## Current State

### Headline — run progression
| Condition | Solves / 50 | Rate |
|---|---|---|
| v3.1 oneshot | 23/50 | 46% |
| v3.1 n1 (prior best) | 30/50 | 60% |
| N=8 TEMP_LADDER first | 37/50 | 74% |
| N=8 clean all fixes | 39/50 | 78% |
| **N=8 dual-path (current)** | **43/50** | **86%** |
| Best-of across runs | 47/50 | 94% |

### Last commits this session
- `b88c7e1` F-2026-04-19-08 dual-path verification (current production shape)
- `0f46cb8` F-2026-04-19-07 strict tape-in-verification (reverted by b88c7e1)
- `7c13461` #28 search cap (env `SEARCH_CAP=20`)
- `eb42425` #26 search-loop closed
- `1ca892a` search handler wired (Art. III.2 dead-code fix)
- `c9c32a1` C-036 harness telemetry

### Telemetry on the 43-solve dual-path run
- `search: 399`, `complete: 184`, `parse_fail: 18`
- **`append: 1`, `complete_via_tape: 1`** — tape path is available but rarely invoked
  - `mathd_algebra_246` was the one problem where the tape fallback fired and won
- Agents mostly one-shot; tape is a standby mechanism, not the main path

### Persistent failures (failed ALL runs ≥2, dual-path included)
- `mathd_algebra_293`
- `induction_sumkexp3eqsumksq`
- `mathd_algebra_332` (solved 1× in strict-tape, failed elsewhere)

### Paired dual-path vs clean (same sample)
- Both solved: 37
- Dual-path only: 6 (includes formerly-persistent `amc12b_2021_p13`)
- Clean only: 2
- Net: +4 solves; McNemar p≈0.145 (N=8 discordant, not stat-sig)

## Next Steps

1. **Variance measurement**: rerun dual-path N=50 with `BOLTZMANN_SEED=31415` to establish confidence interval around 86%. Single-run still noisy.
2. **Incentivize tape**: `append: 1` shows the mechanism is available but ignored. To actually exercise Q_t → ∏p, add economic pull — credit `append` against a small stipend, or create an auto-market on each append for peer-invest. Touches wallet/bus → Step-B protocol.
3. **Tape persistence + resume**: address the user's core concern (memory). Persist tape to disk; on restart from a crash, resume from last tape state. Single-file WAL; currently tape is in-memory only.
4. **Subgoal decomposition**: remaining persistent-fails (`mathd_algebra_293/332`, `induction_sumkexp3eqsumksq`) are hard enough to need planner/prover split.

## Reference
- Notepad: `handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md` F-2026-04-18-01..03, F-2026-04-19-01..08.
- C-036 precedent: `cases/C-036_diversity_probe.yaml`.
- Run script: `experiments/minif2f_v4/analysis/run_temp_ladder.sh` (TEMP_LADDER=1 + SEARCH_CAP=20 defaults).
- Dual-path results: `logs/templadder_n8_20260419T221252.jsonl`.
