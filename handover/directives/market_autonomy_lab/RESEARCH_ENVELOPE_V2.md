# MARKET_AUTONOMY_LAB_RESEARCH_ENVELOPE_V2

Date: 2026-05-16

This envelope authorizes Constitutional Research Mode only.

```text
not a ship authorization
not a main-branch merge authorization
not permission to claim E2/E3/E4 achieved
```

## Grand Objective

Achieve real voluntary agent market-mechanism emergence under the TuringOS
constitution:

```text
live, non-scripted, agent-generated economic action
produced by voluntary agent judgment
backed by ChainTape/CAS/PromptCapsule/EVDecisionTrace provenance
not forced trade
not scripted or PolicyTrader baseline
not price-as-truth
not ghost liquidity
not off-tape truth
not f64/f32 money or probability in market paths
not raw CoT/raw prompt/raw completion/raw log broadcast
```

No E2 candidate is not completion. Clean-negative is not completion.
Clean-negative is progress evidence and must feed the next
constitution-preserving mechanism hypothesis.

## Mode

```text
Mode B: Constitutional Research Mode
Scope: independent worktree / branch only
Ship claim: forbidden
Main merge: forbidden
Unsafe red-track mixing: forbidden
```

Ship Mode remains stricter: every Class-4 atom still requires separate ship
ratification before merge or formal claim.

## Allowed Atoms

```text
Atom 0 — Preserve architect source and ARH-v2 envelope
Atom 1 — BCAST market/no-trade source coverage
Atom 2 — EV diagnostics / PositiveEVIgnored / exhaustive reason taxonomy
Atom 3 — PolicyTrader deterministic baseline
Atom 4 — Market tx count split
Atom 5 — TraderView EV/PnL/Librarian digest improvements
Atom 6 — Hard10 true-problem run, escalating to hard20/hard36 if insufficient
Atom 7 — Clean-context audit and next-hypothesis loop
Atom 8 — REAL-15 role-differentiation verifier and E3 candidate packet
Atom 9 — REAL-16 pinned A/B performance verifier and E4 candidate packet
```

## Allowed Implementation Surfaces

```text
experiments/minif2f_v4/src/bin/evaluator.rs
src/runtime/librarian_broadcast.rs
src/runtime/market_review.rs
src/runtime/market_opportunity_trace.rs
src/runtime/real6_conviction_budget.rs
src/runtime/ev_decision_trace.rs
src/runtime/economic_judgment.rs
src/runtime/policy_trader_trace.rs
src/runtime/market_tx_category.rs
src/runtime/market_e2_candidate_verifier.rs
src/runtime/positive_ev_ignored.rs
src/runtime/role_differentiation.rs
src/runtime/market_performance_e4.rs
src/runtime/mod.rs
src/bin/audit_dashboard.rs
src/bin/real14_e2_candidate_verifier.rs
src/bin/real15_role_differentiation_verifier.rs
src/bin/real16_market_performance_verifier.rs
tests/constitution_librarian_*.rs
tests/constitution_real12_*.rs
tests/constitution_real13*.rs
tests/constitution_real14g_positive_ev_ignored.rs
tests/constitution_real14_e2_candidate_verifier.rs
tests/constitution_real15_role_differentiation.rs
tests/constitution_real16_market_performance.rs
tests/constitution_policy_trader_trace.rs
tests/constitution_market_autonomy_research_envelope.rs
scripts/run_market_autonomy_research_preflight.sh
scripts/run_real12_task_market_probe.sh
scripts/run_real13_market_pressure_probe.sh
scripts/run_real8_market_ab_benchmark.sh
scripts/run_real16_market_performance_benchmark.sh
handover/directives/2026-05-16_MARKET_AUTONOMY_LAB_ARCHITECT_ORIGINAL.md
handover/directives/market_autonomy_lab/**
handover/evidence/**
handover/reports/**
h_vppu_history.json
genesis_payload.toml
```

`genesis_payload.toml` is allowed only for Trust Root rehash of touched pinned
files in this envelope. It must not change genesis authority semantics, post-init
mint authority, signing semantics, or add an unanchored root.

`scripts/run_real12_task_market_probe.sh` and
`scripts/run_real13_market_pressure_probe.sh` are allowed only as research
runner/report scaffolds for the atoms above. They must not enable forced trade,
scripted buys, live REAL-6B, price-as-truth, or PolicyTrader/scripted actions
counted as E2.

`h_vppu_history.json` is allowed only as a non-authoritative evaluator-generated
H-VPPU side effect. It is not ChainTape/CAS evidence, not a source of truth, and
must not support any Market Autonomy candidate claim.

REAL-15 / REAL-16 addendum (2026-05-17): the grand Market Emergence goal
continues beyond E2 into candidate-only E3 and E4 research. The newly listed
role-differentiation and E4 performance verifier surfaces are additive
report/verifier surfaces only. They must not alter sequencer admission,
TypedTx schemas/discriminants, canonical signing payloads, wallet/kernel/bus
authority, CAS ObjectType schema, or economic conservation rules. They may
derive from ChainTape/CAS/exact-join verifier outputs and may emit
candidate-only reports, but they do not authorize E2/E3/E4 achieved claims.

## Forbidden Surfaces

```text
constitution.md
handover/alignment/TRACE_FLOWCHART_MATRIX.md
flowchart hashes
src/state/typed_tx.rs
src/state/sequencer.rs
src/bottom_white/cas/schema.rs
src/kernel.rs
src/bus.rs
src/sdk/tools/wallet.rs
canonical signing payload code
RootBox authority
system key authority
CAS ObjectType schema
economic conservation core except additive test-only views
```

If any forbidden surface is required, stop and write `STOP_PROOF.md`.

## Forbidden Mechanisms

```text
forced trade counted as E2
BullTrader must buy every turn
BearTrader must short every turn
price-as-truth
price-driven Lean accept/reject
ghost liquidity
f64/f32 money or probability in market paths
off-tape WAL truth
private CoT recording
raw prompt broadcast
raw completion broadcast
raw CoT broadcast
raw log broadcast
dashboard/stdout as source of truth
scripted action counted as E2
PolicyTrader action counted as E2
live REAL-6B unless separately ratified
PPUT or internal score exposed as prompt target
```

## Allowed Class-4 Behavior Inside This Envelope

```text
Touch allowed Trust-Root-pinned evaluator/runtime/dashboard files.
Update genesis_payload.toml only for Trust Root rehash of touched pinned files.
Rerun Trust Root verification after every rehash.
Continue automatically if Trust Root verification passes.
```

Trust Root rehash failure after an allowed rehash is a Level 3 Constitutional
Hard Stop.

## Required Ship Gates By Atom

Atom 0:

```text
research_envelope_v2_declares_research_only_not_ship
research_envelope_v2_lists_allowed_and_forbidden_surfaces
research_envelope_v2_requires_stop_proof_before_hard_stop
research_envelope_v2_forbids_clean_negative_as_completion
research_envelope_v2_preserves_architect_source_verbatim
```

Atom 1:

```text
selector_promotes_market_decision_no_trade_into_market_reason_events
digest_clusters_market_decision_no_trade_reasons
selector_promotes_market_review_summary_abstain_missing_into_market_reasons
selector_fails_closed_on_unknown_schema_in_cas_index
market_no_trade_librarian_path_rejects_raw_prompt_completion_cot_logs
dashboard_bcast_section_reports_market_no_trade_cluster_counts
```

Atom 2:

```text
evaluator_preserves_abstain_side_public_ev_basis_and_candidate_amount
ev_trace_does_not_invent_50_50_or_zero_liquidity_for_missing_basis
positive_ev_abstain_with_constraints_pass_is_positive_ev_ignored
ev_reason_taxonomy_is_exhaustive_in_summary_and_dashboard
ev_decision_trace_uses_integer_bps_only
ev_diagnostics_do_not_expose_private_cot_or_raw_logs
```

Atom 3:

```text
constitution_real13_policy_trader_trace
constitution_real13_policy_trader_integer_only
constitution_real13_policy_trader_counterfactual_not_e2
constitution_real13_policy_trader_compares_llm_ev
constitution_real13_policy_trader_dashboard_report
policy_trader_action_is_scripted_fixture_not_agent_economic_action
```

Atom 4:

```text
market_tx_count_splits_structural_agent_scripted_resolution
scripted_fixture_tx_never_counts_as_e2
policy_trader_tx_never_counts_as_e2
agent_economic_action_tx_requires_promptcapsule_provenance
dashboard_reports_split_market_tx_counts
```

Atom 5:

```text
trader_view_includes_bounded_market_digest_cids
trader_view_includes_pnl_without_pput_metric_target
bull_trader_cannot_buy_no
bear_trader_cannot_buy_yes
role_assignment_does_not_force_trade
promptcapsule_read_set_proves_digest_delivery
```

Atom 6:

```text
hard10_problem_set_hash_matches_preregistration
hard10_produces_market_review_windows_or_escalates
hard10_produces_ev_traces_or_escalates
hard20_hard36_use_pinned_seed_and_hash
true_problem_runs_do_not_use_toy_only_evidence
```

Atom 7:

```text
clean_negative_report_names_next_hypothesis
clean_negative_report_is_not_completion
e2_candidate_report_requires_live_non_scripted_agent_tx
reports_include_evidence_dir_audit_tape_config_hash_claim_boundary
status_sync_records_continue_or_hard_stop_with_reason
```

## E2 Candidate Boundary

Only write:

```text
E2 candidate pending audit
```

when all are true:

```text
live non-scripted agent-generated BuyWithCoinRouterTx or short-equivalent
ChainTape/CAS evidence
PromptCapsule provenance
MarketOpportunityTrace exists
EVDecisionTrace or explicit economic rationale exists
audit_tape PROCEED
no forced trade
no price-as-truth
no ghost liquidity
not PolicyTrader
not scripted fixture
```

Never write `E2 achieved` from this research envelope.
