# CODEX REAL-5 Implementation Review R3

Reviewer: clean-context Codex
Date: 2026-05-14
Risk: Class 4 package

Touched FC nodes / invariants:

- FC1 externalized role action loop
- FC2 role assignment + PromptCapsule replay from genesis/batch + ChainTape/CAS
- FC3 evidence/materialized view
- Art. III shielding

## Findings

### No blocking production defects found.

I do not find the R2 VETO pattern still present after remediation. The production evaluator now gates parsed legacy tools through the REAL-5 role gateway before entering the old action dispatch.

The live action path parses the model output, maps the legacy tool into a typed REAL-5 role action, routes it against the runtime role, and only enters the legacy `match action.tool.as_str()` branch when the route is `L4`:

- `experiments/minif2f_v4/src/bin/evaluator.rs:421`
- `experiments/minif2f_v4/src/bin/evaluator.rs:425`
- `experiments/minif2f_v4/src/bin/evaluator.rs:3279`
- `experiments/minif2f_v4/src/bin/evaluator.rs:3283`
- `experiments/minif2f_v4/src/bin/evaluator.rs:3297`
- `experiments/minif2f_v4/src/bin/evaluator.rs:3351`
- `experiments/minif2f_v4/src/bin/evaluator.rs:3404`

The role/action matrix blocks Trader proof/verify output and routes it to `L4E` policy rejection rather than to WorkTx / VerifyTx production branches:

- `src/runtime/real5_roles.rs:490`
- `src/runtime/real5_roles.rs:515`
- `src/runtime/real5_roles.rs:523`
- `src/runtime/real5_roles.rs:536`
- `src/runtime/real5_roles.rs:543`

This is a production remediation, not just a test-scaffold change.

### R2 VETO evidence pattern is removed in the post-VETO trader-first run.

`Agent_0` is still assigned `Trader` in the trader-first post-VETO genesis evidence, with allowed tools `invest` and `abstain`:

- `handover/evidence/g_phase_real_5_trader_first_b8_rolegate_20260514T192523Z/runtime_repo/genesis_report.json`

The post-VETO `run_summary.json` no longer contains accepted `worktx-*Agent_0*` or `verifytx-Agent_0*` entries. My spot check over the accepted IDs found:

```text
g_phase_real_5_trader_first_b8_rolegate_20260514T192523Z accepted Agent_0 work/verify: []
g_phase_real_5_core3_b8_rolegate_20260514T192958Z accepted Agent_0 work/verify: []
```

The trader-first evidence instead records role-policy rejection rows:

- `handover/evidence/g_phase_real_5_trader_first_b8_rolegate_20260514T192523Z/runtime_repo/run_summary.json`
- `handover/evidence/g_phase_real_5_trader_first_b8_rolegate_20260514T192523Z/P000_mathd_algebra_107/evaluator.stdout:1`
- `handover/evidence/g_phase_real_5_trader_first_b8_rolegate_20260514T192523Z/P001_mathd_algebra_125/evaluator.stdout:1`
- `handover/evidence/g_phase_real_5_trader_first_b8_rolegate_20260514T192523Z/P002_mathd_algebra_141/evaluator.stdout:1`

Important distinction: `rejections.jsonl` still contains rejected `Work` rows for Agent_0. These are the L4.E failure-path carrier emitted by `real5_emit_role_gateway_rejection_to_l4e(...)`, not legacy proof dispatch acceptance:

- `experiments/minif2f_v4/src/bin/evaluator.rs:697`
- `experiments/minif2f_v4/src/bin/evaluator.rs:718`
- `experiments/minif2f_v4/src/bin/evaluator.rs:747`

That carrier shape is acceptable for the R2 remediation question because the old accepted Trader WorkTx/VerifyTx pattern is gone.

### PromptCapsuleV2 now binds `visible_context_cid` to the full prompt bytes whose hash is `prompt_context_hash`.

The R2 mismatch is fixed. The evaluator now passes `prompt.as_bytes()` as `visible_context_bytes`, writes those bytes to CAS, computes `Cid::from_content(visible_context_bytes)`, and fails closed if that digest does not equal `prompt_ctx_hash`:

- `experiments/minif2f_v4/src/bin/evaluator.rs:130`
- `experiments/minif2f_v4/src/bin/evaluator.rs:139`
- `experiments/minif2f_v4/src/bin/evaluator.rs:141`
- `experiments/minif2f_v4/src/bin/evaluator.rs:3136`
- `experiments/minif2f_v4/src/bin/evaluator.rs:3154`
- `experiments/minif2f_v4/src/bin/evaluator.rs:3160`

The derived role-view fragment is now stored separately and added to the read set:

- `experiments/minif2f_v4/src/bin/evaluator.rs:147`
- `experiments/minif2f_v4/src/bin/evaluator.rs:156`
- `experiments/minif2f_v4/src/bin/evaluator.rs:162`

The post-VETO CAS index reflects that separation: prompt visible-context objects are full prompt-sized objects, while derived-view objects are small role-view objects:

- `handover/evidence/g_phase_real_5_trader_first_b8_rolegate_20260514T192523Z/cas/.turingos_cas_index.jsonl`
- `handover/evidence/g_phase_real_5_core3_b8_rolegate_20260514T192958Z/cas/.turingos_cas_index.jsonl`

I also spot-checked CAS content hashes for trader-first `real5.prompt.visible_context.v1` and `real5.derived_view.v1` objects; their SHA-256 digests matched the indexed CIDs. This satisfies the specific R2 PromptCapsule replay defect.

### Hidden role switch is mitigated in the runtime path; audit assertion coverage remains a non-blocking gap.

The live turn role is now resolved from `agent_role_by_id`, which is constructed once from the startup role assignment vector used for genesis/batch evidence:

- `experiments/minif2f_v4/src/bin/evaluator.rs:1607`
- `experiments/minif2f_v4/src/bin/evaluator.rs:1631`
- `experiments/minif2f_v4/src/bin/evaluator.rs:3090`
- `experiments/minif2f_v4/src/bin/evaluator.rs:3091`

Missing genesis role assignment fails closed when the REAL-5 role gateway is active:

- `experiments/minif2f_v4/src/bin/evaluator.rs:3092`
- `experiments/minif2f_v4/src/bin/evaluator.rs:3095`

Genesis and batch continuation evidence carry the role assignment manifest CID:

- `experiments/minif2f_v4/src/chain_runtime.rs:556`
- `experiments/minif2f_v4/src/chain_runtime.rs:597`
- `experiments/minif2f_v4/src/batch_orchestrator.rs:321`
- `src/runtime/genesis_report.rs:185`
- `src/runtime/batch_continuation_manifest.rs:79`
- `handover/evidence/g_phase_real_5_trader_first_b8_rolegate_20260514T192523Z/BatchContinuationManifest.json`
- `handover/evidence/g_phase_real_5_core3_b8_rolegate_20260514T192958Z/BatchContinuationManifest.json`

Residual gap: `audit_tape` does not yet have a dedicated assertion that every `PromptCapsuleV2` and `RoleTurnTrace` role matches the genesis/batch role-assignment manifest. The helper exists in tests, and runtime selection is no longer independently reparsing per turn, so I do not treat this as a blocking hidden-role-switch defect for this package. It is a test/audit coverage gap worth closing in a follow-up.

## Non-Blocking Observations

- `AgentRoleAssignment.allowed_tools` is persisted, but the production gate currently enforces the hard-coded role/action matrix rather than consulting per-assignment `allowed_tools`. Because assignments are currently generated from `default_allowed_tools(...)`, this is not a behavioral defect in the submitted evidence. It would matter if future manifests allow custom per-agent tool overrides.
- Role-policy rejections use an L4.E failure-path `WorkTx` carrier whose rejection class may appear as the base predicate-failure class. The role-specific reason is preserved in `RoleTurnTrace` / tool distribution and the `real5_role_policy_reject-*` tx IDs. This is acceptable for the R2 fix but could be clearer in materialized reports.
- Prompt visible-context bytes are now CAS-resolvable as required by the R2 replay fix. I did not find raw completion or private CoT storage. The visible prompt CAS object should remain out of ordinary agent read views; current evidence does not show it being broadcast through dashboards or prompts.
- I found no edits to `src/state/sequencer.rs`, `src/state/typed_tx.rs`, `src/kernel.rs`, `src/bus.rs`, wallet, CAS schema, or canonical signing payload surfaces. The Trust Root-pinned `genesis_payload.toml` update rehashes changed source and was verified by the recorded Trust Root test.

## Verification Reviewed

Recorded dev run:

- `handover/evidence/dev_self_hosting/dev_1778782264956_718239`

Reviewed passing commands:

- `command_0015` exit 0: targeted REAL-5 typed gateway, PromptCapsuleV2, role smoke tests, evaluator check, Trust Root verify, and `git diff --check`.
- `command_0016` exit 0: `bash scripts/run_constitution_gates.sh && cargo test --workspace --no-fail-fast -- --test-threads=1`.

Reviewed post-VETO evidence:

- `handover/evidence/g_phase_real_5_trader_first_b8_rolegate_20260514T192523Z`
- `handover/evidence/g_phase_real_5_core3_b8_rolegate_20260514T192958Z`
- `handover/evidence/real5_overnight_20260514/REAL5_COMPLETION_AND_ADVERSARIAL_TEST_REPORT.md`

Both post-VETO runs have `audit_tape` verdict `PROCEED`, CAS bytes/CID checks pass, money conservation assertions pass, and Art. III projection checks pass in the audited aggregate verdicts.

PROCEED
