# TC-017 Autonomous Routing Legalization

Status: ready
Owner lane: lean-search
Risk class: Class 2 signal-boundary enforcement
FC nodes: FC1 scheduler/search spine, autonomous routing boundary
Dependencies: TC-016
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `src/runtime/g0_completeness.rs`
- `tests/tc_g0_completeness.rs`

Forbidden paths:

- verifier authority
- predicate authority
- rank authority outside G0
- kernel, bus, sequencer, typed-tx schema

Task:

Legalize autonomous routing only as odd-lane heuristic proposal input.
Autonomous routing cannot mutate enumerator queue, G0 rank, verifier result, or
accepted proof state.

Tests first:

- `autonomous_route_is_odd_lane_proposal_only`
- `autonomous_route_cannot_mutate_even_queue`
- `autonomous_route_cannot_override_verifier_rejection`

Rules:

- Autonomous route output is logged as odd-lane candidate metadata.
- Autonomous route output never writes even-lane queue.
- Autonomous route output never changes a final verifier result.
- Autonomous route output never changes G0 candidate digest or rank.

Ship gate:

```bash
cargo test --test tc_g0_completeness --no-fail-fast
```

Expected: command exits 0.

Audit: Constitution Auditor.
Verdict: `NO-VIOLATION` or `VIOLATION-FOUND autonomous-authority <file>:<line>`.
