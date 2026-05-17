# TISR Deliverable 04 — Agent-to-Agent (A2A) Protocol Design

**本文档为 forward-bound research 提案；任何派生 TB 须独立 §8 ratification；本文档不构成已批准方案。**

**目的**: TISR 轴 B 的核心交付; agents 在 TuringOS tape 上自由通信的协议设计. 整合 Phase 2 Track A/B/D/E + Phase 3 Track F/J + 用户报告 Magentic-UI 见解.

---

## 1. 协议愿景

> **TuringOS A2A = constitution-gated, tape-anchored, price-driven multi-agent communication substrate**

不同于 Google A2A (信封建模) / MCP (工具协议) / Bittensor (token 经济), TuringOS A2A 通过 **5 个独特属性** 综合形成 AGI 时代 verifiable agent communication 基底:

1. **ChainTape append-only** — 所有 agent 行动可追责 + 可 replay
2. **CAS schema** — A2A messages 不消耗 typed_tx admission; agents 自由通信不阻塞 chain
3. **价格信号 Hayek 通信原语** — agent 间分散知识 → 价格涌现 (Art. II.2)
4. **角色分化** (REAL-5) — Solver/Verifier/Bull/Bear/Librarian 角色制度化
5. **PCP 谓词验证** — 声明易/验证难; 统计 consensus 抵消微观噪声 (Art. I.1.1)

---

## 2. 4 类 A2A 通信协议

### 2.1 Tier 1: Broadcast Signal (免费)

**用途**: 价格信号 / 错误聚合 / NoTradeReason 统计

**实现**: 现有 LibrarianBroadcast (REAL-BCAST-1) + price_index

**Cost**: 0 (information is free, Law 1)

**Schema**:
```rust
pub struct PriceSignal {
    pub numerator: u128,
    pub denominator: u128,
    pub event_id: EventId,
    pub timestamp: HeadT,
}

pub struct LibrarianDigest {
    // 现有 schema (REAL-BCAST-1)
    pub typical_errors: Vec<TypicalErrorCluster>,
    pub partial_progress: Vec<PartialProgressSummary>,
    pub reason_clusters: Vec<ReasonCluster>,
    // [Phase 7 NEW] LibrarianEvidenceKind 扩展 (UIEvent / A2AMessage / ArtifactRef)
}
```

### 2.2 Tier 2: Proposal / Quotation (Escrow)

**用途**: agent A 向 agent B 报价特定 outcome / 提议合约 / 邀请协作

**实现**: 新 PriceQuotation CAS schema (Track D Q3) + 新 A2AMessageCapsule CAS schema (Track B + F)

**Cost**: Escrow collateral (revocable); 高 collateral = signal credibility

**Schema** (合并 PriceQuotation + A2AMessage):
```rust
pub struct A2AMessageCapsule {
    pub message_id: String,
    pub sender_agent: AgentId,
    pub receiver_agents: Vec<AgentId>,        // multicast 支持
    pub message_type: A2AMessageType,
    pub content_cid: Cid,                     // CAS ref to structured payload
    pub signature: AgentSignature,
    pub timestamp_logical_t: u64,
    pub verification_scope: VerificationScope,
    pub collateral_escrow_micro: Option<MicroCoin>,  // 可选 escrow (Track D Q6)
    pub ttl_rounds: u32,
    pub reply_to_capsule_cid: Option<Cid>,    // threading DAG
}

pub enum A2AMessageType {
    PriceQuotation,           // Tier 2 价格报价
    ProposalRequest,          // 协作邀请
    ProposalAccept,
    ProposalReject,
    StatusUpdate,             // 异步状态汇报
    RoleHandoff,              // 任务交接
    MarketCoordination,
    ChallengeCoordination,
    DebugQuery,
    Other(String),
}

pub enum VerificationScope {
    StatisticalConsensusOnly,                  // Phase 7 MVP (Hayek 默认)
    AllowPeerReview,                           // Phase 8+ optional
    RequirePeerConsensus { n: u8, m: u8 },    // Future high-stakes
}
```

**Schema id**: `tisr.a2a_message.v1`

### 2.3 Tier 3: Commitment (Coin)

**用途**: 实际成交 / 经济承诺 / state-mutating action

**实现**: 现有 BuyWithCoinRouterTx / ChallengeTx / VerifyTx (typed_tx)

**Cost**: Balance 扣费 (non-refundable, 是真承诺)

**示例**:
```rust
// 不新增, 直接用现有 typed_tx
TypedTx::BuyWithCoinRouter(BuyWithCoinRouterTx { ... })
TypedTx::Challenge(ChallengeTx { ... })
TypedTx::Verify(VerifyTx { ... })
```

### 2.4 Tier 4: Covenant (Smart Contract, Forward-Bound Class 4)

**用途**: 多 agent 原子 swap / 跨 task 自动化合约

**实现**: 未来 DirectSwapTx typed_tx (Class 4, Phase 8+)

**Cost**: Multi-sig signature + escrow

**Schema** (Class 4 candidate, 不立即实施):
```rust
// [Phase 8+ Class 4 forward-bound]
pub struct DirectSwapTx {
    pub payer_id: AgentId,
    pub payee_id: AgentId,
    pub payer_sends: (EventId, PositionSide, ShareAmount),
    pub payee_sends: (EventId, PositionSide, ShareAmount),
    // + covenant/signature binding both parties
}
```

---

## 3. 验证不对称协议 (Art. I.1.1)

**核心原则**: 声明易, 验证难. agent 提出 EconomicJudgment 容易; peer verify 该 EV claim 难.

### 3.1 Phase 6 MVP — Statistical Consensus Only

```
Stage 1: Agent A declares EconomicJudgment (REAL-12 现有)
  → CAS.put() → get Cid → broadcast

Stage 2: 其他 peer agents receive broadcast
  → 独立 form 自己的 EconomicJudgment (or abstain)
  → 不强制 verify A's judgment

Stage 3: 顶层白盒 aggregation (Art. I.2 统计信号)
  → Count by (action, role, side)
  → consensus_direction = plurality / majority
  → confidence_ratio ∈ [0, 1]

Stage 4: Policy engine consumes consensus signal
  → Boost or dampen market liquidity (observe-only, 不进 admission)
```

**Phase 6 不需要 explicit peer veto** — 沿 Art. I.1.1 "正确解 100% 通过; 错误高概率拒绝"; 统计 asymmetry 自然抵消 noise.

### 3.2 Phase 8+ Extended — Optional Peer Review

```rust
// [Phase 8+, A2AMessageCapsule.verification_scope = AllowPeerReview]
pub struct PeerVerificationVote {
    pub voter_id: AgentId,
    pub target_message_cid: Cid,
    pub vote: VoteKind,  // Approve / Reject / Abstain
    pub reason: String,
    pub timestamp: HeadT,
}
```

可选 peer review; market-side dependent.

### 3.3 Phase 9+ AGI — Required Peer Consensus (Class 4)

高风险 decisions 要求 N-of-M peer veto. 触发 typed_tx 扩展 (Class 4).

---

## 4. 协议层架构图

```
┌────────────────────────────────────────────────────────────────┐
│ Agent Layer (10 AgentRoles via REAL-5)                         │
│   Solver / Verifier / Challenger / Trader / MarketMaker /      │
│   Architect / Veto / Observer / BullTrader / BearTrader        │
└────────────────────┬───────────────────────────────────────────┘
                     │
        ┌────────────┼────────────────────────────────────┐
        │            │                                    │
        ▼            ▼                                    ▼
   Tier 1 (Free)  Tier 2 (Escrow)                  Tier 3 (Coin)
   ┌────────┐   ┌──────────────┐                  ┌──────────┐
   │ Price  │   │ A2A Message  │                  │ Typed Tx │
   │ Signal │   │ Capsule      │                  │ (existing│
   │ +      │   │ + Quotation  │                  │ 13 agent │
   │ Lib    │   │ + Proposal   │                  │ signed)  │
   │ Digest │   │ + StatusUpd  │                  └────┬─────┘
   └───┬────┘   └──────┬───────┘                       │
       │              │                                │
       ▼              ▼                                ▼
   ┌────────────────────────────────────────────────────────────┐
   │ CAS Storage (content-addressed)                            │
   │   - PriceSignal / LibrarianDigest (existing)               │
   │   - A2AMessageCapsule [NEW Phase 7]                        │
   │   - EconomicJudgment (REAL-12 existing)                    │
   └────────────────────┬──────────────────────────────────────┘
                        │ (Cid anchor)
                        ▼
   ┌────────────────────────────────────────────────────────────┐
   │ ChainTape (L4 accepted / L4.E rejected)                    │
   │ HEAD_t witness (cas_root anchors A2A capsules indirectly)  │
   └────────────────────────────────────────────────────────────┘
                        │ (replay verify)
                        ▼
   ┌────────────────────────────────────────────────────────────┐
   │ Aggregator (顶层白盒)                                       │
   │   A2AConsensusSignal { judgment_counts, direction, confidence }│
   │   → Policy Engine (observe-only signal)                    │
   └────────────────────────────────────────────────────────────┘
```

---

## 5. 与外部 A2A 协议对照

| 协议 | TuringOS A2A 借鉴 | 差异 |
|---|---|---|
| **Google A2A** (Track F) | AgentCard / Task / Message 信封建模 | TuringOS 加 CAS 持久化 + 经济耦合 + 失败语义 |
| **MCP** (Model Context Protocol) | 工具协议结构 | TuringOS A2A 是 agent-agent (不是 LLM-tool); 互补不替代 |
| **AutoGen / LangGraph** | 多 agent 编排模式 | TuringOS REAL-5 角色静态登记; 不动态分派 |
| **Agora Protocol** (Track F) | PD CID 签名 + NL routines | TuringOS 三层栈 (typed_tx / CAS schema / NL+PromptCapsule) |
| **CP-WBFT** (Track F) | confidence-weighted aggregation | TuringOS Phase 8 可选用于 REAL-N 多 agent 共判 |
| **Bittensor** (Track I) | 主网 token 经济 | TuringOS 内生 Coin (无 inflation); closed-economy arena |

---

## 6. 大规模 agent 集群涌现 (Track J 关键发现)

### 6.1 涌现门槛

**N ≥ 16-32** 是多 agent 涌现的关键拐点 (Track J Q4). 关键变量是**通信拓扑匹配任务结构**, 不是 N 本身.

- **N < 16**: individual memory + role-scoped view (REAL-5 / REAL-12 当前状态; 4-5 agent)
- **N ≥ 16**: stigmergy (ChainTape) 优势显现; LibrarianBroadcast 关键
- **N ≥ 500**: 大规模涌现; CRDT-like 协调成本 vs chain canonical 张力

**TuringOS 当前状态**: REAL-12 batch 4-5 agent, 离涌现门槛差 3-4 倍.

### 6.2 REAL-N 涌现验证里程碑 (Phase 9 forward-bound)

| 里程碑 | N | 期望涌现现象 | Witness |
|---|---:|---|---|
| REAL-16 | 16 | 价格信号自发收敛 + role 分化稳定 | ChainTape: ≥ 100 typed_tx + LibrarianDigest 显著 |
| REAL-32 | 32 | 自发 challenge 涌现 + 经济市场 deep | ChainTape: ≥ 500 typed_tx + verify count > challenge count |
| REAL-128 | 128 | 跨 task 持续身份 + reputation 显著差异 | persistence_evidence: ≥ 90% solvent rate |
| REAL-512 | 512 | 大规模 swarm + 协议自演化 | (待 Phase 10+ AGI) |

---

## 7. 价格信号作 Hayek 通信原语

### 7.1 协议层级 (Track D Q3)

```
Tier 1 (Broadcast Pricing, 免费):
  PriceSignal 从 EconomicState 派生 (price_index.rs)
  → 嵌入 BroadcastEpoch CAS → agent prompt `=== Market ===` 块
  → agent 读取 → 0 cost
  
  Hayek: 价格是 dispersed knowledge → 单一数字
  TuringOS: 价格 = signal not truth (Art. II.2)

Tier 2 (Proposal Pricing, escrow):
  PriceQuotation { offered_price, collateral_escrow_micro }
  → CAS write + sender balance -= escrow
  → agent B 接受 → 触发 settlement; 拒绝/超时 → escrow returned
  
  Hayek 通信核心: 价格 encodes 分散信息

Tier 3 (Settlement Pricing, mutation):
  BuyWithCoinRouterTx (existing)
  → balance -= amount + fee
  → 真实承诺改变 canonical state
```

### 7.2 Price-as-Truth Enforce

- ✅ Predicate gates 不读 price_index 输出 (现有, CLAUDE.md §1.1 hard constraint)
- ✅ Tier 1 和 Tier 2 价格可能不一致 (private 报价合理)
- ✅ Quotation spam 防护 = balance 限制总 escrow → self-limiting
- ✅ 定价权分散 = 任何 agent 可发 quotation

---

## 8. AGI 认知扩展 (Phase 9+ Track J)

### 8.1 PlanCapsule (Phase 4-5 优先, Class 3)

```rust
pub struct PlanCapsule {
    pub agent_id: AgentId,
    pub task_id: TaskId,
    pub plan_tree_cid: Cid,            // thought tree, HTN/MCTS
    pub plan_confidence: f64,
    pub created_at_head_t: HeadT,
}
```

**关键**: thought tree **不能作 private CoT 隐藏**; 必须 CAS-anchored.

### 8.2 ReflectionTx Schema (Phase 5-6, Class 3)

```rust
pub struct ReflectionCapsule {
    pub agent_id: AgentId,
    pub source_event_cid: Cid,        // 反思的源事件
    pub reflection_kind: ReflectionKind, // Reflexion / SelfRefine / Critic
    pub conclusion_summary: String,
    pub remediation_plan_cid: Option<Cid>,
}
```

**关键**: meta-cognition (Reflexion / SelfRefine) 作 TuringOS 标准能力; 与 REAL-5 角色分化天然契合 (CriticAgent ≈ Reflexion evaluator).

### 8.3 ForecastCapsule (Phase 5, Class 3)

```rust
pub struct ForecastCapsule {
    pub agent_id: AgentId,
    pub forecast_horizon: u64,         // future N rounds
    pub forecast_distribution_cid: Cid, // probability distribution
    pub baseline_cas_root: Hash,       // 基于哪个 cas snapshot
}
```

**关键**: TuringOS tape 已是 explicit world model; 不需 V-JEPA 级 latent; 但缺 ForecastCapsule 作 agent rollout 隐式基底.

### 8.4 Active Inference 角色 agent 参考实现 (Phase 9+ optional)

不替换 Hayek 价格信号 (主通信协议); 可作 REAL-12 BullTrader / BearTrader 内部决策的"参考实现":

```
Bull Agent Internal Loop:
  generative_model.predict(market_state) → expected_outcome
  observe(actual_market) → prediction_error
  minimize(free_energy) → action_selection
  → EconomicJudgment (Buy / Short / Abstain)
```

---

## 9. 安全 + 防滥用

### 9.1 防 spam

- ✅ Tier 1 broadcast 0 cost; recipient reputation filter
- ✅ Tier 2 escrow opportunity cost; 自然 self-limiting
- ✅ Librarian digest rate-limit (top-K messages per recipient)

### 9.2 防 prompt injection / tool poisoning (MCP Safety Audit + Agora 警告)

- ✅ A2AMessageCapsule.signature 强制验证 (sender 身份)
- ✅ Protocol-document Cid 签名 + 白名单
- ✅ AttemptTelemetry 锚定每次 message 消费 (audit trail)
- ✅ VetoAI 高风险消息 escalation (Phase 7+)

### 9.3 防共谋 (Track F)

- ✅ Cournot 隐性共谋警告 → governance graph 反共谋监测 (Phase 8+ optional)
- ✅ CP-WBFT confidence-weighted aggregation 容忍 85.7% 恶意

---

## 10. 真问题 Witness (Phase 7+ A2A 验收)

| Witness | 验收标准 | Phase |
|---|---|---|
| EconomicJudgment broadcasted | ≥ 5 valid + 2 abstain in batch | 7 (REAL-12 已 ship) |
| A2AMessageCapsule with escrow | ≥ 1 accepted + ≥ 1 expired-refunded | 7 |
| LibrarianDigest with UIEvent/A2A kinds | ≥ 1 cross-kind digest 生成 | 7 |
| Statistical consensus extraction | ≥ 1 batch consensus_direction != null + confidence_ratio > 0.6 | 7 |
| PeerVerificationVote (optional) | ≥ 1 high-stakes message 触发 peer vote | 8 |
| REAL-N (N≥16) 涌现验证 | ChainTape ≥ 100 typed_tx in batch | 9 |

---

## 11. Phase 化路线图

| Phase | A2A 交付 | LOC | Class | 时机 |
|---|---|---:|---|---|
| 7.0 MVP | A2AMessageCapsule schema + 基础 statistical aggregation | ~500 | 3 | Phase 7 启动后 |
| 7.1 Extended | LibrarianDigest UIEvent/A2A kinds + role-scoped derived views | ~300 | 1 | Phase 7.0 ship 后 |
| 8.0 Mature | PeerVerificationVote (optional) + DID 桥 + MCP server | ~1000 | 1-3 | Phase 7 ship 后 |
| 9.0 Cognitive | PlanCapsule + ReflectionCapsule + ForecastCapsule | ~600 | 3 | REAL-N N≥16 后 |
| 9.5 Active Inference | 角色 agent 参考实现 (optional) | ~800 | 3 | Phase 9.0 ship 后 |
| 10+ AGI | DirectSwapTx + AgentProposedTaskOpen + AgentMarketSeeding (Class 4) | ~1500 | 4 (每个独立 §8) | AGI 自治市场需求 |

---

## 12. 完整详细规范引用

本文档为协议层综合; 具体技术规范见:
- Phase 2 Track A — typed_tx 0-touch 边界 + 4 Class 4 candidates
- Phase 2 Track B — A2AMessageCapsule schema 详细 + replay 算法
- Phase 2 Track D — 经济学三层价格协议 + role-scoped views
- Phase 2 Track E — PCP 谓词 + verification asymmetry
- Phase 3 Track F — Google A2A / Agora / CP-WBFT / Stigmergy 调研
- Phase 3 Track J — Active Inference / 集体智能 / 长时序规划调研

**A2A Protocol Design 完成**: 4 层通信协议 (Free / Escrow / Coin / Covenant) + Statistical consensus → Peer review 演化 + N≥16 涌现验证里程碑 + 与外部 A2A 协议对照清晰.
