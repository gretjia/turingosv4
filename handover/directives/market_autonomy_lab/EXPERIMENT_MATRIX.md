# Market Autonomy Lab Experiment Matrix

## Problem Sets

| Set | Purpose | Source | Hash / Pin |
| --- | --- | --- | --- |
| Micro 3-5 | Wiring only | true MiniF2F/Lean | To be pinned before run |
| hard10 | Minimum claim-bearing set | `handover/preregistration/sample_E1v2_hard10_S20260423.txt` | `sha256=138f75bc3a776df11df45fed77c01565e128662ca03e3d863e68dcf8ee22d8fc` |
| hard20 | Escalation if hard10 pressure is insufficient | deterministic sample from `hard36_pool.txt` | `seed=20260516_REAL13_HARD20`, body hash to be recorded in run manifest |
| hard36 | Full soak if hard20 remains ambiguous | `handover/preregistration/hard36_pool.txt` | `sha256=996dc41e4ec0246da938e643d35aa1656068965346e9bc13584e30cc6169a9fb` |

## Arms

| Arm | Market | EV Scaffold | Librarian | PnL/Role Specialization | Claim Use |
| --- | --- | --- | --- | --- | --- |
| A | visible | off | off | baseline | comparator only |
| B | visible | on | off | baseline | EV effect |
| C | visible | on | on | baseline | broadcast effect |
| D | visible | on | on | PnL + Bull/Bear roles | strongest constitutional track |

## Optional In-Envelope Mechanism Arms

These arms are not allowed to become E2 evidence unless their provenance and
claim boundaries are explicit.

| Mechanism | Constitutional Condition | Required Gate |
| --- | --- | --- |
| Lawful MarketMaker liquidity | Collateral-backed, debited from explicit MarketMakerBudget/Treasury | `marketmaker_budget_debit_prevents_ghost_liquidity` |
| Exploration rebate | Explicit budget debit, reported separately, not counted as spontaneous pure market behavior | `rebate_is_budget_backed_and_disclosed` |
| TraderView PnL scoreboard | CAS/ChainTape-derived only, no PPUT prompt target | `trader_view_includes_pnl_without_pput_metric_target` |
| Role-specialized traders | Role forced, trade voluntary | `role_assignment_does_not_force_trade` |

## Pressure Sufficiency

A run is only smoke, not claim-bearing, if it has too few:

```text
MarketReviewSummary records
EVDecisionTrace records
non-trivial rejected/partial proof cycles
unresolved task windows before solve
```

If pressure is insufficient, escalate from Micro to hard10, then to deterministic
hard20/hard36 with larger `MAX_TRANSACTIONS` and timeout.

## Hard10 Claim-Bearing Floor

```text
problem_set: handover/preregistration/sample_E1v2_hard10_S20260423.txt
sha256: 138f75bc3a776df11df45fed77c01565e128662ca03e3d863e68dcf8ee22d8fc
MAX_TRANSACTIONS: 30
PER_PROBLEM_TIMEOUT_S: 900
Librarian: on for broadcast arms
scripted buys: disabled for claim-bearing runs
live REAL-6B: disabled unless separately ratified
```

Hard10 is sufficient pressure only if all are true:

```text
audit_tape = PROCEED
EVDecisionTrace >= 80
MarketReviewSummary >= 80
Bull and Bear traces both present
no more than 2/10 too-fast solves
no blank abstain reason
live_non_scripted_router_tx_count is independently extracted
```

Escalate to hard20 if hard10 has fewer than 80 EV/review windows, more than two
too-fast solves, or fewer than six tasks with sustained rejection/partial/cap
pressure. Escalate to hard36 if hard20 still has more than 25% too-fast solves
or no additional EV/action-policy diversity.

## E3/E4 Research Boundaries

E3 candidate requires:

```text
at least two roles show distinct behavior distributions across consecutive tasks
differences are ChainTape/CAS-derived, not prompt-label-only
at least one role has non-zero market / verify / challenge behavior
pattern persists across at least two tasks or two batches
```

E4 candidate requires:

```text
pinned-input A/B arms
same problem set / model assignment / budgets / timeout / max_tx
Wilson CI or equivalent statistical support
not market_tx_count alone
not small-n descriptive overclaim
```

## Hard20 Pinned Candidate List

```text
aime_1997_p9
aime_1999_p11
algebra_amgm_sumasqdivbgeqsuma
algebra_bleqa_apbon2msqrtableqambsqon8b
amc12_2000_p12
amc12_2000_p6
amc12a_2009_p7
amc12b_2021_p13
amc12b_2021_p4
imo_1962_p2
imo_1981_p6
mathd_algebra_170
mathd_algebra_196
mathd_algebra_246
mathd_algebra_270
mathd_algebra_332
mathd_numbertheory_150
mathd_numbertheory_427
numbertheory_2pownm1prime_nprime
numbertheory_notEquiv2i2jasqbsqdiv8
```

This list must be regenerated and hashed by a deterministic selector before it
is used as conclusion-bearing evidence.
