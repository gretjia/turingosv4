# Market Autonomy Lab Next Step Recommendation

## Recommendation

Proceed under `MARKET_AUTONOMY_LAB_RESEARCH_ENVELOPE_V2`, then continue in
this order:

```text
1. Verify architect original and ARH-v2 envelope gates.
2. Run research preflight guard.
3. Implement Atom 1 and rehash Trust Root if needed and allowed.
4. Implement Atom 2 and rehash Trust Root if needed and allowed.
5. Implement Atom 3 and rehash Trust Root if needed and allowed.
6. Integrate dashboard strings and market tx split reporting.
7. Run hard10 as the minimum claim-bearing pressure set.
8. If no E2 candidate appears, write clean-negative and continue with the next
   mechanism hypothesis rather than stopping.
```

The BCAST market/no-trade failing gate has already been written at
`tests/constitution_librarian_market_no_trade.rs` and currently fails 0/6 as
expected.

The EV diagnostics failing gates have already been written in
`tests/constitution_real12_economic_judgment.rs` and
`tests/constitution_real13a_ev_decision_trace.rs`; they currently fail 1/7 and
3/8 respectively.

The PolicyTrader baseline failing gate has already been written at
`tests/constitution_policy_trader_trace.rs` and currently fails 5/5 as expected.

## Current Next Atom After R11

R11 replaced the stale-binary R10 diagnostic and produced a valid
post-Atom-6 clean-negative:

```text
EVDecisionTrace: 92
EVReason::ProbabilityUncalibrated: 92
PolicyNoPositiveEV: 92
live_non_scripted_router_tx_count: 0
audit_tape: PROCEED
```

Recommended Atom 7:

```text
Trader probability calibration ladder.
```

Goal:

```text
Reduce 0-0 probability placeholder collapse without forcing any trade.
```

Allowed design:

```text
Force structured probability calibration fields.
Ask the trader to choose a coarse public confidence band.
Require a public uncertainty rationale for abstain.
Treat 0-0 as valid only for literal impossibility.
Keep buy_yes / buy_no voluntary.
Keep price as signal only, not probability truth.
Do not expose PPUT or optimize the hidden score in prompt.
```

Atom 7 red gates should require:

```text
TraderView includes a public probability ladder with coarse integer bps bands.
TraderView says 0-0 is not an allowed placeholder when evidence is merely weak.
Evaluator/dashboard continue classifying 0-0/Unknown as ProbabilityUncalibrated.
No forced-trade phrase appears in the new scaffold.
No price-copy-as-probability instruction appears.
```

## Current Next Step After R14 Audit

R14 produced the first nonzero live router activity in this lab:

```text
evidence_dir: handover/evidence/market_autonomy_lab_hard10_action_handoff_R14_20260516T191707Z
audit_tape: PROCEED
buy_with_coin_router: 9
live_non_scripted_router_tx_count: 9
EVDecisionTrace BuyYes / PositiveEV: 9
PolicyTrader BothBuy: 9
scripted_fixture_tx_count: 0
policy_counts_for_e2: false
```

Clean-context audit returned:

```text
CHALLENGE
```

The challenged points are fixable inside the envelope, so this is not a hard
stop. Current recommendation:

```text
1. Keep the existing R14 evidence immutable as challenged progress evidence.
2. Keep the red/green gates for task-scoped router tx IDs, exact submitted trace
   join, allowed candidate wording, and §K absent-guard annotation.
3. Apply the in-envelope fixes and allowed Trust Root rehash.
4. Rerun preflight and targeted atom gates.
5. Run R15 hard10 with the same pinned problem set and fresh binaries.
6. Request a new clean-context audit only if R15 satisfies the candidate
   boundary with exact joined tx evidence.
```

R15 produced an exact-join candidate with Librarian off. R16 repeated hard10
with Librarian on and is the active candidate:

```text
R16 evidence_dir:
  handover/evidence/market_autonomy_lab_hard10_bcast_exact_join_R16_20260516T202248Z
status:
  E2 candidate pending audit
```

Current recommendation:

```text
1. Complete clean-context audit of R16.
2. If audit returns PROCEED, keep the claim boundary at
   "E2 candidate pending audit" and do not claim E2 achieved.
3. If audit returns CHALLENGE and the issue is in-envelope, fix and rerun the
   smallest real evidence path that can revalidate the candidate.
4. If audit returns VETO, write STOP_PROOF.md only if the fix requires an
   unlisted restricted surface or forbidden mechanism; otherwise continue with
   the next in-envelope hypothesis.
5. After a PROCEED candidate, recommended next research step is hard20
   replication or a pinned A/B arm, not E2 ship claim.
```

## Why This Order

BCAST coverage should come first because traders cannot learn from no-trade
history if the digest does not carry market/no-trade clusters. EV diagnostics
comes second because it determines whether the market has no positive EV or
agents are ignoring positive EV. PolicyTrader comes third because it turns that
diagnosis into a deterministic non-E2 baseline.

## Do Not Stop For

```text
no E2 candidate
all abstain
PolicyTrader abstains
allowed Trust Root rehash
clean-negative
hard10 too easy
too few review windows
too few EV traces
CHALLENGE that is fixable inside the envelope
```

These outcomes must feed H1-H6 in `ARH_V2_STOP_POLICY.md`.
