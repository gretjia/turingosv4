# Emergence Metrics E1/E2/E3/E4

Source:
`handover/directives/2026-05-15_REAL10_CONTROLLED_MARKET_EVIDENCE_EXPANSION_ARCHITECT_ORIGINAL.md`

Purpose: prevent narrative drift when market activity increases but spontaneous
market behavior or causal performance gain remains unproven.

## E1 — Market Visibility

Definition:

```text
Agent sees market context.
MarketDecisionTrace / NoTradeReason is tape-visible.
```

REAL-2 / REAL-4 / REAL-5S style evidence may satisfy E1 when the market context
and the agent decision or abstention reason are reconstructable from ChainTape +
CAS.

## E2 — Spontaneous Market Action

Definition:

```text
At least one live, non-scripted, agent-generated BuyWithCoinRouterTx
or short-equivalent.
Must be ChainTape/CAS visible.
No forced trade.
No scripted action.
Audit PROCEED.
```

Non-qualifying evidence:

```text
scripted REAL-7 buys
scripted AttemptPrediction fixtures
prompt-only command that forces a trade
dashboard-only market counters
stdout-only market counters
```

## E3 — Persistent Role Differentiation

Definition:

```text
At least two roles show persistent, distinct action distributions across tasks.
At least one role has nonzero market or verification/challenge behavior.
Derived from ChainTape/CAS, not prompt labels.
At least two consecutive tasks or batches preserve role identity and behavior pattern.
```

Non-qualifying evidence:

```text
role labels only
single-task role diversity index
prompt-assigned role names without behavior divergence
dashboard-only categorization
```

## E4 — Causal Performance Signal

Definition:

```text
Market-enabled condition shows statistically meaningful difference in PPUT,
solve rate, cost, wasted attempts, or verification latency under pinned inputs.
```

Minimum requirements:

```text
same problem set
same model assignment
same budgets
same timeout
same max_tx
same seed/config
only arm toggles differ
Wilson CI or comparable statistical support for the claimed performance signal
```

Non-qualifying evidence:

```text
small-n descriptive benchmark
market_tx_count increase alone
scripted market action alone
manual cherry-pick of successful tasks
```

## Report Rules

```text
Scripted actions cannot satisfy E2.
Role labels alone cannot satisfy E3.
Small-n descriptive evidence cannot claim E4.
Activity increase alone is not emergence.
Waste reduction may be reported as an information-efficiency signal only if it is
derived from ChainTape/CAS and clearly separated from causal solve-rate claims.
```
