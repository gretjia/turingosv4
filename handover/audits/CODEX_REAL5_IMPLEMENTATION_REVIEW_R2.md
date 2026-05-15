# CODEX REAL-5 Implementation Review R2

Reviewer: clean-context Codex
Date: 2026-05-14
Risk: Class 4 package

Touched FC nodes / invariants:

- FC1 externalized role action loop
- FC2 role assignment + PromptCapsule replay from genesis/batch + ChainTape/CAS
- FC3 evidence/materialized view
- Art. III shielding

## Findings

### VETO-1: Production role permissions are not enforced; true evidence shows a Trader emitting VerifyTx/WorkTx.

`Agent_0` is assigned `Trader` in the trader-first real run genesis report:

- `handover/evidence/g_phase_real_5_trader_first_b12_20260514T_FINALZ/runtime_repo/genesis_report.json:112`
- `handover/evidence/g_phase_real_5_trader_first_b12_20260514T_FINALZ/runtime_repo/genesis_report.json:115`

The same run records `Agent_0` producing an accepted `VerifyTx` and multiple `WorkTx` rejections:

- `handover/evidence/g_phase_real_5_trader_first_b12_20260514T_FINALZ/runtime_repo/run_summary.json:14`
- `handover/evidence/g_phase_real_5_trader_first_b12_20260514T_FINALZ/runtime_repo/run_summary.json:29`
- `handover/evidence/g_phase_real_5_trader_first_b12_20260514T_FINALZ/runtime_repo/rejections.jsonl:8`
- `handover/evidence/g_phase_real_5_trader_first_b12_20260514T_FINALZ/runtime_repo/rejections.jsonl:11`
- `handover/evidence/g_phase_real_5_trader_first_b12_20260514T_FINALZ/runtime_repo/rejections.jsonl:14`

This contradicts REAL-5 Atom 4 and the ship-gate claim that role outputs pass through a typed gateway where disallowed role actions route to L4.E as role-policy violations. The scaffold functions exist in `src/runtime/real5_roles.rs:455` and `src/runtime/real5_roles.rs:490`, but `rg` shows `parse_role_action_json(...)` and `route_role_action(...)` are only exercised by tests, not by the evaluator production path. The role-turn trace writer records a post-hoc role outcome at `experiments/minif2f_v4/src/bin/evaluator.rs:5393`, but it does not gate the already-parsed legacy action before WorkTx / VerifyTx admission.

This is a production defect, not a test-scaffold gap. It blocks the REAL-5 completion claim for typed role gateway and role-allowed-tools enforcement.

### CHALLENGE-1: PromptCapsuleV2 does not currently reconstruct the full prompt context it hashes.

The evaluator builds the legacy full prompt, appends the REAL-5 role-view block, then hashes the full prompt:

- `experiments/minif2f_v4/src/bin/evaluator.rs:2943`
- `experiments/minif2f_v4/src/bin/evaluator.rs:2955`
- `experiments/minif2f_v4/src/bin/evaluator.rs:2964`

But `real5_write_prompt_capsule_v2_for_view(...)` stores only `role_view_bytes` as `visible_context_cid`:

- `experiments/minif2f_v4/src/bin/evaluator.rs:127`
- `experiments/minif2f_v4/src/bin/evaluator.rs:143`
- `experiments/minif2f_v4/src/bin/evaluator.rs:148`

So `PromptCapsuleV2.prompt_context_hash` binds the full prompt, while `PromptCapsuleV2.visible_context_cid` resolves only the derived role-view fragment. That leaves the full visible context unreconstructable from the capsule's CID/read-set surface. It also means the actual role run is still driven by the broad legacy `build_agent_prompt(...)` context plus an appended role block, not solely by the role-scoped derived view.

This is a FC2 replay / Art. III prompt-persistence defect. It does not show raw prompt or private CoT storage, but it weakens the claim that PromptCapsuleV2 closes the role/view replay gap.

### CHALLENGE-2: Hidden role-switch checks exist as helpers but are not wired into runtime/audit enforcement.

The implementation adds `detect_hidden_role_switch(...)` and `PromptCapsuleV2::assert_matches_assignment(...)`:

- `src/runtime/real5_roles.rs:193`
- `src/runtime/prompt_capsule.rs:129`

However, current usage is limited to tests. Runtime role resolution still reads `TURINGOS_REAL5_ROLE_ASSIGNMENT` independently at attempt time:

- `experiments/minif2f_v4/src/bin/evaluator.rs:315`

and genesis role assignment is separately built from the same env at boot:

- `experiments/minif2f_v4/src/bin/evaluator.rs:1453`

The successful evidence likely used stable env values, and genesis/batch evidence does carry the role manifest CID. But there is no production verifier or `audit_tape` assertion that every PromptCapsuleV2 / RoleTurnTrace role matches the genesis or batch role assignment. This is a forward enforcement gap for SG-R5.1.3 / SG-R5.3.2, and it becomes more serious once role permissions are actually enforced.

## Non-Blocking Observations

- I found no restricted-surface edits to `src/state/sequencer.rs`, `src/state/typed_tx.rs`, `src/kernel.rs`, `src/bus.rs`, wallet, CAS schema, or canonical signing payload. The recorded dev harness flags `genesis_payload.toml` as a restricted-surface hit; that is expected for Trust Root rehashing and must remain intentional.
- I found no evidence of private CoT or raw prompt/completion bytes being stored in the new REAL-5 CAS schemas. The new objects are CIDs, roles, hashes, public summaries, or enum/class data.
- The true-problem evidence supports only the narrowed scaffold and clean-negative market claim. It does not support E2/E3 market emergence, live agent router buys, model ranking, or persistent role-differentiated market behavior. The completion report correctly states this limitation.
- `cargo fmt --all -- --check` failed in the recorded dev run due broad existing rustfmt drift. I did not treat that as a REAL-5 product gate. `git diff --check` is recorded as exit 0.
- Several REAL-5 tests are fixture/source-grep scaffolds rather than real runtime gates. That is acceptable for early scaffold surfaces, but not sufficient for claiming Atom 4 production enforcement while evidence shows role overreach.

## Verification Reviewed

I inspected the recorded `turingos_dev` run `handover/evidence/dev_self_hosting/dev_1778782264956_718239`:

- `command_0009` exit 0: REAL-5 targeted tests, batch manifest role carry test, AttemptTelemetry v3 tests, Trust Root verify.
- `command_0010` exit 0: `bash scripts/run_constitution_gates.sh && cargo test --workspace --no-fail-fast -- --test-threads=1`.
- `command_0012` exit 0: `git diff --check`.

I also inspected the true-problem evidence paths listed in the review request, especially the trader-first adversarial role-order run that exposes VETO-1.

## Verdict

VETO

REAL-5 cannot be claimed complete yet, even as scaffold completion, because the production evaluator still allows a genesis-assigned Trader to emit proof/verify actions. Fix should wire the typed role gateway into the live action path or explicitly downgrade the claim to "role labels + CAS reason traces only" and defer Atom 4/allowed-tools enforcement.
