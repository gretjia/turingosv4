# TC-108 Lean Error Fact

Status: ready
Owner lane: lean-search
Risk class: Class 2 shielding
FC nodes: FC1 verifier feedback, FC3 replay
Dependencies: TC-101
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `src/runtime/tc_tape_canonical.rs`
- `src/judges/lean_micro_state.rs`
- `tests/tc_tape_canonical_repairs.rs`
- `tests/tc_lean_micro_state_contract.rs`

Forbidden paths: kernel, bus, unshielded verifier output prompt paths.

Task:

Represent Lean feature-layer error facts as bounded public summaries with tape
anchor. Lean remains outside TuringOS kernel.

Test first:

`lean_error_fact_shields_verifier_output_and_links_attempt`.

Assertions:

- verifier-output marker is rejected.
- fact links theorem id, attempt id, and anchor.
- public summary is bounded and classified.

Ship gate:

```bash
cargo test --test tc_tape_canonical_repairs --test tc_lean_micro_state_contract --no-fail-fast
(git diff --name-only origin/main...HEAD; git ls-files -o --exclude-standard) | sort -u | grep -E '^(src/runtime|src/judges|tests)/' | xargs grep -nE 'raw.*std[e]rr|Lean.*std[e]rr'
```

Expected: cargo tests exit 0; grep has no output.

Audit: Shielding Auditor.
Verdict: `NO-VIOLATION` or `VIOLATION-FOUND verifier-output-leak <file>:<line>`.
