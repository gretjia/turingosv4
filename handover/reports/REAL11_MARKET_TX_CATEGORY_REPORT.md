# REAL-11 Market Tx Category Report

Date: 2026-05-15

Source evidence:

```text
handover/evidence/real8x_market_ab_clean_20260515T141331Z
```

The contaminated REAL-10 directory
`handover/evidence/real8x_market_ab_20260515T134453Z` is invalid for
conclusions and is excluded here.

## Re-rendered Split

| Arm | Condition | structural_market_tx_count | agent_economic_action_tx_count | scripted_fixture_tx_count | resolution_tx_count | buy_with_coin_router |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| A | market disabled | 0 | 0 | 0 | 3 | 0 |
| B | market visible, no TaskOutcomeMarket | 10 | 0 | 0 | 3 | 0 |
| C | TaskOutcomeMarket enabled | 42 | 0 | 0 | 12 | 0 |
| D | TaskOutcomeMarket + scripted AttemptPrediction fixture | 38 | 0 | 15 | 13 | 0 |

The clean evidence still has `buy_with_coin_router=0 in all arms`.

`scripted AttemptPrediction fixture does not count as E2`.

## Interpretation

REAL-10 shows structural market activity increases across market-visible arms,
but agent_economic_action_tx_count remains zero. Activity alone is not
emergence.

E2 not achieved. E3 not achieved. E4 not achieved.

Forbidden claim boundary:

```text
forbidden: autonomous prediction market
forbidden: emergent agent economy
forbidden: market-proven performance improvement
forbidden: real-world readiness
forbidden: agent economy beta without E2
forbidden: emergent market beta without E3
```
