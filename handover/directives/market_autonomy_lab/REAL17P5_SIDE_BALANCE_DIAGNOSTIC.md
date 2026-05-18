# REAL-17 P5 Side-Balance Diagnostic

Date: 2026-05-18

## Scope

Risk class: Class 3 evidence / Class 4-adjacent evaluator prompt surface inside
`MARKET_AUTONOMY_LAB_RESEARCH_ENVELOPE_V2`.

Touched FC nodes / invariants:

- FC1 runtime loop: role-scoped agent input view before voluntary action.
- FC1 predicates / wtool boundary: no price-as-truth, no forced trade, no
  scripted or PolicyTrader action counted as E2.
- Evidence truth order: ChainTape/CAS/verifier output precedes dashboard/report
  text.

## Evidence Inputs

- P4c reference evidence:
  `handover/evidence/market_autonomy_lab_real17P4c_tisr_main_hard10_direct_prompt_provenance_20260517T234451Z/arm_D`
- P5 diagnostic evidence:
  `handover/evidence/market_autonomy_lab_real17P5_tisr_main_hard10_bear_first_side_balance_20260518T015238Z/arm_D`
- P5 role assignment:
  `BearTrader,BullTrader,Solver,Verifier,Challenger`

## P5 Result

P5 was a run-only Bear-first side-balance diagnostic. It is not a full hard10
claim-bearing replication because the batch stopped during task 8:

```text
terminated_reason =
  subprocess for task 8 mathd_algebra_332 exited non-zero (3)
```

The task-8 failure path was:

```text
TaskOutcomeMarket seed FAIL-CLOSED:
await for EscrowLock commit failed before seed: ()
```

The audit/verifier evidence for the completed prefix remains useful as a
diagnostic:

```text
audit_tape: PROCEED
exact_join_count: 8
direct_prompt_capsule_provenance_count: 8
scripted_fixture_tx_count: 0
policy_counts_for_e2: false
matched action split: BullTrader BuyYes = 8, BearTrader BuyNo = 0
```

## CAS EV Distribution

P4c reference, extracted from `real13a.ev_decision_trace.v1` CAS objects:

```text
BearTrader No Abstain NegativeEV: 8, avg_edge_bps ~= -606
BearTrader No Abstain PositiveEVIgnored: 18, avg_edge_bps ~= 1078
BullTrader Yes Abstain PositiveEVIgnored: 10, avg_edge_bps ~= 971
BullTrader Yes BuyYes PositiveEV: 13, avg_edge_bps ~= 2904
```

P5 Bear-first diagnostic, extracted from `real13a.ev_decision_trace.v1` CAS
objects:

```text
BearTrader No Abstain NegativeEV: 7, avg_edge_bps ~= -571
BearTrader No Abstain PositiveEVIgnored: 17, avg_edge_bps ~= 1303
BearTrader No Abstain ProbabilityUncalibrated: 1
BullTrader Yes Abstain PositiveEVIgnored: 10, avg_edge_bps ~= 1000
BullTrader Yes BuyYes PositiveEV: 8, avg_edge_bps ~= 3175
```

## Mechanism Diagnosis

The Bear-first role order did not produce live BuyNo / short-equivalent action.
Because P5 moved BearTrader into the first turn position yet the exact-join
matched action remained BullTrader BuyYes only, the current dominant side-balance
bottleneck is not simply `Agent_0` turn position.

The strongest in-envelope diagnosis is:

```text
BearTrader sees positive NO-side EV,
but voluntarily abstains at high rate.
```

This is an action-conversion / salience issue, not a missing public EV basis
issue and not a BuyNo route-positive-control issue.

## Follow-Up Patch Boundary

The follow-up evaluator patch is intentionally narrow and non-forcing:

- Add BearTrader-only public EV scaffold language.
- State that clear public positive EV on NO maps to candidate `buy_no`.
- Preserve `abstain` as valid when confidence, liquidity, balance, or risk
  checks do not pass.
- Forbid converting NO-side positive EV into YES-side action.

This patch does not touch sequencer admission, TypedTx schema/discriminant,
canonical signing payload, wallet, kernel, bus, CAS ObjectType schema, market
money conservation, or historical evidence.

## Claim Boundary

Allowed:

```text
P5 is partial diagnostic evidence for BearTrader NO-side action-conversion work.
```

Forbidden:

```text
P5 proves two-sided market stability.
P5 proves market emergence.
P5 upgrades any E-level label beyond diagnostic / candidate wording.
P5 is a full hard10 replication.
```
