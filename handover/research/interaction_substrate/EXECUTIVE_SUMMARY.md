# TISR Executive Summary

**TuringOS Interaction Substrate Research (TISR-001)** | 2026-05-17

**本文档为 forward-bound research 提案；任何派生 TB 须独立 §8 ratification；本文档不构成已批准方案。**

---

## 一句话定位

> **TISR 是 TuringOS 从"防御宪法基底"走向"AGI 时代可验证 + 经济驱动 + 监管合规的 agent 操作系统底层基底" 的 forward-bound 研究**.

---

## 项目身份

| | |
|---|---|
| **项目名** | TuringOS Interaction Substrate Research |
| **代号** | TISR-001 |
| **Charter Date** | 2026-05-17 |
| **Class** | Phase 0-5 Class 0 (research docs); Phase 6+ Class 1-2 (待实施) |
| **Worktree** | `claude/tisr-2026-05-17` (物理隔离, 不动 main) |
| **架构师 carve-out** | Worktree 物理隔离 + PR 时再 surface; 不消耗 G-Phase 收口工时 |
| **当前状态** | Phase 0-5 100% ship; Phase 6+ 待 G-Phase 完成后启动 |

---

## 双轴愿景 (v2.1 阶段化)

### 轴 A — Software 3.0 HCI
**现阶段 (2026)**: 人 = constitution-level 立法者 + read-view 一等公民 + spec 批准 + init 启动
**未来 (AGI 时代)**: 留 agent 自接入扩展面 (e.g., polymarket 自治交易)

### 轴 B — Agent-to-Agent 自治通信基底
- 沿 **REAL-5** (10 角色) + **REAL-12** (EconomicJudgment CAS) + **REAL-BCAST-1** (Librarian broadcast)
- 价格信号作 **Hayek 通信原语** (Art. II.2)
- 大规模 agent 集群涌现 + 共识 + 自动合约

### 交汇命题 (v2.1)
> 人 / 人代理 agent (Agent_user_0) / agent 通过同一条 tape + 同一套谓词通信; **Signature 二元 (system/agent), ingress 三模式 (system_emitted / agent_submitted via Agent_user_0 / agent_submitted via 其他 agent)**; 人的特权在 constitution-level 而非 ingress-level.

---

## 关键发现 Top 5

1. **lean_market.rs 是 TB-10 已 ship user CLI** (848 行 / 7 subcommand)
   → Phase 6 不是 from-scratch CLI, 而是 lean_market → turingos 演化
   → 节省 ~50% LOC + 保持 TB-10 evidence chain 向后兼容

2. **TISR Phase 6 CLI 与 sequencer 0-touch**
   → 走现有 `emit_system_tx` (6 system variants) + `submit_agent_tx` via Agent_user_0
   → **0 个 Class 4 修改** (typed_tx + sequencer + cas/schema 全部 frozen)

3. **多 agent 涌现门槛 N≥16-32** (Track J 调研)
   → REAL-12 当前 batch 4-5 agent, **差 3-4x**
   → Phase 9 REAL-N (N≥16) 验证里程碑

4. **EU AI Act §12 record-keeping 几乎 spec 上要求 ChainTape 等价物** (Track I 调研)
   → TuringOS **天然踩在监管对齐风口**
   → 长期价值: 监管合规默认 + 独立第三方追责协议级保证

5. **v2 双轴命题修订为 v2.1**:
   - **v1 (已弃)**: "人 = agent 特例" — 与 sequencer.rs:4055 现状架构冲突
   - **v2.1 (最终)**: 人 = constitution-level 立法者; Signature 二元 + ingress 三模式

---

## TuringOS 三大独特价值 (vs 商业 agent platforms)

| 价值 | vs | TuringOS 立场 |
|---|---|---|
| **可验证 substrate** | LangChain / AutoGen / Replit Agent 4 | ChainTape + CAS + replay → "agent 失败可被独立第三方追责" 协议级保证 |
| **constitutional runtime** | Anthropic CAI (training-time) | 运行时强制宪法; **2025-2026 Anthropic Classifiers 与 TuringOS 收敛** ✅ |
| **内生经济市场** | Bittensor / Fetch.ai (token subsidy) | 1 Coin = 1 YES + 1 NO 守恒; 无 inflation; closed-economy arena |

---

## 项目结构 (5 Phase 研究 + 4 Phase 实施)

```
TISR 研究 ✅ 100% 完成:
  Phase 0: 基础设施 + Surface Inventory (5 docs)
  Phase 1: Unified CLI Design 4 track (4 docs)
  Phase 2: 并行架构映射 5 track (5 docs / Phase 2 parallel agents)
  Phase 3: 并行前沿调研 5 track (5 docs / Phase 3 parallel agents + WebSearch)
  Phase 4: 综合 + 差距分析 3 docs
  Phase 5: 6 份最终交付

TISR 实施 ⏳ 待 G-Phase 完成:
  Phase 6: CLI MVP (Class 1-2, ~6-8 周)
  Phase 7: Web UI MVP (Class 1-3 + cas/schema 扩展 Class 4 候选, ~10-12 周)
  Phase 8: A2A 深化 + 协作 + 联邦 (Class 2-3, ~10-12 周)
  Phase 9+: 涌现验证 + 认知扩展 + AGI 时代
```

**总实施估算**: ~26-32 周 (Phase 6-8), 2-3 工程师并行, 总 LOC ~21000 (Class 1-3, **0 Class 4 实施**).

---

## 6 份最终 Deliverable (Phase 5)

| Deliverable | 用途 | 行数 |
|---|---|---:|
| `00_UNIFIED_CLI_SPEC.md` | Phase 6 CLI MVP 实施源头规范 | 210 |
| `01_MASTER_PLAN.md` | 项目主纲领 + 长期定位 | 229 |
| `02_CONSTITUTIONAL_ALIGNMENT.md` | 每特性 vs 宪法条款映射表 | 199 |
| `03_CODE_INTEGRATION_SPEC.md` | 代码级 module 详细 + Cargo.toml | 284 |
| `04_A2A_PROTOCOL_DESIGN.md` | 轴 B 重头戏 + 4 层协议 | 412 |
| `05_ROADMAP_AND_KILL_CRITERIA.md` | 12 月路线图 + witness 清单 | 299 |

---

## 8 + 12 + 6 = 26 关键 Gap (vs 用户上传 Generative HTML 报告)

### 8 强项保留 (报告 + TISR 互证)
Turing UI IR / 4 核心架构 / HTML-first / 安全四层 / 模型路由三层 / Magentic-UI 5 核心 / Design2Code 警示 / 协作演进 (CRDT 推迟)

### 12 弱项补充 (TISR 新增)
A2A 通信 (W1) / 经济模型 (W2) / ChainTape 集成 (W3) / 多模态深度 (W4) / 可验证 AI (W5) / AGI 认知 (W6) / "人 = agent" 命题 (W7) / lean_market 现有 (W8) / G-Phase carve-out (W9) / Class 4 保护 (W10) / 真问题 witness (W11) / EU AI Act 对齐 (W12)

### 6 错误纠正
"人 = agent" 与现架构冲突 (E1) / audit_dashboard 不可扩展 (E2) / Yjs/CRDT 与 chain canonical 冲突 (E3) / MCP vs A2A 互补不替代 (E4) / React + Yjs 假设无后端约束 (E5) / 工时估算修订 (E6)

详见: `40_synthesis/gap_analysis.md`

---

## 7 个 Class 4 Forward-Bound Candidates (全标记, 0 实施)

| 候选 | Phase | 风险 |
|---|---|---|
| cas/schema.rs ObjectType +5 variants | 7 | 向后兼容 (serde default); 仍需独立 §8 |
| AgentProposedTaskOpen typed_tx | 9+ AGI | agent 自主 propose 任务; 改 admission |
| AgentMarketSeeding typed_tx | 9+ | 改 admission |
| DirectSwapTx typed_tx | 8+ | 多 agent 原子 swap; covenant 验证 |
| HumanSignature type | 9+ | 人类 PKI; signature 二元改三元 |
| 新 AgentRole variants (Designer/Curator/Editor/Auditor) | 9+ | enum + classifier 修改 |
| ReputationScorePolicyFilter (admission-bound) | 9+ AGI | reputation → admission (Goodhart 风险) |

---

## 8 Kill Conditions (持续监控)

| # | Trigger | Status |
|---|---|---|
| KC1 | 架构师 PR VETO | ⏳ 未发生 |
| KC2 | G-Phase 期间新 VETO directive | ⏳ 未发生 |
| KC3 | Track 命题被 surface 覆盖 80%+ | ✅ Phase 0 验证 OK |
| KC4 | Phase 6 实际覆盖 ≥ 90% | ⏳ 待 Phase 6 |
| KC5 | 任 Phase 需 typed_tx 修改实施 | ✅ 0 实施 |
| KC6 | Phase 3 调研 70%+ 覆盖用户报告 | ✅ 调研 12 弱项 |
| KC7 | 任 deliverable 暗藏 Class 4 | ✅ 全 forward-bound |
| KC8 | G-Phase 完成时间表给出 | ⏳ 待 architect |

---

## Phase 6 启动条件 (当前 0/7 满足)

✋ G-Phase 收口完成 (SG-G overall §8 packet)
✋ REAL-13 ship
✋ REAL-BCAST-1 ship
✋ REAL-13A ship
✋ typed_tx.rs 自 2026-05-17 起无 schema 变更
✋ Phase 6 separate charter §8 ratification (TISR research charter 之外的独立 charter)
✋ WebSearch / 实施工具预算授权

---

## 与外部 AGI 研究的关系 (Phase 3 调研)

| 方向 | TuringOS 立场 |
|---|---|
| **Karpathy Software 3.0** | 对齐 + 更激进 (协议层 vs UI 层) |
| **Anthropic CAI** | 哲学正交互补 (runtime vs training-time); 收敛中 ✅ |
| **Anthropic Effective Agents** | TuringOS = "constitutional workflow with agent-shaped LLM nodes" |
| **Google A2A 协议** | 借鉴 AgentCard 信封建模; 补充经济学 + verifiability |
| **MCP** | 工具协议 (LLM ↔ tool); 互补不替代 A2A |
| **Bittensor / Fetch.ai** | 经济模型不同 (TuringOS 内生 Coin vs token subsidy) |
| **JEPA / V-JEPA** | TuringOS tape 已是 explicit external world model; 不需 V-JEPA latent |
| **Active Inference** | 不替换 Hayek 价格信号; 可作角色 agent 内部参考实现 |

---

## 风险与约束

### 已识别风险 (Phase 5 ROADMAP §3)
- G-Phase 收口长期未完成 → TISR Phase 6+ 卡住 (KC2 缓解)
- 架构师 PR VETO TISR → archive (KC1, worktree 物理隔离, 无 main 影响)
- cas/schema.rs §8 失败 → Phase 7 fallback 用 Generic ObjectType
- 真问题 witness 跟不上 → "design over evidence"; Phase 6 强制 happy path 验收

### 硬约束 (TISR 全 Phase)
- ❌ 不修改 src/state/typed_tx.rs / sequencer.rs
- ❌ 不引入 f64 money path
- ❌ 不破坏 CompleteSet 守恒
- ❌ 不让 price 进 admission predicate
- ❌ 不引入新 signature type
- ❌ 不写自由 HTML/JS (走 Turing UI IR + Materializer)

---

## 一页结语

TuringOS v4 已是 tape-first constitutional 防御基底 (461 constitution gates green). TISR 研究阶段 (2026-05-17 ship) 在不触及任何 Class 4 surface 的前提下, 为 TuringOS 提供从"防御基底"走向"AGI 时代可验证 + 经济驱动 + 监管合规 agent 操作系统底层基底"的完整 forward-bound 路线图.

**TISR 不是"再做一个 agent 框架"; TISR 是"AGI 时代第一个可验证 + 经济驱动 + 监管合规 agent OS 底层基底" 的演化蓝图**.

实施待 G-Phase 收口 + 架构师独立 §8 ratification.

---

**详细文档导航**: 见 `INDEX.md`
**项目根**: `handover/research/interaction_substrate/` (29 docs)
**Worktree**: `claude/tisr-2026-05-17` @ HEAD `2dd4820` (= main, 无 src/tests 修改)
