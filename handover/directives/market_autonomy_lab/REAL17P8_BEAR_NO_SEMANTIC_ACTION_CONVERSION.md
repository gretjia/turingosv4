# REAL-17 P8 — BearTrader NO-Side Semantic Action Conversion

Date: 2026-05-18
Mode: Constitutional Research Mode under `MARKET_AUTONOMY_LAB_RESEARCH_ENVELOPE_V2`
Risk: Class 2 / trust-root-pinned evaluator rehash inside envelope

## Claim Boundary

P8 is a non-forcing TraderView/action-conversion atom. It is not a market
emergence proof and does not upgrade E2/E3/E4 labels.

Allowed forward label remains:

```text
market emergence candidate -- final audit PROCEED, hardening pending
```

P8 does not claim:

```text
E2 achieved
E3 achieved
E4 achieved
market emergence proven
market mechanism shipped
stable two-sided market
```

## Evidence Trigger

P7 full hard10 evidence:

```text
handover/evidence/market_autonomy_lab_real17P7_tisr_main_hard10_seed_budget_side_balance_20260518T052928Z/arm_D
```

P7 completed all 10 tasks:

```text
batch_evaluator exit: 0
chain continuity: OK across 10 tasks
audit_tape verdict: PROCEED
persistence binding: passing
exact_join_count: 14
direct PromptCapsule provenance: 14/14
BCAST shielding: PASS
BuyNo exact-join: 0
```

P7 CAS EV distribution showed:

```text
BearTrader No Abstain PositiveEVIgnored: 19, avg_edge_bps=841.84
BullTrader Yes BuyYes PositiveEV: 14, avg_edge_bps=2696.79
```

Read-only diagnosis classified this as:

```text
side-balance clean-negative
dominant bottleneck: BearTrader NO-side semantic ambiguity / salience
```

## Hypothesis

BearTrader may still be treating NO-side action as a bet that the mathematical
statement is false, rather than a bet on the TaskOutcomeMarket outcome that no
accepted valid proof appears before the market deadline.

P8 clarifies this distinction in the BearTrader role-scoped public EV scaffold.

## Patch

P8 adds BearTrader-only non-forcing text:

```text
BuyNo means buying the TaskOutcomeMarket NO outcome.
NO outcome means no accepted valid proof before the market deadline.
NO outcome is not a claim that the mathematical statement is false.
Do not require theorem falsehood before considering `buy_no`.
`abstain` remains valid for weak confidence, liquidity, balance, or risk checks.
```

Existing text remains:

```text
`abstain` remains valid when confidence, liquidity, balance, or risk checks do not pass.
Do not convert a NO-side positive EV edge into a YES-side action.
```

The patch does not alter:

```text
market state
router logic
admission
TypedTx schema
sequencer
canonical signing payloads
wallet/kernel/bus authority
CAS ObjectType schema
```

## Red / Green Gates

Red gate before implementation:

```bash
cargo test --test constitution_real13a_ev_decision_trace \
  bear_trader_no_side_prompt_defines_no_as_task_outcome_not_theorem_false \
  -- --test-threads=1
```

Observed expected failure:

```text
BearTrader NO-side scaffold must define NO as task-outcome risk,
not theorem falsity: BuyNo means buying the TaskOutcomeMarket NO outcome
```

Green gates after implementation:

```bash
cargo test --test constitution_real13a_ev_decision_trace \
  bear_trader_no_side_prompt_defines_no_as_task_outcome_not_theorem_false \
  -- --test-threads=1

cargo test --test constitution_real13a_ev_decision_trace \
  bear_trader_positive_no_edge_has_symmetric_non_forcing_action_salience \
  -- --test-threads=1

cargo test --test constitution_real13a_ev_decision_trace -- --test-threads=1

cargo test --test constitution_real12_bull_bear_positive_control \
  --test constitution_real14_e2_candidate_verifier -- --test-threads=1

rustfmt --edition 2021 --check \
  experiments/minif2f_v4/src/bin/evaluator.rs \
  tests/constitution_real13a_ev_decision_trace.rs

git diff --check

cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo \
  -- --test-threads=1
```

Observed result:

```text
REAL-13A EVDecisionTrace gates: 27 passed / 0 failed
REAL-12 Bull/Bear positive controls: 3 passed / 0 failed
REAL-14 exact-join verifier gates: 10 passed / 0 failed
rustfmt check: passed
git diff --check: passed
Trust Root unit gate: passed
```

## Forbidden Mechanism Check

P8 does not introduce:

```text
forced trade
must-buy / must-short language
every-turn betting
price-as-truth
ghost liquidity
f64/f32 market money/probability path
off-tape truth
raw prompt/completion/CoT/log broadcast
scripted or PolicyTrader action counted as E2
```

## Next Evidence Run

Run:

```text
REAL-17 P8 hard10 side-balance rerun
```

Required classification:

```text
If BuyNo exact-join appears:
  label only as BuyNo / two-sided candidate evidence pending audit.

If BuyNo exact-join remains 0:
  write clean-negative and move to the next in-envelope hypothesis:
  side-specific action handoff / structured positive_ev_override_reason /
  stronger CAS-derived missed-positive-EV diagnostic.
```
