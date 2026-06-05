# TC-014A Queue Duplicate Pointer

Status: ready
Owner lane: lean-search
Risk class: Class 2 queue isolation
FC nodes: FC1 scheduler/search spine
Dependencies: TC-013A
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `src/runtime/g0_completeness.rs`
- `tests/tc_g0_completeness.rs`

Forbidden paths: kernel, market authority, verifier authority.

Task:

Add duplicate trace semantics without letting odd observations cover even
enumerator obligations.

Tests first:

- `odd_duplicate_digest_does_not_mask_even_candidate`
- `even_duplicate_records_first_even_trace_pointer`

Rules:

- duplicate suppression is digest-only.
- odd duplicate never marks even as covered.
- even duplicate may point to first even attempt.

Ship gate:

```bash
cargo test --test tc_g0_completeness --no-fail-fast
```

Expected: command exits 0.

Audit: Formal-methods Auditor.
Verdict: `NO-VIOLATION` or `VIOLATION-FOUND queue-isolation <file>:<line>`.
