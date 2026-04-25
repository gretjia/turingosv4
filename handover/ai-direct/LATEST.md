# TuringOS v4 — Handover State
**Updated**: 2026-04-25 (B2-B4 close + mid-term dual audit P0-A/P0-C fixes)
**Session Summary**: Phase B B2 (cost aggregator) + B3 (wall-clock) + B4 (dual PPUT) 全部落地 + 完成 mid-term external dual audit (Codex + Gemini, both CHALLENGE)，**P0-A (Phase-C-safety) + P0-C (first-read placement) 当场修完**，P0-B (schema v2) + P0-D (hybrid_v1 cost) + P0-E (flip assert) 正式 deferred 到 B5 起步先 pickup。**143/143 cargo test --workspace PASS** (was 131 baseline; +12 from B2-B4 unit tests)。`test_wall_clock_first_read_to_final_accept` 已恢复严格 ≥7100ms 断言 (mid-term P0-C fix)。

> **新 session 入口**: 读这个文件 + `handover/audits/B5_DEFERRED_FROM_MIDTERM_AUDIT_2026-04-25.md` (B5 起步必先解决 P0-B/D/E) + `handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md` § B5-B7 + `handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md` § F-2026-04-25-03 (mid-term audit lessons)。这 4 个文件足以无 context 接手当前工作。

## Current State

### Active research arc
**PPUT-driven Capability Compilation Loop (CCL)** — 30-day arc 2026-04-26 → 2026-05-26.
- North Star: Held-out Verified PPUT (H-VPPUT) on heldout-54
- Success criterion: WBCG_PPUT > 0 (≥1 Certified user-space artifact)
- Caps: 30 wall-clock days + USD 500 API budget (硬停)
- Backbone: `deepseek-v4-flash` thinking-off (Phase B+C); 异构 LLM at Phase D (v4-flash thinking-on + Gemini 2.5 Pro)

### Phase A — COMPLETE (Phase B 待启动)
- **A1 ✅** PREREG drafted, 4 rounds revised — `handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md` (922 行 round 4)
- **A2 ✅** 60/20/20 split frozen — adaptation 144 / meta_val 46 / heldout 54; sealed hash `51440807c9ecc5c366d1adb640afcc96fcd227d18e4a35c7f85aaec78475086b`
- **A3 ✅** Notepad pivoted (F-2026-04-25-02 entry)
- **A4 ✅** Dual external audit PASS/PASS (Codex + Gemini, round 4)
- **A5 ✅** Commit gate cleared — commit `913255d`

### Phase B — IN PROGRESS (days 4-10 of 30-day arc)
Detailed plan: `handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md`
Items (all expanded with file paths + acceptance criteria in plan doc):
- **B1 ✅ DONE** JSONL schema v2 (proposal + run-level) — `experiments/minif2f_v4/src/jsonl_schema.rs`; 3 acceptance tests green; legacy `PputResult` shape readable via `RunRecord::from_json` schema_version dispatcher
- **B2 ✅ DONE** C_i 全成本聚合器 — `experiments/minif2f_v4/src/cost_aggregator.rs`; `RunCostAccumulator` records prompt+completion+tool_stdout across all proposals (winning + failed); `test_failed_branches_counted_in_total_cost` PASS; `GenerateResponse.prompt_tokens` exposed for post-hoc API counts
- **B3 ✅ DONE** T_i wall-clock — `experiments/minif2f_v4/src/wall_clock.rs`; `RunWallClock.mark_first_read` (idempotent) + `mark_final_accept` (last-call-wins for B4 post-hoc); `test_wall_clock_first_read_to_final_accept` PASS at strict ≥7100ms (mid-term P0-C fix moved mark_first_read BEFORE prompt construction)
- **B4 ✅ DONE** `pput_verified` vs `pput_runtime` — `experiments/minif2f_v4/src/post_hoc_verifier.rs`; `compute_progress_runtime` / `compute_progress_verified` / `verify_post_hoc`; `make_pput(runtime_accepted, post_hoc_verified, ...)` caller declares both legs explicitly (mid-term P0-A fix); `test_pput_verified_zero_when_lean_rejects` PASS
- B5 11 anti-Goodhart conformance + 5-layer sealing tests — **next entry point** (must pickup deferred P0-B/D/E first per `B5_DEFERRED_FROM_MIDTERM_AUDIT_2026-04-25.md`)
- B6 PPUT-context-leak audit (静态分析 + 运行时门)
- B7 Boot freeze: `pput_accounting_0` in genesis_payload.toml + Trust Root immutability tests; Trust Root manifest must include `cost_aggregator.rs`, `wall_clock.rs`, `post_hoc_verifier.rs`, `jsonl_schema.rs`, `evaluator.rs`, `src/drivers/llm_http.rs` per audit recommendation
- B7-extra **p_0 calibration** (288 paired adaptation-144 × 2 seeds; freeze 进 Trust Root)

### Active background processes
- 无运行中实验 (Phase A 双审已全部完成)
- Codex CLI broker (legacy `pid 348391` → phase-8a-snapshot worktree) — Paper 1 残留, 与 PPUT-CCL 不相关

## What's broken / incomplete

### PPUT-CCL Phase B — to-do (after B2-B4 close)
- B5/B6/B7 + B7-extra: 测试电池 + context-leak gate + Trust Root 未做
- p_0 baseline 未 calibrate (`pput_accounting_0.baseline_regression_rate` 在 genesis_payload.toml 未填)
- Trust Root 集成未实现 (genesis_payload.toml `[trust_root]` SHA-256 表未生成；新 B2-B4 模块需进 manifest)
- conformance test battery 未写: 11 anti-Goodhart + 5-layer sealing + 4 doc/artifact content + 4 lookup-table evasion
- `--mode` flag 未在 evaluator binary 实现 (Phase C 工作；安排在 B5)

### Mid-term dual audit (2026-04-25) deferred items — 必须 B5 起步先解决
binding checklist: `handover/audits/B5_DEFERRED_FROM_MIDTERM_AUDIT_2026-04-25.md`
- **P0-B**: schema v2 emit alignment — evaluator emit 仍是 legacy `PputResult` 而非 v2 `RunAggregate` (no `schema_version`, missing `progress: u8` / `run_id` / `split` / `mode` / model_snapshot 等); B1 dispatcher 把新行误判为 Legacy
- **P0-D**: hybrid_v1 condition `..r2` field-spread 丢弃失败 oneshot 的 C_i (Codex 发现)
- **P0-E**: `RunCostAccumulator::flip_last_failed_to_accepted` 静默饱和 — 应改为 `assert!` 暴露 over-flip wiring bug

### B1 evaluator emit gap (still open after B2-B4 mid-term audit)
- B1 的 `RunAggregate` v2 schema 仍未在任何 emit path 上线；B2-B4 加的字段都打在 legacy `PputResult` 上。P0-B 处理这个差距 — B5 第一步。

### Paper 1 — separate stack
- arXiv 投稿延后 (tag `paper1-v2.1.1` 待 LaTeX 转换 + 元数据)
- 不 gate PPUT-CCL arc; 用户可任意时间回头处理

### Paper 1 — separate stack
- arXiv 投稿延后 (tag `paper1-v2.1.1` 待 LaTeX 转换 + 元数据)
- 不 gate PPUT-CCL arc; 用户可任意时间回头处理

## Open Questions for User
**全部 D1-D4 已 resolved 2026-04-26 default 接受** — 见 `handover/ai-direct/OPEN_DECISIONS_2026-04-26.md` Resolved 区。本 session 关闭；下一 session 在 Phase B B5 起步（先 pickup deferred P0-B/D/E，再写 conformance battery）。

## Next session — first action
1. Read `LATEST.md` (this file) + `B5_DEFERRED_FROM_MIDTERM_AUDIT_2026-04-25.md` + `PHASE_B_IMPLEMENTATION_PLAN.md` § B5
2. Smoke check: `cargo test --workspace` 全部 PASS（B2-B4 close 后基线 = 143/143 parallel green）
3. Pickup deferred P0s (in order): P0-B (schema v2 switch — evaluator emit `RunAggregate` not `PputResult`) → P0-D (hybrid_v1 disable or aggregate) → P0-E (flip assert)
4. Then B5 conformance battery: 11 anti-Goodhart + 5-layer sealing + 4 content + 4 lookup-evasion + isolation + binary purity tests per `PHASE_B_IMPLEMENTATION_PLAN.md` § B5
5. Then B6 (context-leak audit, half day) → B7 (Trust Root + Boot freeze) → B7-extra (p_0 calibration overnight)

## Mid-term audit (2026-04-25) summary
- Codex (274s, 67K char prompt) + Gemini (62s, 67K char prompt): both **CHALLENGE**, high conviction
- Convergent P0s on architectural fragility (Phase-C-safety) + schema v2 drift; Codex-only on first-read placement, hybrid_v1, flip saturation
- **Fixed in B2-B4 commit**: P0-A (make_pput refactored — caller MUST declare runtime + post_hoc legs), P0-C (mark_first_read moved before prompt construction; conformance test back to ≥7100ms strict)
- **Deferred to B5**: P0-B/D/E (binding checklist `B5_DEFERRED_FROM_MIDTERM_AUDIT_2026-04-25.md`)
- **Compute spent on mid-term audit**: ~$3-5 (~67K char prompt × 2). Phase B audit budget remaining for B5/B6/B7/B7-extra transition: ~$10-15.

## Reference (canonical sources of truth)

### PPUT-CCL arc
| 文件 | 用途 |
|---|---|
| `handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md` | 完整 pre-registration spec (round 4 frozen) — 总章法 |
| `handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json` | 三 split frozen output + sealed hash |
| `handover/preregistration/scripts/split_pput_ccl.py` | 可重现 split 生成 (seed = `20260426_PPUT_CCL`) |
| `handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md` | **Phase B 详细实施计划** — 新 session 接手 Phase B 必读 |
| `handover/architect-insights/PPUT_DRIVEN_FULL_PASS_2026-04-25.md` | 架构师 v1 测度论 FULL PASS (verbatim) |
| `handover/architect-insights/GEMINI_DEEPTHINK_FULL_PASS_2026-04-26.md` | 架构师 v2 本体论 FULL PASS (verbatim) |
| `handover/audits/DUAL_AUDIT_PPUT_CCL_VERDICT_ROUND4_2026-04-26.md` | 最终 round-4 PASS/PASS verdict + 4-round 演化总结 |
| `handover/audits/DUAL_AUDIT_PPUT_CCL_VERDICT_2026-04-26.md` | round-1 CHALLENGE verdict (历史) |
| `handover/audits/{CODEX,GEMINI}_PPUT_CCL_AUDIT*_2026-04-26.md` | 4 rounds × 2 auditors = 8 audit files |
| `handover/audits/run_{codex,gemini}_pput_ccl_audit*.{py,sh}` | 可重现双审 (4 rounds) |
| `handover/ai-direct/OPEN_DECISIONS_2026-04-26.md` | 待用户回的决策点 |
| `handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md` (F-2026-04-25-02) | arc launch finding |

### Paper 1 (separate, lower priority)
| 文件 | 用途 |
|---|---|
| `handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md` | Paper 1 final draft (tag `paper1-v2.1.1`) |
| `handover/audits/DUAL_AUDIT_V2_1_VERDICT_2026-04-25.md` | Paper 1 R3 PASS/PASS verdict |

### Repo state
- HEAD: B2-B4 close + mid-term audit close (post `c6087f7`; two new commits — code + audit infrastructure — local SHA stamped at commit time)
- origin/main HEAD: `fd291d7` (Paper 1 hygiene; PPUT-CCL chain `913255d`/`4e4afc7`/`47b3dba`/`2a8921b`/`c6087f7` + B2-B4 close 仍 **local-only**, not pushed)
- Tags pushed: `paper1-v2.1.1`, `archive/art-ii1-v3-abandoned-20260416`
- Branches: `main` (active), 23 archive refs preserved

### Memory entry points (auto-loaded每 session)
- `MEMORY.md` indexes `project_pput_ccl_arc.md` → points here (`LATEST.md`)
- `feedback_phased_checkpoint.md`, `feedback_dual_audit*.md`, `feedback_step_b_protocol.md` are critical for Phase B execution discipline

## Compute spent
- Phase A4 dual audit: ~$15-20 across 4 rounds (~440K Codex tokens + ~310K Gemini tokens)
- B2-B4 mid-term dual audit: ~$3-5 (~67K char prompt × 2; Codex 274s + Gemini 62s)
- Remaining budget: ~$475-480 for Phase B-E (288 p_0 runs + Phase C ablation N=20 × 5 modes + Phase D shadow CCL + Phase E sealed eval + remaining B5/B6/B7 audits)
