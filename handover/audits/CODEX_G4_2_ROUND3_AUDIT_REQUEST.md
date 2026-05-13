# CODEX G4.2 Round 3 Clean-Context Audit Request

Date: 2026-05-13
Task: G4.2 model identity replay / no hidden switch
Risk class: 4
Verdict required: `PROCEED | CHALLENGE | VETO`

## Ratification Source

Use the archived architect ratification as the primary local fact source:

- `handover/directives/2026-05-13_TB_G_G4_2_§8_ARCHITECT_RATIFICATION.md`

Key ratified chain:

```text
Agent_i
-> genesis-assigned model identity
-> AttemptTelemetry actual model
-> audit assertion: no hidden model switch
-> dashboard/report divergence by model family
```

Key boundary:

```text
G4.2 is model identity replay + hidden switch prevention.
It is not multi-model market behavior research.
```

## FC / Invariants

- FC2 Boot / Genesis: `GenesisReport.agent_model_assignment` is the replay authority.
- FC1 AttemptTelemetry: every new LLM attempt records actual model identity.
- FC3 audit/report view: no-hidden-switch assertion and model-family report are derived from GenesisReport + AttemptTelemetry + ChainTape + CAS.
- Art. III shielding: PromptCapsule model linkage must not store raw prompt/completion/CoT.

## Prior Review Closures To Verify

Round 1 verdict file:

- `handover/audits/CODEX_G4_2_ROUND1_VERDICT.md`

Round 1 findings to verify closed:

- AttemptTelemetry must use provider/proxy actual model (`GenerateResponse.model`) on success paths.
- Hidden switch must be a blocking `audit_tape` assertion, not dashboard-only.
- ModelAssignmentManifest CAS write must fail closed for new G4.2 runs.

Round 2 verdict file:

- `handover/audits/CODEX_G4_2_ROUND2_VERDICT.md`

Round 2 finding to verify closed:

- New G4.2 model-identity runs must not lose `genesis_report.json` and pass hidden-switch audit as "historical"; genesis report write failure for populated model assignment / manifest CID must fail closed.

## Files / Surfaces

Review current working tree diff plus untracked G4.2 files. Intended G4.2 paths include:

- `src/runtime/genesis_report.rs`
- `src/runtime/attempt_telemetry.rs`
- `src/runtime/audit_assertions.rs`
- `src/bin/audit_dashboard.rs`
- `experiments/minif2f_v4/src/agent_models.rs`
- `experiments/minif2f_v4/src/chain_runtime.rs`
- `experiments/minif2f_v4/src/bin/evaluator.rs`
- `scripts/run_g_phase_batch.sh`
- `tests/constitution_g4_multi_llm.rs`
- `tests/constitution_g4_no_hidden_model_switch.rs`
- `tests/constitution_prompt_capsule.rs`
- `genesis_payload.toml`
- `handover/directives/2026-05-13_TB_G_G4_2_§8_ARCHITECT_RATIFICATION.md`

Known dirty files that pre-existed or are verification-generated and are not intended implementation scope:

- `h_vppu_history.json`
- `rules/enforcement.log`

Forbidden stop-condition surfaces must remain untouched:

- `src/state/sequencer.rs`
- `src/state/typed_tx.rs`
- canonical signing payloads
- `src/kernel.rs`
- `src/bus.rs`

## Evidence Paths

Current dev harness run:

- `handover/evidence/dev_self_hosting/dev_1778674964290_3969679`

Fresh current-source G4.2 smoke:

- `handover/evidence/g_phase_g4_2_mini_challenge_fix_2026-05-13T14-33-04Z`
- `G_PHASE_BATCH_MANIFEST.json`: required families = 3, observed families = 4
- `aggregate_verdict.json`: audit_tape verdict = PROCEED
- `g4_2_run_report.md`: §G.3 model-family activity, `hidden_switch_verdict: Proceed`, no ranking claim
- `PERSISTENCE_BINDING_REPORT.json`: passing with witnessed persistence count

Earlier failed smoke preflight is also preserved as fail-closed evidence:

- `handover/evidence/g_phase_g4_2_mini_challenge_fix_2026-05-13T14-32-40Z`

## Verification Commands

All commands below are recorded in the dev harness run artifacts.

- command_0040: red test for round-2 CHALLENGE closure failed as expected before fix.
- command_0041: `cargo test --test constitution_g4_no_hidden_model_switch -- --nocapture` passed after fix.
- command_0044: `cargo test --test constitution_g4_multi_llm -- --nocapture` passed.
- command_0045: `cargo test --test constitution_prompt_capsule -- --nocapture` passed.
- command_0046: `cargo test --test constitution_g4_no_hidden_model_switch -- --nocapture` passed.
- command_0047: `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo -- --nocapture` passed.
- command_0048: `cargo build --bin audit_tape --bin audit_dashboard` passed.
- command_0049: `target/debug/audit_tape ... --out aggregate_verdict_after_challenge_fix.json` passed with `verdict=PROCEED passed=41 failed=0 halted=0 skipped=11`.
- command_0050: `target/debug/audit_dashboard ... --run-report` passed.
- command_0051: `bash scripts/run_constitution_gates.sh` passed.
- command_0052: `cargo test --workspace --no-fail-fast -- --test-threads=1` passed.
- command_0054: fresh smoke without dirty-tree override failed closed at runner preflight, preserved as evidence.
- command_0055: fresh current-source smoke with `TURINGOS_G_PHASE_DIRTY_OK=1`, `PHASE_D_HETERO_OK=1`, and 10 `AGENT_MODELS` passed.
- command_0056: `target/debug/audit_dashboard ... --run-report` passed for the fresh current-source smoke.

## Required Review Shape

Lead with findings ordered by severity and cite files/lines. Distinguish production defects from test-scaffold gaps. End with exactly one verdict:

```text
Verdict: PROCEED
```

or

```text
Verdict: CHALLENGE
```

or

```text
Verdict: VETO
```
