# Auto-Research Summary — 2026-04-17

## Session achievements (data-backed)

### Experiments completed (12 runs, ~$120 API, ~48h wall)
- v3.1 reasoner N=50: n1=30/50 (60%) STRICT WIN over oneshot 23/50
- v3.2 chat N=50: chat+scaffold 46-50% vs chat+oneshot 0%
- v3.3 Art. II.1 treatment N=50: n1+5 but n3 unchanged
- Fix #2 Art. III.3 N=20: ABANDONED (tape empty)
- Fix #4 force-append solo: FAILED (0 appends)
- Bundle (II.1 + force-append): tape alive! Bernoulli excess +0.7%
- 3-way parallel (oracle-cache / agent-verify / async-oracle) N=20
- P3-hybrid N=20
- Phase 3 incremental tactic N=20: FAILED (LLM granularity mismatch)

### Key findings
1. **F-2026-04-16-07**: Bernoulli excess from -31% to +0.7% — negative interaction ELIMINATED
2. **F-2026-04-17-01**: oracle-cache best branch (n3=6>n1=5, 0 timeouts)
3. **F-2026-04-17-03**: Constitutional ∏p loop correctly requires verify-before-write
4. **F-2026-04-17-04**: Incremental tactic verification requires tactic-level LLM output (future)
5. **Force-append is unconstitutional** (auditor ruling: micromanagement)

### Architecture state
- Tape activation: proven possible (depth 18.8 on oracle-cache)
- Negative interaction: eliminated when tape active
- North Star (n↑→PPUT↑ super-linear): NOT YET achieved
- Bottleneck identified: LLM produces full proofs, not decomposed subgoals

## Best branch to merge: oracle-cache @353b20f
- Art. II.1 broadcast (TopKClasses) + oracle cache + C-027 payload limits
- n3>n1 signal present (6>5 on N=20)
- 0 timeouts
- Does NOT include force-append (removed per constitutional audit)

## Next session priority: Subgoal Decomposition (DeepSeek-Prover-V2 style)
- Add "decomposer" pass: split theorem goal into subgoals (lemmas)
- Assign different subgoals to different agents (natural diversity)
- Each agent solves its subgoal independently → tape accumulates verified sublemmas
- Assembly agent combines sublemmas into complete proof
- This is Pattern A from literature agent, aligned with Art. II.2 (价格信号引导不同方向)
- Expected: genuine n↑→PPUT↑ because agents do DIFFERENT WORK, not same work differently

## Files for next session
- `/handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md` — full research state
- `/handover/ai-direct/PLAN_PHASE3_CONSTITUTIONAL_LOOP_2026-04-17.md` — Phase 3 design
- `/handover/ai-direct/PLAN_PHASE2_2026-04-17.md` — P1-P4 proposals (P3 hybrid partial success)
- `/handover/ai-direct/HYPOTHESIS_PERCOLATION_2026-04-16.md` — N-scaling framework
- `/routines/daily_drift.yaml` — active daily constitutional audit
- Branches: `experiment/oracle-cache` (merge candidate), `experiment/phase3-incremental` (future)
