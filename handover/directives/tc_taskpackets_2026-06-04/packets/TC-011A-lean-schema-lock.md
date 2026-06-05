# TC-011A Lean Micro-State Schema Lock

Status: ready
Owner lane: lean-search
Risk class: Class 2 feature-layer verifier adapter
FC nodes: FC1 JudgeAI feedback, FC3 replay
Dependencies: TC-002
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `src/judges/lean_micro_state.rs`
- `tests/tc_lean_micro_state_contract.rs`

Forbidden paths: kernel, bus, sequencer, typed-tx schema, CAS schema.

Task:

Lock Lean micro-state record shape. Lean is feature/workload/verifier layer
only, not TuringOS kernel.

Tests first:

- `micro_state_ids_are_content_hashes`
- `micro_step_json_never_contains_verified_or_verdict_kind`

Rules:

- IDs are deterministic content hashes.
- No clocks, random UUIDs, or mutable counters in state ids.
- `LeanStepOutcome` variants are only `Advanced`, `Complete`, `Failed`,
  `Timeout`, `Rejected`.
- No serialized `Verified` state exists.

Ship gate:

```bash
cargo test --test tc_lean_micro_state_contract --no-fail-fast
```

Expected: command exits 0.

Audit: Formal-methods Auditor.
Verdict: `NO-VIOLATION` or `VIOLATION-FOUND lean-authority <file>:<line>`.
