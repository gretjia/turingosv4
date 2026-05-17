# Market Autonomy Lab Class-4 Ratification Request

This request is prepared before implementation of Trust-Root-pinned source
changes. It is not itself authorization.

## Requested Atom Authorization

### Atom 1 — BCAST-MARKET-COVERAGE-PATCH

Allowed paths:

```text
src/runtime/librarian_broadcast.rs
src/runtime/market_review.rs
src/runtime/market_decision_trace.rs
tests/constitution_librarian_*.rs
src/bin/audit_dashboard.rs
genesis_payload.toml
```

Purpose:

```text
MarketDecisionTrace / NoTradeReason / MarketReviewSummary public no-trade
clusters enter LibrarianDigest for Trader/Bull/Bear views.
```

Proposed authorization text:

```text
I authorize Atom 1 BCAST-MARKET-COVERAGE-PATCH only, including Trust Root
rehash for the named files if required. The patch may add CAS-backed
LibrarianDigest market/no-trade coverage and tests, but must not change
TypedTx, sequencer admission, canonical signing payloads, CAS ObjectType schema,
or count any scripted/PolicyTrader/counterfactual action as E2.
```

Falling gates to write first:

```text
selector_promotes_market_decision_no_trade_into_market_reason_events
digest_clusters_market_decision_no_trade_reasons
selector_promotes_market_review_summary_abstain_missing_into_market_reasons
selector_fails_closed_on_unknown_schema_in_cas_index
market_no_trade_librarian_path_rejects_raw_prompt_completion_cot_logs
dashboard_bcast_section_reports_market_no_trade_cluster_counts
```

Status:

```text
written in: tests/constitution_librarian_market_no_trade.rs
red evidence: handover/evidence/dev_self_hosting/dev_1778933784024_2984070/artifacts/command_0016_stdout.txt
result: 0 passed, 6 failed
```

### Atom 2 — EV-DIAGNOSTIC-PATCH

Allowed paths:

```text
src/runtime/ev_decision_trace.rs
src/runtime/economic_judgment.rs
experiments/minif2f_v4/src/bin/evaluator.rs
tests/constitution_real13a_ev_decision_trace.rs
src/bin/audit_dashboard.rs
genesis_payload.toml
```

Purpose:

```text
Complete EV/no-trade taxonomy, preserve abstain evidence, and classify
PositiveEVIgnored when positive EV is available but the agent abstains.
```

Proposed authorization text:

```text
I authorize Atom 2 EV-DIAGNOSTIC-PATCH only, including Trust Root rehash for the
named files if required. The patch may preserve public abstain EV basis, remove
invented 50/50 or zero-liquidity defaults, and classify PositiveEVIgnored, but
must not use f64/f32 money, expose raw prompt/completion/CoT/log/stderr, or
count EVDecisionTrace as E2.
```

Falling gates to write first:

```text
evaluator_preserves_abstain_side_public_ev_basis_and_candidate_amount
ev_trace_does_not_invent_50_50_or_zero_liquidity_for_missing_basis
positive_ev_abstain_with_constraints_pass_is_positive_ev_ignored
ev_reason_taxonomy_is_exhaustive_in_summary_and_dashboard
```

Status:

```text
written in: tests/constitution_real12_economic_judgment.rs
written in: tests/constitution_real13a_ev_decision_trace.rs
red evidence: handover/evidence/dev_self_hosting/dev_1778933784024_2984070/artifacts/command_0025_stdout.txt
red evidence: handover/evidence/dev_self_hosting/dev_1778933784024_2984070/artifacts/command_0026_stdout.txt
result: REAL-12 target 6 passed / 1 failed; REAL-13A target 5 passed / 3 failed
```

### Atom 3 — POLICYTRADER-BASELINE

Allowed paths:

```text
src/runtime/policy_trader_trace.rs
src/runtime/mod.rs
tests/constitution_policy_trader_trace.rs
src/bin/audit_dashboard.rs
scripts/run_real13_market_pressure_probe.sh
genesis_payload.toml
```

Purpose:

```text
Generic CAS deterministic PolicyTrader baseline that is never counted as E2.
```

Proposed authorization text:

```text
I authorize Atom 3 POLICYTRADER-BASELINE only, including Trust Root rehash for
the named files if required. The baseline must be deterministic,
counterfactual-only, CAS-backed, integer-only, and explicitly excluded from E2
and live agent economic action counts.
```

Falling gates to write first:

```text
constitution_real13_policy_trader_trace
constitution_real13_policy_trader_integer_only
constitution_real13_policy_trader_counterfactual_not_e2
constitution_real13_policy_trader_compares_llm_ev
constitution_real13_policy_trader_dashboard_report
```

Status:

```text
written in: tests/constitution_policy_trader_trace.rs
red evidence: handover/evidence/dev_self_hosting/dev_1778933784024_2984070/artifacts/command_0028_stdout.txt
result: 0 passed, 5 failed
```

## Not Requested

```text
typed transaction schema or discriminant change
sequencer admission change
canonical signing payload change
CAS ObjectType schema change
constitution or flowchart change
forced trade
price-as-truth
ghost liquidity
live REAL-6B
raw prompt/completion/CoT/log/stderr broadcast
scripted or PolicyTrader action counted as E2
```

## Required Acceptance

```bash
cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo -- --test-threads=1
cargo test --test constitution_librarian_market_no_trade -- --test-threads=1
cargo test --test constitution_librarian_selector --test constitution_librarian_digest --test constitution_librarian_prompt_injection --test constitution_librarian_no_raw_leakage --test constitution_real12_economic_judgment --test constitution_real13a_ev_decision_trace --test constitution_real13b_market_review_window --test constitution_policy_trader_trace
bash scripts/run_constitution_gates.sh
cargo test --workspace --no-fail-fast -- --test-threads=1
git diff --check
```

Clean-context Codex review is required before any ship-path or E2-candidate
claim.
