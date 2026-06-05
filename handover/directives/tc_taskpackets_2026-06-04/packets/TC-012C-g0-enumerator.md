# TC-012C Exact G0 Enumerator

Status: ready
Owner lane: lean-search
Risk class: Class 2 bounded completeness
FC nodes: FC1 search spine
Dependencies: TC-012B
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `src/runtime/g0_completeness.rs`
- `tests/tc_g0_completeness.rs`

Forbidden paths: LLM callers, Lean runtime calls, market authority.

Task:

Enumerator outputs exactly the bounded G0 corpus in rank-then-digest order.

Tests first:

- `g0_known_rank_corpus_count_is_exact`
- `g0_enumerator_order_is_rank_then_digest`
- `g0_enumerator_is_manifest_order_independent`

Rules:

- No Lean calls.
- No model calls.
- No market price input.
- No hidden automation.

Ship gate:

```bash
cargo test --test tc_g0_completeness --no-fail-fast
```

Expected: command exits 0.

Audit: Formal-methods Auditor.
Verdict: `NO-VIOLATION` or `RECONSTRUCTION-FAILURE g0-corpus`.
