# REAL-14F E2 Candidate Report

REAL-17 main-CAS integration note: this document is preserved as pre-CAS-repair
historical context only. It is not forward claim-bearing on
`codex/real17-emergence-hardening-20260517`; REAL-17 must regenerate
ChainTape/CAS/verifier evidence on the updated CAS Git commit-chain baseline
before any forward claim can rely on it.

status: `E2 candidate pending audit`

claim boundary: candidate only; no `E2 achieved`, no `market emergence proven`,
no `market mechanism shipped`, no `E3/E4 achieved`.

## Evidence

| Field | Value |
| --- | --- |
| evidence_dir | `handover/evidence/market_autonomy_lab_hard10_real14F_ev_basis_20260517T012509Z` |
| problem_set_hash | `138f75bc3a776df11df45fed77c01565e128662ca03e3d863e68dcf8ee22d8fc` |
| config_hash | `ce7496650684a74fb9d81b10cbe19e3f2ebbd7595da97df31f97755cc0f0a141` |
| audit_tape | `PROCEED` |
| aggregate assertions | `52`, failed `0`, halted `0` |
| independent verifier | `REAL14F_E2_CANDIDATE_VERIFIER.json`, verdict `PROCEED` |
| exact_join_count | `3` |
| scripted_fixture_tx_count | `0` |
| policy_counts_for_e2 | `false` |
| BCAST shielding | `PASS` |

The independent verifier recomputes:

```text
L4 BuyWithCoinRouterTx tx_id set
intersection submitted MarketDecisionTrace router_tx_id set
```

It returns:

```text
l4_router_tx_count=3
submitted_trace_tx_count=3
exact_join_count=3
duplicate_l4_router_tx_id_count=0
duplicate_submitted_trace_tx_id_count=0
failure_reasons=[]
```

## Matched Actions

All three matched actions are YES-side BullTrader actions:

```text
router-task-outcome-Agent_0-task-n5_mathd_algebra_208_1778981591894-Agent_0-0
router-task-outcome-Agent_0-task-n5_mathd_algebra_246_1778981626960-Agent_0-0
router-task-outcome-Agent_0-task-n5_mathd_algebra_332_1778981694822-Agent_0-0
```

Each matched row has:

```text
l4_buyer=Agent_0
l4_direction=BuyYes
l4_pay_coin_micro=1000
market_decision_agent_id=Agent_0
market_decision_direction=BuyYes
market_decision_amount_micro=1000
ev_decision_trace_count=1
ev_actions=[BuyYes]
ev_reasons=[PositiveEV]
market_opportunity_trace_count=1
role_turn_trace_count=1
live_agent_role=BullTrader
actor_is_policy_trader=false
actor_is_live_agent_role=true
```

Residual risk: PromptCapsule linkage is indirect through EVDecisionTrace because
MarketDecisionTrace does not carry a direct PromptCapsule field. The verifier
records this as residual risk, not as a direct-field claim.

## Mechanism Result

REAL-14F stabilized public EV basis for this hard10 run:

```text
EVDecisionTrace total=38
public_basis_available=38
public_basis_missing=0
delivery_rate_bps=10000
PolicyTrader traces=38
PolicyTrader positive EV=23
PolicyTrader positive EV but LLM abstained=20
```

This shifts the next bottleneck from missing public basis to action conversion:
20 positive-EV opportunities were still ignored by live agents.

## Boundary

This is a YES-side candidate only. It does not establish two-sided market
behavior, E3 role differentiation, or E4 performance improvement.
