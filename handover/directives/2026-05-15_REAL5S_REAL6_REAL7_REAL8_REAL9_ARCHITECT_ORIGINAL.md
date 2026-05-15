# REAL-5S / REAL-6 / REAL-7 / REAL-8 / REAL-9 Architect Original

本文件逐字保存用户提供的架构师原文，作为 REAL-5S -> REAL-9 执行方案、审计与后续 Harness 的事实源。

```text
4. 新版最终路线

我把路线统一改成 REAL 命名：

REAL-5S  Scaffold ratification / clean-negative closure
REAL-6   Event Timing & Lawful Pressure
REAL-7   V3-Equivalent Structural Smoke
REAL-8   Formal Market A/B Benchmark
REAL-9   Launch synthesis / Whitepaper update
5. REAL-5S — 收口 REAL-5，不再拖延
目标

正式确认：

REAL-5 proves role scaffolding.
REAL-5 does not prove market emergence.
Atom 5S-A — Scaffold Ratification Report

产出：

REAL5_SCAFFOLD_RATIFICATION_REPORT.md

必须写：

role gateway blocks Trader proof-style leakage
Verifier behavior observed
Trader buy=0
NoPool dominates
No E2/E3 claim
Atom 5S-B — Clean Negative Report

产出：

REAL5_CLEAN_NEGATIVE_NO_TRADE_REPORT.md

必须回答：

Why no trade?
  NoPool dominates.
  Post-accept node market timing too late.
  Prompt-only exhausted.
Ship Gates
SG-5S.1 REAL-5 scaffold claim narrowed.
SG-5S.2 No E2/E3 emergence claim.
SG-5S.3 Role gateway regression tests pass.
SG-5S.4 Clean-negative report filed.
SG-5S.5 Constitution gates no regression.
6. REAL-6 — Event Timing & Lawful Pressure

REAL-6 是核心阶段。

REAL-6A — TaskOutcomeMarket
目标

在任务开始时创建市场：

Event:
  task will be solved within budget/deadline.
为什么先做它

它比 NodeSurviveMarket 更早，比 AttemptPredictionMarket 更安全。

结构
TaskOutcomeEvent {
    event_id,
    task_id,
    deadline_round,
    max_budget,
    created_by_task_open_tx,
}
执行
TaskOpenTx / EscrowLockTx
-> MarketSeedTx for task_outcome event
验收标准
SG-6A.1 TaskOutcomeMarket exists before first WorkTx.
SG-6A.2 TraderView contains active TaskOutcomeMarket.
SG-6A.3 NoPool no longer dominates when task market exists.
SG-6A.4 Scripted trader can Buy YES/NO on TaskOutcomeMarket.
SG-6A.5 Real LLM trader emits MarketDecisionTrace or classified NoTradeReason.
SG-6A.6 EventResolveTx YES if verified proof before budget/deadline.
SG-6A.7 EventResolveTx NO if exhausted/deadline without verified proof.
SG-6A.8 No ghost liquidity.
SG-6A.9 CTF conserved.
SG-6A.10 Price never affects Lean predicate.
REAL-6B — AttemptPredictionMarket
目标

让 candidate proof 在 Lean final resolution 前有一个短期预测市场。

Sealed Oracle 流程
SubmitCandidateTx
-> AttemptPredictionMarket opens
-> exactly K logical tape ticks for Trader / Verifier / Challenger
-> MarketCloseTx
-> OracleResolveTx executes Lean result
验收标准
SG-6B.1 No sleep-based artificial blocking.
SG-6B.2 K logical tape ticks are deterministic and replayable.
SG-6B.3 Lean oracle remains absolute truth.
SG-6B.4 MarketCloseTx happens before OracleResolveTx.
SG-6B.5 Trader actions during window are ChainTape-visible.
SG-6B.6 Price does not affect verification.
SG-6B.7 No ghost liquidity.
当前阶段限制
REAL-6B = design + scripted fixture only.
No live real-LLM ship until explicit Class-4 ratification.
REAL-6C — Conviction Budget / PnL Feedback
目标

恢复 v3 的经济压力，但保持 v4 合宪。

原则
free cognition
paid conviction
结构
ConvictionBudget {
    agent_id,
    available_micro,
    reserved_micro,
    realized_pnl,
    unrealized_pnl,
    risk_cap,
}

但实现必须是：

ChainTape fold / materialized view
not HashMap sidecar
风险限制
below risk cap:
  cannot Trader/Challenger high-risk action
  can still observe/read/abstain/solve/possibly verify
验收标准
SG-6C.1 PnL derived from ChainTape/CAS.
SG-6C.2 No PnL HashMap sidecar source-of-truth.
SG-6C.3 Agent prompt sees scoped PnL summary.
SG-6C.4 Low-balance agent blocked from high-risk market actions.
SG-6C.5 Low-balance agent not erased / reset.
SG-6C.6 AutopsyCapsule generated after significant loss.
REAL-6D — Opportunity Scheduler Observe-Only
目标

价格 / PnL 进入调度观察，但不执行 admission change。

结构
SchedulerDecisionTrace {
    head_t,
    visible_agents,
    visible_nodes,
    price_signals,
    pnl_signals,
    recommended_agent,
    recommended_role,
    recommended_action,
    observe_only: true,
}
验收标准
SG-6D.1 Scheduler trace includes price/PnL signals.
SG-6D.2 observe_only=true.
SG-6D.3 Recommendation does not change sequencer admission.
SG-6D.4 Price does not affect L4/L4.E.
SG-6D.5 Dashboard shows scheduler recommendation as non-binding.
7. REAL-7 — V3-Equivalent Structural Smoke
目标

不是复制 v3 数量，而是重建 v3 的结构压力。

v3 evidence 显示某些 run 产生过非零市场活动，比如 OMEGA_v3chat_N3_50k 的 raw-backed 指标包括 tx_count=436、nodes=127、markets=127 等。
但 v3 的核心价值不是数字本身，而是短反馈环：LLM action → wallet pressure → market price → prompt/scheduler visibility → OMEGA settlement。

最小结构目标
>= 5 agents
>= 3 roles active
>= 3 tasks
>= 1 TaskOutcomeMarket
>= 1 scripted AttemptPredictionMarket
>= 1 BuyYesWithCoinRouterTx
>= 1 BuyNoWithCoinRouterTx or Short equivalent
>= 1 VerifyTx
>= 1 ChallengeTx or NoChallengeReason
>= 1 EventResolveTx
>= 1 PnL delta
>= 1 AutopsyCapsule if loss occurs
Ship Gates
SG-7.1 Structural pattern achieved.
SG-7.2 No forced investment.
SG-7.3 No price-as-truth.
SG-7.4 No ghost liquidity.
SG-7.5 All market actions ChainTape-visible.
SG-7.6 Dashboard regenerates from ChainTape + CAS.
SG-7.7 Clean comparison to v3 metrics without claiming identical equivalence.
8. REAL-8 — Formal Market A/B Benchmark
A/B conditions
A: market disabled
B: market visible, no TaskOutcomeMarket
C: TaskOutcomeMarket enabled
D: TaskOutcomeMarket + scripted AttemptPrediction fixture
Metrics
solve rate
verified PPUT
false accept rate
cost per verified proof
market tx count
NoTradeReason distribution
PnL dispersion
role diversity index
audit failure rate
Ship Gates
SG-8.1 Same problem set across arms.
SG-8.2 Same model assignment.
SG-8.3 Same budgets.
SG-8.4 All runs chain-backed.
SG-8.5 No overclaim of causality.
SG-8.6 Negative result is valid and documented.
9. REAL-9 — Whitepaper / Launch Synthesis

把结果写入：

TuringOS Generative Economy Whitepaper Update
TuringOS Market Developer Manual

必须明确：

v4 does not copy v3.
v4 rebuilds v3's economic pressure under constitution.
price = signal, not truth.
market = role-specific institution, not prompt decoration.
10. 给 AI coder 的最终执行指令
Architect final ruling:

Adopt selected parts of the reviewed opinion.

1. Rename route:
   Use REAL-5S / REAL-6 / REAL-7 / REAL-8 / REAL-9.
   Do not use Phase E0-E7.

2. Accept Event Timing diagnosis:
   NoPool means post-accept node market is too late.
   Stop prompt-only variants.

3. REAL-5S:
   Ratify scaffold completion and clean-negative no-trade report.
   No E2/E3 claim.

4. REAL-6A:
   Implement TaskOutcomeMarket at TaskOpen/EscrowLock.
   Event = task solved before budget/deadline.
   Market exists before first WorkTx.

5. REAL-6B:
   Design AttemptPredictionMarket using Sealed Oracle:
      SubmitCandidateTx
      K logical tape ticks
      MarketCloseTx
      OracleResolveTx
   No sleep.
   Scripted only until Class-4 ratification.

6. REAL-6C:
   Implement ConvictionBudget/PnL as ChainTape-derived view.
   No HashMap sidecar.
   Free cognition, paid conviction.
   Risk-gated role availability, not forced role coercion.

7. REAL-6D:
   Opportunity Scheduler observe-only.
   Price/PnL can be logged as signal.
   No admission change.
   No price-as-truth.

8. REAL-7:
   Run V3-equivalent structural smoke.
   Do not chase v3 tx count.
   Require structural evidence:
      persistent agents,
      roles,
      task outcome market,
      buy/sell or clean-negative,
      verify,
      challenge,
      PnL,
      autopsy.

9. REAL-8:
   Formal market A/B benchmark.

10. Forbidden in all ship claims:
    no forced trades,
    no price-as-truth,
    no ghost liquidity,
    no f64 economy,
    no off-tape WAL as truth,
    no private CoT recording,
    no raw-log broadcast.
11. 最终判断

我认可这份意见的核心：

v3 taught us pressure.
v4 gives us law.
Next phase must build lawful pressure.

但我把它落实为更严格的 v4 方案：

REAL-5S:
  close scaffold and no-trade diagnosis.

REAL-6:
  move market earlier,
  add lawful conviction pressure,
  keep price observe-only.

REAL-7:
  structural v3-equivalent smoke.

REAL-8:
  controlled A/B benchmark.

REAL-9:
  whitepaper / launch synthesis.

真正要吸收的不是“更多交易功能”，而是：

在不突破宪法的情况下，让不确定性、价格、角色、资本和验证，在同一条 ChainTape 上形成持续反馈。
```
