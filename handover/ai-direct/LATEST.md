# TuringOS v4 — Handover State
**Updated**: 2026-04-21
**Session Summary**: Constitutional TuringOS complete. Four-doctor synthesis (Turing+Satoshi+Hayek+Drucker) implemented through Phases 0→7. First real depth-N golden paths produced (23, 20, 17 nodes); persistent-fail mathd_algebra_332 cracked via genuine multi-step δ-construction. 9/9 audit pass. Art. IV topology now fully executable at runtime.

## Current State

### What works
- All Art. IV topology mechanisms landed on main: WAL persistence (Phase 1), founder grant + settle_portfolios (Phase 2), mandatory wtool on ∏p=1 (Phase 2.1), audit artifacts + native_decide block (Phase 0 + F-20-05), Hayek problem bounty (Phase 3A), cross-problem wallet (Phase 4), Librarian message board with emergent role self-select (Phase 6-emergent), and Turing per-tactic δ-step with three-way oracle verdict (Phase 7).
- Honest single-run benchmarks: 17/20 Phase 2.1c baseline, 35/50 + 41/50 dual-path N=50, 9/20 Phase 7 step-only with 9/9 audit pass.
- 100% external re-verifiability via audit_proof.py + standalone proofs/*.lean artifacts.
- Law 2 conservation exact across all phases (verified by 5 unit tests in tests/reward_pull_conservation.rs).
- Crash-resume integration test passes (tests/wal_resume.rs).

### What's broken/incomplete
- Step-only mode loses 8 solves vs monolithic baseline (45% vs 85%) — Lean per-tactic elaboration is slower than full-proof one-shot. Production should default to dual-mode (both step and complete available; agent self-selects).
- Reward curve constants (γ, β, θ, BOUNTY_LP, FOUNDER_GRANT_GAMMA, SATOSHI_GAMMA_REBATE) still env-var, not constitutional defaults — yellow on red line #5.
- Librarian board overwrites per-tick instead of accumulating cross-problem session log.
- Phase 3B Satoshi citation rebate implemented on branch but not merged (was waiting for Phase 7 to make ancestry chains real).
- Phase 5 (cryptographic signing + permissionless onboarding) not started — needed for opening to external agents.

### Active experiments
- None running (Phase 7 N=20 batch completed; main binary rebuilt).
- Worktree branches preserved for archival: feat/tape-phase-{1-wal,2-rewardpull,2.1-mandatory-wtool,2.5-portfolio-prompt,4-cross-problem}, feat/phase-{3a-hayek,3b-satoshi,6-emergent,7-turing}.

## Next Steps

1. **Dual-mode N=50 honest benchmark** — run main binary without `TURING_STEP_ONLY` so both `step` and `complete` are available; measure solve rate + DAG depth distribution. Expected: solve rate recovers above 35/50 honest while depth histogram retains diversity from step usage on hard problems.
2. **Merge Phase 3B Satoshi citation rebate** — now that Phase 7 produces depth-17/20/23 ancestry chains, the rebate will actually fire and pay non-terminal contributors.
3. **Lift reward-curve env-vars to constitutional defaults** (close yellow on red line #5; canonicalize as C-042 spec).
4. **Variance N=50 dual-mode at multiple seeds** to establish a confidence band on the dual-mode headline.
5. **Phase 5 design**: cryptographic agent identity + zero-balance external registration (C-038) when ready to open to outside agents.

## Open Questions

- Dual-mode adoption: will deepseek-chat agents adaptively select `step` for hard problems, or always default to `complete`? Empirical question for next batch.
- Right magnitude of cold-fee on `complete` to nudge step adoption without tanking easy-solve rate. (Earlier cold-fee experiment refuted at flat 500/2000; might be revisitable now that step is a real alternative.)
- When to canonicalize the four-doctor reward curve (γ, B, θ) — likely after one more dual-mode validation run.
- Model upgrade to Opus / GPT-5: how much would step-acceptance rate rise (currently 41% partial-OK vs reject)?

## Reference
- Plan: `~/.claude/plans/replicated-enchanting-sundae.md`
- Checkpoints: `handover/ai-direct/CHECKPOINT_PHASE_*.md` (0, 1, 2, 2_1c, 4, 2_5, 3A+6_emergent, 7)
- Notepad: `handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md`
- New precedents this session: C-036 (telemetry), C-037 (WAL), C-039 (artifact), C-041 (cross-problem wallet), C-043 (mandatory wtool)
- Latest commits: f2f4fed (LATEST.md), e0a75ec (Phase 7 merge), 220e466 (Phase 7 checkpoint)
