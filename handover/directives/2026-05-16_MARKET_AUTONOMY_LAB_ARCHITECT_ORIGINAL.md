# Market Autonomy Lab Architect Original

This file preserves the architect source text provided by the user for the
Market Autonomy Lab. It is intentionally a source archive, not an execution
claim, ship claim, or evidence report.

## Verbatim Architect Source

## 总裁决

我建议现在**允许 Codex 在独立 worktree 中进行大胆的 agent 市场经济激发实验**，但必须分成两条严格隔离的轨道：

```text
A. Constitutional Research Track
   合宪研究轨道。
   可作为 TuringOS 正式路线的候选证据。

B. Unsafe Red-Track Sandbox
   不合宪或宪法张力机制的隔离研究轨道。
   只允许作为研究，不允许进入 ship claim / E2 / E3 / E4 证据。
```

你问“是否可以强制设立纯投资和做空的 Agent”，我的判断是：

```text
可以强制设立角色。
不可以强制交易，并把交易算作自发市场涌现。
```

也就是说：

```text
允许：
  Agent 接入时被分配为 BullTrader / BearTrader / MarketMaker / Verifier / Solver。
  BullTrader 只能做 long-side economic judgment / buy yes / abstain。
  BearTrader 只能做 short-side economic judgment / buy no / abstain。
  每一轮必须输出结构化 EconomicJudgment / EVDecisionTrace。

不允许作为合宪 ship evidence：
  “你必须买 YES”
  “你必须买 NO”
  “每轮必须下注”
  然后把它统计为 E2 spontaneous market action。
```

这与 Constitution Tension Register 一致：forced investment 可以激活流动性，但会把 harness action 伪装成 agent autonomy，因此只能作为 unsafe research，不能作为 ship claim。

---

# 1. 当前事实判断

## 1.1 REAL-BCAST-1 已经从 VETO 修复到可用，但 claim 必须窄化

最初 Codex 对 REAL-BCAST-1 给了 VETO，因为 `evaluator.rs` 是 Trust-Root pinned 文件，实际 hash 与 `genesis_payload.toml` 中的 pinned hash 不一致，A-arm runner 在结论性执行前退出，Trust Root unit gate 也失败。这意味着没有 Class-4 rehash 就不能 ship。你随后补充的 R2 资料显示，已经完成 explicit Class-4 Trust Root rehash，Trust Root verify 通过，REAL-BCAST-1 现在有 clean implementation/evidence path。

但 R2 也指出一个重要限制：REAL-BCAST-1 的 selector source coverage 仍是 MVP 级别，目前主要 ingest `EVDecisionTrace`、`EconomicJudgment`、`LeanResult`、`AttemptTelemetry`，尚未完整 ingest `EvidenceCapsule`、`MarkovEvidenceCapsule`、L4.E public summaries、`MarketDecisionTrace`、`NoTradeReasonTrace` 等。

所以现在可以说：

```text
REAL-BCAST-1 broadcast substrate works.
不能说：
  完整 broadcast layer 已经覆盖所有证据源。
```

## 1.2 hard10 stress 暴露了真正瓶颈

REAL-BCAST-1 hard10 stress 的结论非常重要：

```text
harder problems do create:
  multi-append pressure
  partial proof progress
  repeated rejections
  many market review windows
  lots of EVDecisionTrace / LibrarianDigest evidence

but:
  still no live agent economic action
```

该报告的结论是：

```text
REAL-BCAST-1 works.
E2 NOT ACHIEVED.
Weak point: live agents repeatedly classify opportunities as NegativeEV / abstain.
```

也就是说，广播机制开始工作了，但市场行为仍没起来。

## 1.3 现在的核心问题不是“没有市场”，而是“Agent 没有正 EV 行动”

你给的 REAL-13 Market Pressure Probe 显示：

```text
EVDecisionTrace = 106
Bull = 53
Bear = 53
buy_yes = 0
buy_no = 0
abstain = 106
MarketReviewSummary = 106
live_non_scripted_router_tx = 0
E2 NOT ACHIEVED
```

这说明市场视图、EV 结构、广播、角色 turn 都在产生，但 Agent 持续把机会判断为不值得下注。

所以当前瓶颈从：

```text
NoPool
```

升级成了：

```text
NegativeEV / no positive economic action
```

这比早期好，因为系统已经能解释“不交易”了。

---

# 2. 自主实验前必须先修补 / 核查的事项

在让 Codex 自主探索前，我要求先完成以下 preflight。否则它会在不干净状态上跑出不可用证据。

## 2.1 Trust Root 必须完全干净

Codex 必须先执行：

```bash
cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo -- --test-threads=1
```

必须 PASS。

如果失败：

```text
STOP。
不得运行任何真实问题实验。
不得生成 conclusion-bearing evidence。
必须先做 Class-4 Trust Root rehash ratification。
```

原因：REAL-BCAST-1 的第一次 VETO 就是 Trust Root pinned file mismatch。这个不能再犯。

---

## 2.2 REAL-BCAST selector 至少要补 MarketDecision / NoTrade / MarketReview coverage

R2 说 REAL-BCAST-1 MVP 可以 ship，但它的 selector coverage 不完整。对于 autonomous market experiment，至少需要 ingest：

```text
MarketDecisionTrace
NoTradeReasonTrace
MarketReviewSummary
EVDecisionTrace
EconomicJudgment
LeanResult
AttemptTelemetry
L4.E public rejection summaries
```

最低要求：

```text
Market experiment 中出现的“为什么不交易”必须能进入 Librarian digest。
```

否则 Codex 会反复看到价格，却看不到历史上为什么不下注。

这一步可以作为 Codex 的 Atom 0：

```text
BCAST-MARKET-COVERAGE-PATCH
```

Ship gate：

```text
librarian_digest contains market-review / no-trade / EV reason clusters
unknown JSON fails closed
no raw prompt / raw CoT / raw Lean stderr leak
```

---

## 2.3 EVDecisionTrace 必须能区分“没有正 EV”与“有正 EV 但 Agent 不行动”

当前 106 次 EVDecisionTrace 全部 abstain。下一步必须把每个 abstain 分类成：

```text
NegativeEV
EdgeBelowThreshold
LiquidityTooLow
AmountTooSmall
RiskCapExceeded
InsufficientBalance
OracleRiskTooHigh
InsufficientConfidence
PositiveEVIgnored
Unknown
```

尤其要新增：

```text
PositiveEVIgnored
```

如果系统发现：

```text
agent_probability_bps > implied_probability_bps + threshold
且 balance / liquidity / risk cap 都允许
但 Agent abstain
```

那就是 PositiveEVIgnored。

这会直接回答：

```text
市场本身没有正 EV？
还是 Agent 不会执行经济理性？
```

---

## 2.4 必须建立 PolicyTrader baseline

在让 LLM Trader 自由行动前，Codex 必须跑一个非 LLM deterministic baseline：

```text
PolicyTrader:
  if EV > threshold and all risk checks pass -> buy
  else abstain
```

它不算 E2。
它只用于判断：

```text
当前市场配置下是否存在可行动正 EV。
```

如果 PolicyTrader 都不买：

```text
市场参数 / 事件结构没有正 EV。
```

如果 PolicyTrader 买，而 LLM 不买：

```text
LLM 目标函数 / prompt / role view / risk framing 不足。
```

这一步极其关键，否则我们只会继续猜。

---

## 2.5 market_tx_count 必须拆分

每个报告必须区分：

```text
StructuralMarketTx
  MarketSeed
  Pool creation
  EventResolve
  structural setup

AgentEconomicActionTx
  live non-scripted BuyWithCoinRouterTx
  live short-equivalent

ScriptedFixtureTx
  positive controls
  PolicyTrader baseline

ResolutionTx
  EventResolve / settlement
```

之前 REAL-10 已经证明 market_tx_count 上升，但 buy_with_coin_router 仍为 0。不能再用一个 market_tx_count 混淆“市场结构活跃”和“Agent 经济行动”。

---

# 3. 给 Codex 的自主 worktree 实验方案

下面是一份可以直接交给 Codex 的任务书。

---

## Codex Autonomous Market Economy Lab

### Worktree / Branch

```bash
git worktree add ../turingosv4-market-autonomy-lab main
cd ../turingosv4-market-autonomy-lab
git checkout -b codex/market-autonomy-lab-YYYYMMDD
```

### 总目标

```text
在不违反 TuringOS 宪法的前提下，自主探索如何激活 Agent 市场经济行为。
```

### 成功不是只看买入数量

成功分四级：

```text
E1+:
  更高质量市场判断：EV / NoTrade / Librarian broadcast 变得更具体。

E2-candidate:
  出现 live, non-scripted, agent-generated BuyWithCoinRouterTx / short-equivalent。
  必须 pending audit，不能立即 claim E2。

E3-candidate:
  多个角色在连续任务中形成稳定行为差异。

E4-candidate:
  市场条件对 PPUT / solve rate / wasted attempts / verification latency 有统计支持。
```

---

# 4. Codex 可自主探索的设计空间

## 4.1 合宪允许的探索

Codex 可以自主尝试以下内容：

### A. Role assignment 强化

允许设置：

```text
BullTrader
BearTrader
ContrarianTrader
RiskAverseTrader
VerifierTrader
MarketMaker
Solver
Challenger
Observer
```

要求：

```text
角色强制。
交易不强制。
每个经济角色必须输出 EVDecisionTrace / EconomicJudgment。
```

### B. 纯投资 / 纯做空 Agent

允许：

```text
BullTrader:
  allowed tools = buy_yes, abstain

BearTrader:
  allowed tools = buy_no, abstain
```

禁止：

```text
BullTrader must buy every turn.
BearTrader must short every turn.
```

### C. EV threshold 变化

可以测试：

```text
threshold_bps = 0 / 50 / 100 / 250
max_risk_micro = 500 / 1000 / 5000
```

但必须记录：

```text
threshold config
risk config
agent balance
risk cap
```

### D. Explicit PnL scoreboard

可以把 ChainTape-derived PnL 摘要放进 TraderView：

```text
realized_pnl
unrealized_pnl
open_positions
missed_positive_ev_count
risk_cap
balance
```

但不能暴露 PPUT metric 作为 prompt target，因为 runtime metric exposure 有 Goodhart 风险。

### E. Librarian-enhanced trader view

可以用 REAL-BCAST digest 给 Trader 广播：

```text
recent NegativeEV clusters
repeated abstain reasons
market outcome history
EV mistakes
missed positive EV patterns
```

要求：

```text
CAS-derived
role-cropped
no raw CoT
no raw logs
```

### F. Lawful subsidies / rebates

允许研究：

```text
explicit market exploration rebate
```

条件：

```text
rebate comes from explicit MarketMakerBudget / Treasury budget
budget is debited
no ghost liquidity
not counted as spontaneous pure market behavior without disclosure
```

这可以测试“没有交易是否因为交易成本 / liquidity 过低”。

### G. TaskOutcomeMarket parameter variation

允许调整：

```text
liquidity depth
seed size
spread
deadline
risk cap
```

前提：

```text
all collateral-backed
integer math only
no f64/f32
```

---

## 4.2 不允许的探索，除非 Unsafe Red-Track

以下不允许进入合宪实验：

```text
forced trade
price-as-truth
price-driven Lean accept / reject
hidden memory injection
raw CoT broadcast
raw log broadcast
off-tape WAL as truth
f64/f32 money
ghost liquidity
scripted actions counted as E2
```

如果 Codex 想探索它们，必须单独开：

```text
TURINGOS_UNSAFE_RESEARCH=1
```

并且报告必须写：

```text
NOT SHIP EVIDENCE
NOT E2
NOT AUTONOMY CLAIM
```

Constitution Tension Register 已经把 forced investment、price-driven predicate、hidden memory injection、metric exposure 等列为 Red Track research objects，不是 ship claims。

---

# 5. 实验设计

## 5.1 实验分层

### Layer 0 — Preflight

必须先过：

```bash
cargo fmt --all -- --check
cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo -- --test-threads=1
bash scripts/run_constitution_gates.sh
cargo test --test constitution_librarian_selector
cargo test --test constitution_librarian_digest
cargo test --test constitution_librarian_prompt_injection
cargo test --test constitution_real13b_market_review_window
```

### Layer 1 — Positive Controls

必须证明：

```text
PolicyTrader can buy when EV > threshold.
Scripted BullTrader buy yes works.
Scripted BearTrader buy no / short works.
```

这些不算 E2。

### Layer 2 — True Problem Micro-Probe

使用真题：

```text
3–5 MiniF2F problems
roles:
  Solver
  BullTrader
  BearTrader
  Verifier
  Challenger
  MarketMaker optional
```

要求：

```text
no forced trade
live REAL-6B disabled unless separately ratified
no scripted buys counted
```

### Layer 3 — Hard10 Probe

使用 harder true problems：

```text
10 problems
same pinned model assignment
same budget
same max_tx
same timeout
```

目标：

```text
observe whether EV/PnL/BCAST improves E2-candidate likelihood.
```

### Layer 4 — A/B Test

四个 arm：

```text
A: baseline, market visible, no EV scaffold
B: EV scaffold on
C: EV scaffold + Librarian market digest
D: EV scaffold + Librarian + PnL scoreboard + role-specialized traders
```

禁止：

```text
Do not compare to unsafe forced-trade arm unless red-track separated.
```

---

# 6. 真实题测试要求

## 6.1 问题集要求

必须用真实 MiniF2F / Lean problem，不允许 synthetic toy 作为结论证据。

建议：

```text
Micro:
  3–5 problems
  1 easy, 2 medium, 2 hard

Hard10:
  10 problems
  include at least:
    2 easy
    4 medium
    4 hard
```

必须固定：

```text
problem_set_hash
model_assignment_hash
budget_config_hash
prompt_template_hash
runtime_config_hash
```

## 6.2 每个 run 必须产出

```text
audit_tape verdict
audit_dashboard run report
PromptCapsule read_set proof
LibrarianDigest CIDs
EVDecisionTrace counts
MarketOpportunityTrace counts
MarketReviewSummary counts
AgentEconomicActionTx count
StructuralMarketTx count
ScriptedFixtureTx count
NoTradeReason / EVReason distribution
PnL deltas
role activity table
```

## 6.3 E2-candidate 判断

只有满足以下全部条件，才能写：

```text
E2 candidate pending audit
```

条件：

```text
1. live non-scripted BuyWithCoinRouterTx / short-equivalent exists.
2. tx is agent-generated, not PolicyTrader/scripted fixture.
3. PromptCapsule provenance exists.
4. MarketOpportunityTrace shows actionable market.
5. EVDecisionTrace shows positive EV or explicit chosen rationale.
6. audit_tape PROCEED.
7. no forced trade.
8. no price-as-truth.
9. no ghost liquidity.
```

不能直接写：

```text
E2 achieved
```

必须先经过 clean-context audit。

## 6.4 失败也必须产出机制解释

如果没有 E2-candidate，报告必须回答：

```text
No positive EV?
Positive EV ignored?
No actionable market?
Insufficient balance?
Risk cap blocked?
Liquidity too low?
Oracle risk too high?
Librarian digest not delivered?
Prompt budget suppressed market section?
Model parsing failed?
```

---

# 7. Codex 自主实验的评分函数

Codex 不能只优化 “是否买了”。
建议评分：

```text
score =
  3 * live_non_scripted_router_tx_count
+ 2 * positive_ev_detected_count
+ 2 * ev_reason_specificity_score
+ 1 * no_unknown_reason_reduction
+ 1 * market_digest_delivery_rate
+ 1 * trader_turn_coverage
- 5 * any_constitution_violation
- 3 * raw_log_leak
- 3 * ghost_liquidity
- 3 * price_as_truth
- 2 * evidence_contamination
```

但注意：

```text
这个 score 只是研究选择函数，不是 Agent prompt target。
不要把 score 暴露给运行中的 Agent。
```

---

# 8. Codex 的交付物

Codex 必须提交：

```text
1. EXPERIMENT_CHARTER.md
2. EXPERIMENT_MATRIX.md
3. FORBIDDEN_CLAIMS.md
4. PRE_FLIGHT_REPORT.md
5. TEST_RESULTS_SUMMARY.md
6. E2_CANDIDATE_REPORT.md or CLEAN_NEGATIVE_REPORT.md
7. CONSTITUTIONAL_RISK_REGISTER.md
8. NEXT_STEP_RECOMMENDATION.md
```

每个结论必须标注：

```text
evidence dir
audit_tape verdict
config hash
claim boundary
```

---

# 9. 建议给 Codex 的直接 prompt

下面这段可以直接给 Codex：

```text
You are authorized to create an independent worktree for a bold but constitution-preserving market-autonomy experiment.

Goal:
Find whether TuringOS can induce live agent economic action without forced trade, price-as-truth, ghost liquidity, off-tape truth, f64 money, raw CoT, or raw log broadcast.

Worktree:
Create a new branch/worktree from current main:
  codex/market-autonomy-lab-YYYYMMDD

Do not push to main.
Do not edit restricted surfaces unless you stop and request Class-4 ratification.

Preflight:
1. Verify Trust Root passes.
2. Run constitution gates.
3. Run targeted Librarian/market tests.
4. Confirm REAL-BCAST selector includes market-review / no-trade / EV reason inputs or patch that first.
5. Confirm unknown schemas fail closed.
6. Confirm no raw prompt/completion/CoT/stderr appears in digest or prompt.

Allowed experiments:
- Role-specialized agents:
  BullTrader, BearTrader, ContrarianTrader, RiskAverseTrader, MarketMaker, Solver, Verifier, Challenger.
- Force role assignment.
- Force economic judgment.
- Do not force trade.
- Add EV/PnL/Librarian market digest to TraderView.
- Vary EV threshold, risk budget, liquidity depth, market seed, role ordering.
- Run deterministic PolicyTrader baseline, but never count it as E2.
- Explore explicit budget-backed rebates only if debited from MarketMakerBudget and reported separately.

Forbidden in constitutional track:
- forced buys/shorts
- price-as-truth
- ghost liquidity
- f64/f32 money
- off-tape WAL truth
- private CoT recording
- raw-log broadcast
- dashboard/stdout as source of truth
- scripted/PolicyTrader action counted as E2
- live REAL-6B unless separately ratified

Testing:
Run true MiniF2F/Lean problems, not toy-only tests.
Use:
  Micro: 3–5 problems
  Hard10: 10 problems
  A/B arms:
    A baseline market-visible
    B EV scaffold
    C EV + Librarian digest
    D EV + Librarian + PnL scoreboard + role-specialized traders

All arms must pin:
  problem set
  model assignment
  budgets
  timeout
  max_tx
  prompt template hash
  runtime config hash

Metrics:
- E1/E2/E3/E4 classification
- live_non_scripted_router_tx_count
- agent_economic_action_tx_count
- structural_market_tx_count
- scripted_fixture_tx_count
- EVDecisionTrace count
- MarketOpportunityTrace count
- PositiveEVIgnored count
- NoTrade / EVReason distribution
- PnL deltas
- role activity table
- audit_tape verdict
- dashboard regeneration

E2 candidate requires:
  live non-scripted agent-generated BuyWithCoinRouterTx or short-equivalent,
  ChainTape/CAS evidence,
  PromptCapsule provenance,
  MarketOpportunityTrace,
  EVDecisionTrace or explicit economic rationale,
  audit_tape PROCEED,
  no forced trade,
  no price-as-truth,
  no ghost liquidity.

If no E2 candidate:
Produce a clean-negative mechanism report:
  no positive EV,
  positive EV ignored,
  no actionable market,
  insufficient balance,
  risk cap,
  liquidity too low,
  oracle risk,
  broadcast not delivered,
  prompt budget issue,
  parser failure,
  model abstention.

Deliver:
  EXPERIMENT_CHARTER.md
  EXPERIMENT_MATRIX.md
  PRE_FLIGHT_REPORT.md
  TEST_RESULTS_SUMMARY.md
  E2_CANDIDATE_REPORT.md or CLEAN_NEGATIVE_REPORT.md
  CONSTITUTIONAL_RISK_REGISTER.md
  NEXT_STEP_RECOMMENDATION.md
```

---

# 10. 我的最终判断

你现在可以让 Codex 做大胆探索，但不能让它在主线里“乱跑”。

最重要的边界是：

```text
大胆探索 ≠ 违宪探索。
```

当前最有价值的探索不是“强制交易”，而是：

```text
强制角色
强制经济判断
强制证据上 tape
允许不交易
但必须解释不交易
```

如果 Codex 能在这个框架下产生 live non-scripted router tx，那就是非常重要的 E2 candidate。
如果仍然没有，它也会给我们一个高质量 clean-negative：

```text
到底是没有正 EV？
还是 Agent 不会执行正 EV？
还是广播没送到？
还是市场时机仍然不对？
```

这比继续盲目调 prompt 或强行下注要有价值得多。
