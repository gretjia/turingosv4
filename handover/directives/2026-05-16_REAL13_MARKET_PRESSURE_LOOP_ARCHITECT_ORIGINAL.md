# REAL-13 Market Pressure Loop Architect Original

This file preserves the architect/user source for the REAL-13 market-pressure
loop package. It is the local directive source for implementation and audit.

## Source A — Market Pressure Loop Ruling

```text
v4 市场不涌现，不是因为 LLM 不够强，也不是 CPMM 数学错，而是 v4 在修复 v3 宪法漏洞时，把 v3 中真正激发市场行为的“经济压力 + 认知低摩擦 + 即时反馈”一起削弱了。
```

Architect correction:

```text
v4 的宪法基座是正确的；
但 v4 尚未把“合宪的经济压力回路”设计到 Agent 行为循环中。
```

v3 useful spark:

```text
LLM action
-> wallet pressure
-> market price
-> prompt / scheduler visibility
-> OMEGA settlement
-> memory / PnL feedback
```

Final synthesis:

```text
吸收 v3 的压力，不吸收 v3 的污染。
吸收 Gemini 的认知界面洞察，不吸收它可能导致违宪的强制交易或幽灵流动性。
```

Accepted insights:

```text
1. v4 的 NoPerceivedEdge 是理性结果，不是 bug。
2. v3 起市场靠的是“压力回路”，不是单纯 market code。
3. 纯 Trader / Bull / Bear 角色必须与 Solver 分离。
4. LLM 友好的认知桥梁是必要的。
5. microCoin 大整数会造成行为摩擦。
6. 合法做市商，而不是幽灵流动性。
7. Free cognition / Paid conviction 要明确。
8. Trader prompt 不应该暴露大量合规术语；系统内部仍必须强审计。
9. Buy / Short 不需要长篇自然语言解释，只需 EVDecisionTrace 数值字段 + action；Abstain 必须提供结构化 EVReason。
10. Private Alpha Tool / Private Eval Sandbox 可以私有于其他 Agent，但 ToolResult 必须进入 CAS，审计者可验证。
11. WorkTx.stake 应重解释为 CreatorBond / SubmissionBond / responsibility bond，不作为 voluntary long signal。
```

Rejected:

```text
1. 回到 ghost liquidity。
2. 强制交易作为 E2 证据。
3. price-driven predicate。
```

Final route:

```text
REAL-13A — EVDecisionTrace / Expected-Value Scaffolding.
REAL-13B — Dedicated Market Review Turn.
REAL-13C — Trader UX / Cognitive Bridge.
REAL-13D — Signal Purification: forced stake ≠ voluntary market signal.
REAL-13E — Legal MarketMaker / Liquidity.
REAL-13F — Private Alpha Sandbox.
REAL-13G — Heterogeneous Traders.
REAL-13H — Live Integrated Probe.
REAL-14 — E2 Audit + Role Differentiation.
REAL-15 — live REAL-6B / AttemptPrediction only if still needed.
REAL-RED — Unsafe Research only.
```

Direct AI-coder command:

```text
Architect final verdict:

Absorb Gemini's insights selectively.

Root cause:
v4 lacks v3's dedicated market review loop and trader UX.
v3 produced activity through role pressure, auto markets, wallet feedback, and investment rounds.
Do not copy v3's ghost liquidity, f64, or forced investment.

Proceed:

REAL-13A:
  EVDecisionTrace / Expected-Value Scaffolding.

REAL-13B:
  Dedicated Market Review Turn.
  Bull/Bear must review market windows.
  They must output EVDecisionTrace.
  Buy/Short voluntary.
  Abstain allowed with EV reason.

REAL-13C:
  Trader UX / Cognitive Bridge.
  Use integer bps / percent hints.
  Use DisplayCoin adapter.
  Remove Lean tactic burden from Trader prompt.
  No decimals/f64.

REAL-13D:
  Signal purification.
  WorkTx stake = CreatorBond, not voluntary long.
  E2 ignores forced stake.

REAL-13E:
  Legal MarketMaker.
  Genesis-funded liquidity only.
  Optional legal asymmetric seeding via real trades.
  No ghost liquidity.

REAL-13F:
  Private Alpha Sandbox.
  ToolResult in CAS.
  Visible to audit, optionally shielded from other agents.

REAL-13G:
  Heterogeneous traders.
  >=3 model families.
  no hidden model switch.

REAL-13H:
  Live integrated micro-probe.
  If live buy/short appears, label E2 candidate pending audit.

REAL-14:
  E2 audit + E3 role differentiation.

REAL-15:
  live REAL-6B only if event timing remains bottleneck.

Forbidden:
  no forced trade in ship path
  no price-as-truth
  no ghost liquidity
  no f64/f32
  no private CoT
  no raw-log broadcast
  no off-tape WAL truth
  no model ranking claim
```

## Source B — Async / Barrier Ruling

Architect ruling:

```text
不要一步到位做完全 async。
但也不要做一个未来无法迁移的纯回合制。
最佳方案是：先实现“逻辑回合制的 Market Review Turn”，但底层用 async-ready 架构设计：异步生成，确定性收集，单线程/单序列提交。
```

Name:

```text
Barriered Async Market Review
屏障式异步市场审查
```

Core:

```text
Agent 可以并发思考；
系统必须按逻辑回合收集；
ChainTape 必须按确定顺序提交。
```

Recommended route:

```text
REAL-13B-v1:
  Deterministic Round-Based Market Review

REAL-13B-v2:
  Barriered Async Market Review

REAL-15+:
  Full Async Market Arena
```

REAL-13B-v1 ship gates:

```text
SG-13B-v1.1 每个 active market window 创建 MarketReviewWindow。
SG-13B-v1.2 eligible Bull/Bear 都进入 review turn。
SG-13B-v1.3 每个 review turn 都生成 EVDecisionTrace。
SG-13B-v1.4 Abstain 必须有 EVReason。
SG-13B-v1.5 Buy/Short 使用现有 BuyWithCoinRouterTx。
SG-13B-v1.6 no forced trade。
SG-13B-v1.7 no price-as-truth。
SG-13B-v1.8 no ghost liquidity。
SG-13B-v1.9 no f64/f32。
SG-13B-v1.10 audit_tape PROCEED。
SG-13B-v1.11 若无 live buy/short，报告必须分类：
  all EV negative
  edge below threshold
  positive EV ignored
  risk cap blocked
  liquidity too low
  parser/gateway failed
```

REAL-13B-v2 ship gates:

```text
SG-13B-v2.1 async agent tasks may run concurrently.
SG-13B-v2.2 all responses collected before commit barrier closes.
SG-13B-v2.3 commit order deterministic and replayable.
SG-13B-v2.4 stale parent refresh occurs before router tx submission.
SG-13B-v2.5 pool reserve updates are serialized by sequencer.
SG-13B-v2.6 timeout produces MissingReviewTrace / NoResponseTrace.
SG-13B-v2.7 replay reproduces same tx order.
SG-13B-v2.8 no hidden scheduler state.
SG-13B-v2.9 no dashboard-only evidence.
```

Direct AI-coder command:

```text
Do not implement unrestricted full async now.

Implement Market Review Turn in two layers:

Layer 1:
  Deterministic sequential round semantics.

Layer 2:
  Async-ready architecture with barriered async mode.

Default mode:
  TURINGOS_MARKET_REVIEW_MODE=sequential

Allowed experimental mode:
  TURINGOS_MARKET_REVIEW_MODE=barriered_async

Forbidden for ship path now:
  full async without barrier

Add:
  MarketReviewScheduler
  MarketReviewWindow
  MarketReviewResponse
  MarketReviewSummary

Rules:
  - Bull/Bear must enter review window.
  - They must output EVDecisionTrace.
  - Buy/Short voluntary.
  - Abstain allowed with EV reason.
  - All responses CAS-backed.
  - Router txs serialized by sequencer.
  - Commit order deterministic.
  - Replay must reproduce order.
  - No forced trade.
  - No price-as-truth.
  - No ghost liquidity.
  - No f64/f32.
```
