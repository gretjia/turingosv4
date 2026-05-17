# Phase 4 — Phased Dual-Axis Synthesis

**本文档为 forward-bound research 提案；任何派生 TB 须独立 §8 ratification；本文档不构成已批准方案。**

**目的**: 验证 TISR v2 阶段化双轴命题 (PHILOSOPHY.md) 在 TuringOS 实际架构下的可行性、张力点、未来演化路径; Phase 5 `01_MASTER_PLAN.md` 的核心输入.

---

## 1. v2 双轴命题回顾

### 1.1 阶段化版本 (PHILOSOPHY.md v2)

**轴 A — Software 3.0 HCI**:
- **现阶段 (2026)**: 人 = 特权角色 (spec 批准者 + init 启动者 + read-view 一等公民); 走 system_emitted ingress 间接发起
- **未来 (AGI 时代)**: 留 agent 自接入扩展面 (e.g., polymarket 自治交易); 不在现阶段假设其成立

**轴 B — Agent-to-Agent 自治通信基底**:
- 沿 REAL-5 (10 角色) + REAL-12 (EconomicJudgment) + REAL-BCAST-1 (Librarian broadcast)
- 价格信号作 Hayek 通信原语
- 大规模 agent 集群涌现 + 共识 + 自动合约

### 1.2 v2 交汇命题

> **"人 = 特权角色（spec 批准 + init 启动）；agent = 工作角色；两者通过同一条 tape + 同一套谓词通信，但 ingress 路径不同"**

---

## 2. 命题验证: 每条 sub-claim 的 TISR 调研支撑

### 2.1 Claim "人是特权角色, 走 system_emitted ingress"

**验证来源**: Track A Q1
- `sequencer.rs:4208-4235` 的 `emit_system_tx(SystemEmitCommand)` 是 system_emitted 唯一入口 ✅
- 6 个 SystemEmitCommand variants (FinalizeReward / TaskExpire / TerminalSummary / ChallengeResolve / TaskBankruptcy / EventResolve) 完全覆盖人触发市场 / 结算 / 仲裁需求 ✅
- 不需要新 typed_tx variant ✅
- Phase 6 CLI `turingos market trigger ...` 命令走此路径 ✅

**Track A Q4 详细路径追溯**:
- lean_market `run-task` 实际是 **agent_submitted** (Agent_user_0 签署), 不是 system_emitted
- Phase 6 CLI 应支持**两条路**:
  - 真 system_emitted (人作系统裁决者 — finalize / event_resolve / task_expire)
  - 通过 Agent_user_0 走 agent_submitted (人作 spec 提交者 — TaskOpen / EscrowLock 走 agent_user_0)

**命题精炼**: 人有**两种 ingress 路径**:
1. **系统裁决者** (system_emitted): 触发 finalize / resolve / expire
2. **Spec 提交者** (agent_submitted via Agent_user_0): 触发 TaskOpen / EscrowLock / MarketSeed (现 lean_market 模式)

→ v2 命题"人走 system_emitted ingress"应**修订**为"人走 system_emitted 或通过 Agent_user_0 走 agent_submitted; 两种都不需要新 ingress type"

### 2.2 Claim "agent 是工作角色, 走 agent_submitted ingress"

**验证来源**: Track A + Phase 0 inventory
- REAL-5 10 个 AgentRole 全部 agent-submitted ✅
- 13 个 agent-signed typed_tx variants 全部支持 agent 自治行动 ✅
- Sequencer agent ingress barrier (sequencer.rs:4037-4090) 强制 agent ≠ direct state writer ✅

**命题成立** ✅

### 2.3 Claim "两者通过同一条 tape + 同一套谓词通信"

**验证来源**: Track B (ChainTape) + Track E (谓词)
- ChainTape L4 + L4.E 同时承载 system_emitted 和 agent_submitted tx ✅
- HEAD_t witness 不区分两类 tx (state_root advance 一致) ✅
- Predicate registry (Art. I.1.1 PCP) 对两类 tx 均适用 ✅

**命题成立** ✅

### 2.4 Claim "ingress 路径不同"

**验证来源**: Track A
- agent_ingress barrier (sequencer.rs:4054) 强制分流 ✅
- AgentSignature vs SystemSignature 类型区分 (typed_tx.rs:82) ✅

**命题成立** ✅ (但 Agent_user_0 模式表明: "人通过 agent_submitted" 也是合法路径; 关键不是"人走哪条 ingress", 而是"人有 ingress 入口")

---

## 3. 阶段化演化路径

### 3.1 现阶段 (2026 - Phase 6/7)

```
人 → CLI / Web form
   ↓
   ├── 间接系统裁决: emit_system_tx (FinalizeReward / EventResolve / ...)
   ├── Spec 提交: submit_agent_tx via Agent_user_0 (TaskOpen / EscrowLock)
   └── 只读观察: read view / dashboard / replay

agent → SDK / scaffold
     ↓
     └── 工作: submit_agent_tx (Work / Verify / Challenge / CpmmSwap / ...)
```

**特点**: 人是"特权但受限"主体; 不能直接写 agent-only variants (Work / Verify 等); 但能间接触发 system 裁决 + 通过 Agent_user_0 启动任务.

### 3.2 中期 (2027-2028 - Phase 8+)

```
人 → 多接入入口 (CLI / Web / Voice / VR)
   ↓
   ├── (同前)
   └── 委托 agent 代理: agent (代理用户身份) submit_agent_tx

agent → 多模态 (text / voice / vision / 具身)
     ↓
     ├── (同前)
     └── 与其他 agent A2A 协商 (CAS A2AMessageCapsule + escrow)
```

**特点**: 人开始**委托** agent 代理 (类似经纪人); agent 间通信成熟 (A2A 协议化).

### 3.3 长期 (2029+ - Phase 9+, AGI 时代)

```
人 → 立法者 / 仲裁者 (constitution 修订, dispute resolution)
   ↓
   └── 不再直接发起 task; agent 自治市场

agent → 自治市场参与者
     ↓
     ├── 自主 propose task (AgentProposedTaskOpen — Class 4)
     ├── 自主 seed market (AgentMarketSeeding — Class 4)
     ├── 自治合约 (DirectSwapTx — Class 4)
     └── 跨 TuringOS 实例联邦
```

**特点**: 人从"特权角色"升级为"立法者/仲裁者"; agent 从"工作角色"升级为"市场参与者". 命题**演化**为: "人 = constitution-level 主体; agent = market-level 主体; 两者通过同一宪法 + 不同 ingress 治理".

---

## 4. 张力点分析

### 4.1 张力 1: 人 vs agent 的边界模糊

**张力**: Agent_user_0 是特殊 agent role (代表用户提名的任务发起者). 是"人"还是"agent"?

**Track A Q4 答案**: lean_market 现有模式是"人通过 Agent_user_0 间接走 agent_submitted"; 因此 Agent_user_0 在 sequencer 视角是 agent (AgentSignature), 在 user journey 视角是人代理.

**张力解决**: v2 命题应**显式**承认 Agent_user_0 类型存在 — 不是简单的二元 (人/agent), 而是**人代理 agent** (人控制的 agent identity).

### 4.2 张力 2: REAL-12 Bull/Bear 是 agent 自治, 但 spec 来自人

**张力**: REAL-12 Bull/Bear 角色 + EconomicJudgment 是 agent 自主决策, 但 Bull/Bear assignment 是 system_emitted (GenesisReport AgentRoleAssignment) 由人 init 阶段决定.

**Track D Q2 答案**: 角色分配在 genesis-phase 由人控制; 但 agent 在 task-phase 内自主判断 (Buy/Short/Abstain). 这是**人立法 + agent 执行**模式.

**张力解决**: 现阶段是 "**人立法 (角色 + 预算 + 规则) + agent 在框架内自治执行**". 这是 v2 命题的自然延伸 — 不是"人 = agent 特例", 而是"人立法, agent 在法律框架内自治".

### 4.3 张力 3: A2A 通信是 agent 间, 但 Librarian broadcast 涉及人的可见性

**张力**: REAL-BCAST-1 LibrarianBroadcast 是 agent 间通信 (轴 B), 但人可读取 Librarian digest (轴 A).

**Track B + F 答案**: A2A 是 agent 间数据流, 但 LibrarianDigest 是 derived view (CAS-anchored), 人可读. 这与 dashboard 是同一模式 — agent 自治产生数据, 人作 read-view 一等公民.

**张力解决**: 轴 A 和轴 B 不是分离平行的; **轴 A 的 read-view 部分天然涵盖轴 B 的输出**. 双轴是**生成-观察**关系, 不是**两个独立系统**.

---

## 5. v2 命题最终修订 (v2.1)

基于张力分析, 提议**精炼修订**:

### v2 → v2.1 命题

> **"人 = constitution-level + read-view 一等公民 (现阶段)";
> **"人代理 agent (Agent_user_0 类型) = 人控制的 agent identity, 走 agent_submitted ingress";
> **"agent = 工作角色 (自治执行人立法的规则), 走 agent_submitted ingress";
> **"两者通过同一条 tape + 同一套谓词通信; 人的特权在 constitution-level (spec 批准 + init 启动 + 仲裁) 而非 ingress-level"**

### 工程含义

- **Phase 6 MVP**: 人通过 CLI 间接发起 system_emitted (裁决) 或 通过 Agent_user_0 发起 agent_submitted (任务)
- **Phase 7 Web**: 同 Phase 6 + Web UI 主入口
- **Phase 8+**: 引入更多人代理 agent identity (Agent_user_N); 允许人 spawn 自己的 worker agent
- **Phase 9+ (AGI)**: 人逐步退出 task-level operation, 升级为 constitution-level 修订者; agent 自治市场成熟

---

## 6. 双轴交汇命题工程落地

### 6.1 "同一条 tape" 的实现

| 数据 | tape 位置 | 主体 |
|---|---|---|
| 人触发的 finalize / resolve | ChainTape L4 (system_emitted) | 人 (via CLI) |
| 人提交的 spec / task | ChainTape L4 (agent_submitted via Agent_user_0) | 人代理 agent |
| Agent 的 Work / Verify / Challenge | ChainTape L4 (agent_submitted) | agent |
| Agent 的 EconomicJudgment | CAS (REAL-12 schema) | agent |
| Agent 间 LibrarianBroadcast | CAS (REAL-BCAST-1 schema) | agent (via Librarian role) |
| 人观察的 dashboard / report | CAS-derived read view | 人 (read-only) |

**结论**: 所有人 + agent 的活动都在 ChainTape + CAS, 无独立 ingress. ✅

### 6.2 "同一套谓词" 的实现

| 谓词 | 适用对象 |
|---|---|
| PCP soundness (Art. I.1.1) | 所有 Lean proof verification (无论 agent 还是人代理) |
| UIContentPredicate (Track E Q1) | 所有 UI input (人 + agent 都需要通过) |
| A2AMessageSignatureVerifier (Track E §9) | 所有 A2A message (agent ↔ agent) |
| Sequencer admission rules | 所有 typed_tx (system_emitted + agent_submitted) |

**结论**: 谓词体系对人和 agent 一视同仁; 无 "人豁免谓词" 设计. ✅

### 6.3 "ingress 路径不同" 的精炼

实际有**3 条 ingress 路径** (不是 2 条):
1. **system_emitted** (人作系统裁决者, 通过 emit_system_tx)
2. **agent_submitted via Agent_user_0** (人代理 agent 走 agent_submitted)
3. **agent_submitted via 其他 agent** (REAL-5 角色 agent 自治)

但**底层只有 2 种 signature 类型** (SystemSignature / AgentSignature), 因此**密码学层级仍是二元**.

**精炼**: "ingress 路径不同" 应改为 "**signature 类型二元 (system / agent); 但 user 实际有多种 ingress 模式 (system 裁决 / Agent_user_0 代理 / 委托其他 agent)**".

---

## 7. 与 AGI 时代演化对比 (其他研究方向的命题)

### 7.1 vs Karpathy "Software 3.0" (Track H)

- **Karpathy**: 人写 prompt, AI 写代码, 系统执行
- **TuringOS v2.1**: 人写 spec + 修订 constitution, agent 在 constitution 内自治, 系统强制宪法

**差异**: Karpathy framework 缺少 "宪法强制" 一层; TuringOS 在其上加宪法 runtime layer.

### 7.2 vs Anthropic "Constitutional AI" (Track I)

- **Anthropic CAI**: 训练时灌注 constitution (training-time alignment)
- **TuringOS**: 运行时强制 constitution (runtime alignment)

**Track I 发现**: Anthropic 2025-2026 Constitutional Classifiers 转向 runtime + classifier; 与 TuringOS 哲学**收敛**. ✅

### 7.3 vs Bittensor / Fetch.ai 经济模型 (Track I)

- **Bittensor**: agent 间挖矿竞争, 主网 token 经济
- **TuringOS**: agent 在 constitution-gated arena 内经济竞争, **不发行 token**, 内生 Coin (1 Coin = 1 YES + 1 NO)

**差异**: TuringOS 经济**无 inflation** (Bittensor subsidy 模型违反守恒); 是 **closed-economy arena**. ✅

### 7.4 vs JEPA / V-JEPA 世界模型 (Track J)

- **JEPA**: agent 内部隐式 latent world model
- **TuringOS**: ChainTape + CAS 是**外部显式 world model**; agent 不需要内部 V-JEPA

**Track J Q1 答案**: TuringOS tape 已是 explicit world state; agent 不需要 V-JEPA 级 latent model. 缺 ForecastCapsule (近期 ChainTape 语义摘要 + 未来预测) 作 agent rollout 隐式基底.

---

## 8. 命题的工程边界

### 8.1 命题适用范围

✅ **适用**:
- TuringOS 内部 (单实例); 人 + agent 共享 ChainTape + CAS
- Phase 6-9: 现阶段到中期 AGI 时代
- Lean proof market / Polymarket / 通用 task market

❌ **不适用**:
- 跨 TuringOS 实例联邦 (Phase 9+, 需扩展 DID 桥)
- TuringOS 与外部系统集成 (e.g., 真实金融市场, 需扩展 oracle)
- 完全自治 agent (无人在环, Phase 10+ AGI 自治)

### 8.2 命题不解决的问题

- 跨实例 agent 身份可携带性 (Track I Q5; 留 DID 桥)
- agent 间真实合约自动执行 (Phase 8+ DirectSwapTx, Class 4)
- 人的 AI 委托代理 (Phase 8+ Agent_user_N 扩展)
- 大规模 agent 集群 (N≥1000) 涌现 (Phase 10+)

---

## 9. Synthesis 结论

### 9.1 v2 命题验证状态
- ✅ "人是特权角色 (走 system_emitted ingress)" — **修订**为 "人有多种 ingress 模式 (system / Agent_user_0 / 委托)"
- ✅ "agent 是工作角色 (走 agent_submitted ingress)" — **成立**
- ✅ "同一条 tape + 同一套谓词" — **成立** ✅
- ✅ "ingress 路径不同" — **精炼**为 "signature 类型二元; 但 user 实际有多种 ingress 模式"

### 9.2 v2.1 命题最终版

> **"人 = constitution-level 立法者 + read-view 一等公民 (现阶段);
> 人代理 agent (Agent_user_0 类型) = 人控制的 agent identity;
> agent = 工作角色 (constitution-internal 自治);
> 三者通过同一条 tape + 同一套谓词通信;
> Signature 二元, ingress 三模式;
> 人的特权在 constitution-level 而非 ingress-level"**

### 9.3 工程价值

- v2.1 命题与 TuringOS 现架构**完全兼容** (0 个 Class 4 修改)
- 为 Phase 6 → Phase 9 提供清晰演化路径
- 与外部 AGI 研究 (Karpathy / Anthropic / JEPA) 对照清晰
- 留 AGI 时代扩展面 (Phase 10+ 完全自治)

### 9.4 Phase 5 输出指导

本文档作为 `01_MASTER_PLAN.md` 双轴愿景章节的核心输入. Phase 5 应:
- 在所有 deliverable 头部使用 v2.1 命题
- Phase 6-9 路线图基于 v2.1 阶段化演化
- 与 Generative HTML 报告的对比章节使用 v2.1 命题

**Phased Dual-Axis Synthesis 完成**: v2 命题验证 + 修订为 v2.1; 3 张力点解决; 与 AGI 研究对照; 工程边界清晰.
