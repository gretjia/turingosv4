# DECISION — REAL-6B AttemptPredictionMarket Sealed Oracle

Date: 2026-05-15

Directive:

```text
REAL-6B — AttemptPredictionMarket

目标

让 candidate proof 在 Lean final resolution 前有一个短期预测市场。

Sealed Oracle 流程
SubmitCandidateTx
-> AttemptPredictionMarket opens
-> exactly K logical tape ticks for Trader / Verifier / Challenger
-> MarketCloseTx
-> OracleResolveTx executes Lean result
```

Current-stage limit:

```text
REAL-6B = design + scripted fixture only.
No live real-LLM ship until explicit Class-4 ratification.
```

## Decision

REAL-6B in this atom implements a deterministic scripted fixture and design
record only. It does not introduce production `SubmitCandidateTx`,
`MarketCloseTx`, or `OracleResolveTx` typed transaction discriminants. It does
not change sequencer admission, canonical signing payloads, wallet semantics,
Lean verification, or live LLM scheduling.

The fixture records the future sealed-oracle order:

```text
SubmitCandidate
AttemptPredictionMarketOpen
K logical role-window ticks
MarketClose
OracleResolve
```

The role-window ticks are deterministic tape ticks, not wall-clock sleeps.
Trader, Verifier, and Challenger scripted actions are marked ChainTape-visible.
Lean oracle result is absolute truth; observed price is a signal only and does
not affect verification.

## Forward Boundary

Moving from scripted fixture to live real-LLM or production tx wire requires a
future explicit Class-4 ratification because it would define real market event
timing, close semantics, and oracle resolution semantics.

## Gates

```text
SG-6B.1 No sleep-based artificial blocking.
SG-6B.2 K logical tape ticks are deterministic and replayable.
SG-6B.3 Lean oracle remains absolute truth.
SG-6B.4 MarketCloseTx happens before OracleResolveTx.
SG-6B.5 Trader actions during window are ChainTape-visible.
SG-6B.6 Price does not affect verification.
SG-6B.7 No ghost liquidity.
```
