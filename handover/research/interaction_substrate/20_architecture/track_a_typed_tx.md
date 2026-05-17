# Phase 2 / Track A — Typed Tx / Sequencer 重点节点采样

**本文档为 forward-bound research 提案；任何派生 TB 须独立 §8 ratification；本文档不构成已批准方案。**

**作者**: TISR Phase 2 Explore subagent (2026-05-17)
**Scope**: 重点节点采样, 不做完整 11k 行映射 (typed_tx.rs 4636 + sequencer.rs 7129)

---

## 1. 任务 Scope

**范围**: 关键路径节点采样 (Signature 类型 + TypedTx 19 variants + agent ingress barrier + emit_system_tx + genesis preseed + lean_market trigger)
**排除**: 不重复 Phase 0 inventory; 不做完整 11k 行映射
**目标**: 回答 "TISR Phase 6 CLI 如何最小侵入地与 typed_tx/sequencer 适配"

---

## 2. 关键节点读后笔记

### 2.1 Signature 类型分离 (typed_tx.rs:80-100)

```
AgentSignature(Ed25519 [u8;64])  — agent-side detached signature over canonical_digest
SystemSignature(Ed25519 [u8;64]) — system-emitted signature (distinct type-at-source)
```

**含义**: Codex safety gate — type-distinct signature prevents agent↔system signature confusion at API boundaries. 任何新 tx variant 必须遵守: agent-signed tx 用 AgentSignature, system-emitted tx 用 SystemSignature (从 emit_system_tx 内部签署).

### 2.2 TypedTx 完整 19 Variants (typed_tx.rs:2327-2380)

**Agent-Signed** (13): Work, Verify, Challenge, Reuse, TaskOpen, EscrowLock, CompleteSetMint, CompleteSetRedeem, MarketSeed, CompleteSetMerge, CpmmPool, CpmmSwap, BuyWithCoinRouter
**System-Emitted** (6): FinalizeReward, TaskExpire, TerminalSummary, ChallengeResolve, TaskBankruptcy, EventResolve

### 2.3 Agent Ingress Barrier (sequencer.rs:4037-4090)

```
submit_agent_tx() → pre-queue rejection match:
  FinalizeReward | TaskExpire | TerminalSummary
  | ChallengeResolve | TaskBankruptcy | EventResolve
  ⇒ SubmitError::SystemTxForbiddenOnAgentIngress
```

Anti-Oreo Art V.1.3 结构性强制 (不是文档规范). 任何新系统 variant 自动扩展此 match (never bypass).

### 2.4 emit_system_tx 函数与 SystemEmitCommand (sequencer.rs:4208-4235)

```rust
pub async fn emit_system_tx(&self, command: SystemEmitCommand)
  → Result<SystemEmitReceipt, EmitSystemError>

Step 1: build_signed_system_tx(command) — 构造 + 内部签署
Step 2: verify_emitted_system_tx_signature(&tx) — 防御深度
Step 3: next_emit_id.fetch_add(1) — 分配 emit_id (与 submit_id 独立计数器)
Step 4: queue_tx.try_send(envelope) — 推送到共享队列
```

**SystemEmitCommand 枚举** (sequencer.rs:3574-3664) 6 variants:
- ChallengeResolve(target_challenge_tx_id, resolution)
- FinalizeReward(claim_id) — Q-derives task_id / solver / reward
- TaskExpire(task_id, escrow_tx_id, reason)
- TerminalSummary(run_id, task_id, ..., evidence_capsule_cid)
- TaskBankruptcy(task_id, evidence_capsule_cid, ...)
- EventResolve(task_id, outcome) — Open→Finalized (YES) 或 Open→Bankrupt (NO)

### 2.5 Genesis / Preseed 模式 (lean_market.rs:146-246)

lean_market `cmd_run_task` spawn evaluator subprocess with env vars (TURINGOS_USER_TASK_MODE / TURINGOS_USER_TASK_BOUNTY_MICRO / TURINGOS_CHAINTAPE_PRESEED). evaluator preseed branch 检测 env vars → submit TaskOpen + EscrowLock signed by **Agent_user_0** (TB-9 durable keystore).

**不是 emit_system_tx**: user task 是 agent-signed (Agent_user_0), 通过 submit_agent_tx 路径. system_emitted 只在结果确定后触发 (FinalizeReward / EventResolve).

### 2.6 EconomicJudgment — CAS Schema (economic_judgment.rs:1-92)

CAS-backed REAL-12 Bull/Bear 经济判断. **NOT typed_tx — 不消耗 typed_tx admission**.

Schema ID: `"real12.economic_judgment.v1"`.
Fields: agent_id, role, task_id, intended_side, action (Buy/Short/Abstain), reason (EconomicReason), estimated_probability_band, public_summary.

**agent 间通信的"判断"层走 CAS, 不走 typed_tx**.

---

## 3. Q1: TISR Phase 6 CLI 如何 trigger system_emitted tx?

**答**: Phase 6 CLI 应直接调用 **`sequencer.emit_system_tx(SystemEmitCommand)`**, 绝对不走 submit_agent_tx (会触发 SystemTxForbiddenOnAgentIngress).

**现成函数** (sequencer.rs:4208-4235):
```rust
pub async fn emit_system_tx(&self, command: SystemEmitCommand)
  → Result<SystemEmitReceipt, EmitSystemError>
```

唯一的 system-emitted 入口. Phase 6 CLI 需要:
1. 获取 Sequencer 实例 (通过 runtime bootstrap)
2. 构造相应的 SystemEmitCommand 变体 (6 种之一)
3. await emit_system_tx(command)
4. 处理 Result<SystemEmitReceipt, EmitSystemError>

**参数映射** (CLI → SystemEmitCommand):

| CLI 命令示例 | SystemEmitCommand | 参数来源 |
|---|---|---|
| `turingos market trigger finalize --claim-id X` | FinalizeReward { claim_id } | CLI 直接指定 |
| `turingos market trigger event-resolve --task T --outcome YES` | EventResolve { task_id, outcome } | CLI 直接指定 |
| `turingos task expire --task T --escrow X --reason Deadline` | TaskExpire { task_id, escrow_tx_id, reason } | CLI 指定; sponsor_agent/bounty Q-derived |
| `turingos terminal-summary --run R --task T --outcome Pass` | TerminalSummary { run_id, task_id, ... } | CLI 指定; evidence_capsule_cid CAS lookup |

**无新函数需求**: emit_system_tx + build_signed_system_tx 已覆盖全 6 个 command. CLI 只需包装命令行参数 → SystemEmitCommand 的转换逻辑 (Track CLI-A 范围).

---

## 4. Q2: Agent 自接入 polymarket 的 Class 4 forward-bound candidates

TISR Phase 6 轴 A (CLI 触发 system_emitted) 与轴 B (agents 在 tape 上通信) 的分界点是: **轴 A 的所有任务初始化都由人类 spec 批准后通过 CLI 触发 (走 system_emitted 路径)**.

"Agent 自接入 polymarket" (未来扩展) 意味着 agents 可以在 tape 上自主声明新任务/新市场, 绕过人类批准. 需要的 typed_tx 演化:

### Class 4 Forward-Bound Candidates (不在 TISR scope)

1. **AgentProposedTaskOpen** — agent-signed (非 system_emitted) TaskOpen 变体. Must extend agent ingress barrier match. Requires Codex VETO: agent_tx 能否修改 task_markets_t 状态? Class 4: new variant + ingress 扩展 + dispatch 分叉.

2. **AgentMarketSeeding** — 当前 MarketSeed 是 system_emitted (人类触发). Agent-signed 版本允许 agents 自主 seed 已存在任务的市场. Conflict check 需要 (prevent duplicate seed). Class 4: new variant + dispatch 分叉.

3. **AgentPoolInitiation** — 当前 CpmmPool 已 agent-signed (architect §7.5). 此变体允许 agents 自主声明新 CPMM 市场 (不依赖人类 CLI 初始化). Fee governance: agent-set vs fixed protocol default. Class 4.

4. **AgentOraclePriceFeed** (REAL-12 Extended) — agents 提交"对局外价格的公开判断"创建 L4 证据链. Use case: agent 自主 seed 市场前先提交价格观察 (transparency). Class 4: new variant + CAS integration.

### 通用约束 (未来设计时)

- 所有 agent-proposed variants 必须通过 manifest-when-set 签名门 (sequencer.rs:4099+)
- 每个新 variant 自动扩展 submit_agent_tx ingress barrier (never bypass)
- 若涉及经济状态变更 (escrow/collateral), 必须 Codex VETO signed-off
- 不允许创建新 system-emitted-only variant (与轴 A "人类 spec + CLI" 原则冲突)

---

## 5. Q3: A2A message 走 CAS schema vs typed_tx?

**结论**: **是的, TISR 轴 B 的 agent-to-agent 消息应该走 CAS schema, 不走 typed_tx**.

**理由**:

1. **typed_tx 是 L4 ledger 状态转移的公式**. 每个 variant 必须:
   - 映射到 state root advance
   - 可能涉及经济资金流动
   - 通过 dispatch_transition 进入 replay 验证
   - 占用 bounded queue (MAX_TX_PER_RUN 约束)

2. **CAS schema 是 evidence / audit trail 存储层**. 每个 object:
   - 不改变 state root (纯信息层)
   - 自由编码结构化判断 / 通信 / 元数据
   - 通过 content-addressed Cid 被 typed_tx 引用 (WorkTx.proposal_cid 即此模式)
   - 绕过 queue 限制 (off-chain storage; 仅 Cid 进入 L4)

3. **EconomicJudgment 设计验证** (economic_judgment.rs:1-8): "missing middle layer between market visibility and router action ... generic CAS evidence object ... does NOT change typed transaction schema, sequencer admission, wallet semantics, or Lean predicates."

4. **REAL-5 + REAL-12 合体案例**: Bull/Bear 角色分化 + 形成 EconomicJudgment 存入 CAS + agents 互相引用/批评 + 最终 action 通过 CpmmSwap/Router typed_tx 执行.

5. **librarian_broadcast** 已是 CAS + Cid 引用模式 (REAL-BCAST-1 in flight).

**Architecture Pattern** (最小侵入):
```
typed_tx (L4):  [Work] [Verify] [Challenge] ... [CpmmSwap]
                    ↑
                    └─ references via Cid

CAS (L3):       [ProposalPayload] [EconomicJudgment] [AgentSummary] [A2AMessage]
                    ↑
                    └─ Cid (content hash)

双向:
- typed_tx.proposal_cid → CAS payload
- typed_tx.economic_judgment_refs → Vec<Cid>
```

**TISR Phase 6 Implication**:
- Phase 6 CLI 不需要支持 "A2A message tx variant"
- A2A messaging infrastructure 走 librarian_broadcast + CAS (Track B 负责)
- CLI 只需支持 querying / displaying CAS-based A2A history
- **0-touch 边界**: typed_tx/sequencer 不变, CAS 层扩展 (不冲突)

---

## 6. Q4: lean_market evaluator child fork 路径追溯

**结论**: lean_market spawn 的 evaluator child 走 **agent_ingress, 不是 system_emitted**.

**路径**:
1. `lean_market.rs cmd_run_task` (line 146-246): Command::new(evaluator_bin) + env vars
2. evaluator.rs preseed branch (line ~858+): 检测 TURINGOS_USER_TASK_MODE=1
3. evaluator 内部 **submit_agent_tx(TaskOpen)** + **submit_agent_tx(EscrowLock)** — 签署主体是 Agent_user_0
4. **TaskOpen 和 EscrowLock 都在允许列表** (不在 sequencer.rs:4041-4054 拒绝清单中)
5. system_emitted 仅在结果确定后触发 (FinalizeReward / EventResolve / TaskExpire 等)

**为何 TaskOpen "轴 A 人 init" 但仍走 agent_ingress**:
- TaskOpen 由人类 (CLI env var) 触发, 但**签署主体**是 Agent_user_0
- Agent_user_0 是特殊 agent role, 代表 "用户提名的任务发起者"
- 从 sequencer 视角: agent-submitted variants, 不是 system-emitted

**TISR Phase 6 Implication**:
- Phase 6 CLI `turingos task open` 需要支持两条路:
  - **轴 A (Phase 6 MVP)**: 沿 lean_market 模式, 通过 Agent_user_0 (或新建 Agent_user_N) 走 agent_ingress 提交 TaskOpen + EscrowLock
  - **轴 B (Future Class 4)**: agents 自主 propose 任务 (AgentProposedTaskOpen)
- 现有 TURINGOS_USER_TASK_MODE env-var 模式是对轴 A seed path 的支撑; Phase 6 CLI 可保留或重构 (lib 化)

---

## 7. Q5: 人作为特殊 agent 的最小架构变更 (Class 4 forward-bound)

**目前设计** (Phase 6 MVP): 人类 = 外部 spec 批准者 (链外) → CLI 调 emit_system_tx → 所有非 Agent_user_0 初始化都是 system_emitted.

**未来想象** (Class 4): 人类 = Agent_human / Agent_auditor (链上身份) → 人类通过 Agent 签名提交各类 init tx → sequencer 统一 submit_agent_tx 路径.

### Class 4 Minimal Architecture Changes

**Change 1**: 新增 AgentRole variants (`real5_roles.rs`)
```rust
pub enum AgentRole {
    // ...existing 10 variants...
    HumanAuditor,        // 人类审计员 (仅 read; 无交易权)
    HumanSponsor,        // 人类任务发起者 (TaskOpen / EscrowLock 权)
}
```
Impact: Codex VETO required for each new role; agent_role_classifier.rs 推断扩展.

**Change 2**: `HumanSignature` variant (`typed_tx.rs`)
```rust
pub struct HumanSignature([u8; 64]); // secp256k1 或与人类密钥库一致
```
Impact: 影响 verify.rs Gate 4 (replay-time 签名验证). Codex sign-off for new Signature type.

**Change 3**: system_emitted variants 的双路径 dispatch (`sequencer.rs`)
- 允许 agent-submitted FinalizeRewardProposed / EventResolveProposed 变体 (人类作为 proposer)
- dispatch_transition 需要分叉 (system route vs agent-proposed route)

**Change 4**: Agent Pubkey Manifest 扩展 (`agent_keypairs.rs`)
- Map<AgentRole, PublicKeyType> 区分 agent vs human key

**Change 5**: Anti-Oreo 扩展 (Constitution Art V.1.3 修订)
- "human ≠ high-leverage financial actor" 政策 gate
- HumanSponsor 可以 TaskOpen 但不能 CpmmSwap

### 最小可行路径 (Phase 7+)
1. Add HumanAuditor + HumanSponsor roles (real5_roles.rs)
2. Add HumanSignature type (typed_tx.rs) — Class 4 **MUST VETO**
3. Extend manifest (agent_keypairs.rs)
4. Add dual-path dispatch 仅限于 TaskOpen / EscrowLock (最低风险)
5. Codex review: new rule gates in dispatch_transition

**为何 most minimal**: 不创建新 queue / 不改 emit_id 计数器 / 不触及 Core Variants (Work/Verify/Challenge) / 单向升级.

**风险阈值 (Codex VETO)**:
- HumanSignature type → **MUST VETO** (签名类型 = 安全核心)
- dual-path dispatch → **MUST VETO** (state root 转移逻辑变更)
- Policy gate (Anti-Oreo extension) → **MUST VETO** (constitutional rule)
- HumanAuditor role → **OK** (无状态转移权)

---

## 8. 总结: TISR Phase 6 CLI 与 typed_tx/sequencer 的 0-touch 边界

### 核心发现

1. **typed_tx + sequencer 已 STEP_B frozen**. Phase 6 MVP **不需要修改任何 variant 或 sequencer 函数签名**.
2. Phase 6 CLI 最小集成点: `sequencer.emit_system_tx()` (已存在)
3. 6 个 system_emitted variants 完全覆盖轴 A 需求 (TaskOpen/EscrowLock 通过 lean_market-like agent_ingress)
4. CAS schema (EconomicJudgment / A2A messages / librarian_broadcast) 与 typed_tx 解耦; CLI 只需 query/display
5. Agent ingress barrier 已包含所有 system 变量的拒绝列表 (structural enforcement)

### Phase 6 CLI 的 0-touch 实现

```
turingos (Phase 6 main CLI)
├── task
│   ├── open            → submit_agent_tx(TaskOpen + EscrowLock) via Agent_user_0
│   ├── view            → query chain + CAS [no queue]
│   └── tick            → CLI 时间推进 trigger (system_emitted lifecycle)
├── market
│   ├── trigger seed    → submit_agent_tx(MarketSeed) [agent-signed]
│   ├── trigger pool    → submit_agent_tx(CpmmPool) [agent-signed]
│   ├── trigger resolve → emit_system_tx(EventResolve)
│   └── trigger final   → emit_system_tx(FinalizeReward)
├── batch ...           → emit_system_tx(TerminalSummary / etc.)
├── dashboard           → query chain + CAS [no queue]
└── verify ...          → chain replay [read-only]

ALL existing typed_tx variants: ✓ REUSABLE
NO new variant needed: ✓
NO sequencer function signature change: ✓
NO queue re-architecture: ✓
```

### Phase 6 与未来扩展的断界

- **轴 A (Phase 6 MVP)**: 人类 (通过 CLI) = system spec approver; 用现有 system-emitted variants
- **轴 B (Phase 6 Concurrent, Track B 负责)**: Librarian broadcast + CAS A2A messaging; 不触及 typed_tx
- **轴 A Extension (Phase 7+, Class 4)**: Human = special agent role; HumanSignature; dual-path dispatch; **Require Codex VETO for each change**

---

## 9. Kill Condition 5/7 自检

### Kill Condition 5: "任何新 typed_tx variant 提议必须标 Class 4"

- ✅ Q1: 建议 `emit_system_tx()`, 无新 variant
- ✅ Q2: 列出 4 个 candidates, 全部标 "Class 4 forward-bound, 不在 TISR scope"
- ✅ Q4: lean_market 走现有 agent_ingress, 无新 variant
- ✅ Q5: 虽需要 HumanSignature + dual-path, 但全部标 "Class 4 forward-bound"

**结论**: 无暗藏的 Class 4 提议混入正文 ✅

### Kill Condition 7: "任何 sequencer admission 修改必须标 Class 4"

- ✅ Q1: emit_system_tx 无 admission 修改
- ✅ Q2: 所有 agent-proposed candidates 说明 "需扩展 ingress barrier match (never bypass)", 标 Class 4
- ✅ Q3: A2A messages 走 CAS, 无 typed_tx admission 修改
- ✅ Q4: TaskOpen/EscrowLock 已在允许列表
- ✅ Q5: 人角色扩展会修改 ingress barrier, 但标 "Class 4 MUST VETO"

**结论**: ingress barrier 相关改动全部标 Class 4 ✅

---

## 10. 参考节点清单

| 文件 | 行号 | 含义 |
|---|---|---|
| `typed_tx.rs` | 82-100 | AgentSignature vs SystemSignature 类型区分 |
| `typed_tx.rs` | 2327-2380 | TypedTx 19 variants 完整枚举 |
| `typed_tx.rs` | 2417-2425 | HasSubmitter trait |
| `sequencer.rs` | 4037-4090 | submit_agent_tx ingress barrier |
| `sequencer.rs` | 3574-3664 | SystemEmitCommand 6 variants |
| `sequencer.rs` | 4208-4235 | emit_system_tx 函数 |
| `sequencer.rs` | 4242-4529 | build_signed_system_tx (6 variants) |
| `sequencer.rs` | 4534-4600 | verify_emitted_system_tx_signature |
| `lean_market.rs` | 146-246 | cmd_run_task preseed (agent_ingress TaskOpen) |
| `economic_judgment.rs` | 1-92 | EconomicJudgment CAS schema (non-typed_tx) |

**Track A 完成: Phase 6 CLI 与 sequencer 的 0-touch 边界已论证; 无新 typed_tx 变体提议; 4 个 Class 4 forward-bound candidates 标明为未来扩展.**
