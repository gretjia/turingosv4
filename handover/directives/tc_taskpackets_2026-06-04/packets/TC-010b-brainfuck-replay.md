# TC-010b Brainfuck Tape Replay

Status: ready
Owner lane: substrate
Risk class: Class 2 replay evidence
FC nodes: FC1 tape, FC3 replay
Dependencies: TC-010a
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `src/runtime/tc_universal_witness.rs`
- `tests/tc_universal_witnesses.rs`

Forbidden paths: kernel, sequencer, typed-tx schema.

Task:

Make Brainfuck steps reconstructable from emitted tape/CAS witness facts.

Test first:

- `brainfuck_replay_from_genesis_is_byte_identical`
- `brainfuck_tamper_test_fails`
- `brainfuck_capped_non_halting_run_resumes_deterministically`

Ship gate:

```bash
cargo test --test tc_universal_witnesses --no-fail-fast
```

Expected: command exits 0.

Audit: Replay Auditor.
Verdict: `NO-VIOLATION` or `RECONSTRUCTION-FAILURE <artifact>`.
