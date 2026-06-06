# A10 Projection Cache Clean-Context Audit

Date: 2026-06-06

Reviewer: Claude CLI, clean context, no session persistence, no tools.

Task: A10 Projection Cache With GitOid Watermark

Risk class: Class 3

FC nodes / invariants: Art.0.2, FC1-N1, FC1-N13, FC1 dashboard-not-source invariant

Verdict: `NO-VIOLATION`

## Scope Reviewed

- `src/runtime/projection.rs`
- `src/economy/projections.rs`
- `tests/economy_projection_cache_watermark.rs`
- `tests/projection_cache_not_source_of_truth.rs`

## Evidence Provided To Reviewer

- `cargo test --test economy_projection_cache_watermark --no-fail-fast -- --test-threads=1`: 2 passed, 0 failed
- `cargo test --test projection_cache_not_source_of_truth --no-fail-fast -- --test-threads=1`: 2 passed, 0 failed
- `cargo test --test economy_tape_replay --no-fail-fast -- --test-threads=1`: 2 passed, 0 failed
- `cargo test --test economy_conservation --no-fail-fast -- --test-threads=1`: 1 passed, 0 failed
- `cargo test --test tb_18r_cas_reload_split_brain --no-fail-fast -- --test-threads=1`: 7 passed, 0 failed
- `bash scripts/run_constitution_gates.sh`: `[k-1-5] total=167 failed=0`
- `cargo test --test constitution_matrix_drift --no-fail-fast`: 3 passed, 0 failed
- `git diff --check`: exit 0
- `grep -RInE 'f32|f64' src/economy tests/economy_* && exit 1 || true`: exit 0, no matches
- `cargo test --workspace --no-fail-fast`: exit 0

## Reviewer JSON

```json
{
  "verdict": "NO-VIOLATION",
  "findings": [],
  "checked": [
    "cache key = projection_id+version+ChainTape GitOid",
    "head_oid GitOid format-validated",
    "cache hit legal only when derived_from_tape_head==current head",
    "tampered current-head entry forces full replay + overwrite",
    "stale cache delta-applied only with GitOid ancestry proof",
    "delta result equals full replay",
    "full replay always available and float-free",
    "derive_economy_projection_with_cache has zero production callers",
    "ProjectionCache absent from sequencer/typed_tx/predicate_receipt/settlement/CAS",
    "no f32/f64 in src/economy or src/runtime/projection.rs"
  ],
  "line_refs": [
    "src/economy/projections.rs:140-203",
    "src/economy/projections.rs:218-239",
    "src/runtime/projection.rs:29-101",
    "src/runtime/tape_event.rs:21-44",
    "tests/projection_cache_not_source_of_truth.rs:96-114"
  ]
}
```

## Reviewer Note

The reviewer noted a watermark-matching but content-poisoned cache entry could
return on cache hit, but classified it as non-constitutional because this cache
has no canonical consumers and full replay remains the source of truth.
