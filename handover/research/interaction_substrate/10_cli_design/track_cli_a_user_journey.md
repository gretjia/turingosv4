# Phase 1 / Track CLI-A — User Journey 设计

**本文档为 forward-bound research 提案；任何派生 TB 须独立 §8 ratification；本文档不构成已批准方案。**

**目的**: 从用户视角设计 TuringOS v4 端到端 user journey，识别缺失/未连通的环节。Phase 1 CLI 整体的 source-of-truth。

---

## 0. 重大发现 — 项目已有 user CLI `lean_market`

Phase 0 inventory **未发现** `experiments/minif2f_v4/src/bin/lean_market.rs`（因为只盘点了 `src/bin/`）。Phase 1 启动时勘察发现该 CLI 是 **TB-10 Atom 2 已 ship 的 user-facing CLI**（848 行，TB-10 lean_market_mvp_smoke evidence 2026-05-02）。

**这改变 Phase 1 定位**: 不是 from-scratch 设计；是 **演化 lean_market → 统一 `turingos` CLI**。

### 0.1 `lean_market` 现有 subcommand 清单

来源: `experiments/minif2f_v4/src/bin/lean_market.rs:48-65`

| Subcommand | 行号 | 功能 | TB |
|---|---:|---|---|
| `run-task` | 146 | 启动任务: bootstrap chaintape + post `TaskOpenTx` + `EscrowLockTx` + 触发 evaluator solver loop | TB-10 |
| `view-task` | 293 | 看任务状态 (task_id / status / bounty) | TB-10 |
| `view-wallet` | 318 | 看钱包 (sponsor balance / solver payout) | TB-10 |
| `view-replay` | 596 | 重放验证 (replay_full_transition 检查) | TB-10 |
| `tick` | 384 | 推进时间 (TaskExpire / TaskBankruptcy 触发) | TB-11/12 |
| `view-bankruptcy` | 555 | 看破产记录 | TB-11/12 |
| `view-positions` | 465 | 看仓位 (NodePosition) | TB-12 |

### 0.2 `lean_market` 架构限制

来源: `lean_market.rs:13-28` 注释:

> Architecture: lean_market is a SINGLE-PROCESS thin wrapper. The `run-task` subcommand spawns the evaluator binary as a child process with `TURINGOS_USER_TASK_MODE=1` + `TURINGOS_USER_TASK_BOUNTY_MICRO=<n>` + a fresh chaintape path.

**限制**:
1. **Lean-only**: 只覆盖 Lean proof market 场景；不支持 polymarket / role assignment / market trigger
2. **Single-process child fork**: evaluator 作为 child process 启动；不支持长期 batch / 多 task 串联
3. **Manual subcommand parsing** (`subcommand.first().map(|s| s.as_str())`)；无 clap
4. **No JSON output option**: 全部 text output
5. **Bootstrap fresh chaintape only**: 不能 resume / attach 已有 chaintape (`Sequencer fail-closes on non-empty chaintape per TB-6 NonEmptyRuntimeRepo gate`)
6. **No web/HTML mode**
7. **位于 experiments/ crate**: 不是主 `src/bin/`；用户必须知道 experiments path 才能用

---

## 1. 用户视角端到端 User Journey (覆盖 TuringOS 全功能)

按"从 0 上手到产出 evidence + 复盘"的完整路径设计。**11 个核心场景** × **从批准到导出** = TuringOS 完整体验。

### 1.1 总图

```mermaid
flowchart LR
    A[用户来到 TuringOS] --> B[场景 1: 配置环境 + 创建 spec]
    B --> C[场景 2: 启动 init / 创建 batch]
    C --> D[场景 3: 部署 agent + 配置 role]
    D --> E[场景 4: 提交 task / 投放问题]
    E --> F[场景 5: 监控 chain + 观察 agent 行为]
    F --> G[场景 6: 触发市场 (CPMM/event_resolve/finalize)]
    G --> H[场景 7: 查 audit / verify tape]
    H --> I[场景 8: 看 EconomicJudgment / PnL / role 分化]
    I --> J[场景 9: 跨问题持久化 / batch 串联]
    J --> K[场景 10: 出错 / 破产 / 恢复]
    K --> L[场景 11: 导出 evidence / 复盘 / 回放]
```

### 1.2 11 个场景详细

#### 场景 1: 配置环境 + 创建 spec

**用户期望**:
- 安装 turingos (cargo install / docker pull)
- 配置: model API key / runtime_repo 路径 / CAS 路径
- 创建 spec (genesis_payload.toml 等)

**当前覆盖**:
- ❌ 安装: 无 cargo install 配置；需用户克隆 + cargo build
- ⚠️  配置: `genesis_payload.toml` 等手写，无脚手架
- ⚠️  spec 创建: `genesis_report.rs` 是程序内构造；无用户 CLI

**Phase 1 CLI 需求**:
- `turingos init` — 创建工程目录 + 默认 genesis_payload.toml + 配置向导
- `turingos config set <key> <value>` — 配置管理
- `turingos spec new <spec_type>` — 创建特定类型 spec 模板 (proof / polymarket / multi-agent)

#### 场景 2: 启动 init / 创建 batch

**用户期望**:
- 从 spec 启动 init 模块
- 创建 batch (multi-task 串联)
- 启动 runtime_repo + 初始化 CAS

**当前覆盖**:
- ✅ Lean 单 task: `lean_market run-task` (但 fresh chaintape only)
- ⚠️  Batch: `BatchContinuationManifest` 已落地为 schema (`src/runtime/batch_continuation_manifest.rs`)；无 user CLI
- ❌ Non-Lean task type: 无支持

**Phase 1 CLI 需求**:
- `turingos batch new --name <name> --spec <spec.toml>` — 创建 batch
- `turingos batch start <batch_id>` — 启动 batch
- `turingos batch list` — 列出 batch
- `turingos task open --spec <spec> --bounty <micro>` — 通用 task open (包含 Lean / polymarket)

#### 场景 3: 部署 agent + 配置 role

**用户期望**:
- 部署 N 个 agent (different LLM models)
- 给 agent 分配 role (Bull / Bear / Solver / Verifier / Librarian / Architect / Veto / Observer)
- 配置 agent 预算 / 视图策略

**当前覆盖**:
- ✅ AgentRoleAssignment schema 已落地 (`src/runtime/real5_roles.rs`)
- ✅ ModelAssignmentManifest 已落地 (G4.2)
- ⚠️  配置必须在 `genesis_payload.toml` 手写；无 CLI 子命令
- ❌ 部署/启停 agent: 无独立 CLI

**Phase 1 CLI 需求**:
- `turingos agent deploy --model <model> --role <role> --budget <micro>` — 部署 agent
- `turingos agent list` — 列出已部署 agent
- `turingos role assign --agent <id> --role <role>` — 改角色
- `turingos agent view --id <id>` — 看 agent 当前视图 (role-scoped DerivedView)

#### 场景 4: 提交 task / 投放问题

**用户期望**:
- 提交 Lean theorem / polymarket event / MiniF2F problem
- 设置 bounty / deadline / verifier 要求
- 选择 model (默认 vs 指定)

**当前覆盖**:
- ✅ Lean: `lean_market run-task --problem <id>` (固定 evaluator)
- ❌ Polymarket: 无 CLI
- ❌ 通用 task type: 无

**Phase 1 CLI 需求**:
- `turingos task open --type lean --problem <id> --bounty <micro>` — Lean task
- `turingos task open --type polymarket --event <event.toml>` — Polymarket task
- `turingos task open --type custom --spec <spec.toml>` — 通用 task (forward-bound 未来扩展)

#### 场景 5: 监控 chain + 观察 agent 行为

**用户期望**:
- 实时 / 半实时看 ChainTape 进展
- 观察 agent 决策 (MarketDecisionTrace / NoTradeReason / EconomicJudgment)
- 看 Librarian broadcast (REAL-BCAST-1)

**当前覆盖**:
- ✅ `audit_dashboard.rs` 提供 16 个 render section text 输出 (但 batch process 后才能看完整)
- ✅ MarketDecisionTrace / NoTradeReason / EconomicJudgment schema 已落地
- ❌ 实时流式: 无 (audit_dashboard 是 batch 后生成)
- ❌ 半实时: 无 tail / watch 模式

**Phase 1 CLI 需求**:
- `turingos watch chain --repo <path> --cas <path>` — 实时 tail ChainTape (新 tx 流式输出)
- `turingos watch agent --id <id>` — tail 特定 agent 的决策
- `turingos watch market` — tail market activity (CPMM swap / event resolve / finalize)
- `turingos watch broadcast` — tail Librarian broadcast

#### 场景 6: 触发市场 (CPMM / event_resolve / finalize)

**用户期望**:
- 创建 CpmmPool / MarketSeed (系统级)
- 触发 EventResolve (人工裁决 polymarket)
- 触发 FinalizeReward (清算 solver payout)

**当前覆盖**:
- ⚠️  通过 evaluator child process 间接触发；无独立 CLI
- ❌ 人工触发: 无 (TB-N3 之后由系统自动触发)

**Phase 1 CLI 需求** (现阶段人是 system_emitted 路径的间接发起者):
- `turingos market trigger seed --task <id>` — 触发 MarketSeed (system_emitted)
- `turingos market trigger cpmm-pool --task <id> --reserves <yes,no>` — 触发 CpmmPool init
- `turingos market trigger event-resolve --task <id> --outcome <yes\|no>` — 人工裁决 (system_emitted)
- `turingos market trigger finalize --task <id>` — 触发 FinalizeReward

**关键约束**: 所有 trigger 都是 system_emitted；人在这里是 system_emitted 的间接发起者；CLI 必须在 sequencer 内通过 system signature path 提交 (不冲击 R-022 type 错误)

#### 场景 7: 查 audit / verify tape

**用户期望**:
- 看 dashboard (text / json / html)
- 验证 chaintape 完整性
- 检查 tamper

**当前覆盖**:
- ✅ `audit_dashboard.rs` (text + json) — TISR Phase 1 直接整合
- ✅ `audit_tape.rs` / `audit_tape_tamper.rs` — TISR Phase 1 直接整合
- ✅ `verify_chaintape.rs` — TISR Phase 1 直接整合
- ❌ HTML output: 无 (Phase 5 候选)

**Phase 1 CLI 需求**:
- `turingos audit dashboard --repo <path> --cas <path> [--format text|json|html]` — wraps audit_dashboard
- `turingos audit tamper --repo <path>` — wraps audit_tape_tamper
- `turingos verify chaintape --repo <path>` — wraps verify_chaintape
- `turingos verify tape --repo <path>` — wraps audit_tape

#### 场景 8: 看 EconomicJudgment / PnL / role 分化

**用户期望**:
- 看每个 agent 的 EconomicJudgment 历史
- 看 PnL 走势 / 跨问题持久化 (REAL-5/REAL-12/REAL-G1)
- 看 role 分化是否真实 (Bull 真买多, Bear 真买空)

**当前覆盖**:
- ✅ Schema 已落地 (`economic_judgment.rs` / `agent_pnl.rs` / `persistence_evidence.rs`)
- ⚠️  audit_dashboard 内嵌部分；不独立
- ❌ 跨 task / 跨 batch 持久化视图: 无独立 CLI

**Phase 1 CLI 需求**:
- `turingos report judgments --agent <id> [--from <batch>] [--to <batch>]` — agent EconomicJudgment 历史
- `turingos report pnl --agent <id>` — PnL 走势
- `turingos report roles` — role 分化报告 (Bull vs Bear vs Solver vs Observer 行为分布)
- `turingos report persistence --batch <id>` — 跨问题持久化报告 (REAL-G1)

#### 场景 9: 跨问题持久化 / batch 串联

**用户期望**:
- 把多个 task 串联为一个 batch
- 看跨 task agent 身份/钱包/角色保持
- 看市场记忆是否累积

**当前覆盖**:
- ✅ `BatchContinuationManifest` schema (`batch_continuation_manifest.rs`)
- ✅ G1.2-4 ship gates (manifest_records_all_tasks_in_order 等)
- ❌ User CLI: 无 (G-Phase G1 仍在收口)

**Phase 1 CLI 需求**:
- `turingos batch new --name <name>` — 见场景 2
- `turingos batch add --task <task_id>` — 追加 task 到 batch
- `turingos batch view --id <id>` — 看 batch 全景
- `turingos batch persistence --id <id>` — 跨 task 一致性检查

#### 场景 10: 出错 / 破产 / 恢复

**用户期望**:
- agent 余额耗尽 → bankruptcy 自动触发
- 看 bankruptcy 记录
- 从 checkpoint 恢复

**当前覆盖**:
- ✅ TaskBankruptcy typed_tx + `lean_market view-bankruptcy`
- ✅ `resume_preflight.rs` runner preflight 检查
- ⚠️  恢复路径: `resume_preflight` 是 read-only 检查；实际 resume 不存在

**Phase 1 CLI 需求**:
- `turingos report bankruptcy --repo <path>` — wraps lean_market view-bankruptcy
- `turingos preflight --repo <path>` — wraps resume_preflight
- `turingos resume --repo <path>` — 实际 resume (Phase 6 候选; 可能 forward-bound 到 Class 2)

#### 场景 11: 导出 evidence / 复盘 / 回放

**用户期望**:
- 导出 evidence bundle (ChainTape + CAS + dashboard + audit)
- 把 evidence 分享给团队/架构师
- 从 evidence bundle 完全 replay

**当前覆盖**:
- ⚠️  `genesis_report.json` 创建于 run-time (`genesis_report.rs`)；无 export 子命令
- ⚠️  `replay_full_transition` 函数存在 (`transition_ledger.rs`); 通过 `lean_market view-replay` 触发
- ❌ 一键 export bundle: 无
- ❌ 一键 replay from bundle: 无

**Phase 1 CLI 需求**:
- `turingos export evidence --repo <path> --cas <path> --out <bundle.tar.gz>` — 打包 evidence
- `turingos replay --bundle <bundle.tar.gz>` — 从 bundle 完全 replay
- `turingos report markov --bundle <bundle.tar.gz>` — wraps generate_markov_capsule

---

## 2. 用户角色差异化 User Journey

### 2.1 用户类型矩阵

| 用户类型 | 主要场景 | 关键需求 | 备注 |
|---|---|---|---|
| **研究者** (默认) | 1-11 全部 | 完整 evidence + audit + replay | 当前 TuringOS 主用户 |
| **审计员** | 7, 8, 11 | 看 dashboard / verify / report | 一般不启动 task |
| **DevOps** | 1, 2, 10 | 配置 / 恢复 | 不关心 agent 行为内容 |
| **架构师** | 全部 + Veto/Architect role agent 配置 | 元层观察 + spec 修订 | TISR Phase 7+ 深化 |
| **第三方开发者** | 4, 6, 11 | 投放 task + 看结果 | 未来 SaaS / API 用户 |

### 2.2 默认 User Journey (研究者视角)

```
turingos init --project tisr-research-2026-05-17
turingos config set model deepseek-chat
turingos batch new --name minif2f-multi-llm-week1
turingos agent deploy --model deepseek-chat --role Solver --budget 1000000
turingos agent deploy --model claude-haiku --role Verifier --budget 500000
turingos agent deploy --model gpt-4o --role Challenger --budget 500000
turingos task open --type lean --problem mathd_algebra_171 --bounty 100000
turingos watch chain &   # background tail
turingos watch broadcast &   # background tail
# (等待 agent 工作; ~10-30 min)
turingos audit dashboard --format text
turingos report pnl --agent Agent_user_0
turingos report roles
turingos export evidence --out batch_minif2f_week1.tar.gz
```

12 步覆盖了 11 个场景的核心路径。这是 TISR Phase 5 ROADMAP 的 "happy path" 目标。

---

## 3. Phase 1 CLI 设计原则 (CLI-A 输出)

### 3.1 核心原则

1. **演化 lean_market 不重造**: 保留现有 7 个 subcommand 的行为 + 重命名为 `turingos task` / `turingos report` 子命令
2. **统一 parent `turingos`**: 用 clap-based 解析；所有 subcommand 走 `turingos <sub>`
3. **不引入新 typed_tx**: 严格符合 Class 1-2 边界
4. **system_emitted 路径承担人介入**: 场景 6 (market trigger) 严格走 system_emitted
5. **--format text|json|html** 三模式: text 默认；JSON 走 serde；HTML 走 minimal template (Phase 5 候选)
6. **watch 子命令支持 tail**: 长时间运行场景需要
7. **配置文件 vs CLI flag 二元**: 复杂参数走 toml；简单走 flag

### 3.2 与 `turingos_dev` 共存

- `turingos_dev` 保留为 dev evidence sidecar (`open / record-diff / record-command / record-audit / validate / close / summarize`)
- `turingos` (新) 是 user-facing main CLI
- 两者**不合并**: 角色分明 (dev sidecar vs user product)
- 长期可考虑 `turingos dev` 子命令包装 dev sidecar

### 3.3 与 experiments/ 的关系

- `lean_market` 当前在 `experiments/minif2f_v4/src/bin/` (TB-10 历史决定)
- TISR Phase 6 (CLI MVP 实施): **将 `lean_market` 提升到 `src/bin/turingos.rs`** 作为统一 user CLI
- `experiments/minif2f_v4/src/bin/` 保留 `evaluator.rs` / `batch_evaluator.rs` / `comprehensive_arena.rs` (实验级)；lean_market 出 experiments

### 3.4 与 audit_dashboard 等现有 bin 的关系

- 现有 bin 保留独立 binary (向后兼容)
- 新 `turingos` 用 **同名 library function** 包装 (`audit_dashboard::build_report` 等)
- 用户优先用 `turingos audit dashboard`；老 user / scripts 仍可用 `audit_dashboard` 直接调用

---

## 4. Phase 1 CLI-A 完成 — 输出 + 下一步

### 4.1 本 Track 完成

- ✅ lean_market 现有 7 subcommand 盘点
- ✅ 11 个场景 user journey 拆解
- ✅ 每场景的当前覆盖 vs 需求差距明确
- ✅ 5 个用户类型差异化 journey
- ✅ "happy path" 默认 12 步路径
- ✅ Phase 1 CLI 设计 7 个核心原则

### 4.2 Track CLI-B 输入 (本 Track 输出的 hand-off)

Track CLI-B (现有 bin 集成清单) 应聚焦:
- 11 个现有 bin (含 lean_market) 各自的 entry/exit point
- 哪些 bin 直接 wrap (一对一 subcommand)
- 哪些 bin 需要 library 化重构 (e.g., audit_dashboard::build_report 作为 lib function)
- experiments/lean_market 提升到 src/bin/turingos 的路径

### 4.3 Track CLI-C 输入

Track CLI-C (缺失能力清单) 应聚焦:
- 场景 5 实时 watch 模式 (4 个 watch 子命令)
- 场景 6 system_emitted 触发路径 (4 个 market trigger 子命令)
- 场景 9 batch 管理 (4 个 batch 子命令)
- 场景 11 evidence export/replay (3 个 export/replay 子命令)
- 场景 1 init/config 脚手架 (3 个 config 子命令)
- 估算 LOC: ~1500-2000 (现有 lean_market 848 行作为基础)

### 4.4 Track CLI-D 输入

Track CLI-D (CLI Architecture) 应聚焦:
- clap-based 解析层 (~300 LOC)
- subcommand 命名规范 (动词-名词 / 名词-动词?)
- --format text|json|html 输出层 (~400 LOC)
- 配置 (genesis_payload.toml) 接入
- 状态 (runtime_repo) 接入
- error 抽象 (`turingos: <error>`) 风格统一

---

## 5. Anti-pattern 防御

### 5.1 不做的事

- ❌ **不**重写 lean_market 已 ship 的 7 个 subcommand 行为
- ❌ **不**引入新 typed_tx variant (Class 4 surface 修改)
- ❌ **不**做 web UI (Phase 1 是 CLI; web UI 在 Phase 7 轴 A 深化)
- ❌ **不**让 CLI 直接写 ChainTape (必须走 sequencer)
- ❌ **不**让 CLI 包含 agent 决策逻辑 (CLI 是工具; agent 是另一回事)

### 5.2 Kill Condition 4 检查

按 Charter §5 Kill Condition 4:
> Phase 1 (CLI) 发现现有 `src/bin/` 已 90%+ 覆盖用户工作流 → CLI Phase 5 spec 缩为薄包装

**判定**:
- 现有 11 个 bin + lean_market 共 12 个 covered subcommand
- 11 个 user journey 场景 + ~30 个子命令需求
- 覆盖度: 12/30 ≈ **40%**
- **不触发** Kill Condition 4 (CLI 设计正常 scope, 不缩为薄包装)

### 5.3 Kill Condition 5 检查

按 Charter §5 Kill Condition 5:
> Phase 2 任何 track 提议需要 typed_tx schema 修改才能落地 → 标 forward-bound

**判定**:
- 本 CLI-A 文档**未**提议任何新 typed_tx
- 所有 30+ 子命令需求都走现有 typed_tx (agent: Work/Verify/Challenge/.../BuyWithCoinRouter; system: TaskOpen/EscrowLock/EventResolve/MarketSeed/CpmmPool/FinalizeReward 等)
- **不触发** Kill Condition 5 ✅

---

## 6. 验收 — CLI-A 出口自检

- ✅ 11 个场景全覆盖 (init/batch/agent/task/watch/market/audit/report/persistence/recovery/export)
- ✅ 5 个用户类型差异化 journey
- ✅ Happy path 默认 12 步路径
- ✅ 与 lean_market / turingos_dev / audit_dashboard 关系明确
- ✅ Anti-pattern 防御清单
- ✅ Kill Condition 4/5 检查通过
- ✅ forward-bound 头部声明
- ✅ Track CLI-B/C/D 输入 hand-off 清晰
