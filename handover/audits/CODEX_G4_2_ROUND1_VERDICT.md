# CODEX G4.2 Round 1 Clean-Context Verdict

Date: 2026-05-13
Reviewer: clean-context Codex
Verdict: VETO

## Findings

[P0] AttemptTelemetry records the requested model, not the actual model returned by the proxy.
[src/drivers/llm_http.rs](/home/zephryj/projects/turingosv4/src/drivers/llm_http.rs:137) parses `body["model"]` into `GenerateResponse.model`, but the evaluator ignores that field and writes `agent_model` into telemetry at the R2 callsites, e.g. [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:3118) and [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:4137). That means the audit compares genesis assignment against the requested model string, not the provider/proxy-reported actual model. A proxy reroute or provider fallback would be invisible, directly breaking the ratified chain `genesis-assigned model identity -> AttemptTelemetry actual model -> no hidden model switch`.

[P0] Hidden model switch does not block the ship audit path.
The new model-identity audit exists as `audit_model_identity_from_paths`, but it is only rendered by `audit_dashboard --run-report` at [audit_dashboard.rs](/home/zephryj/projects/turingosv4/src/bin/audit_dashboard.rs:2528). It is not part of `audit_tape` / `run_all_assertions`; the assertion battery ends with the existing AttemptTelemetry/LeanResult/tamper checks at [audit_assertions.rs](/home/zephryj/projects/turingosv4/src/runtime/audit_assertions.rs:3506) through [audit_assertions.rs](/home/zephryj/projects/turingosv4/src/runtime/audit_assertions.rs:3574). Therefore a hidden-switch `Block` would be dashboard text, not a blocking audit verdict. This contradicts the ratified requirement that mismatch "blocks audit."

[P1] Model-assignment provenance CAS write is fail-soft.
`write_synthetic_l4_l4e_gate_and_genesis_report_with_model_assignment` logs and continues if the `ModelAssignmentManifest` CAS write fails, leaving `model_assignment_manifest_cid` as `None` at [chain_runtime.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/chain_runtime.rs:528). For G4.2 ship evidence, resolver provenance is part of the required replay fact set, so this should fail closed for new G4.2 runs.

## Non-Blocking Checks

No diff was present for the forbidden stop-condition surfaces: `sequencer.rs`, `typed_tx.rs`, signing payloads, `kernel.rs`, or `bus.rs`.

The smoke evidence does show 10 genesis assignments, 4 observed model families, no ranking claim in §G.3, and a run-report `hidden_switch_verdict: Proceed`; however, because "actual model" is not actually sourced from the response and the hidden-switch check is not in the blocking audit path, that evidence does not establish the G4.2 Class-4 gate.

Verdict: VETO
