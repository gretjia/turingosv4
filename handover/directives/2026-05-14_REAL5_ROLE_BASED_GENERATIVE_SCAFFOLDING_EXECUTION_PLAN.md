# REAL-5 Role-Based Generative Scaffolding 执行方案

## Summary

本方案已按独立 Agent 审计意见修正：原先要点版被判 `CHALLENGE`，原因是没有把架构师 Atom 0-9、验收 SG、REAL-6、验证总表、AI coder 口令原文直接纳入。当前版本将这些内容作为执行方案的不可改写事实源逐字保留，并把本地化实现说明放在原文之后，避免语义漂移。

执行总目标：

```text
从 proof-solver market
升级为 role-based generative market。
```

执行前第一步必须保存架构师原文到：

```text
handover/directives/2026-05-14_REAL5_ROLE_BASED_GENERATIVE_SCAFFOLDING_ARCHITECT_ORIGINAL.md
```

执行方案保存到：

```text
handover/directives/2026-05-14_REAL5_ROLE_BASED_GENERATIVE_SCAFFOLDING_EXECUTION_PLAN.md
```

独立审计保存到：

```text
handover/audits/CODEX_REAL5_EXECUTION_PLAN_ALIGNMENT_REVIEW.md
```

风险分级默认按 **Class 4 package** 处理，因为 Atom 3 要求升级 `PromptCapsule`，而当前 repo 明确存在 architect-pinned exact 7-field shape gate。`PromptCapsule v2 / dual-reader` 只允许作为架构师 Atom 3 schema 的 Class-4 实现方式，不允许作为语义替代或静默降级。

## Step 0 — 原文归档

执行者必须先逐字保存用户给出的架构师原文，不做摘要、不改写、不重排。保存文件必须包含完整原文块。

原文事实源：

```text
handover/directives/2026-05-14_REAL5_ROLE_BASED_GENERATIVE_SCAFFOLDING_ARCHITECT_ORIGINAL.md
```

## Architect Verbatim Requirements

以下要求逐字来自架构师原文。执行者不得压缩、改写或语义替换。

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

## Implementation Adapter

执行者按上面原文逐 Atom 落地；下面只是仓库本地化约束，不改变原文语义。

- 打开 `turingos_dev`：`module=REAL-5 Role-Based Generative Scaffolding`，`risk=4`，`unit=atom` 或按 Atom 拆分多个 atom run。FC 映射为 FC1 externalized role action loop、FC2 genesis/batch role assignment + PromptCapsule replay、FC3 dashboard/audit/materialized view、Art. III shielding、market/economic budget gates。
- Atom 0 先做文档与 decision records。不得把 charter 写成 DeFi 扩展、交易策略优化或 prompt tuning 继续轮。
- Atom 1 默认把初始 role assignment 存入 `GenesisReport`；resume/continuation 场景可同步进入 `BatchContinuationManifest`。不得写入 `EconomicState`，除非未来有独立 §8 批准角色可交易/可变。
- Atom 2 只实现 role-scoped `derive_view`；禁止 `get_all_context`；ordinary read view 不包含 raw CoT、raw logs、raw diagnostics。
- Atom 3 是 Class-4 schema/authority 点。当前 `PromptCapsule` exact 7-field gate 必须被显式迁移；可采用 `PromptCapsuleV2` + dual-reader 兼容旧证据，但最终新外部化 attempt 必须满足架构师给出的 role/view schema 与 SG-R5.3.*。不得用 sidecar 假装完成 Atom 3。
- Atom 4 的 `RoleAction` 必须是唯一 role-output gateway。角色越权输出必须进 L4.E 或 typed abstain/no-reason trace，不能静默跳过。
- Atom 5 的 `TickBudget` 是 scheduling/capability budget，不是 Coin 思考费；账本必须可从 ChainTape/CAS 派生。
- Atom 6-7 必须让 Trader/Verifier/Challenger 在“无动作”时也产生 tape-visible reason trace。
- Atom 8 只能 proposal + veto + sandbox/canary；不得自动修改工具、predicate 或生产系统。
- Atom 9 只证明 role scaffold 和 evidence reconstruction；没有交易时按 REAL-6 前置条件产出 clean-negative，不继续 A07/A08/A09 prompt tuning。

## Test Plan

新增或扩展测试文件建议：

- `tests/constitution_real5_role_assignment.rs`
- `tests/constitution_real5_role_scoped_view.rs`
- `tests/constitution_real5_prompt_capsule_v2.rs`
- `tests/constitution_real5_typed_generation_gateway.rs`
- `tests/constitution_real5_tick_budget.rs`
- `tests/constitution_real5_trader_activation.rs`
- `tests/constitution_real5_verifier_challenger.rs`
- `tests/constitution_real5_architect_veto_scaffold.rs`
- `tests/constitution_real5_role_based_smoke.rs`

必须覆盖原文 gate：

```text
role_assignment_replayable
role_prompt_capsule_resolves
role_allowed_tools_enforced
role_action_typed
role_action_l4_l4e_routing
market_visible_to_trader
market_decision_or_no_trade_required
scripted_trade_l4
failed_trade_l4e
no_ghost_liquidity
price_signal_not_truth
tick_budget_consumed_on_externalized_generation
invalid_generation_penalty
accepted_generation_reward
architect_proposal_veto_path
```

执行命令：

```bash
cargo test --test constitution_real5_role_assignment
cargo test --test constitution_real5_role_scoped_view
cargo test --test constitution_real5_prompt_capsule_v2
cargo test --test constitution_real5_typed_generation_gateway
cargo test --test constitution_real5_tick_budget
cargo test --test constitution_real5_trader_activation
cargo test --test constitution_real5_verifier_challenger
cargo test --test constitution_real5_architect_veto_scaffold
cargo test --test constitution_real5_role_based_smoke
bash scripts/run_constitution_gates.sh
cargo test --workspace --no-fail-fast -- --test-threads=1
```

REAL-5 smoke evidence command shape：

```bash
PHASE_D_HETERO_OK=1 \
TURINGOS_MARKET_ARENA_PROMPT=1 \
TURINGOS_REAL5_ROLE_VIEWS=1 \
TURINGOS_REAL5_ROLE_ASSIGNMENT="Solver,Trader,Verifier,Challenger,Observer" \
bash scripts/run_g_phase_batch.sh g_phase_real_5_role_smoke_<UTC> mini
```

验收 evidence 必须能证明：

```text
one persistent runtime_repo
same CAS
>= 5 agents
3–5 MiniF2F tasks
market enabled
role views enabled
no forced trading
```

## Audit And Approval

计划审计：

- 独立 Agent 已对要点版给出 `CHALLENGE`，要求把原文完整纳入。
- 当前方案已按该审计意见修正。
- 执行阶段必须把最终方案落盘后，再生成：
  `handover/audits/CODEX_REAL5_EXECUTION_PLAN_ALIGNMENT_REVIEW.md`
- 审计问题必须逐条对照架构师原文，结论格式：
  `PROCEED | CHALLENGE | VETO`

实现审计：

- 完成 gates + smoke + evidence 后，必须启动 clean-context Codex review。
- 审计产物：
  `handover/audits/CODEX_REAL5_IMPLEMENTATION_REVIEW.md`
- `VETO` 阻塞 ship；`CHALLENGE` 必须修复或显式 forward-deferral；`PROCEED` 仍不替代 gates/evidence。

## Assumptions

- 架构师原文是 REAL-5 当前最高 directive；若与旧 PromptCapsule 7-field gate 冲突，以本次 REAL-5 批准后的 Class-4 schema migration 为准。
- REAL-5 不改 constitution/flowchart 文本；但会触碰 replay/evidence/schema 权威面，因此按 Class 4 package 处理。
- 不追求 E2/E3 强行交易结论；REAL-5 的首要成功是 role-based generation scaffold、tape-visible decisions、typed gateway、role-scoped view 与 reconstructable evidence。
