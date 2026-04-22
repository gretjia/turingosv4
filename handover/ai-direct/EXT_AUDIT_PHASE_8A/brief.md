# Phase 8.A Diff Audit Brief — 2026-04-22

**Step-B Phase 1c**: Diff-only audit before A/B test.
**Change scope**: `src/bus.rs` — restricted file (requires dual external audit per STEP_B_PROTOCOL).
**Bug being fixed**: C-049 (Codex audit N-2, `handover/audits/EXT_CODEX_2026-04-22.md`)

## 背景

`bus.snapshot()` 硬编码返回空 `balances: HashMap::new()` 和 `portfolios: HashMap::new()`。
后果：agent prompt 永远显示 `Balance: 0 Coins`，即便 wallet 里有 coin。这让 Art. II.2 经济信号（Hayek bounty / TAPE_ECONOMY）从 2026-04-16 起**一直失真**，至少 5 次实验（F-2026-04-18-02, Phase 2/3A/6 tape/market = 0）的结论都不可信。

## 补丁位置

- Worktree: `/home/zephryj/projects/turingosv4/.claude/worktrees/phase-8a-snapshot/`
- Branch: `experiment/phase-8a-snapshot-fix`
- Commit: `54a32af`
- Diff: `src/bus.rs` 16 行，`tests/snapshot_nonempty.rs` 124 行（新建）

## 核心改动

```rust
// BEFORE
balances: HashMap::new(), // filled by wallet tool query
portfolios: HashMap::new(),

// AFTER
let (balances, portfolios) = self.tools.iter()
    .find_map(|t| t.as_any().downcast_ref::<WalletTool>())
    .map(|w| (w.balances.clone(), w.portfolios.clone()))
    .unwrap_or_default();
```

## 测试

4 个新测试全绿（`tests/snapshot_nonempty.rs`）：
1. `snapshot_balances_nonempty_after_genesis` — genesis 后 balances 可见
2. `snapshot_portfolio_appears_after_invest` — invest 后 portfolios 出现（走 evaluator 同路径）
3. `snapshot_reflects_wallet_mutations` — 直接 wallet mutation 可见
4. `snapshot_empty_when_no_wallet_mounted` — tool-less bus 不 panic

## 审计要求

对 diff **ONLY** 审计，**不评审"该不该修这个 bug"**（那是 Phase 0 已过的问题）。

### 必答 5 项

1. **最小性**：改动是否最小？有无 scope creep（顺手改了其他东西）？
2. **正确性**：fix 逻辑是否正确？`.clone()` 每次 snapshot 调用会克隆整个 `HashMap<String, Portfolio>` — 对性能有无可见影响？是否存在 `&dyn Any` downcast 失败的边界情况？
3. **测试充分性**：4 个测试是否覆盖本修复的关键路径？有何遗漏？尤其：
   - `halt_and_settle` 后 balance 更新是否被测到？
   - multiple agents with different portfolios？
   - market 未解决但 portfolio 已存在的情况？
4. **Trojan 检查**：diff 里有无副作用超出 snapshot 函数的改动？注释更新有无暗藏规则变更？
5. **宪法债**：此改动是否引入新的违宪风险？（例如：把 WalletTool 状态暴露给 snapshot 是否违反 Art. III.4 Goodhart 屏蔽？）

### 裁决格式

PASS / CHALLENGE / VETO。任一 VETO → 停止合并，返工。

### 参考资料

- `/home/zephryj/projects/turingosv4/handover/audits/EXT_CODEX_2026-04-22.md` § N-2（原始发现）
- `/home/zephryj/projects/turingosv4/handover/ai-direct/STEP_B_PROTOCOL.md` Phase 1c
- `/home/zephryj/projects/turingosv4/cases/C-049_*.yaml`（待立）
- `/home/zephryj/projects/turingosv4/src/sdk/snapshot.rs`（UniverseSnapshot 定义）
- `/home/zephryj/projects/turingosv4/src/sdk/tools/wallet.rs`（WalletTool 定义）

## 输出要求

Markdown 报告，长度自决。结论清晰（PASS/CHALLENGE/VETO + 理由）。
