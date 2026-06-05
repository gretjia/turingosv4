# TC-103 Map-Reduce Tick Fact

Status: ready
Owner lane: gateway
Risk class: Class 1 additive helper
FC nodes: FC2 map-reduce tick
Dependencies: TC-101
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `src/runtime/tc_tape_canonical.rs`
- `tests/tc_tape_canonical_repairs.rs`

Forbidden paths: typed-tx schema, sequencer, signing payloads.

Task:

Represent map-reduce tick provenance as a tape-canonical fact linked to an
existing accepted transition. Do not create a new transaction kind.

Test first:

`map_reduce_tick_fact_links_existing_l4_tx`.

Assertions:

- fact kind is `map_reduce_tick`.
- anchor includes accepted L4 head.
- stdout-only tick is rejected by constructor.

Ship gate:

```bash
cargo test --test tc_tape_canonical_repairs --no-fail-fast
```

Expected: command exits 0.

Audit: Data-integrity Auditor.
Verdict: `NO-VIOLATION` or `RECONSTRUCTION-FAILURE <artifact>`.
