# TuringOS v4 — Handover State

**Updated**: 2026-04-19 17:55 UTC
**Session Summary**: N-scaling flat curve led to discovery of full agent correlation → 4 compounding fixes landed (temp ladder, search handler, search loop, search cap) + C-036 harness telemetry. Clean single-run N=50 = **39/50 = 78%**, up from v3.1 baseline 60%.

## Current State

### Headline results
| Condition | Solves / 50 | Rate |
|---|---|---|
| v3.1 oneshot | 23/50 | 46% |
| v3.1 n1 (prior best) | 30/50 | 60% |
| N=8 fixed-temp (nscaling baseline) | ~28/50 (inferred) | 56% |
| N=8 TEMP_LADDER (first pass) | 37/50 | 74% |
| **N=8 TEMP_LADDER + search-cap + loop (clean)** | **39/50** | **78%** |
| Best-of across 4 runs | 46/50 | 92% |

### Landed changes (6 commits this session)
- `c9c32a1` C-036 harness telemetry (tool_dist, unique_payload_ratio, zero-tick warn, per-agent echo)
- `1ca892a` F-2026-04-19-02: Art. III.2 search handler wired (was dead code silently dropped)
- `656b00e` F-2026-04-19-03: N=50 validation
- `eb42425` #26 search-loop closed (hits surface in next prompt)
- `7c13461` #28 F-2026-04-19-05 fix: search budget cap per agent (env `SEARCH_CAP=20`)
- `682b589` F-2026-04-19-06 capped retry = 7/13 (2.3× pre-cap)

No constitution edits. One new precedent: `cases/C-036_diversity_probe.yaml`.

### Constitutional engine health (clean N=50 telemetry)
- `search`: 1245 (Art. III.2 active + cap-bounded)
- `complete`: 238 (dominant path)
- `invest`: 73 (Art. II.2 markets modestly active; nearly 2× vs first run's 43)
- `parse_fail`: 4 (tiny)
- **`append`: 0** — tape still empty across every problem

### Persistent failures (4/50, failed all 4 runs)
- `amc12b_2021_p13`
- `induction_sumkexp3eqsumksq`
- `mathd_algebra_293`
- `mathd_algebra_332`

These are likely genuinely hard within the scaffold+LLM ceiling. Candidates for subgoal-decomposition (DeepSeek-Prover-V2 pattern).

## Next Steps

1. **Statistical noise measurement**: rerun clean N=50 with a different `BOLTZMANN_SEED` / LLM seed to measure single-run variance. Current cumulative best-of (92%) vs single-run (78%) shows ~14pp variance band.
2. **Subgoal decomposition** for the 4 persistent failures. Mechanism-level: a planner agent posts `have …` sub-goals, workers fill them. Bigger architectural change.
3. **Tape activation**: `append=0` persistent across all runs. Economic incentive to earn on append, charge on complete — touches wallet.rs → Step-B protocol.
4. **Search as content grep**: F-2026-04-19-04 — SearchTool is filename-only but agents ask symbolic queries. Upgrade to grep inside .lean files would raise hit rate.

## Open Questions

- Bernoulli gap: predicted N=8 ≈ 99.9%, observed 78%. Residual 22pp is (a) scaffold ceiling on hard problems, (b) sampling correlation floor even with temp ladder, or (c) tape-empty bottleneck. Best-of analysis (92%) suggests (a) dominates for the 4 persistent fails; (b)/(c) explain the other ~8pp.
- Should TEMP_LADDER become the default (not opt-in env flag)?
- Is search content-grep worth building, or is the cap sufficient now?

## Reference
- Notepad: `handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md` — F-2026-04-18-01..03, F-2026-04-19-01..06.
- C-036 precedent: `cases/C-036_diversity_probe.yaml`.
- Run scripts: `experiments/minif2f_v4/analysis/run_temp_ladder.sh` (TEMP_LADDER=1, SEARCH_CAP=20 defaults).
- Clean N=50 results: `logs/templadder_n8_20260419T121906.jsonl`.
