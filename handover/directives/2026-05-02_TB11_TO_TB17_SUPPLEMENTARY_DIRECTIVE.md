# Supplementary architect directive — TB-11 → TB-17 full requirements
## Lossless archive (Kolmogorov compression policy: no summary, full original)

**Date**: 2026-05-02 evening (post first TB-11 ruling at
`2026-05-02_TB11_EPISTEMIC_EXHAUST_ARCHITECT_RULING.md`).

**Trigger**: User-architect dispatched a supplementary directive while AI
coder was drafting TB-11 charter, providing (a) confirmation of TB-11
redirect, (b) renamed and re-ordered TB-12..TB-17, (c) explicit FR/CR/SG
requirement numbering for each TB.

**Authority chain**:
- 2026-05-02 evening user message (verbatim §1 below) issued under
  authorization "make it your own understanding but always align with the
  constitution. then you can make it all the way to the end of the
  developing TB phases without my intervention".
- This archive is operative for TB-11 implementation gates AND for TB-12
  through TB-17 charter design when those TBs land.

---

## §0 Wrapper instructions from user (verbatim)

```text
ultrathink 补充，你需要这些准确的指引，在你的开发中遇到任何问题，以下内容是你
必须要对齐的，祝你开发顺利，进入全自动模式，我在终点等你：
```

---

## §1 Architect supplementary text (verbatim)

> According to a document from 2026-05-02, 当前状态是：TB-10 已完成 **Lean Proof Task Market MVP**，用户开任务、锁 bounty、Agent 解题、系统验证、系统付款、durable identity 归属都已经在 canonical ChainTape 字节层面被验证；原计划下一步是 TB-11 做 NodeMarket Decision Record + Position Index，并且明确"不交易"。
>
> 但结合 DeepThink 对"幽灵坟场"的批判，以及我们对 Zeta 压力测试的复盘，我建议**重新排序 TB-11 到 TB-17**：
>
> > **先处理失败证据与资本释放，再做 NodeMarket。**
> > 没有失败锚点，Short/NO 没有真实结算对象；没有 TaskExpire，bounty 会被永久锁死；没有 EvidenceCapsule，失败努力仍然只能留在 evaluator 私有日志里。DeepThink 对这一点的批判是成立的：TuringOS 防住了 T4 欺骗攻击，没有错误付款，但也暴露出 "Invisible Graveyard"——大量失败尝试没有进入可审计的 ChainTape/CAS 结构，资金也可能锁死。
>
> 所以新版路线应从：
>
> ```text
> TB-11 NodeMarket
> ```
>
> 改为：
>
> ```text
> TB-11 Epistemic Exhaust & Capital Liberation
> ```
>
> 然后把你列出的 TB-12–TB-17 顺延并升级。
>
> ---
>
> # 0. 新版总路线
>
> ```text
> TB-11  Epistemic Exhaust & Capital Liberation
> TB-12  NodeMarket Position Index
> TB-13  CompleteSet + MarketSeedTx
> TB-14  PriceIndex v0 + Boltzmann Masking
> TB-15  Lamarckian Autopsy + Markov EvidenceCapsule
> TB-16  Controlled Market Smoke Arena
> TB-17  Real-World Readiness Gate
> ```
>
> 核心变化：
>
> ```text
> 旧 TB-12 的 NodeMarket 前置条件不满足；
> 必须先补 TB-11 的失败证据与资金释放。
> ```
>
> 这样才符合宪法基本法：
>
> ```text
> Information is Free
> Only Investment Costs Money
> 1 Coin = 1 YES + 1 NO
> on_init 是唯一合法铸币点
> ```
>
> 也符合反奥利奥架构的信号管理原则：顶层白盒负责量化、广播、屏蔽；失败日志不能无限制污染 Agent 上下文，只能被压缩、归档、摘要广播。
>
> ---
>
> # TB-11 — Epistemic Exhaust & Capital Liberation
>
> ## 1. 目标
>
> TB-11 的目标不是 NodeMarket，而是解决当前最大的认识论缺口：
>
> ```text
> 失败努力不能消失；
> 失败证据不能污染上下文；
> 失败任务不能永久锁死资金。
> ```
>
> Zeta 压力测试说明了这个问题：系统正确拒绝了错误证明，没有错误付款，但大量失败尝试仍然主要留在 evaluator 私有日志里，ChainTape 只看到任务开立和 bounty 锁定。
>
> TB-11 应建立：
>
> ```text
> EvidenceCapsule
> RunExhaustedTx
> TaskBankruptcyTx
> TaskExpireTx
> ```
>
> ## 2. 新增对象
>
> ### 2.1 EvidenceCapsule
>
> ```rust
> pub struct EvidenceCapsule {
>     pub capsule_id: Cid,
>     pub run_id: RunId,
>     pub task_id: TaskId,
>     pub solver_agent: Option<AgentId>,
>
>     pub attempt_count: u64,
>     pub lean_error_count: u64,
>     pub sorry_block_count: u64,
>     pub protocol_parse_failure_count: u64,
>     pub partial_accept_count: u64,
>
>     pub started_at_round: u64,
>     pub ended_at_round: u64,
>     pub terminal_reason: ExhaustionReason,
>
>     pub public_summary: String,
>     pub evidence_manifest_cid: Cid,
>     pub compressed_log_cid: Cid,
>
>     pub privacy_policy: CapsulePrivacyPolicy,
>     pub sha256: Hash,
> }
> ```
>
> ### 2.2 RunExhaustedTx
>
> ```rust
> pub struct RunExhaustedTx {
>     pub tx_id: TxId,
>     pub parent_state_root: Hash,
>     pub task_id: TaskId,
>     pub run_id: RunId,
>     pub solver_agent: Option<AgentId>,
>     pub attempt_count: u64,
>     pub evidence_capsule_cid: Cid,
>     pub terminal_reason: ExhaustionReason,
>     pub system_signature: SystemSignature,
> }
> ```
>
> ### 2.3 TaskBankruptcyTx
>
> ```rust
> pub struct TaskBankruptcyTx {
>     pub tx_id: TxId,
>     pub parent_state_root: Hash,
>     pub task_id: TaskId,
>     pub evidence_capsule_cid: Cid,
>     pub bankruptcy_reason: BankruptcyReason,
>     pub system_signature: SystemSignature,
> }
> ```
>
> ### 2.4 TaskExpireTx
>
> ```rust
> pub struct TaskExpireTx {
>     pub tx_id: TxId,
>     pub parent_state_root: Hash,
>     pub task_id: TaskId,
>     pub sponsor_agent: AgentId,
>     pub escrow_tx_id: TxId,
>     pub refund_amount: MicroCoin,
>     pub reason: ExpireReason,
>     pub system_signature: SystemSignature,
> }
> ```
>
> ## 3. Requirements
>
> ### Functional requirements
>
> ```text
> FR-11.1
> MAX_TX exhausted / timeout / solver give-up creates EvidenceCapsule in CAS.
>
> FR-11.2
> EvidenceCapsule contains counts for failed attempts, Lean errors, sorry-blocks, parse failures, partial accepts.
>
> FR-11.3
> RunExhaustedTx is system-emitted and accepted into L4.
>
> FR-11.4
> RunExhaustedTx references evidence_capsule_cid.
>
> FR-11.5
> TaskBankruptcyTx creates task-level failure anchor for future Short / NO resolution.
>
> FR-11.6
> TaskExpireTx refunds escrow to sponsor when expiry policy is satisfied.
>
> FR-11.7
> Dashboard renders Exhausted / Bankrupt / Expired states.
>
> FR-11.8
> Raw failed logs are stored in CAS but shielded from ordinary Agent read views.
> ```
>
> ### Constitutional requirements
>
> ```text
> CR-11.1
> Do not write every Lean error as a separate L4 transition.
>
> CR-11.2
> L4 stores O(1) failure anchor.
> CAS stores O(N) evidence.
>
> CR-11.3
> No fake accepted WorkTx.
>
> CR-11.4
> No payout, no slash, no NodeMarket in TB-11.
>
> CR-11.5
> EvidenceCapsule public_summary may be broadcast;
> raw compressed evidence is audit-only / authorized-view only.
> ```
>
> ### Ship gates
>
> ```text
> SG-11.1
> Zeta-style hard-fail run creates EvidenceCapsule.
>
> SG-11.2
> RunExhaustedTx appears in L4 and replay verifies.
>
> SG-11.3
> TaskExpireTx refunds locked bounty after expiry.
>
> SG-11.4
> Refund preserves total CTF.
>
> SG-11.5
> Dashboard regenerates exhausted / expired state from ChainTape + CAS.
>
> SG-11.6
> Ordinary Agent read view does not contain raw failed logs.
>
> SG-11.7
> Future Short/NO mechanism can reference TaskBankruptcyTx.
> ```
>
> ## 4. 禁止事项
>
> ```text
> No NodeMarket
> No CompleteSet
> No AMM
> No CPMM
> No Short payout
> No Slash
> No per-attempt L4 spam
> No raw failure log broadcast
> No ghost liquidity
> ```
>
> ---
>
> # TB-12 — NodeMarket Position Index
>
> ## 1. 目标
>
> TB-12 才正式引入 NodeMarket 的最小位置层，但仍然不交易。
>
> ```text
> WorkTx.stake -> FirstLongPosition
> ChallengeTx.stake -> ShortPosition
> VerifyTx.bond != market position
> NodePosition not Coin holding
> ```
>
> 原本 TB-11 的 NodeMarket 计划要求包括 `DECISION_NODEMARKET_POLYMARKET_CPMM.md`、NodePosition、FirstLongPosition、ChallengeShortPosition，且明确 no trading。
> 这个方向保留，但现在顺延到 TB-12。
>
> ## 2. 新增对象
>
> ```rust
> pub enum PositionSide {
>     Long,
>     Short,
> }
>
> pub enum PositionKind {
>     FirstLong,
>     ChallengeShort,
> }
>
> pub struct NodePosition {
>     pub position_id: TxId,
>     pub node_id: TxId,
>     pub task_id: TaskId,
>     pub owner: AgentId,
>     pub side: PositionSide,
>     pub amount: MicroCoin,
>     pub source_tx: TxId,
>     pub kind: PositionKind,
>     pub opened_at_round: u64,
> }
> ```
>
> ## 3. Requirements
>
> ### Functional requirements
>
> ```text
> FR-12.1
> Accepted WorkTx with stake creates NodePosition { side: Long, kind: FirstLong }.
>
> FR-12.2
> Accepted ChallengeTx with stake creates NodePosition { side: Short, kind: ChallengeShort }.
>
> FR-12.3
> VerifyTx.bond never creates NodePosition.
>
> FR-12.4
> NodePosition references node_id = target WorkTx.
>
> FR-12.5
> NodePosition can reference TaskBankruptcyTx / RunExhaustedTx as future NO anchor.
> ```
>
> ### Constitutional requirements
>
> ```text
> CR-12.1
> NodePosition is exposure index, not Coin holding.
>
> CR-12.2
> NodePosition.amount must not be included in total_supply_micro.
>
> CR-12.3
> Position creation cannot override predicate outcome.
>
> CR-12.4
> Price / position signal is not truth.
> ```
>
> ### Ship gates
>
> ```text
> SG-12.1
> WorkTx.stake -> FirstLongPosition test passes.
>
> SG-12.2
> ChallengeTx.stake -> ShortPosition test passes.
>
> SG-12.3
> VerifyTx.bond != market position test passes.
>
> SG-12.4
> total_supply_micro unchanged after position derivation.
>
> SG-12.5
> Dashboard can show long/short exposure per node.
>
> SG-12.6
> No trading tx variants introduced.
> ```
>
> ## 4. 禁止事项
>
> ```text
> No CompleteSet
> No MarketOrder
> No MarketTrade
> No AMM
> No automatic liquidity
> No price-based settlement
> ```
>
> ---
>
> # TB-13 — CompleteSet + MarketSeedTx
>
> ## 1. 目标
>
> 引入 Polymarket 数学核心，但仍不做交易。
>
> ```text
> 1 locked Coin = 1 YES_E + 1 NO_E
> ```
>
> ## 2. 新增对象
>
> ```rust
> pub struct CompleteSetMintTx {
>     pub tx_id: TxId,
>     pub parent_state_root: Hash,
>     pub event_id: EventId,
>     pub owner: AgentId,
>     pub amount: MicroCoin,
>     pub signature: AgentSignature,
> }
>
> pub struct CompleteSetRedeemTx {
>     pub tx_id: TxId,
>     pub parent_state_root: Hash,
>     pub event_id: EventId,
>     pub owner: AgentId,
>     pub outcome: OutcomeSide,
>     pub share_amount: ShareAmount,
>     pub system_signature: Option<SystemSignature>,
> }
>
> pub struct MarketSeedTx {
>     pub tx_id: TxId,
>     pub parent_state_root: Hash,
>     pub event_id: EventId,
>     pub provider: AgentId,
>     pub collateral_amount: MicroCoin,
>     pub signature: AgentSignature,
> }
> ```
>
> ## 3. Requirements
>
> ### Functional requirements
>
> ```text
> FR-13.1
> CompleteSetMintTx debits balances_t by amount.
>
> FR-13.2
> CompleteSetMintTx credits conditional_collateral_t by amount.
>
> FR-13.3
> CompleteSetMintTx issues equal YES_E and NO_E shares.
>
> FR-13.4
> CompleteSetRedeemTx pays winning shares only after system-resolved outcome.
>
> FR-13.5
> MarketSeedTx seeds initial liquidity using explicit provider funds.
> ```
>
> ### Constitutional requirements
>
> ```text
> CR-13.1
> No ghost liquidity.
>
> CR-13.2
> No automatic YES/NO injection.
>
> CR-13.3
> YES/NO shares are claims, not Coin.
>
> CR-13.4
> Locked collateral is Coin holding; shares are not.
>
> CR-13.5
> on_init remains the only base Coin mint.
> ```
>
> ### Ship gates
>
> ```text
> SG-13.1
> Mint 1 Coin -> 1 YES + 1 NO, total Coin conserved.
>
> SG-13.2
> YES/NO shares not counted in total Coin supply.
>
> SG-13.3
> MarketSeedTx fails if provider lacks balance.
>
> SG-13.4
> MarketSeedTx cannot create liquidity without collateral.
>
> SG-13.5
> Redeem unavailable before outcome resolution.
>
> SG-13.6
> Redeem after YES outcome pays YES, not NO.
> ```
>
> ## 4. 禁止事项
>
> ```text
> No automatic per-node 100 YES + 100 NO.
> No Treasury magic seed unless Treasury balance is debited.
> No DPMM / pro-rata payout.
> No AMM yet.
> No trading yet.
> ```
>
> ---
>
> # TB-14 — PriceIndex v0 + Boltzmann Masking
>
> ## 1. 目标
>
> 把 long/short exposure 与 conditional shares 转化为价格信号，并用于 read-view / scheduler masking。
>
> ```text
> Price is signal, not truth.
> ```
>
> ## 2. 新增对象
>
> ```rust
> pub struct NodeMarketEntry {
>     pub node_id: TxId,
>     pub task_id: TaskId,
>     pub event_id: EventId,
>     pub long_interest: MicroCoin,
>     pub short_interest: MicroCoin,
>     pub price_yes: Option<Decimal>,
>     pub price_no: Option<Decimal>,
>     pub liquidity_depth: MicroCoin,
> }
>
> pub struct BoltzmannMaskPolicy {
>     pub beta: f64,
>     pub min_liquidity: MicroCoin,
>     pub price_margin: Decimal,
>     pub epsilon_exploration: f64,
> }
> ```
>
> ## 3. Requirements
>
> ### Functional requirements
>
> ```text
> FR-14.1
> Compute price_yes = long / (long + short) when liquidity exists.
>
> FR-14.2
> Compute price_no = short / (long + short).
>
> FR-14.3
> Expose PriceIndex as broadcast/statistical signal.
>
> FR-14.4
> Boltzmann scheduler may mask parent only if child price sufficiently dominates.
>
> FR-14.5
> Masked parent remains in ChainTape and can be recovered.
> ```
>
> ### Constitutional requirements
>
> ```text
> CR-14.1
> Price cannot override predicates.
>
> CR-14.2
> Price cannot make failed proposal accepted.
>
> CR-14.3
> Masked means read-view/scheduler mask only, not deletion.
>
> CR-14.4
> Low-liquidity price cannot mask parent.
>
> CR-14.5
> Unresolved-challenged child cannot mask parent.
> ```
>
> ### Ship gates
>
> ```text
> SG-14.1
> PriceIndex computes expected YES/NO probabilities.
>
> SG-14.2
> No-liquidity node has price=None.
>
> SG-14.3
> Parent not deleted from ChainTape after masking.
>
> SG-14.4
> Predicate failure still dominates high price.
>
> SG-14.5
> Boltzmann selection includes epsilon exploration.
>
> SG-14.6
> Dashboard shows price as signal, not outcome.
> ```
>
> ## 4. 禁止事项
>
> ```text
> No market trading.
> No price-based settlement.
> No parent deletion.
> No Goodhart leak of private predicates.
> No masking unresolved-challenge nodes.
> ```
>
> ---
>
> # TB-15 — Lamarckian Autopsy + Markov EvidenceCapsule
>
> ## 1. 目标
>
> 把失败与爆仓转化为私有学习与下一轮 capsule，而不是全局日志污染。
>
> 这对齐无损宪法的 Flowchart 3：`logs archive as ground truth` 与 `constitution as ground truth` 共同构成元架构输入，但 Markov 规则要求默认只读最新 capsule，而不是全历史日志。
>
> ## 2. 新增对象
>
> ```rust
> pub struct AgentAutopsyCapsule {
>     pub capsule_id: Cid,
>     pub agent_id: AgentId,
>     pub event_id: EventId,
>     pub loss_amount: MicroCoin,
>     pub loss_reason_class: LossReasonClass,
>     pub violated_risk_rule: Option<RiskRuleId>,
>     pub suggested_policy_patch: Option<Cid>,
>     pub evidence_cids: Vec<Cid>,
>     pub public_summary: String,
>     pub private_detail_cid: Cid,
> }
>
> pub struct MarkovEvidenceCapsule {
>     pub capsule_id: Cid,
>     pub previous_capsule_cid: Option<Cid>,
>     pub constitution_hash: Hash,
>     pub l4_root: Hash,
>     pub l4e_root: Hash,
>     pub cas_root: Hash,
>     pub typical_errors: Vec<TypicalErrorSummary>,
>     pub unresolved_obs: Vec<ObsId>,
>     pub next_session_context_cid: Cid,
> }
> ```
>
> ## 3. Requirements
>
> ### Functional requirements
>
> ```text
> FR-15.1
> Loss / bankruptcy / failed market event creates AgentAutopsyCapsule.
>
> FR-15.2
> Autopsy uses ChainTape/CAS evidence, not self-narration.
>
> FR-15.3
> MarkovEvidenceCapsule generated at end of TB/run.
>
> FR-15.4
> Next InitAI context defaults to constitution + latest capsule.
>
> FR-15.5
> Markov override is required for deep history reads.
> ```
>
> ### Constitutional requirements
>
> ```text
> CR-15.1
> Raw failure logs are not broadcast globally.
>
> CR-15.2
> Autopsy is private/scoped unless error becomes typical.
>
> CR-15.3
> ArchitectAI may propose improvements from logs, but cannot mutate constitution.
>
> CR-15.4
> JudgeAI/VetoAI remains veto-only.
>
> CR-15.5
> Capsules are evidence compression, not hidden source of truth.
> ```
>
> ### Ship gates
>
> ```text
> SG-15.1
> Failed/losing agent gets private AutopsyCapsule.
>
> SG-15.2
> Raw private details do not enter other Agent read view.
>
> SG-15.3
> Latest Markov capsule can bootstrap next session.
>
> SG-15.4
> Deep-history read without override fails.
>
> SG-15.5
> Typical error broadcast uses summary, not raw log.
>
> SG-15.6
> Dashboard can regenerate capsule summary from ChainTape + CAS.
> ```
>
> ## 4. 禁止事项
>
> ```text
> No global raw autopsy broadcast.
> No forced prompt stuffing of all past failures.
> No automatic predicate mutation.
> No MetaTape self-modification.
> No constitution change.
> ```
>
> ---
>
> # TB-16 — Controlled Market Smoke Arena
>
> ## 1. 目标
>
> 在受控沙盒中跑通：
>
> ```text
> compute + position + price + masking + autopsy
> ```
>
> 但仍不开放真实市场。
>
> ## 2. 场景
>
> ```text
> Lean task
> multiple Agents
> WorkTx FirstLong
> ChallengeTx Short
> PriceIndex updates
> Boltzmann scheduler selects next candidate
> some agents lose positions
> Autopsy generated
> ```
>
> ## 3. Requirements
>
> ### Functional requirements
>
> ```text
> FR-16.1
> At least 3 agents participate.
>
> FR-16.2
> At least one WorkTx creates FirstLongPosition.
>
> FR-16.3
> At least one ChallengeTx creates ShortPosition.
>
> FR-16.4
> At least one price update occurs.
>
> FR-16.5
> At least one Boltzmann mask event occurs.
>
> FR-16.6
> At least one AutopsyCapsule is generated.
> ```
>
> ### Constitutional requirements
>
> ```text
> CR-16.1
> Total Coin conserved.
>
> CR-16.2
> No ghost liquidity.
>
> CR-16.3
> No price overriding predicates.
>
> CR-16.4
> No raw failure broadcast.
>
> CR-16.5
> No real user funds.
>
> CR-16.6
> All activity replayable from ChainTape + CAS.
> ```
>
> ### Ship gates
>
> ```text
> SG-16.1
> Controlled market smoke produces replayable ChainTape.
>
> SG-16.2
> Dashboard shows positions, prices, masks, autopsies.
>
> SG-16.3
> No fake accepted nodes.
>
> SG-16.4
> Unsolved tasks show failure evidence / bankruptcy anchors.
>
> SG-16.5
> All market balances conserved.
>
> SG-16.6
> No unresolved evidence gaps.
> ```
>
> ## 4. 禁止事项
>
> ```text
> No public chain.
> No real-money market.
> No external domain.
> No unbounded leverage.
> No AMM trading unless explicitly scoped.
> No DPMM / pro-rata.
> ```
>
> ---
>
> # TB-17 — Real-World Readiness Gate
>
> ## 1. 目标
>
> TB-17 不是做真实世界任务，而是判断 TuringOS 是否具备进入真实世界的条件。
>
> 真实世界问题往往不是 Lean Proof 这种 T2，而是 T3/T4：难解也难验，甚至外观欺骗。验证非对称性文章强调，T2 结构之所以适合，是因为生成难、验证相对客观；真实世界问题必须先被降维成可验证结构。
>
> ## 2. 新增文档
>
> ```text
> REAL_WORLD_READINESS_REPORT.md
> DOMAIN_SELECTION_CRITERIA.md
> ORACLE_REQUIREMENTS.md
> CHALLENGE_COURT_REQUIREMENTS.md
> SAFETY_BOUNDARY.md
> IRREVERSIBLE_ACTION_POLICY.md
> ```
>
> ## 3. Requirements
>
> ### Functional requirements
>
> ```text
> FR-17.1
> Define allowed real-world domain categories.
>
> FR-17.2
> Define oracle requirements for each category.
>
> FR-17.3
> Define challenge window and evidence requirements.
>
> FR-17.4
> Define human escalation conditions.
>
> FR-17.5
> Define delayed settlement requirements.
>
> FR-17.6
> Define irreversible-action ban.
> ```
>
> ### Constitutional requirements
>
> ```text
> CR-17.1
> No real-world domain without oracle.
>
> CR-17.2
> No subjective task without 0/1 predicate plan.
>
> CR-17.3
> No irreversible external action.
>
> CR-17.4
> No settlement before challenge window.
>
> CR-17.5
> No price-as-truth.
>
> CR-17.6
> No bypass of Human RootBox for high-risk domains.
> ```
>
> ### Ship gates
>
> ```text
> SG-17.1
> Real-world readiness report passes audit.
>
> SG-17.2
> At least 3 candidate domains classified.
>
> SG-17.3
> At least 1 low-risk pilot domain approved.
>
> SG-17.4
> Oracle design documented.
>
> SG-17.5
> ChallengeCourt design documented.
>
> SG-17.6
> Human escalation path documented.
>
> SG-17.7
> No production real-world task launched yet.
> ```
>
> ## 4. 禁止事项
>
> ```text
> No live external action.
> No real-world payout without oracle.
> No medical/legal/financial high-risk domain.
> No autonomous deployment.
> No public chain settlement.
> No agent-only arbitration.
> ```
>
> ---
>
> # 1. 更新后的路线总表
>
> | TB    | 名称                                     | 主目标                                                          | 不做什么          |
> | ----- | -------------------------------------- | ------------------------------------------------------------ | ------------- |
> | TB-11 | Epistemic Exhaust & Capital Liberation | EvidenceCapsule、RunExhaustedTx、TaskBankruptcyTx、TaskExpireTx | 不做 NodeMarket |
> | TB-12 | NodeMarket Position Index              | FirstLong / Short exposure                                   | 不做 trading    |
> | TB-13 | CompleteSet + MarketSeedTx             | 1 Coin = YES + NO，显式 seed                                    | 不做 AMM        |
> | TB-14 | PriceIndex + Boltzmann                 | 价格信号与 scheduler masking                                      | 不让价格替代谓词      |
> | TB-15 | Lamarckian Autopsy + Markov Capsule    | 私有尸检与 Markov 交接                                              | 不广播 raw logs  |
> | TB-16 | Controlled Market Smoke Arena          | 受控市场演习                                                       | 不接真实世界        |
> | TB-17 | Real-World Readiness Gate              | 真实世界准入标准                                                     | 不执行真实世界任务     |
>
> ---
>
> # 2. 最重要的重新排序原则
>
> ```text
> 先失败锚点，再市场做空。
> 先资本释放，再价格机制。
> 先证据胶囊，再尸检学习。
> 先受控市场，再真实世界。
> ```
>
> 也就是说：
>
> ```text
> TB-11 是 NodeMarket 的地基；
> 不是 NodeMarket 的替代品。
> ```
>
> ---
>
> # 3. 给 AI coder 的直接执行口令
>
> ```text
> Architect ruling:
>
> Update roadmap TB-11 through TB-17.
>
> TB-11 is no longer NodeMarket. TB-11 = Epistemic Exhaust & Capital Liberation.
>
> Reason:
> The Zeta pressure test exposed Invisible Graveyard:
> many failed attempts were evaluator-private, while ChainTape only saw task open / escrow lock.
> We must not pollute L4 with per-attempt debug logs, but we also cannot leave failures invisible.
>
> TB-11 requirements:
> 1. EvidenceCapsule in CAS for MAX_TX exhausted / timeout / solver give-up.
> 2. RunExhaustedTx accepted in L4, referencing evidence_capsule_cid.
> 3. TaskBankruptcyTx creates task-level failure anchor.
> 4. TaskExpireTx refunds escrow according to expiry policy.
> 5. Raw logs are shielded.
> 6. Dashboard renders exhausted / bankrupt / expired tasks.
> 7. No NodeMarket, no CompleteSet, no AMM, no Slash, no per-attempt L4 spam.
>
> TB-12:
> NodeMarket Position Index:
> WorkTx.stake -> FirstLongPosition.
> ChallengeTx.stake -> ShortPosition.
> VerifyTx.bond != market position.
> NodePosition not Coin holding.
>
> TB-13:
> CompleteSet + MarketSeedTx:
> 1 locked Coin = YES_E + NO_E.
> No ghost liquidity.
> No automatic YES/NO injection.
>
> TB-14:
> PriceIndex v0 + Boltzmann masking:
> Price is signal, not truth.
> Masking affects read view only.
> Parent nodes remain in ChainTape.
>
> TB-15:
> Lamarckian Autopsy + Markov EvidenceCapsule:
> Private autopsy, not global raw-log broadcast.
> Latest capsule + constitution is default InitAI context.
>
> TB-16:
> Controlled Market Smoke Arena:
> Compute + positions + price + mask + autopsy in sandbox.
> No real money / no public chain.
>
> TB-17:
> Real-World Readiness Gate:
> No real-world task until oracle, challenge, delayed settlement, human escalation, and irreversible-action ban are documented.
>
> After TB-17:
> Begin real-world domain selection and oracle design.
> ```
>
> ---
>
> # 4. 最终判断
>
> 这版路线比原版更强，因为它修正了一个核心顺序错误：
>
> 原版顺序：
>
> ```text
> NodeMarket -> CompleteSet -> Price -> Autopsy
> ```
>
> 新版顺序：
>
> ```text
> Failure Evidence -> Capital Release -> NodeMarket -> CompleteSet -> Price -> Autopsy -> Controlled Arena -> Real-World Gate
> ```
>
> 这个顺序更符合 TuringOS 的第一性原理：
>
> ```text
> 没有失败锚点，就没有可靠 short。
> 没有资本释放，就没有健康任务市场。
> 没有证据胶囊，就没有 Autopsy。
> 没有受控市场，就不能进入真实世界。
> ```
>
> 所以，从 TB-11 开始，项目应该先把"失败如何被埋葬、审计、释放资金"做对，再进入真正的 Polymarket/NodeMarket。

---

## §2 Reconciliation with prior 2026-05-02 ruling

This supplementary directive is **fully consistent** with the prior ruling at
`handover/directives/2026-05-02_TB11_EPISTEMIC_EXHAUST_ARCHITECT_RULING.md`.
The supplement adds:

1. **Numbered FR/CR/SG requirements** for each TB (FR-11.1..8 / CR-11.1..5 /
   SG-11.1..7 for TB-11; analogous for TB-12..17). The TB-11 charter MUST adopt
   this numbering.
2. **Renamed and re-ordered TB-12..17**:
   - Old AI-coder draft (`RULING_TB11_EPISTEMIC_EXHAUST_2026-05-02.md`):
     `TB-12 NodeMarket M0/M1 → TB-13 CompleteSet → TB-14 CPMM → TB-15 PriceIndex/Boltzmann → TB-16 Lamarckian Autopsy → TB-17 Pre-Real-World PCP Gate`
   - **Architect supplement (authoritative)**:
     `TB-12 NodeMarket Position Index → TB-13 CompleteSet + MarketSeedTx → TB-14 PriceIndex v0 + Boltzmann Masking → TB-15 Lamarckian Autopsy + Markov EvidenceCapsule → TB-16 Controlled Market Smoke Arena → TB-17 Real-World Readiness Gate`
   - Notable change: explicit AMM/CPMM is **NOT a separate TB** in the
     supplement — it folds into TB-14 PriceIndex (price is computed from
     long/short interest, no AMM router). AMM may surface later as a
     post-TB-17 product decision.
3. **Explicit struct definitions** for TB-12+ (NodePosition, CompleteSetMintTx,
   MarketSeedTx, NodeMarketEntry, BoltzmannMaskPolicy, AgentAutopsyCapsule,
   MarkovEvidenceCapsule). These are forward-binding for those TB charters.

## §3 Forward annotation (non-authoritative)

### TB-11 charter mapping

The TB-11 charter at `handover/tracer_bullets/TB-11_charter_2026-05-02.md`
must be amended (in same commit as this archive) to reference the
architect's FR/CR/SG numbering:

| Architect requirement | Charter location |
|---|---|
| FR-11.1 (capsule on exhaustion) | Atom 3 + Atom 4(b) |
| FR-11.2 (capsule contains counts) | Atom 1(b) EvidenceCapsule fields |
| FR-11.3 (RunExhaustedTx system-emitted accepted in L4) | Atom 1(a) TerminalSummary additive bump + Atom 2(a) dispatch |
| FR-11.4 (RunExhausted references capsule_cid) | Atom 1(a) `evidence_capsule_cid: Option<Cid>` |
| FR-11.5 (TaskBankruptcyTx future short anchor) | Atom 1(a) NEW variant + Atom 2(a) dispatch |
| FR-11.6 (TaskExpireTx refunds on policy met) | Atom 2(a) dispatch + Atom 4(c) tick |
| FR-11.7 (dashboard renders 3 states) | Atom 5 §12 |
| FR-11.8 (raw logs CAS-stored, shielded) | Atom 3 privacy_policy enum |
| CR-11.1..5 | Atom 7 recursive audit Clauses 1, 4 |
| SG-11.1..7 | Charter §6 ship gates G1..G11 superset (each SG-11.x maps to one G) |

### TB-12 → TB-17 charter pre-binding

When future sessions land those TBs, the architect's struct definitions are
the wire-format contract. AI coder MAY suggest schema additions but MUST
flag any deviation in the charter §7 open questions for explicit auto-ratify
or human verdict.

### Memory updates

`MEMORY.md` index entry for the roadmap should be updated to:

```
- [TB-11→TB-17 roadmap](project_tb11_to_tb17_roadmap.md) — **2026-05-02 architect supplement: TB-11 EpistemicExhaust → TB-12 NodeMarket Position Index → TB-13 CompleteSet+MarketSeed → TB-14 PriceIndex+Boltzmann → TB-15 Autopsy+Markov → TB-16 Controlled Arena → TB-17 RealWorld Gate. AMM/CPMM folded into TB-14, not separate TB.**
```

(new memory file; see Atom 0 section of TB-11 charter ratification).
