# REAL-11 Agent Economic Action Activation — Architect Original

This file preserves the user-provided architect ruling verbatim as the local
fact source for REAL-11 planning and later implementation. Do not summarize this
file as the authority; quote or reference it directly.

```text
> **REAL-10 可以批准为一次干净、合宪、可审计的受控市场实验证据；但它同时明确告诉我们：当前市场机制仍停留在“结构性市场活动增加”，不是“Agent 自发经济行为出现”。**
> 下一步不应该继续简单扩大同一套 REAL-8 A/B 到 30 题，也不应该急着 live REAL-6B；应先开一个新的 **REAL-11：Agent Economic Action Activation**，专门解决 “market_tx_count 上升但 buy_with_coin_router = 0” 这个核心问题。

---

# 1. 对 REAL-10 报告问题的回答

## 1.1 clean evidence 是否有效？

**有效。**

你给出的 clean evidence：

```text
handover/evidence/real8x_market_ab_clean_20260515T141331Z/
```

可以作为 REAL-10 的 canonical evidence。

而：

```text
handover/evidence/real8x_market_ab_20260515T134453Z/
```

只能作为 contamination/remediation record，不能再用于结论。

后续必须在 `LATEST.md`、TB_LOG 或 REAL-10 ratification 中明确：

```text
only real8x_market_ab_clean_20260515T141331Z is conclusion-bearing.
real8x_market_ab_20260515T134453Z is invalid for conclusions.
```

---

## 1.2 测试和宪法 gates 是否足够支持 REAL-10 ratify？

**足够支持窄化 ratify。**

你给出的结果是：

```text
constitution gates: 461 passed / 0 failed / 1 ignored
workspace tests: exit 0
targeted tests:
  constitution_real8_market_ab_benchmark: 9/9
  constitution_real10_trace_cleanup: 6/6
  constitution_real10_emergence_metrics: 4/4
Trust Root: pass
audit_tape: pass
```

这符合目前 TuringOS 的 Constitution Harness Engineering 标准。Execution Matrix 本身也明确要求：GREEN 必须是“测试存在、断言真实 invariant、且 cargo test 通过”，不能只靠文档或审计。

所以我批准：

```text
REAL-10 = RATIFY AS CLEAN CONTROLLED MARKET EVIDENCE
```

但只限于下面这个 claim。

---

## 1.3 REAL-10 能证明什么？

它证明：

```text
E1:
  market-visible arms B/C/D 可以产生可审计市场上下文；
  market tx count 明显高于 market-disabled A；
  所有 arms audit PROCEED；
  输入 pinning 没有 drift；
  claim boundary 正确。
```

具体数据：

```text
A: market disabled                         market_tx=0   solve=5/15
B: market visible, no TaskOutcomeMarket    market_tx=10  solve=5/15
C: TaskOutcomeMarket enabled               market_tx=42  solve=6/15
D: TaskOutcomeMarket + scripted fixture    market_tx=38  solve=4/15
```

这说明：

```text
market structure is active on tape.
```

但不能说明：

```text
agents are trading.
```

因为核心交易指标仍是：

```text
buy_with_coin_router = 0 for all arms.
```

这点必须写入 ratification。

---

## 1.4 REAL-10 不能证明什么？

不能证明：

```text
E2 spontaneous market action
E3 persistent role differentiation
E4 causal performance improvement
live REAL-6B approval
market-caused solve improvement
model ranking
autonomous secondary market alive
```

你的 Emergence Metrics 定义是正确的：

```text
E2 必须是 live、non-scripted、agent-generated BuyWithCoinRouterTx 或 short-equivalent；
scripted action 不能算 E2；
role label 不能算 E3；
small-n descriptive evidence 不能 claim E4。
```

这个边界非常重要。

---

## 1.5 Codex REAL-8 / REAL-9 审计如何处理？

接受 Codex `PROCEED`。

Codex 没发现 production defects，并且认可 stale-parent 修复：`VerifyTx` 构造前会从当前 `q_snapshot().state_root_t` 刷新，且没有绕过 sequencer admission；同时 `VerifyTx` canonical signing payload 包含 `parent_state_root`。

但是，Codex 也指出当前 stale-parent fix 的测试仍偏 source-grep / count，不是直接 behavioral unit test。这个 gap 在 REAL-10 前置任务里已经应该被补；如果已经补了，就可以关闭。如果还没有，应作为 REAL-11 Atom 0 的前置核查。

---

# 2. 宪法与整体方案 Gap Analysis

## 2.1 宪法主干：已经不是当前瓶颈

当前宪法实现的主干已经很强：

```text
Art. 0.1 四元素映射：GREEN
Art. 0.2 Tape Canonical：GREEN
FC1 / FC2 / FC3 gates 已经体系化
Constitution gates 461 / 0 / 1
```

Execution Matrix 的作用就是把自然语言宪法变成 repo-side executable CI；每一行绑定 constitution clause / flowchart node 到 code surface、test、smoke evidence、status、kill condition。

宪法本身也说，顶层白盒的职责不是 micromanagement，而是对系统信息进行：

```text
量化
广播
屏蔽
```

人类工程师的核心价值是设计让 Agent 可靠工作的环境，而不是继续微观写代码。

所以目前的主要问题不是：

```text
宪法没落地。
```

而是：

```text
宪法落地后的生成性经济是否真的产生行为。
```

---

## 2.2 市场层 gap：market_tx_count ≠ agent economic action

这是我认为 REAL-10 暴露出的最重要洞察。

你现在有：

```text
B/C/D market_tx_count > 0
```

但：

```text
buy_with_coin_router = 0
```

所以必须拆分两个指标：

```text
StructuralMarketTx:
  MarketSeed
  TaskOutcomeMarket setup
  EventResolve
  scripted AttemptPrediction fixture
  market infrastructure tx

AgentEconomicActionTx:
  BuyWithCoinRouterTx
  Sell / short equivalent
  live agent-generated invest
  live agent-generated liquidity action
```

当前 REAL-10 证明的是：

```text
StructuralMarketTx works.
```

尚未证明：

```text
AgentEconomicActionTx emerges.
```

这应当成为下一步报告和 dashboard 的硬区分。否则 `market_tx_count` 增加会制造叙事误导。

---

## 2.3 与 v3 的关系：v3 的压力环还没在 v4 中闭合

v3 最有价值的不是旧实现，而是短反馈回路：

```text
LLM action
-> wallet pressure
-> market price
-> prompt / scheduler visibility
-> OMEGA settlement
-> memory / PnL feedback
```

v3 research packet 明确把这个反馈环称为 v3 的 useful spark，并建议 v4 用 typed transactions、integer accounting、ChainTape/CAS replay 和 price-as-signal guardrails 重建它。

REAL-10 现在只证明了其中一部分：

```text
market price / market structure visible
```

还没有闭合：

```text
agent economic action
-> PnL consequence
-> next prompt/scheduler behavior
```

所以当前 gap 是：

```text
lawful pressure substrate exists,
but pressure has not yet become agent behavior.
```

---

## 2.4 与 REAL-5 / REAL-9 的关系

REAL-5S → REAL-9 报告已经把项目准确定位为：

```text
TuringOS now has a chain-backed role scaffold and lawful market-pressure substrate,
not spontaneous market emergence.
```

该报告也明确说 REAL-5 证明 role scaffolding，不证明 market emergence。

REAL-10 没有改变这个结论，只是增强了它：

```text
市场结构活动增加了；
但 Agent 仍未自发交易；
性能提升未证明。
```

所以现在项目从“工程 scaffolding”进入“科学因果实验”的阶段。

---

# 3. 我对当前项目进展的评价

我的评价是：

```text
Constitution landing: 强。
ChainTape / CAS / L4 / L4.E: 强。
Role scaffold: 强。
Market infrastructure: 已经能跑。
Benchmark discipline: 明显进步。
Agent economic emergence: 未证明。
Causal performance gain: 未证明。
```

这不是坏事。
这说明项目已经从：

```text
能不能合宪地跑？
```

进化到：

```text
合宪市场机制是否真的带来智能增益？
```

这个问题更接近研究核心，也更难。

---

# 4. 下一步总路线：不要继续简单扩大 REAL-8，先开 REAL-11

我建议不要马上做：

```text
REAL-8Y 30 tasks per arm
```

原因：

```text
REAL-10 已经说明同一机制下 market_tx_count 增加，但 buy_with_coin_router = 0。
继续扩大样本很可能只会更精确地确认“结构市场活跃，agent economic action 仍为 0”。
```

如果目标是性能估计，可以扩大 REAL-8。
但如果目标是市场机制真正激活，下一步应该是：

```text
REAL-11 — Agent Economic Action Activation
```

它专门解决：

```text
为什么 market_tx 增加了，但 Agent 不产生 live router action？
```

---

# 5. REAL-11 总目标

```text
把 market activity 从 structural tx 推进到 agent economic action。
```

也就是说：

```text
从：
  Market exists.
  Agent sees market.
  NoTradeReason exists.

推进到：
  Agent can and sometimes does make live economic action.
  If not, the system can explain exactly why.
```

---

# 6. REAL-11 原子级计划

## Atom 0 — REAL-10 narrow ratification + evidence hygiene

### 任务

```text
1. 归档 REAL-10 narrow ratification。
2. 标记 clean evidence dir 为唯一 conclusion-bearing evidence。
3. 标记 contaminated evidence dir 为 remediation-only。
4. 确认 stale-parent behavioral test 是否已补。
```

### 验收

```text
SG-11.0.1 REAL-10 ratification file exists.
SG-11.0.2 invalid evidence dir is excluded from all reports.
SG-11.0.3 clean evidence dir is canonical.
SG-11.0.4 stale-parent behavioral test passes or forward OBS exists.
```

---

## Atom 1 — Metric decomposition: structural vs agent economic action

### 新增分类

```rust
enum MarketTxCategory {
    StructuralMarketTx,
    AgentEconomicActionTx,
    ScriptedFixtureTx,
    ResolutionTx,
}
```

### 任务

将当前 report 中的 `market_tx_count` 拆成：

```text
structural_market_tx_count
agent_economic_action_tx_count
scripted_fixture_tx_count
resolution_tx_count
```

### 验收

```text
SG-11.1.1 REAL-10 report can be re-rendered with split market tx categories.
SG-11.1.2 buy_with_coin_router appears under AgentEconomicActionTx only if live non-scripted.
SG-11.1.3 scripted AttemptPrediction fixture does not count as E2.
SG-11.1.4 dashboard clearly separates structural market activity from agent economic action.
```

---

## Atom 2 — Router positive-control fixture

REAL-10 中 `buy_with_coin_router = 0`，所以必须确认：

```text
router action path physically works.
```

### 设计

使用 deterministic scripted agent，不算 E2，只做 wire proof：

```text
active market pool
scripted trader payload
BuyWithCoinRouterTx
audit_tape PROCEED
CTF conserved
PnL updated
```

### 验收

```text
SG-11.2.1 scripted BuyYesWithCoinRouterTx enters L4.
SG-11.2.2 scripted BuyNo / short-equivalent path enters L4 or L4.E with explicit class.
SG-11.2.3 insufficient balance routes L4.E.
SG-11.2.4 missing pool routes L4.E.
SG-11.2.5 CTF conserved.
SG-11.2.6 no ghost liquidity.
SG-11.2.7 no f64 money path.
```

如果这个失败，就不要做 live REAL-6B；先修 substrate。

---

## Atom 3 — Market opportunity visibility audit

目标不是再看 market 是否存在，而是看：

```text
Agent 在其 turn 时是否真正有 actionable opportunity。
```

新增：

```rust
MarketOpportunityTrace {
    agent_id,
    role,
    task_id,
    head_t,
    visible_markets,
    actionable_markets,
    available_balance,
    router_available,
    reason_if_no_actionable_market,
}
```

### 验收

```text
SG-11.3.1 Every Trader turn has MarketOpportunityTrace.
SG-11.3.2 If actionable_markets = 0, NoTradeReason must not be generic.
SG-11.3.3 If actionable_markets > 0 and no trade, record NoPerceivedEdge / AgentDeclined / RiskCap / Balance / PromptBudget.
SG-11.3.4 Opportunity trace derives from ChainTape + CAS.
```

这一步会回答：

```text
Agent 不交易是因为没机会，还是有机会但没行动？
```

---

## Atom 4 — PnL / incentive visibility check

交易不发生，可能是 Agent 看不到收益后果。

### 任务

确保 TraderView 中明确包含：

```text
available balance
open positions
realized PnL
unrealized PnL
risk cap
bankruptcy/autopsy summary
```

### 验收

```text
SG-11.4.1 Trader PromptCapsule includes PnL summary.
SG-11.4.2 PnL summary derives from ChainTape fold, not sidecar.
SG-11.4.3 Below-risk-cap agents cannot execute risky market action.
SG-11.4.4 Low-balance status appears as signal, not as hidden state.
```

---

## Atom 5 — Live E2 micro-probe without REAL-6B

在不进入 live AttemptPrediction 的情况下，先测：

```text
TaskOutcomeMarket + PnL-visible Trader + actionable opportunity
```

运行：

```text
3–5 tasks
same pinned config
market enabled
TraderView with PnL
no forced trade
```

### 验收

```text
SG-11.5.1 If live non-scripted BuyWithCoinRouterTx occurs: E2 achieved.
SG-11.5.2 If no buy occurs: every no-buy turn has MarketOpportunityTrace + NoTradeReason.
SG-11.5.3 No forced trade.
SG-11.5.4 No price-as-truth.
SG-11.5.5 audit_tape PROCEED.
```

---

## Atom 6 — Decision gate

根据 Atom 5 决定：

### 分支 A：出现 live buy

```text
E2 achieved.
进入 REAL-12：role differentiation / E3。
```

### 分支 B：无 live buy，但机会存在

```text
Agent sees actionable market but abstains.
进入 Trader objective redesign:
  role objective stronger,
  PnL prompt more explicit,
  maybe risk-adjusted return prompt,
  still no forced trade.
```

### 分支 C：无 live buy，因为无 actionable market

```text
进入 event timing redesign:
  prepare live REAL-6B Class-4 packet.
```

### 分支 D：router positive-control 失败

```text
回到 substrate fix。
```

---

# 7. 是否现在授权 live REAL-6B？

我的裁决：

```text
暂不授权。
```

但是，我现在比之前更接近授权它，因为 REAL-10 说明：

```text
TaskOutcomeMarket 增加结构性 market tx，
但没有产生 live buy。
```

如果 REAL-11 Atom 3–5 证明：

```text
Agent 没有 actionable market window
```

那么 live REAL-6B 就成为合理下一步。

但在这之前，先要回答：

```text
router path 本身能不能 live？
Trader turn 有没有 actionable market？
Trader 是否看到 PnL / risk？
```

---

# 8. 如果要做更大 benchmark，应该什么时候？

在 REAL-11 前，不建议做 30 tasks per arm。

原因：

```text
当前最大未知不是统计误差，而是机制缺口。
```

等 REAL-11 结束后：

```text
如果 E2 achieved:
  进入 REAL-12 / REAL-8Y，扩大样本。

如果 E2 not achieved:
  扩大 REAL-8 不会解决问题。
  应先做 live REAL-6B 或 objective redesign。
```

---

# 9. 对上线策略的更新

## 9.1 当前可以对内发布的状态

可以称：

```text
TuringOS Research Alpha:
  constitutional ChainTape substrate
  role scaffold
  lawful market-pressure experimental framework
  controlled A/B benchmark harness
```

不能称：

```text
autonomous prediction market
emergent agent economy
market-proven performance improvement
```

## 9.2 进入 Beta 的最低门槛

至少需要：

```text
E1 solid
Router positive-control green
AgentEconomicActionTx path proven
NoTradeReason / MarketOpportunityTrace complete
No overclaim docs
```

若要称“agent economy beta”，还需要：

```text
E2 at least once under live non-scripted condition.
```

若要称“emergent market beta”，还需要：

```text
E3 persistent role differentiation.
```

---

# 10. 给 AI coder 的直接指令

```text
Architect verdict after REAL-10:

1. Ratify REAL-10 narrowly:
   clean, pinned, audited market evidence;
   E1 satisfied for B/C/D;
   E2/E3/E4 not achieved.

2. Do not run REAL-8Y 30-task expansion yet.
   Bigger sample is not the next bottleneck.
   The bottleneck is buy_with_coin_router = 0.

3. Open REAL-11: Agent Economic Action Activation.

Atoms:
0. REAL-10 ratification + evidence hygiene.
1. Split market_tx_count into:
   StructuralMarketTx,
   AgentEconomicActionTx,
   ScriptedFixtureTx,
   ResolutionTx.
2. Router positive-control fixture:
   scripted BuyWithCoinRouterTx must enter L4 and conserve CTF.
3. MarketOpportunityTrace:
   every Trader turn records actionable markets, balance, router availability.
4. PnL / incentive visibility:
   Trader PromptCapsule includes ChainTape-derived balance, PnL, risk cap.
5. Live E2 micro-probe:
   3–5 tasks, no forced trade, attempt live non-scripted router action.
6. Decision gate:
   if live buy -> E2 achieved, proceed REAL-12;
   if opportunity exists but abstain -> objective redesign;
   if no opportunity -> prepare live REAL-6B Class-4 packet;
   if router fixture fails -> substrate fix.

Hard constraints:
- no forced trade
- no price-as-truth
- no ghost liquidity
- no f64 economy
- no off-tape WAL truth
- no private CoT recording
- no raw-log broadcast
- dashboard/materialized view only
```

---

# 11. 最终判断

REAL-10 是一个好节点。它证明了：

```text
市场结构能在 ChainTape 上运行；
受控实验能 cleanly compare arms；
claim boundary 能被守住；
宪法 gates 没有退化。
```

但它也清楚证明：

```text
市场结构活动 ≠ Agent 经济行动。
```

所以不要继续只扩大样本，也不要急着 live REAL-6B。
下一步要先把问题拆到最小：

```text
市场动作路径能否通过 scripted positive control？
Trader turn 是否真的有 actionable market？
Agent 是否看见 PnL / risk？
如果看见仍不交易，为什么？
```

这就是 REAL-11 的意义。
它会把 TuringOS 从“市场结构存在”推进到“Agent 经济行为是否可能”的关键分界点。
```
