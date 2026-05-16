# REAL-12 Role-Specialized Economic Agents — Architect Original

Date: 2026-05-16 UTC

This file preserves the architect ruling supplied by the user for the REAL-12 /
REAL-13 stage. It is the local fact source for the execution plan and
implementation audit.

```text
TuringOS 当前不是宪法落地不足的问题，而是“合宪市场实验已经能跑，但 Agent 自发经济行为仍未被激活”的问题。
换句话说：防御型宪法基座已经基本成熟；生成型经济机制还在寻找正确的激励与事件结构。

所以，下一步不应回到大范围宪法返工，也不应继续简单扩大 REAL-8/REAL-10 同类 A/B。现在要进入一个更明确的阶段：

REAL-12 / REAL-13:
从“市场结构存在”转向“Agent 经济行动触发机制”

1. 对最新报告的裁决

1.1 REAL-10：可以 ratify，但只能窄化

REAL-10 的 clean evidence 可以接受。你给出的结果非常干净：

constitution gates: 461 passed / 0 failed / 1 ignored
workspace tests: exit 0
REAL-8X clean evidence dir: real8x_market_ab_clean_20260515T141331Z
disallowed_config_drift = []

四个 arm 的结论是：

A market disabled:                         market_tx=0,  solve=5/15
B market visible, no TaskOutcomeMarket:    market_tx=10, solve=5/15
C TaskOutcomeMarket enabled:               market_tx=42, solve=6/15
D TaskOutcomeMarket + scripted fixture:    market_tx=38, solve=4/15

这能证明：

E1: market visibility / market structure activity

但不能证明：

E2: spontaneous live agent market action
E3: persistent role differentiation
E4: causal performance improvement

这个结论和你已经写出的 Emergence Metrics 是一致的：E2 必须是 live、non-scripted、agent-generated 的 BuyWithCoinRouterTx 或 short-equivalent；scripted action 不算；role label 不算 E3；small-n descriptive benchmark 不算 E4。

我同意这个边界。
它防止项目从“有市场结构”过度叙事成“市场经济已经涌现”。

1.2 Codex REAL-8 / REAL-9：PROCEED 可接受

Codex 没发现 production defect，并且认可 stale-parent 修复：VerifyTx 构造前刷新当前 q_snapshot().state_root_t，且没有绕过 sequencer admission；同时 VerifyTx signing payload 包含 parent_state_root。

但 Codex 也指出一个非阻塞 gap：

stale-parent fix 的测试仍偏 source-grep / count，
不是 direct behavioral unit test。

如果你已经在 REAL-10 中补了 direct behavioral test，那这个 gap 可以关闭。
如果还没补，它应该是下一步前置原子：

REAL-NEXT-A0:
  direct stale-parent behavioral test

1.3 REAL-11：PROCEED，但证明的是“路径已清楚”，不是 E2 已成功

Codex REAL-11 Final Implementation Review R2 的结论是 PROCEED，且 R1 的两个 CHALLENGE 已关闭：

Class-4 ratification 已进入 final Harness manifest；
traceability row 指向 patched canonical evidence；
router positive-control 是 scripted-not-E2；
patched E2 micro-probe live_real6b_enabled=false、no scripted buys、buy_with_coin_router=0、E2 NOT ACHIEVED。

所以我的裁决是：

REAL-11 = PROCEED as diagnostic and substrate clarification.
REAL-11 ≠ E2 achieved.

它证明：

1. router scripted positive-control 能跑；
2. Class-4 process 清楚；
3. patched E2 micro-probe 没有假装 scripted buy；
4. live non-scripted buy 仍未出现。

这意味着我们已经排除了一个重要可能：

“是不是 router wiring 根本不能用？”

答案是：不是。
router positive-control 能产生 buy_with_coin_router=6，说明底层路径能跑。
真正问题是：

live Agent 不选择走这条路径。

2. 宪法实现完整度审计

2.1 宪法主干已经相当完整

Constitution Execution Matrix 的定义本身很严格：GREEN 必须是有真实 invariant test 且通过；只有文档或审计不算 landed。

目前 Art. 0.1 / Art. 0.2 等最核心的图灵机原教旨条款已经是 GREEN：四元素映射、Tape Canonical、no parallel ledger、no global Markov pointer、Wave 3 真实负载绑定都已经进入执行证据。

这说明：

TuringOS 的 tape-first / predicate-first / dashboard-not-source-of-truth 主体已经落地。

宪法本身也明确：顶层白盒不是 micromanagement 的全知独裁者，而是负责信号的量化、广播、屏蔽；人类工程师的核心价值是设计让 Agent 可靠工作的环境，而不是继续微观写代码。

所以我不会再说“宪法没有落地”。
更准确是：

宪法防御面已经强；
宪法生成面仍需要更深入实现。

2.2 宪法深层还没有完全释放：广播与角色分工

当前系统主要把宪法理解为：

不能作弊
不能 ghost liquidity
不能 price-as-truth
不能 dashboard-as-source-of-truth
不能 off-tape

这些都对。

但宪法还包含另一面：

量化后的信号要选择性广播；
屏蔽要服务于角色分工；
价格是统计信号；
Agent 的生成要被环境诱导，而不是被 prompt 教训。

现在市场机制不 emerging，不是因为宪法太严，而是因为 信号广播给了不合适的角色，或者给得太晚。

REAL-4 / REAL-5 已经说明，prompt-only activation 基本走到头了：market context 可见时能产生 no-trade trace，但多轮 prompt 变体没有产生 live trade。

这意味着：

“让 proof solver 看见价格”不是足够条件。

下一步必须让：

Trader / Challenger / Verifier / MarketMaker

成为真正的制度角色，而不仅是 prompt 标签。

3. 项目整体进展判断

我现在对项目的整体评价是：

1. Constitution substrate: 强。
2. ChainTape / CAS / L4 / L4.E: 强。
3. Role scaffold: 已有实质基础。
4. Market substrate: 已经结构性可运行。
5. Benchmark discipline: 明显成熟。
6. Live agent economic action: 尚未出现。
7. Emergent role differentiation: 尚未出现。
8. Market performance gain: 尚未证明。

REAL-5S → REAL-9 报告也明确说，当前最强的准确 claim 是：TuringOS 有 chain-backed role scaffold 和 lawful market-pressure substrate，而不是 spontaneous market emergence 已经被证明。

这就是我现在的总体判断。

4. 当前真正的瓶颈

当前瓶颈不是：

1. 没有市场结构。
2. 没有宪法 gate。
3. 没有 router 路径。
4. 没有 role scaffold。

这些都基本有了。

当前瓶颈是：

Agent 没有 live economic motive 去主动触发 router。

根据你提供的信息，REAL-11 已经证明：

scripted router positive-control works;
live non-scripted E2 micro-probe still buy_with_coin_router=0;
E2 NOT ACHIEVED.

所以现在必须承认：

市场结构和工具可用，不等于 Agent 会把它当成目标函数。

这和 v3 的经验形成强对比。
v3 的有用火花是短反馈回路：

LLM action
-> wallet pressure
-> market price
-> prompt / scheduler visibility
-> OMEGA settlement
-> memory / PnL feedback

这个 feedback loop 在 v3 中确实产生了市场活动，例如 OMEGA_v3chat_N3_50k 有 278 executed buys、127 markets、149 YES buys、129 NO buys、199 cross-agent buys。

但 v3 的实现有大量宪法风险：f64、off-tape、forced investment、creator auto-long、system MM free injection 等。Architect Brief 明确建议 v4 只吸收 v3 的 pressure loop，不复制 v3 钱包或市场代码。

5. 对“是否强制设立纯投资 / 做空 Agent”的独立判断

5.1 可以强制设立角色

我支持：

在 Agent 接入时，由系统强制分配角色：
  Solver
  Trader
  ShortTrader / Bear
  Verifier
  Challenger
  MarketMaker
  Observer

这不违反宪法。
因为角色分配是白盒制度，是环境设计的一部分。

宪法并没有说所有 Agent 必须自由选择所有工具。相反，宪法强调顶层白盒要通过量化、广播、屏蔽来设计环境。

所以：

角色强制 = 合宪
行为强制 = 需要小心

5.2 不能把“强制投资”当作自发市场证据

我不支持：

强制每个 Trader 必须买 / 必须做空，
然后把这个算作 E2 spontaneous emergence。

Constitution Tension Register 已经把 forced investment 标为 Red Track：它可能点燃流动性，但会把 harness action 冒充成 agent autonomy，不能作为 ship claim。

所以我的裁决是：

可以强制设立 Trader / ShortTrader 角色；
不可以强制其必须交易并计为 E2。

5.3 正确做法：强制角色，允许 abstain，但要求结构化理由

Trader 角色必须执行：

观察市场
评估 edge
输出 Buy / Short / Abstain

但不强迫 Buy / Short。

这意味着 Trader 的每一轮必须产生：

MarketDecisionTrace

其中 action 可以是：

BuyYes
BuyNo / Short
Pass
Abstain

但如果 Pass / Abstain，必须给出结构化 reason：

NoPerceivedEdge
NoActionableMarket
InsufficientBalance
RiskCapExceeded
LiquidityTooLow
ExpectedValueNegative
PromptBudgetExceeded
UnresolvedOracleRisk

这才是合宪的 Drucker / Hayek 版本：

不是强迫交易；
而是强迫经济判断可观测。

5.4 可以设立纯投资和纯做空 Agent

我建议下一步正式加入两个角色：

TraderLong
TraderShort

或更清楚：

BullTrader
BearTrader

但定义必须是：

BullTrader:
  被授权寻找正 EV long / YES 机会；
  允许 buy yes；
  允许 abstain；
  不允许 submit proof / verify / challenge unless explicitly allowed。

BearTrader:
  被授权寻找正 EV short / NO 机会；
  允许 buy no / short；
  允许 abstain；
  不允许 submit proof / verify unless explicitly allowed。

这不是 forced trade。
这是 forced role specialization。

我支持这样做。

6. 下一步方案：REAL-12 Role-Specialized Economic Agents

我建议现在不要继续泛化 REAL-11，而是开：

REAL-12 — Role-Specialized Economic Agents

目标：

把 “Trader” 从可选工具用户，升级成有明确职责、视图、预算和输出义务的经济角色。

7. REAL-12 原子计划

Atom 0 — REAL-11 窄化 ratification

产物：

handover/directives/2026-05-15_REAL11_NARROW_RATIFICATION.md

内容：

Ratified:
  router positive-control works
  patched E2 micro-probe clean
  E2 not achieved
  no live REAL-6B approval
  no forced trade
  no price-as-truth

验收：

SG-12.0.1 REAL-11 ratification file exists.
SG-12.0.2 E2 NOT ACHIEVED explicitly recorded.
SG-12.0.3 scripted router positive-control not counted as E2.

Atom 1 — Role specialization schema

新增 / 扩展：

enum AgentRole {
    Solver,
    Verifier,
    Challenger,
    BullTrader,
    BearTrader,
    MarketMaker,
    Observer,
}

规则：

BullTrader allowed:
  BuyYesWithCoinRouterTx
  MarketDecisionTrace
  Abstain

BearTrader allowed:
  BuyNoWithCoinRouterTx / short-equivalent
  MarketDecisionTrace
  Abstain

Solver allowed:
  WorkTx
  Proof attempts
  maybe read market summary only

Verifier allowed:
  VerifyTx

Challenger allowed:
  ChallengeTx

验收：

SG-12.1.1 role assignment replayable from Genesis/BatchManifest.
SG-12.1.2 BullTrader cannot submit WorkTx.
SG-12.1.3 BearTrader cannot submit VerifyTx.
SG-12.1.4 Solver cannot submit BuyRouter unless explicitly allowed.
SG-12.1.5 Illegal role action routes L4.E RoleActionNotAllowed.

Atom 2 — Role-specific views

BullTraderView

YES price
task outcome YES market
node survive YES market
available balance
PnL
risk cap
liquidity
deadline / budget remaining

BearTraderView

NO price
candidate weakness signals
unsolved task risk
challenge status
failed attempts
market depth
available balance
PnL

SolverView

Lean goal
local proof context
limited price summary only
no full market dashboard

验收：

SG-12.2.1 BullTraderView includes long-side opportunities.
SG-12.2.2 BearTraderView includes short-side opportunities.
SG-12.2.3 SolverView does not include excessive market noise.
SG-12.2.4 raw logs / private CoT not included.
SG-12.2.5 PromptCapsule stores view hash + read set.

Atom 3 — Mandatory economic judgment, not mandatory trade

每个 Bull/Bear Trader turn 必须输出：

EconomicJudgment {
    agent_id,
    role,
    visible_markets,
    chosen_market: Option<EventId>,
    intended_side: Option<YesNo>,
    intended_amount: Option<MicroCoin>,
    action: Buy | Short | Abstain,
    reason: EconomicReason,
    prompt_capsule_cid,
}

允许：

Abstain

但必须说明 reason。

验收：

SG-12.3.1 every BullTrader turn has EconomicJudgment.
SG-12.3.2 every BearTrader turn has EconomicJudgment.
SG-12.3.3 Abstain has structured reason.
SG-12.3.4 EconomicJudgment is CAS-backed and ChainTape-anchored.
SG-12.3.5 no private CoT included.

Atom 4 — Positive-control with role specialization

用 scripted BullTrader / BearTrader 证明路径：

BullTrader scripted buy yes
BearTrader scripted buy no / short

验收：

SG-12.4.1 scripted BullTrader BuyYes enters L4.
SG-12.4.2 scripted BearTrader BuyNo/short enters L4 or explicit L4.E if no route.
SG-12.4.3 PnL / position changes derived from ChainTape.
SG-12.4.4 CTF conserved.
SG-12.4.5 no ghost liquidity.

Atom 5 — Live role-specialized micro-probe

运行：

3–5 tasks
roles:
  1 Solver
  1 BullTrader
  1 BearTrader
  1 Verifier
  1 Challenger
market enabled
no forced trade

验收：

SG-12.5.1 BullTrader produces EconomicJudgment.
SG-12.5.2 BearTrader produces EconomicJudgment.
SG-12.5.3 At least one Trader sees actionable market.
SG-12.5.4 If no live buy/short, reason distribution is specific, not generic.
SG-12.5.5 If live buy/short appears, E2 achieved.
SG-12.5.6 audit_tape PROCEED.

Atom 6 — Decision gate

如果出现 live non-scripted buy/short

E2 achieved.
进入 REAL-13：role differentiation / E3。

如果无 live buy，但 EconomicJudgment 显示 no perceived positive EV

说明模型没有足够交易判断能力或市场 event 没有 edge。
进入 REAL-13A：Trader objective / expected value scaffolding。

如果无 actionable market

进入 REAL-13B：Event Timing Redesign / live REAL-6B ratification。

如果 positive-control 失败

回 substrate fix。

8. 是否需要设立 MarketMaker Agent？

我建议：

可以设立，但不要在 REAL-12 中作为主路径。

原因：

MarketMaker 负责流动性，不负责方向性判断。

如果先加入 MarketMaker，可能把结构性 market_tx_count 继续提高，但仍然无法证明 E2。

所以：

REAL-12 先做 Bull/Bear Trader。
REAL-13 再做 MarketMaker。

9. 是否需要强制 long / short？

我的最终立场：

强制角色：YES。
强制经济判断：YES。
强制交易行为：NO，除非 unsafe research。

形式化：

Allowed:
  “You are BearTrader. You must evaluate short opportunities and output BuyNo or Abstain with reason.”

Forbidden as ship evidence:
  “You must buy NO every turn.”

如果要研究强制交易，可以开：

REAL-RED forced-investment research harness
TURINGOS_UNSAFE_RESEARCH=1

但其结果不得用于 E2 / autonomy claim。
Constitution Tension Register 已明确：forced investment can ignite liquidity, but cannot prove autonomy and belongs to unsafe research, not ship claim. 

10. 对整体方案的更新

我现在建议路线改为：

REAL-11:
  completed; diagnostic says router path works, E2 absent.

REAL-12:
  role-specialized economic agents:
    BullTrader / BearTrader
    mandatory EconomicJudgment
    no forced trade

REAL-13:
  if E2 appears:
    role differentiation / E3
  if E2 absent:
    event timing / live REAL-6B or expected-value scaffolding

REAL-14:
  larger A/B benchmark with E2/E3-specific arms

REAL-15:
  whitepaper / beta launch synthesis

11. 给 AI coder 的直接指令

Architect verdict:

REAL-11 PROCEED accepted.
But E2 is still NOT achieved.

Do not simply rerun larger REAL-8.
Do not force trades as ship evidence.

Open REAL-12: Role-Specialized Economic Agents.

Core decision:
- Assign explicit roles at agent admission.
- Add BullTrader and BearTrader roles.
- Force economic judgment, not economic action.
- Buy/Short must remain voluntary.
- Abstain is allowed but must have structured reason.

Atoms:
0. REAL-11 narrow ratification.
1. Role specialization schema.
2. BullTraderView / BearTraderView / SolverView.
3. Mandatory EconomicJudgment per Trader turn.
4. Scripted Bull/Bear positive-control.
5. Live role-specialized micro-probe.
6. Decision gate.

Ship gates:
- BullTrader cannot submit WorkTx.
- BearTrader cannot submit VerifyTx.
- Illegal role action -> L4.E.
- Every Bull/Bear turn has EconomicJudgment.
- Abstain has structured reason.
- Scripted buy yes/no works.
- Live run either achieves E2 or gives specific reason why not.
- No forced trade.
- No price-as-truth.
- No ghost liquidity.
- No f64 money.
- No private CoT.
- Dashboard is materialized view only.

If no live buy/short:
- If actionable market exists and agents abstain:
    improve expected-value scaffolding.
- If no actionable market exists:
    prepare live REAL-6B Class-4 packet.
- If substrate fails:
    fix router/economic substrate.

12. 最终判断

我现在可以确认我们的总方案方向是对的：

1. 宪法防御基座已经够强；
2. market substrate 已经能跑；
3. router positive-control 证明路径可用；
4. live E2 没出现说明“Agent 经济角色”还不够具体。

所以不要推翻宪法，也不要强制交易来制造假涌现。
正确下一步是：

强制设立经济角色，强制其做经济判断，但不强制其交易。

这最符合宪法深层含义：

顶层白盒设计角色、视图、工具、预算、谓词；
中层黑盒在制度边界内做选择；
底层白盒记录选择、执行后果、结算损益。

这才是 TuringOS 的反奥利奥式市场智能。
```
