# TC-009b Minsky Tape Replay

Status: ready
Owner lane: substrate
Risk class: Class 2 replay evidence
FC nodes: FC1 tape, FC3 replay
Dependencies: TC-009a
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `src/runtime/tc_universal_witness.rs`
- `tests/tc_universal_witnesses.rs`

Forbidden paths: kernel, sequencer, typed-tx schema.

Task:

Make bounded Minsky steps reconstructable from emitted tape/CAS witness facts.

Test first:

- `minsky_replay_from_genesis_is_byte_identical`
- `minsky_tamper_test_fails`
- `minsky_capped_non_halting_run_resumes_deterministically`

Claim wording:

Use “bounded replay-reconstructable witness interpreter,” not standalone proof
of full Turing completeness.

Ship gate:

```bash
cargo test --test tc_universal_witnesses --no-fail-fast
```

Expected: command exits 0.

Audit: Replay Auditor.
Verdict: `NO-VIOLATION` or `RECONSTRUCTION-FAILURE <artifact>`.
