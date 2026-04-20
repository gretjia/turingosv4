# TuringOS v4 — Handover State
**Updated**: 2026-04-20
**Session Summary**: 6 compounding architecture fixes + C-036 telemetry + user-raised Art. IV tape-∏p fix → best single-run 43/50 (86%), up from v3.1 baseline 30/50 (60%).

## Current State

### Headline
| Condition | Solves / 50 | Rate |
|---|---|---|
| v3.1 n1 (prior best) | 30/50 | 60% |
| N=8 TEMP_LADDER first | 37/50 | 74% |
| N=8 clean all fixes | 39/50 | 78% |
| **N=8 dual-path (current shape)** | **43/50** | **86%** |
| Best-of across 6 runs | 47/50 | 94% |

### Working
- TEMP_LADDER mechanism: per-agent temp 0.10..1.30, breaks agent correlation
- SEARCH_CAP=20: prevents 200-tx search loops
- Search feedback loop (hits → next prompt)
- C-036 harness telemetry (tool_dist, unique_payload_ratio, zero-tick warn, agent config echo)
- Dual-path ∏p: verify(payload) alone OR verify(tape+payload); Q_t fed if useful, not required

### Broken / incomplete
- `append: 1` across 43 solved problems — tape path exists but agents don't fill it; no economic incentive yet
- Tape is in-memory only — no persistence / resume (user's original "memory any tasks" concern)
- Search is filename-only (F-19-04) — symbolic queries return 0 hits
- 3 persistent failures across all 6 runs: `mathd_algebra_293`, `mathd_algebra_332`, `induction_sumkexp3eqsumksq`

### Active experiments
- Variance run N=50 with `BOLTZMANN_SEED=31415` (PID 4066291, early pace 8/8 solved)
  - ETA ~3h; validates whether 86% is reproducible or run-variance lucky

## Next Steps
1. **Wait for variance run** — confidence interval on 86%.
2. **Tape incentive (Step-B)**: credit `append` or auto-market each node so agents actually fill Q_t. Touches wallet/bus — branch first.
3. **Tape persistence / resume**: WAL to disk; restart recovers tape state. Addresses user's core Turing-memory point.
4. **Subgoal decomposition** for the 3 persistent failures (planner/prover split).

## Open Questions
- Is the 4pp gap between 43/50 single-run and 47/50 best-of driven by LLM sampling noise, or by tape-path occasionally helping/hurting? Variance run will start to answer.
- Tape incentive design: flat credit per `append` vs. market-based — which is more constitution-faithful to Law 2?
- Should the softened prompt go further (explicit "one-shot if confident") or is current copy at the right balance?

## Reference
- Notepad: `handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md` — F-2026-04-18-01..03, F-2026-04-19-01..08.
- C-036 precedent: `cases/C-036_diversity_probe.yaml`.
- Dual-path results: `logs/templadder_n8_20260419T221252.jsonl` (commit `8307b0d`).
- Variance run (in progress): `logs/templadder_n8_20260420T020239.jsonl`.
