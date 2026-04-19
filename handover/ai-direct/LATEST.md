# TuringOS v4 — Handover State

**Updated**: 2026-04-18 23:30 UTC
**Session Summary**: N-scaling experiment complete (FLAT curve) → diagnosed catastrophic agent correlation → temp-ladder mechanism intervention launched.

## Current State

- **6/7 constitutional components** landed on main (Art. II.1 broadcast, Art. II.2.1 skill diversity, Art. III.2 Librarian, Art. IV map-reduce tick, oracle-cache, generic nN). Art. III.3 correlation shielding remains stub (deferred — needs tape population).
- **N-scaling result (logs/nscaling_n*_20260418T143117.jsonl)**: PPUT(N=1,2,3,5,8) = (60%, 55%, 60%, 55%, 55%) on 20-problem sample. **FLAT** curve. Same 11 problems solved across all N; same 8 always fail. Bernoulli predicted ~99.9% at N=8 — violated by ~45pp.
- **Root cause (F-2026-04-18-01/02/03)**:
  - All 8 agents submit BYTE-IDENTICAL proofs (verified on `induction_1pxpownlt1pnx` n=8 trace)
  - Tape stays empty (`tape=0`) — agents only use `complete`, never `append`/`invest`
  - Temperature was fixed at 0.2 for all agents → no sampling decorrelation
- **Smoke test**: with `TEMP_LADDER=1` (per-agent temp 0.10..1.30), the previously unsolvable `induction_1pxpownlt1pnx` was SOLVED at tx=155 by Agent_3.
- **Active**: temp-ladder N=8 batch on 20 problems running (PID 3881314, log `exp_templadder.log`, results `templadder_n8_20260418T232656.jsonl`). ETA ~2-3h.

## Next Steps

1. Monitor temp-ladder batch; record solve count vs nscaling baseline (11/20).
2. If temp-ladder >12/20 → mechanism validated → replicate at full N=50.
3. If temp-ladder ≈11/20 → temperature alone insufficient → escalate to tactic-disjoint role specialization (Bull/Bear analog) or sub-goal decomposition.
4. Address dead infrastructure: agents bypass tape — consider mechanism to reward `append` (e.g., disable `complete` for first K tx).

## Open Questions

- Does temperature decorrelation alone restore Bernoulli scaling, or is tape-emptiness the dominant bottleneck?
- Is the 60% scaffold ceiling (8 always-failing problems) a model limit or architecture limit?
- After tape becomes active, does Art. II.1 broadcast (TopK error classes) actually steer agents away from repeated mistakes?

## Reference

- v3.1 baseline (preserved for context): n1 30/50 (60%) > oneshot 23/50 (46%) STRICT WIN +7. See git log `e58e021`.
- Notepad: `handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md` — F-2026-04-18-01/02/03 finding details.
