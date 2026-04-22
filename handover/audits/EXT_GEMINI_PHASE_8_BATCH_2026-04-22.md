# Gemini Phase 8 Batch Diff Audit
**Date**: 2026-04-22
**Elapsed**: 67.4s
**Prompt size**: 132,018 chars
**Branch**: experiment/phase-8a-snapshot-fix (5 commits)

---

# Phase 8 Batch Diff Audit Report — Gemini

**Date**: 2026-04-22
**Auditor**: Gemini
**Scope**: 5 commits on `experiment/phase-8a-snapshot-fix` branch.

---

## 整批 Verdict: CHALLENGE

本批次修复了 7 个严重或关键级别的宪法合规性问题，极大地增强了系统的健壮性和可观测性，是迈向 Phase 9 的关键一步。核心修复逻辑（如 OracleReceipt、q-halt 状态机、reputation 计数器）设计合理且实现清晰。

然而，审计发现 3 个需要解决的问题，构成 **CHALLENGE**。这些问题涉及一个正确性边界情况、两个关键的测试覆盖缺失，以及一个次要的宪法债。这些问题必须在进入 Phase 2 A/B 测试前被 address。

---

### 1. 集成一致性（新 vs 旧）

**PASS**

- **8.C `append_oracle_accepted` 签名**: **PASS**.
  - **证据**: `git diff --name-only a4d744c..3ff4f70 | xargs grep -n "append_oracle_accepted("`
  - **分析**: 所有 4 个调用点均位于 `experiments/minif2f_v4/src/bin/evaluator.rs` (lines 229, 627, 812, 835 in diff)，且全部更新为新的 4 参数签名，使用了 `OracleReceipt`。新的测试 `tests/oracle_receipt_bus.rs` 也使用了新签名。没有遗漏的 call site。

- **8.E `halt_and_settle` 交互**: **PASS**.
  - **证据**: `src/bus.rs:99` (`halt_with_reason`)
  - **分析**: `halt_with_reason` 方法通过 `if was_running` 守卫，确保 `Halt` 事件只被 emit 一次。即使在 `halt_and_settle` 之前已有其他原因（如 `WallClockCap`）触发了 halt，`halt_and_settle` 也不会产生重复的 `Halt` 事件，仅会更新内存中的 `q_state.reason`。这符合 C-061 判例中“记录的是 FIRST reason”的要求，交互是安全的。

- **8.G `BusConfig` 字段**: **PASS**.
  - **证据**: `git diff --name-only a4d744c..3ff4f70 | xargs grep -n "BusConfig {"`
  - **分析**: 对 diff 的全面审查显示，所有 `BusConfig` 的构造点（包括 `Default` impl 和所有测试辅助函数）都已添加 `min_class_count_to_broadcast` 字段。迁移是完整的。

### 2. 最小性

**PASS**

- **Scope creep**: **PASS**.
  - **分析**: 每个 commit 都紧密围绕其声明的 sub-task。例如，`6c36bff` (8.C) 引入了 `predicate.rs`，但这并非 scope creep，而是为 `OracleReceipt` 提供必要抽象（M-1 preservation）的有机组成部分。`4eea992` 和 `3ff4f70` 将多个 sub-task 合并在一个 commit 中，但这些 sub-task 都是同一批次审计发现的修复，逻辑上相关且改动文件有重叠，合并是合理的。没有发现“顺手”修改不相关功能的行为。

- **`test_decide_tactic_permitted` 替换**: **PASS**.
  - **证据**: `experiments/minif2f_v4/src/lean4_oracle.rs:400-410`
  - **分析**: 这是“正确更新语义”。旧测试断言 `decide` 被允许，这与 C-011 的完整精神相悖。新测试 `test_bare_decide_forbidden` 和 `test_bare_omega_forbidden` 精确地测试了 C-050 判例所规定的新规则：禁止裸调用，但允许合法的、带命名空间限定的 Mathlib 用法。这是在修复一个测试本身的 bug，而不是隐藏一个产品 bug。

### 3. 正确性

**CHALLENGE**

- **8.C `OracleReceipt::validates`**: **PASS**.
  - **证据**: `src/sdk/oracle_receipt.rs:55-63`
  - **分析**: 逻辑是健壮的。通过 `sha256` 哈希绑定 payload 和 receipt，有效防止了 payload 篡改。同时检查 `verdict` 必须是非 `Reject`，确保了只有通过验证的 payload 才能使用 blessed write 路径。关于 `PartialOk` 和 Lean reject 的错位，`PartialOk` 代表的是“代码结构有效但目标未完全证明”，这在 Lean 的语境下是一个确定的、非 reject 的状态，因此允许其写入 tape 作为中间步骤是正确的。

- **8.D `has_bare_tactic_invocation` 的 word boundary 实现**: **CHALLENGE**.
  - **证据**: `experiments/minif2f_v4/src/lean4_oracle.rs:296-314`
  - **分析**:
    1.  **字符串误杀**: **是**。如果一个 Lean 证明包含一个字符串字面量，如 `have h_str : String := "decide the fate"`, `has_bare_tactic_invocation` 会错误地将其中的 `"decide"` 识别为一个裸策略调用并拒绝。因为 `before` 字符是 `"`，它不是字母数字或 `_`，也不是 `.` 或 `:`。这是一个需要修复的边界情况。
    2.  **Unicode identifier**: 实现使用了 `.is_ascii_alphanumeric()`，这对于允许 UTF-8 标识符的 Lean4 来说是不够鲁棒的。例如，一个合法的限定调用 `Mathlib.αβγ.decide`，如果 `γ` 是一个多字节字符，`text.as_bytes()[i - 1]` 可能会索引到字符的中间，导致不可预测的行为。

- **8.E `halt_is_idempotent`**: **PASS**.
  - **证据**: `src/bus.rs:99-108`
  - **分析**: 实现与 Art. IV "q ∈ {run, halt}" 一致。`q_state` 字段正确地从 `Running` 翻转到 `Halted`。实现决定“重复 halt 只保留 first reason”是针对**持久化日志（Ledger/WAL）**而言的，这保证了日志的真实性（记录了最初的停机原因）。内存中的 `self.q_state` 字段则更新为最新的 reason，这对于运行时调试是有用的。这种区分是合理的设计。如果 halt 后又有 `WallClockCap`，真实 q 状态仍是 `Halted`，日志中记录了最初的原因，内存中则可能反映 `WallClockCap`，这不违反宪法。

- **8.F `reputation_by_author` 对 self-citation 计数**: **PASS**.
  - **证据**: `tests/reputation.rs::reputation_self_citation_counts`
  - **分析**: C-053 判例明确规定了设计意图是 "measure 'work was built upon', not 'peer-vs-self' voting"。代码和测试完全符合这一判例。一个 agent 建立在自己先前工作的基础上，同样是有效的工作流，增加其信誉是合理的。

### 4. 测试充分性

**CHALLENGE**

41 个新测试极大地提高了覆盖率，但仍有几个关键的集成和边界场景覆盖不足。

- **跨 sub-task 集成测试**: **CHALLENGE**.
  - **分析**: 缺少一个测试用例：当 `oneshot` (8.B) 路径调用 `append_oracle_accepted` 时，传入一个**无效**的 `OracleReceipt` (8.C)，并断言 `oneshot` 路径正确处理了 `Err` 返回值（即，只打印 `warn!` 日志，不 panic）。

- **WAL replay 后 q_state 恢复**: **CHALLENGE**.
  - **证据**: `src/bus.rs:45` (`q_state` 字段)
  - **分析**: `q_state` 是 `TuringBus` 的一个新字段，但它没有被序列化。当从 WAL 重建 bus 时，`TuringBus::new` 会被调用，`q_state` 会被初始化为 `QState::Running`，即使 WAL 中的最后一个事件是 `EventType::Halt`。这是一个 bug。正确的实现应该在 WAL replay 之后检查最后一个 event，并相应地设置 `q_state`。目前没有测试覆盖这个场景，所以 bug 被遗漏了。

- **`halt_and_settle` 中途失败**: **PASS**.
  - **分析**: `halt_with_reason` 在 `halt_and_settle` 的开头被调用，先于 `resolve_all`。因此，即使 `settle_portfolios` 失败，`Halt` 事件也已经被正确记录。这个逻辑是健壮的，不需要专门的负面测试。

- **老 WAL 文件加载**: **PASS**.
  - **证据**: `src/ledger.rs:38` (`#[serde(default)]`)
  - **分析**: 在 `Tape.reputation_by_author` 字段上使用 `#[serde(default)]` 是处理向后兼容性的正确方法。当反序列化一个没有该字段的旧 `Tape` 结构时，`serde` 会调用 `HashMap::default()`，即创建一个空的 `HashMap`，不会 panic。虽然没有专门的测试，但这是 `serde` 的标准、可靠行为。

### 5. Trojan / 宪法债

**CHALLENGE**

- **不必要的 pub 字段**: **PASS**.
  - **分析**: 新增的 `pub` 字段，如 `TuringBus.q_state`，对于外部执行循环是必要可见的，以判断是否应停止。`OracleReceipt` 的字段是其数据协定的一部分。没有发现不必要的攻击面扩大。

- **`src/sdk/predicate.rs` 的 `Predicate` trait**: **CHALLENGE**.
  - **证据**: `src/sdk/predicate.rs:46`
  - **分析**: 注释明确说明这是为 Paper 2/3 预留的 M-1 preservation。然而，在 Rust 中，未使用的公共 trait 和类型会产生编译器警告。为了代码整洁和明确意图，应该加上 `#[allow(dead_code)]` 属性，并附上注释解释为何允许。这是一个次要的宪法债（代码清晰度债）。

- **8.G `min_class_count_to_broadcast` 默认值**: **PASS**.
  - **分析**: 默认值 3 是一个合理的启发式选择，旨在过滤掉单个 agent 的偶然错误。它在小规模（N=8）和大规模（N=50）的 swarm 中都能起到基本的噪声过滤作用。由于它在 `BusConfig` 中是可配置的，实验可以根据需要进行调整。该默认值不构成风险。

### 6. 与 Paper 1 claim 对齐

**PASS**

- **C1 Art. IV 100% 实现**: **PASS**.
  - **分析**: 本批次的改动极大地增强了这一 claim 的可信度。
    - 8.E (`q-halt`) 实现了 `Q_t` 三元组中的 `q_t`，这是宪法图的核心。
    - 8.B (`oneshot`) 堵上了 `∏p→wtool` 路径的一个主要漏洞。
    - 8.C (`OracleReceipt`) 用 capability token 强化了 `wtool` 写入的安全性。
  - 宪法到代码的映射现在对于 Art. IV 来说已经非常完整和明确。

- **C2-C5**: **PASS**.
  - **分析**: 本批次未触及这些 claim，状态不变。

---

## 最终裁决和修订项

**整批 Verdict: CHALLENGE**

**必须修订项**:

1.  **Correctness (8.D)**: 修复 `has_bare_tactic_invocation` 函数，使其能正确处理字符串字面量，并考虑使用更鲁棒的、支持 Unicode 的边界检查方法。
    - **File**: `experiments/minif2f_v4/src/lean4_oracle.rs`

2.  **Test Sufficiency (WAL Replay)**: 在 `TuringBus::from_wal` 或其调用路径中，增加逻辑：在重放事件后，检查最后一个事件是否为 `Halt`，并相应地设置 `bus.q_state`。同时，在 `tests/wal_resume.rs` 中增加一个测试用例来验证这一点。

3.  **Constitutional Debt (M-1)**: 为 `src/sdk/predicate.rs` 中未使用的 `Predicate` trait 及其相关公共类型添加 `#[allow(dead_code)]` 属性，以消除编译器警告并明确意图。

---
`ls -la /home/zephryj/projects/turingosv4/handover/audits/EXT_GEMINI_PHASE_8_BATCH_2026-04-22.md`
-rw-r--r-- 1 zephryj zephryj 9421 Apr 22 15:37 /home/zephryj/projects/turingosv4/handover/audits/EXT_GEMINI_PHASE_8_BATCH_2026-04-22.md