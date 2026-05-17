# REAL-14H Decision Packet

REAL-17 main-CAS integration note: this document is preserved as pre-CAS-repair
historical context only. It is not forward claim-bearing on
`codex/real17-emergence-hardening-20260517`; REAL-17 must regenerate
ChainTape/CAS/verifier evidence on the updated CAS Git commit-chain baseline
before any forward claim can rely on it.

decision: `PROCEED`
labels:
- `E2 candidate pending audit`
- `Two-sided market candidate`
- `E2 replicated candidate` for the REAL-14G + REAL-14H pair

evidence_dir: `handover/evidence/market_autonomy_lab_hard10_real14H_frozen_real14G_replication_20260517T025400Z`
config_hash: `197d9b3e6f0a9a588cbb370054e38ac527aaea10c39cca3338adb36cf34d8b87`
problem_set_hash: `138f75bc3a776df11df45fed77c01565e128662ca03e3d863e68dcf8ee22d8fc`

Forbidden claims remain forbidden: no `E2 achieved`, no `market emergence
proven`, no `market mechanism shipped`, no `E3 achieved`, no `E4 achieved`.

## Summary

REAL-14H re-ran the hard10 track after REAL-14G and reproduced exact-join
agent economic action with stronger side coverage.

```text
audit_tape: PROCEED
exact_join_count: 13
buy_yes_count: 11
buy_no_count: 2
scripted_fixture_tx_count: 0
policy_counts_for_e2: false
BCAST shielding: PASS
```

This moves the lab from YES-side-only evidence to a two-sided candidate:

```text
BullTrader BuyYes: present
BearTrader BuyNo: present
```

## Evidence

Independent exact-join verifier:

```text
l4_router_tx_count: 13
submitted_trace_tx_count: 13
exact_join_count: 13
duplicate_l4_router_tx_id_count: 0
duplicate_submitted_trace_tx_id_count: 0
scripted_fixture_tx_count: 0
policy_counts_for_e2: false
verdict: PROCEED
```

Dashboard materialized view, derived from ChainTape/CAS:

```text
agent_economic_action_tx_count: 13
buy_with_coin_router_count: 13
router_buy_yes: 11
router_buy_no: 2
ev_decision_trace_total_cas: 40
ev_public_basis_delivery_rate_bps: 10000
policy_positive_ev_count: 29
positive_ev_ignored_total_cas: 16
positive_ev_action_conversion_rate_bps: 4482
librarian_shielding_verdict: PASS
```

The two BuyNo candidates are:

```text
router-task-outcome-Agent_1-task-n5_amc12_2000_p12_1778986645493-Agent_1-6
router-task-outcome-Agent_1-task-n5_mathd_algebra_208_1778986920491-Agent_1-1
```

They are BearTrader live-agent rows, not PolicyTrader/scripted fixtures.

## Replication

REAL-14G was already clean-context `PROCEED` as:

```text
E2 candidate pending audit
```

REAL-14H independently reproduced nonzero exact-join live agent action on the
same hard10 problem-set hash. Under the project ladder, the pair may be
reported as:

```text
E2 replicated candidate
```

This remains candidate-only evidence. It is not `E2 achieved`.

## Comparison

| metric | REAL-14G | REAL-14H |
| --- | ---: | ---: |
| exact_join_count | 8 | 13 |
| buy_yes_count | 8 | 11 |
| buy_no_count | 0 | 2 |
| policy_positive_ev_count | 25 | 29 |
| PositiveEVIgnored | 17 | 16 |
| action_conversion_rate_bps | 3200 | 4482 |
| public EV basis delivery | 38/38 | 40/40 |
| audit_tape | PROCEED | PROCEED |
| BCAST shielding | PASS | PASS |

## Current Ladder State

| ladder item | status |
| --- | --- |
| E2 candidate | present |
| E2 replicated candidate | present |
| Two-sided market candidate | present |
| E3 candidate | not yet established |
| E4 candidate | not yet established |
| market emergence candidate | not yet established |

## Residual Risks

1. PromptCapsule linkage is still indirect through EVDecisionTrace.
2. Some BullTrader rows have multiple matching EVDecisionTrace rows; exact
   router tx_id join disambiguates the candidate count.
3. E3 role differentiation is not yet demonstrated because REAL-14H proves
   two-sided market action, not stable multi-batch role distributions.
4. E4 is not tested; no causal performance claim is made.

## Next Recommendation

Open:

```text
REAL-15 -- Persistent Role Differentiation / E3 Candidate Study
```

The next research question is no longer whether voluntary market action can
appear. It is whether Bull/Bear/Verifier/Challenger/Solver roles form stable,
distinct, ChainTape/CAS-derived behavior distributions across multiple
true-problem batches without forced trade or price-as-truth.
