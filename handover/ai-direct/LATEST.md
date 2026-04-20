# TuringOS v4 — Handover State
**Updated**: 2026-04-20 (flight-window autonomous work)
**Session Summary**: Dual-path verified firm at 84% mean (range 82-86%); tape-economy bold hypothesis refuted — economic cold fee alone does not activate tape at any fee level.

## Current State

### Headline — confirmed
| Run | Solves / N | Rate |
|---|---|---|
| v3.1 n1 (prior best) | 30/50 | 60% |
| Dual-path seed=74677 | 43/50 | 86% |
| Dual-path seed=31415 | 41/50 | 82% |
| **Dual-path mean (2 seeds)** | **42/50** | **84%** |

### Bold hypothesis test (refuted)
Branch `feat/tape-economy-v1` — COMPLETE_COLD_FEE on empty tape:
- v1 fee=500: 16/20, `complete_cold_fee=51` = `complete=51`, `append=0`
- v2 fee=2000: 16/20, `complete_cold_fee=54` = `complete=54`, `append=0`

**Conclusion**: agents pay cold fee 100% of the time until bankrupt, then skip.
Never switch to append. Economic-only cannot activate tape. Branch held,
not merged — design doc and telemetry preserved for next session.

### Working (main branch)
- TEMP_LADDER per-agent (0.10..1.30) — decorrelation
- SEARCH_CAP=20 per-agent — no more 200-tx search loops
- Art. III.2 search feedback loop (hits → next prompt)
- Art. IV dual-path ∏p: `verify(payload)` or `verify(tape+payload)`, accept either
- C-036 harness telemetry (tool_dist, unique_payload_ratio, zero-tick warn)

### Persistent failures (across 6+ runs)
- `mathd_algebra_293`, `mathd_algebra_332`, `induction_sumkexp3eqsumksq`

## Next Steps (needs user input)
1. **Choose tape-activation mechanism**: structural gate (forbid complete on empty tape) vs progressive gate (first K tx no-complete) vs reward-pull (bonus for tape-based solve). Economic alone is proven insufficient.
2. **Tape persistence (WAL)** — user's original "memory any tasks" concern. Still open; ledger.rs edit, non-restricted.
3. **Subgoal decomposition** for 3 persistent fails.
4. **Merge decision** for `feat/tape-economy-v1`: currently only useful as infrastructure; the actual behavior change is null. Recommend: keep branch, do NOT merge, revisit with a pull-based mechanism instead of push-based fee.

## Open Questions
- Is rational-agent bankruptcy (F-20-04 observed) a signal that this model is too short-term-optimizing to learn tape? Different model might behave differently.
- Would a *reward* for tape-based solves (instead of *penalty* for direct) change anything? Current kernel.resolve_all already rewards YES-holders on GP nodes; perhaps agents just don't see the reward chain clearly.

## Branch state
- `main`: commit `50f2ecb` with variance + v1 findings
- `feat/tape-economy-v1` (worktree `../v4-tape-economy/`): commit `ac079b0` has the cold-fee infrastructure. NOT merged. Two successful v1/v2 runs against it documented.

## Reference
- Design doc: `handover/ai-direct/TAPE_ECONOMY_v1_2026-04-20.md`
- Notepad: `handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md` — F-2026-04-20-02/03/04
- C-036 precedent: `cases/C-036_diversity_probe.yaml`
- Variance run: `logs/templadder_n8_20260420T020239.jsonl`
- v1 fee=500:  `logs/templadder_n8_20260420T044330.jsonl`
- v2 fee=2000: `logs/templadder_n8_20260420T063054.jsonl`
