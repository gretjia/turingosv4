# TC-011C Lean Final Recert Bridge

Status: ready
Owner lane: lean-search
Risk class: Class 2 verifier bridge
FC nodes: FC1 JudgeAI predicate, FC3 replay
Dependencies: TC-011B
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `src/judges/lean_micro_state.rs`
- `src/judges/lean_judge.rs`
- `tests/tc_lean_micro_state_contract.rs`

Forbidden paths: kernel, bus, sequencer, typed-tx schema.

Task:

Bridge assembled proof candidates to final `LeanJudge` recertification and
axiom report. Micro-state cannot accept proof.

Tests first:

- `complete_outcome_requires_final_lean_judge_verified`
- `proof_artifact_requires_checked_axiom_report`
- `unclean_axiom_report_fails_closed`

Rules:

- Only final judge acceptance plus checked axiom report can create accepted
  proof artifact.
- `clean=true` requires `checked_by_print_axioms=true`.
- Lean remains a workload verifier, not a TuringOS kernel dependency.

Ship gate:

```bash
cargo test --test tc_lean_micro_state_contract --no-fail-fast
```

Expected: command exits 0.

Audit: Formal-methods Auditor.
Verdict: `NO-VIOLATION` or `VIOLATION-FOUND lean-authority <file>:<line>`.
