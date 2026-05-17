# REAL14 Metrics Comparison

Scope: R16 candidate validation plus R17 replication and R18 BCAST-off ablation.
R14 remains listed only as a superseded precursor. Conclusions remain bounded to
`E2 candidate pending audit`; no row below claims E2 achieved.

| Field | R14 | R16 | Delta R16-R14 |
| --- | ---: | ---: | ---: |
| batch_exit | 0 | 0 | 0 |
| audit_exit | 0 | 0 | 0 |
| buy_with_coin_router_count | 9 | 8 | -1 |
| agent_economic_action_tx_count | 9 | 8 | -1 |
| exact_join_count | null | 8 | null |
| scripted_fixture_tx_count | 0 | 0 | 0 |
| policy_counts_for_e2 | false | false | n/a |
| ev_decision_trace_total | 112 | 112 | 0 |
| positive_ev_count | 9 | 8 | -1 |
| positive_ev_ignored_count | 58 | 48 | -10 |
| librarian_digest_count | 278 | 278 | 0 |
| librarian_shielding_verdict | PASS | PASS | n/a |
| config_hash | null | null | null |

| Field | R16 | R17 | R18 |
| --- | ---: | ---: | ---: |
| BCAST flag | on | on | off |
| problem_set_sha256 | `138f75bc...22d8fc` | `138f75bc...22d8fc` | `138f75bc...22d8fc` |
| audit_tape_verdict | PROCEED | PROCEED | PROCEED |
| buy_with_coin_router_count | 8 | 0 | 0 |
| agent_economic_action_tx_count | 8 | 0 | 0 |
| independent_exact_join_count | 8 | 0 | 0 |
| verifier_verdict | PROCEED | PROCEED | PROCEED |
| scripted_fixture_tx_count | 0 | 0 | 0 |
| policy_counts_for_e2 | false | false | false |
| ev_decision_trace_total | 112 | 40 | 40 |
| ev_buy_yes_count | 8 | 0 | 0 |
| ev_buy_no_count | 0 | 0 | 0 |
| ev_abstain_count | 104 | 40 | 40 |
| policy_positive_ev_count | 56 | 0 | 0 |
| policy_positive_ev_llm_abstained_count | 48 | 0 | 0 |
| policy_insufficient_public_basis_count | 0 | 40 | 40 |
| librarian_digest_count | 278 | 98 | 0 |
| librarian_shielding_verdict | PASS | PASS | PASS |

| Dashboard materialized-view field | R14 | R16 | Delta R16-R14 |
| --- | ---: | ---: | ---: |
| matched_submitted_router_tx_id_count | null | 8 | null |
| submitted_market_decision_router_tx_ids | null | 8 | null |
| duplicate_router_tx_id_count | null | 0 | null |
| duplicate_submitted_router_tx_id_count | null | 0 | null |
| scripted_or_unproven_router_tx_count | 0 | 0 | 0 |
| policy_positive_ev_count | 67 | 56 | -11 |
| policy_positive_ev_llm_abstained_count | 58 | 48 | -10 |
| policy_no_positive_ev_count | 44 | 56 | 12 |
| policy_insufficient_public_basis_count | 1 | 0 | -1 |
| librarian_role_crop_cas_count | 278 | 278 | 0 |
| librarian_market_reason_cluster_count | 831 | 831 | 0 |
| librarian_no_trade_reason_cluster_count | 554 | 554 | 0 |
| librarian_ev_reason_cluster_count | 1632 | 1263 | -369 |

Missing fields:

| Field | R14 missing_reason | R16 missing_reason |
| --- | --- | --- |
| exact_join_count | independent verifier not generated for superseded R14 | n/a |
| config_hash | config_hash not present in R14 manifest/logs | config_hash not present in R16 manifest/logs |
| R14 matched_submitted_router_tx_id_count | not rendered in R14 dashboard exact-join fields | n/a |
| R14 submitted_market_decision_router_tx_ids | not rendered in R14 dashboard exact-join fields | n/a |
| R14 duplicate_router_tx_id_count | not rendered in R14 dashboard exact-join fields | n/a |
| R14 duplicate_submitted_router_tx_id_count | not rendered in R14 dashboard exact-join fields | n/a |

Sources:

| Run | Evidence dir | Primary files |
| --- | --- | --- |
| R14 | `handover/evidence/market_autonomy_lab_hard10_action_handoff_R14_20260516T191707Z` | `run_log.txt`, `aggregate_verdict.json`, `audit_dashboard_run_report.txt`, `cas/.turingos_cas_index.jsonl` |
| R16 | `handover/evidence/market_autonomy_lab_hard10_bcast_exact_join_R16_20260516T202248Z` | `run_log.txt`, `aggregate_verdict.json`, `audit_dashboard_run_report.txt`, `cas/.turingos_cas_index.jsonl`, `handover/directives/market_autonomy_lab/E2_CANDIDATE_REPORT.md` |
| R17 | `handover/evidence/market_autonomy_lab_hard10_real14_R17_20260516T234921Z` | `run_log.txt`, `aggregate_verdict.json`, `audit_dashboard_run_report.txt`, `cas/.turingos_cas_index.jsonl`, verifier JSON |
| R18 | `handover/evidence/market_autonomy_lab_hard10_real14_R18_bcast_off_20260517T000136Z` | `run_log.txt`, `aggregate_verdict.json`, `audit_dashboard_run_report.txt`, `cas/.turingos_cas_index.jsonl`, verifier JSON |

Independent verifier:

| Run | Verifier JSON | Verifier verdict | Exact join | Provenance coverage | BCAST shielding |
| --- | --- | ---: | ---: | --- | --- |
| R16 | `handover/directives/market_autonomy_lab/REAL14_R16_VERIFIER_REPORT.json` | `PROCEED` | 8 | 8/8 matched tx have EVDecisionTrace, MarketOpportunityTrace, PromptCapsule linkage, and live BullTrader role evidence | PASS over 278 digests / 278 role crops / 278 visible contexts |
| R17 | `handover/directives/market_autonomy_lab/REAL14_R17_VERIFIER_REPORT.json` | `PROCEED` | 0 | n/a, no matched tx | PASS over 98 digests / 98 role crops / 98 visible contexts |
| R18 | `handover/directives/market_autonomy_lab/REAL14_R18_VERIFIER_REPORT.json` | `PROCEED` | 0 | n/a, no matched tx | PASS over 0 digests / 0 role crops / 98 visible contexts |

Verifier caveat:

R16 PromptCapsule linkage is indirect via matched EVDecisionTrace because the current
`MarketDecisionTrace` schema has no direct PromptCapsule field. That is recorded as
a residual audit risk in the verifier JSON and does not upgrade the claim beyond
`E2 candidate pending audit`.

Replication interpretation:

R16 remains a single-run candidate. R17 did not replicate it under BCAST-on, and
R18 did not produce buys with BCAST off. Both R17 and R18 are clean-negative
runs: the dominant mechanism bottleneck is not `PositiveEVIgnored`, but absence
of public EV basis (`policy_insufficient_public_basis_count=40/40`).
