# TISR Phase 6.3.x — Charter R2.1 Path Patch

**Revision**: R2.1 (post-W0 path resolution; orchestrator-only patch)
**Date**: 2026-05-18
**Base**: R2 §A12 case 2 stop-and-ratify trigger
**Architect authorization**: chat 2026-05-18 (autonomous-orchestrator mandate; pre-ratified §A12 fallback Option (i))

## §A12 case 2 trigger — confirmed

W0 inspection (commit 323a01cc) confirmed:
- `src/bin/turingos.rs` is a single-file bin (316 LOC) using `#[path = "turingos/<name>.rs"]` attributes for all submodules.
- `src/bin/turingos/mod.rs` does NOT exist.
- This is R2 §A12 case 2: the bin-local placement of `grill_envelope.rs` (in R1 §4) is impossible to import from `src/runtime/grill_predicates.rs` (R3's location) — different crates.

## Resolution chosen — Option (i)

Move `grill_envelope.rs` to the library crate at `src/runtime/grill_envelope.rs`. Rationale per W0 plan Task 2: `TurnPayload` is pure data; library crate is the natural home; `src/runtime/mod.rs` already hosts analogous pure-data modules (`prompt_capsule`, `evidence_capsule`, `attempt_telemetry`).

## R1 §4 Allowed Paths — diff

```
- src/bin/turingos/grill_envelope.rs                (NEW: ~+200 LOC)
+ src/runtime/grill_envelope.rs                      (NEW: ~+200 LOC)
```

`src/runtime/mod.rs` was already listed in R1 §4 for the `grill_predicates` `pub mod` line; the W2 atom adds a sibling `pub mod grill_envelope;` to the same file.

## Import-path canon (for W3 through W8)

- W3 (`src/runtime/grill_predicates.rs`) imports:
  ```rust
  use crate::grill_envelope::{TurnPayload, CANONICAL_SLOTS, REQUIRED_SLOTS};
  ```
- W4 (`src/bin/turingos/cmd_llm.rs`) imports:
  ```rust
  use turingosv4::runtime::grill_envelope::{parse_and_validate, TurnPayload};
  ```
- W6 (`src/bin/turingos/cmd_spec.rs`) imports:
  ```rust
  use turingosv4::runtime::grill_envelope::TurnPayload;
  use turingosv4::runtime::grill_predicates::{run_turn_predicates, termination_predicate, Lang, PredicateBundle};
  ```
- W7 (`src/web/spec.rs`) imports same as W6.

## TurnPayload field-name canon (resolved)

R1 §1 and R1 §9.W4 used `question_text` / `rationale_brief` in some places and `question` / `rationale` in others (drift from Researcher A §2.1 verbatim envelope vs charter prose).

**Canon for atom dispatch (W0 plan Task 4 alignment)**:
- Field: `question: Option<String>` (NOT `question_text`)
- Field: `rationale: String` (NOT `rationale_brief`)
- Field: `playback: Option<String>` (unchanged)

This matches Researcher A §2.1 OUTPUT CONTRACT verbatim — the LLM is told to emit exactly these field names. Charter §1 prose using `question_text` is treated as documentation drift; sub-atoms use the canonical names.

## Forbidden surfaces — unchanged

Zero change to R1 §3 Class-4 forbidden list. `src/runtime/` is Class-2 territory (library crate, no typed_tx / no ObjectType / no canonical signing payload touched).

## Effect on §9 atom dispatch

- W2 brief uses path `src/runtime/grill_envelope.rs`.
- W3 brief uses import `use crate::grill_envelope::...`.
- Atom dependency graph (R2 §A16) unchanged.

## End of R2.1
