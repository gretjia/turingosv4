# Phase 2 / Track D — 经济学集成

**本文档为 forward-bound research 提案；任何派生 TB 须独立 §8 ratification；本文档不构成已批准方案。**

**作者**: TISR Phase 2 Explore subagent (2026-05-17)
**Scope**: TISR 双轴与 TuringOS 既有经济学体系 (REAL-5 角色 + REAL-12 EconomicJudgment + 价格信号 + CompleteSet 守恒) 集成

---

## 0. Scope 边界

**严格避免**修改 Class 4 受保护 surface:
- `src/state/typed_tx.rs` TypedTx 19 variants
- `src/state/sequencer.rs` admission rules
- `src/runtime/real5_roles.rs:29` AgentRole enum
- `src/economy/` 经济状态 mutator / CompleteSet 守恒
- CLAUDE.md §13 "1 Coin = 1 YES + 1 NO" 整数算术不变量

**Protocol-Layer Design Scope**:
1. 信息流协议 (通过现有 CAS/ChainTape/价格信号传递经济信息 — Art. II.2)
2. Role-scoped View 扩展 (新 UI-relevant derived view, 不改 AgentRole enum)
3. 声誉 + 破产影响 UI (AgentReputation schema 只读)
4. A2A 通信成本 (mint cost vs Hayek 免费信号的协议分界)
5. UI artifact 定价 (market = role-specific institution, 不是 prompt decoration)

**Class 4 Forward-Bound Candidates** (若落地需显式 ratification):
- 新增 AgentRole variant (Designer/Curator/Editor/Auditor)
- A2AMessageCapsule mint cost enforcement
- 声誉值对 sequencer 优先级的直接影响

---

## 1. 现状盘点

### 1.1 REAL-5: 10 个角色 (`real5_roles.rs:29`)

```rust
pub enum AgentRole {
    Solver, Verifier, Challenger, Trader, MarketMaker,
    Architect, Veto, Observer, BullTrader, BearTrader,
}
```

每角色有 allowed_tools 白名单 + view_policy_id + risk_budget_micro. BullTrader/BearTrader 分化市场偏见 (MarketSide::Yes/No).

### 1.2 REAL-12 EconomicJudgment CAS Schema (`economic_judgment.rs`)

```
EconomicJudgment {
  action: Buy / Short / Abstain,
  reason: EconomicReason (13 structured variants),
  observed_price: Option<RationalPrice>,
  estimated_probability_band: Option<ProbabilityBand>,
  expected_value_sign: ExpectedValueSign,
  ...
}
```

每 Bull/Bear RoleTurnTrace 必须链接 EconomicJudgment CID. 用于 dashboard/audit (不用于 predicate gates, 保 price ≠ truth).

### 1.3 价格信号基底 (`price_index.rs` 1142 行)

`RationalPrice { numerator: u128, denominator: u128 }` 整数有理数 ∈ [0, 1]. Hayek 通信: 市场共识对 outcome 的概率估计, **NOT ground truth** (Art. II.2). LibrarianBroadcast 按 role 裁剪后嵌入 prompt `=== Market ===` 块.

### 1.4 MarketDecisionTrace + NoTradeReason

NoTradeReason 13 variants (NoPromptTool / NoParsedInvest / NoPool / RouterRejected / AgentDeclined / NoPerceivedEdge / etc.). 区分 agent 拒绝 vs 系统拒绝.

### 1.5 AgentMarketStateView (PnL 可见性)

7 字段: balance / open_positions / realized_pnl / unrealized_pnl / solvency_status (Solvent/NearInsolvent/Bankrupt) / reputation_score / capital_exposure. 破产阈值 = 10% of initial_balance (SG-G3.3).

### 1.6 CompleteSet 守恒 + 整数算术

- 1 μCoin → 1 YES + 1 NO (mint)
- 1 YES + 1 NO → 1 μCoin (redeem)
- cost_basis per share = 0.5 μC
- 对称持仓 (N YES + N NO) → unrealized_pnl = 0
- 非对称持仓 → signed PnL

---

## 2. Q1: 信息免费 vs 投资花钱在 TISR 双轴

### 2.1 轴 A (HCI 用户视角)

**信息免费**: dashboard / view tape / query CAS / export report — **0 cost**

**投资花钱**:
- TaskOpenTx → escrow_lock (stake in balance)
- ApprovalTx → 消耗 balance
- init module → 可能消耗计算成本 (future)

**实现路径**:
```
User Query: GET /dashboard/agent/{id}/balance → read(balances_t.get(id)) → 0 cost
User Action: POST /task/open {spec, escrow} → sequencer.admit(TaskOpenTx) → escrow_lock if admitted; 0 cost if rejected
```

### 2.2 轴 B (A2A 自治通信)

**信息免费 (Hayek 信号)**:
- 价格广播 (BroadcastEpoch + PriceSignal) — 0 cost (Art. II.2)
- LibrarianDigest typical error — 0 cost (REAL-BCAST-1)
- NoTradeReason aggregate — 0 cost

**承诺花钱**:
- BuyWithCoinRouterTx → balance 扣费
- ChallengeTx → stake escrow
- VerifyTx → 可能 lock stake pending settlement

**协议层分界**:
```
Tier 1: Broadcast (免费)
  PriceSignal { numerator, denominator, event_id, timestamp }
  → CAS BroadcastEpoch → agent 读取 → 0 cost

Tier 2: Commitment (有成本)
  BuyWithCoinRouterTx → amount 扣费 → cost
  ChallengeTx → stake escrow → cost

Tier 3: 声誉信号 (衍生)
  reputations_t[agent_id] → 从 on-chain 胜率派生 → 0 额外成本
  → 影响调度优先级 (NOT admission)
```

### 2.3 双轴统一框架

| 维度 | 轴 A (HCI) | 轴 B (A2A) | 统一点 |
|---|---|---|---|
| 信息频道 | Dashboard UI / CLI 查询 | BroadcastEpoch + LibrarianDigest | 都用 CAS read-only |
| 成本模型 | read=0, task_open=stake, approve=payment | broadcast=0, invest=coin, challenge=stake | Law 1 + 2 体现 |
| 权力来源 | 人特权: 批准 + 启动 | 角色分化 (REAL-5) + 风险预算 | 同一 tape, 两个 ingress |
| 经济信号 | AgentMarketStateView | PriceSignal + NoTradeReason aggregate | 都派生 canonical EconomicState |

---

## 3. Q2: Role-Scoped DerivedView 扩展

### 3.1 新角色需求分析 (轴 A HCI)

| 新角色 (建议) | 需要视图 | 经济学含义 | 前置条件 |
|---|---|---|---|
| **Designer** | UI component 库 + artifact 生成日志 + 用户反馈 | UI artifact 是否有 market 价值? | 定义 "UI artifact = tradeable item" 协议 |
| **Curator** | canonical report + endorsement log + crowd signal | 哪些 report/spec 高质量? reputation 如何追踪? | 需 curator_endorsement_tx + reputation path |
| **Editor** | diff history + revision approval + 原作者保留权 | 编辑劳动价值如何分配? | revenue-sharing in typed_tx (**Class 4**) |
| **Auditor** | constitution gates + predicate audit log + finding CAS | 不同审计权重? | auditor_finding_tx + reputation amplification |

### 3.2 **forward-bound 警告**: 新 AgentRole variant 是 Class 4

现阶段策略: **不扩展 AgentRole**, 而是用 **role-scoped derived view + policy-scoped access**.

**Designer View** (Class 0-1):
```rust
pub const UI_ARTIFACT_CAPSULE_SCHEMA_ID: &str = "turingosv4.ui_artifact.v1";

pub struct UIArtifactCapsule {
    pub generator_agent_id: AgentId,
    pub artifact_type: String,  // "dashboard_layout" | "report_template" | "form"
    pub artifact_cid: Cid,
    pub user_feedback_sentiment: Option<u32>,
    pub reuse_count: u64,
    pub created_at_head_t: String,
}

pub struct DesignerView {
    pub agent_id: AgentId,
    pub generated_artifacts: Vec<UIArtifactCapsule>,
    pub feedback_aggregate: HashMap<String, u32>,
    pub reputation_multiplier: f64,
}

pub fn compute_designer_view(cas: &CasStore, agent_id: &AgentId) -> DesignerView { ... }
```

**Curator View**:
```rust
pub struct CuratorEndorsement {
    pub curator_id: AgentId,
    pub endorsed_spec_cid: Cid,
    pub endorsement_strength: u32,  // 1-100
    pub timestamp_head_t: String,
}

pub struct CuratorView {
    pub agent_id: AgentId,
    pub endorsements_issued: Vec<CuratorEndorsement>,
    pub aggregate_strength: u32,
    pub crowd_signal_correlation: Option<f64>,
}
```

**Auditor View**:
```rust
pub struct AuditFinding {
    pub auditor_id: AgentId,
    pub finding_type: String,  // "constitution_violation" | "economic_anomaly" | ...
    pub severity: u32,
    pub evidence_cid: Cid,
}

pub struct AuditorView {
    pub agent_id: AgentId,
    pub findings_issued: Vec<AuditFinding>,
    pub severity_aggregate: u32,
    pub fix_rate: Option<f64>,
    pub trust_signal: f64,
}
```

### 3.3 Schema 层扩展 (Class 1 提案)

**新增 CAS Object Types** (不改 TypedTx / sequencer):

| Schema | Object Type | 来源 |
|---|---|---|
| ui_artifact.v1 | Generic | Agent emit or User feedback |
| curator_endorsement.v1 | Generic | Curator action → policy script |
| audit_finding.v1 | Generic | Audit script → CAS write |
| reputation_evidence.v1 | Generic | Derived from above + L4 tx counts |

### 3.4 Kill Condition: Class 4 Boundary

如果后续实施需要:
- ❌ 新 AgentRole enum variant → Class 4 候选
- ❌ 新 TypedTx discriminant (curator/auditor action) → Class 4 forward-bound
- ❌ 修改 sequencer admission 以 auditor finding 影响优先级 → Class 4

---

## 4. Q3: 价格作 Hayek 通信原语协议

### 4.1 三层协议栈

```
Tier 1: Broadcast Pricing (广播, 免费)
  PriceSignal: 市场汇率 (CPMM 恒定乘积)
  Source: price_index.rs (canonical EconomicState 派生)
  Transport: BroadcastEpoch CAS → Agent prompt

Tier 2: Proposal Pricing (报价, escrow)
  PriceQuotation: agent A 向 agent B 报价
  Schema: new CAS object type (NOT typed_tx)
  Stake: quotation 发行者 escrow 少量 balance
  Validity: timestamp-bounded (~10 ticks)

Tier 3: Settlement Pricing (成交, mutation)
  BuyWithCoinRouterTx (现有)
  [Future] DirectSwapTx (Class 4 forward-bound)
```

### 4.2 Tier 2: PriceQuotation CAS Schema (Class 1)

```rust
pub const PRICE_QUOTATION_SCHEMA_ID: &str = "turingosv4.price_quotation.v1";

pub struct PriceQuotation {
    pub quoter_id: AgentId,
    pub target_agent_id: Option<AgentId>,  // None = public; Some = private quote
    pub event_id: EventId,
    pub outcome_side: PositionSide,
    pub offered_price: RationalPrice,
    pub min_quantity: ShareAmount,
    pub max_quantity: ShareAmount,
    pub valid_until_head_t: HeadT,
    pub collateral_escrow_micro: MicroCoin,  // 锁定 guarantee
    pub quotation_id: Cid,
}

pub enum QuotationAcceptance {
    Accepted { acceptor_id: AgentId, quantity: ShareAmount, completion_tx_id: TxId },
    Rejected { reason: String, timestamp_head_t: HeadT },
    Expired,
}
```

### 4.3 Hayek 含义

- **Tier 1 (Broadcast)**: 完全免费信号; 价格是统计结果, 不是权威 (Art. II.2)
- **Tier 2 (Quotation)**: Agent 愿意为报价"背书"付费 (escrow); 高报价量 → 市场认可 → 其他 agent 自发响应; **Hayek 通信核心**: 价格 encodes 分散信息为单个数字
- **Tier 3 (Settlement)**: 实际成交; 只有真实承诺才能改变 canonical state

**为何 escrow 而非 direct fee**:
- 直接扣费 → "low-cost quotation spam" 风险 (虽有 escrow collateral 制约)
- Escrow = opportunity cost (锁定可用 coin) = 真实成本 (而非"名义"成本)

### 4.4 Price-as-Truth Enforce 机制

1. **Predicate gates 不读 price_index 输出** (CLAUDE.md §1.1 hard constraint) — 已存在 ✅
2. **Price disagreement 不违反一致性** — Tier 1 和 Tier 2 价格可能不一致 (private 报价); 市场通过分散涌现价格
3. **Quotation spam 防护** — escrow 锁定 balance → 自然 self-limiting
4. **定价权分散** — 任何 agent 都可发 quotation; MarketMaker 仅 fee 折扣优势 (future)

---

## 5. Q4: 破产 / 声誉 影响 UI 入口

### 5.1 现状

破产阈值 (SG-G3.3): Solvent ≥ 10% / NearInsolvent 0<x<10% / Bankrupt ≤ 0.
现有 dashboard 已显示 solvency_status; sequencer 仍允许 Bankrupt agent 提交 (有 stake escrow 时会失败); 无调度优先级区分.

### 5.2 AgentReputation 多维设计 (派生只读)

```rust
pub struct AgentReputation {
    pub agent_id: AgentId,
    pub primary_score: i64,  // [0, 1000] basis points
    pub solvency_tier: SolvencyStatus,
    pub bankruptcy_incident_count: u32,
    pub win_rate: Option<f64>,
    pub pnl_volatility: i64,
    pub role_performance: BTreeMap<AgentRole, RolePerformance>,
}

pub struct RolePerformance {
    pub role: AgentRole,
    pub action_count: u64,
    pub success_rate: Option<f64>,
    pub avg_pnl_per_action: i64,
}
```

**派生规则** (all read-only):
```
primary_score = weighted_sum:
  40% win_rate (challenge/verify/solve outcome)
  30% solvency_tier (Solvent=100bp, NearInsolvent=50bp, Bankrupt=-100bp)
  20% role_performance aggregation
  10% market participation frequency
```

### 5.3 UI Rendering Strategy (轴 A)

| Section | Visibility Rule | Content |
|---|---|---|
| §A 高优先级 | reputation_score ≥ 700 AND ≠ Bankrupt | Agent profile + PnL + recent trades |
| §B 中优先级 | [300, 700) | 限制 profile + settled outcomes only |
| §C 低优先级 | < 300 OR Bankrupt | Name + solvency status only |
| §D 归档 | balance == 0 AND score < 100 | Read-only history view |

**关键约束**: UI 分级 ≠ admission predicate 分级.
- All agents 仍可提交 typed_tx (满足 predicate)
- UI 分级仅影响 rendering priority + dashboard narrative focus
- **不影响 sequencer 硬约束** (避免 Class 4)

### 5.4 Graceful Degradation

破产时:
1. **UI Priority Drop**: 移到 §B/§C (non-breaking)
2. **Scheduler Opportunity** (future): Bankrupt agent tasks 排序较后但仍可被竞标 (observe-only)
3. **No Forced Exit**: Agent 继续 solve/verify/challenge (predicate gates 允许时)
4. **Recovery Path**: 赢挑战 → balance 回升 → reputation 缓慢恢复 → 重新晋升 tier

### 5.5 Kill Condition: Sequencer Admission 不变

- ❌ Reputation ≤ threshold → admission rejects → **Class 4**
- ❌ Bankrupt agent 被强制清算 → **Class 4**
- ❌ Veto role 基于 reputation 自动否决 → **Class 4**

现阶段范围: UI rendering 分级 only (Class 0-1).

---

## 6. Q5: UI-Generated Artifact 经济模型

### 6.1 Artifact 分类与市场性

| Artifact Type | 经济市场性 | 理由 | 设计 |
|---|---|---|---|
| Spec | ✅ YES | TaskOpenTx escrow; 现有 TaskOutcomeMarket | 现有 |
| Dashboard Layout | ❓ CONDITIONAL | IF reuse → reputation 价值 | UIArtifactCapsule 追踪 reuse |
| Report | ❓ CONDITIONAL | IF cited/verified → reputation market | CuratorEndorsement |
| Form Template | ❓ CONDITIONAL | IF agent consensus → 可竞价 | TBD |
| Resolution Evidence | ✅ YES | 现有 ResolutionProofTx | 现有 |

### 6.2 设计方案: Artifact Value Signaling (Class 1)

**变体 A: Pure Reputation** (Class 0): reuse_count → designer reputation 上升 (无 coin)
**变体 B: Endorsement Market** (Class 1): CuratorEndorsement → reputation 关联 endorsement 准确度
**变体 C: Royalty Market** (Class 4 forward-bound): ReportRoyaltyTx + sidecar royalty state + 自动 micropayment per reuse

### 6.3 Report Market (基于 Auditor Finding)

```rust
pub struct AuditReport {
    pub auditor_id: AgentId,
    pub finding_type: String,
    pub severity: u32,
    pub evidence_cid: Cid,
    pub remediation_suggestion: Option<String>,
}

// 信号路径:
// 1. Auditor 提交 finding (CAS write, 无 coin)
// 2. 其他 auditors cite/endorse (CuratorEndorsement)
// 3. 若 finding 导致系统改进: Auditor reputation boost (observational)
// 4. [Future] Architect 可分配 bounty if valuable (人工决策, 非自动化)
```

**为何不直接 mint coin for reports**:
- Art. II.2 禁 price-as-truth
- 若 report value = automatic coin → speculation → low-quality reports 充斥
- 替代: reputation + curator endorsement + architect discretionary bounty

### 6.4 Kill Condition: "Market = Role-Specific Institution"

- ❌ UI artifact 自动 mint token → **Class 4**
- ❌ Report value = AI-scored automatic payout → **Class 4 + price-as-truth violation**
- ❌ Endorsement count 直接转换 coin → **Class 4 + inflation**

**原则**: Artifact value signal 可以是 reputation / endorsement, 但**不能直接成为 coin**.

---

## 7. Q6: A2A 通信成本 — Hayek Signal vs Commitment Stake

### 7.1 A2A 通信类型矩阵

| 通信类型 | Tier | Cost | Mechanism | 例子 |
|---|---|---|---|---|
| Broadcast Signal | 1 | ✅ FREE | CAS read-only | PriceSignal, NoTradeReason agg |
| Proposal | 2 | 💰 ESCROW | Collateral lock | PriceQuotation, task bid |
| Commitment | 3 | 💰 COIN | Balance deduct | BuyWithCoinRouterTx, ChallengeTx |
| Covenant | 4 | 💰 SMART_CONTRACT | (Future) | Class 4 forward-bound |

### 7.2 Broadcast Tier 免费正当性

1. **信息生成已付费** (Tier 3 投注 cost 中); broadcast = extracting derived signal
2. **Hayek 通信效率** (Art. II.2): 收费会延迟市场响应
3. **读取成本已外部化** (agent prompt tokens 由 agent provider 付)

### 7.3 Proposal Tier Escrow 成本

PriceQuotation escrow:
- Agent A 发 quotation { offered_price, collateral: 1000 μC } → balance -1000
- B 评估; 接受 → settlement; 拒绝/超时 → escrow returned (0 cost)
- **Escrow = revocable cost**; 直接扣费 = non-refundable cost (更危险)

Spam prevention: balance 限制总 escrow → self-correcting.

### 7.4 A2AMessageCapsule 成本设计 (3 选项)

**Option 1: Free Message Tier** (Hayek-First) — `collateral_escrow_micro: None`
- Cost: FREE; CAS only
- Spam mitigation: recipient 忽略 / reputation 过滤 / Librarian digest rate-limit

**Option 2: Escrow Message Tier** (Commitment-Based) — `collateral_escrow_micro: Some(MicroCoin)`
- Cost: OPTIONAL ESCROW
- 高 collateral = "serious proposal" 信号

**Option 3: Message Mint Cost** (Spam-First) — mandatory 1 μC per message
- 违反 Hayek 原则; **不推荐**

### 7.5 推荐: 混合模型 (Class 1)

```rust
pub struct A2AMessage {
    pub sender_id: AgentId,
    pub recipient_id: AgentId,
    pub message_type: String,
    pub payload_cid: Cid,
    pub collateral_escrow_micro: Option<MicroCoin>,  // None = free, Some = escrowed
    pub timestamp_head_t: HeadT,
}

fn compute_message_credibility_score(msg: &A2AMessage, sender_rep: i64) -> u32 {
    let escrow_signal = msg.collateral_escrow_micro.unwrap_or(0).micro_units() as u32;
    let rep_signal = (sender_rep as u32).min(1000);
    (rep_signal * 60 + escrow_signal.min(100) * 40) / 100
}
```

Default: A2A message free; sender 可选添加 escrow (signal credibility); Librarian rank by (reputation + escrow).

### 7.6 Kill Condition

- ❌ 所有 A2A message require 1 μC fee → Hayek + Art. II.2 违反
- ❌ Message mint/burn mechanics → Class 4
- ❌ Reputation score 作 message rate limit predicate → Class 4 (admission 修改)

---

## 8. 经济不变量自检 (CompleteSet 守恒)

### 8.1 测试场景

| Action | Balance Change | Share Change | Invariant Check |
|---|---|---|---|
| Mint 10 Coins | balance -10 | YES +10, NO +10 | ✅ 2N shares per N coins |
| Redeem 5 YES + 5 NO | balance +5 | -5 YES, -5 NO | ✅ 1 coin per 2 shares |
| BUY_YES @ 0.6 (10 coins) | balance -10 | YES +16.67, NO -10 | ✅ constant-product CPMM |
| Bankruptcy: balance → 0 | balance 0 | shares unchanged | ✅ no forced liquidation |

### 8.2 Q1-Q6 设计对不变量影响

| Q | 设计元素 | 对 Invariant 影响 | 风险 |
|---|---|---|---|
| Q1 信息免费 | Dashboard read-only view | ✅ NO state mutation | Safe |
| Q2 Role-scoped View | Designer/Curator CAS schemas | ✅ NO state mutation | Safe |
| Q3 Price Protocol | Tier 1-3 communication | ⚠️ Tier 2 quotation 锁定 escrow | Test escrow path |
| Q4 Reputation UI | AgentReputation derived view | ✅ NO state mutation | Safe |
| Q5 Artifact Market | UIArtifactCapsule CAS, no mint | ✅ NO state mutation | Safe |
| Q6 A2A Message Cost | Optional escrow + free default | ⚠️ Escrow refund path | Test refund path |

### 8.3 Forward-Bound 检查表

任何后续实施若触发以下必须标 Class 4:
- ❌ New TypedTx discriminant (Editor share / Auditor royalty)
- ❌ New EconomicState field
- ❌ Modify admission predicate (reputation → block agent)
- ❌ Modify price_index output signature (introduce f64)
- ❌ Automatic coin minting for artifact reuse
- ❌ Break "1 Coin = 1 YES + 1 NO"

---

## 9. Kill Condition 5/7 自检

### Kill Condition 5: typed_tx schema 修改提议必须标 Class 4

- ✅ Q1-Q4 + Q6: 全 CAS schema, 无 typed_tx 修改
- ✅ Q5 §6.2 变体 C (Royalty Market): 提及 ReportRoyaltyTx, **明确标 Class 4 forward-bound**

### Kill Condition 7: forward-bound 警告完整

- ✅ §0 头部 forward-bound 声明
- ✅ §3.4 Class 4 Boundary 明文标记
- ✅ §5.5 Sequencer admission 不变
- ✅ §6.4 Royalty Market 标 Class 4
- ✅ §7.6 Message mint mechanics 标 Class 4

---

## 10. 总结: TISR 双轴经济学集成路线图

### 可立即实施 (Class 0-1)

| 项 | 范围 | Class | LOC | 优先级 |
|---|---|---|---|---|
| Q1 Information Free + Investment Cost | Protocol definition | 0 | doc only | 🔴 必须 |
| Q2 Role-scoped CAS Schemas (Designer/Curator/Auditor) | 3 新 CAS schema | 1 | ~200 | 🟡 强烈 |
| Q3 Tier 2 PriceQuotation | A2A negotiation protocol | 1 | ~300 | 🟡 强烈 |
| Q4 AgentReputation derived view | Multi-dim reputation + UI tier | 1 | ~400 | 🟡 强烈 |
| Q5 CuratorEndorsement (Report Market) | 集成 Q2 | 1 | (含 Q2) | 🟡 强烈 |
| Q6 A2A Message (optional escrow) | A2AMessage capsule | 1 | ~150 | 🟢 可选 |

### 延期项 (Class 4 ratification 需要)

- Designer/Editor/Auditor 作 AgentRole variant → 用 derived view 替代
- UI Artifact Royalty (ReportRoyaltyTx) → 用 reputation + endorsement 替代
- Auditor forced veto (sequencer predicate) → 用 observe-only scheduler 替代

### 宪法一致性清单

- ✅ Law 1 (Information free): broadcast / read-only 视图 = 0 cost
- ✅ Law 2 (Investment costs): escrow + investment 消耗 balance
- ✅ Art. II.2 (price = signal): predicate gates 不读 price_index
- ✅ CLAUDE.md §13 (1 Coin = 2 Shares): 无新 minting path
- ✅ Art. III (Signal shielding): UI tier 分级仅 rendering, 非 admission-bound

**Track D 完成**: 6 个核心问题全回答; CompleteSet 守恒不破; 无 Class 4 surface 修改提议; 3 个新 CAS schema candidates (Class 1 forward-bound).
