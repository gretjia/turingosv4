# ADR 2026-06-05: Path B GitTape As Agentic OS Substrate

Status: accepted for execution planning
Date: 2026-06-05
Risk of this ADR: Class 0
Future implementation risk: Class 2 to Class 4 by touched surface
Parent plan: `handover/directives/2026-06-05_AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT.md`

## Context

The constitution defines `Q_t = <q_t, HEAD_t, tape_t>` as version-controlled
state. The repo's truth order treats ChainTape, CAS, and deterministic replay
as Tier 2 facts. Derived reports, dashboards, alignment matrices, and latest
pointers are not sources of truth.

Audit snapshots #280 and #283 contain useful quarry material, but neither may
be merged as a production branch. The next production path must be small atoms
cut from `origin/main`.

## Decision

TuringOS adopts Path B:

```text
GitTape/ChainTape = sole source of truth for Q_t
Vec<Node> legacy = compatibility layer only
MarketTape = forbidden as a parallel ledger
Wallet/market/price/search/librarian/dashboard/report = derived projections
Phase E and later OS substrate work cannot proceed without Path B gates
```

Every state-bearing runtime component must answer:

```text
canonical_record = which GitTape/ChainTape event or CAS object
replay_recipe = how to derive it from genesis + tape + CAS
truth_tier = Tier 2 fact or derived view
```

## Layer Boundaries

```text
L0 Constitution / Human Sudo
L1 Boot Trust Root / Manifest
L2 GitTape / ChainTape World State
L3 External Call Outbox / Side-effect Gateway
L4 Agent Process Model / Agent View Shielding
L5 Predicate / Verifier Framework
L6 Economy Service
L7 Scheduler / Search / Allocation Policies
L8 Workload Adapters
L9 Evidence / Benchmark / Reports
```

Allowed dependency direction:

```text
L1 verifies L0 and initializes L2.
L2 records events and CAS references.
L3-L8 append typed events through L2 or derive views from L2.
L9 reports derive from L2/CAS and never feed canonical state.
```

Forbidden dependency direction:

```text
Reports/dashboards/latest pointers -> runtime authority
Economy projections -> predicate verdict authority
Workload adapter result -> kernel authority
Cache/session/UI state -> source of truth
Benchmark evidence -> canonical state
```

## Projection Rule

Projection caches are allowed only as optimizations:

```text
cache_key = projection_id + projection_version + derived_from_tape_head
valid_read = derived_from_tape_head == current GitTape HEAD
stale_read = apply deltas from cached head to current head
repair = full replay from genesis
```

Dropping a cache must not change replay output.

## Git Physical Integrity

GitTape implementation atoms must use a single writer or optimistic concurrency
control with expected-old `GitOid` checks. Ref movement errors are not warnings;
they are failed writes or explicit retries.

Generated GitTape repositories must pass:

```bash
git -C <generated_repo> fsck --full
```

## Consequences

Positive:

```text
One physical ledger.
Deterministic replay.
Market economy preserved as OS service.
Benchmark and workload claims stay user-space.
```

Costs:

```text
Projection caches need GitOid watermarks.
External calls need Intent -> Terminal closure.
Parallel agents need writer discipline or OCC.
Old memory-ledger helpers must be treated as compatibility-only.
```

## Acceptance Checks

```bash
grep -RIn 'Path B' handover/architecture/ADR_2026-06-05_PATH_B_GITTAPE_AS_OS_SUBSTRATE.md
grep -RIn 'GitTape/ChainTape = sole source of truth' handover/architecture/ADR_2026-06-05_PATH_B_GITTAPE_AS_OS_SUBSTRATE.md
grep -RIn 'MarketTape = forbidden as a parallel ledger' handover/architecture/ADR_2026-06-05_PATH_B_GITTAPE_AS_OS_SUBSTRATE.md
git diff --check
```
