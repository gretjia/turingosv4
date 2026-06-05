# TC-013A Strict Dovetail Scheduler

Status: ready
Owner lane: lean-search
Risk class: Class 2 scheduler invariant
FC nodes: FC1 scheduler/search spine
Dependencies: TC-012C
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `src/runtime/g0_completeness.rs`
- `tests/tc_g0_completeness.rs`

Forbidden paths: kernel, bus, market authority.

Task:

Implement strict 1:1 dovetail trace semantics.

Tests first:

- `scheduler_emits_one_trace_per_tick`
- `witness_index_i_attempted_by_even_tick_2i`
- `odd_tick_never_attempts_even_candidate`

Locked tick semantics:

- tick 0 is even enumerator.
- tick 1 is odd heuristic.
- no lane backfill.
- no lane stealing.
- exhausted odd lane still emits odd no-candidate trace.

Ship gate:

```bash
cargo test --test tc_g0_completeness --no-fail-fast
```

Expected: command exits 0.

Audit: Formal-methods Auditor.
Verdict: `NO-VIOLATION` or `VIOLATION-FOUND starvation <file>:<line>`.
