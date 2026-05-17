# REAL-14 Clean-Negative Report — R17/R18

claim_boundary: `E2 candidate pending audit` only for R16; R17/R18 are clean-negative replication/ablation runs.

## Runs

| Run | Evidence dir | BCAST | Problem set sha256 | audit_tape | exact_join_count | Interpretation |
| --- | --- | --- | --- | --- | ---: | --- |
| R17 | `handover/evidence/market_autonomy_lab_hard10_real14_R17_20260516T234921Z` | on | `138f75bc3a776df11df45fed77c01565e128662ca03e3d863e68dcf8ee22d8fc` | PROCEED | 0 | clean-negative replication |
| R18 | `handover/evidence/market_autonomy_lab_hard10_real14_R18_bcast_off_20260517T000136Z` | off | `138f75bc3a776df11df45fed77c01565e128662ca03e3d863e68dcf8ee22d8fc` | PROCEED | 0 | clean-negative ablation |

## Mechanism Classification

| Question | R17 | R18 | Evidence |
| --- | --- | --- | --- |
| No positive EV? | likely yes, under public policy basis | likely yes, under public policy basis | `policy_positive_ev_count=0` |
| Positive EV ignored? | no | no | `policy_positive_ev_llm_abstained_count=0` |
| No actionable market? | not primary; markets seeded structurally | not primary; markets seeded structurally | `market_seed=11`, `cpmm_pool=11`, `buy_with_coin_router=0` |
| Insufficient public EV basis? | yes | yes | `policy_insufficient_public_basis_count=40/40` |
| Broadcast not delivered? | no for R17; intentionally off for R18 | intentionally off | R17 `librarian_digest_count=98`, R18 `librarian_digest_count=0` |
| Model parsing failed? | no direct evidence of parser causing buy loss | no direct evidence of parser causing buy loss | EV traces exist: 40 each |

## Next Hypothesis

R16 appears to have produced complete public EV basis and eight voluntary
BullTrader buy_yes actions; R17/R18 did not form any public positive EV basis.
The next constitution-preserving atom should therefore target EV basis
stability, not forced action:

```text
Atom REAL-14F — Public EV Basis Stabilization
Goal: make quoted_price, agent_probability_bps, amount, liquidity_depth, and
expected_value_micro reliably present for Bull/Bear turns without forcing buy.
Gate: PolicyTrader positive-EV opportunities reappear on pinned hard10 while
PolicyTrader remains counterfactual and excluded from E2.
```

Forbidden follow-up remains unchanged: no forced trade, no price-as-truth, no
ghost liquidity, no raw prompt/completion/CoT/log broadcast, and no
PolicyTrader/scripted action counted as E2.
