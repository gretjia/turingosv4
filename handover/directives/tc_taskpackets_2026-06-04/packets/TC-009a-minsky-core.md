# TC-009a Bounded Minsky Interpreter Core

Status: ready
Owner lane: substrate
Risk class: Class 1 workload witness
FC nodes: FC1 workload step, FC3 replay
Dependencies: TC-101
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `src/runtime/tc_universal_witness.rs`
- `src/runtime/mod.rs`
- `tests/tc_universal_witnesses.rs`

Forbidden paths: kernel, sequencer, typed-tx schema.

Task:

Implement a bounded 2-counter Minsky interpreter as a workload witness, not a
kernel component.

Test first:

- `minsky_addition_runs_to_halt`
- `minsky_multiplication_runs_to_halt`
- `minsky_zero_branch_selects_expected_instruction`
- `minsky_copy_preserves_source_when_program_says_so`

Ship gate:

```bash
cargo test --test tc_universal_witnesses --no-fail-fast
```

Expected: command exits 0.

Audit: Formal-methods Auditor.
Verdict: `NO-VIOLATION` or `VIOLATION-FOUND witness-semantics <file>:<line>`.
