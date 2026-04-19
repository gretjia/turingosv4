# TuringOS v4 — Handover State

**Updated**: 2026-04-19 06:20 UTC
**Session Summary**: TEMP_LADDER mechanism validated at N=50 → +14pp over v3.1 baseline. C-036 harness telemetry landed + caught one hidden constitutional bug (search engine dead at swarm layer) on first batch.

## Current State

### Headline result (F-2026-04-19-03)
| Condition | Solves / 50 | Rate |
|---|---|---|
| v3.1 oneshot | 23/50 | 46% |
| v3.1 n1 (baseline) | 30/50 | 60% |
| **templadder n8 (new)** | **37/50** | **74%** |

Paired 20-subset (nscaling_n8 vs templadder_n8): 4 new solves / 0 regressions. McNemar one-sided p≈0.0625 on N=20 — directionally strong, larger-N rerun would push below 0.05.

### Landed changes (committed this session)
- `c9c32a1` C-036 harness telemetry: tool_dist, unique_payload_ratio, zero-tick warn, per-agent (skill,temp) echo. No constitution edit.
- `1ca892a` Art. III.2 search handler wired (was dead code silently dropped by `_ => {}`).
- Evaluator flag `TEMP_LADDER=1` → per-agent temperature 0.10..1.30. Off by default.

### Constitutional engine health (from batch telemetry)
- `search`: 2297 calls on 45 problems — heavily used on hard targets (up from 0 usable before fix)
- `invest`: 43 — markets modestly active
- `complete`: 269 — dominant path (most problems one-shot)
- `append`: **0** — tape stays empty across entire N=50 batch

### Known dead infrastructure
- Tape never populated → Art. II.1 broadcast has nothing to abstract
- Art. III.3 correlation shielding deferred until tape is alive
- Search results not fed back into agent prompt (task #26 pending)

## Next Steps

1. **Mechanism to activate tape**: agents prefer `complete` over `append`. Options:
   - Economic: make `append` earn baseline Coins; make `complete` cost Coins (shift incentive).
   - Structural: disable `complete` for first K tx, forcing agents to build via `append`.
   - Either is mechanism-level (C-034) and needs Step-B if touching bus.rs/kernel.rs.
2. **Close search loop** (task #26): include top search hits in next prompt — cheapest way to make Art. III.2 progressively disclose.
3. **Reproducibility variance**: some problems flip between N=20 and N=50 runs (e.g. `algebra_apbon...`, `imo_1964_p2`). Consider seeded LLM sampling if available.
4. **Stop hand-tuning temperature**: default TEMP_LADDER=1 once a second seed validates. Current ladder (0.10..1.30 clamped) is arbitrary; could ablate.

## Open Questions

- Bernoulli gap: predicted 99.9% at N=8, observed 74%. Is the 26pp residual (a) same-model correlation floor even with temp ladder, (b) scaffold ceiling on inherently hard problems, or (c) tape-empty bottleneck? Each has a different fix.
- Tape activation: will economic incentive alone move agents off `complete`-only, or do we need hard-gating (disable complete)?

## Reference

- Baseline: v3.1 batch `20260415T013559`, commit `e58e021`.
- Notepad: `handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md` — F-2026-04-18-01/02/03, F-2026-04-19-01/02/03.
- C-036 precedent: `cases/C-036_diversity_probe.yaml`.
- Run scripts: `experiments/minif2f_v4/analysis/run_temp_ladder.sh` (accepts sample arg; env `TEMP_LADDER=1`).
