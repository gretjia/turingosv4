# A05 Tape Event Envelope And Projection Preflight

Date: 2026-06-05

Parent plan:
`handover/directives/2026-06-05_AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT.md`

Atom: A05. Tape Event Envelope and Projection Trait

Document role: Class 0 preflight. This document does not authorize new runtime
event authority, projection authority, economy authority, or constitution-gate
edits by itself.

## Decision

A05 should not start as production implementation until A04 chooses the OS L2
ChainTape authority. A05 defines the generic event envelope and projection
trait that later atoms use for economy, scheduler, and external calls. If it is
implemented against the wrong physical tape, it becomes a second source of
truth.

Safe work now:

- docs-only preflight
- interface-contract correction
- test-shape design

Blocked until A04 authority is ratified:

- claiming projection replay from `src/git_tape_ledger.rs`
- adding generic claim gates to `scripts/constitution_gates.manifest.toml`
- using market-specific structures as the generic event envelope
- treating reports, manifests, dashboards, or stdout as proof of replay

## Hard Blockers

- A05-HB1: Production A05 is blocked until A04 selects the ChainTape-L4
  physical authority or a ratified substitute.
- A05-HB2: A05 cannot use `market_tape_shared`, benchmark manifests,
  dashboards, reports, stdout, or TDMA-only `GitTapeLedger` as projection truth.
- A05-HB3: CAS schema, typed transaction, or predicate/economy authority edits
  must be reclassified before implementation.

## Current-State Facts

Parent-plan A05 allowed paths:

```text
src/runtime/tape_event.rs
src/runtime/projection.rs
src/runtime/mod.rs
tests/tape_event_envelope_roundtrip.rs
tests/tape_projection_replay.rs
tests/constitution_headline_recompute_from_chaintape.rs
tests/constitution_router_name_matches_mechanism.rs
scripts/constitution_gates.manifest.toml
handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md
```

Existence check:

```text
MISSING src/runtime/tape_event.rs
MISSING src/runtime/projection.rs
EXISTS src/runtime/mod.rs
MISSING tests/tape_event_envelope_roundtrip.rs
MISSING tests/tape_projection_replay.rs
MISSING tests/constitution_headline_recompute_from_chaintape.rs
MISSING tests/constitution_router_name_matches_mechanism.rs
EXISTS scripts/constitution_gates.manifest.toml
EXISTS handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md
```

Dirty-path check for the corrected A05 allowed paths:

```text
pre-existing dirty paths include:
  src/runtime/mod.rs
  handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md

Implementation must read and preserve those edits. Do not overwrite them as
part of A05 scaffolding.
```

Existing nearby tape-canonical witnesses:

```text
tests/constitution_tape_canonical_gate.rs:1
  tape canonical constitution gate
tests/constitution_tape_canonical_gate.rs:89
  dashboard regenerates from ChainTape + CAS
tests/constitution_tape_canonical_gate.rs:120
  chain_derived_facts must derive from L4 + CAS, not evaluator stdout
tests/constitution_tape_canonical_gate.rs:146
  all externalized attempts have CAS payload
tests/constitution_no_parallel_ledger.rs:1
  no parallel ledger source-of-truth
tests/offline_replay_no_llm_dependency_static_check.rs:1
  offline replay modules must not import LLM/network clients
src/runtime/chain_derived_run_facts.rs:1
  ChainDerivedRunFacts derives bit-exact facts from L4/L4.E/CAS
src/runtime/market_e2_candidate_verifier.rs:1
  existing projection-like verifier reconstructs from ChainTape/CAS, not
  dashboards
```

Existing router/mechanism-adjacent witnesses:

```text
fc_alignment_conformance.rs contains argmax / Boltzmann mask policy witnesses.
constitution_real11_* and market_autonomy_lab files contain historical
router/action reports, but these are not generic projection contracts.
```

Implementation path correction:

```text
src/runtime/mod.rs is required if the new runtime modules are intended to be
crate-visible.

handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md is required when A05 adds
new constitution tests/gates, otherwise constitution_matrix_drift can reject
the change even when the test code itself compiles.
```

Forbidden source-of-truth drift:

```text
market_tape_shared
manifest-only proof
dashboard-only proof
stdout-derived headline
router label that says softmax while implementation collapses to argmax
```

## Risk Classification

Risk floor: Class 2. A05 creates generic runtime interfaces and tests.

Promote to Class 3 if:

- projections become economy or market settlement input
- projections become CAS integrity input
- projection caches are introduced
- source-of-truth claims are added to constitution gates

Promote to Class 4 if:

- typed transaction schema changes
- canonical signing payload changes
- sequencer admission changes
- trust-root / constitution / flowchart authority changes

## Recommended Contract

The first implementation contract should be data-shape only and should target
the ratified ChainTape-L4 authority from A04.

Minimum generic envelope:

```text
TapeEvent {
  logical_t: u64,
  tape_head_oid: String,
  kind: TapeEventKind,
  payload_cid: Option<Cid>,
  source_tx_kind: Option<TxKind>,
}
```

Minimum projection trait:

```text
Projection {
  type Output;
  fn derive_from_tape(events: &[TapeEvent]) -> Result<Self::Output, ProjectionError>;
}
```

Do not include market, wallet, scheduler, or external-call policy fields in
the generic envelope. Those are later projections over the envelope.

## Atomized A05 Tasks

### A05.0 Preflight Lock

Description:
Record the missing files, predecessor dependency, and generic-contract boundary.

Acceptance:

```bash
for f in \
  handover/directives/2026-06-05_A05_TAPE_EVENT_PROJECTION_PREFLIGHT.md \
  handover/directives/2026-06-05_AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT.md
do
  git diff --no-index --check /dev/null "$f" || true
done
```

Expected:

```text
no whitespace errors.
A05 preflight states A04 ChainTape-L4 authority is a predecessor.
```

### A05.1 Envelope Roundtrip

Description:
After A04 ratifies ChainTape-L4, add the envelope type and roundtrip tests.

Acceptance:

```bash
cargo test --test tape_event_envelope_roundtrip --no-fail-fast -- --test-threads=1
git diff --check
```

Expected:

```text
serde/canonical roundtrip is stable.
event kind is generic and not market-specific.
payload CIDs remain opaque.
```

### A05.2 Projection Replay

Description:
Add projection trait tests that derive only from ChainTape events.

Acceptance:

```bash
cargo test --test tape_projection_replay --no-fail-fast -- --test-threads=1
cargo test --test constitution_headline_recompute_from_chaintape --no-fail-fast -- --test-threads=1
```

Expected:

```text
projection output is recomputable from ChainTape/Git OIDs and CAS CIDs.
manifest-only proof fails.
stdout-derived headline fails.
```

### A05.3 Claim Gates

Description:
Only after the projection trait exists, add generic claim gates to the
constitution gate manifest.

Acceptance:

```bash
cargo test --test constitution_router_name_matches_mechanism --no-fail-fast -- --test-threads=1
bash scripts/run_constitution_gates.sh
cargo test --test constitution_matrix_drift --no-fail-fast
grep -RInE '(^|[^A-Za-z])market_tape_shared(::|[^A-Za-z])' src tests scripts && exit 1 || true
git diff --check
```

Expected:

```text
lying manifest positive control fails.
softmax name over argmax implementation fails.
no market_tape_shared dependency.
```

## Final Pre-Implementation Gate

A05 implementation may start only when all are true:

- A04 has selected ChainTape-L4 as the OS L2 authority, or this atom is
  explicitly downgraded to a docs/test-only contract
- no implementation PR claims projection completion from TDMA `GitTapeLedger`
- first code change is an envelope/projection failing test
- `src/runtime/mod.rs` and matrix drift impact are handled explicitly
- no economy/scheduler/external-call semantics are smuggled into the generic
  envelope

Clean-context audit input for a future implementation PR:

```text
Task brief: A05 Tape Event Envelope and Projection Trait.
Risk class: Class 2; promote to Class 3 if economy or CAS integrity consumes
the projection.
FC nodes: Art. 0.2, FC1-N1, FC1-N13, FC1-N14, FC1-N15.
Evidence: A04 predecessor evidence, A05 tests, constitution gates,
constitution_matrix_drift output, no-market_tape_shared grep.
Verdict domain: NO-VIOLATION | VIOLATION-FOUND | RECONSTRUCTION-FAILURE |
SECOND-SOURCE-DRIFT
```
