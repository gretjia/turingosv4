# TuringOS v4 — Handover State
**Updated**: 2026-04-22 (late — Phase 2 A/B running)
**Status**: **Phase 8 implementation complete**; Phase 2 A/B N=20 oneshot running (main baseline vs experiment). Gate 8→9 waiting on A/B verdict (~2-4h wall time). All 3 rounds of external audit cleared (R1-α Ed25519 VETO-free).

## Phase 8 status (as of 2026-04-22 12:17 UTC)
- 8 commits on `experiment/phase-8a-snapshot-fix` (4 BLOCKER + 3 Critical + R1-α + R2 v2 + R3 + cases/checkpoint)
- 184+ tests green (lib + 6 integration binaries)
- 10 new judicial cases立档 (C-044/045/046/048/049/050/053/055/061/067)
- R1-α Ed25519 cryptographic capability — Codex + Gemini round-3 PASS
- R2/R3 CHALLENGE residuals fixed post round-3
- Self-audit PASS (MEASUREMENT_ERROR signaling correct; oracles_frozen lifecycle clean)
- Smoke test: `mathd_algebra_148` oneshot 80s solve + durable WAL (4-event hash chain incl. Halt{OmegaAccepted})
- **Phase 2 A/B running**: N=20 oneshot, main baseline vs experiment branch, seed=74677, ~2-4h expected, Gate: experiment ΣPPUT ≥ 90% × baseline ΣPPUT

**Session Summary (2026-04-22)**:
- 三路外部审计（Codex/Gemini/DeepSeek）VETO × 2 + 3 条 Claude 漏掉的代码层漏洞
- 立 C-052 PPUT 判例 + CLAUDE.md 新增 Report Standard 节
- PPUT 历史重审（**经 C-066 独立核查修订**）：Mean PPUT (solved-only) 跨 run 可比；Phase 7 的 5.354 在历史上排第 3（top 2 为 6.158 / 5.561，均为 depth-1 quick solve）；Phase 7 独特贡献是首次 `per_tactic` + 首次 depth≥10 solves ≥ 2（3 个 depth 17/20/23）。真实数据以 `PPUT_RAW_DATA_2026-04-22.md` 为准
- 战略方向调整：Phase 10 从 "Launch Ready" 改为 "Paper Preprint Ready"（外部接入推迟到 Phase 11+）
- 5 项决策已锁定（见 DECISIONS_2026-04-22.md）
- 泛化路线图：M-1 Predicate trait 预留 + Paper 2 (zeta_sum_proof) / Paper 3 (omegav4) 路径

**Prior Session Summary (2026-04-21)**: Constitutional TuringOS complete. Four-doctor synthesis implemented through Phases 0→7. First real depth-N golden paths produced (23, 20, 17 nodes). 9/9 audit pass. Art. IV topology now fully executable at runtime. [此 summary 部分基于 solve-count 视角，需用 PPUT 重估]

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

## Next Steps (2026-04-22 revision — supersedes 2026-04-21 version below)

**See `PLAN_FINAL_PHASE_8_TO_PAPER_2026-04-22.md`（待写，汇总后出炉）**

Immediate:
1. **Phase 8 BLOCKER 修复**（必过，非统计问题）
   - 8.A `bus.snapshot()` 空 balances — 最紧急，不修所有经济实验失真
   - 8.B `oneshot` 路径走 C-043 mandatory wtool（违 Art. IV）
   - 8.C `append_oracle_accepted` 加 OracleReceipt capability token（Codex V-1）
   - 8.D `decide`/`omega` Mathlib 语境白名单（C-011 完整执行）
   - 附加 M-1 `Predicate` trait 预留（Paper 2/3 泛化用）
2. **Phase 9 论文级统计基线**
   - 6 seeds × N=50 dual-mode / step-only
   - Gate: ΣPPUT CI 下界 ≥ 83.0 **或** Mean PPUT ≥ 5.0 **或** ΣPPUT on depth≥10 CI 下界 > 0
3. **Phase 10 Paper**：Art. V 三进程 + N=244 × 3 seeds + reproducibility bundle + arXiv submit
4. **Phase 11+（推迟）**：外部 agent 接入、P0-1/2/7 原计划内容

---

### ⚠️ Prior Next Steps (2026-04-21, deprecated 2026-04-22):

1. ~~Dual-mode N=50 honest benchmark~~ — 已知 N=50 power=46% 不足，改为 6 seeds × N=50
2. ~~Merge Phase 3B Satoshi citation rebate~~ — 推迟到 BLOCKER 修完
3. ~~Lift reward-curve env-vars to constitutional defaults~~ — 并入 Phase 10 Wave B
4. ~~Variance N=50 dual-mode at multiple seeds~~ — 统一为 Phase 9 计划
5. ~~Phase 5 design (cryptographic agent identity)~~ — 推迟到 Phase 11+

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
