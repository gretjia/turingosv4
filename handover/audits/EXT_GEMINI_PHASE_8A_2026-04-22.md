# Gemini Phase 8.A Diff Audit
**Date**: 2026-04-22
**Elapsed**: 42.5s
**Prompt size**: 48,861 chars
**Branch**: experiment/phase-8a-snapshot-fix @ 54a32af

---

# Diff Audit Report: Phase 8.A (C-049 `bus.snapshot()` fix)

**Date**: 2026-04-22
**Auditor**: `Claude-4.7-Opus-Audit-Self-Correction-Mode`
**Subject**: `experiment/phase-8a-snapshot-fix`, commit `54a32af`
**Scope**: `src/bus.rs` diff, `tests/snapshot_nonempty.rs` new file

---

## **裁决: PASS**

此补丁是针对严重 Bug C-049 的一个最小、正确且经过充分测试的修复。它解决了 Art. II.2 经济信号失真的核心问题，没有引入范围蔓延或明显的宪法债。性能影响（`.clone()`）是为保证快照不变性而付出的必要代价，在当前系统规模下可以接受。

---

## 详细审计 (必答 5 项)

### 1. 最小性

**PASS**. 改动是最小化的。

- **代码改动**: 仅限于 `src/bus.rs` 的 `snapshot()` 函数内部。它用一个 5 行的迭代器链替换了两行硬编码的 `HashMap::new()`。没有触及函数签名、其他函数或模块。
- **注释**: 新增的注释清晰地解释了“为什么”要改（C-049, Art. II.2），直接服务于本次修复，不属于范围蔓延。
- **测试**: 新增的测试文件 `tests/snapshot_nonempty.rs` 完全专注于验证此修复，没有“顺手”修复其他不相关的问题。

结论：无 scope creep。改动严格限制在修复 C-049 所需的最小范围内。

### 2. 正确性

**PASS**. 修复逻辑正确，边界情况已处理。

- **核心逻辑**: `self.tools.iter().find_map(|t| ...).map(|w| ...).unwrap_or_default()` 链是 Rust 中处理此类问题的惯用且健壮的方法。
    - `find_map` 正确地遍历 `Vec<Box<dyn TuringTool>>`，并安全地尝试将每个 tool 向下转型为 `WalletTool`。
    - `map` 在找到 `WalletTool` 后执行克隆操作。
    - `unwrap_or_default` 优雅地处理了 `find_map` 返回 `None`（即总线上没有挂载 `WalletTool`）的情况，返回默认的空 `(HashMap, HashMap)`，这与修复前的行为（对于空钱包场景）一致，并由 `snapshot_empty_when_no_wallet_mounted` 测试覆盖。
- **性能影响 (`.clone()`)**:
    - 每次调用 `snapshot()` 都会深度克隆 `balances` 和 `portfolios` 两个 `HashMap`。对于 `portfolios`（一个嵌套的 `HashMap`），这可能涉及相当大的数据量，尤其是在大量 agent 和大量市场的情况下。
    - **但这对于正确性是必要的**。`UniverseSnapshot` 的核心设计原则是**不可变快照**（Art. III.3）。返回引用（`&`）会破坏这种隔离，允许 agent prompt 生成逻辑（或其他下游消费者）意外地看到 `WalletTool` 状态在单个 agent 回合内的变化，或引入生命周期复杂性。
    - 结论：性能开销是为实现宪法规定的“快照隔离”而付出的合理代价。在当前实验规模（N<100 agents）下，这种开销是可见的但并非瓶颈。如果未来系统扩展到数千个 agent，此处的性能可能需要重新评估（例如使用 `im::HashMap` 等持久化数据结构），但对于当前修复而言，`.clone()` 是正确选择。
- **`&dyn Any` downcast 失败**: 已被 `find_map` 的逻辑完美处理。如果一个 tool 不是 `WalletTool`，`downcast_ref` 返回 `None`，`find_map` 会继续尝试下一个 tool。如果所有 tool 都不是 `WalletTool`，`find_map` 最终返回 `None`，由 `unwrap_or_default` 处理。不存在 panic 风险。

### 3. 测试充分性

**PASS**. 4 个新测试覆盖了关键路径和核心回归场景。

- **已覆盖的关键路径**:
    1.  **Genesis 后状态**: `snapshot_balances_nonempty_after_genesis` 确认了初始资金注入能被快照捕获。这是最基本的 happy path。
    2.  **交易后状态**: `snapshot_portfolio_appears_after_invest` 模拟了 `evaluator` 的投资路径，验证了投资行为（余额减少、投资组合出现）能正确反映在快照中。
    3.  **状态一致性**: `snapshot_reflects_wallet_mutations` 确认了快照反映的是 `WalletTool` 的**当前**状态，而不是某个陈旧的缓存。
    4.  **边界情况**: `snapshot_empty_when_no_wallet_mounted` 确保了在没有经济系统（无 `WalletTool`）的环境中，`snapshot()` 不会 panic，这对于 tool-less 的单元测试至关重要。

- **提问项分析 (遗漏检查)**:
    - **`halt_and_settle` 后 balance 更新**: **间接覆盖**。`snapshot_reflects_wallet_mutations` 测试表明，任何对 `WalletTool` 内部状态的直接修改都会被快照捕获。`halt_and_settle` 最终也会通过调用 `wallet.credit()` 等方法来修改 `WalletTool` 状态。因此，虽然没有专门的 `settle` 测试，但现有测试已经验证了其底层机制。增加一个显式的 `settle` 测试会更好，但其缺失不构成 VETO 的理由。
    - **Multiple agents with different portfolios**: **部分覆盖**。测试设置了 `Agent_0` 和 `Agent_1`。`snapshot_portfolio_appears_after_invest` 测试了 `Agent_1` 投资后其投资组合的变化，但没有断言 `Agent_0` 的投资组合保持不变。这是一个微小的遗漏，但鉴于修复逻辑是克隆整个 `portfolios` map，该逻辑不太可能只对一个 agent 正确而对另一个 agent 错误。
    - **Market 未解决但 portfolio 已存在**: **已覆盖**。`snapshot_portfolio_appears_after_invest` 正是测试了这种情况。投资发生后，市场尚未解决，测试断言此时投资组合中必须出现相应的头寸。

结论：测试是充分的，覆盖了所有核心逻辑和风险点。

### 4. Trojan 检查

**PASS**. diff 中没有发现超出 `snapshot` 函数范围的副作用或隐藏改动。

- **函数签名**: `snapshot(&self)` 保持只读，不接受 `&mut self`。
- **实现**: 函数体内没有修改任何 `self` 的状态。它只是从 `self.tools` 读取数据。
- **注释**: 注释内容与代码改动直接相关，解释了修复的原因和背景，没有引入新的规则或歧义。
- **测试代码**: 测试辅助函数 `do_invest` 被明确标记为“mirroring evaluator.rs”，其逻辑透明，仅用于在测试中设置 `WalletTool` 的状态。

结论：此 diff 是“所见即所得”的直接修复，无隐藏行为。

### 5. 宪法债

**PASS**. 此改动**减少**了宪法债，没有引入新的违宪风险。

- **修复了 Art. II.2 违宪状态**: 之前的代码硬编码返回 0 余额，直接违反了 Art. II.2（经济信号必须对 agent 可见）的核心要求。此修复使经济信号得以正确传递，是**合宪性修复**。
- **关于 Art. III.4 (Goodhart 屏蔽)**: 将真实的 `WalletTool` 状态暴露给 `snapshot` **不违反** Goodhart 屏蔽。
    - Art. III.4 旨在防止 agent 直接操纵其**评估指标**。Agent 的钱包余额是其在系统内的**资源或状态**，而不是评估其贡献的最终指标。
    - 向 agent 提供其准确的资源信息是其做出理性经济决策的前提。隐藏这些信息反而会破坏经济模拟的有效性。
    - 正确的 Goodhart 屏蔽应该作用于评估函数本身（例如，不应简单地将“最终余额”作为奖励），而不是作用于 agent 感知自身状态的能力。

结论：此改动是向着更符合宪法精神的方向迈出的一步，解决了已知的严重违宪问题，且没有引入新的宪法风险。