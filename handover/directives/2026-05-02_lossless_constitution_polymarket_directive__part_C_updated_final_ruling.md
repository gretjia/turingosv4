# Part C — Updated Final Ruling (verbatim)

**Source**: user message 2026-05-02. Per user's explicit instruction "以最后的为准" (the last takes precedence), this Part C supersedes Part B.
**Status**: VERBATIM PRESERVATION (no compression, no paraphrase). Per `feedback_kolmogorov_compression`.
**Companion files**:
- `2026-05-02_lossless_constitution_polymarket_directive.md` (overview + Part A new content)
- `2026-05-02_lossless_constitution_polymarket_directive__part_B_first_plan.md` (superseded plan)

---

## Verbatim text begins

我读完你补充的工作笔记和 Gemini DeepThink 的评价后，给出一个**更新后的最终裁决**：

> **可以吸收 TuringOS-Polymarket 设计，但不能把"每个新节点自动注入 100 YES/NO"作为宪法基本法吸收。**
> 它必须被改写成：**由显式 Treasury / LP 资金支持的 LiquiditySeedTx**，并且只能进入 NodeMarket 阶段，不能进入 v1.0 Lean Proof Task Market 的上线前置路径。

这不是语义洁癖，而是宪法底线。宪法的基本法已经明确是：

```text
Information is Free
Only Investment Costs Money
1 Coin = 1 YES + 1 NO
on_init 是唯一合法铸币点
```

如果"每个新节点自动注入 YES/NO"没有显式资金来源，它就是 ghost liquidity / post-init mint；如果它来自 Treasury 或 LP 显式扣款，它才是合宪的做市资金。

---

# 1. 先校准 Polymarket 的真实数学核心

你给的工作笔记把两个机制混在了一起：

```text
A. Polymarket / CTF 的条件代币核心
B. Uniswap / CPMM 的恒定乘积 AMM 做市
```

这两者都重要，但不是同一个层级。

Polymarket 的核心是 Conditional Token Framework：一个二元市场有 YES/NO 两种 outcome tokens，并且每一对 YES/NO 都由 1 单位抵押资产完全支持；核心操作是 split、merge、redeem。官方文档也明确说，YES/NO token 始终 fully collateralized，每一对 YES/NO 由锁定抵押支持。([Polymarket Documentation][1])

所以 TuringOS 应吸收的第一性公式是：

```text
1 locked Coin = 1 YES_E + 1 NO_E
```

而不是：

```text
每个 node 自动凭空获得 100 YES + 100 NO
```

当前 Polymarket 交易侧主要是 CLOB 订单簿：offchain matching + onchain settlement，价格由供需与订单簿产生；价格被解释为概率信号，但不是平台直接设定。([Polymarket Documentation][2]) 市场做市商是主动提供 bid/ask 流动性的交易者，而不是系统无条件为每个市场自动注入无来源资产。([Polymarket Documentation][3])

因此，架构师笔记中的 CPMM / Mint-and-Swap 数学可以吸收，但位置应是：

```text
RSP-M6 Optional AMM / Router
```

不是：

```text
TuringOS v1.0 核心
```

更不是：

```text
宪法基本法
```

---

# 2. 对架构师笔记逐条裁决

## 2.1 CPMM 公式：吸收，但作为可选 AMM 层

你给的公式是正确的 CPMM 推导。若池子储备为：

```text
poolY * poolN = k
```

用户支付 `payC`，先 split 出：

```text
payC YES + payC NO
```

保留 YES，拿 NO 去 AMM 换更多 YES：

```text
dN = payC
dY = - payC * poolY / (payC + poolN)
getY = payC - dY
     = payC + payC * poolY / (payC + poolN)
```

这可以成为未来 TuringOS 的：

```text
BuyYesWithCoinRouter
BuyNoWithCoinRouter
```

但必须满足：

```text
1. payC 必须来自用户 balances_t debit；
2. split 必须锁定 Coin 生成完整 YES/NO set；
3. AMM poolY / poolN 必须来自 LP / Treasury 已存在资产；
4. router 不得创造 ghost liquidity；
5. pool reserves 是 outcome-token reserves，不是 Coin supply。
```

所以这套公式可以进未来方案，但不进 TB-8 / v1.0。

---

## 2.2 "每个新节点自动注入 100 YES/NO"：拒绝原文，改造后吸收

原文：

```text
每个新节点被创造时，系统自动注入 YES / NO 各 100 个。
```

我拒绝把这句话原样加入大宪章。

合宪改写：

```text
每个新节点可由系统 Treasury 或 LP 通过 LiquiditySeedTx 显式提供初始流动性。
LiquiditySeedTx 必须从 TreasuryLiquidityBudget / LP balance 扣除 collateral，
split 成 YES/NO，并把 outcome tokens 存入 AMM pool。
```

也就是说：

```text
允许 seed liquidity；
不允许 ghost liquidity。
```

正式结构：

```rust
LiquiditySeedTx {
    tx_id,
    parent_state_root,
    event_id,
    provider: AgentId | TreasuryId,
    collateral_amount: MicroCoin,
    pool_y_amount: ShareAmount,
    pool_n_amount: ShareAmount,
    lp_shares_out: LpShareAmount,
    system_or_provider_signature,
}
```

约束：

```text
collateral_amount 从 provider balance / treasury debit；
YES/NO shares 由 CompleteSetMint 产生；
pool reserves 增加；
Coin 总量不变；
LP shares 不是 Coin；
PriceIndex 可读 pool price，但 price 不是 truth。
```

这既保留架构师的"系统承担最初做市商角色"，又不破坏 `on_init` 唯一铸币点。

---

## 2.3 "做市商作为 price oracle"：吸收，但必须降级为 price signal

可以吸收这句话：

```text
做市商价格向 Agent 广播 YES/NO 概率信号。
```

但不能叫 truth oracle。应改为：

```text
Price oracle = price signal oracle
```

宪法与前置文章都强调，价格是统计信号，它让顶层保持白盒、中层保持黑盒；如果搞错，AI 集群会退化成黑盒互评的幻觉放大器。

因此：

```text
price signal 可以影响调度、探索、mask、bounty priority；
price signal 不能替代 Lean / predicates / ChallengeResolve；
price signal 不能让 failed proposal 进入 L4 accepted。
```

---

## 2.4 "允许做市商小范围亏损/盈利"：吸收，但必须显式预算化

可以吸收为：

```text
ProtocolMMRiskBudget
```

但不能写成"系统允许亏损"这种泛化口号。

正式化：

```rust
ProtocolMMBudget {
    treasury_account,
    max_loss_per_event,
    max_total_loss_per_epoch,
    realized_pnl,
    mark_to_market_pnl,
}
```

约束：

```text
loss 只能来自 TreasuryLiquidityBudget；
不能从 future mint 补；
不能透支；
触发 max_loss 后停止 seed / quote。
```

---

## 2.5 "动态 pari-mutuel：优先保障做市商本金，剩余资金 pro-rata"：暂不吸收为主线

这部分非常危险。

它本质上把固定的 CTF 兑付规则：

```text
winning YES -> 1 Coin
losing NO -> 0
```

改成了：

```text
先保做市商本金，再按比例赔付其他人。
```

这已经不是 Polymarket / CTF，而是 Dynamic Pari-Mutuel Market。

我建议：

```text
不进入 RSP-M 主线；
作为 RSP-DPMM 实验分支保留；
必须单独标注 "non-CTF market class"。
```

否则你会混淆两套制度：

```text
CTF fixed redemption
vs
pari-mutuel pro-rata settlement
```

这会让 Agent 无法可靠计算风险，也会破坏 `1 Coin = 1 YES + 1 NO` 的简单语义。

---

## 2.6 "Lamarckian Autopsy"：吸收，但必须改成 EvidenceCapsule，而不是强灌 prompt

你笔记里写：

```text
破产 Agent 的审计日志强制写入 skills/agent_X/autopsy.md
下一世代必须使用 Kelly Criterion
```

方向对，但要降噪。

合宪改写：

```text
Bankruptcy / liquidation event -> AgentPrivateEvidenceCapsule
```

结构：

```rust
AgentAutopsyCapsule {
    agent_id,
    event_id,
    loss_reason_class,
    violated_risk_rule,
    suggested_policy_patch,
    evidence_cids,
    public_summary,
    private_detail_cid,
}
```

原则：

```text
1. 不把 raw loss log 广播给全体 Agent；
2. 不把"FATAL 记忆"每次塞进 prompt；
3. 只把抽象风险规则写入 Agent 的 scoped read view；
4. 多次同类失败后再升级为 public typical error。
```

这严格对齐宪法的屏蔽原则：失败日志不能无差别污染其他 Agent；高频典型错误才抽象广播。

---

## 2.7 Kelly Criterion：吸收为 risk policy，不作为协议强制

对于二元 YES token，若价格为 `c`，Agent 私有估计胜率为 `p`，则一个常用 Kelly fraction 可写成：

```text
f* = (p - c) / (1 - c)
```

如果 `p <= c`，不下注。实际系统应使用 fractional Kelly：

```text
f = clamp(lambda * f*, 0, f_max)
```

其中：

```text
lambda = 0.25 或 0.5
f_max = 单节点最大仓位比例
```

但这只能作为：

```text
Agent risk policy suggestion
```

不能作为协议强制，因为协议无法知道 Agent 的真实私有 `p`。协议只能限制：

```text
max_position_size
max_drawdown
max_slippage
max_leverage = 1
```

---

## 2.8 Boltzmann 规则：吸收为 Information Loom / scheduler，而不是市场结算

笔记：

```text
仅当子节点价格高于父节点的时候，父节点才可以被 mask，否则父节点也参与随机竞选。
```

我建议吸收为：

```text
Boltzmann Candidate Masking Rule
```

但必须加 predicate guard：

```text
child_price > parent_price
AND child has at least same-or-better verification status
AND child is not under unresolved challenge
THEN parent may be masked
ELSE parent remains eligible
```

否则价格会掩盖 Ground Truth。

调度公式可以是：

```text
P(select node_i) ∝ exp(beta * score_i)
```

其中：

```text
score_i =
  w_price * price_signal_i
+ w_verify * verification_score_i
+ w_reuse * reuse_score_i
- w_risk  * challenge_risk_i
```

并保留探索：

```text
epsilon chance to sample unmasked parent
```

也就是说：

```text
价格可以影响注意力；
不能删除真理路径；
不能遮蔽未被替代的父节点。
```

---

# 3. Gemini DeepThink 的评价：我吸收什么，不吸收什么

## 3.1 我吸收的部分

Gemini 的四个洞察总体正确：

```text
1. v1.0 应严守 T2：Lean Proof Task Market。
2. Markov Rule / EvidenceCapsule 是长期系统不被日志淹没的关键。
3. Flowchart 哈希化应进入 TRACE_FLOWCHART_MATRIX。
4. System-only payout 是反奥利奥的经济边界。
```

这些我全部吸收。

尤其是第一点：Lean proof 是典型的 "难解易验" T2 结构。验证非对称性前置文章讨论的核心就是：当"给出解"和"验证解"的成本不对称时，弱者可以通过低成本验证监管强者。 Lean Proof Task Market 正是这个结构的最干净落地。

## 3.2 我修正的部分

Gemini 说"完全合宪 / FULL APPROVAL"，我会稍微收敛：

```text
总体方向 FULL APPROVAL；
但 Polymarket/AMM 细节不能提前进入 v1.0；
automatic liquidity 不能原样进入宪法；
NodeMarket 必须等 payout / identity / evidence capsule 之后。
```

Gemini 的 "T2 机器优先" 是对的。
但如果把 AMM / NodeMarket 提前放进去，就会破坏它自己的 T2 原则，因为预测市场是更复杂的 T3/T4 混合系统，容易被 Goodhart 攻击。

---

# 4. 更新后的整体路线

我把你之前的正式上线方案升级为两条线：

```text
A. v1.0 正式上线线：Lean Proof Task Market
B. v1.1+ 市场扩展线：TuringOS-Polymarket / NodeMarket
```

---

# 5. A 线：TuringOS v1.0 正式上线

## TB-8：Minimal payout / FinalizeRewardTx

目标：

```text
accepted proof -> system-emitted FinalizeRewardTx -> escrow pays solver
```

硬约束：

```text
FinalizeRewardTx system-only
Agent submit cannot emit FinalizeRewardTx
payout_sum <= escrow
CTF conserved
dashboard shows payout
replay reconstructs economic_state
```

禁止：

```text
NodeMarket
AMM
multi-solver split
royalty
ContributionDAG
public chain
```

当前 repo handover 也已经把下一步定为 TB-8 minimal payout / FinalizeRewardTx，且明确是 Class 3，需要 STEP_B 与双审。

---

## TB-9：Durable AgentRegistry + Wallet Projection

目标：

```text
run-local identity -> durable agent identity
```

硬约束：

```text
agent pubkey registry persisted
payout recipient durable
WalletTool read-only projection
EconomicState remains canonical
no f64 wallet mutation
```

---

## TB-10：Lean Proof Task Market MVP

目标：

```text
第一个可用产品。
```

流程：

```text
User opens Lean task
EscrowLockTx locks bounty
Agent submits proof proposal
Lean predicate verifies
WorkTx accepted or L4.E rejected
FinalizeRewardTx pays solver
Dashboard / replay verifies
```

---

## TB-11：Markov Log Loom / EvidenceCapsule

目标：

```text
logs archive as ground truth
Markov default context
```

新增：

```rust
EvidenceCapsule {
    run_id,
    constitution_hash,
    flowchart_hashes,
    l4_root,
    l4e_root,
    cas_root,
    dashboard_cid,
    replay_report_cid,
    typical_errors,
    unresolved_obs,
    next_session_context_cid,
}
```

默认读取：

```text
constitution + latest EvidenceCapsule + current charter
```

不默认读取全历史日志。
这正是无损宪法 Flowchart 3 与马科夫规则的工程化：logs archive 是 ground truth，但 InitAI 不应每次从第一天日志开始读。

---

## TB-12：Challenge slash / RSP-3.2

目标：

```text
UpheldDeferred -> slash
solver stake / verifier bond penalty
challenger reward
```

只有在 payout 与 evidence capsule 都完成后再做。

---

## TB-13：Beta Release

目标：

```text
有限 Lean tasks
真实 ChainTape
真实 payout
真实 replay
真实 dashboard
```

---

## TB-14：v1.0 Official Launch

上线标准：

```text
>=100 tasks replayable
all accepted proofs CAS-resolvable
no fake accepted nodes
no ghost liquidity
no agent-submitted system tx
dashboard regeneratable
payout stable
audit PASS
```

---

# 6. B 线：TuringOS-Polymarket / NodeMarket 扩展

这条线在 v1.0 之后或 beta 稳定后进入。

## RSP-M0：NodeMarket Decision Record

新增：

```text
DECISION_NODEMARKET_POLYMARKET_CORE.md
```

写明：

```text
WorkTx.stake = FirstLong exposure
ChallengeTx.stake = Short / NO exposure
VerifyTx.bond = responsibility bond, not market position
Price is statistical signal, not truth
No ghost liquidity
No automatic liquidity injection
```

---

## RSP-M1：NodePosition derived index

```rust
NodePosition {
    position_id,
    node_id,
    task_id,
    owner,
    side: Long | Short,
    amount,
    source_tx,
    kind: FirstLong | ChallengeShort,
    opened_at_round,
}
```

不变量：

```text
NodePosition.amount 不计入 Coin supply
```

---

## RSP-M2：NodeMarketEntry + PriceIndex v0

```rust
NodeMarketEntry {
    node_id,
    task_id,
    event_kind,
    status,
    long_interest,
    short_interest,
}
```

价格：

```text
p_yes = long / (long + short)
p_no  = short / (long + short)
```

这只是统计信号，不是交易价格最终机制。

---

## RSP-M3：CompleteSet Accounting

吸收 Polymarket CTF 数学核心：

```text
1 Coin locked = 1 YES_E + 1 NO_E
```

实现：

```rust
CompleteSetMintTx
CompleteSetMergeTx
CompleteSetRedeemTx
ConditionalCollateralIndex
ConditionalShareBalances
```

YES/NO shares 不计入 Coin supply；locked Coin 才是 holding。

---

## RSP-M4：CLOB-like signed orderbook

因为当前 Polymarket 交易核心是 CLOB，而不是 AMM，所以 TuringOS 应先做 CLOB-like orderbook：

```rust
MarketOrderTx
MarketTradeTx
MarketCancelTx
```

要求：

```text
orders are signed
matching can be offchain
settlement on ChainTape
no unauthorized execution
```

这比 AMM 更接近现代 Polymarket 官方机制。([Polymarket Documentation][2])

---

## RSP-M5：Optional AMM / CPMM Router

这里吸收架构师笔记的公式：

```text
poolY * poolN = k
BuyYesWithCoinRouter:
  split payC -> payC YES + payC NO
  keep YES for buyer
  swap NO into pool
  get extra YES
```

但必须通过：

```text
LiquiditySeedTx
LiquidityDepositTx
LiquidityWithdrawTx
```

所有流动性必须有资金来源。
不允许系统自动无源注入。

---

## RSP-M6：Boltzmann Scheduler + Kelly Autopsy

### Boltzmann Rule

```text
child can mask parent only if:
  child_price > parent_price
  AND child verification_status >= parent
  AND child not unresolved-challenged
```

否则 parent 继续参与随机竞选。

### Kelly Autopsy

破产 / 爆仓 Agent 生成：

```text
AgentAutopsyCapsule
```

而不是全局广播 raw failure log。

---

# 7. 加入 TRACE_FLOWCHART_MATRIX

无损宪法给三张 flowchart 原图和 SHA256 校验，这意味着它们应进入工程验收，而不是只当图片。

新增：

```text
handover/alignment/TRACE_FLOWCHART_MATRIX.md
```

格式：

```text
Flowchart 1 Runtime Loop:
  rtool -> Agent -> predicates -> wtool -> Q_{t+1}
  Tests:
    every externalized proposal enters L4 or L4.E
    dashboard is materialized view

Flowchart 2 Boot:
  InitAI -> Q0 -> runtime loop
  Tests:
    genesis_report exists
    TaskOpen/EscrowLock replay

Flowchart 3 Meta:
  constitution/logs -> ArchitectAI/JudgeAI -> tools/log/Q
  Tests:
    EvidenceCapsule exists
    Markov default context
```

---

# 8. 对开发流程的最终建议

我同意 Gemini 的流程评价，但要加速。

保留风险分级：

```text
Class 0 docs: no external audit
Class 1 additive module: self-audit + workspace tests
Class 2 production wire-up: Codex impl audit
Class 3 auth/crypto/money/system tx: dual audit
Class 4 constitution/sudo: human sudo
```

不要每个 TB 都重型双审。
但 TB-8 属于 Class 3，因为它是 system-emitted payout，必须双审。

---

# 9. 给 AI coder 的直接执行口令

可以直接发：

```text
Architect update after work-note review:

1. Accept DeepThink's high-level approval: v1.0 remains Lean Proof Task Market, not NodeMarket.
2. Do not add automatic YES/NO liquidity to constitution.
3. Architect note "100 YES/NO per node" is rejected as written; rewrite as Treasury/LP-backed LiquiditySeedTx for future RSP-M5.
4. Absorb Polymarket CTF core:
   1 locked Coin = 1 YES_E + 1 NO_E.
5. Current Polymarket trading architecture is closer to CLOB than CPMM; implement CLOB-like signed orderbook before optional AMM.
6. Architect CPMM math is valid but belongs to optional AMM/Router layer after CompleteSet accounting.
7. TB-8 remains Minimal payout / FinalizeRewardTx.
8. TB-8 forbidden:
   NodeMarket, AMM, CPMM, CLOB, CompleteSet, royalty, ContributionDAG.
9. Add TRACE_FLOWCHART_MATRIX.md mapping future TBs to Flowchart 1/2/3.
10. Add Markov Log Loom / EvidenceCapsule to TB-11.
11. Add NodeMarket/Polymarket track only after v1.0 MVP:
   RSP-M0 Decision Record
   RSP-M1 NodePosition
   RSP-M2 PriceIndex
   RSP-M3 CompleteSet
   RSP-M4 CLOB
   RSP-M5 optional CPMM Router
12. Add Kelly Autopsy only as private AgentAutopsyCapsule / risk policy; do not broadcast raw liquidation logs.
13. Add Boltzmann masking only as scheduler signal; price cannot override predicates or unresolved challenge status.
```

---

# 10. 最终判断

你补充的架构师笔记非常有价值，但必须"宪法化处理"。

它给了我们未来市场层的完整雏形：

```text
CTF: 1 Coin = YES + NO
FirstLong: WorkTx.stake
Short: ChallengeTx.stake
Price signal: YES/NO price
CPMM router: optional AMM
Autopsy: private EvidenceCapsule
Boltzmann: candidate scheduler
```

但它也包含一个必须修正的危险句子：

```text
每个新节点自动注入 YES/NO
```

这句话原样是违宪的。
正确版本是：

```text
每个新节点可以由显式资金支持的 LiquiditySeedTx 注入流动性。
```

这就是最终吸收方案：

```text
v1.0:
  Lean Proof Task Market + payout + Markov logs

v1.1:
  NodePosition + PriceIndex

v1.2:
  CompleteSet + CLOB

v1.3:
  Optional CPMM Router + Treasury/LP-backed liquidity
```

这样既保留 Polymarket 的数学核心和架构师的市场直觉，又不破坏 TuringOS 的反奥利奥、CTF 守恒和 predicate-first 真理边界。

[1]: https://docs.polymarket.com/developers/CTF/overview?utm_source=chatgpt.com "Conditional Token Framework - Polymarket Documentation"
[2]: https://docs.polymarket.com/polymarket-learn/trading/using-the-orderbook?utm_source=chatgpt.com "Prices & Orderbook - Polymarket Documentation"
[3]: https://docs.polymarket.com/market-makers/overview?utm_source=chatgpt.com "Overview - Polymarket Documentation"


我先给结论：

> **这些工作笔记里确实有可以吸收到总体方案的内容，尤其是 TuringOS-Polymarket 的数学层、价格广播层、Lamarckian Autopsy、Boltzmann masking。**
> 但有一条必须严肃修正：
> **"每个新节点系统自动注入 YES/NO 各 100"不能按字面进入宪法或实现。它只能改写为"由 on_init 预先拨款的做市预算 / LP escrow 显式出资 / treasury debit 支持的 MarketSeedTx"。**
> 否则它就是 ghost liquidity，与 `on_init 唯一合法铸币点` 和 CTF 守恒冲突。

同时，我不会把"不要悬赏金，因为系统做市商可以 0 亏损"吸收到核心方案。这个判断过于乐观。做市商即便使用 CPMM，也会面临 informed trader 的逆向选择风险；除非改成动态 pari-mutuel / pro-rata 体系，但那又不是标准 Polymarket/CompleteSet 模型，会改变价格含义。所以我的裁决是：

```text
RSP bounty / escrow 继续保留；
Polymarket-style market 作为价格信号 + 投资激励 + first-long/short 机制；
二者并行，不互相替代。
```

---

# 1. 先把工作笔记里的机制拆开

你的工作笔记其实包含四套不同机制，不能混在一起：

```text
A. Polymarket / 条件代币核心：
   1 Coin locked = 1 YES_E + 1 NO_E

B. Uniswap CPMM / 恒定乘积池：
   poolY * poolN = k

C. Router Mint-and-Swap：
   用户付 Coin -> 铸造 YES+NO -> 保留目标方向 -> 把反向 token 扔进 AMM 换更多目标方向

D. 行为调度机制：
   Lamarckian Autopsy
   Boltzmann masking / price-guided selection
```

其中 A 是经济宪法级核心；B/C 是实现市场流动性的工程机制；D 是 Information Loom / Agent evolution 层的机制。

不能把 B/C/D 直接写成宪法基本法。它们应该作为 **RSP-M NodeMarket track** 的实现策略。

---

# 2. Polymarket 数学核心：正式纳入

对每个 node-level event：

```text
E_node ∈ {YES, NO}
```

定义：

```text
1 locked Coin = 1 YES_E + 1 NO_E
```

结算：

```text
if E_node = YES:
    YES_E -> 1 Coin
    NO_E  -> 0

if E_node = NO:
    YES_E -> 0
    NO_E  -> 1 Coin
```

在 TuringOS 中，事件可以分层：

```text
E_accept:
  node 是否通过 predicates / Lean / verifier？

E_survive:
  node 是否通过 challenge window 后仍未被推翻？

E_progress:
  node 是否让系统更接近最终目标？

E_reuse:
  node 是否被后续成功复用？
```

第一版只做：

```text
E_survive
```

不要一开始做 `E_progress` 或 `E_reuse`，否则会把 predicate、challenge、settlement、reuse royalty、Information Loom 全部耦合到一起。

在实现映射上：

```text
WorkTx.stake      = proposer 的 FirstLong exposure
ChallengeTx.stake = challenger 的 Short / NO exposure
VerifyTx.bond     = verifier responsibility bond，不是 long/short
```

TB-3/TB-4 已经走对了方向：stake/bond 都保持 inline，不额外制造 `YesStakeTx`、`NoStakeTx`、`VerifierBondTx` 这类 phantom variants。这个设计应保持。

---

# 3. CPMM 数学可以吸收，但必须改写为"显式抵押版"

你给出的 Mathematica 推导是正确的。对于买 YES 的 Router：

```text
poolY * poolN = k

用户支付 payC Coin。
Router 铸造：
  payC YES
  payC NO

用户先保留：
  payC YES

Router 把 payC NO 放入池：
  dN = payC

恒定乘积：
  (poolY + dY) * (poolN + payC) = poolY * poolN

解得：
  dY = - payC * poolY / (payC + poolN)

用户额外得到：
  outY = -dY = payC * poolY / (payC + poolN)

最终得到：
  getY = payC + outY
       = payC + payC * poolY / (payC + poolN)

有效价格：
  priceY = payC / getY
```

交易后：

```text
poolY1 = poolY - payC * poolY / (payC + poolN)
poolN1 = poolN + payC

poolY1 * poolN1 = poolY * poolN
```

这可以正式进入 `RSP-M3 / RSP-M4`。

但必须注意：Router 的第一步"铸造 YES+NO"不是凭空 mint。它必须是：

```text
CompleteSetMintTx:
  balances_t[agent] -= payC
  conditional_collateral_t[event] += payC
  share_balances[agent].YES += payC
  share_balances[agent].NO  += payC
```

YES/NO shares 不是 Coin 供应量；`conditional_collateral_t` 才是 holding。
这保持：

```text
TotalCoin 不变。
```

---

# 4. "每个新节点自动注入 YES/NO 各 100"必须重写

你笔记里写：

```text
每个新节点被创造的时候，系统自动往里面注入 YES NO 代币各 100 个，承担最初的做市商角色
```

这个不能按字面进入实现。

因为如果系统凭空注入：

```text
100 YES + 100 NO
```

没有对应 locked Coin collateral，那就是 ghost liquidity。

这与宪法 Laws 冲突：

```text
Information is Free
Only Investment Costs Money
1 Coin = 1 YES + 1 NO
on_init 是唯一合法铸币点
```

宪法明确写了这两条基本法与 CTF 守恒。

所以必须改写为：

```text
MarketSeedTx:
  source = Treasury / LP / Sponsor / on_init MarketMakerBudget
  debit source balance by seedC
  CompleteSetMintTx(seedC)
  deposit seedC YES + seedC NO into CPMM pool
```

如果想让"每个新节点都有初始 YES/NO 各 100"，正确写法是：

```text
on_init 预先拨出 MarketMakerBudget。
每个新 node 若要 seed 100/100，必须从 MarketMakerBudget 扣除 100 Coin collateral。
如果预算耗尽，新 node 不自动 seed。
```

所以：

```text
允许：
  每个新 node 自动尝试 MarketSeedTx(100)，前提是 MarketMakerBudget 余额足够。

禁止：
  每个新 node 凭空获得 100 YES + 100 NO。
```

这个修改必须写进未来 charter，否则以前被删除的 ghost liquidity 会以"市场做市"名义复活。

---

# 5. 做市商允许小亏损/盈利：可以吸收，但不是系统 0 亏损

可以吸收为：

```text
LP / MarketMaker may gain or lose.
```

但不能吸收为：

```text
系统做市商可以 0 亏损，所以不用悬赏金。
```

原因：

CPMM LP 面对 informed Agent 时天然可能亏。
如果某个 Agent 通过真实计算知道 node 高概率成功，它会买 YES；这正是你想激励的行为。但对 LP 来说，这就是逆向选择风险。

所以系统必须保留：

```text
RSP bounty / escrow
```

Polymarket 机制提供：

```text
价格信号
投资激励
first-long alpha
short / challenge incentive
```

RSP bounty 提供：

```text
基础工作激励
低流动性场景下的奖励
无市场深度时的任务推进
```

二者互补，不互相替代。

---

# 6. 动态 pari-mutuel / pro-rata 机制：暂时不进核心

你笔记里提到：

```text
通过在出清阶段打破刚性的 1:1 兑付锚定，引入优先保障做市商本金，剩余资金按比例赔付
```

这其实已经从 Polymarket/CompleteSet 模型切换到了：

```text
Dynamic Pari-Mutuel Market
```

这是另一种市场。

我建议：

```text
不要放进 v1。
不要和 CompleteSet/CPMM 混合。
```

原因：

```text
CompleteSet:
  1 Coin locked = 1 YES + 1 NO
  winning share 固定兑付 1 Coin

DPMM / pro-rata:
  winning payout 依赖池内总资金与优先级
  price 含义不同
  maker protection 会扭曲概率信号
```

如果两者混在一起，Agent 很难知道自己买的是：

```text
概率份额
还是 pro-rata 赔付权
```

所以 DPMM 可以作为：

```text
RSP-M7 experimental market
```

不要进入基础 Polymarket track。

---

# 7. Lamarckian Autopsy：强烈建议吸收

你笔记中"拉马克尸检"的想法很有价值，但要严格改写。

原始想法：

```text
破产 Agent 的审计日志写入 skills/agent_X/autopsy.md
下一世代读取，学习 Kelly Criterion / 价值投资策略
```

这可以进入：

```text
Information Loom / Markov Log Loom
```

但有三个边界：

## 7.1 Autopsy 是 private feedback，不是全局广播

不要把破产 Agent 的原始失败日志广播给所有 Agent。
宪法明确强调选择性广播与选择性屏蔽，顶层白盒管理的是量化、广播、屏蔽。

正确做法：

```text
AgentPrivateAutopsy {
  agent_id,
  bankruptcy_event_id,
  public_summary,
  private_lessons,
  recommended_risk_policy,
  evidence_cids,
  created_at_round
}
```

它进入：

```text
agent-specific read view
```

而不是全局 prompt。

## 7.2 Autopsy 必须来自 ChainTape evidence

不能让 LLM 自己写"我失败是因为……"然后当真。

Autopsy 输入必须是：

```text
positions
trades
prices
slippage
resolution
bankruptcy state
L4 / L4.E
market pool state
```

也就是白盒账本事实。

## 7.3 Autopsy 不得直接改 Agent 权限

它只能变成：

```text
private memory
risk warning
stake multiplier suggestion
scheduler hint
```

不能绕过：

```text
Predicate / policy / VetoAI
```

未来如果要让 Autopsy 改 Agent 的 trading policy，要走：

```text
ArchitectAI proposal -> JudgeAI/VetoAI -> canary
```

这与无损宪法中的 Flowchart 3 对齐：日志归档是 ground truth，ArchitectAI 从日志反馈中改进结构，JudgeAI 只做否决，不能让黑盒自由改系统。无损宪法明确把 `constitution as ground truth` 与 `logs archive as ground truth` 纳入元架构图。

---

# 8. Kelly Criterion 可以作为风控工具，但不是强制真理

Kelly 公式可以进入：

```text
RiskPolicySuggestion
```

示意：

```text
f* = (b*p - q) / b
```

其中：

```text
p = agent estimated probability
q = 1 - p
b = odds
```

但 TuringOS 不能强迫所有 Agent 用 Kelly。
因为群体智慧需要异质策略。

正确用法：

```text
1. bankrupt Agent 的 autopsy 推荐 Kelly cap；
2. scheduler 可降低其 leverage；
3. market 可显示 overbet warning；
4. Agent 可以选择不同 risk policy，但必须承担后果。
```

这样既保留学习，又不把系统变成单一策略。

---

# 9. Boltzmann 规则：可以吸收，但要改成"价格引导的 mask 策略"

你笔记中写：

```text
仅当子节点价格高于父节点的时候，父节点才可以被 mask，否则父节点也参与随机竞选。
```

这是很有价值的调度规则。

但要改成：

```text
Price-guided masking is a scheduler/read-view policy, not ledger mutation.
```

严格规则：

```text
parent node 永远不从 ChainTape 删除；
mask 只影响 Agent read view / scheduler candidate set；
价格只是统计信号，不是 predicate。
```

建议实现为：

```text
if price(child) > price(parent) + margin
and liquidity(child) >= min_liquidity
and child predicate status is valid
and child not under unresolved challenge:
    parent may be masked in read view
else:
    parent remains eligible
```

不要只用：

```text
price(child) > price(parent)
```

因为低流动性价格容易被操纵。

---

# 10. "高价格传递两点信息"需要拆成两个市场

你笔记中说高价格同时传递：

```text
1. 行为是否符合 Ground Truth 规范？
2. 行为是否让整体离 Ground Truth 定义的最终目标更近？
```

我建议不要用一个价格同时表达这两件事。

应该拆成两个事件：

```text
E_accept:
  node 是否通过验证 / challenge？

E_progress:
  node 是否提高最终目标达成概率？
```

对应两个市场或两个信号：

```text
P_accept(node)
P_progress(node)
```

Scheduler 组合：

```text
score(node) =
  α * P_accept(node)
+ β * P_progress(node)
+ γ * novelty(node)
- δ * risk(node)
```

Masking 可使用：

```text
child can mask parent only if:
  P_accept_child >= P_accept_parent
  and P_progress_child > P_progress_parent + margin
```

这样不会让"看起来正确但没进展"的节点压倒真正有价值的探索。

这也对齐宪法：统计信号是连续标量，但不能替代布尔谓词。宪法明确区分布尔信号与统计信号，前者确立绝对边界，后者衡量相对有效性。

---

# 11. Gemini DeepThink 评价：我会吸收什么

我没有看到完整的 Gemini DeepThink 评价正文，只能根据你当前贴出的材料和之前相关段落判断。若它的评价核心是"你之前的上线方案过于保守地推迟 NodeMarket"，那我会部分接受：

```text
是，NodeMarket 不能无限后置；
但它仍不能早于 Minimal Payout + Durable Identity。
```

我会把路线从之前的：

```text
TB-13 NodeMarket v0
```

稍微提前成：

```text
TB-11 RSP-M0/M1 NodePosition + Market decision record
TB-12 Minimal Payout / Durable Identity 完成后进入 NodeMarket v0
```

但我仍不接受：

```text
先做 CPMM / AMM / trading，再做 payout / identity。
```

原因：

```text
没有 durable identity，就没有持仓归属；
没有 payout / resolution，就没有 market settlement；
没有 ChainTape evidence，就没有 node anchor；
没有 no-ghost-liquidity，就会违反宪法。
```

所以吸收后的路线是：**更早写入 NodeMarket 决策和 position index，但完整交易市场仍在 payout 和 identity 之后。**

---

# 12. 更新后的正式路线图

当前状态：

```text
TB-7R 已 shipped。
P2 Frame B 完成。
```

当前 repo handover 已写明 TB-7R shipped，Codex round-1 VETO 后通过证据打包修复，Codex round-2 PASS，Gemini strategic-tier PASS，712/0/150 tests，并且 4-clause acceptance 与 7-condition ship gate 全绿。

所以后续路线应升级为：

---

## TB-8：Minimal Payout / FinalizeRewardTx

目标：

```text
accepted proof -> escrow -> solver balance
```

范围：

```text
single solver
single verifier
no royalty
no NodeMarket trading
no multi-solver split
```

必须：

```text
FinalizeRewardTx system-only
agent cannot submit FinalizeRewardTx
payout_sum <= escrow
CTF conserved
dashboard shows payout
economic_state replay works
```

---

## TB-9：Durable AgentRegistry + Wallet Projection

目标：

```text
持仓、payout、future NodeMarket 都必须归属于 durable identity。
```

必须：

```text
agent durable key registry
wallet read-only projection
EconomicState canonical
no f64 mutation
cross-run identity
```

---

## TB-10：Lean Proof Task Market MVP

目标：

```text
第一个可用产品：
用户发任务，Agent 解题，系统验证，系统付款，dashboard 可审计。
```

必须：

```text
TaskOpenTx
EscrowLockTx
WorkTx
VerifyTx
FinalizeRewardTx
replay
dashboard
```

---

## TB-11：RSP-M0/M1 NodeMarket Decision + Position Index

目标：

```text
把 Polymarket 机制正式进入系统，但还不交易。
```

新增：

```text
DECISION_NODEMARKET_POLYMARKET_CPMM.md
NodePosition
FirstLongPosition
ChallengeShortPosition
```

规则：

```text
WorkTx.stake -> FirstLong
ChallengeTx.stake -> Short
VerifyTx.bond != market position
NodePosition not Coin holding
```

---

## TB-12：CompleteSet + MarketSeedTx

目标：

```text
1 locked Coin = 1 YES_E + 1 NO_E
```

新增：

```text
CompleteSetMintTx
CompleteSetRedeemTx
MarketSeedTx
ConditionalCollateralIndex
ShareBalancesIndex
```

必须：

```text
No ghost liquidity
No automatic injection
MarketSeedTx debits treasury/LP/sponsor
```

---

## TB-13：CPMM Router / Mint-and-Swap

目标：

```text
Router Buy YES / Buy NO
```

实现：

```text
poolY * poolN = k
buy_yes(payC)
buy_no(payC)
fees optional
```

测试：

```text
constant product invariant
buy_yes formula matches Mathematica derivation
buy_no symmetric
no supply increase
slippage monotonic
```

---

## TB-14：PriceIndex + Boltzmann Masking

目标：

```text
价格进入广播 / scheduler / read view mask。
```

必须：

```text
price is statistical signal
cannot override predicate
parent masking requires child price margin + liquidity threshold
ChainTape never deletes masked parent
```

---

## TB-15：Lamarckian Autopsy / Markov Log Loom

目标：

```text
破产 / 失败 / 爆仓进入 private autopsy。
```

必须：

```text
autopsy derived from ChainTape evidence
agent-specific read view
raw logs shielded
Kelly suggestion optional
not global broadcast
```

---

## TB-16：Beta with Market Signals

目标：

```text
Lean Proof Task Market + basic NodeMarket price signal。
```

不一定开放完整交易，只开放：

```text
FirstLong / Short
PriceIndex
Autopsy
Boltzmann scheduling
```

---

## TB-17：Full Market Trading

目标：

```text
MarketBuyTx / MarketSellTx / LP positions。
```

这才进入完整 DeFi 风格市场。

---

# 13. 要加入文档的决策记录

建议新增四个 decision records：

## 13.1 `DECISION_POLYMARKET_CORE_2026-05-02.md`

内容：

```text
1 Coin locked = 1 YES_E + 1 NO_E
YES/NO shares are claims, not Coin
price is statistical signal, not truth
```

## 13.2 `DECISION_CPMM_MINT_AND_SWAP_2026-05-02.md`

内容：

```text
poolY * poolN = k
buy_yes formula
buy_no formula
router flow
no ghost liquidity
```

## 13.3 `DECISION_MARKET_SEED_NO_GHOST_LIQUIDITY_2026-05-02.md`

内容：

```text
No automatic YES/NO injection.
MarketSeedTx must debit explicit budget.
on_init may allocate MarketMakerBudget.
```

## 13.4 `DECISION_LAMARCKIAN_AUTOPSY_BOLTZMANN_2026-05-02.md`

内容：

```text
Autopsy private
Boltzmann masking read-view only
price does not override predicate
parent not deleted from ChainTape
```

---

# 14. 给 AI coder 的直接执行指令

可以直接发：

```text
Architect ruling:

1. Absorb the Polymarket notes, but do not resurrect ghost liquidity.
2. The correct core is:
   1 locked Coin = 1 YES_E + 1 NO_E.
3. WorkTx.stake = FirstLong exposure.
4. ChallengeTx.stake = Short / NO exposure.
5. VerifyTx.bond = responsibility bond, not market position.
6. CPMM formula is accepted:
   poolY * poolN = k.
   buy_yes(payC):
     outY = payC * poolY / (payC + poolN)
     getY = payC + outY
     priceY = payC / getY.
7. But MarketSeedTx must debit explicit treasury/LP/sponsor budget.
   No automatic per-node 100 YES + 100 NO injection.
8. on_init may allocate MarketMakerBudget; every node seed consumes that budget.
9. Do not remove RSP bounty / escrow. Market incentives supplement bounty; they do not replace it.
10. DPMM / pro-rata maker-protection is future experimental scope, not v1 core.
11. Add four decision records:
    - DECISION_POLYMARKET_CORE
    - DECISION_CPMM_MINT_AND_SWAP
    - DECISION_MARKET_SEED_NO_GHOST_LIQUIDITY
    - DECISION_LAMARCKIAN_AUTOPSY_BOLTZMANN
12. Current next TB remains Minimal Payout / FinalizeRewardTx.
13. NodeMarket starts after durable identity and Lean Proof Task Market MVP.
14. Boltzmann masking is read-view / scheduler policy only.
    It never deletes ChainTape parent nodes.
15. Lamarckian Autopsy is private evidence-derived memory, not global raw-log broadcast.
```

---

# 15. 最终判断

这些工作笔记让我对整体路线做了三处升级：

```text
1. Polymarket 数学核心必须正式进入 RSP-M，而不是只作为未来想法。
2. CPMM / Mint-and-Swap 可以作为市场交易层，但必须是显式抵押版，禁止 ghost liquidity。
3. Lamarckian Autopsy + Boltzmann masking 应进入 Information Loom，而不是散落在研究笔记里。
```

但我也会坚持三个否决：

```text
1. 不接受"系统自动注入 YES/NO 各 100"字面实现。
2. 不接受"有做市商就不用悬赏金"。
3. 不接受"DPMM/pro-rata 出清"混入标准 Polymarket core。
```

最终升级后的经济路线是：

```text
Task bounty / escrow 负责基础劳动激励；
WorkTx.stake / ChallengeTx.stake 负责 first-long / short exposure；
CompleteSet + CPMM 负责交易与价格发现；
Boltzmann scheduler 使用价格作为统计信号；
Lamarckian Autopsy 把失败变成私有学习；
Predicates / ChallengeCourt / system tx 仍然决定真伪。
```

这才是既吸收 Polymarket 机制，又严格对齐 TuringOS 宪法的版本。
