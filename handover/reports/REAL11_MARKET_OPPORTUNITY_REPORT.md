# REAL-11 Market Opportunity Report

Date: 2026-05-15

Atom: REAL-11 Atom 3.

## Claim

`MarketOpportunityTrace` is implemented as a pure derived view from `QState`
economic state. It answers whether a Trader turn had an actionable market
without adding a new CAS object type, changing sequencer admission, changing tx
schema, or using stdout as source of truth.

Schema version:

```text
real11.market_opportunity_trace.v1
```

## Gate Coverage

```text
SG-11.3.1 Every Trader turn has MarketOpportunityTrace.
SG-11.3.2 If actionable_markets = 0, NoTradeReason must not be generic.
SG-11.3.3 If actionable_markets > 0 and no trade, record a stable no-trade reason.
SG-11.3.4 Opportunity trace derives from ChainTape + CAS / QState economic fold.
```

Implemented targeted checks:

```text
Trader turn with active pool and balance -> actionable_markets > 0.
Trader turn with no pool -> actionable_markets = 0 and reason NoPool.
Trader turn with low/zero balance -> AmountExceedsBalance, not Unknown.
Trader turn with prompt budget hiding market -> PromptBudgetExceeded.
Every Trader turn fixture has exactly one MarketOpportunityTrace.
Trace contains no raw prompt, raw completion, private CoT, stderr, or raw log.
```

## Boundary

This is not an E2 claim. It is an observability and diagnosis gate for why
`buy_with_coin_router=0` persists.
