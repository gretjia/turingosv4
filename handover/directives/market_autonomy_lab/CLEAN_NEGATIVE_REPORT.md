# Market Autonomy Lab Clean Negative Report

## Verdict

```text
CLEAN NEGATIVE, NOT COMPLETION
```

No new claim-bearing experiment was run in this cycle. The current clean-negative
boundary is inherited from the existing REAL-BCAST hard10 evidence and swarm
review.

## Evidence Boundary

```text
evidence report: handover/reports/REAL_BCAST_1_AND_HARD10_STRESS_REPORT.md
audit_tape: PROCEED
EVDecisionTrace: 106
MarketReviewSummary: 106
LibrarianDigest: 261
agent_economic_action_tx_count: 0
buy_with_coin_router: 0
```

Interpretation:

```text
Broadcast and market review substrate are producing evidence.
Live voluntary agent economic action is still absent.
All observed EVDecisionTrace outcomes in the hard10 stress report are abstain /
NegativeEV, so E2 is not present.
```

## Mechanism Bottlenecks

Swarm review identified three immediate mechanism bottlenecks:

1. BCAST market/no-trade blind spot:
   `MarketDecisionTrace` and `MarketReviewSummary` do not yet feed no-trade
   clusters into `LibrarianDigest`.

2. EV diagnostic collapse:
   `PositiveEVIgnored` exists as an enum but is not produced by the evaluator
   path, while abstain-side public EV basis is dropped.

3. PolicyTrader baseline absence:
   There is not yet a CAS-backed deterministic baseline to answer whether
   positive EV exists under the current market parameters.

## Not Completion

This report must not be used to stop the Market Autonomy Lab. It only selects
the next constitution-preserving atom sequence:

```text
Atom 1: BCAST-MARKET-COVERAGE-PATCH
Atom 2: EV-DIAGNOSTIC-PATCH
Atom 3: POLICYTRADER-BASELINE
```

All three atoms now have red gates written. Implementation remains blocked on
per-atom Class-4 authorization because the expected fixes touch
Trust-Root-pinned runtime/evaluator/dashboard/genesis surfaces.

No E2 candidate exists until a live, non-scripted, agent-generated router or
short-equivalent transaction appears with ChainTape/CAS, PromptCapsule,
MarketOpportunityTrace, EVDecisionTrace or explicit rationale, audit_tape
PROCEED, and no forced trade / price-as-truth / ghost liquidity.

## R11 Atom 6 Clean Negative

```text
evidence_dir: handover/evidence/market_autonomy_lab_hard10_probability_calibration_R11_20260516T174033Z
problem_set: handover/preregistration/sample_E1v2_hard10_S20260423.txt
problem_set_sha256: 138f75bc3a776df11df45fed77c01565e128662ca03e3d863e68dcf8ee22d8fc
audit_tape: PROCEED
persistence_passing: true
EVDecisionTrace: 92
MarketReviewSummary: 92
PolicyTraderTrace: 92
EconomicJudgment: 92
live_non_scripted_router_tx_count: 0
buy_with_coin_router: 0
E2: NOT ACHIEVED
```

R11 is the valid post-Atom-6 evidence run. R10 is diagnostic-only because it
was accidentally launched with `CARGO_TARGET_DIR=target/codex-main` while the
runner consumes fixed `target/release/*` binaries; R11 rebuilt and used the
fresh default release evaluator.

R11 mechanism explanation:

```text
EVReason distribution:
  ProbabilityUncalibrated: 92
  NegativeEV: 0
  PositiveEV: 0
  PositiveEVIgnored: 0

PolicyTrader comparison:
  PolicyNoPositiveEV: 92

EconomicJudgment distribution:
  Abstain / NoPerceivedEdge / Unknown / probability band 0-0: 92
```

Interpretation:

```text
Atom 6 succeeded as a diagnostic patch: R9/R10-style 0-0 probability collapse
is no longer mislabeled as real NegativeEV.

Atom 6 did not achieve market emergence. Traders still externalize no
calibrated probability basis and therefore do not produce voluntary economic
action.
```

Next bottleneck:

```text
Atom 7: Trader probability calibration ladder.
```

## R12 Atom 7 Clean Negative

```text
evidence_dir: handover/evidence/market_autonomy_lab_hard10_probability_ladder_R12_20260516T180909Z
audit_tape: PROCEED
EVDecisionTrace: 102
MarketReviewSummary: 102
PolicyTraderTrace: 102
EconomicJudgment: 102
live_non_scripted_router_tx_count: 0
buy_with_coin_router: 0
E2: NOT ACHIEVED
```

Mechanism explanation:

```text
PolicyPositiveEV_LLMAbstained: 48
PolicyNoPositiveEV: 54
PositiveEVIgnored: 3
ProbabilityUncalibrated: 3
NegativeEV: 96
```

R12 partially reduced probability-placeholder collapse but still did not
activate live voluntary router action.

## R13 Atom 8 Clean Negative

```text
evidence_dir: handover/evidence/market_autonomy_lab_hard10_positive_ev_ignored_R13_20260516T184417Z
audit_tape: PROCEED
EVDecisionTrace: 112
MarketReviewSummary: 112
PolicyTraderTrace: 112
EconomicJudgment: 112
live_non_scripted_router_tx_count: 0
buy_with_coin_router: 0
E2: NOT ACHIEVED
```

Mechanism explanation:

```text
PositiveEVIgnored: 65
NegativeEV: 45
ProbabilityUncalibrated: 2
PolicyPositiveEV_LLMAbstained: 65
PolicyNoPositiveEV: 47
```

R13 is a high-value clean-negative: the market produced public positive EV,
PolicyTrader would have bought, and LLM traders still abstained. The next
constitution-preserving hypothesis was not to force trade; it was to make the
positive-EV action handoff explicit while preserving voluntary choice.

## R14 Challenge Boundary

R14 is no longer a clean-negative because it contains 9 live
`BuyWithCoinRouter` rows. However, its clean-context audit returned
`CHALLENGE`, so it is not an audit-ready active E2 candidate.

R14 is tracked separately in `E2_CANDIDATE_REPORT.md` as challenged progress
evidence. The required next step is a fixed R15 hard10 rerun.

Allowed wording for a future clean rerun remains:

```text
E2 candidate pending audit
```

## Superseded By R16 Candidate

R15 repaired the exact-join issue with Librarian off. R16 repeated hard10 with
Librarian on and is no longer clean-negative:

```text
evidence_dir: handover/evidence/market_autonomy_lab_hard10_bcast_exact_join_R16_20260516T202248Z
audit_tape: PROCEED
live_non_scripted_router_tx_count: 8
matched_submitted_router_tx_id_count: 8
duplicate_router_tx_id_count: 0
scripted_fixture_tx_count: 0
policy_counts_for_e2: false
LibrarianDigest: 278
```

Boundary:

```text
E2 candidate pending audit
not E2 achieved
```
