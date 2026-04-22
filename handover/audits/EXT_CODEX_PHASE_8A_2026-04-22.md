# Codex Phase 8.A Diff Audit
**Date**: 2026-04-22
**Subagent**: codex:codex-rescue
**Branch**: `experiment/phase-8a-snapshot-fix` @ `54a32af`
**File save**: Codex CLI sandbox blocked write; Claude persisted this transcript manually per C-066.

---

## OVERALL VERDICT: **CHALLENGE**

| # | Item | Verdict | Claude synthesis |
|---|---|---|---|
| 1 | Minimality | **PASS** | diff confined to `snapshot()` wallet fill-in plus one focused regression test file |
| 2 | Correctness | **CHALLENGE** | logic matches existing `WalletTool` downcast contract, but full-map `.clone()` cost on a per-tick `snapshot()` path is unmeasured |
| 3 | Test sufficiency | **CHALLENGE** | the 4 new tests miss `snapshot()` state after `halt_and_settle()` and do not cover simultaneous distinct portfolios across agents |
| 4 | Trojan check | **PASS** | no side-effecting runtime path changed beyond returning wallet state from `snapshot()` |
| 5 | Constitutional debt | **PASS** | this populates preexisting wallet fields and does not expose verifier or metric internals covered by Art. III.4 |

---

## 要补救的 2 条 CHALLENGE

**CHALLENGE-A (Correctness/clone cost)**:
- 全 HashMap clone 在 per-tick snapshot 路径上未测量成本
- 当前系统规模（N≤32 agents）下可能无感，但生产前应有明确上限 justification 或基准

**CHALLENGE-B (Test coverage gap)**:
- 缺 `halt_and_settle()` 后 snapshot balance 反映 payout 的测试
- 缺 multi-agent 同时持有不同 portfolio 的快照测试

---

## Dual Audit 对比

- **Gemini**: 5/5 PASS（`handover/audits/EXT_GEMINI_PHASE_8A_2026-04-22.md`）
- **Codex**: 3 PASS + 2 CHALLENGE（本文档）

按 `feedback_dual_audit_conflict.md`（保守胜出）→ 总裁决 **CHALLENGE**。

后续：addressment 之后可以合并（CHALLENGE 不是 VETO，是"需修订"），不需要再次完整双审，只需 local test + 自审。

---

## Meta 观察

Codex 声称 `/home/zephryj/projects/turingosv4/handover/audits/EXT_CODEX_PHASE_8A_2026-04-22.md` 由于 "read-only filesystem in this sandbox" 无法写入。这与上次（同一环境）能够写入 `EXT_CODEX_2026-04-22.md` 的历史不一致。待调查是否 Codex 配置变更。

**C-066 应用**：Claude 验证"文件已写"的 claim 失败 → 改由 Claude 手动保存 Codex 的 result 字段内容作为审计输出。VETO 不触发（verdict 内容本身 credible，仅保存机制问题）。
