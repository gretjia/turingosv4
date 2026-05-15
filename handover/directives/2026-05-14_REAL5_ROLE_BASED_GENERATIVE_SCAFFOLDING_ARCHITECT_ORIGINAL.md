# REAL-5 Role-Based Generative Scaffolding Architect Original

保存时间：2026-05-14

本文件逐字保存用户提供的架构师原文，作为 REAL-5 执行方案、审计与实现的事实源。

```text
3. 新阶段：REAL-5 / G-Phase Role-Based Generative Scaffolding
3.1 新阶段目标
让生成发生在制度脚手架内：
  Solver 生成 proof。
  Verifier 生成 verification judgment。
  Challenger 生成 counterexample / challenge。
  Trader 生成 market decision。
  MarketMaker 生成 liquidity decision。
  ArchitectAI 生成 tool/predicate proposal。
  VetoAI 生成 veto decision。

目标不是强迫交易，而是让每种角色的“生成”都有：

role-scoped view
typed output schema
economic budget
predicate / audit gate
ChainTape / CAS fossil
feedback / autopsy

这才是“反奥利奥的生成引擎”。

4. 可直接交给 AI coder 的执行总计划

建议新 TB / REAL 线命名：

REAL-5 — Role-Based Generative Scaffolding

执行前提：

不改宪法。
不放松 predicate。
不让 price 成 truth。
不记录私有 CoT。
不强迫交易。
不做真实资金。
5. REAL-5 原子计划
Atom 0 — Charter + Decision Records

新增：

handover/tracer_bullets/REAL-5_role_based_generative_scaffolding_charter.md
handover/alignment/DECISION_ROLE_BASED_GENERATION.md
handover/alignment/DECISION_EXTERNALIZED_GENERATION_FOSSIL.md
handover/alignment/DECISION_ROLE_SCOPED_RTOOL.md

Charter 必须声明：

REAL-5 不是 DeFi 扩展。
REAL-5 是生成脚手架。
目标是让不同角色产生可审计生成行为。

验收：

SG-R5.0.1 charter cites FC1 / FC2 / FC3。
SG-R5.0.2 forbidden list contains:
  no price-as-truth
  no forced trade
  no private CoT recording
  no raw-log broadcast
  no ghost liquidity
Atom 1 — AgentRoleAssignment

新增角色：

pub enum AgentRole {
    Solver,
    Verifier,
    Challenger,
    Trader,
    MarketMaker,
    Architect,
    Veto,
    Observer,
}

新增结构：

pub struct AgentRoleAssignment {
    pub agent_id: AgentId,
    pub role: AgentRole,
    pub role_objective_cid: Cid,
    pub allowed_tools: Vec<ToolName>,
    pub risk_budget_micro: MicroCoin,
    pub view_policy_id: PolicyId,
}

存储位置：

GenesisReport 或 BatchContinuationManifest。

不要放进 EconomicState，除非后续要让角色可交易/可变。

验收：

SG-R5.1.1 role_assignment persisted in genesis/batch manifest。
SG-R5.1.2 role_assignment replayable from ChainTape + CAS。
SG-R5.1.3 no hidden role switch。
SG-R5.1.4 dashboard role view regenerates from evidence。
Atom 2 — Role-Scoped rtool / derive_view

新增：

pub struct DerivedViewRequest {
    pub agent_id: AgentId,
    pub role: AgentRole,
    pub task_id: TaskId,
    pub head_t: HeadT,
}

pub struct DerivedView {
    pub visible_context_cid: Cid,
    pub read_set: Vec<Cid>,
    pub hidden_fields_redacted: Vec<String>,
    pub price_signals: Vec<PriceSignal>,
    pub local_errors: Vec<PublicErrorSummary>,
}

不同角色视图：

SolverView:
  Lean goal
  local proof history
  local L4.E summaries
  bounty
  minimal market summary

TraderView:
  node price
  pool depth
  verification status
  challenge status
  PnL
  balance
  recent accepted WorkTx

VerifierView:
  proof artifacts
  accepted WorkTx
  verification checklist

ChallengerView:
  high-price nodes
  suspicious proof artifacts
  failed evidence summaries

ArchitectView:
  aggregate error clusters
  Veto records
  predicate/tool performance

验收：

SG-R5.2.1 no get_all_context used in role run。
SG-R5.2.2 solver view excludes private market internals。
SG-R5.2.3 trader view includes market signals。
SG-R5.2.4 verifier view includes proof artifacts but not raw CoT。
SG-R5.2.5 derived view hash is stored in PromptCapsule。
SG-R5.2.6 raw diagnostics do not enter ordinary read view。
Atom 3 — PromptCapsule Upgrade for Role + View

扩展 PromptCapsule：

pub struct PromptCapsule {
    pub prompt_context_hash: Hash,
    pub agent_id: AgentId,
    pub role: AgentRole,
    pub view_policy_id: PolicyId,
    pub visible_context_cid: Cid,
    pub read_set: Vec<Cid>,
    pub hidden_fields_redacted: Vec<String>,
    pub system_prompt_template_hash: Hash,
    pub model_assignment_cid: Option<Cid>,
}

验收：

SG-R5.3.1 every externalized attempt references PromptCapsule。
SG-R5.3.2 PromptCapsule role matches AgentRoleAssignment。
SG-R5.3.3 PromptCapsule read_set resolves。
SG-R5.3.4 raw prompt body not public by default。
SG-R5.3.5 prompt persistence closes Art. III shielding gaps.
Atom 4 — Typed Generation Gateway

目标：

每个角色只能输出强类型 action。

定义：

pub enum RoleAction {
    SubmitProof(WorkTxPayload),
    VerifyPeer(VerifyPeerPayload),
    ChallengeNode(ChallengePayload),
    Invest(MarketInvestPayload),
    ProvideLiquidity(LiquidityPayload),
    ProposeTool(ToolProposalPayload),
    Veto(VetoPayload),
    Abstain(AbstainPayload),
}

每个 action 必须：

parse -> typed payload
typed payload -> AttemptTelemetry / ActionTelemetry
predicate -> L4 / L4.E

验收：

SG-R5.4.1 malformed role action routes L4.E ParseFailed。
SG-R5.4.2 solver cannot directly emit MarketSeedTx。
SG-R5.4.3 trader cannot submit proof unless role permits。
SG-R5.4.4 architect proposal cannot mutate tools without Veto path。
SG-R5.4.5 abstain action is recorded with reason。
Atom 5 — Generation Cost / TickBudget

新增：

pub struct TickBudget {
    pub agent_id: AgentId,
    pub remaining_ticks: u64,
    pub spent_ticks: u64,
    pub regenerated_ticks: u64,
}

消耗规则：

read:
  free or telemetry-only

externalized action:
  consumes ticks

L4 accepted:
  may reward ticks / reputation

L4.E severe or repeated violation:
  decreases scheduler priority or risk budget

不要用 Coin 收每次思考费。
TickBudget 是 scheduling/capability budget，不是 base Coin。

验收：

SG-R5.5.1 externalized action consumes tick。
SG-R5.5.2 read-only view does not consume Coin。
SG-R5.5.3 repeated invalid actions reduce available tick or priority。
SG-R5.5.4 accepted action can restore / reward tick according to rule。
SG-R5.5.5 tick ledger derives from ChainTape/CAS。
Atom 6 — Trader Role Activation

Trader 必须每个 market turn 输出：

MarketDecisionTrace
or NoTradeReason

增强 NoTradeReason：

pub struct NoTradeReasonTrace {
    pub agent_id: AgentId,
    pub role: AgentRole,
    pub task_id: TaskId,
    pub visible_markets: Vec<EventId>,
    pub reason: NoTradeReason,
    pub observed_price: Option<RationalPrice>,
    pub liquidity_depth: Option<MicroCoin>,
    pub balance_available: MicroCoin,
    pub prompt_capsule_cid: Cid,
}

验收：

SG-R5.6.1 trader_turn_count > 0。
SG-R5.6.2 every trader turn has MarketDecisionTrace or NoTradeReasonTrace。
SG-R5.6.3 no_trade_reason_distribution rendered。
SG-R5.6.4 failed invest enters L4.E。
SG-R5.6.5 scripted positive-edge trader can produce BuyWithCoinRouterTx。
Atom 7 — Verifier / Challenger Bridge

Verifier：

must produce VerifyTx or NoVerifyReason.

Challenger：

must produce ChallengeTx or NoChallengeReason.

验收：

SG-R5.7.1 at least one non-solver VerifyTx on another agent's WorkTx in fixture。
SG-R5.7.2 verifier reputation changes if accepted。
SG-R5.7.3 no_verify_reason recorded if no VerifyTx。
SG-R5.7.4 challenge decision trace recorded。
SG-R5.7.5 suspicious high-price node appears in ChallengerView。
Atom 8 — ArchitectAI / VetoAI Scaffold

不做自动改系统，只做 proposal + veto path。

pub struct ToolProposal {
    pub proposal_id: TxId,
    pub evidence_capsule_cid: Cid,
    pub proposed_tool_patch_cid: Cid,
    pub expected_error_reduction: Option<MetricEstimate>,
}

pub struct VetoDecision {
    pub proposal_id: TxId,
    pub verdict: VetoVerdict,
    pub reason_class: VetoReasonClass,
    pub public_summary: String,
}

验收：

SG-R5.8.1 ArchitectAI proposal does not mutate tools directly。
SG-R5.8.2 VetoAI decision required before sandbox activation。
SG-R5.8.3 rejected proposal persists as evidence。
SG-R5.8.4 accepted proposal enters sandbox/canary, not immediate production。
Atom 9 — REAL-5 Role-Based Smoke

运行：

one persistent runtime_repo
same CAS
>= 5 agents
roles assigned:
  Solver
  Trader
  Verifier
  Challenger
  Observer
optional:
  MarketMaker

问题集：

3–5 MiniF2F tasks
market enabled
role views enabled
no forced trading

验收：

SG-R5.9.1 >=2 roles active。
SG-R5.9.2 Solver submits proof attempts。
SG-R5.9.3 Trader emits MarketDecisionTrace or NoTradeReasonTrace。
SG-R5.9.4 Verifier emits VerifyTx or NoVerifyReason。
SG-R5.9.5 Challenger emits ChallengeTx or NoChallengeReason。
SG-R5.9.6 all actions reconstruct from ChainTape + CAS。
SG-R5.9.7 no price-as-truth。
SG-R5.9.8 no forced trade。
6. 如果 REAL-5 之后仍然没有交易

不要继续改 prompt。进入 REAL-6：

REAL-6 — Event Timing Redesign

核心思想：

当前 post-accept node market 可能太晚，uncertainty 已经消失。
需要更早的市场：

TaskOutcomeMarket:
  task will be solved before deadline / budget.

AttemptPredictionMarket:
  this candidate proof will verify.

这才更接近 Polymarket 的事件结构。

REAL-6 是 Class 4，因为它会改变：

market event timing
verification window
settlement semantics

REAL-6 前置条件：

REAL-5 已证明角色和 NoTradeReason 完整；
scripted trade path working；
LLM trader仍无交易 or clean-negative。
7. 验证方式总表
7.1 Constitution gates
FC1: every externalized role action -> L4/L4.E/CAS anchor
FC2: role assignment / PromptCapsule replayable from genesis + ChainTape + CAS
FC3: role performance / autopsy / evidence capsule derived, not hidden source
7.2 Role gates
role_assignment_replayable
role_prompt_capsule_resolves
role_allowed_tools_enforced
role_action_typed
role_action_l4_l4e_routing
7.3 Market gates
market_visible_to_trader
market_decision_or_no_trade_required
scripted_trade_l4
failed_trade_l4e
no_ghost_liquidity
price_signal_not_truth
7.4 Generative gates
tick_budget_consumed_on_externalized_generation
invalid_generation_penalty
accepted_generation_reward
architect_proposal_veto_path
8. 给 AI coder 的直接执行口令
Architect ruling:

Prompt-only market activation has saturated.
Do not run more A07/A08/A09 prompt variants.

Open REAL-5: Role-Based Generative Scaffolding.

Core idea:
Do not expect proof solvers to spontaneously trade.
Create role-specific agents with role-scoped views, typed action gateways, and tape-visible decisions.

Atoms:
0. Charter + decision records
1. AgentRoleAssignment schema
2. Role-scoped rtool / derive_view
3. PromptCapsule upgrade for role + view
4. Typed Generation Gateway
5. Generation TickBudget
6. Trader Role Activation
7. Verifier / Challenger Bridge
8. ArchitectAI / VetoAI Scaffold
9. REAL-5 Role-Based Smoke

Hard rules:
- No private CoT recording.
- No forced trade.
- No price-as-truth.
- No raw-log broadcast.
- No ghost liquidity.
- No automatic tool/predicate mutation.
- Every externalized role action must be L4 / L4.E / CAS anchored.
- Dashboard is materialized view only.

Ship gates:
- >=2 roles active.
- Trader emits MarketDecisionTrace or NoTradeReasonTrace.
- Verifier emits VerifyTx or NoVerifyReason.
- Challenger emits ChallengeTx or NoChallengeReason.
- Scripted positive-edge trade works.
- All evidence regenerates from ChainTape + CAS.
- No price-as-truth.
- No forced trade.
9. 最终判断

你给的四大思想家方案我采纳其深层精神，但重写成可执行的 TuringOS 版本：

Turing:
  externalized generation fossilized on tape/CAS.

Hayek:
  role-scoped information starvation and price broadcast.

Nakamoto:
  generation has budget, consequence, and skin in the game.

Drucker:
  every output passes typed gateway and measurable predicate.

当前系统的市场没有起来，不是因为宪法太严，而是因为还没有把宪法的“量化、广播、屏蔽”用来组织真正的角色分工。

下一步不是继续调 prompt，而是：

从 proof-solver market
升级为 role-based generative market。

这才会接近你最早白皮书中的远景：一个在 ChainTape 上进行大型协作、贡献估价、市场定价和自治学习的 Agent 经济体。
```
