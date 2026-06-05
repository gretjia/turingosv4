# TC-018B Prompt Guard

Status: ready
Owner lane: lean-search
Risk class: Class 2 shielding
FC nodes: FC1 prompt boundary
Dependencies: TC-018A
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `src/runtime/tc_agent_view.rs`
- `tests/tc_agent_view_shielding.rs`

Forbidden paths: kernel, bus, raw prompt stores.

Task:

Fail closed before prompt crosses LLM boundary if forbidden TC content appears.

Tests first:

- `tc_prompt_guard_blocks_unshielded_verifier_output_hidden_body_private_diagnostic`
- `tc_prompt_guard_allows_bounded_error_class`

Blocked sentinels:

- `verifier-output`
- `Lean verifier output`
- `hidden theorem body`
- `private diagnostic`
- full theorem-bank marker

Ship gate:

```bash
cargo test --test tc_agent_view_shielding --no-fail-fast
(git diff --name-only origin/main...HEAD; git ls-files -o --exclude-standard) | sort -u | grep -E '^(src/runtime|tests)/' | xargs grep -nE 'raw.*std[e]rr|Lean.*std[e]rr'
```

Expected: cargo test exits 0; grep has no output.

Audit: Shielding Auditor.
Verdict: `NO-VIOLATION` or `VIOLATION-FOUND leakage <file>:<line>`.
