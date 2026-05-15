# REAL-5S Clean Negative No-Trade Report

Date: 2026-05-15 UTC

Directive:

```text
Atom 5S-B — Clean Negative Report
```

Required question:

```text
Why no trade?
  NoPool dominates.
  Post-accept node market timing too late.
  Prompt-only exhausted.
```

## Summary

Why no trade?

```text
NoPool dominates.
Post-accept node market timing too late.
Prompt-only exhausted.
```

Trader buy=0.

No E2/E3 claim.

## Evidence

Post-VETO runs:

```text
handover/evidence/g_phase_real_5_trader_first_b8_rolegate_20260514T192523Z
handover/evidence/g_phase_real_5_core3_b8_rolegate_20260514T192958Z
```

Both runs have:

```text
audit_tape verdict: PROCEED
buy_with_coin_router=0
```

The trader-first post-VETO run additionally shows that `Agent_0=Trader`
produced proof-style outputs, but those outputs were blocked by the role
gateway and anchored as policy rejections instead of accepted WorkTx/VerifyTx.
That means the remaining no-trade result is no longer explained by hidden
role-permission leakage.

## Mechanism Diagnosis

NoPool dominates.

The overnight ledger records that every successful bounded run with Trader
NoTrade reason traces classified the trader-side no-trade condition as
`NoPool`. The `market K=0` adversary did not turn this into
`PromptBudgetExceeded`; it stayed `NoPool`, which points away from prompt
top-K elision and toward market timing / availability.

Post-accept node market timing too late.

The REAL-5 market surface depends on node-survive markets that appear after
accepted WorkTx. In the bounded true-problem runs, those markets either appear
after the uncertainty has largely collapsed or outside the effective Trader
decision window. This matches the architect's diagnosis that the market must
move earlier:

```text
REAL-6 Event Timing & Lawful Pressure
REAL-6A TaskOutcomeMarket
```

Prompt-only exhausted.

REAL-5 tried role order, market-context size, adversarial problem selection,
and post-VETO role-gateway enforcement. These changed visibility and policy
classification but did not produce live router buys. Continuing prompt-only
variants would not address the dominant `NoPool` mechanism.

## Non-Claims

This report does not claim:

```text
E2: spontaneous trading emergence
E3: role differentiation / market behavior emergence
```

Negative result is valid and documented:

```text
REAL-5 proves role scaffolding.
REAL-5 does not prove market emergence.
```

## Next Step

Proceed to REAL-6 only under the approved Class-4 plan:

```text
REAL-6A — TaskOutcomeMarket
Event:
  task will be solved within budget/deadline.
```
