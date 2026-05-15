# TRACE_MATRIX Backlink Cleanup — REAL-5S -> REAL-9

Justification source: `handover/alignment/OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md`

## Summary

- R-022 skipped entries parsed: 154
- Unique file/symbol registrations added to TRACE_MATRIX §J.2: 150
- Legacy `trace_removal` audit-trail rows added to TRACE_MATRIX §J.3: 2
- Cleanup class: one-time bulk-ship exception closure, not policy relaxation.
- Implementation choice: docs-first §J registration; no restricted Rust source surface is edited for this cleanup.

## SG-10.1

```text
SG-10.1.1 No new public surface lacks TRACE/§J backlink: covered by constitution_real10_trace_cleanup.
SG-10.1.2 R-022 not treated as waiver: this report records the skip as a one-time cleanup exception.
SG-10.1.3 Future hook would not require skip for same class of change: §J entries now exist for the skipped public surfaces.
```

## Files Covered

| File | Skipped entries |
| --- | ---: |
| `src/bottom_white/ledger/transition_ledger.rs` | 1 |
| `src/bus.rs` | 2 |
| `src/drivers/llm_http.rs` | 1 |
| `src/ledger.rs` | 1 |
| `src/lib.rs` | 7 |
| `src/runtime/adapter.rs` | 3 |
| `src/runtime/agent_scheduler.rs` | 7 |
| `src/runtime/attempt_telemetry.rs` | 2 |
| `src/runtime/g7_structural_smoke.rs` | 6 |
| `src/runtime/mod.rs` | 3 |
| `src/runtime/prompt_capsule.rs` | 6 |
| `src/runtime/real5_roles.rs` | 73 |
| `src/runtime/real6_attempt_prediction.rs` | 13 |
| `src/runtime/real6_conviction_budget.rs` | 8 |
| `src/runtime/real6_task_outcome.rs` | 5 |
| `src/sdk/mod.rs` | 9 |
| `src/sdk/tools/mod.rs` | 2 |
| `src/state/typed_tx.rs` | 3 |
| `src/wal.rs` | 2 |

## Normalization Rules

```text
pub_fn_foo -> foo
pub_struct_Foo -> Foo
pub_enum_Foo -> Foo
pub_trait_Foo -> Foo
pub_const_FOO -> FOO
pub_mod_foo -> foo
pub_type_Foo -> Foo
pub_static_FOO -> FOO
pub_const_fn -> fn (matches current R-022 parser behavior for `pub const fn`)
trace_removal -> legacy cleanup-document entry represented in §J.3, not an open §J.2 public surface
```

## Registered Surfaces

| File | Line | Enforcement symbol | §J symbol |
| --- | ---: | --- | --- |
| `src/bottom_white/ledger/transition_ledger.rs` | 686 | `pub_fn_canonical_decode` | `canonical_decode` |
| `src/bus.rs` | 219 | `pub_fn_append` | `append` |
| `src/bus.rs` | 236 | `pub_fn_append_oracle_accepted` | `append_oracle_accepted` |
| `src/drivers/llm_http.rs` | 88 | `pub_fn_generate` | `generate` |
| `src/ledger.rs` | 237 | `pub_fn_append` | `append` |
| `src/lib.rs` | 2 | `pub_mod_bottom_white` | `bottom_white` |
| `src/lib.rs` | 6 | `pub_mod_kernel` | `kernel` |
| `src/lib.rs` | 7 | `pub_mod_ledger` | `ledger` |
| `src/lib.rs` | 10 | `pub_mod_sdk` | `sdk` |
| `src/lib.rs` | 11 | `pub_mod_state` | `state` |
| `src/lib.rs` | 12 | `pub_mod_top_white` | `top_white` |
| `src/lib.rs` | 13 | `pub_mod_wal` | `wal` |
| `src/runtime/adapter.rs` | 764 | `pub_fn_tb_real6a_emit_task_outcome_no_after_exhaustion` | `tb_real6a_emit_task_outcome_no_after_exhaustion` |
| `src/runtime/adapter.rs` | 1504 | `pub_fn_tb_real6a_seed_task_outcome_market_after_escrow` | `tb_real6a_seed_task_outcome_market_after_escrow` |
| `src/runtime/adapter.rs` | 1589 | `pub_fn_tb_real6a_invest_task_outcome_to_router_tx` | `tb_real6a_invest_task_outcome_to_router_tx` |
| `src/runtime/agent_scheduler.rs` | 14 | `pub_const_SCHEDULER_DECISION_TRACE_SCHEMA_ID` | `SCHEDULER_DECISION_TRACE_SCHEMA_ID` |
| `src/runtime/agent_scheduler.rs` | 37 | `pub_struct_SchedulerPnlSignal` | `SchedulerPnlSignal` |
| `src/runtime/agent_scheduler.rs` | 61 | `pub_fn_write_scheduler_decision_trace_to_cas` | `write_scheduler_decision_trace_to_cas` |
| `src/runtime/agent_scheduler.rs` | 78 | `pub_fn_read_scheduler_decision_trace_from_cas` | `read_scheduler_decision_trace_from_cas` |
| `src/runtime/agent_scheduler.rs` | 87 | `pub_fn_scheduler_decision_trace_cids` | `scheduler_decision_trace_cids` |
| `src/runtime/agent_scheduler.rs` | 130 | `pub_fn_build_observe_only_scheduler_trace` | `build_observe_only_scheduler_trace` |
| `src/runtime/agent_scheduler.rs` | 153 | `pub_fn_render_scheduler_trace_section` | `render_scheduler_trace_section` |
| `src/runtime/attempt_telemetry.rs` | 897 | `pub_fn_decode_attempt_telemetry_shared_slot` | `decode_attempt_telemetry_shared_slot` |
| `src/runtime/g7_structural_smoke.rs` | 9 | `pub_const_G7_STRUCTURAL_GUARD_SCHEMA_ID` | `G7_STRUCTURAL_GUARD_SCHEMA_ID` |
| `src/runtime/g7_structural_smoke.rs` | 45 | `pub_struct_G7StructuralGuard` | `G7StructuralGuard` |
| `src/runtime/g7_structural_smoke.rs` | 57 | `pub_fn_structural_fixture` | `structural_fixture` |
| `src/runtime/g7_structural_smoke.rs` | 72 | `pub_struct_G7StructuralGuardSummary` | `G7StructuralGuardSummary` |
| `src/runtime/g7_structural_smoke.rs` | 81 | `pub_fn_write_g7_structural_guard_to_cas` | `write_g7_structural_guard_to_cas` |
| `src/runtime/g7_structural_smoke.rs` | 98 | `pub_fn_summarize_g7_structural_guards_from_cas` | `summarize_g7_structural_guards_from_cas` |
| `src/runtime/mod.rs` | 226 | `pub_mod_real6_task_outcome` | `real6_task_outcome` |
| `src/runtime/mod.rs` | 230 | `pub_mod_real6_attempt_prediction` | `real6_attempt_prediction` |
| `src/runtime/mod.rs` | 234 | `pub_mod_real6_conviction_budget` | `real6_conviction_budget` |
| `src/runtime/prompt_capsule.rs` | 64 | `pub_const_PROMPT_CAPSULE_V2_SCHEMA_ID` | `PROMPT_CAPSULE_V2_SCHEMA_ID` |
| `src/runtime/prompt_capsule.rs` | 115 | `pub_struct_PromptCapsuleV2` | `PromptCapsuleV2` |
| `src/runtime/prompt_capsule.rs` | 129 | `pub_fn_assert_matches_assignment` | `assert_matches_assignment` |
| `src/runtime/prompt_capsule.rs` | 154 | `pub_fn_read_set_resolves` | `read_set_resolves` |
| `src/runtime/prompt_capsule.rs` | 270 | `pub_fn_write_prompt_capsule_v2_to_cas` | `write_prompt_capsule_v2_to_cas` |
| `src/runtime/prompt_capsule.rs` | 289 | `pub_fn_read_prompt_capsule_v2_from_cas` | `read_prompt_capsule_v2_from_cas` |
| `src/runtime/real5_roles.rs` | 21 | `pub_const_ROLE_ASSIGNMENT_MANIFEST_SCHEMA_ID` | `ROLE_ASSIGNMENT_MANIFEST_SCHEMA_ID` |
| `src/runtime/real5_roles.rs` | 22 | `pub_const_ROLE_TURN_TRACE_SCHEMA_ID` | `ROLE_TURN_TRACE_SCHEMA_ID` |
| `src/runtime/real5_roles.rs` | 24 | `pub_type_ToolName` | `ToolName` |
| `src/runtime/real5_roles.rs` | 25 | `pub_type_PolicyId` | `PolicyId` |
| `src/runtime/real5_roles.rs` | 26 | `pub_type_HeadT` | `HeadT` |
| `src/runtime/real5_roles.rs` | 29 | `pub_enum_AgentRole` | `AgentRole` |
| `src/runtime/real5_roles.rs` | 41 | `pub_const_ALL` | `ALL` |
| `src/runtime/real5_roles.rs` | 52 | `pub_const_fn` | `fn` |
| `src/runtime/real5_roles.rs` | 91 | `pub_struct_AgentRoleAssignment` | `AgentRoleAssignment` |
| `src/runtime/real5_roles.rs` | 101 | `pub_struct_RoleAssignmentManifest` | `RoleAssignmentManifest` |
| `src/runtime/real5_roles.rs` | 108 | `pub_fn_sorted_role_assignment` | `sorted_role_assignment` |
| `src/runtime/real5_roles.rs` | 115 | `pub_fn_default_allowed_tools` | `default_allowed_tools` |
| `src/runtime/real5_roles.rs` | 128 | `pub_fn_role_assignment_from_csv` | `role_assignment_from_csv` |
| `src/runtime/real5_roles.rs` | 167 | `pub_fn_write_role_assignment_manifest_to_cas` | `write_role_assignment_manifest_to_cas` |
| `src/runtime/real5_roles.rs` | 184 | `pub_fn_read_role_assignment_manifest_from_cas` | `read_role_assignment_manifest_from_cas` |
| `src/runtime/real5_roles.rs` | 193 | `pub_fn_detect_hidden_role_switch` | `detect_hidden_role_switch` |
| `src/runtime/real5_roles.rs` | 220 | `pub_fn_render_role_assignment_dashboard` | `render_role_assignment_dashboard` |
| `src/runtime/real5_roles.rs` | 236 | `pub_struct_DerivedViewRequest` | `DerivedViewRequest` |
| `src/runtime/real5_roles.rs` | 244 | `pub_struct_PriceSignal` | `PriceSignal` |
| `src/runtime/real5_roles.rs` | 251 | `pub_struct_PublicErrorSummary` | `PublicErrorSummary` |
| `src/runtime/real5_roles.rs` | 257 | `pub_struct_DerivedView` | `DerivedView` |
| `src/runtime/real5_roles.rs` | 268 | `pub_struct_DerivedViewInput` | `DerivedViewInput` |
| `src/runtime/real5_roles.rs` | 275 | `pub_fn_fixture` | `fixture` |
| `src/runtime/real5_roles.rs` | 287 | `pub_fn_derive_role_view` | `derive_role_view` |
| `src/runtime/real5_roles.rs` | 294 | `pub_fn_derive_role_view_with_context_bytes` | `derive_role_view_with_context_bytes` |
| `src/runtime/real5_roles.rs` | 367 | `pub_struct_WorkTxPayload` | `WorkTxPayload` |
| `src/runtime/real5_roles.rs` | 372 | `pub_struct_VerifyPeerPayload` | `VerifyPeerPayload` |
| `src/runtime/real5_roles.rs` | 377 | `pub_struct_ChallengePayload` | `ChallengePayload` |
| `src/runtime/real5_roles.rs` | 382 | `pub_struct_MarketInvestPayload` | `MarketInvestPayload` |
| `src/runtime/real5_roles.rs` | 388 | `pub_struct_LiquidityPayload` | `LiquidityPayload` |
| `src/runtime/real5_roles.rs` | 394 | `pub_struct_ToolProposalPayload` | `ToolProposalPayload` |
| `src/runtime/real5_roles.rs` | 399 | `pub_struct_VetoPayload` | `VetoPayload` |
| `src/runtime/real5_roles.rs` | 404 | `pub_struct_AbstainPayload` | `AbstainPayload` |
| `src/runtime/real5_roles.rs` | 409 | `pub_enum_RoleAction` | `RoleAction` |
| `src/runtime/real5_roles.rs` | 421 | `pub_const_fn` | `fn` |
| `src/runtime/real5_roles.rs` | 436 | `pub_struct_RoleActionRejection` | `RoleActionRejection` |
| `src/runtime/real5_roles.rs` | 442 | `pub_enum_RoleActionRoute` | `RoleActionRoute` |
| `src/runtime/real5_roles.rs` | 455 | `pub_fn_parse_role_action_json` | `parse_role_action_json` |
| `src/runtime/real5_roles.rs` | 490 | `pub_fn_legacy_tool_to_role_action` | `legacy_tool_to_role_action` |
| `src/runtime/real5_roles.rs` | 513 | `pub_fn_route_role_action` | `route_role_action` |
| `src/runtime/real5_roles.rs` | 560 | `pub_struct_TickBudget` | `TickBudget` |
| `src/runtime/real5_roles.rs` | 568 | `pub_enum_TickEvent` | `TickEvent` |
| `src/runtime/real5_roles.rs` | 575 | `pub_fn_derive_tick_budget` | `derive_tick_budget` |
| `src/runtime/real5_roles.rs` | 599 | `pub_struct_RationalPrice` | `RationalPrice` |
| `src/runtime/real5_roles.rs` | 605 | `pub_fn_new` | `new` |
| `src/runtime/real5_roles.rs` | 618 | `pub_struct_NoTradeReasonTrace` | `NoTradeReasonTrace` |
| `src/runtime/real5_roles.rs` | 631 | `pub_enum_TraderTurnWitness` | `TraderTurnWitness` |
| `src/runtime/real5_roles.rs` | 636 | `pub_fn_verify_trader_turns` | `verify_trader_turns` |
| `src/runtime/real5_roles.rs` | 654 | `pub_struct_ScriptedTradeRoute` | `ScriptedTradeRoute` |
| `src/runtime/real5_roles.rs` | 659 | `pub_fn_scripted_positive_edge_trade` | `scripted_positive_edge_trade` |
| `src/runtime/real5_roles.rs` | 674 | `pub_struct_VerifyPeerFixture` | `VerifyPeerFixture` |
| `src/runtime/real5_roles.rs` | 681 | `pub_fn_verify_peer_fixture` | `verify_peer_fixture` |
| `src/runtime/real5_roles.rs` | 694 | `pub_fn_apply_verifier_reputation_delta` | `apply_verifier_reputation_delta` |
| `src/runtime/real5_roles.rs` | 703 | `pub_struct_NoVerifyReason` | `NoVerifyReason` |
| `src/runtime/real5_roles.rs` | 709 | `pub_enum_VerifierTurnWitness` | `VerifierTurnWitness` |
| `src/runtime/real5_roles.rs` | 715 | `pub_struct_NoChallengeReason` | `NoChallengeReason` |
| `src/runtime/real5_roles.rs` | 721 | `pub_struct_ChallengeDecisionTrace` | `ChallengeDecisionTrace` |
| `src/runtime/real5_roles.rs` | 728 | `pub_fn_challenge_decision_trace` | `challenge_decision_trace` |
| `src/runtime/real5_roles.rs` | 747 | `pub_enum_RoleTurnOutcome` | `RoleTurnOutcome` |
| `src/runtime/real5_roles.rs` | 785 | `pub_struct_RoleTurnTrace` | `RoleTurnTrace` |
| `src/runtime/real5_roles.rs` | 796 | `pub_fn_new` | `new` |
| `src/runtime/real5_roles.rs` | 816 | `pub_fn_write_role_turn_trace_to_cas` | `write_role_turn_trace_to_cas` |
| `src/runtime/real5_roles.rs` | 834 | `pub_struct_RoleTurnTraceSummary` | `RoleTurnTraceSummary` |
| `src/runtime/real5_roles.rs` | 846 | `pub_fn_summarize_role_turn_traces_from_cas` | `summarize_role_turn_traces_from_cas` |
| `src/runtime/real5_roles.rs` | 889 | `pub_struct_MetricEstimate` | `MetricEstimate` |
| `src/runtime/real5_roles.rs` | 896 | `pub_struct_ToolProposal` | `ToolProposal` |
| `src/runtime/real5_roles.rs` | 904 | `pub_enum_VetoVerdict` | `VetoVerdict` |
| `src/runtime/real5_roles.rs` | 910 | `pub_enum_VetoReasonClass` | `VetoReasonClass` |
| `src/runtime/real5_roles.rs` | 917 | `pub_struct_VetoDecision` | `VetoDecision` |
| `src/runtime/real5_roles.rs` | 924 | `pub_fn_proposal_activation_status` | `proposal_activation_status` |
| `src/runtime/real5_roles.rs` | 941 | `pub_struct_Real5SmokeInput` | `Real5SmokeInput` |
| `src/runtime/real5_roles.rs` | 959 | `pub_struct_Real5SmokeReport` | `Real5SmokeReport` |
| `src/runtime/real5_roles.rs` | 966 | `pub_fn_evaluate_real5_smoke` | `evaluate_real5_smoke` |
| `src/runtime/real6_attempt_prediction.rs` | 18 | `pub_const_REAL6B_SCHEMA_ID` | `REAL6B_SCHEMA_ID` |
| `src/runtime/real6_attempt_prediction.rs` | 19 | `pub_const_REAL6B_STAGE_LIMIT` | `REAL6B_STAGE_LIMIT` |
| `src/runtime/real6_attempt_prediction.rs` | 23 | `pub_enum_LeanOracleResult` | `LeanOracleResult` |
| `src/runtime/real6_attempt_prediction.rs` | 29 | `pub_enum_AttemptPredictionStepKind` | `AttemptPredictionStepKind` |
| `src/runtime/real6_attempt_prediction.rs` | 40 | `pub_struct_AttemptPredictionStep` | `AttemptPredictionStep` |
| `src/runtime/real6_attempt_prediction.rs` | 53 | `pub_const_fn` | `fn` |
| `src/runtime/real6_attempt_prediction.rs` | 64 | `pub_struct_AttemptPredictionFixture` | `AttemptPredictionFixture` |
| `src/runtime/real6_attempt_prediction.rs` | 83 | `pub_fn_first_logical_t` | `first_logical_t` |
| `src/runtime/real6_attempt_prediction.rs` | 91 | `pub_fn_attempt_prediction_event_id` | `attempt_prediction_event_id` |
| `src/runtime/real6_attempt_prediction.rs` | 99 | `pub_fn_build_scripted_attempt_prediction_fixture` | `build_scripted_attempt_prediction_fixture` |
| `src/runtime/real6_attempt_prediction.rs` | 229 | `pub_fn_validate_attempt_prediction_fixture` | `validate_attempt_prediction_fixture` |
| `src/runtime/real6_attempt_prediction.rs` | 318 | `pub_fn_write_attempt_prediction_fixture_to_cas` | `write_attempt_prediction_fixture_to_cas` |
| `src/runtime/real6_attempt_prediction.rs` | 338 | `pub_fn_attempt_prediction_fixture_cids` | `attempt_prediction_fixture_cids` |
| `src/runtime/real6_conviction_budget.rs` | 26 | `pub_struct_ConvictionBudget` | `ConvictionBudget` |
| `src/runtime/real6_conviction_budget.rs` | 36 | `pub_enum_ConvictionAction` | `ConvictionAction` |
| `src/runtime/real6_conviction_budget.rs` | 47 | `pub_struct_ConvictionActionAvailability` | `ConvictionActionAvailability` |
| `src/runtime/real6_conviction_budget.rs` | 52 | `pub_fn_derive_conviction_budget` | `derive_conviction_budget` |
| `src/runtime/real6_conviction_budget.rs` | 88 | `pub_fn_conviction_action_allowed` | `conviction_action_allowed` |
| `src/runtime/real6_conviction_budget.rs` | 117 | `pub_fn_route_role_action_with_conviction_budget` | `route_role_action_with_conviction_budget` |
| `src/runtime/real6_conviction_budget.rs` | 161 | `pub_fn_render_scoped_conviction_budget_summary` | `render_scoped_conviction_budget_summary` |
| `src/runtime/real6_conviction_budget.rs` | 183 | `pub_fn_write_significant_loss_autopsy_to_cas` | `write_significant_loss_autopsy_to_cas` |
| `src/runtime/real6_task_outcome.rs` | 16 | `pub_enum_TaskOutcomeMarketKind` | `TaskOutcomeMarketKind` |
| `src/runtime/real6_task_outcome.rs` | 22 | `pub_struct_TaskOutcomeEvent` | `TaskOutcomeEvent` |
| `src/runtime/real6_task_outcome.rs` | 31 | `pub_fn_task_outcome_event_for_task` | `task_outcome_event_for_task` |
| `src/runtime/real6_task_outcome.rs` | 49 | `pub_fn_task_outcome_price_signal` | `task_outcome_price_signal` |
| `src/runtime/real6_task_outcome.rs` | 62 | `pub_struct_TaskOutcomeMarketSeedOutcome` | `TaskOutcomeMarketSeedOutcome` |
| `src/sdk/mod.rs` | 1 | `pub_mod_actor` | `actor` |
| `src/sdk/mod.rs` | 10 | `pub_mod_error_abstraction` | `error_abstraction` |
| `src/sdk/mod.rs` | 18 | `pub_mod_prompt` | `prompt` |
| `src/sdk/mod.rs` | 19 | `pub_mod_prompt_guard` | `prompt_guard` |
| `src/sdk/mod.rs` | 20 | `pub_mod_protocol` | `protocol` |
| `src/sdk/mod.rs` | 21 | `pub_mod_sandbox` | `sandbox` |
| `src/sdk/mod.rs` | 22 | `pub_mod_snapshot` | `snapshot` |
| `src/sdk/mod.rs` | 23 | `pub_mod_tool` | `tool` |
| `src/sdk/mod.rs` | 24 | `pub_mod_tools` | `tools` |
| `src/sdk/tools/mod.rs` | 2 | `pub_mod_search` | `search` |
| `src/sdk/tools/mod.rs` | 3 | `pub_mod_wallet` | `wallet` |
| `src/state/typed_tx.rs` | 2011 | `pub_fn_canonical_digest` | `canonical_digest` |
| `src/state/typed_tx.rs` | 2144 | `pub_fn_to_legacy_signing_payload` | `to_legacy_signing_payload` |
| `src/wal.rs` | 45 | `pub_fn_path` | `path` |
| `src/wal.rs` | 69 | `pub_fn_replay` | `replay` |

## Closed Audit-Trail Entries

These entries came from R-022 `trace_removal` cleanup lines. They are represented in TRACE_MATRIX §J.3, not §J.2, because they are historical backlink removals rather than new open public API surfaces.

| File | Enforcement symbol | TRACE_MATRIX section |
| --- | --- | --- |
| `src/runtime/attempt_telemetry.rs` | `trace_removal` | §J.3 closed / graduated rows |
| `src/state/typed_tx.rs` | `trace_removal` | §J.3 closed / graduated rows |
