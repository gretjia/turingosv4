# Phase 8 Batch Diff Audit Brief — 2026-04-22

**Step-B Phase 1c** (dual external audit before Phase 2 A/B).
**Scope**: 5 commits on `experiment/phase-8a-snapshot-fix` branch.
**27 files changed, +1826 / -18 lines, 41 new tests green.**

## 核心修复（7 sub-tasks + 10 判例）

| sub-task | 判例 | 内容 | commit |
|---|---|---|---|
| 8.A | C-049 | `bus.snapshot()` 从 WalletTool enumerate balances + portfolios（先前硬编码空）| `a4d744c` |
| 8.B | C-048 | `oneshot` 强制走 C-043 mandatory wtool via ephemeral bus + receipt | `4eea992` |
| 8.C | C-067 | `append_oracle_accepted` 加 OracleReceipt capability token + M-1 Predicate trait | `6c36bff` |
| 8.D | C-050 | `decide`/`omega` Mathlib 语境白名单（qualified 允许，bare 禁）| `4eea992` |
| 8.E | C-061 | q-halt 显式状态机 + `EventType::Halt { reason: HaltReason }` | `3ff4f70` |
| 8.F | C-053 | `Tape.reputation_by_author` 计数器 + 暴露到 UniverseSnapshot | `3ff4f70` |
| 8.G | C-055 | `BusConfig.min_class_count_to_broadcast` 广播频率阈值 | `3ff4f70` |

## 先验上下文

**8.A 已过一次 Phase 1c 单独双审**（2026-04-22 earlier）:
- Gemini 5/5 PASS
- Codex 3 PASS + 2 CHALLENGE → 已 address (clone cost 注释 + 2 新测试 + RAII EnvGuard)
- 详见 `handover/audits/EXT_CODEX_PHASE_8A_2026-04-22.md` + `EXT_GEMINI_PHASE_8A_2026-04-22.md`

**本次批审聚焦**：8.B/C/D/E/F/G 的新改动 + 集成是否有隐患。8.A 已过项不要重审，除非发现它与新改动产生冲突。

## 必答 6 项（整批范围）

### 1. 集成一致性（新 vs 旧）
- 8.C 把 `append_oracle_accepted` 签名从 3 参数改 4 参数（加 `&OracleReceipt`）；所有 3 个 call site (evaluator 622/807/829) + 8.B 新增 oneshot call site 都已更新。是否有其他 call site 被遗漏？
- 8.E 在 `halt_and_settle` 开头 call `halt_with_reason(OmegaAccepted)`。是否会与 `settle_portfolios` / `resolve_bounty` 路径产生意外交互？（例如重复 Halt event 风险）
- 8.G 把 threshold 从 env var 改到 `BusConfig.min_class_count_to_broadcast`（决定性实验需要）。所有 7 处 `BusConfig {...}` 构造都已加 field。漏了吗？

### 2. 最小性
- 每个 sub-task 的 diff 是否严格最小？有无 scope creep（顺手改其他东西）？
- 特别检查：`lean4_oracle.rs` 的 `test_decide_tactic_permitted` 被替换为 `test_bare_decide_forbidden` — 是"正确更新语义"还是"偷改测试 hide bug"？

### 3. 正确性
- 8.C `OracleReceipt::validates` sha256 + verdict 检查逻辑是否健壮？
  - 特别：能否有 `PartialOk { confidence: 1.0 }` 但 payload 实际被 Lean reject 的错位？
- 8.D `has_bare_tactic_invocation` 的 word boundary 实现：
  - 字符串中 `"decide"` 会不会误杀？（before=`"`，不是 `.` 或 `:` → 被禁）
  - Unicode identifier 前缀？Lean4 允许 UTF-8 identifier
- 8.E `halt_is_idempotent` 决定："重复 halt 只保留 first reason" — 这是否与 Art. IV "q ∈ {run, halt}" 严格一致？如果 halt 后又有 WallClockCap 触发，真实 q 状态应是什么？
- 8.F `reputation_by_author` 对 self-citation 计数 — `tests/reputation.rs::reputation_self_citation_counts` 明确 assert 加 1。理由写在 C-053 ruling。是否合理？

### 4. 测试充分性
- 41 new tests 覆盖各 sub-task 的 happy path。覆盖**不足**的：
  - 跨 sub-task 集成测试？（e.g. oneshot（8.B）+ OracleReceipt 验证失败（8.C））
  - WAL replay 后 q_state 恢复？（8.E + WAL 互动）
  - halt_and_settle 在 settle_portfolios 中途失败时 halt event 仍正确记录？
  - 老 WAL 文件（无 reputation_by_author 字段）load 不 panic？（`#[serde(default)]` 生效）

### 5. Trojan / 宪法债
- 有无在 BusConfig / TuringBus struct 加了**不必要**的 pub 字段扩大攻击面？
- `src/sdk/predicate.rs` 的 `Predicate` trait 是 M-1 preservation — Paper 1 不使用。是否应该 `#[allow(dead_code)]`？或至少加编译器 suppression 说明？
- 8.G `min_class_count_to_broadcast` 默认 3 是否合理？不同 swarm 大小（N=1 / N=8 / N=50）下是否应该按比例？

### 6. 与 Paper 1 claim 对齐
审 5 条 Paper claim 在本批次后的合规性：
- **C1** Art. IV 100% 实现：新增 q-halt（C-061）+ OracleReceipt（C-067）+ oneshot wtool（C-048）。宪法→代码 mapping 现在满吗？
- **C2** depth-N DAG 真实构造：未改，Phase 9 重测
- **C3** Law 2 守恒：未改（Phase 8.Z/9.B 任务）
- **C4** Art. V runtime veto 实证：未改（deferred C-044）
- **C5** N=244 ΣPPUT：未改（Phase 10b）

## 裁决格式

对每个必答项给 **PASS / CHALLENGE / VETO** + 证据（file:line）。
任一 VETO → 停；任一 CHALLENGE → 必须 address 之后才能 Phase 2。

## 参考资料（所有路径绝对）

- Diff: `/home/zephryj/projects/turingosv4/handover/ai-direct/EXT_AUDIT_PHASE_8_BATCH/phase_8_batch.diff`
- Worktree: `/home/zephryj/projects/turingosv4/.claude/worktrees/phase-8a-snapshot/`
- Checkpoint: `/home/zephryj/projects/turingosv4/.claude/worktrees/phase-8a-snapshot/handover/ai-direct/CHECKPOINT_PHASE_8_2026-04-22.md`
- Cases (10 new): `.claude/worktrees/phase-8a-snapshot/cases/C-{044,045,046,048,049,050,053,055,061,067}_*.yaml`
- Prior 8.A audit: `/home/zephryj/projects/turingosv4/handover/audits/EXT_{CODEX,GEMINI}_PHASE_8A_2026-04-22.md`
- Plan doc: `/home/zephryj/projects/turingosv4/handover/ai-direct/PLAN_FINAL_PHASE_8_TO_PAPER_2026-04-22.md`
- Constitution: `/home/zephryj/projects/turingosv4/constitution.md`

## 输出

Markdown report. 长度自决。每个必答项单独结论。最后给整批 verdict:
- **PASS** → 进 Phase 2 A/B
- **CHALLENGE** → 列举修订项，不阻塞但必须 address
- **VETO** → 停止，返工

## 保存位置

- Codex: `/home/zephryj/projects/turingosv4/handover/audits/EXT_CODEX_PHASE_8_BATCH_2026-04-22.md`
- Gemini: `/home/zephryj/projects/turingosv4/handover/audits/EXT_GEMINI_PHASE_8_BATCH_2026-04-22.md`

**完成前用 `ls -la <path>` 验证文件确实保存（C-066）。**
