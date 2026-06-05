# TC-011B Lean Step Adapter Fixtures

Status: ready
Owner lane: lean-search
Risk class: Class 2 feature-layer adapter
FC nodes: FC1 verifier feedback
Dependencies: TC-011A
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `src/judges/lean_micro_state.rs`
- `tests/tc_lean_micro_state_contract.rs`
- `src/judges/lean_judge.rs` only for reuse, not authority changes

Forbidden paths: kernel, bus, sequencer, typed-tx schema.

Task:

Add a feature-layer step adapter fixture path. `Complete` means assembled proof
candidate, not final truth.

Tests first:

- `lean_step_intro_advances`
- `lean_step_simp_completes`
- `lean_step_backtracks_from_parent_state`
- `lean_step_feedback_is_bounded_not_verifier_output`

Rules:

- Parent states are immutable.
- Feedback is bounded public text.
- Unshielded Lean output never enters prompt-facing view.
- No step outcome accepts a proof.

Ship gate:

```bash
cargo test --test tc_lean_micro_state_contract --no-fail-fast
```

Expected: command exits 0.

Audit: Formal-methods Auditor.
Verdict: `NO-VIOLATION` or `VIOLATION-FOUND verifier-output-leak <file>:<line>`.
