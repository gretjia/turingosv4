# REAL-10 Decision Gate Report

This report is the planned Atom 5 report path for REAL-10. The evidence-local
canonical report is:

`handover/evidence/real8x_market_ab_clean_20260515T141331Z/REAL10_DECISION_GATE_REPORT.md`

The earlier directory
`handover/evidence/real8x_market_ab_20260515T134453Z/` is explicitly invalid
for conclusions because it was contaminated by an accidental rerun and is
preserved only as remediation evidence.

## Claim Boundary

REAL-8X CLEAN supports a narrow statement:

```text
pinned, audited market-visible conditions produced more market activity than
market-disabled A, while all arms exited 0 and all audits returned PROCEED.
```

REAL-8X CLEAN does not support:

```text
autonomous market emergence
live non-scripted router / short behavior
persistent role differentiation
market-caused solve-rate / PPUT improvement
live REAL-6B approval
```

## Clean Evidence Summary

| Arm | Condition | exit/audit/tasks | market_tx_count | buy_with_coin_router | solve_rate |
| --- | --- | ---: | ---: | ---: | ---: |
| A | market disabled | 0 / PROCEED / 15 | 0 | 0 | 5/15 |
| B | market visible, no TaskOutcomeMarket | 0 / PROCEED / 15 | 10 | 0 | 5/15 |
| C | TaskOutcomeMarket enabled | 0 / PROCEED / 15 | 42 | 0 | 6/15 |
| D | TaskOutcomeMarket + scripted AttemptPrediction fixture | 0 / PROCEED / 15 | 38 | 0 | 4/15 |

`arm_config_manifests/REAL8X_CONFIG_AUDIT.json` records:

```text
disallowed_config_drift=[]
```

## E1/E2/E3/E4

```text
E1: satisfied for B/C/D market-visible arms.
E2: not achieved; buy_with_coin_router=0 for all arms and D is scripted.
E3: not established; role_diversity_index alone is insufficient.
E4: not established; Wilson intervals overlap and evidence remains descriptive.
```

## Next Gate

The next step is not a live REAL-6B ship. If pursuing E2, prepare a separate
Class-4 live REAL-6B ratification packet. If pursuing performance evidence, run
a larger clean benchmark with the same pinned-input discipline and no scripted
actions counted toward E2.
