# REAL-11 PnL / Incentive Visibility Report

Date: 2026-05-15

Atom: REAL-11 Atom 4.

## Claim

Trader-visible PnL/risk information is provided by existing chain-derived views:

```text
src/sdk/your_position.rs
src/runtime/agent_pnl.rs
src/runtime/real6_conviction_budget.rs
```

The view includes:

```text
available balance
open positions
realized PnL
unrealized PnL
risk cap
bankruptcy/autopsy summary
```

The source is `ChainTape/CAS-derived QState fold`; no PnL HashMap sidecar is
introduced as source of truth.

## Gate Coverage

```text
SG-11.4.1 Trader PromptCapsule / prompt view includes PnL summary.
SG-11.4.2 PnL summary derives from ChainTape fold, not sidecar.
SG-11.4.3 Below-risk-cap agents cannot execute risky market action.
SG-11.4.4 Low-balance status appears as signal, not hidden state.
```

Targeted tests verify:

```text
rendered Trader view contains balance/PnL/risk/autopsy summary;
rendered values match compute_agent_pnl;
low-balance Trader high-risk market action is blocked;
read/observe remains allowed;
no raw CoT, raw logs, or Lean stderr enter Trader PnL view.
```

## Boundary

This is not a forced-trade mechanism. Cognition remains free; conviction is
risk-gated.
