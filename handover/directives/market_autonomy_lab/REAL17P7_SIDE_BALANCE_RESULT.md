# REAL-17 P7 — Side-Balance hard10 Result

Date: 2026-05-18
Mode: Constitutional Research Mode under `MARKET_AUTONOMY_LAB_RESEARCH_ENVELOPE_V2`
Base commit: `a1c75c186529026f4b715686146abf68646ce206`

## Claim Boundary

P7 is full hard10 side-balance hardening evidence after P6D seed
poll-budget stabilization.

P7 supports:

```text
full hard10 D-arm evidence completed
E2 replicated candidate remains supported on YES side
direct PromptCapsule provenance remains supported for matched tx rows
```

P7 does not support:

```text
stable two-sided market
BuyNo / short-side replication
E2 achieved
E3 achieved
E4 achieved
market emergence proven
market mechanism shipped
```

The correct side-balance classification is:

```text
side-balance clean-negative: YES-only replicated action, NO-side PositiveEVIgnored remains
```

## Evidence

Evidence root:

```text
handover/evidence/market_autonomy_lab_real17P7_tisr_main_hard10_seed_budget_side_balance_20260518T052928Z
```

Arm D evidence:

```text
handover/evidence/market_autonomy_lab_real17P7_tisr_main_hard10_seed_budget_side_balance_20260518T052928Z/arm_D
```

Run facts:

```text
batch_evaluator exit: 0
chain continuity: OK across 10 tasks
audit_tape verdict: PROCEED
audit_tape assertions: 41 passed / 0 failed / 0 halted / 11 skipped
persistence binding: passing, n_witnessed=3
REAL16 wrapper verdict: Veto
wrapper reason: single-arm benchmark report still marks the benchmark packet as non-claim-bearing;
                arm-level ChainTape/CAS verifier evidence is PROCEED
```

The wrapper `Veto` is not treated as a constitutional hard stop because the
full batch and the arm-level audit/verifier path passed. It is a benchmark
packet boundary: one D arm cannot support stronger E4 comparison claims.

## Exact-Join / Provenance

Verifier:

```text
arm_D/REAL16_ARM_D_E2_VERIFIER.json
```

Key fields:

```text
verdict: PROCEED
exact_join_count: 14
l4_router_tx_count: 14
submitted_trace_tx_count: 14
direct_prompt_capsule_provenance_count: 14
indirect_prompt_capsule_provenance_count: 0
missing_direct_prompt_capsule_provenance_count: 0
duplicate_l4_router_tx_id_count: 0
duplicate_submitted_trace_tx_id_count: 0
scripted_fixture_tx_count: 0
policy_counts_for_e2: false
BCAST shielding verdict: PASS
```

Matched action split:

```text
BullTrader BuyYes / BuyYes: 14
BearTrader BuyNo / BuyNo: 0
```

Therefore P7 is YES-side only.

## CAS EV Distribution

CAS source:

```text
arm_D/cas/.turingos_cas_index.jsonl
schema_id = real13a.ev_decision_trace.v1
```

Distribution:

```text
BearTrader No Abstain NegativeEV: 3, avg_edge_bps=-284
BearTrader No Abstain PositiveEVIgnored: 19, avg_edge_bps=841.84
BearTrader No Abstain ProbabilityUncalibrated: 3, avg_edge_bps=-4950
BullTrader Yes Abstain NegativeEV: 1, avg_edge_bps=0
BullTrader Yes Abstain PositiveEVIgnored: 12, avg_edge_bps=959.17
BullTrader Yes BuyYes PositiveEV: 14, avg_edge_bps=2696.79
```

Interpretation:

```text
public NO-side positive EV existed for BearTrader turns,
but BearTrader did not convert any NO-side positive EV into live BuyNo action.
```

This keeps the dominant bottleneck at:

```text
NO-side action conversion / BearTrader PositiveEVIgnored
```

## P6D Stabilization Outcome

P6c had failed at task 8 with:

```text
TaskOutcomeMarket seed FAIL-CLOSED: await REAL-6A MarketSeedTx commit
```

P7 crossed task 8 and completed all 10 tasks after the P6D patch routed the
TaskOutcomeMarket seed waits through `TURINGOS_REAL6A_POLL_BUDGET_MS`.

This supports P6D as a runner-stability fix for the REAL-6A seed wait path.

## Next Hypothesis

P8 should not add force. The next in-envelope hypothesis is:

```text
BearTrader may still treat NO-side action as betting that the mathematical
statement is false, instead of betting on the TaskOutcomeMarket outcome that no
valid proof will be accepted before the market deadline.
```

Recommended P8 atom:

```text
REAL-17 P8 — BearTrader NO-side semantic action conversion
```

Allowed mechanism:

```text
Clarify in the role-scoped TraderView that BuyNo means buying the NO outcome
of the task market: no accepted valid proof / no YES resolution before deadline.
This is not a claim that the theorem is false.
Abstain remains valid whenever confidence, liquidity, balance, or risk checks
do not pass.
```

Required red gates:

```text
bear_trader_no_side_prompt_defines_no_as_task_outcome_not_theorem_false
bear_trader_no_side_positive_ev_keeps_abstain_valid
bear_trader_no_side_positive_ev_forbids_forced_short_language
bear_trader_no_side_action_schema_preserves_buy_no_and_abstain_only
```

P8 acceptance should require:

```text
no forced short
no price-as-truth
no ghost liquidity
no f64/f32 market path
exact-join verifier for any BuyNo candidate
clean-negative report if BuyNo remains absent
```
