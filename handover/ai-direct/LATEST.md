# TuringOS v4 — Handover State
**Updated**: 2026-04-26 (B1 close)
**Session Summary**: PPUT-CCL arc Phase A 完整收尾 — PREREG + split + 4-round dual-audit PASS/PASS + A5 commit gate (commit `913255d`). Phase B B1 (JSONL schema v2) 半天破冰完成，3/3 acceptance tests PASS，全套 131/131 parallel green。Phase B B2 (cost aggregator, ~1 day) cleared as next entry point. Paper 1 v2.1.1 arXiv 投稿延后, 但仓库就绪 (tag `paper1-v2.1.1` 已推 origin).

> **新 session 入口**: 读这个文件 + `handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md` § 6 + `handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md` + `handover/ai-direct/OPEN_DECISIONS_2026-04-26.md`. 这 4 个文件足以无 context 接手当前工作。

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
- **B1 ✅ DONE** JSONL schema v2 (proposal + run-level) — `experiments/minif2f_v4/src/jsonl_schema.rs`; 3 acceptance tests green; legacy `PputResult` shape readable via `RunRecord::from_json` schema_version dispatcher; evaluator emit wiring deferred to B2/B3/B4 (no fields to populate yet)
- B2 C_i 全成本聚合器 (all agents × branches × failures × tool stdout) — **next entry point**
- B3 T_i wall-clock = first-read → final-accept
- B4 `pput_verified` vs `pput_runtime` 双字段
- B5 11 anti-Goodhart conformance + 5-layer sealing tests
- B6 PPUT-context-leak audit (静态分析 + 运行时门)
- B7 Boot freeze: `pput_accounting_0` in genesis_payload.toml + Trust Root immutability tests
- B7-extra **p_0 calibration** (288 paired adaptation-144 × 2 seeds; freeze 进 Trust Root)

### Active background processes
- 无运行中实验 (Phase A 双审已全部完成)
- Codex CLI broker (legacy `pid 348391` → phase-8a-snapshot worktree) — Paper 1 残留, 与 PPUT-CCL 不相关

## What's broken / incomplete

### PPUT-CCL Phase B — to-do (after B1 close)
- B2-B7 + B7-extra: 全套 evaluator emit-path 改造 + 测试电池 + Trust Root 未做
- p_0 baseline 未 calibrate (`pput_accounting_0.baseline_regression_rate` 在 genesis_payload.toml 未填)
- Trust Root 集成未实现 (genesis_payload.toml `[trust_root]` SHA-256 表未生成)
- conformance test battery 未写: 11 anti-Goodhart + 5-layer sealing + 4 doc/artifact content + 4 lookup-table evasion
- `--mode` flag 未在 evaluator binary 实现
- B1 的 `ProposalRow` / `RunAggregate` 还没在 evaluator 任何 emit path 上线 — 由 B2 负责 wire-in

### Paper 1 — separate stack
- arXiv 投稿延后 (tag `paper1-v2.1.1` 待 LaTeX 转换 + 元数据)
- 不 gate PPUT-CCL arc; 用户可任意时间回头处理

## Open Questions for User
**全部 D1-D4 已 resolved 2026-04-26 default 接受** — 见 `handover/ai-direct/OPEN_DECISIONS_2026-04-26.md` Resolved 区。本 session 关闭；下一 session 在 Phase B B1 起步（JSONL schema v2）。

## Next session — first action
1. Read `LATEST.md` (this file) + `PHASE_B_IMPLEMENTATION_PLAN.md` § B2
2. Smoke check: `cargo test --workspace` 全部 PASS（B1 close 后基线 = 131/131 parallel green；env-var flake 已 fix，无需 --test-threads=1）
3. Start B2 (cost aggregator, est. 1 day): 建 `src/cost_aggregator.rs` + 改 `experiments/minif2f_v4/src/bin/evaluator.rs` 主循环 + 各 tool `execute` 加 `tool_stdout` 返回
4. B2 完成判据: `cargo test test_failed_branches_counted_in_total_cost` PASS + 3 个历史 Phase 1 run 手动 spot-check (重算 C_i 与 jsonl emit 一致)
5. B2 完成后：B3 (wall-time, half day, 可与 B2 并行) → B4 → B5 → B6 → B7 → B7-extra (overnight calibration)

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
- HEAD: B1 close commit (post `47b3dba`; SHA stamped at commit time)
- origin/main HEAD: `fd291d7` (Paper 1 hygiene; PPUT-CCL chain `913255d`/`4e4afc7`/`47b3dba` + B1 close 仍 **local-only**, not pushed)
- Tags pushed: `paper1-v2.1.1`, `archive/art-ii1-v3-abandoned-20260416`
- Branches: `main` (active), 23 archive refs preserved

### Memory entry points (auto-loaded每 session)
- `MEMORY.md` indexes `project_pput_ccl_arc.md` → points here (`LATEST.md`)
- `feedback_phased_checkpoint.md`, `feedback_dual_audit*.md`, `feedback_step_b_protocol.md` are critical for Phase B execution discipline

## Compute spent
- Phase A4 dual audit: ~$15-20 across 4 rounds (~440K Codex tokens + ~310K Gemini tokens)
- Remaining budget: ~$480-485 for Phase B-E (288 p_0 runs + Phase C ablation N=20 × 5 modes + Phase D shadow CCL + Phase E sealed eval)
