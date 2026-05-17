# Phase 1 / Track CLI-C — 缺失能力清单

**本文档为 forward-bound research 提案；任何派生 TB 须独立 §8 ratification；本文档不构成已批准方案。**

**目的**: 对照 CLI-A 11 个用户场景和 CLI-B 现有 bin 覆盖度，明确缺失子命令需求 + LOC 估算 + Class 评估 + 优先级。Phase 6 (CLI MVP) 实施清单的核心输入。

---

## 1. 总览

| 来源 | 数量 | 备注 |
|---|---:|---|
| 现有 12 bin 包装为 turingos subcommand | ~12 个 | CLI-B 详述 |
| **新增**缺失功能 | **~25 个** | 本文档详述 |
| **总计** turingos subcommand | **~37 个** | Phase 6 MVP 目标 |

**新增 25 个的 Class 分布**:
- Class 1: ~18 个 (60-72%) — 配置/视图/CLI 脚手架，不写 ChainTape
- Class 2: ~7 个 (28-40%) — 触发 system_emitted tx (经 sequencer)，涉及状态变更

**总新增 LOC 估算**: ~2000-2800 行 Class 1-2 代码

---

## 2. 缺失能力 25 个子命令详述

### 场景 1: 配置环境 + 创建 spec (3 个新)

#### 2.1 `turingos init` — 创建工程目录 + 配置向导

**需求**: 用户首次使用时一键创建标准工程目录。

**功能**:
- 创建 `runtime_repo/` / `cas/` 目录
- 生成默认 `genesis_payload.toml` 模板
- 生成默认 `agent_pubkeys.json` 占位
- 生成默认 `constitution.md` 链接（指向 `constitution.md`）
- 交互式或 flag-based 配置向导 (model API key / default bounty / etc.)

**输入**: `--project <name> [--template proof|polymarket|multi-agent]`
**输出**: 工程目录 + 配置文件

**LOC**: ~150 (含模板字符串)
**Class**: 1 (纯 filesystem 操作，无 sequencer 调用)
**依赖**: 无

#### 2.2 `turingos config set/get` — 配置管理

**需求**: 用户可读写 turingos 全局/项目配置。

**功能**:
- 读写 `~/.config/turingos/config.toml` (全局) 或 `./turingos.toml` (项目)
- 嵌套 key path (`turingos config set model.default deepseek-chat`)
- 列出当前配置 (`turingos config list`)

**输入**: `set <key> <value>` / `get <key>` / `list`
**输出**: 配置值 / 列表

**LOC**: ~100
**Class**: 1
**依赖**: 无

#### 2.3 `turingos spec new` — 创建特定类型 spec 模板

**需求**: 用户根据 spec 类型选择模板生成。

**功能**:
- 模板: proof / polymarket / multi-agent
- 输出: 完整 toml 模板，含必填字段标注

**输入**: `--type <type> --out <path>`
**输出**: spec.toml 模板文件

**LOC**: ~120 (含 3 个模板字符串)
**Class**: 1
**依赖**: 无

**场景 1 合计**: 3 个新, ~370 LOC, 全 Class 1

---

### 场景 2: 启动 init / 创建 batch (4 个新)

#### 2.4 `turingos batch new` — 创建 batch manifest

**需求**: 创建一个空 BatchContinuationManifest，准备接纳 task。

**功能**:
- 调用 `BatchContinuationManifest::new(name)` 构造
- 写入 `<runtime_repo>/BatchContinuationManifest.json`
- 注册 batch ID

**输入**: `--name <name> [--repo <path>]`
**输出**: batch ID + manifest 路径

**LOC**: ~80
**Class**: 1 (构造 manifest, 不写 ChainTape)
**依赖**: `runtime::batch_continuation_manifest`

#### 2.5 `turingos batch start` — 启动 batch (开始接受 task)

**需求**: 标记 batch 为 active；初始化 runtime_repo + CAS (调用 bootstrap factory)。

**功能**:
- 调用 `runtime::bootstrap::initial_preseed_micro` 准备 genesis balances
- 调用 `runtime::adapter::genesis_with_balances` 构建初始 QState
- 写 genesis_report.json (调 `runtime::genesis_report`)
- 标记 batch active

**输入**: `--id <batch_id> [--spec <spec.toml>]`
**输出**: genesis_report.json + 状态 active

**LOC**: ~200
**Class**: **2** (触发 system_emitted genesis tx via sequencer)
**依赖**: `runtime::bootstrap` / `adapter` / `genesis_report`

**关键约束**: 走 system_emitted 路径；不需要新 typed_tx；现有 genesis 流程可直接复用。

#### 2.6 `turingos batch list` — 列出 batch

**需求**: 列出所有 batch (active / completed)。

**功能**:
- 扫描 `runtime_repo/` 下所有 BatchContinuationManifest.json
- 解析 status / 进度 / task 数

**输入**: `[--status active|completed|all]`
**输出**: batch 列表 (text/json)

**LOC**: ~60
**Class**: 1 (read-only)
**依赖**: `runtime::batch_continuation_manifest`

#### 2.7 `turingos batch view` — 看 batch 详细

**需求**: 看 batch 内 task 列表 + agent 状态 + 进度。

**功能**:
- 读 BatchContinuationManifest
- 解析每个 task 的 status
- 显示 batch-level 统计

**输入**: `--id <batch_id>`
**输出**: batch 全景 (text/json)

**LOC**: ~100
**Class**: 1
**依赖**: `runtime::batch_continuation_manifest` + `runtime::run_summary`

**场景 2 合计**: 4 个新, ~440 LOC, 1 Class 2 + 3 Class 1

---

### 场景 3: 部署 agent + 配置 role (4 个新)

#### 2.8 `turingos agent deploy` — 部署 agent

**需求**: 一键部署新 agent (生成 keypair + 注册 role + 配置 model)。

**功能**:
- 调用 `agent_keystore::generate_keypair`
- 写 keypair 到 keystore (encrypted)
- 写 pubkey 到 `agent_pubkeys.json`
- 添加 `AgentRoleAssignment` 到 GenesisReport (如果在 genesis 阶段) 或 BatchContinuationManifest
- 注册 model identity (G4.2 ModelAssignmentManifest)

**输入**: `--model <model> --role <role> [--budget <micro>] [--name <name>]`
**输出**: agent_id + pubkey + keystore 位置

**LOC**: ~250
**Class**: **2** (修改 keystore + manifest, 但仍在 genesis-phase 不写 live ChainTape)
**依赖**: `runtime::agent_keystore` / `agent_keypairs` / `real5_roles::AgentRoleAssignment`

**关键约束**: 必须在 batch start 之前执行 (genesis-phase only)；不能 hot-add agent (post-genesis 是 Class 4 surface 修改)。

#### 2.9 `turingos agent list` — 列出已部署 agent

**功能**: 解析 `agent_pubkeys.json` + GenesisReport 显示已部署 agent.

**输入**: `[--repo <path>]`
**输出**: agent 列表 (id, role, model, budget)

**LOC**: ~70
**Class**: 1
**依赖**: `runtime::agent_keystore`

#### 2.10 `turingos role assign` — 修改 agent role

**需求**: 在 genesis-phase 改 agent role。

**功能**: 修改 GenesisReport 中 `AgentRoleAssignment.role`。

**输入**: `--agent <id> --role <role>`
**输出**: 修改确认

**LOC**: ~80
**Class**: **2** (修改 genesis report; 仍 pre-batch-start)
**依赖**: `runtime::genesis_report` / `real5_roles`

**关键约束**: 必须在 batch start 之前；post-genesis role 修改是 Class 4 (会破坏 G4.2 actual-vs-genesis mismatch detector)。

#### 2.11 `turingos agent view` — 看 agent 当前视图 (role-scoped DerivedView)

**需求**: 显示 agent role 视角下的 scoped view (REAL-5 Atom 2 derive_view)。

**功能**:
- 调用 `runtime::audit_views::role_scoped_view(agent_id, head_t)`
- 显示 visible_context_cid / read_set / hidden_fields_redacted / price_signals

**输入**: `--id <agent_id> [--head <head_t>]`
**输出**: DerivedView dump (text/json)

**LOC**: ~100
**Class**: 1 (read-only view derivation)
**依赖**: `runtime::audit_views`

**场景 3 合计**: 4 个新, ~500 LOC, 2 Class 2 + 2 Class 1

---

### 场景 4: 提交 task / 投放问题 (1 个新, lean_market run-task 通用化)

#### 2.12 `turingos task open` — 通用 task open (Lean + Polymarket + custom)

**需求**: 替代 `lean_market run-task`，支持多种 task 类型。

**功能**:
- `--type lean`: 沿 lean_market run-task 逻辑 (Lean problem + bounty + spawn evaluator)
- `--type polymarket`: 创建 polymarket event (调 MarketSeedTx + EventOpen)
- `--type custom`: 通用 task open (TaskOpenTx + EscrowLockTx)

**输入**: `--type <type> [--problem <id>] [--event <event.toml>] [--bounty <micro>] [--repo <path>]`
**输出**: task_id + 启动状态

**LOC**: ~400 (含 3 个分支 + 共用 boilerplate)
**Class**: **2** (触发 system_emitted TaskOpen + EscrowLock; 走 sequencer)
**依赖**: 现有 lean_market run-task + 新增 polymarket / custom 路径

**关键约束**:
- 走现有 typed_tx (TaskOpenTx / EscrowLockTx / MarketSeed / CpmmPool)
- 不新增 typed_tx variant
- evaluator spawn 模式保留作为子选项（不强制）

**场景 4 合计**: 1 个新 (大), ~400 LOC, Class 2

---

### 场景 5: 监控 chain + 观察 agent 行为 (4 个新 watch 子命令)

#### 2.13 `turingos watch chain` — 实时 tail ChainTape

**需求**: 长时间运行场景，用户需要实时看新 tx 出现。

**功能**:
- 周期轮询 `runtime_repo` 新 commit
- 解析新 commit 中的 TypedTx + 渲染 (text/json line)
- 类似 `tail -f`

**输入**: `--repo <path> [--cas <path>] [--filter <tx_kind>]`
**输出**: tx 流 (stdout)

**LOC**: ~200
**Class**: 1 (read-only 轮询)
**依赖**: `bottom_white::ledger::transition_ledger`

**关键约束**: 不写任何状态；纯 read-only tail。

#### 2.14 `turingos watch agent` — tail 特定 agent 决策

**功能**: tail `AttemptTelemetry` / `EconomicJudgment` / `MarketDecisionTrace` for one agent.

**输入**: `--id <agent_id> [--types attempt,judgment,decision]`
**输出**: agent 决策流

**LOC**: ~180
**Class**: 1
**依赖**: `runtime::attempt_telemetry` / `economic_judgment` / `market_decision_trace`

#### 2.15 `turingos watch market` — tail market activity

**功能**: tail CPMM swap / event resolve / finalize_reward / complete_set_mint 等市场 tx.

**输入**: `[--type cpmm|event|finalize|all]`
**输出**: market tx 流

**LOC**: ~150
**Class**: 1
**依赖**: `state::price_index` + `bottom_white::ledger`

#### 2.16 `turingos watch broadcast` — tail Librarian broadcast

**功能**: tail `librarian_broadcast` emitted BroadcastEpoch / RoleNotificationView.

**输入**: `[--scope <role>]`
**输出**: broadcast 流

**LOC**: ~150
**Class**: 1
**依赖**: `runtime::librarian_broadcast`

**场景 5 合计**: 4 个新 watch, ~680 LOC, 全 Class 1 (read-only tail)

---

### 场景 6: 触发市场 (CPMM / event_resolve / finalize) (4 个新)

**关键约束**: 全部走 **system_emitted** 路径。人在 CLI 这里是 system_emitted 的间接发起者；CLI 必须用 `Sequencer::emit_system_tx` 而不是 `submit_agent_tx`，否则触发 `SystemTxForbiddenOnAgentIngress` (R-022)。

#### 2.17 `turingos market trigger seed` — 触发 MarketSeed

**需求**: 人工创建 market seed (TB-13 schema)。

**功能**: 构造 `MarketSeedTx` + 调 `sequencer.emit_system_tx`.

**输入**: `--task <id> --liquidity <yes_no,no_no> [--repo <path>]`
**输出**: tx_id + ChainTape commit hash

**LOC**: ~150
**Class**: **2** (system_emitted; 走现有 sequencer)
**依赖**: `state::sequencer::emit_system_tx` + `typed_tx::MarketSeed`

#### 2.18 `turingos market trigger cpmm-pool` — 触发 CpmmPool init

**功能**: 构造 `CpmmPoolTx` + emit system.

**输入**: `--task <id> --reserves <yes,no>`
**输出**: tx_id

**LOC**: ~150
**Class**: 2
**依赖**: `typed_tx::CpmmPool`

#### 2.19 `turingos market trigger event-resolve` — 人工裁决 polymarket event

**功能**: 构造 `EventResolveTx` + emit system.

**输入**: `--task <id> --outcome yes|no`
**输出**: tx_id

**LOC**: ~120
**Class**: 2
**依赖**: `typed_tx::EventResolve`

**关键约束**: EventResolve 是 polymarket 的 oracle 输入；在 TISR 现阶段轴 A 中，**人扮演 oracle 角色**；未来 (轴 A 扩展) 可由 agent committee 承担。

#### 2.20 `turingos market trigger finalize` — 触发 FinalizeReward

**功能**: 构造 `FinalizeRewardTx` + emit system.

**输入**: `--task <id>`
**输出**: tx_id

**LOC**: ~100
**Class**: 2
**依赖**: `typed_tx::FinalizeReward`

**场景 6 合计**: 4 个新, ~520 LOC, 全 Class 2 (system_emitted via sequencer)

---

### 场景 8: 看 EconomicJudgment / PnL / role 分化 (3 个新)

#### 2.21 `turingos report judgments` — agent EconomicJudgment 历史

**需求**: 看一个 agent 在 batch 内所有 EconomicJudgment 决策。

**功能**:
- 扫描 CAS 中 `economic_judgment.v1` schema 对象
- 过滤 agent_id
- 格式化输出 (action / reason / probability_band / abstain_reason)

**输入**: `--agent <id> [--from <batch>] [--to <batch>]`
**输出**: judgment 列表 (text/json)

**LOC**: ~150
**Class**: 1
**依赖**: `runtime::economic_judgment` + CAS scan

#### 2.22 `turingos report pnl` — PnL 走势

**功能**: 从 ChainTape 计算 agent PnL 时序 (REAL-G3 PnL evidence).

**输入**: `--agent <id> [--batch <id>]`
**输出**: PnL 时序 (text/json/csv)

**LOC**: ~180
**Class**: 1
**依赖**: `runtime::agent_pnl`

#### 2.23 `turingos report roles` — role 分化报告

**功能**: 对照 BullTrader / BearTrader / Solver / Observer 行为分布 (是否真正分化).

**输入**: `--batch <id>`
**输出**: role 分化矩阵 (text/json)

**LOC**: ~200
**Class**: 1
**依赖**: `runtime::real5_roles` + `economic_judgment` aggregation

**场景 8 合计**: 3 个新, ~530 LOC, 全 Class 1

---

### 场景 10: 出错 / 破产 / 恢复 (1 个新)

#### 2.24 `turingos resume` — 从 chaintape 恢复 batch 工作

**需求**: 中断的 batch 可以从已有 chaintape resume.

**功能**:
- 调 `resume_preflight::check` (现有 lib function)
- 通过检查后, 重新 attach 已有 chaintape + 继续后续 task
- 注意 `Sequencer fail-closes on non-empty chaintape per TB-6 NonEmptyRuntimeRepo gate` — 必须用专门的 resume 路径

**输入**: `--repo <path> --contract <path>`
**输出**: resume 状态 + 下一步操作

**LOC**: ~250
**Class**: **2** (修改 sequencer attach 路径; 涉及 NonEmptyRuntimeRepo 边界)
**依赖**: `runtime::resume_preflight` + 新增 sequencer attach mode

**关键约束**: 这是 Phase 6 的**风险点**；需要小心 NonEmptyRuntimeRepo gate; 可能需要 Codex audit; 如果发现需要 sequencer 修改，立即标 Class 4 forward-bound 并 archive 此 sub.

**场景 10 合计**: 1 个新, ~250 LOC, Class 2

---

### 场景 11: 导出 evidence / 复盘 / 回放 (1 个新)

#### 2.25 `turingos export evidence` — 打包 evidence bundle

**需求**: 一键导出完整 evidence (ChainTape + CAS + dashboard + audit + replay_report).

**功能**:
- 打包 runtime_repo + cas + agent_pubkeys + pinned_pubkeys + genesis_payload + constitution + dashboard.txt
- 输出 tar.gz
- 可选自动生成 dashboard / verify_chaintape / audit_tape 一次 evidence emission

**输入**: `--repo <path> --cas <path> --out <bundle.tar.gz> [--include-audit] [--include-dashboard]`
**输出**: tar.gz 文件

**LOC**: ~200
**Class**: 1 (filesystem 操作)
**依赖**: 上述 audit/verify/dashboard subcommand 调用

**场景 11 合计**: 1 个新, ~200 LOC, Class 1

---

## 3. 完整 25 个新增 subcommand 汇总

| # | Subcommand | 场景 | LOC | Class | 优先级 |
|---|---|---|---:|---|---|
| 2.1 | `turingos init` | 1 配置 | 150 | 1 | P0 |
| 2.2 | `turingos config set/get` | 1 配置 | 100 | 1 | P0 |
| 2.3 | `turingos spec new` | 1 配置 | 120 | 1 | P1 |
| 2.4 | `turingos batch new` | 2 batch | 80 | 1 | P0 |
| 2.5 | `turingos batch start` | 2 batch | 200 | **2** | P0 |
| 2.6 | `turingos batch list` | 2 batch | 60 | 1 | P0 |
| 2.7 | `turingos batch view` | 2 batch | 100 | 1 | P0 |
| 2.8 | `turingos agent deploy` | 3 agent | 250 | **2** | P0 |
| 2.9 | `turingos agent list` | 3 agent | 70 | 1 | P0 |
| 2.10 | `turingos role assign` | 3 agent | 80 | **2** | P0 |
| 2.11 | `turingos agent view` | 3 agent | 100 | 1 | P0 |
| 2.12 | `turingos task open` | 4 task | 400 | **2** | P0 |
| 2.13 | `turingos watch chain` | 5 watch | 200 | 1 | P1 |
| 2.14 | `turingos watch agent` | 5 watch | 180 | 1 | P1 |
| 2.15 | `turingos watch market` | 5 watch | 150 | 1 | P1 |
| 2.16 | `turingos watch broadcast` | 5 watch | 150 | 1 | P1 |
| 2.17 | `turingos market trigger seed` | 6 market | 150 | **2** | P0 |
| 2.18 | `turingos market trigger cpmm-pool` | 6 market | 150 | **2** | P0 |
| 2.19 | `turingos market trigger event-resolve` | 6 market | 120 | **2** | P0 |
| 2.20 | `turingos market trigger finalize` | 6 market | 100 | **2** | P0 |
| 2.21 | `turingos report judgments` | 8 report | 150 | 1 | P1 |
| 2.22 | `turingos report pnl` | 8 report | 180 | 1 | P1 |
| 2.23 | `turingos report roles` | 8 report | 200 | 1 | P1 |
| 2.24 | `turingos resume` | 10 recovery | 250 | **2** | P2 (风险) |
| 2.25 | `turingos export evidence` | 11 export | 200 | 1 | P1 |

**总计**: 25 个新, ~3860 LOC

**Class 分布**:
- Class 1: 17 个 (~2110 LOC, 55%)
- Class 2: 8 个 (~1750 LOC, 45%)
- **Class 3/4**: 0 个 ✅ (Kill Condition 5 不触发)

**优先级分布**:
- **P0** (Happy path 必须): 13 个 (~2330 LOC)
- **P1** (扩展功能): 11 个 (~1280 LOC)
- **P2** (风险/可选): 1 个 (~250 LOC, resume)

---

## 4. Phase 6 MVP 实施建议 (3 阶段)

### 4.1 Phase 6.1 — Bootstrap (P0 only, ~6-8 周)

13 个 P0 subcommand + ~12 个 CLI-B wrap subcommand = **25 个 subcommand MVP**.

**LOC 估算**: ~2330 (P0 新增) + ~600 (lib 化 audit_dashboard 等的 boilerplate) + ~700 (clap + 共用层) = ~3630 LOC

**Class**: 1-2

**验收**:
- Happy path 12 步可完整跑通 (CLI-A §2.2)
- 所有 P0 subcommand 单元测试 + 集成测试 covered
- 真问题 witness: 至少 1 个完整 batch (3 task / 4 agent) 走完 + 产出 evidence bundle

### 4.2 Phase 6.2 — Watch + Report (P1, ~4-6 周)

11 个 P1 subcommand (watch / report / spec / export).

**LOC 估算**: ~1280

**Class**: 1

**验收**:
- 4 个 watch 子命令在长 batch 运行下稳定 tail
- 3 个 report 子命令产出与 audit_dashboard 一致 (cross-check)

### 4.3 Phase 6.3 — Resume + Stretch (P2, 可选, ~2-4 周)

`turingos resume` (Class 2, 风险) + 任何 stretch goal.

**关键风险**: NonEmptyRuntimeRepo gate 可能要求 sequencer 修改 (Class 4); 如发生立即 archive 该功能。

**LOC 估算**: ~250

**Class**: 2 (or 4 forward-bound if blocked)

---

## 5. 关键约束 — Kill Condition 检查

### 5.1 Kill Condition 4 (>=90% 覆盖 → 缩为薄包装)

12 个现有 bin (40% 覆盖) + 25 个新增 (60% 新增) = 37 个总 subcommand。

**新增/总比 = 25/37 = 68%** — Phase 1 CLI 是显著新增设计，不是薄包装。

**不触发 Kill Condition 4** ✅

### 5.2 Kill Condition 5 (需 typed_tx 修改 → forward-bound 标记)

逐一检查 25 个新 subcommand 是否需要新 typed_tx:

- 1-11, 13-16, 21-23, 25: 不涉及 typed_tx，纯 read or config (Class 1)
- 5 (batch start): 用现有 genesis 流程, 不需要新 tx (Class 2)
- 8 (agent deploy): 修改 keystore + manifest, 不写 ChainTape (Class 2)
- 10 (role assign): 修改 genesis report, 不需要新 tx (Class 2)
- 12 (task open): 用现有 TaskOpen + EscrowLock + MarketSeed + CpmmPool (Class 2)
- 17-20 (market trigger): 用现有 MarketSeed / CpmmPool / EventResolve / FinalizeReward (Class 2)
- 24 (resume): **风险** — 可能需要 sequencer 修改 (NonEmptyRuntimeRepo 边界); 如确认走 Class 4 forward-bound

**不触发 Kill Condition 5** for 24 个；只有 #24 resume 可能触发，已标 P2 风险。

✅

### 5.3 Kill Condition 7 (deliverable 暗藏 Class 4 → Charter 补丁)

逐一检查本 CLI-C 是否暗藏 Class 4:
- 全文未提议任何新 typed_tx variant ✅
- 全文未提议任何 sequencer admission 规则变更 ✅
- 全文未提议任何 canonical signing payload 修改 ✅
- 全文未提议任何宪法修改 ✅

**不触发 Kill Condition 7** ✅

---

## 6. 与现存 in-flight 工作的协调

### 6.1 与 REAL-13 (market pressure loop) 协调

REAL-13 (in flight 2026-05-16) 是 market pressure loop 实施。如果 REAL-13 引入新 typed_tx，TISR Phase 6 应在 REAL-13 ship 之后再启动；否则 CLI 可能因为 typed_tx schema 漂移需要重写。

**协调点**: Phase 6 启动前 `git log --since="2026-05-16" -- src/state/typed_tx.rs` 验证无新 variant。

### 6.2 与 REAL-BCAST-1 (Librarian broadcast loop) 协调

REAL-BCAST-1 (in flight) 是 Librarian broadcast loop 实施。`turingos watch broadcast` 直接依赖该模块。如果 REAL-BCAST-1 在 Phase 6 启动前 ship，可直接调用稳定 API；否则 watch broadcast 应 P2 延后。

**协调点**: `turingos watch broadcast` 上线时机 = REAL-BCAST-1 ship 时机。

---

## 7. Phase 1 CLI-C 完成 — 输出 + 下一步

### 7.1 本 Track 完成

- ✅ 25 个缺失 subcommand 全列举 + LOC + Class
- ✅ 11 个用户场景全覆盖
- ✅ Class 分布: 17 Class 1 + 8 Class 2 + **0 Class 3/4** (Kill Condition 5/7 不触发)
- ✅ 3 阶段 Phase 6 实施建议 (P0 → P1 → P2)
- ✅ 与现存 in-flight (REAL-13 / REAL-BCAST-1) 协调点

### 7.2 Track CLI-D 输入

CLI-D 应规划:
- clap-based parent command 树形结构 (turingos <noun> <verb>)
- 共用 `--repo` / `--cas` / `--format` flag 系统
- 输出格式抽象 (text/json/html minimal template; html 是 Phase 5 候选)
- 错误抽象 + exit code 统一
- Lib 化重构路径 (CLI-B §4)
- 与 `turingos_dev` 并存
- 与 `lean_market` 并存 + 演化策略
- 与 in-flight REAL-13/BCAST-1 协调 timing

---

## 8. 验收 — CLI-C 出口自检

- ✅ 25 个缺失 subcommand 全列举 (LOC + Class + 优先级)
- ✅ Kill Condition 4/5/7 全检查通过
- ✅ Phase 6 MVP 3 阶段建议 (~6-14 周总周期)
- ✅ 与 in-flight 工作协调点明确
- ✅ forward-bound 头部声明
- ✅ Track CLI-D 输入 hand-off 清晰
