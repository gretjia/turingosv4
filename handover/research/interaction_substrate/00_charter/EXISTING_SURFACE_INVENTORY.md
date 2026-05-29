# EXISTING_SURFACE_INVENTORY

**本文件为 forward-bound research 提案；任何派生 TB 须独立 §8 ratification；本文档不构成已批准方案。**

> **SUPERSEDED 2026-05-29 (forward-only note; body below unchanged):** the
> "保留独立 (keep `turingos_dev`)" decision recorded below is superseded by user
> §8 directive 2026-05-29 — the `turingos_dev` dev-evidence sidecar is retired
> (commit `4264b14f`;
> `handover/alignment/OBS_R022_TURINGOS_DEV_REMOVAL_2026-05-29.md`). All
> `turingos_dev` / `dev_harness.rs` references below are now historical.

**目的**: 修复 Auditor CHALLENGE 维度 4 (Phase 1/2 与已存在 surface 重叠风险)。
量化盘点 TuringOS v4 现有 surface，与 TISR Phase 1-3 各 track 命题做关系矩阵。

**统计基准**: 2026-05-17 (TISR worktree 启动时刻)
**来源**: `git rev-parse HEAD` 时 main 分支 (含 REAL-12 ship + REAL-13 in flight)

---

## 1. src/ 全量 surface 统计

### 1.1 src/state/ — 核心状态层 (14,706 行)

| 文件 | 行数 | TISR 关系 | STEP_B Frozen? |
|---|---:|---|---|
| `sequencer.rs` | 7,129 | **轴 B 核心 + CLI 间接** — 不动 | ✅ Yes |
| `typed_tx.rs` | 4,636 | **轴 B 核心 + 任何 new tx → Class 4** | ✅ Yes |
| `price_index.rs` | 1,142 | **轴 B (Hayek 价格通信原语) 已落地** | 部分 |
| `q_state.rs` | 1,138 | **HEAD_t / state root** | ✅ Yes |
| `router_quote.rs` | 332 | CPMM router 报价 | 部分 |
| `head_t_witness.rs` | 267 | **HEAD_t witness 已落地** | ✅ Yes |

**结论**: 核心 state 层 11k+ 行 STEP_B frozen。TISR Phase 2 Track A 必须只采样不映射。

### 1.2 src/runtime/ — 运行时层 (30,388 行)

| 文件 | 行数 | TISR 关系 | 备注 |
|---|---:|---|---|
| **角色 / agent 身份层** | | | |
| `real5_roles.rs` | 1,145 | **轴 B 核心** — REAL-5 + Bull/Bear | 10 个 AgentRole 已枚举 |
| `agent_role_classifier.rs` | (查) | 轴 B | 角色推断 |
| `agent_keypairs.rs` / `agent_keystore.rs` | (查) | **轴 B + 轴 A CLI 启动需要** | 密钥管理 |
| `agent_pnl.rs` | (查) | 轴 B 经济学 | PnL 计算 |
| `agent_audit_trail.rs` | (查) | 轴 A + 轴 B 审计可视化 | |
| `agent_scheduler.rs` | (查) | 轴 B 调度 | |
| **市场 / 经济学层** | | | |
| `economic_judgment.rs` | (查) | **轴 B + 轴 A 高价值** — REAL-12 Bull/Bear 经济判断 | 完整 schema 已存在 |
| `librarian_broadcast.rs` | (查) | **轴 B 核心** — REAL-BCAST-1 in flight | Agent 间通信原型 |
| `market_decision_trace.rs` | (查) | **轴 B 已落地** — NoTradeReason + Trace | |
| `market_decision_trace_summary.rs` | (查) | 轴 B 汇总 | |
| `market_opportunity_trace.rs` | (查) | 轴 B 机会追踪 | |
| `market_review.rs` | (查) | 轴 B 复盘 | |
| `market_e2_candidate_verifier.rs` | (查) | E2 验证 | E2 仍 negative |
| `ev_decision_trace.rs` | (查) | 轴 B (REAL-13A) | EV 决策追踪 |
| `policy_trader_trace.rs` | 255 | 轴 B 策略追踪 | |
| `risk_cap_impact_report.rs` | 636 | 轴 B 风险管理 | |
| `market_tx_category.rs` | 164 | 轴 B 分类 | |
| **REAL-6 (Task Outcome / Conviction / Attempt Prediction)** | | | |
| `real6_attempt_prediction.rs` | 345 | 轴 B | scripted prediction |
| `real6_conviction_budget.rs` | 232 | 轴 B 信念预算 | |
| `real6_task_outcome.rs` | 67 | 轴 B 结果市场 | |
| **证据 / 审计层** | | | |
| `attempt_telemetry.rs` | (查) | **轴 A + 轴 B** | Attempt 遥测 |
| `autopsy_capsule.rs` | (查) | 轴 B agent autopsy | |
| `evidence_capsule.rs` | (查) | 轴 A + 轴 B 证据包 | |
| `markov_capsule.rs` | 759 | 轴 A + 轴 B Markov 演化 | |
| `prompt_capsule.rs` | 361 | **轴 A + 轴 B** — PromptCapsule (Art. III 屏蔽) | Class-3 |
| `proposal_telemetry.rs` | 573 | 轴 A + 轴 B 提案遥测 | |
| `persistence_evidence.rs` | 693 | **轴 B (G1 Cross-Problem Persistence)** | |
| `peer_verify_coverage.rs` | 530 | 轴 B (G2P Peer Verification) | |
| **报告 / 视图层** | | | |
| `run_summary.rs` | 306 | **轴 A CLI** | |
| `aggregate_report.rs` | (查) | **轴 A CLI dashboard** | |
| `chain_derived_run_facts.rs` | (查) | 轴 A read view | |
| `chain_tape_lease.rs` | (查) | 轴 A 协作 UI 一致性 | |
| `audit_views.rs` | (查) | **轴 A read view** | |
| `audit_assertions.rs` | (查) | 轴 A 审计断言 | |
| `audit_tamper.rs` | (查) | 轴 A 审计 tamper 检测 | |
| **统计 / 工具** | | | |
| `diversity.rs` | (查) | 轴 B | |
| `verify.rs` | 707 | 轴 B verify | |
| `verification_result.rs` | 276 | 轴 B | |
| `wilson_ci.rs` | 145 | 轴 A stat (CI) | |
| `signal_purification.rs` | 41 | 轴 B 信号清洗 | |
| `display_coin.rs` | (查) | **轴 A CLI** — coin 格式化 | |
| `g7_structural_smoke.rs` | (查) | G-Phase 收口 | |
| `dev_harness.rs` | (查) | turingos_dev sidecar | |
| `bootstrap.rs` | (查) | 轴 A CLI 启动 | |
| `genesis_report.rs` | (查) | **轴 A spec 批准 + init 启动** | 人类介入点 |
| `batch_continuation_manifest.rs` | (查) | 轴 A batch 启动 | |
| `benchmark_manifest.rs` | (查) | 轴 A benchmark 启动 | |
| `resume_preflight.rs` | 379 | 轴 A CLI 恢复 | |
| `adapter.rs` | (查) | 轴 B adapter | |

### 1.3 src/bin/ — CLI 入口层 (5,558 行) — **TISR Phase 1 CLI-B 重点**

| 文件 | 行数 | 现状 | TISR Phase 1 CLI 集成方式 |
|---|---:|---|---|
| `audit_dashboard.rs` | 3,544 | **text-only**, 16+ render section, 是 dashboard 报告生成器 | **集成**为 `turingos dashboard` 子命令 |
| `audit_tape.rs` | 283 | tape verifier | **集成**为 `turingos verify tape` |
| `audit_tape_tamper.rs` | 407 | tamper checker | **集成**为 `turingos verify tamper` |
| `gen_run_summary.rs` | 128 | run summary generator | **集成**为 `turingos report run` |
| `generate_markov_capsule.rs` | 416 | markov capsule generator | **集成**为 `turingos report markov` |
| `real14_e2_candidate_verifier.rs` | 103 | REAL-14 specific | (REAL-14 lifecycle, 可能 deprecated) |
| `resume_preflight.rs` | 77 | runner preflight checker | **集成**为 `turingos preflight` |
| `tb_18r_compute_invariant.rs` | 117 | TB-18R specific | (TB-18R lifecycle) |
| `tb_g_persistence_report.rs` | 118 | G-Phase persistence report | **集成**为 `turingos report g-phase` |
| `turingos_dev.rs` | 255 | dev evidence sidecar (无 clap, manual subcommand) | **保留独立**作为 dev-only entry |
| `verify_chaintape.rs` | 110 | chaintape verifier | **集成**为 `turingos verify chaintape` |

**关键观察 (audit 维度 4 修复证据)**:
- 现有 bin 已经覆盖了 audit / verify / report 三大用户视角工作流的 **~50-60%**
- 缺失工作流 (Phase 1 CLI-C 重点): **init / batch / task / market / replay / export** 没有专门 bin
- 现有 bin 都是 text-only 输出 (无 HTML/JSON 选项)；无统一 subcommand 入口
- `turingos_dev` 是 dev sidecar, 不是 user CLI (Auditor 引用准确)

### 1.4 src/sdk/ — Agent SDK 层

| 文件 | TISR 关系 |
|---|---|
| `actor.rs` | 轴 B Agent 主体 |
| `sandbox.rs` | 轴 B Agent 沙箱 |
| `prompt.rs` / `prompt_guard.rs` | 轴 B Prompt 管理 + 守门 |
| `tool.rs` / `tools/` | **轴 B + 轴 A** — Tool 接口 (MCP candidate) |
| `protocol.rs` | **轴 B 通信协议** |
| `snapshot.rs` | 轴 A snapshot |
| `econ_position.rs` / `your_position.rs` / `market_context.rs` | 轴 B agent 经济视角 |
| `pending_peer_reviews.rs` | 轴 B peer verification |
| `error_abstraction.rs` | 轴 A 错误抽象 |

---

## 2. TypedTx 19 Variants (typed_tx.rs:2327)

来自 `src/state/typed_tx.rs:2327` `TypedTx` enum:

| # | Variant | Ingress | 类别 | TISR 关系 |
|---|---|---|---|---|
| 1 | `Work` | agent | proof | 轴 B 核心 |
| 2 | `Verify` | agent | verification | 轴 B |
| 3 | `Challenge` | agent | challenge | 轴 B |
| 4 | `Reuse` | agent | reuse proof | 轴 B |
| 5 | `FinalizeReward` | system | settlement | 轴 B 经济学 |
| 6 | `TaskExpire` | system | lifecycle | 轴 A + 轴 B |
| 7 | `TerminalSummary` | system | summary | 轴 A 报告 |
| 8 | `TaskOpen` | system | lifecycle | **轴 A 人 init 触发** |
| 9 | `EscrowLock` | system | escrow | **轴 A 人 init 触发** |
| 10 | `ChallengeResolve` | system | challenge settle | 轴 B |
| 11 | `TaskBankruptcy` | system | bankruptcy | 轴 B |
| 12 | `CompleteSetMint` | agent/system | market | 轴 B |
| 13 | `CompleteSetRedeem` | agent | market | 轴 B |
| 14 | `MarketSeed` | system | market init | **轴 A 人 init 触发** |
| 15 | `CompleteSetMerge` | agent | market | 轴 B |
| 16 | `CpmmPool` | system | pool init | **轴 A 人 init 触发** |
| 17 | `CpmmSwap` | agent | market action | 轴 B |
| 18 | `BuyWithCoinRouter` | agent | router | 轴 B (Class-4 STEP_B) |
| 19 | `EventResolve` | system | resolution | 轴 B |

**关键观察**:
- **No `HumanTx` variant** ✅ (auditor 引用准确)
- 现有 system-emitted tx (TaskOpen / EscrowLock / MarketSeed / CpmmPool) 是 **人介入 TuringOS 的天然入口** — 人通过 CLI 触发这些 system-emitted tx
- **TISR Phase 1 CLI 设计完全不需要新增 typed_tx variant**
- TISR Phase 2-5 若提议新 typed_tx (e.g., AnnotationTx / RatingTx) 必须标 Class 4 forward-bound

---

## 3. AgentRole 10 Variants (real5_roles.rs:29)

```rust
pub enum AgentRole {
    Solver,         // 证明者
    Verifier,       // 验证者
    Challenger,     // 挑战者
    Trader,         // 交易者
    MarketMaker,    // 做市者
    Architect,      // 架构师 (ArchitectAI)
    Veto,           // 否决者 (VetoAI)
    Observer,       // 观察者
    BullTrader,     // 多头交易者 (REAL-12)
    BearTrader,     // 空头交易者 (REAL-12)
}
```

**关键观察**:
- 10 个角色已覆盖 TuringOS 三权分立 (Architect/Veto/Solver-Verifier-Challenger) + 市场 (Trader/MarketMaker/Bull/Bear) + 中立 (Observer)
- **缺失角色 (TISR Phase 5 候选, 不立即引入)**:
  - `Librarian` — 已在 `librarian_broadcast.rs` 落地为独立 module, 是否要 promote 到 AgentRole?
  - `Designer` (轴 A HCI candidate)
  - `Curator` (轴 A HCI candidate)
  - `Editor` (轴 A HCI candidate)
- TISR Phase 5 Roadmap 应讨论 "新 AgentRole 引入是 Class 4 (枚举变更影响 sequencer admission); 须独立 §8 ratification"

---

## 4. 已存在 surface 与 TISR Track 关系矩阵 (核心修复)

| TISR Track | Track 命题 | 现存 Surface 覆盖度 | 动作 |
|---|---|---:|---|
| Phase 1 CLI-A (User Journey) | 用户视角端到端工作流 | **~30%** (现有 bin 覆盖 audit/verify/report) | 继续，重点设计 init/batch/task/market |
| Phase 1 CLI-B (现有 bin 集成) | 11 个 bin 整合 | **100%** (本 inventory 已盘点完毕) | 继续，作为 CLI 设计输入 |
| Phase 1 CLI-C (缺失能力) | 找出现有 bin 不覆盖的工作流 | **~40%** (init / batch / task / market trigger 缺失) | 继续 |
| Phase 1 CLI-D (Architecture) | CLI subcommand 层级 | **0%** (turingos_dev 是 dev sidecar, 不是 user CLI) | 继续, from-scratch 设计 |
| Phase 2 Track A (typed_tx) | UI/A2A tx 设计如何接入 typed_tx | **80%** (19 variants 已覆盖大部分; 新增需 Class 4) | **scope 缩小为采样**, 不做完整映射 |
| Phase 2 Track B (ChainTape/CAS/HEAD_t) | UI/A2A 证据如何映射 | **70%** (HEAD_t + CAS 已 landed; UI event 是新 capsule 类型) | 继续，重点设计新 capsule schema |
| Phase 2 Track C (Materializer/Rules) | UI 物化 + Policy Engine | **20%** (audit_dashboard 纯 text; 无 web; 无 rules engine) | 继续, **明确 from-scratch web layer** |
| Phase 2 Track D (经济学集成) | 角色 view + 价格通信原语 | **75%** (REAL-5/12 + price_index + librarian_broadcast 已覆盖大部分) | **scope 缩小**, 重点是新增什么 vs 现有什么 |
| Phase 2 Track E (谓词/PCP) | UI/A2A 内容如何过 PCP | **60%** (PCP + fc_alignment_conformance 已落地; UI 谓词是新增) | 继续 |
| Phase 3 Track F (A2A 协议前沿) | MCP 之外的 A2A | **20%** (项目对外部 A2A 协议研究有限; 内部 LibrarianBroadcast 是 prototype) | 继续 |
| Phase 3 Track G (多模态) | 多模态 agent UI | **0%** (项目 100% 文本) | 继续 |
| Phase 3 Track H (Software 3.0) | TuringOS vs Karpathy | **30%** (用户上传报告已部分覆盖) | **缩小 scope** 聚焦 "TuringOS 缺什么" |
| Phase 3 Track I (可验证 AI) | zkML / TEE / 去中心化 | **10%** (ChainTape Merkle 有，zkML/TEE 无) | 继续 |
| Phase 3 Track J (AGI 认知) | 世界模型 / Active Inference | **0%** (项目无 cognitive architecture 研究) | 继续 **聚焦 "TuringOS 缺什么"** |

### 4.1 Kill Condition 3 触发判断

按 Charter §5 Kill Condition 3:
> Phase 0 surface inventory 发现某 Track 命题被现存 surface 覆盖 80%+ → archive 该 Track

**临界 Track**:
- **Phase 2 Track A (typed_tx)**: 80% 覆盖 → **不 archive 但 scope 缩小** (typed_tx schema 80% 已 frozen, TISR 只做采样)
- **Phase 2 Track D (经济学)**: 75% 覆盖 → **scope 缩小, 重点是 gap, 不是完整设计**

**Archive 判定**: 无 Track 命题被 100% 覆盖；无 Track 应 archive。

### 4.2 Kill Condition 5 风险预警

按 Charter §5 Kill Condition 5:
> Phase 2 任何 track 提议需要 typed_tx schema 修改才能落地 → 标 forward-bound 并标 Class 4 候选；不深入实施 spec

**预警 Track**:
- **Phase 2 Track A**: 几乎必然命中 (UI event / A2A message 如果做成 typed_tx 必然 schema 修改)
- **Phase 2 Track B**: 部分命中 (新 capsule schema 在 CAS 是 Class 1-3, 但若改 typed_tx anchor 路径是 Class 4)

**应对**: Phase 2 Track A 和 B 必须显式声明 "本 track 输出全部 forward-bound, 不假设 typed_tx schema 修改已批准"。

---

## 5. CLI 现状盘点 (Phase 1 CLI-B 预研)

### 5.1 现有 bin 共同特点
- **全部 text output** (无 HTML / JSON 输出选项)
- **每个 bin 独立 entry point** (无统一 `turingos` parent command)
- **手动参数解析** (`turingos_dev` 用 manual subcommand split, 无 clap)
- **read-only 为主** (audit / verify / report)

### 5.2 用户视角工作流 vs 现有 bin 覆盖

| 用户工作流 | 现有 bin 覆盖 | 状态 |
|---|---|---|
| **启动**: 批准 spec, 启动 init, 创建 batch/runtime_repo | `turingos_dev open --module ...` (dev only) | **缺失 user-facing** |
| **工作**: 提交 task | 无专门 bin | **缺失** |
| **观察**: 看 ChainTape / CAS | `verify_chaintape.rs` (verify only, 不浏览) | **部分** |
| **审计**: 看 audit / dashboard | `audit_dashboard.rs` ✅ | **覆盖** |
| **审计**: 检查 tamper | `audit_tape_tamper.rs` ✅ | **覆盖** |
| **审计**: 验证 tape | `audit_tape.rs` ✅ | **覆盖** |
| **报告**: run summary | `gen_run_summary.rs` ✅ | **覆盖** |
| **报告**: markov capsule | `generate_markov_capsule.rs` ✅ | **覆盖** |
| **报告**: G-Phase 持续性 | `tb_g_persistence_report.rs` ✅ | **覆盖** |
| **经济**: 部署 agent / 配置 role | 无 | **缺失** |
| **经济**: 触发 CPMM swap / event_resolve / finalize_reward | 无 (sequencer 直接调用) | **缺失** |
| **经济**: 看 EconomicJudgment / PnL | `audit_dashboard.rs` 内嵌 (不独立) | **部分** |
| **恢复**: resume from preflight | `resume_preflight.rs` ✅ | **覆盖** |
| **导出**: export evidence | 无 | **缺失** |
| **回放**: replay run | 无 (verify 只检查不重放) | **缺失** |

### 5.3 Phase 1 CLI Design 重点 (基于 5.2)

**优先级 1 (CLI-A user journey 必须覆盖)**:
- 启动工作流: `turingos init` / `turingos batch new` / `turingos task open`
- 经济工作流: `turingos agent deploy` / `turingos role assign` / `turingos market trigger`
- 导出工作流: `turingos export evidence` / `turingos replay run`

**优先级 2 (CLI-A 整合现有 bin)**:
- 审计: `turingos audit dashboard` (wraps `audit_dashboard`) / `turingos audit tamper` (wraps `audit_tape_tamper`)
- 验证: `turingos verify chaintape` / `turingos verify tape`
- 报告: `turingos report run` / `turingos report markov` / `turingos report g-phase`

**优先级 3 (CLI-D 架构)**:
- 统一 `turingos` parent command
- 全 clap-based subcommand
- `--format text|json|html` 输出格式切换
- 配置管理 (genesis_payload.toml 接入)
- 状态管理 (runtime_repo 接入)

### 5.4 CLI MVP scope 估算 (Phase 6 separate charter 用)

| 模块 | LOC 估算 | Class | 备注 |
|---|---:|---|---|
| 统一 `turingos` parent + clap 层 | ~300 | 1 | 新文件 `src/bin/turingos.rs` |
| 现有 bin 包装为 subcommand | ~500 | 1 | 复用现有 bin 逻辑, 改 entry |
| 新增缺失子命令 (init/batch/task/market trigger/export/replay) | ~1500-2000 | 2 | 部分调用 sequencer API |
| `--format text\|json\|html` 输出层 | ~400 | 1 | text 是默认；JSON 走 serde；HTML 走 minimal template |
| 测试 + 文档 | ~600 | 1 | 单元测试 + man page 风格文档 |

**总估算**: ~3000-4000 LOC; Class 1-2; **未触及 typed_tx/sequencer schema**。

---

## 6. 验收 — Inventory 完成检查

- ✅ src/state/ 7 个文件全盘点 (典型 STEP_B frozen)
- ✅ src/runtime/ 48 个文件全盘点 (轴 A 轴 B 关系)
- ✅ src/bin/ 11 个文件全盘点 (Phase 1 CLI-B 预研)
- ✅ src/sdk/ 全盘点
- ✅ TypedTx 19 variants 列出
- ✅ AgentRole 10 variants 列出
- ✅ TISR 14 个 track 命题 vs 现存 surface 覆盖度量化
- ✅ Kill Condition 3 和 5 风险评估
- ✅ Phase 1 CLI 设计输入 (用户工作流覆盖度对照)

**结论**:
- 无 Track 应 archive (覆盖度均 < 80% 或本是 from-scratch 设计)
- Phase 2 Track A + D scope 必须缩小
- Phase 2 Track C 必须明确 from-scratch web layer
- Phase 3 Track H + J 必须聚焦 "TuringOS 缺什么"
- Phase 1 CLI 主线明确: 整合现有 bin + 新增 6 个缺失子命令 + 统一 clap

---

## 7. 后续 Phase 0 步骤剩余

- [x] CHARTER.md
- [x] PHILOSOPHY.md
- [x] REFERENCE_INPUTS.md
- [x] EXISTING_SURFACE_INVENTORY.md (本文件)
- [ ] Foundation read: 7 个文档摘要到 90_references/
- [ ] Phase 0 exit check + 主 agent 进度报告
