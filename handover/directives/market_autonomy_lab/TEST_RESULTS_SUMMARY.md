# Market Autonomy Lab Test Results Summary

## Current Cycle

This cycle is operating under `MARKET_AUTONOMY_LAB_RESEARCH_ENVELOPE_V2`, not
Ship Mode. R14 produced the first nonzero live router activity, then
clean-context audit returned `CHALLENGE`. R15 repaired the exact-join issue with
Librarian off; R16 repeated hard10 with Librarian on and is now tracked as:

```text
E2 candidate pending audit
```

## Hard10 Baseline Evidence

Source report:

```text
handover/reports/REAL_BCAST_1_AND_HARD10_STRESS_REPORT.md
```

Pinned hard10 set:

```text
handover/preregistration/sample_E1v2_hard10_S20260423.txt
sha256=138f75bc3a776df11df45fed77c01565e128662ca03e3d863e68dcf8ee22d8fc
```

Existing hard10 counts:

| Metric | Value |
| --- | ---: |
| `audit_tape` | `PROCEED` |
| `problem_count` | 10 |
| `tx_count` | 213 |
| `structural_market_tx_count` | 24 |
| `resolution_tx_count` | 10 |
| `scripted_or_unproven_router_tx_count` | 0 |
| `buy_with_coin_router` | 0 |
| `agent_economic_action_tx_count` | 0 |
| `EVDecisionTrace total` | 106 |
| `Bull / Bear` | 53 / 53 |
| `buy_yes / buy_no / abstain` | 0 / 0 / 106 |
| `ev_decision_reason_NegativeEV` | 106 |
| `MarketReviewSummary` | 106 |
| `LibrarianDigest` | 261 |
| `librarian_role_crop_cas_count` | 261 |
| `librarian_shielding_verdict` | `PASS` |

Boundary:

```text
This supports a clean-negative finding only.
It does not support E2.
```

## Missing Metric Fields For Next Cycles

The next implementation atoms must make these fields extractable from
ChainTape/CAS-derived reports:

```text
policy_trader_trace_total_cas
policy_positive_ev_count
policy_positive_ev_llm_abstained_count
PositiveEVIgnored
NoTradeReason distribution
MarketDecisionTrace no-trade clusters
MarketReviewSummary abstain/missing clusters
structural_market_tx_count
agent_economic_action_tx_count
scripted_fixture_tx_count
resolution_tx_count
live_non_scripted_router_tx_count distinct from agent_economic_action_tx_count
```

## ARH-v2 Metric Extraction Checklist

ARH-v2 completion requires extracting every field below from ChainTape/CAS
derived evidence. A clean-negative run is not completion; it is only a
baseline condition until these metrics are present and reviewable.

- hard10 hash match
- audit_tape verdict
- EVDecisionTrace total/Bull/Bear/buy_yes/buy_no/abstain
- MarketReviewSummary count
- LibrarianDigest count
- AgentEconomicActionTx count
- StructuralMarketTx count
- ScriptedFixtureTx count
- ResolutionTx count
- PositiveEVIgnored count
- policy_trader_trace_total_cas
- no-trade distribution
- PnL deltas
- role activity table

## Local Verification

```text
Trust Root unit gate: PASS
cargo fmt --check: PASS
git diff --check: PASS
constitution gates: PASS after hydrating missing historical evidence fixtures
```

The lines above describe the previous pre-R14 verification point. After the R14
CHALLENGE fixes and allowed Trust Root rehash, the same gates must be rerun
before R15 evidence is started.

## R14 Challenged Candidate Evidence

Fresh verification before R14:

```text
cargo test --test constitution_real13a_ev_decision_trace -- --test-threads=1: 19 passed
cargo fmt --all -- --check: PASS
git diff --check: PASS
Trust Root unit gate: PASS
cargo test -p minif2f_v4 --test constitution_g1_2_subprocess_resume -- --test-threads=1: 5 passed
bash scripts/run_constitution_gates.sh: 461 passed, 0 failed, 1 ignored
```

R14 true-problem evidence:

```text
evidence_dir: handover/evidence/market_autonomy_lab_hard10_action_handoff_R14_20260516T191707Z
problem_set: handover/preregistration/sample_E1v2_hard10_S20260423.txt
problem_set_sha256: 138f75bc3a776df11df45fed77c01565e128662ca03e3d863e68dcf8ee22d8fc
batch_evaluator: exit=0
audit_tape: PROCEED
persistence: is_passing=true n_witnessed=5
```

R14 counts:

| Metric | Value |
| --- | ---: |
| `buy_with_coin_router` | 9 |
| `buy_yes_router_count` | 9 |
| `buy_no_router_count` | 0 |
| `live_non_scripted_router_tx_count` | 9 |
| `agent_economic_action_tx_count` | 9 |
| `structural_market_tx_count` | 22 |
| `scripted_fixture_tx_count` | 0 |
| `scripted_or_unproven_router_tx_count` | 0 |
| `EVDecisionTrace` | 112 |
| `EconomicJudgment` | 112 |
| `MarketReviewSummary` | 112 |
| `PolicyTraderTrace` | 112 |
| `LibrarianDigest` | 278 |

Clean-context audit boundary:

```text
R14 clean-context audit verdict: CHALLENGE.
R14 is progress evidence, not an audit-ready active E2 candidate.
R14 does not support E2 achieved.
```

R14 challenge findings now covered by red/green gates:

```text
router tx-id reuse across tasks
dashboard counted action via min(router_total, submitted_count) instead of exact submitted trace join
report wording used forbidden "E2 candidate achieved" phrase
§K absent G7 structural guard needed explicit N/A interpretation
```

Post-CHALLENGE fix gates already added:

```text
real12_task_outcome_router_suffix_includes_task_identity
dashboard_counts_e2_candidate_router_actions_by_exact_submitted_trace_join
dashboard_marks_absent_g7_structural_guards_as_non_sentinel
REAL-12/REAL-13 runner wording forbids "E2 candidate achieved"
```

## R15 Exact-Join Replication

```text
evidence_dir: handover/evidence/market_autonomy_lab_hard10_exact_join_R15_20260516T195846Z
problem_set_sha256: 138f75bc3a776df11df45fed77c01565e128662ca03e3d863e68dcf8ee22d8fc
batch_exit: 0
audit_tape: PROCEED
persistence_passing: true
```

R15 key counts:

```text
agent_economic_action_tx_count: 13
matched_submitted_router_tx_id_count: 13
duplicate_router_tx_id_count: 0
duplicate_submitted_router_tx_id_count: 0
scripted_or_unproven_router_tx_count: 0
scripted_fixture_tx_count: 0
EVDecisionTrace: 106
MarketReviewSummary: 106
LibrarianDigest: 0
```

Boundary:

```text
R15 confirms the exact-join remediation, but it is not the active candidate
because Librarian was off.
```

## R16 BCAST Candidate Evidence

```text
evidence_dir: handover/evidence/market_autonomy_lab_hard10_bcast_exact_join_R16_20260516T202248Z
problem_set_sha256: 138f75bc3a776df11df45fed77c01565e128662ca03e3d863e68dcf8ee22d8fc
batch_exit: 0
audit_tape: PROCEED
audit_assertions: passed=41 failed=0 halted=0 skipped=11
persistence_passing: true
persistence_n_witnessed: 5
```

R16 key counts:

| Metric | Value |
| --- | ---: |
| `agent_economic_action_tx_count` | 8 |
| `matched_submitted_router_tx_id_count` | 8 |
| `duplicate_router_tx_id_count` | 0 |
| `duplicate_submitted_router_tx_id_count` | 0 |
| `scripted_or_unproven_router_tx_count` | 0 |
| `scripted_fixture_tx_count` | 0 |
| `EVDecisionTrace` | 112 |
| `MarketReviewSummary` | 112 |
| `MarketOpportunityTrace` | 112 |
| `EconomicJudgment` | 112 |
| `PolicyTraderTrace` | 112 |
| `LibrarianDigest` | 278 |
| `LibrarianRoleCrop` | 278 |

Boundary:

```text
R16 supports the research wording "E2 candidate pending audit".
It does not support "E2 achieved".
```

R16 clean-context audit CHALLENGE closure:

```text
P1: RESEARCH_ENVELOPE_V2 now lists REAL-12/REAL-13 runner scripts and
    constrains h_vppu_history.json to non-authoritative side-effect use.
P2: dashboard scripted_fixture_tx_count now derives from CAS-backed
    scripted_attempt_prediction_market_count.
P3: future REAL-12 reports label all EconomicJudgment reason rows as
    economic_judgment_reason_distribution.
closure audit verdict: PROCEED
```

Post-closure targeted red/green gates:

```text
dashboard_derives_scripted_fixture_count_from_cas_not_constant:
  RED before fix, GREEN after fix
research_envelope_v2_lists_allowed_and_forbidden_surfaces:
  RED before envelope clarification, GREEN after clarification
real12_probe_labels_economic_judgment_reason_distribution_without_abstain_drift:
  RED before label fix, GREEN after fix
```

## Atom 6 Verification And R11 Evidence

Fresh verification before R11:

```text
cargo fmt --all -- --check: PASS
git diff --check: PASS
Trust Root unit gate: PASS
research preflight: Level2 allowed Trust Root rehash checkpoint
targeted atom tests: PASS
constitution gates: 461 passed, 0 failed, 1 ignored
```

Atom 6 targeted tests:

```text
target: constitution_real13a_ev_decision_trace
result: 14 passed, 0 failed
new gates:
  trader_ev_scaffold_rejects_zero_zero_probability_as_placeholder
  evaluator_classifies_zero_zero_unknown_probability_as_uncalibrated
  ev_reason_taxonomy_is_exhaustive_in_summary_and_dashboard
```

R11 evidence:

```text
evidence_dir: handover/evidence/market_autonomy_lab_hard10_probability_calibration_R11_20260516T174033Z
problem_set_hash: 138f75bc3a776df11df45fed77c01565e128662ca03e3d863e68dcf8ee22d8fc
audit_tape: PROCEED
persistence_passing: true
EVDecisionTrace: 92
MarketReviewSummary: 92
PolicyTraderTrace: 92
EconomicJudgment: 92
Bull/Bear: 46 / 46
AgentEconomicActionTx: 0
BuyWithCoinRouter: 0
StructuralMarketTx: 26
ScriptedFixtureTx: 0
LibrarianDigest: 224
Librarian market/no-trade clusters: 669 / 446
```

R11 EV classification:

```text
ProbabilityUncalibrated: 92
NegativeEV: 0
PositiveEV: 0
PositiveEVIgnored: 0
PolicyNoPositiveEV: 92
agent_probability_bps=0: 92
edge_bps=-5000: 92
```

R10 was excluded from Atom 6 conclusions because it was launched with a custom
`CARGO_TARGET_DIR` while the runner consumed fixed `target/release/*` binaries.
R11 corrected that runner-preflight gap by rebuilding and using the default
release binary.

## New Red Gates

Atom 1 BCAST market/no-trade coverage now has a failing gate target:

```text
test target: constitution_librarian_market_no_trade
evidence: handover/evidence/dev_self_hosting/dev_1778933784024_2984070/artifacts/command_0016_stdout.txt
result: 0 passed, 6 failed
```

The failures are behavioral, not compile errors:

```text
MarketDecisionTrace no-trade is not promoted into MarketReason events.
MarketDecisionTrace no-trade is not clustered in LibrarianDigest.
MarketReviewSummary abstain/missing is not promoted into market reasons.
Unknown Generic schema in the CAS index is not rejected by selector.
Raw prompt/private CoT in market no-trade summary is not caught by selector.
Dashboard does not render BCAST market/no-trade cluster counts.
```

This red gate must remain red until Atom 1 is authorized and implemented.

Atom 2 EV diagnostics now has failing gates in existing REAL-12 / REAL-13A
targets:

```text
target: constitution_real12_economic_judgment
evidence: handover/evidence/dev_self_hosting/dev_1778933784024_2984070/artifacts/command_0025_stdout.txt
result: 6 passed, 1 failed

target: constitution_real13a_ev_decision_trace
evidence: handover/evidence/dev_self_hosting/dev_1778933784024_2984070/artifacts/command_0026_stdout.txt
result: 5 passed, 3 failed
```

The failures pin:

```text
abstain-side public EV basis is dropped by evaluator
missing EV basis is converted into invented 50/50 or zero-liquidity defaults
PositiveEVIgnored is not emitted by evaluator classifier
EV reason summary/dashboard do not expose zero-count taxonomy rows
```

Atom 3 PolicyTrader baseline now has a failing gate target:

```text
target: constitution_policy_trader_trace
evidence: handover/evidence/dev_self_hosting/dev_1778933784024_2984070/artifacts/command_0028_stdout.txt
result: 0 passed, 5 failed
```

The failures pin:

```text
PolicyTraderTrace module does not exist
integer-only counterfactual baseline fields do not exist
counterfactual_only / counts_for_e2=false exclusion is not implemented
LLM-vs-policy EV comparison buckets are absent
dashboard does not derive PolicyTrader metrics from CAS
```
