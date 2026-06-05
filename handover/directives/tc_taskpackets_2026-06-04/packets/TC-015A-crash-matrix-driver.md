# TC-015A RAM Statelessness Crash Matrix Driver

Status: ready
Owner lane: reliability
Risk class: Class 2 reliability
FC nodes: FC2 restart, FC3 replay
Dependencies: TC-007d, TC-014B
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `src/runtime/tc_crash_matrix.rs`
- `src/runtime/mod.rs`
- `tests/tc_crash_matrix.rs`

Forbidden paths: kernel, sequencer, typed-tx schema.

Task:

Create a kill-after-every-committed-transition crash matrix for TC surfaces.

Tests first:

- `crash_matrix_restarts_from_git_cas_only`
- `snapshots_are_acceleration_only`
- `gateway_crash_states_recover_to_terminal_records`
- `scheduler_crash_preserves_even_lane_prefix`

Rules:

- restart cannot require RAM cache.
- replay cannot call network or LLM.
- snapshots may speed recovery but cannot be required for correctness.

Ship gate:

```bash
cargo test --test tc_crash_matrix --no-fail-fast
```

Expected: command exits 0.

Audit: Reliability Auditor.
Verdict: `NO-VIOLATION` or `RECONSTRUCTION-FAILURE <artifact>`.
