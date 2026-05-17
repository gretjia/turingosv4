# TISR Philosophy — Dual-Axis Vision (v2 阶段化)

**本文档为 forward-bound research 提案；任何派生 TB 须独立 §8 ratification；本文档不构成已批准方案。**

---

## 0. 一句话哲学

**TuringOS 不是 "Software 3.0 web app + agent backend"；TuringOS 是 "宪法门控的生成式竞技场，人类与 agent 通过同一条 tape 通信，但 ingress 路径不同"。**

---

## 1. 双轴愿景

### 1.1 轴 A — Software 3.0 HCI 层 (Human-AI Interaction)

**目标**: 让 TuringOS 成为 AGI 时代人类介入 agent 生态的主入口，不让"会写 SQL"或"会读 stdout"成为门槛。

**具体期望** (现阶段 2026):
- HTML-first Workspace（或 CLI 先行）替代纯文字化 CLI 调用
- 计划、动作、证据、审批四个维度可视化（Magentic-UI 风格 co-planning / action guards / multi-tasking）
- Generative UI 通过宪法门控（不是让模型随便输出 HTML）
- 多模态、协作、回放作为一等能力

**关键约束** (现阶段 2026):
- **人 = 特权角色，不是 agent**: 人批准 spec、启动 init 模块、提交 task；人不参与 agent 间通信
- **人 = read-view 一等公民**: 任何 ChainTape / CAS / HEAD_t 状态人都能看到（受 shielding 规则）
- **人 = system-emitted 命令的间接发起者**: 人不能直接在 sequencer agent_ingress 提交 tx；人发起的工作经 system_emitted 路径
- **不引入 HumanTx ingress**: 这是 Class 4 surface 修改，超出 TISR scope

**未来扩展面** (留 hooks, 不假设成立):
- AI 时代可能比预期更快到来；polymarket 环节可能让 agent 自动接入交易
- Phase 5 ROADMAP 应预留 "agent 自接入" 升级路径，但不在 Phase 0-5 假设其落地

### 1.2 轴 B — Agent-to-Agent 自治通信基底 (Multi-Agent Substrate)

**目标**: 让 agents 在同一条 tape 上自由交通、交汇、沟通，**不需要人类参与**。

**基底已存在** (Phase 0 inventory 将量化):
- `src/runtime/real5_roles.rs` — 8 个角色（Solver / Verifier / Challenger / Trader / MarketMaker / Architect / Veto / Observer / Bull / Bear / Librarian）
- `src/runtime/economic_judgment.rs` — REAL-12 Bull/Bear EconomicJudgment CAS schema
- `src/runtime/librarian_broadcast.rs` — REAL-BCAST-1 Librarian broadcast loop (in flight)
- `src/runtime/market_decision_trace.rs` — MarketDecisionTrace + NoTradeReason
- `src/state/typed_tx.rs` — 19 typed_tx variants（含 CPMM / EventResolve / FinalizeReward）
- ChainTape (L4/L4.E) + CAS + HEAD_t — agent 行动可观察、可审计、可回放
- `constitution.md` Art. II.2 — 广播价格信号（Hayek 式通信原语）

**TISR 期望深化** (现阶段 research, 不实施):
- 价格作为 universal 通信原语的形式化（不是 RPC，不是 message-passing；是 market signal）
- Mechanism Design for AI Agents — 拍卖、博弈、合约、声誉
- 大规模 agent 集群的 emergence 条件（什么时候 N agent → swarm behavior）
- A2A 协议层与 MCP 的关系（MCP = tool protocol；A2A = agent protocol；区别 + 互补）

**关键约束**:
- **价格 ≠ 真相**: 沿 REAL-9 whitepaper verbatim "price = signal, not truth"
- **不强迫交易**: agent 可以 NoTradeReason; 沿 REAL-5/REAL-12 设计
- **不记录私有 CoT**: 沿宪法 Art. III shielding
- **不要 ghost liquidity / f64 money path**: 沿宪法 Art. 0 + CLAUDE.md §13

---

## 2. 双轴交汇命题 (v2 修订版)

### 2.1 错误命题 (v1, 已弃)

~~"人 = agent 特例"~~

**为什么错**: 与 `src/state/typed_tx.rs:2327` + `src/state/sequencer.rs:4055` 现状架构不兼容。要落地必须引入 HumanTx 第三 ingress 或包装 human 为特殊 agent — 都是 Class 4 surface 修改。

### 2.2 正确命题 (v2)

**"人 = 特权角色（spec 批准 + init 启动）；agent = 工作角色（在 tape 上行动）；两者通过同一条 tape + 同一套谓词通信，但 ingress 路径不同"**

#### 形式化

```
Ingress(human) := system_emitted_path
  人通过 CLI/UI → system_tx 发起 init / batch / task / approval
  人在 sequencer 角度是 "system actor"

Ingress(agent) := agent_submitted_path
  agent 通过 SDK → typed_tx 提交 work / market / verify
  agent 在 sequencer 角度是 "agent actor"

Observation(human) := read_view (with shielding)
  人可以看 ChainTape / CAS / HEAD_t / Dashboard

Observation(agent) := scoped_view (role-based)
  agent 按 role 看到不同的 scoped view (DerivedView)

Common-substrate := { ChainTape, CAS, Predicates, Constitution }
  两者共享同一条 tape + 同一套谓词
```

#### 哲学意义

- **人是主人**: 现阶段 spec 批准 + init 启动是人的特权（CLAUDE.md §10 Class 4 §8 sign-off 体现）
- **Agent 是工作者**: 在 tape 上自治行动，但 ingress 路径受限于 sequencer admission
- **共享基底**: 不是两套独立系统，而是 **一条 tape + 两种 ingress**
- **未来留余**: AI 时代来临后，人的特权可能逐步移交给 agent (e.g., polymarket 自治)；当时再 promote 命题到 v3

### 2.3 v2 命题的工程含义

- TISR Phase 4-5 deliverables 默认使用 v2 命题
- Phase 5 `04_A2A_PROTOCOL_DESIGN.md` 设计 agent 间通信时，**不**包含 "人也用这套协议"
- Phase 5 `00_UNIFIED_CLI_SPEC.md` 设计 CLI 时，**默认**人走 system_emitted 路径
- 未来 (Phase 7+ 轴 A 深化) 若需引入 HumanTx，必须 Class 4 §8 separate ratification

---

## 3. 为什么 TuringOS 适合做这件事

### 3.1 现有 TuringOS 已经具备的基底

(Phase 0 inventory 将量化)

| 基底能力 | 来源 | 与 TISR 关系 |
|---|---|---|
| Tape (L4/L4.E) + CAS + HEAD_t | `src/state/transition_ledger.rs` + `src/bottom_white/cas/store.rs` | TISR 不动；作为通信底层 |
| 8 个 Agent Role | `src/runtime/real5_roles.rs` + `agent_role_classifier.rs` | TISR 扩展（如 Designer / Curator）但保留原结构 |
| Economic Judgment CAS | `src/runtime/economic_judgment.rs` (REAL-12) | TISR 扩展（如 UIJudgment）但保留 schema 框架 |
| Librarian Broadcast | `src/runtime/librarian_broadcast.rs` (REAL-BCAST-1) | TISR 直接用 |
| Predicate / Lean / PCP | `src/sdk/predicates/` + `tests/fc_alignment_conformance.rs` | TISR 扩展（UI/A2A 谓词）但保留 PCP soundness |
| Constitution 三权分立 | Art. V (Constitution / ArchitectAI / VetoAI) | TISR 保留；ArchitectAI 提案 + VetoAI 否决 + Constitution 基准 |

### 3.2 TuringOS 与外部 agent 框架的关键差异

| 维度 | LangGraph / AutoGen / Crew AI | TuringOS |
|---|---|---|
| Agent 行为记录 | 内存 / log / 偶尔向量库 | **ChainTape L4 accepted + L4.E rejected + CAS 高维证据** |
| Agent 间通信 | RPC / message passing | **价格信号 + tape 共享 + 谓词广播** |
| 多 agent 协调 | 编排框架 (graph / chain) | **市场涌现 + 角色分化 + 经济激励** |
| Verifiability | 通常无 | **Lean / PCP 谓词 + replay verifier + audit_tape** |
| Determinism | 通常无 | **HEAD_t 单调推进 + bit-exact replay** |
| Economic discipline | 无 | **CompleteSet (1 Coin = 1 YES + 1 NO) + 守恒 + 选择性广播** |

**结论**: TuringOS 已经是 **AGI 时代多 agent 自治通信基底的可信替代方案**。轴 B 不是无中生有的设计；是把已有基底从 "证明 agent" 推广到 "完整 agent 经济社会"。

---

## 4. AGI 时代的预期演化

### 4.1 短期 (2026)
- TISR Phase 0-5: research 文档
- TISR Phase 6: CLI MVP 上线，人通过 CLI 接入 TuringOS 全功能
- G-Phase 收口（SG-G overall §8）

### 4.2 中期 (2027-2028)
- HTML-first Workspace (轴 A 深化)
- Multi-LLM agent persistent identity (G4.2 已落地，深化)
- 真问题 benchmark 上规模化（MiniF2F / 数学竞赛 / 软件工程 task）
- Agent 间合约自动化 (轴 B 深化)

### 4.3 长期 (2029+)
- 人逐步从 "spec 批准者" 升级为 "宪法订立者"
- Agent 从 "工作者" 升级为 "市场参与者 + 部分自治"
- TuringOS 作为 AGI 社会的 verifiable substrate
- 跨 TuringOS 联邦（多个 instance 互通；类似多链）

### 4.4 杀手风险
- 真问题 witness 跟不上设计速度 → 沦为 design-over-evidence
- 架构师指令冲突未及时 surface → 主线分裂
- Class 4 surface 误以为已 ratify → 引发系统性回归

TISR 的责任: **保持 ideation 与 evidence 同步推进**；不让"哲学先行"变成"工程债务"。

---

## 5. 与已存在哲学的对照

### 5.1 vs Karpathy "Software 3.0"
- **共同**: 上下文即程序 (context-as-program)；agent-native surfaces
- **差异**: Karpathy 强调 "vibe coding → agentic engineering"，TuringOS 强调 **"vibe → tape → predicate → constitution"**

### 5.2 vs Anthropic "Constitutional AI"
- **共同**: 宪法作为对齐机制
- **差异**: Anthropic constitution = training-time alignment；TuringOS constitution = **runtime alignment + verifiable replay**

### 5.3 vs Hayek "Use of Knowledge in Society"
- **共同**: 价格作为分散知识的通信协议
- **差异**: Hayek 在自由市场上；TuringOS 在 **宪法门控 + 谓词验证** 的市场上 (constitution-gated arena)

### 5.4 vs Turing "Computing Machinery and Intelligence"
- **共同**: tape + state transition + verifiable computation
- **差异**: Turing 单机；TuringOS **多 agent + 经济激励 + 宪法门控**

---

## 6. 结语

TISR 不是一个 "前端项目" 或 "对话产品"；TISR 是 **追问 TuringOS 这条 tape 上能否承载 AGI 时代人机协同 + agent 自治通信的研究**。

如果答案是 "不能"，TISR 应该诚实地说不能。
如果答案是 "能，但需要 Class 4 修改"，TISR 应该把那些修改具体写出来，作为后续 separate charter 的源。
如果答案是 "能，且当前架构已够"，TISR 应该把现有架构如何接通用户视角的工作流写出来（这正是 Phase 1 CLI 的任务）。

**"我不要凑活"** (user verbatim) — TISR 严格自检：每个 Phase 出口都问 "这是凑活，还是真的在路上？"
