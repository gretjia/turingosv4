# A10 Projection Cache Preflight

Date: 2026-06-05

Parent plan:
`handover/directives/2026-06-05_AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT.md`

Atom: A10. Projection Cache With GitOid Watermark

Document role: Class 0 preflight. This document does not authorize economy
projection caching, sequencer admission, typed transaction, CAS schema, or
ChainTape writer changes by itself.

## Decision

A10 is preflight-only right now. The parent plan makes A10 a cache atom over
A09 economy projections, but A05 generic projection substrate, A09 economy
projection files, and all A10 files are absent.

A10 must wait for:

- A04/A05 to settle ChainTape/L4 head identity and projection input shape
- A09 to produce economy projections that are worth caching
- A09 acceptance proving the uncached replay output first

Safe work now:

- docs-only preflight
- cache-not-source-of-truth contract
- GitOid watermark contract
- negative-test inventory

Blocked until predecessors exist:

- implementing `src/runtime/projection.rs` as production cache substrate
- caching economy projections before A09 replay output exists
- accepting cache content as predicate/economy admission input
- hiding full replay behind an optional cache-only path
- touching ChainTape writer/OCC/single-writer authority

## Current-State Facts

Parent-plan A10 allowed paths:

```text
src/economy/projections.rs
src/runtime/projection.rs
tests/economy_projection_cache_watermark.rs
tests/projection_cache_not_source_of_truth.rs
```

Corrected implementation path inventory:

```text
src/runtime/mod.rs
src/runtime/projection.rs
src/economy/mod.rs
src/economy/projections.rs
src/bottom_white/ledger/transition_ledger.rs
src/runtime/chain_tape_lease.rs
src/runtime/resume_preflight.rs
src/runtime/verify.rs
src/bottom_white/cas/mod.rs
src/bottom_white/cas/store.rs
src/bottom_white/cas/git_chain.rs
tests/economy_projection_cache_watermark.rs
tests/projection_cache_not_source_of_truth.rs
tests/economy_tape_replay.rs
tests/economy_conservation.rs
tests/build_session_replay_after_cache_delete.rs
tests/tb_18r_cas_reload_split_brain.rs
```

Write-scope guidance:

```text
src/runtime/mod.rs
  needed only if src/runtime/projection.rs must be crate-visible
src/economy/mod.rs
  needed only if src/economy/projections.rs must be crate-visible
src/bottom_white/ledger/transition_ledger.rs
  read-only head-watermark source unless A04 explicitly authorizes change
src/runtime/chain_tape_lease.rs
src/runtime/resume_preflight.rs
src/runtime/verify.rs
  read-only head-continuity precedents
src/bottom_white/cas/*
  read-only cache-not-truth precedent; do not change CAS schema for A10
src/state/sequencer.rs
src/state/typed_tx.rs
src/bottom_white/cas/schema.rs
  out of A10 write scope without explicit higher-risk ratification
```

Existence check:

```text
MISSING src/economy/projections.rs
MISSING src/runtime/projection.rs
MISSING tests/economy_projection_cache_watermark.rs
MISSING tests/projection_cache_not_source_of_truth.rs
MISSING src/economy/events.rs
MISSING src/economy/conservation.rs
MISSING src/economy/settlement.rs
MISSING src/economy/price_broadcast.rs
MISSING tests/economy_tape_replay.rs
MISSING tests/economy_conservation.rs
EXISTS src/runtime/mod.rs
EXISTS src/economy/mod.rs
EXISTS src/bottom_white/ledger/transition_ledger.rs
EXISTS src/runtime/chain_tape_lease.rs
EXISTS src/runtime/resume_preflight.rs
EXISTS src/runtime/verify.rs
EXISTS src/bottom_white/cas/mod.rs
EXISTS src/bottom_white/cas/store.rs
EXISTS src/bottom_white/cas/git_chain.rs
EXISTS tests/build_session_replay_after_cache_delete.rs
EXISTS tests/tb_18r_cas_reload_split_brain.rs
```

Dirty-path check for relevant inventory:

```text
pre-existing dirty paths include:
  handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md
  src/economy/monetary_invariant.rs
  src/runtime/mod.rs
  src/state/sequencer.rs
  src/state/typed_tx.rs

Implementation must read and preserve those edits. Do not overwrite them as
part of A10 scaffolding.
```

Existing cache/replay/head witnesses:

```text
src/bottom_white/cas/mod.rs:11
  refs/chaintape/cas is authority; sidecar index is cache
src/bottom_white/cas/store.rs:450
  CAS commit-chain advances before sidecar cache or hot index accepts object
src/bottom_white/cas/store.rs:461
  sidecar cache failure removes stale cache after canonical chain advance
src/bottom_white/cas/store.rs:1162
  missing sidecar rebuilds from CAS commit-chain
src/bottom_white/cas/store.rs:1392
  tampered sidecar cache fails closed when CAS chain exists
src/bottom_white/cas/store.rs:1471
  failed CAS ref update does not write sidecar or hot index
src/runtime/replay.rs:65
  build-session replay reconstructs from CAS and does not call network
src/runtime/replay.rs:151
  replay verifies cross-CID references resolve in CAS
tests/build_session_replay_after_cache_delete.rs:103
  deleting CAS sidecar cache does not break replay
src/bottom_white/ledger/transition_ledger.rs:336
  LedgerWriter exposes head_commit_oid_hex
src/bottom_white/ledger/transition_ledger.rs:1097
  canonical L4 ref is refs/chaintape/l4
src/bottom_white/ledger/transition_ledger.rs:1102
  canonical CAS ref is refs/chaintape/cas
src/bottom_white/ledger/transition_ledger.rs:1186
  fresh checkout can read refs/chaintape/l4 directly
src/bottom_white/ledger/transition_ledger.rs:1308
  Git2LedgerWriter maps head OID to 40-char hex string
src/runtime/resume_preflight.rs:315
  resume preflight snapshots head hex, state root, and chain length
src/runtime/chain_tape_lease.rs:208
  lease checks expected head against live ChainTape head
src/runtime/verify.rs:170
  replay report exposes head_commit_oid_hex
```

Economy projection precedents, not A10 cache:

```text
src/state/price_index.rs:1
  PriceIndex is derived view; price is signal, not truth
src/state/price_index.rs:146
  compute_price_index is pure over EconomicState
src/state/q_state.rs:181
  price_index_t is absent from canonical EconomicState
src/state/q_state.rs:1091
  node_market_t is absent from EconomicState
src/state/q_state.rs:472
  task_markets_t.total_escrow is a derived cached index with invariant
src/economy/monetary_invariant.rs:53
  DerivedCacheMismatch is already a constitutional cache-drift error
src/economy/monetary_invariant.rs:1000
  drifted task_market total_escrow cache is rejected in tests
```

## Risk Classification

Risk floor: Class 3 if A10 caches economy projections, because stale or
tampered cache can corrupt money/market/settlement reads if consumed
incorrectly.

Class 2 is possible only for a non-economy generic projection-cache substrate
with test fixtures that cannot influence production economy, predicate, or
admission behavior.

Promote to Class 4 if:

- ChainTape writer/OCC/single-writer authority changes
- sequencer admission changes
- typed tx schema or discriminants change
- canonical signing payload changes
- CAS `ObjectType` or schema authority changes
- trust-root / constitution / flowchart authority changes

## Recommended Contract

Generic projection cache entry:

```text
ProjectionCacheKey {
  projection_id: String,
  projection_version: u32,
  derived_from_tape_head: GitOid,
}

ProjectionCacheEntry<T> {
  key: ProjectionCacheKey,
  last_applied_logical_t: u64,
  source_tape_kind: "chaintape_l4",
  value_cid: Cid,
  value_hash: Hash,
  created_by: String,
}
```

Cache semantics:

```text
cache hit only if derived_from_tape_head == current ChainTape/L4 head.
stale cache may be used only as a delta-apply starting point if ancestry from
cached head to current head is proven.
if ancestry cannot be proven, fall back to full replay.
tampered cache fails closed or is ignored.
dropping cache does not change replay result.
cache cannot be canonical input to predicates, settlement, sequencer admission,
typed tx construction, or ChainTape/CAS verification.
```

Economy adapter after A09:

```text
EconomyProjectionCache =
  ProjectionCacheEntry<EconomyProjection>

valid iff:
  EconomyProjection.derived_from_tape_head ==
  ProjectionCacheKey.derived_from_tape_head ==
  current ChainTape/L4 head
```

## Atomized A10 Tasks

### A10.0 Preflight Lock

Description:
Record missing A10 files, predecessor dependencies, cache-not-truth contract,
GitOid head availability, and acceptance commands.

Acceptance:

```bash
for f in \
  handover/directives/2026-06-05_A10_PROJECTION_CACHE_PREFLIGHT.md \
  handover/directives/2026-06-05_AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT.md
do
  git diff --no-index --check /dev/null "$f" || true
done
```

Expected:

```text
no whitespace errors
```

### A10.1 Generic Projection Cache Substrate

Description:
After A05 projection substrate exists, add generic projection cache data types
and helper functions using non-economy fixtures first.

Primary paths:

```text
src/runtime/mod.rs
src/runtime/projection.rs
tests/projection_cache_not_source_of_truth.rs
```

Guidance:

```text
Use explicit ProjectionCacheKey.
Require current ChainTape/L4 head to be supplied by caller.
Do not read global latest pointers.
Do not write ChainTape/CAS authority from the cache module.
Do not import sequencer or typed_tx admission code.
```

Acceptance:

```bash
cargo test --test projection_cache_not_source_of_truth --no-fail-fast -- --test-threads=1
git diff --check
```

Expected:

```text
cache deletion and cache tamper cannot change canonical replay output.
```

### A10.2 Economy Projection Cache Adapter

Description:
After A09 replay output exists, add the economy-specific adapter for
`EconomyProjection`.

Primary paths:

```text
src/economy/mod.rs
src/economy/projections.rs
tests/economy_projection_cache_watermark.rs
tests/economy_tape_replay.rs
tests/economy_conservation.rs
```

Guidance:

```text
Cache key = projection id + projection version + ChainTape/L4 GitOid.
Cache value must carry the same derived_from_tape_head.
Current head mismatch => stale cache path, not hit.
Stale cache may delta-apply only after ancestry proof.
Full replay remains mandatory fallback and test path.
```

Acceptance:

```bash
cargo test --test economy_projection_cache_watermark --no-fail-fast -- --test-threads=1
cargo test --test economy_tape_replay --no-fail-fast -- --test-threads=1
cargo test --test economy_conservation --no-fail-fast -- --test-threads=1
```

Expected:

```text
cache hit only when derived_from_tape_head == current head.
stale cache updates by replaying deltas or falls back to full replay.
dropping cache yields byte-equivalent economy projection.
```

### A10.3 Negative Authority Tests

Description:
Add tests proving projection cache cannot become source of truth for predicates,
settlement, admission, or reports.

Primary paths:

```text
tests/projection_cache_not_source_of_truth.rs
tests/economy_projection_cache_watermark.rs
tests/tb_18r_cas_reload_split_brain.rs
```

Guidance:

```text
Test stale head.
Test tampered value hash.
Test deleted cache.
Test wrong projection id/version.
Test attempt to use cache as predicate/economy admission input.
Use existing CAS sidecar fail-closed tests as precedent, not as authority for
economy cache behavior.
```

Acceptance:

```bash
cargo test --test projection_cache_not_source_of_truth --no-fail-fast -- --test-threads=1
cargo test --test economy_projection_cache_watermark --no-fail-fast -- --test-threads=1
cargo test --test tb_18r_cas_reload_split_brain --no-fail-fast -- --test-threads=1
```

Expected:

```text
tampered cache is ignored or fails closed.
cache cannot be accepted as canonical predicate or economy input.
```

## Final Pre-Implementation Gate

A10 implementation may start only when all are true:

- A04/A05 provide ChainTape-L4 head identity and projection substrate.
- A09 economy projection exists and has an uncached replay baseline.
- the first code change is a failing `projection_cache_not_source_of_truth` or
  `economy_projection_cache_watermark` test.
- `cargo test --test projection_cache_not_source_of_truth --no-fail-fast -- --test-threads=1`
  fails for the missing cache-not-truth guard before implementation.
- `cargo test --test economy_projection_cache_watermark --no-fail-fast -- --test-threads=1`
  fails for the missing GitOid watermark before implementation.
- no cache path can feed predicates, settlement, sequencer admission, typed tx,
  ChainTape/CAS verification, or reports as authority.

## Full A10 Acceptance

After A04/A05/A09 exist and A10 implementation is complete:

```bash
cargo test --test economy_projection_cache_watermark --no-fail-fast -- --test-threads=1
cargo test --test projection_cache_not_source_of_truth --no-fail-fast -- --test-threads=1
cargo test --test economy_tape_replay --no-fail-fast -- --test-threads=1
cargo test --test economy_conservation --no-fail-fast -- --test-threads=1
cargo test --test tb_18r_cas_reload_split_brain --no-fail-fast -- --test-threads=1
bash scripts/run_constitution_gates.sh
cargo test --test constitution_matrix_drift --no-fail-fast
git diff --check
grep -RInE 'f32|f64' src/economy tests/economy_* && exit 1 || true
```

Static changed-file guard for future implementation PRs:

```bash
git diff --name-only main...HEAD | \
  rg '^(src/state/sequencer.rs|src/state/typed_tx.rs|src/bottom_white/cas/schema.rs)$' && exit 1 || true
```

Expected:

```text
PREDICATES-GREEN
cache hit only when derived_from_tape_head == current head.
stale cache is delta-applied only with ancestry proof or repaired by full replay.
tampered cache fails closed or is ignored.
dropping cache does not change replay result.
cache is never accepted as canonical predicate/economy/admission input.
```

## Hard Blockers

```text
A10-IMPLEMENTABLE-AFTER-A09
```

Hard blockers:

- A05 generic projection substrate is missing.
- A09 economy projection source/test files are missing.
- All parent-plan A10 source/test files are missing.
- ChainTape head identity must come from A04/A05 authority, not a dashboard or
  global latest pointer.
- Existing CAS sidecar tests are useful precedent, but they are not an economy
  projection cache implementation.
- Dirty restricted/authority-adjacent paths must be preserved until their
  owning work is understood.

Clean-context audit input for a future implementation PR:

```text
Task brief: A10 Projection Cache With GitOid Watermark.
Risk class: Class 3 for economy projection cache; Class 4 if restricted
authority surfaces are touched.
FC nodes: Art. 0.2, FC1-N1, FC1-N13, FC1 dashboard-not-source invariant.
Evidence: A04/A05/A09 predecessor evidence, A10 tests, constitution gates,
static changed-file guard output.
Verdict domain: NO-VIOLATION | VIOLATION-FOUND | RECONSTRUCTION-FAILURE |
SECOND-SOURCE-DRIFT
```
