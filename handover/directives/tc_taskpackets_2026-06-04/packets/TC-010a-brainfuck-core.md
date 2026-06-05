# TC-010a Bounded Brainfuck Interpreter Core

Status: ready
Owner lane: substrate
Risk class: Class 1 workload witness
FC nodes: FC1 workload step, FC3 replay
Dependencies: TC-101
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `src/runtime/tc_universal_witness.rs`
- `tests/tc_universal_witnesses.rs`

Forbidden paths: kernel, sequencer, typed-tx schema.

Task:

Implement a bounded Brainfuck interpreter as an independent workload witness.

Test first:

- `brainfuck_loop_runs_to_halt`
- `brainfuck_copy_program_preserves_expected_cell`
- `brainfuck_output_program_emits_expected_bytes`

Ship gate:

```bash
cargo test --test tc_universal_witnesses --no-fail-fast
```

Expected: command exits 0.

Audit: Formal-methods Auditor.
Verdict: `NO-VIOLATION` or `VIOLATION-FOUND witness-semantics <file>:<line>`.
