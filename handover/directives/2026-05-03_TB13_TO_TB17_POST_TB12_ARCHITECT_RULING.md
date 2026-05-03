# 2026-05-03 — TB-13 → TB-17 Post-TB-12 Architect Ruling (Lossless Archive)

**Source**: User-architect ultrathink directive delivered 2026-05-03 after TB-12 ship
(commit `fa36eca`).

**Status**: Lossless verbatim archive per `feedback_kolmogorov_compression`. The
original architect text is reproduced below in two parts:
- **Part A** — Audit + roadmap directive (markdown body of the directive).
- **Part B** — Companion roadmap file `TuringOS_TB13_to_TB17_Roadmap_After_TB12_2026-05-03.md`
  pasted at the tail of the directive.

Annotation layer (delta vs prior 2026-05-02 supplementary directive) is in §C
at the bottom; do NOT read annotation as substitute for the verbatim text.

---

## Part A — Architect ultrathink directive (verbatim)

According to a document from 2026-05-03, TB-12 的 shipped summary 显示它已按 NodeMarket Position Index 的窄化目标完成：`WorkTx.stake -> FirstLongPosition`、`ChallengeTx.stake -> ShortPosition`、`VerifyTx.bond != market position`、`NodePosition not Coin holding`，并且用户提供的结果显示 `cargo test --workspace = 759 / 0 / 150`、`8/8 SG-12.x`、`11/11 G-gates`、`14/14 forbidden tokens`、`9/9 immutable-record invariants` 均通过。这个阶段可以接受，但我没有在本轮直接读取到 GitHub commit `fa36eca` 的源码 diff；下面的审计以你提供的 TB-12 summary、已检索到的路线文档和前序宪法/白皮书约束为依据。

我也写了一份本地路线图文件：

[下载：TuringOS_TB13_to_TB17_Roadmap_After_TB12_2026-05-03.md](sandbox:/mnt/data/TuringOS_TB13_to_TB17_Roadmap_After_TB12_2026-05-03.md)

---

# 1. TB-12 阶段性审计结论

## 1.1 接受 TB-12 shipped

我接受 TB-12 的 shipped 状态，但只接受它的**窄化声明**：

```text
TB-12 proved:
  NodePosition exposure index exists.
  WorkTx.stake becomes FirstLongPosition.
  ChallengeTx.stake becomes ChallengeShortPosition.
  VerifyTx.bond is not a market position.
  NodePosition is not Coin holding.
```

TB-12 **没有证明**：

```text
CompleteSet
YES/NO share accounting
MarketSeedTx
PriceIndex
Boltzmann masking
market trading
AMM
CPMM
Polymarket settlement
real-world readiness
```

这点必须在 `LATEST.md` 和 TB_LOG 后续说明里继续保持窄化，避免团队把 TB-12 的“风险敞口索引”误认为“市场已经上线”。

---

## 1.2 TB-12 的关键正确点

你这次没有直接接受 Gemini PASS，而是做了 ship-gate exact naming 对齐，补了 SG-12.6 的真实测试，修正了 SG-12.5 / SG-12.7 / SG-12.8 的命名漂移。这是正确的。

因为在当前项目里，测试名不只是 cosmetic。它已经变成一种 traceability contract：

```text
架构师要求 SG-12.5；
代码里必须有对应 gate；
审计时必须能一眼对应。
```

所以：

```text
“架构正确性 PASS”
不能替代
“指定 ship gate exact-name PASS”
```

你让 AI coder 在 ship 前修掉这些 gap，是对的。

---

## 1.3 需要 carry forward 的 TB-12 风险

TB-12 后续最重要的风险是：**NodePosition 会在后续被误用为 money ledger 或 tradable share balance。**

必须继续写入后续 charter：

```text
NodePosition = immutable exposure record / index
NodePosition != Coin
NodePosition != YES/NO share
NodePosition != tradable balance
NodePosition != LP share
NodePosition.amount not counted in total_supply_micro
```

TB-12 的路线文档本来也明确要求 `NodePosition.amount must not be included in total_supply_micro`，并且 Price / position signal 不能成为 truth。

这条要在 TB-13/TB-14 反复保护，否则 CompleteSet 一接入，很容易出现“双账本”：

```text
stake balance
node position
YES/NO shares
collateral
```

四者不能混。

---

# 2. 总体路线是否要调整？

**核心路线不需要推翻，但 TB-13 必须加一个前置隔离步骤。**

你提供的 TB-12 summary 里已经指出：

```text
TB-13 prerequisite:
  隔离 src/prediction_market.rs legacy f64 CPMM
  已在 OBS_TB_12_LEGACY_CPMM_QUARANTINE 跟踪
```

这是非常重要的。
如果 TB-13 一边引入 `CompleteSet + MarketSeedTx`，一边仓库里还存在旧的 `f64 CPMM` 可被 import，那么未来很容易 drift 回：

```text
f64 price
ghost liquidity
automatic injection
AMM before collateral accounting
```

所以我的第一个路线修正是：

> **TB-13 的 Atom 0.5 必须是 legacy CPMM quarantine。**

不是“顺手做”，而是 TB-13 的前置 ship gate。

---

# 3. 更新后的 TB-13 到 TB-17 总表

```text
TB-13  CompleteSet + MarketSeedTx
       先隔离 legacy f64 CPMM；引入 1 locked Coin = YES_E + NO_E；不做 AMM。

TB-14  PriceIndex v0 + Boltzmann Masking
       价格是统计信号，不是真理；mask 只影响 read view / scheduler。

TB-15  Lamarckian Autopsy + Markov EvidenceCapsule
       私有尸检 + Markov capsule，不广播 raw logs，不自动改谓词。

TB-16  Controlled Market Smoke Arena
       在沙盒里跑 compute + positions + price + mask + autopsy，不接真实资金/真实世界。

TB-17  Real-World Readiness Gate
       只做真实世界准入标准，不执行真实世界任务。
```

这个顺序仍然符合之前确定的原则：

```text
先失败锚点，再市场做空。
先资本释放，再价格机制。
先证据胶囊，再尸检学习。
先受控市场，再真实世界。
```

这也是你前一版路线中明确写出的重排原则。

---

# 4. TB-13 — CompleteSet + MarketSeedTx

## 4.1 目标

引入 Polymarket / CTF 的数学核心：

```text
1 locked Coin = 1 YES_E + 1 NO_E
```

但 TB-13 **仍然不做**：

```text
AMM
CPMM router
orderbook
MarketOrderTx
MarketTradeTx
PriceIndex
DPMM / pro-rata
automatic liquidity
```

TB-13 只做抵押与份额会计。

---

## 4.2 Atom 0.5：legacy CPMM quarantine

这是 TB-13 的第一步。

### Requirements

```text
FR-13.0.1
src/prediction_market.rs legacy f64 CPMM must be quarantined.

FR-13.0.2
New CompleteSet / MarketSeedTx code must not import legacy prediction_market.rs.

FR-13.0.3
No f64 in CompleteSet / MarketSeedTx / market accounting path.

FR-13.0.4
Legacy CPMM must be clearly labeled:
  legacy
  not used by RSP-M
  not constitutional
  not production market path.
```

### Ship gates

```text
SG-13.0.1
legacy_cpm_api_not_imported_by_complete_set passes.

SG-13.0.2
no_f64_in_complete_set_or_market_seed passes.

SG-13.0.3
prediction_market_legacy_quarantined passes.

SG-13.0.4
OBS_TB_12_LEGACY_CPMM_QUARANTINE either closed or explicitly carried as non-importable legacy.
```

### Halting triggers

```text
HALT if new TB-13 code imports legacy prediction_market.rs.

HALT if f64 appears in new CompleteSet / MarketSeed code.

HALT if any AMM / CPMM router function is introduced in TB-13.
```

---

## 4.3 新增对象

```rust
pub struct EventId(...);

pub enum OutcomeSide {
    Yes,
    No,
}

pub struct ShareAmount {
    pub units: i128,
}

pub struct ConditionalCollateralIndex(
    pub BTreeMap<EventId, MicroCoin>
);

pub struct ConditionalShareBalances(
    pub BTreeMap<(AgentId, EventId, OutcomeSide), ShareAmount>
);

pub struct CompleteSetMintTx {
    pub tx_id: TxId,
    pub parent_state_root: Hash,
    pub event_id: EventId,
    pub owner: AgentId,
    pub amount: MicroCoin,
    pub signature: AgentSignature,
}

pub struct CompleteSetRedeemTx {
    pub tx_id: TxId,
    pub parent_state_root: Hash,
    pub event_id: EventId,
    pub owner: AgentId,
    pub outcome: OutcomeSide,
    pub share_amount: ShareAmount,
    pub signature_or_system_resolution_ref: ResolutionRef,
}

pub struct MarketSeedTx {
    pub tx_id: TxId,
    pub parent_state_root: Hash,
    pub event_id: EventId,
    pub provider: AgentId,
    pub collateral_amount: MicroCoin,
    pub signature: AgentSignature,
}
```

---

## 4.4 Functional requirements

```text
FR-13.1
CompleteSetMintTx debits balances_t by amount.

FR-13.2
CompleteSetMintTx credits conditional_collateral_t by amount.

FR-13.3
CompleteSetMintTx issues equal YES_E and NO_E shares.

FR-13.4
CompleteSetRedeemTx is impossible before system-resolved outcome.

FR-13.5
CompleteSetRedeemTx after YES outcome pays YES shares and not NO shares.

FR-13.6
MarketSeedTx uses explicit provider funds.

FR-13.7
MarketSeedTx may prepare collateralized YES/NO inventory for future market layers, but cannot quote, trade, or price.
```

---

## 4.5 Constitutional requirements

```text
CR-13.1
No ghost liquidity.

CR-13.2
No automatic YES/NO injection.

CR-13.3
YES/NO shares are claims, not Coin.

CR-13.4
Locked collateral is Coin holding; shares are not.

CR-13.5
on_init remains the only base Coin mint.

CR-13.6
Price / share state cannot override predicates or challenge outcome.
```

---

## 4.6 Ship gates

```text
SG-13.1
Mint 1 Coin -> 1 YES + 1 NO, total Coin conserved.

SG-13.2
YES/NO shares are not counted in total Coin supply.

SG-13.3
MarketSeedTx fails if provider lacks balance.

SG-13.4
MarketSeedTx cannot create liquidity without collateral.

SG-13.5
Redeem unavailable before outcome resolution.

SG-13.6
Redeem after YES outcome pays YES, not NO.

SG-13.7
No f64 in new CompleteSet / MarketSeed path.

SG-13.8
No import/use of legacy CPMM in TB-13 modules.
```

---

## 4.7 Forbidden

```text
No automatic per-node 100 YES + 100 NO.
No Treasury magic seed unless Treasury balance is debited.
No DPMM / pro-rata payout.
No AMM.
No CLOB / orderbook.
No MarketOrderTx.
No MarketTradeTx.
No price oracle.
No NodeMarketEntry canonical state.
No f64.
```

---

## 4.8 Loop-mode instruction

AI coder may run TB-13 atoms autonomously until pre-ship audit.

Must halt immediately if:

```text
total_supply_micro changes incorrectly;
shares are counted as Coin;
MarketSeedTx succeeds without balance debit;
legacy CPMM import appears;
f64 appears in new market modules;
any price / AMM / trade logic appears.
```

Risk class:

```text
Class 3
```

Reason:

```text
CompleteSet and MarketSeedTx are money/collateral surfaces.
```

---

# 5. TB-14 — PriceIndex v0 + Boltzmann Masking

## 5.1 目标

将 TB-12 的 `NodePosition` 和 TB-13 的 share/collateral state 转化为价格信号，并用于 read-view / scheduler masking。

核心原则：

```text
Price is signal, not truth.
```

这与宪法中的信号管理一致。宪法把顶层白盒的工作定义为量化、广播、屏蔽，并把统计信号用于衡量相对有效性；但布尔谓词仍然确立绝对边界。

---

## 5.2 新增对象

```rust
pub struct RationalPrice {
    pub numerator: u128,
    pub denominator: u128,
}

pub struct NodeMarketEntry {
    pub node_id: TxId,
    pub task_id: TaskId,
    pub event_id: EventId,
    pub long_interest: MicroCoin,
    pub short_interest: MicroCoin,
    pub yes_share_depth: ShareAmount,
    pub no_share_depth: ShareAmount,
    pub price_yes: Option<RationalPrice>,
    pub price_no: Option<RationalPrice>,
    pub liquidity_depth: MicroCoin,
}

pub struct BoltzmannMaskPolicy {
    pub beta_num: i64,
    pub beta_den: i64,
    pub min_liquidity: MicroCoin,
    pub price_margin: RationalPrice,
    pub epsilon_exploration_num: u64,
    pub epsilon_exploration_den: u64,
}
```

---

## 5.3 Functional requirements

```text
FR-14.1
Compute price_yes = long / (long + short) when exposure liquidity exists.

FR-14.2
Compute price_no = short / (long + short).

FR-14.3
If no long/short liquidity exists, price=None.

FR-14.4
Expose PriceIndex as read-only broadcast/statistical signal.

FR-14.5
Boltzmann scheduler may mask parent only if child price sufficiently dominates.

FR-14.6
Masked parent remains in ChainTape and can be recovered.
```

---

## 5.4 Constitutional requirements

```text
CR-14.1
Price cannot override predicates.

CR-14.2
Price cannot make failed proposal accepted.

CR-14.3
Masked means read-view/scheduler mask only, not deletion.

CR-14.4
Low-liquidity price cannot mask parent.

CR-14.5
Unresolved-challenged child cannot mask parent.

CR-14.6
Goodhart-sensitive private predicates remain hidden.
```

---

## 5.5 Ship gates

```text
SG-14.1
PriceIndex computes expected YES/NO probabilities.

SG-14.2
No-liquidity node has price=None.

SG-14.3
Parent not deleted from ChainTape after masking.

SG-14.4
Predicate failure still dominates high price.

SG-14.5
Boltzmann selection includes epsilon exploration.

SG-14.6
Dashboard shows price as signal, not outcome.

SG-14.7
Unresolved challenge blocks masking.

SG-14.8
Low-liquidity manipulation cannot mask parent.
```

---

## 5.6 Forbidden

```text
No market trading.
No price-based settlement.
No parent deletion.
No Goodhart leak of private predicates.
No masking unresolved-challenge nodes.
No f64.
No AMM.
No DPMM.
No price-as-oracle language.
```

---

## 5.7 Loop-mode instruction

AI coder can run implementation autonomously until pre-ship audit.

Halt if:

```text
price affects predicate result;
price changes L4/L4.E decision;
parent node deleted/removed from ChainTape;
f64 introduced;
price computed for zero-liquidity node as non-None;
unresolved challenge is masked as safe.
```

Risk class:

```text
Class 2
```

If Boltzmann masking changes actual Agent read view in production, promote to:

```text
Class 3
```

---

# 6. TB-15 — Lamarckian Autopsy + Markov EvidenceCapsule

## 6.1 目标

把失败、爆仓、亏损、反复错误转化为私有学习与 Markov capsule，而不是全局 raw-log 污染。

这对齐无损宪法的 Flowchart 3：`logs archive as ground truth` 和 `constitution as ground truth` 共同构成元架构输入，但 Markov 规则要求默认只读最新 capsule，而不是全历史日志。

---

## 6.2 新增对象

```rust
pub struct AgentAutopsyCapsule {
    pub capsule_id: Cid,
    pub agent_id: AgentId,
    pub event_id: EventId,
    pub loss_amount: MicroCoin,
    pub loss_reason_class: LossReasonClass,
    pub violated_risk_rule: Option<RiskRuleId>,
    pub suggested_policy_patch: Option<Cid>,
    pub evidence_cids: Vec<Cid>,
    pub public_summary: String,
    pub private_detail_cid: Cid,
}

pub struct MarkovEvidenceCapsule {
    pub capsule_id: Cid,
    pub previous_capsule_cid: Option<Cid>,
    pub constitution_hash: Hash,
    pub l4_root: Hash,
    pub l4e_root: Hash,
    pub cas_root: Hash,
    pub typical_errors: Vec<TypicalErrorSummary>,
    pub unresolved_obs: Vec<ObsId>,
    pub next_session_context_cid: Cid,
}
```

---

## 6.3 Functional requirements

```text
FR-15.1
Loss / bankruptcy / failed market event creates AgentAutopsyCapsule.

FR-15.2
Autopsy uses ChainTape/CAS evidence, not self-narration.

FR-15.3
MarkovEvidenceCapsule generated at end of TB/run.

FR-15.4
Next InitAI context defaults to constitution + latest capsule.

FR-15.5
Markov override is required for deep history reads.

FR-15.6
Public summary can broadcast typical error; private detail remains scoped.
```

---

## 6.4 Constitutional requirements

```text
CR-15.1
Raw failure logs are not broadcast globally.

CR-15.2
Autopsy is private/scoped unless error becomes typical.

CR-15.3
ArchitectAI may propose improvements from logs, but cannot mutate constitution.

CR-15.4
JudgeAI/VetoAI remains veto-only.

CR-15.5
Capsules are evidence compression, not hidden source of truth.

CR-15.6
Markov default prevents context poisoning.
```

---

## 6.5 Ship gates

```text
SG-15.1
Failed/losing agent gets private AutopsyCapsule.

SG-15.2
Raw private details do not enter other Agent read view.

SG-15.3
Latest Markov capsule can bootstrap next session.

SG-15.4
Deep-history read without override fails.

SG-15.5
Typical error broadcast uses summary, not raw log.

SG-15.6
Dashboard can regenerate capsule summary from ChainTape + CAS.

SG-15.7
Markov capsule references constitution hash and flowchart hashes.

SG-15.8
Autopsy does not mutate predicates/tools automatically.
```

---

## 6.6 Forbidden

```text
No global raw autopsy broadcast.
No forced prompt stuffing of all past failures.
No automatic predicate mutation.
No MetaTape self-modification.
No constitution change.
No hidden source-of-truth capsule.
No private loss detail in public read view.
```

---

## 6.7 Loop-mode instruction

Class:

```text
Class 2
```

Promote to Class 3 if it modifies Agent read-view authorization.

Halt if:

```text
raw logs leak to general read view;
capsule becomes hidden source of truth;
autopsy mutates policy automatically;
Markov override bypasses audit.
```

---

# 7. TB-16 — Controlled Market Smoke Arena

## 7.1 目标

在受控沙盒中跑通：

```text
compute + position + complete set + price + mask + autopsy
```

仍不开放真实市场。

---

## 7.2 Scenario

```text
Lean task
multiple Agents
WorkTx FirstLong
ChallengeTx Short
CompleteSet share inventory
PriceIndex updates
Boltzmann scheduler selects next candidate
some agents lose positions
Autopsy generated
```

---

## 7.3 Functional requirements

```text
FR-16.1
At least 3 agents participate.

FR-16.2
At least one WorkTx creates FirstLongPosition.

FR-16.3
At least one ChallengeTx creates ShortPosition.

FR-16.4
At least one CompleteSetMintTx exists.

FR-16.5
At least one price update occurs.

FR-16.6
At least one Boltzmann mask event occurs.

FR-16.7
At least one AutopsyCapsule is generated.
```

---

## 7.4 Constitutional requirements

```text
CR-16.1
Total Coin conserved.

CR-16.2
No ghost liquidity.

CR-16.3
No price overriding predicates.

CR-16.4
No raw failure broadcast.

CR-16.5
No real user funds.

CR-16.6
All activity replayable from ChainTape + CAS.

CR-16.7
All market activity is sandbox-labeled.
```

---

## 7.5 Ship gates

```text
SG-16.1
Controlled market smoke produces replayable ChainTape.

SG-16.2
Dashboard shows positions, prices, masks, autopsies.

SG-16.3
No fake accepted nodes.

SG-16.4
Unsolved tasks show failure evidence / bankruptcy anchors.

SG-16.5
All market balances conserved.

SG-16.6
No unresolved evidence gaps.

SG-16.7
At least one loss -> autopsy path.

SG-16.8
Sandbox flag prevents real-money interpretation.
```

---

## 7.6 Forbidden

```text
No public chain.
No real-money market.
No external domain.
No unbounded leverage.
No AMM trading unless explicitly scoped.
No DPMM / pro-rata.
No medical/legal/financial domains.
No production user funds.
```

---

## 7.7 Loop-mode instruction

Risk class:

```text
Class 3 integration smoke
```

AI coder may implement autonomously, but ship requires external audit.

Halt if:

```text
any conservation failure;
raw log leak;
price-as-truth behavior;
non-sandbox funds used;
unresolved evidence gap.
```

---

# 8. TB-17 — Real-World Readiness Gate

## 8.1 目标

TB-17 不执行真实世界任务，只判断是否具备进入真实世界的条件。

真实世界问题往往不是 Lean Proof 这种 T2，而是 T3/T4：难解也难验，甚至外观欺骗。验证非对称性文档强调，Lean Proof 这种“难生成、易验证”结构之所以适合，是因为可以利用验证非对称性监管黑盒。

---

## 8.2 新增文档

```text
REAL_WORLD_READINESS_REPORT.md
DOMAIN_SELECTION_CRITERIA.md
ORACLE_REQUIREMENTS.md
CHALLENGE_COURT_REQUIREMENTS.md
SAFETY_BOUNDARY.md
IRREVERSIBLE_ACTION_POLICY.md
```

---

## 8.3 Functional requirements

```text
FR-17.1
Define allowed real-world domain categories.

FR-17.2
Define oracle requirements for each category.

FR-17.3
Define challenge window and evidence requirements.

FR-17.4
Define human escalation conditions.

FR-17.5
Define delayed settlement requirements.

FR-17.6
Define irreversible-action ban.

FR-17.7
Define domain risk tiers T1/T2/T3/T4-style.
```

---

## 8.4 Constitutional requirements

```text
CR-17.1
No real-world domain without oracle.

CR-17.2
No subjective task without 0/1 predicate plan.

CR-17.3
No irreversible external action.

CR-17.4
No settlement before challenge window.

CR-17.5
No price-as-truth.

CR-17.6
No bypass of Human RootBox for high-risk domains.

CR-17.7
No agent-only arbitration.
```

---

## 8.5 Ship gates

```text
SG-17.1
Real-world readiness report passes audit.

SG-17.2
At least 3 candidate domains classified.

SG-17.3
At least 1 low-risk pilot domain approved.

SG-17.4
Oracle design documented.

SG-17.5
ChallengeCourt design documented.

SG-17.6
Human escalation path documented.

SG-17.7
No production real-world task launched yet.

SG-17.8
Irreversible action policy tested against examples.
```

---

## 8.6 Forbidden

```text
No live external action.
No real-world payout without oracle.
No medical/legal/financial high-risk domain.
No autonomous deployment.
No public chain settlement.
No agent-only arbitration.
No irreversible external actuation.
No real-world pilot before report approval.
```

---

## 8.7 Loop-mode instruction

Risk class:

```text
Class 0 / Class 4 hybrid
```

AI coder can draft documents autonomously, but cannot ratify.

Required:

```text
Human architect sign-off.
```

---

# 9. 全局防漂移规则：TB-13 到 TB-17 必须继承

每个 TB 必须声明：

```text
phase_id
roadmap_exit_criteria_addressed
kill_criteria_tested
flowchart_trace
risk_class
forbidden list
```

每个经济 TB 必须证明：

```text
no ghost liquidity
no total supply mutation
no f64 in money path
no agent-submitted system tx
dashboard is materialized view only
```

每个市场信号必须写明：

```text
price is signal, not truth
```

每个失败任务必须至少满足一个：

```text
L4.E rejection evidence
或
EvidenceCapsule + L4 anchor
```

每个真实世界计划必须：

```text
pass TB-17 first
```

---

# 10. 建议更新后的总路线表

| TB    | 名称                                  | 主目标                              | 绝对不做         |
| ----- | ----------------------------------- | -------------------------------- | ------------ |
| TB-12 | NodeMarket Position Index           | FirstLong / Short exposure       | 不做 trading   |
| TB-13 | CompleteSet + MarketSeedTx          | 1 locked Coin = YES + NO，显式 seed | 不做 AMM       |
| TB-14 | PriceIndex + Boltzmann              | 价格信号与 scheduler masking          | 不让价格替代谓词     |
| TB-15 | Lamarckian Autopsy + Markov Capsule | 私有尸检与 Markov 交接                  | 不广播 raw logs |
| TB-16 | Controlled Market Smoke Arena       | 受控市场演习                           | 不接真实世界       |
| TB-17 | Real-World Readiness Gate           | 真实世界准入标准                         | 不执行真实世界任务    |

TB-12 已完成，因此下一步直接从 TB-13 开始，但请把 TB-13 的 Atom 0.5 设为 **legacy CPMM quarantine**。

---

# 11. 给 AI coder 的 loop-mode 总指令

可以直接发：

```text
Architect ruling after TB-12 ship:

TB-12 accepted as shipped, but only under narrowed claim:
NodePosition exposure index, no trading, no CompleteSet, no price, no AMM.

Proceed to TB-13 with updated roadmap.

TB-13:
CompleteSet + MarketSeedTx.
First atom MUST quarantine legacy f64 CPMM in src/prediction_market.rs.
No f64 in new market modules.
No automatic YES/NO injection.
No ghost liquidity.
No AMM.
No trading.

TB-14:
PriceIndex v0 + Boltzmann.
Price is statistical signal, not truth.
Masking affects read view/scheduler only.
Parent nodes remain in ChainTape.

TB-15:
Lamarckian Autopsy + Markov EvidenceCapsule.
Private/scoped autopsy.
Latest capsule + constitution is default InitAI context.
No raw global broadcast.

TB-16:
Controlled Market Smoke Arena.
Sandbox only.
No real funds.
No public chain.
No external domain.

TB-17:
Real-World Readiness Gate.
Documents only.
No real-world task launched.
Human architect sign-off required.

For every TB:
declare phase_id, roadmap_exit_criteria_addressed, kill_criteria_tested, flowchart_trace, risk_class, forbidden list.
Run cargo test --workspace.
Run risk-class audit.
Stop on conservation failure, f64 in money path, ghost liquidity, price-as-truth, or dashboard-as-source-of-truth.
```

---

# 12. 最终判断

TB-12 的完成是一个重要里程碑：它把“Agent 对 node 承担的经济方向”从口号变成了 `NodePosition` 结构。

但现在还不能进入交易。
后续必须严格按顺序来：

```text
CompleteSet collateral
-> Price signal
-> Autopsy/Markov learning
-> Controlled market smoke
-> Real-world readiness
```

尤其 TB-13 要守住一个底线：

> **YES/NO 是 claim，不是 Coin；MarketSeed 是显式抵押，不是自动注入；legacy f64 CPMM 不能进入新经济核心。**

只要这条线守住，TB-13 到 TB-17 可以让 AI coder 进入较高自治度的 `/loop` 模式推进。

---

## Part B — Companion roadmap file (verbatim)

(Begins after the directive's `;` separator; archived because it reproduces and
slightly extends Part A. Where Part B contradicts Part A, Part A wins per
architect ordering.)

TuringOS TB-12 审计与 TB-13→TB-17 执行路线图

0. 当前裁决
TB-12 可以作为 NodeMarket Position Index 阶段通过，但它的 ship claim 必须保持窄化：
它证明的是 exposure index；
它没有证明 trading；
它没有证明 CompleteSet；
它没有证明 PriceIndex；
它没有证明市场结算；
它没有证明真实世界 readiness。
TB-13 到 TB-17 的主线不需要推翻，但需要加硬：
TB-13 必须先隔离 legacy f64 CPMM。
TB-13 只做 CompleteSet + collateral-backed MarketSeedTx，不做 AMM。
TB-14 的 PriceIndex 只能是统计信号/只读物化视图，不是 truth。
TB-15 必须把 Autopsy 与 MarkovEvidenceCapsule 区分。
TB-16 必须是 sandbox controlled market，不接真实资金/公链/真实世界。
TB-17 必须是 readiness gate，不执行真实世界任务。

1. TB-12 审计结论
1.1 接受 TB-12 shipped
根据用户提供的 shipped summary：
ship commit: fa36eca；
cargo test --workspace = 759 / 0 / 150；
8/8 SG-12.x exact-name ship gates PASS；
11/11 G-gates PASS；
14/14 forbidden tokens PASS；
9/9 immutable-record invariants PASS；
WorkTx.stake -> FirstLongPosition；
ChallengeTx.stake -> ChallengeShortPosition；
VerifyTx.bond != market position；
NodePosition not Coin holding；
no CompleteSet / no trading / no AMM / no automatic liquidity / no price-based settlement。
阶段性结论：TB-12 的方向和执行结果都可以接受。

1.2 TB-12 仍需 carry-forward 的风险
必须在 TB-13/TB-14 之前确认：
NodePosition 是否从 total_supply_micro 中永久排除；
NodePositionsIndex 是否只作为 exposure index，而非 share balance；
ChallengeShort 是否能引用 TaskBankruptcyTx / RunExhaustedTx；
Dashboard §13 是否是只读物化视图；
src/prediction_market.rs legacy f64 CPMM 是否彻底隔离；
没有任何 hidden import path 让 legacy CPMM 进入 TB-13 新市场逻辑。

2. TB-13 — CompleteSet + MarketSeedTx
[Part B's TB-13 section restates Part A §4 — see Part A for canonical text.]

[The remainder of Part B — sections 2.1–8.7 of the companion roadmap — restates
TB-13 through TB-17 in slightly compressed form (FR-13.x / CR-13.x / SG-13.x and
parallel families for TB-14 / TB-15 / TB-16 / TB-17). Part A is the canonical
reading; Part B is preserved here only for round-trip traceability of the
original sandbox attachment.]

***7. Global anti-drift gates for TB-13→TB-17
Every TB must declare:
phase_id
roadmap_exit_criteria_addressed
kill_criteria_tested
flowchart_trace
risk_class
forbidden list

Every TB must run:
cargo test --workspace
recursive self-audit
risk-class-appropriate external audit

Every economic TB must prove:
no ghost liquidity
no total supply mutation
no f64 in money path
no agent-submitted system tx

Every dashboard claim must be:
materialized view only
regeneratable from ChainTape + CAS

Every market signal must state:
price is signal, not truth

Every failed task must either:
produce L4.E rejection evidence, or
produce EvidenceCapsule + L4 anchor

Every future real-world plan must:
pass TB-17 first.

***8. AI Coder loop-mode master instruction
Proceed TB-by-TB. For each TB:
Write charter.
Run self-check against this roadmap.
Implement atoms.
After each atom:
cargo test --workspace
update capsule/handover
Before ship:
recursive self-audit
risk-class audit
Stop for user review at ship gate unless the TB is Class 0/1 and pre-authorized.
Never implement a future TB feature early.
Never weaken forbidden list.
Never treat price as predicate.
Never create liquidity without collateral.
Never let dashboard become source of truth.

---

## Part C — Annotation layer (NOT canonical; for context only)

### Delta vs prior architect rulings

| Topic | Prior (2026-05-02 supplementary) | This ruling (2026-05-03 post-TB-12) |
| ----- | -------------------------------- | ----------------------------------- |
| TB-12 status | Charter defined | **SHIPPED accepted (narrowed claim only)** |
| TB-13 prereq | OBS-tracked legacy CPMM | **Atom 0.5 = MANDATORY pre-ship gate** for TB-13 |
| `NodeMarketEntry` | Listed in roadmap as TB-14 derived view | Re-affirmed: NOT canonical state, derived view only |
| AMM/CPMM router | Forbidden in TB-13 | Forbidden through TB-15; first allowed scope is TB-16 sandbox arena (and only "AMM trading unless explicitly scoped") |
| Ship-gate exact-name discipline | Implicit | **Explicit: test name = traceability contract** (architect §1.2) |
| TB-15 rename | "Lamarckian Autopsy + Markov EvidenceCapsule" | Confirmed; capsule type split into `AgentAutopsyCapsule` (private/scoped) vs `MarkovEvidenceCapsule` (session bootstrap) |
| TB-17 sign-off | Class 0/4 hybrid implied | **Explicit: Human architect sign-off required**; AI coder may draft, cannot ratify |

### Layer 1 invariant impact check

Per `architect-ingest` §2:

- **kernel.rs zero domain knowledge**: ✅ unchanged — TB-13 lives in `src/economy/` + new `src/state/` typed-tx surface; no kernel.rs domain-knowledge regression.
- **Append-Only DAG**: ✅ unchanged — `CompleteSetMintTx` / `CompleteSetRedeemTx` / `MarketSeedTx` are append-only typed-tx; `ConditionalCollateralIndex` + `ConditionalShareBalances` are derived QState projections.
- **Economic conservation**: ✅ explicit reinforcement — CR-13.1..6 + SG-13.1 (1 Coin → 1 YES + 1 NO, total Coin conserved) + SG-13.2 (shares not in total Coin supply).

No Layer 1 violation. Authorization-gate check passed; awaiting explicit user execute order before drafting TB-13 charter.

### Memory cross-references touched

- `feedback_kolmogorov_compression` — applied; original directive archived verbatim above.
- `feedback_tb_phase_tag_required` — TB-13 charter MUST declare `phase_id` (P5 MetaTape primary per 9-phase roadmap; TB-12 was P3+P4 carry-forward; TB-13 introduces conditional-share core which is the substrate of MetaTape's "compressed evidence loom" — needs ratification at charter-time, not assumption).
- `feedback_no_retroactive_evidence_rewrite` — directive's "Atom 0.5 legacy CPMM quarantine" is forward-binding; existing pre-2026-05 BinaryMarket scaffolding at `src/prediction_market.rs` does NOT need its history rewritten, just its forward import surface closed.
- `feedback_smoke_before_batch` — every atom transition must run `experiments/minif2f_v4/src/bin/oracle.rs --smoke` before larger validation.
- `feedback_workspace_test_canonical` — `cargo test --workspace` is the canonical TB-13 ship-gate test count.
- `feedback_dual_audit` — TB-13 declared Class 3 by architect (§4.8); requires Codex + Gemini hybrid full dual audit at ship.
- `feedback_iteration_cap_24h` — TB-13 is production wire-up (money/collateral surface) → 72h-to-feedback-loop exception applies; mandatory escalation if slipped.
- `feedback_session_label_codification` — Atom 0.5 = legacy CPMM quarantine label introduced in this directive; codified in this archive + TB-13 charter.
- `feedback_step_b_protocol` — `src/state/sequencer.rs` will need extension for new dispatch arms; STEP_B_PROTOCOL parallel-branch A/B applies.
- `feedback_no_fake_menus` — when TB-13 atom sequencing is determinate, state and execute, do not surface options.
- `feedback_chaintape_externalized_proposal` — `CompleteSetMintTx` is a single LLM-or-system-emitted external tx; not per-tactic.

### Authorization status

**NOT YET AUTHORIZED to execute**. Per `architect-ingest` §4, present analysis,
await explicit user approval before:
1. Writing TB-13 charter to `handover/tracer_bullets/TB-13_charter_2026-05-03.md`.
2. Closing or carrying-forward `OBS_TB_12_LEGACY_CPMM_QUARANTINE`.
3. Modifying `MEMORY.md` with TB-13 reference entry.
4. Drafting TB-14 / TB-15 / TB-16 / TB-17 stub charters.

The user-architect's directive concludes with "TB-12 已完成，因此下一步直接从 TB-13
开始" + the §11 loop-mode master instruction, which **reads as authorization** to
proceed, but explicit confirmation requested per the no-fake-menus / explicit-go
discipline.
