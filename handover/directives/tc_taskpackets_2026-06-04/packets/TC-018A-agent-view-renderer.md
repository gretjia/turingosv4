# TC-018A TC Agent View Renderer

Status: ready
Owner lane: lean-search
Risk class: Class 2 shielding
FC nodes: FC1 rtool input shielding
Dependencies: TC-011B
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `src/runtime/tc_agent_view.rs`
- `src/runtime/mod.rs`
- `tests/tc_agent_view_shielding.rs`

Forbidden paths: kernel, bus, raw diagnostic stores.

Task:

Create a TC-specific allowlist renderer for agent prompt views.

Tests first:

- `tc_goal_view_hides_theorem_body_and_full_landscape`
- `tc_goal_view_exposes_only_state_id_goals_and_bounded_feedback`

Allowlist:

- state id
- theorem id only if public
- goal index
- case label
- public hypotheses summary
- public target summary
- bounded error class and summary

Blocked:

- hidden theorem body
- full theorem bank
- private diagnostic CID/body
- unshielded verifier output

Ship gate:

```bash
cargo test --test tc_agent_view_shielding --no-fail-fast
```

Expected: command exits 0.

Audit: Shielding Auditor.
Verdict: `NO-VIOLATION` or `VIOLATION-FOUND leakage <file>:<line>`.
