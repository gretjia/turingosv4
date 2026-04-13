# TuringOS v4 — Handover State
**Updated**: 2026-04-13
**Session Summary**: Multi-agent auto-research 团队架构完成 + 全部 Rust 代码从宪法重写 + MiniF2F v4 实验 harness 就绪 + Codex 双审计 CLEAN

## 本次 Session 完成的工作

### 1. 架构设计 (参考 AutoResearch + Meta-Harness + Hermes)
- 三权分立 (Art. V.1): ArchitectAI (Claude Opus) → JudgeAI (Gemini) → Codex Auditor
- 双循环: Outer (harness 改进, 文件系统历史) + Inner (swarm 求解)
- 单一指标 (Karpathy 原则): MiniF2F v2 solve rate (488 题)
- Swarm: DeepSeek V3.2 Reasoner (主力) + Claude Opus (多样性种群)
- 计划文件: `.claude/plans/humming-tickling-sifakis.md`

### 2. Rust 代码重写 — 3,762 行 (core 3,036 + experiment 726)

| 模块 | 行数 | 宪法依据 | V3 教训 |
|------|------|---------|---------|
| ledger.rs | 470 | Law 1, Magna Carta | V3L-09 (无静默失败), V3L-24 (无/tmp) |
| prediction_market.rs | 337 | Law 2 (CTF守恒) | V3L-41/42/43 (无铸币), V3L-44 (无固定税) |
| kernel.rs | 274 | Law 1 (零领域知识) | V3L-45, V3L-23 |
| bus.rs | 406 | Art. II/III | V3L-11 (串行), V3L-21, V3L-31 |
| sdk/actor.rs | 276 | Art. II.2.1 | V3L-14 (Boltzmann非ArgMax) |
| sdk/protocol.rs | 236 | Art. I.1.1 | V3L-08/09/15/16 |
| sdk/tools/wallet.rs | 224 | Law 2 | V3L-22 (Goodhart), V3L-39 |
| sdk/sandbox.rs | 203 | Art. I.1 | V3L-01/02/07 |
| drivers/llm_http.rs | 191 | Art. IV | V3L-25/26/27 |
| lean4_oracle.rs | 213 | Art. I.1 (布尔谓词) | V3L-01/02/07 |
| evaluator.rs | 276 | 全部 | oneshot + n1 + n3 模式 |
| 其余 | 656 | — | tool, prompt, snapshot, search, librarian |

### 3. 测试 — 102 全部通过
- Core: 93 tests (ledger 13, PM 14, kernel 10, bus 11, sdk 42, driver 3)
- Oracle: 9 tests (sorry/identity/forbidden)
- Harness: 48/48 PASS
- Constitutional check: 13 passes, 0 violations

### 4. Codex + Auditor 双审计
- **Auditor**: VERDICT: CLEAN (6/6)
- **Codex**: 8 findings (2 CRITICAL, 4 HIGH, 2 MEDIUM) — **全部已修复**
  - bus invest 不扣钱 → wallet debit + refund on failure
  - wallet credit 公开 + 负数 deduct → pub(crate) + 正数验证
  - ledger hash 不覆盖 detail → 已修复
  - ledger verify 不检测截断 → seq 检查
  - protocol 对畸形 action tag 走 fallback → reject
  - BinaryMarket 字段公开 → 私有 + getters
  - temperature ≤ 0 → fallback 0.5
  - trace_ancestors 单父 → 记录为设计决策

### 5. MiniF2F v4 实验 Harness
- `experiments/minif2f_v4/` — workspace member, release binary 编译成功
- Lean4Oracle: sorry 3层防御, identity theft, forbidden patterns
- 评估器: oneshot / n1_turingos / n3_turingos 三模式
- 批量运行器: `run_batch.sh [oneshot|n1|n3] [test|valid|all]`
- 历史追踪器: `history.py` (跨 run 比较, history.jsonl 导出)
- 数据集: 直接引用 v3 的 488 题 (磁盘不够, 不复制)

---

## Current State

### 组件清单

| 组件 | 数量 | 状态 |
|------|------|------|
| constitution.md | 730 行, 20 条款 | DONE |
| Cases | 35 个 | DONE |
| V3 Lessons | 50 个 | DONE |
| **Rust Core** | **3,036 行, 14 模块** | **DONE** |
| **Experiment** | **726 行, evaluator + oracle** | **DONE** |
| **单元测试** | **102 个** | **102/102 PASS** |
| Hooks | 3 个 | DONE |
| Rules | 10 条 YAML | DONE |
| Harness Tests | 48 个 | 48/48 PASS |
| Constitutional Check | 13 passes | 0 violations |
| **Dual Audit** | Codex + Auditor | **CLEAN** |

### 关键架构决策

1. **从宪法重写，不迁移 v3** — 每模块追溯到宪法条款 + V3L 教训
2. **Generator ≠ Evaluator** — JudgeAI 用 Gemini (不同模型族)
3. **单一指标** — solve_count / 488 (Lean 4 type-checker 是 oracle)
4. **Meta-Harness 文件系统历史** — ArchitectAI 读全部 run 历史
5. **MiniF2F 数据引用 v3 路径** — 磁盘不够, 不复制 (v3 保留)
6. **DeepSeek timeout** — 120s + 3次重试 + 指数退避 (用户确认)

---

## Next Steps (给下一个 session)

### 立即可执行 (需要 API keys)
1. **设置环境变量**: `export DEEPSEEK_API_KEY=... LLM_PROXY_URL=...`
2. **运行 oneshot baseline**: `./experiments/minif2f_v4/run_batch.sh oneshot test`
3. **记录 baseline 数字**: X/244 solved → 这是起点

### 后续
4. **部署 LLM proxy** on linux1-lx (Python async, ThreadingMixIn)
5. **运行 n3 swarm**: 3 agent prediction market 协调
6. **首次 outer loop**: ArchitectAI 读 history → 提出改进 → JudgeAI 校验
7. **Multi-model**: DeepSeek (主力) + Claude Opus (多样性)

## Open Questions
- v3 仓库保留 (已确认: 不删除)
- DeepSeek API 配额是否够 488 题 x 多轮? (用户确认: 可以, 注意 timeout)
- 是否需要 llm_proxy.py 中转, 还是直接调 DeepSeek API? (取决于网络环境)
