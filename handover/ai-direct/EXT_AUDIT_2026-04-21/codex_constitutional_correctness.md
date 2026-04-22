# Codex 外部审计任务 — 宪法对齐 & 代码细节

**项目**：TuringOS v4，Rust 2021，tokio + serde_json
**commit**：`e0a75ec`（Phase 7 Turing per-tactic δ-step 合并）
**任务性质**：只读代码审计。不修改文件，不运行 cargo build。允许 grep / read / find。
**返回**：Markdown 报告。每条结论必须带 `file:line` 证据。输出长度自行决定，够用即止。

---

## 上下文位置（审计必读）

项目根目录：`/home/zephryj/projects/turingosv4/`

| 类型 | 路径 |
|---|---|
| 宪法（唯一 ground truth） | `constitution.md` |
| 判例库（35 条已立 + 本次建议的 C-044~C-047） | `cases/C-*.yaml`、`cases/V3_LESSONS.md` |
| 微内核 Rust 代码 | `src/bus.rs`、`src/kernel.rs`、`src/ledger.rs`、`src/wal.rs`、`src/prediction_market.rs`、`src/lib.rs`、`src/main.rs` |
| SDK 工具 | `src/sdk/tools/wallet.rs`、`src/sdk/tools/search.rs`、`src/sdk/tools/librarian.rs`、`src/sdk/tools/mod.rs` |
| 实验 runtime loop | `experiments/minif2f_v4/src/bin/evaluator.rs`、`experiments/minif2f_v4/src/lean4_oracle.rs` |
| 守恒测试 | `tests/reward_pull_conservation.rs`、`tests/wal_resume.rs` |
| Claude Code sub-agent prompt（你要验证它们是否只是 prompt） | `.claude/agents/proposer.md`、`.claude/agents/auditor.md` |
| 历史审计归档目录（须核验是否空） | `handover/directives/`、`handover/audits/` |
| 例行任务 | `routines/daily_drift.yaml` |
| Claude 内部审计摘要（本次对照基线） | `handover/ai-direct/EXT_AUDIT_2026-04-21/claude_internal_findings.md` |
| 当前研究状态 | `handover/ai-direct/LATEST.md`、`handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md` |
| Phase 7 checkpoint | `handover/ai-direct/CHECKPOINT_PHASE_7_TURING_2026-04-21.md` |
| Phase 7 DAG 分析（另一 Claude agent 的产出） | `experiments/minif2f_v4/analysis/PHASE7_DAG_ANALYSIS.md` |
| Phase 7 实验日志 | `experiments/minif2f_v4/logs/templadder_n8_20260421T164014.jsonl` |
| Phase 7 proof artifacts | `experiments/minif2f_v4/proofs/imo_1964_p2_*.lean`、`proofs/mathd_algebra_332_*.lean`、`proofs/imo_1981_p6_*.lean` |

commit: `e0a75ec`（Phase 7 Turing per-tactic δ-step 合并后）

## 背景

Claude 内部审计声称 4 项：(a) Art. V 三权分立仅 prompt 空壳；(b) F-2026-04-20-05 `native_decide` 绕过已三层封堵；(c) 宪法文件 `constitution.md` mode 644 + 无 git hook 锁定；(d) `graveyard` + TopKClasses 双写是冗余结构。你需要独立验证或推翻。

## 必答 6 项（每项给出 PASS / VETO / CHALLENGE + 证据）

### Q1. Art. V 三权分立是否真为"空壳"？
- 在整个 repo（不仅 `src/`）grep `ArchitectAI|JudgeAI|architect_ai|judge_ai`，报告所有匹配位置
- 检查 `.claude/agents/proposer.md`、`.claude/agents/auditor.md` 是否被 Rust 代码或 CI 管道实际调用，还是仅作为 Claude Code sub-agent 定义
- 检查 `handover/directives/` 和 `handover/audits/` 是否为空（`ls -la`）
- 判决：这是"纯 prompt 角色"还是"系统内独立进程"？

### Q2. F-2026-04-20-05 三层封堵完整性
Claude 声称有三层防御：
1. `experiments/minif2f_v4/src/lean4_oracle.rs:113` `verify_omega_detailed` 内 `check_payload` 前置
2. `src/bus.rs:182` `append_internal` 非 oracle_blessed 路径跑 `forbidden_patterns`
3. `routines/daily_drift.yaml` 每日 positive verify

请：
- 逐行核对三处代码是否真的做了 Claude 声称的事
- 搜索是否还有**第四条**绕过路径（例如任何其他调 Lean 的入口、任何 `oracle_blessed=true` 被不当设置的地方）
- 特别检查：`append_oracle_accepted` (`bus.rs:174-177`) 是否存在不经 oracle 就能设 blessed 的路径？

### Q3. 宪法文件保护
- `ls -la /home/zephryj/projects/turingosv4/constitution.md` 报告 mode
- `git config --get-all core.hooksPath` 和 `ls .git/hooks/` 是否有 commit hook 锁定宪法
- 是否存在 CI / routine 检查宪法变更？
- 按 Art. V.1.1 "只读 + sudo-only"，当前状态是**合规**还是**违宪**？

### Q4. Karpathy TOP-10 热路径声明核对
Claude 提出 10 个代码优化点（见 `handover/ai-direct/EXT_AUDIT_2026-04-21/claude_internal_findings.md` §3）。选以下 3 条做独立验证：
- `src/bus.rs:244-246` InvestOnly 路径三重 clone — 真的在热路径吗？是否每次 invest 都走这段？
- `src/bus.rs:416-424` `Box<dyn TuringTool>` + 4× `.downcast_mut::<WalletTool>()` — 每个 tx 调用次数？
- `src/ledger.rs:115-135` `trace_ancestors` 每次查都 alloc `HashSet<String>` — 实际调用频率？

每条给出：(a) Claude 声明是否属实 (b) 是否还有更深层的相关问题

### Q5. 园丁 Agent（Art. III.1）是否真的缺失
在整个 repo（含 `handover/`, `routines/`, `.github/`）grep `gardener|prune|evict|tape_gc|garbage|tombstone|tape.remove|ledger.remove|tape.delete`。
- 报告所有匹配
- 判决：Art. III.1 "后台园丁 Agent 定期扫描并屏蔽偏离黄金原则的陈旧代码与过期文档" — 有任何实现吗？即使不叫 gardener？

### Q6. `recent_rejections(author, ...)` 参数被吞问题
- `src/bus.rs:494-537` 检查 `recent_rejections` 函数签名
- 确认：在 `TopKClasses` 分支里 `author` 参数是否真的被忽略、只做全局聚合？
- 如果是，这是否与 Art. III.3 "刻意屏蔽横向相关性"冲突？还是符合 C-022 "typical error shield broadcast"？
- 建议：是否需要立新判例 C-047？

## 返回格式

```markdown
# Codex External Audit — 2026-04-21

| # | 声明 | 判决 | 证据 |
|---|---|---|---|
| Q1 | Art. V 空壳 | PASS/VETO/CHALLENGE | file:line |
| ... |

## 深度发现（Claude 未提及的问题）
- ...

## 总体建议
- MERGE / HOLD / REVERT
```

## 禁止事项
- 不要修改任何文件
- 不要运行 cargo build / cargo test（编译时间预算已耗尽）
- 不要对宪法的"哲学正确性"做评论，只关心"代码是否与宪法字面对齐"
- 不要重复 Claude 已经给出的结论，除非你有新证据或反驳
