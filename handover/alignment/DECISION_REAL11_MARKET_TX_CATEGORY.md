# DECISION: REAL-11 Market Tx Category Split

Date: 2026-05-15

## Decision

REAL-11 separates market activity into four categories:

```text
StructuralMarketTx
AgentEconomicActionTx
ScriptedFixtureTx
ResolutionTx
```

This prevents `market_tx_count` from being misread as live agent economic
action.

## Category Rules

`StructuralMarketTx` includes system-created market infrastructure such as
`MarketSeedTx` and `CpmmPoolTx`.

`AgentEconomicActionTx` requires all of:

```text
live agent-generated action
non-scripted
not forced
router or short-equivalent tx
PromptCapsule or MarketDecisionTrace provenance
ChainTape/CAS anchor
audit PROCEED
```

`ScriptedFixtureTx` includes scripted fixtures and missing-provenance router
actions. Scripted actions do not satisfy E2.

`ResolutionTx` includes settlement or oracle-resolution activity such as
`EventResolveTx`, `MarketCloseTx`, and `OracleResolveTx`.

## Claim Boundary

REAL-10 proves structural market activity, not spontaneous market emergence.
The clean REAL-10 evidence has `buy_with_coin_router=0 in all arms`, so
`AgentEconomicActionTx` is zero for REAL-10.

Forbidden overclaims:

```text
forbidden: autonomous prediction market
forbidden: emergent agent economy
forbidden: market-proven performance improvement
forbidden: real-world readiness
forbidden: model ranking
forbidden: live REAL-6B approval
forbidden: price-as-truth
```
