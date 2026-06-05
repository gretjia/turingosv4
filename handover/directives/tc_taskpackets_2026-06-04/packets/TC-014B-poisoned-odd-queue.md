# TC-014B Poisoned Odd Queue Adversary

Status: ready
Owner lane: lean-search
Risk class: Class 2 queue isolation
FC nodes: FC1 scheduler/search spine
Dependencies: TC-014A
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `src/runtime/g0_completeness.rs`
- `tests/tc_g0_completeness.rs`

Forbidden paths: kernel, bus, verifier authority.

Task:

Prove adversarial odd queue cannot skip, pop, reorder, or mask even candidates.

Tests first:

- `poisoned_high_price_odd_queue_cannot_skip_pop_reorder_or_mask_even`
- `odd_queue_exhaustion_cannot_change_even_schedule`

Rules:

- use separate queues.
- no shared heap.
- no fallback from odd lane into even lane or even lane into odd lane.

Ship gate:

```bash
cargo test --test tc_g0_completeness --no-fail-fast
```

Expected: command exits 0.

Audit: Formal-methods Auditor.
Verdict: `NO-VIOLATION` or `VIOLATION-FOUND queue-isolation <file>:<line>`.
