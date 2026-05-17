# NON_CLAIMS.md — UI IR Spike Shielding Boundaries

This file explicitly states what the UI IR spike does NOT do.
Shielding boundaries are load-bearing; do not remove without architect review.

## What this spike is

A local, fixture-based intermediate representation (IR) renderer for
TuringOS UI views. Class 1 (additive, isolated, local).

---

## Hard non-claims

### 1. Does NOT serve HTML

No web server. No HTML output. No HTTP endpoints. No WebSocket connections.
No React, no JavaScript, no frontend framework of any kind.

Phase 7 (Web MVP) is the appropriate milestone for HTML/browser serving.
This spike feeds Phase 7 design only.

### 2. Does NOT use ChainTape/CAS as authoritative source

All data in `fixtures/` is fixture-based simulation of what
`audit_dashboard`, `turingos agent list`, and `turingos task view` would
emit. Fixtures are NOT live reads from ChainTape or CAS.

Real ChainTape/CAS wiring is Phase 7+ work.

### 3. Reports are materialized views — never source of truth (FC3-N31)

Quoting TuringOS CLAUDE.md §5.4 and constitution FC3 invariant N31:

> "A read-only materialized view. It must be deletable and regeneratable
> from ChainTape + CAS. Never treat dashboard as source of truth."

Rendered output from this spike is a materialized view.
Deleting and regenerating rendered output changes nothing in the canonical
system state.

### 4. Does NOT participate in sequencer admission

No typed transaction schema change. No new `TypedTx` variant. No sequencer
predicate. No sequencer admission arm.

### 5. Does NOT enter predicate truth

Rendered UI IR output is not fed back into any predicate, oracle, or
constitution gate. No LLM decision is made based on rendered text.

### 6. Does NOT touch Trust Root surfaces

The following files are NOT modified by this spike:

- `Cargo.toml`
- `Cargo.lock`
- `src/lib.rs`
- `src/kernel.rs`
- `src/bus.rs`
- `src/state/sequencer.rs`
- `src/state/typed_tx.rs`
- `src/bottom_white/cas/schema.rs`

No Rust source files exist in `experiments/tisr_ui_spike/`.
No workspace `Cargo.toml` change is required.

### 7. Does NOT claim benchmark validity

Fixture data uses realistic-but-fake values. No fixture represents a real
completed benchmark run. Attempt equality values (`PASS`) in fixtures are
illustrative, not evidence.

### 8. Does NOT constitute constitutional evidence

This spike cannot be cited as evidence for constitution gate passage.
Constitution gates are in `tests/constitution_*.rs` and run via
`bash scripts/run_constitution_gates.sh`.

---

## Risk classification

**Class 1** — additive, isolated, local, fixture-based renderer.

No Class 3 or Class 4 surface touched.
No architect §8 ratification required for this spike.
No Codex audit required for this spike.

---

FC-trace: **FC3-N31** — UI IR as materialized view, never authority.
