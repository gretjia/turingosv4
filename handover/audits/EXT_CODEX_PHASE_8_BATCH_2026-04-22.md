# Codex Phase 8 Batch Diff Audit
**Date**: 2026-04-22
**Subagent**: codex:codex-rescue
**Branch**: `experiment/phase-8a-snapshot-fix` (5 commits: a4d744c → 6df4e88)
**File save**: Codex CLI sandbox blocked write; Claude persisted this transcript manually per C-066.

---

## OVERALL VERDICT: **VETO**

---

| # | Item | Verdict | One-line |
|---|---|---|---|
| 1 | Integration consistency | CHALLENGE | halt_with_reason + q_state can diverge; live state vs persisted reason 不一致 |
| 2 | Minimality / no test tampering | PASS | — |
| 3 | Correctness | **VETO** | OracleReceipt forgeable by any in-process caller; step mode receipt blesses `tactic` but oracle verified `prefix`; has_bare_tactic string/Unicode holes |
| 4 | Test sufficiency | CHALLENGE | 缺 oneshot × bad receipt 集成；WAL replay 无 halt 路径；pre-resolution halt 失败；backward compat serde 无测试 |
| 5 | Trojan / constitutional debt | CHALLENGE | public surface 过宽（OracleReceipt 可自由构造 / Predicate trait 仍 unused），min_count=3 默认未跨 swarm 尺寸辩护 |
| 6 | Paper C1 Art. IV | **VETO** | oneshot 用 `TuringBus::new(...)` with `wal: None`；event 丢失；drops in-memory after return |

---

## Q3 正确性 VETO 详析

Evidence: `src/sdk/oracle_receipt.rs:20-25, 27-45, 51-59`; `src/bus.rs:215-224`; `experiments/minif2f_v4/src/bin/evaluator.rs:816-826, 858-863`; `experiments/minif2f_v4/src/lean4_oracle.rs:320-335`; `src/ledger.rs:85-95`; `tests/reputation.rs:92-100`.

- **OracleReceipt 仍可伪造**：`pub struct` 字段 + `pub fn for_lean4_complete/for_lean4_partial` 公开构造，validates() 只检验 self-asserted hash + 非 Reject verdict。任何 in-process caller 可构造任意 payload + 任意 verdict 的合法 receipt。"capability token" 名实不副 — 真正的 capability 需要不可伪造（签名 / 注册 nonce）。
- **step 模式 receipt 绑错东西**：`verify_partial(prefix)` 验证的是 `prefix = tape_chain_so_far + tactic`，receipt 只 bind `tactic` (payload_hash = sha256(tactic))。一个在 context A 下合法的 `(tactic, PartialOk)` 对可被 replay 到 context B（不同 parent chain），proof semantics 被破坏。
- `has_bare_tactic_invocation()` raw byte 扫描：Unicode identifier 不安全（UTF-8 续字节可能被误读）；字符串字面量 / 注释上下文仍有 false positive。

## Q6 Paper C1 VETO 详析

Evidence: `constitution.md:457-524`; `experiments/minif2f_v4/src/bin/evaluator.rs:221-240`; `src/bus.rs:91-103`; `experiments/minif2f_v4/src/bin/evaluator.rs:639-674, 804-847`.

- swarm `complete` / `step` 路径现在接近 Art. IV topology，**但 oneshot 仍写入 ephemeral `TuringBus::new(...)` with `wal: None`**，never records `halt`, drops in-memory state immediately after returning `PputResult`.
- 8.B 声称的 "Art. IV compliant"：append_oracle_accepted 被调用，但 Halt event 从未触发（oneshot 不调 halt_and_settle），且因为 wal:None 所以即使 append 的 ledger event 也是 memory-only，bus 出作用域即丢。
- 配合 forgeable receipt，Paper C1 "Art. IV 100% 实现" claim **不成立**。

## Additional Risks

- **reputation 仍不 agent-visible**：UniverseSnapshot 增字段但 `src/sdk/prompt.rs:11-20, 48-79` 渲染模板只含 market_ticker + balance + errors + search；evaluator 传 `snap.market_ticker` 和 `snap.get_balance` 没有 reputation。8.F 的 "信誉累积对 agent 可见" 未真正达成（仅 structurally 在 snapshot，不在 prompt）。
- `min_class_count_to_broadcast = 3` 默认值未针对 N=1 与 N=50 做辩护（N=1 时 3 意味着任何错误都不广播）。

## 推荐修订（VETO 解锁前）

1. **Make receipt issuance come only from oracle execution**，**not from public constructors**；at minimum make fields private and bind receipt to context (`parent`, theorem/problem, verdict domain) before Phase 2.
2. **If oneshot must count under Art. IV**, back it with durable WAL-backed bus state and record `halt`, not just ephemeral append.
3. Unicode / comment / string literal regression tests for `has_bare_tactic_invocation()`.
4. Old-WAL deserialization + halt-failure replay tests.
5. prompt.rs 集成 reputation 到 agent-visible template。

---

## Verdict 汇总 vs Gemini

| Finding | Gemini | Codex |
|---|---|---|
| OracleReceipt forgeability | PASS | **VETO** |
| oneshot Art. IV durable | (not flagged) | **VETO** |
| has_bare_tactic_invocation holes | CHALLENGE | CHALLENGE (in VETO cluster) |
| WAL replay q_state | CHALLENGE | CHALLENGE |
| reputation not in prompt | (not flagged) | CHALLENGE (additional risk) |
| Predicate trait dead_code | CHALLENGE | CHALLENGE (public surface) |
| halt_with_reason divergence | (not flagged) | CHALLENGE |
| Step mode receipt context binding | (not flagged) | **VETO** |

**保守裁决** (feedback_dual_audit_conflict)：任一 VETO 胜出 → **整批 VETO**。

---

## Meta 观察（C-066）

Codex sandbox 再次阻塞 file write（第 3 次同类事件）。Claude 手动持久化 verdict 内容至本文件。C-066 ruling 生效良好 — verdict 内容 credible，持久化机制问题已知。建议 Phase 10c 修 codex CLI sandbox 配置（或者接受并在 PLAN 里明确"Codex CLI sandbox 预期不能 write，Claude 手工持久化"为稳态）。
