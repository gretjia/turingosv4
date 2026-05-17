# Market Autonomy Lab Status Sync / OBS

## Observation

`handover/ai-direct/LATEST.md` currently summarizes the session at REAL-12 and
lists REAL-13A as a next step. Later repository artifacts show additional
state:

```text
REAL-13A/B/C/D/H scaffold exists.
REAL-BCAST-1 librarian broadcast loop exists.
hard10 stress evidence exists and remains E2-negative.
```

## Truth Order Handling

This OBS does not rewrite historical handover state. For this lab:

```text
constitution.md
flowchart hashes
ChainTape/CAS evidence
executable gates
REAL-13 / REAL-BCAST reports and evidence
LATEST.md
```

If future work changes dynamic project state, write a new handover update or
OBS rather than mutating old evidence.

## Current Constitutional Mode

As of this packet, the lab is continuing under
`MARKET_AUTONOMY_LAB_RESEARCH_ENVELOPE_V2`:

```text
mode: Constitutional Research Mode
not Ship Mode
not authorization to merge main
not authorization to claim E2/E3/E4 achieved
```

Allowed Trust-Root-pinned evaluator/runtime/dashboard files may be touched only
inside the envelope, with `genesis_payload.toml` updated only for the touched
file hashes and Trust Root verification rerun after every rehash.

Current status:

```text
R14 clean-context audit verdict: CHALLENGE
R15 exact-join replication: PROCEED, Librarian off
R16 BCAST hard10 candidate: PROCEED, Librarian on
hard_stop: false
current claim boundary: E2 candidate pending audit
clean_context_audit_r1: CHALLENGE
clean_context_challenge_closure_audit: PROCEED
```

No `STOP_PROOF.md` is required unless the next fix requires an unlisted
restricted surface, forbidden mechanism, unisolatable evidence contamination, or
Trust Root remains failing after an allowed rehash.

## R9 Evidence Metric Extraction OBS

Source evidence dir:

```text
handover/evidence/market_autonomy_lab_hard10_ev_scaffold_R9_20260516T163654Z
```

Scope and classification:

```text
risk_class: Class 0 OBS / metric extraction only
touched_fc_nodes: FC1 materialized evidence views over ChainTape/CAS;
  no sequencer admission, typed tx schema, signing payload, constitution, or
  flowchart authority changed.
```

Extracted identifiers:

```text
run_id: market_autonomy_lab_hard10_ev_scaffold_R9_20260516T163654Z_7b39499_t000
run_tag: market_autonomy_lab_hard10_ev_scaffold_R9_20260516T163654Z
batch_id: market_autonomy_lab_hard10_ev_scaffold_R9_20260516T163654Z_7b39499
problem_set: handover/preregistration/sample_E1v2_hard10_S20260423.txt
problem_set_hash: UNKNOWN
problem_set_hash_missing_path:
  handover/evidence/market_autonomy_lab_hard10_ev_scaffold_R9_20260516T163654Z/G_PHASE_BATCH_MANIFEST.json
```

Audit and tape summary:

```text
audit_tape_verdict: PROCEED
aggregate_verdict: PROCEED
audit_assertions: passed=41 failed=0 halted=0 skipped=11
L4_entries: 75
L4E_entries: 152
CAS_object_count: 2566
```

CAS-derived REAL-13 metrics:

```text
ev_decision_trace_total_cas: 104
ev_decision_trace_role_distribution:
  BullTrader: 52
  BearTrader: 52
ev_decision_trace_action_distribution:
  Abstain: 104
EVReason_distribution:
  NegativeEV: 104
agent_probability_bps_distribution:
  0: 104
MarketReviewSummary_count: 104
PolicyTraderTrace_count: 104
PolicyTraderTrace_comparison_distribution:
  PolicyNoPositiveEV: 104
PolicyTraderTrace_counts_for_e2_distribution:
  false: 104
PolicyTraderTrace_counterfactual_only_distribution:
  true: 104
live_non_scripted_router_tx_count: 0
```

## R16 E2 Candidate OBS

Source evidence dir:

```text
handover/evidence/market_autonomy_lab_hard10_bcast_exact_join_R16_20260516T202248Z
```

R16 is the active candidate run:

```text
audit_tape_verdict: PROCEED
batch_exit: 0
persistence_passing: true
buy_with_coin_router: 8
agent_economic_action_tx_count: 8
matched_submitted_router_tx_id_count: 8
duplicate_router_tx_id_count: 0
duplicate_submitted_router_tx_id_count: 0
scripted_or_unproven_router_tx_count: 0
scripted_fixture_tx_count: 0
EVDecisionTrace PositiveEV / BuyYes: 8
PolicyTrader counts_for_e2: false
LibrarianDigest: 278
LibrarianRoleCrop: 278
librarian_shielding_verdict: PASS
```

Current claim boundary:

```text
E2 candidate pending audit
not E2 achieved
not ship evidence
clean-context CHALLENGE closure audit: PROCEED
```

## R14 Challenged Candidate OBS

Source evidence dir:

```text
handover/evidence/market_autonomy_lab_hard10_action_handoff_R14_20260516T191707Z
```

R14 is the first run in this lab where the market action layer is nonzero:

```text
audit_tape_verdict: PROCEED
buy_with_coin_router: 9
live_non_scripted_router_tx_count: 9
agent_economic_action_tx_count: 9
scripted_fixture_tx_count: 0
scripted_or_unproven_router_tx_count: 0
EVDecisionTrace PositiveEV / BuyYes: 9
PolicyTrader BothBuy: 9
policy_counts_for_e2: false
```

Current claim boundary:

```text
R14 clean-context audit verdict: CHALLENGE
R14 is progress evidence only and is superseded by R15/R16
not E2 achieved
not ship evidence
```

Audit issues now converted into gates/fixes:

```text
PromptCapsule provenance is linked through role-turn traces rather than embedded
directly in submitted MarketDecisionTrace objects.

Dashboard G7 structural smoke needed an explicit absent-guard N/A annotation.

Submitted trace tx-id suffixes were reused across tasks; evaluator suffix now
includes task identity, and dashboard counts must use exact L4/submitted-trace
tx-id join rather than min-based inference.
```

REAL-12 / task-market metrics:

```text
MarketOpportunityTrace_count: 104
economic_judgment_total: 104
bull_judgment_count: 52
bear_judgment_count: 52
abstain_structured_reason_count: 104
economic_judgment_reason_distribution:
  NoPerceivedEdge: 104
buy_with_coin_router: 0
buy_yes_router_count: 0
buy_no_router_count: 0
agent_economic_action_tx_count: 0
no_trade_no_perceived_edge: 251
no_trade_zero_amount: 0
no_trade_no_pool: 0
no_trade_amount_exceeds_balance: 0
```

Structural market tx split:

```text
structural_market_tx_count: 26
agent_economic_action_tx_count: 0
scripted_or_unproven_router_tx_count: 0
scripted_fixture_tx_count: 0
resolution_tx_count: 9
buy_with_coin_router_count: 0

tx_kind_counts:
  task_open: 13
  escrow_lock: 10
  market_seed: 13
  cpmm_pool: 13
  cpmm_swap: 0
  buy_with_coin_router: 0
  event_resolve: 9
  terminal_summary: 7
  finalize_reward: 2
  work: 3
  verify: 5
  challenge: 0
```

REAL-BCAST LibrarianDigest counts:

```text
librarian_digest_cas_count: 254
librarian_role_crop_cas_count: 254
librarian_market_reason_cluster_count: 759
librarian_no_trade_reason_cluster_count: 506
librarian_ev_reason_cluster_count: 506
librarian_shielding_verdict: PASS
```

Conclusion:

```text
clean_negative: true
progress_evidence: true
E2_achieved: false
```

This R9 packet is clean-negative/progress evidence only. It shows CAS-visible
EV decision and policy-trader traces with complete abstention under
`NegativeEV`, zero live non-scripted router transactions, and no E2 achievement.

## R10 Stale-Binary OBS

```text
run_tag: market_autonomy_lab_hard10_probability_calibration_R10_20260516T171129Z
classification: diagnostic-only / stale-binary runner-preflight gap
audit_tape: PROCEED
E2_achieved: false
```

R10 was launched with `CARGO_TARGET_DIR=target/codex-main`. The G-phase runner
build command honored that target directory, but the script's binary paths are
fixed at `target/release/evaluator` and `target/release/batch_evaluator`.
Therefore R10 used the older release evaluator and must not be used as
post-Atom-6 evidence.

Corrective action:

```text
removed target/codex-main build cache
reran hard10 as R11 with default target/release binaries
verified target/release/evaluator contains Atom 6 ProbabilityUncalibrated token
```

## R11 Atom 6 Fresh-Binary Evidence

Source evidence dir:

```text
handover/evidence/market_autonomy_lab_hard10_probability_calibration_R11_20260516T174033Z
```

Extracted identifiers:

```text
run_tag: market_autonomy_lab_hard10_probability_calibration_R11_20260516T174033Z
batch_id: market_autonomy_lab_hard10_probability_calibration_R11_20260516T174033Z_7b39499
problem_set: handover/preregistration/sample_E1v2_hard10_S20260423.txt
problem_set_hash: 138f75bc3a776df11df45fed77c01565e128662ca03e3d863e68dcf8ee22d8fc
```

Audit and tape summary:

```text
audit_tape_verdict: PROCEED
audit_assertions: passed=41 failed=0 halted=0 skipped=11
persistence_passing: true
```

CAS-derived market metrics:

```text
EVDecisionTrace_count: 92
MarketReviewSummary_count: 92
PolicyTraderTrace_count: 92
EconomicJudgment_count: 92
BullTrader / BearTrader: 46 / 46
AgentEconomicActionTx: 0
BuyWithCoinRouter: 0
StructuralMarketTx: 26
ScriptedFixtureTx: 0
```

EV mechanism distribution:

```text
EVReason_distribution:
  ProbabilityUncalibrated: 92
  NegativeEV: 0
  PositiveEV: 0
  PositiveEVIgnored: 0
PolicyTraderComparison:
  PolicyNoPositiveEV: 92
EconomicJudgment:
  Abstain / NoPerceivedEdge / Unknown / probability band 0-0: 92
```

REAL-BCAST counts:

```text
librarian_digest_cas_count: 224
librarian_market_reason_cluster_count: 669
librarian_no_trade_reason_cluster_count: 446
```

Conclusion:

```text
clean_negative: true
progress_evidence: true
E2_achieved: false
next_bottleneck: probability externalization collapse
next_atom: Atom 7 Trader probability calibration ladder
```
