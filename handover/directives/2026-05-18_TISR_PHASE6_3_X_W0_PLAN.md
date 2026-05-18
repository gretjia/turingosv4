# TISR Phase 6.3.x W0 Plan — Pre-flight Inspection

**Atom**: W0 (worktree + plan)
**Date**: 2026-05-18
**Risk class**: Class 0 (docs only; no source modification)

---

## Task 1: Branch State Verification

```
git branch --show-current  → codex/tisr-phase6-3-x-grill-driven  ✓
git log --oneline -5:
  c6176a5d TISR Phase 6.3.x charter R1+R2 + 2 audit verdicts (pre-W0)
  f0755dbc TISR Phase 6.3.x research: 3 parallel Opus xhigh design docs for LLM-driven grill
  886f7596 REAL-17 market emergence hardening on CAS main (#8)
  8c1032c0 Docs: update README after PR #6 Phase 7 Web MVP ship (#7)
  eab583fd TISR Phase 7 Web MVP: spec/generate/play CLI-wrapped web stack, 4-round real-LLM closure (#6)
git status --short: only .DS_Store + experiments/minif2f_v4/target/ + handover/.DS_Store (untracked)
```

HEAD confirmed as `c6176a5d`. Branch correct. No uncommitted source-file changes.

---

## Task 2: R2 §A12 Module-Layout Pre-flight Inspection

**File evidence**:

- `src/bin/turingos.rs` EXISTS (316 LOC). It is a **single-file bin** — there is NO `src/bin/turingos/mod.rs`. The bin declares each submodule using explicit `#[path = "turingos/<name>.rs"]` attributes (lines 22-80). Current bin-local modules: `cmd_spec`, `cmd_llm`, `spec_capsule`, `siliconflow_client`, `common`, plus ~18 other `cmd_*.rs` files.

- `src/bin/turingos/mod.rs` does **NOT** exist. The bin crate root is `src/bin/turingos.rs`.

- `src/runtime/mod.rs` declares all runtime library modules as `pub mod <name>;` (verified: `attempt_telemetry`, `prompt_capsule`, `evidence_capsule`, etc.).

**Decision — grill_envelope.rs location**:

R1 §4 Allowed Paths places `grill_envelope.rs` at `src/bin/turingos/grill_envelope.rs` (bin-local). R1 §4 also places `grill_predicates.rs` at `src/runtime/grill_predicates.rs` (library crate). These are in **different crates**: the binary crate cannot be imported by the library crate.

**Resolution chosen: option (i) — move `grill_envelope.rs` to `src/runtime/grill_envelope.rs`** (library crate).

Rationale: (a) `TurnPayload` is a pure data type with no binary-crate dependencies — it belongs in the library; (b) `src/runtime/mod.rs` already hosts analogous pure-data modules (`prompt_capsule`, `evidence_capsule`, `attempt_telemetry`); (c) the binary crate can import library types via `turingosv4::runtime::grill_envelope::TurnPayload`; (d) option (ii) would require `grill_predicates.rs` as a bin-local module, making it invisible to the library-level test harness and breaking `tests/grill_predicates_*.rs`; (e) option (iii) is explicitly not recommended. The Allowed Paths §4 list must be amended: replace `src/bin/turingos/grill_envelope.rs` with `src/runtime/grill_envelope.rs`; add `src/runtime/mod.rs` entry for `pub mod grill_envelope` (already listed in R1 §4 for `grill_predicates` — same line).

**Correct import path for W3** (`src/runtime/grill_predicates.rs` importing from W2):
```rust
use crate::grill_envelope::{TurnPayload, CANONICAL_SLOTS, REQUIRED_SLOTS};
// (crate here = the library crate; grill_envelope is pub mod in src/runtime/mod.rs)
```

W6 (`src/bin/turingos/cmd_spec.rs`) imports via the crate dependency:
```rust
use turingosv4::runtime::grill_envelope::TurnPayload;
use turingosv4::runtime::grill_predicates::{run_turn_predicates, termination_predicate, Lang};
```

W2's registration in `src/bin/turingos.rs`: NOT needed (library type; no `#[path]` mod declaration required in bin root).

---

## Task 3: W6 `cmd_spec.rs` Edit Ranges

File: `src/bin/turingos/cmd_spec.rs` (581 LOC).

| Function | Line range | W6 instruction |
|---|---|---|
| `canonical_questions(lang)` | 293–344 | **MUST NOT modify** |
| `system_prompt(lang)` | 346–395 | **MUST NOT modify** |
| `build_synthesis_user_message(...)` | 397–410 | **MUST NOT modify** |
| Arg-parsing loop (`while let Some(a) = iter.next()`) | 157–179 | W6 adds `"--mode"` arm here + `"--meta-prompt"` arm (R2 A7); existing `--skip-llm` stays |
| Static-mode loop body | 190–286 (after arg-parse validation; the `let questions = ...` → CAS write block) | W6 branches here: `if mode == Driven { run_driven_mode(...) } else { <existing static body> }` |
| Insert `fn run_driven_mode(...)` | After line 581 (end of file) | ~250 LOC driven-mode function (R1 §4 budget) |

W6 also adds `#[path = "turingos/grill_envelope.rs"]` registration — NOT needed because `grill_envelope` moves to library crate (Task 2 decision). W6 registration needed for nothing new in `src/bin/turingos.rs` module registry.

LOC estimate for W6 additions: ~250 LOC source (`run_driven_mode` function body + `--mode`/`--meta-prompt` flag arm + `Mode` enum + triage shell-out per R2 A5) + ~250 LOC tests per R1 §4.

---

## Task 4: W2 + W3 Public Fn Signatures (final, post-R2)

### W2 — `src/runtime/grill_envelope.rs`

```rust
/// Canonical 8 interview slot identifiers (vocabulary).
pub const CANONICAL_SLOTS: &[&str] = &[
    "job", "anchor", "memory", "first_run",
    "robustness", "scope", "acceptance", "mirror",
];

/// Required slot subset (7 of 8; excludes "mirror").
/// Termination predicate checks covered_slots ⊇ REQUIRED_SLOTS.
pub const REQUIRED_SLOTS: &[&str] = &[
    "job", "anchor", "memory", "first_run",
    "robustness", "scope", "acceptance",
];

/// Parsed, validated turn payload from one LLM response.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TurnPayload {
    pub turn: u32,
    pub question: Option<String>,
    pub covered_slots: Vec<String>,
    pub open_slots: Vec<String>,
    pub confidence: f64,
    pub done: bool,
    pub rationale: String,
    /// Present only when done=true.
    pub playback: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EnvelopeParseError {
    /// Input is not valid JSON.
    InvalidJson(String),
    /// Required field missing from parsed object.
    MissingField(&'static str),
    /// Field has wrong type.
    WrongType { field: &'static str, expected: &'static str },
    /// `turn` value is 0 or > 15.
    TurnOutOfRange(u32),
    /// `confidence` value outside [0.0, 1.0].
    ConfidenceOutOfRange(f64),
}

/// Parse raw LLM response string into TurnPayload. Does not validate slot
/// vocabulary or monotonicity (those are predicate concerns).
pub fn parse_turn_payload(raw: &str) -> Result<TurnPayload, EnvelopeParseError>;

/// Parse + validate: calls parse_turn_payload then checks turn range and
/// confidence range. Does NOT validate slot vocabulary (that is P3).
pub fn parse_and_validate(raw: &str) -> Result<TurnPayload, EnvelopeParseError>;
```

### W3 — `src/runtime/grill_predicates.rs`

```rust
use crate::grill_envelope::TurnPayload;

/// Per-turn language selector, mirroring cmd_spec.rs Lang.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lang { Zh, En }

/// Typed predicate verdict (R2 §A8).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum PredicateVerdict {
    Pass,
    Fail(PredicateFailureClass),
}

impl PredicateVerdict {
    pub fn is_pass(&self) -> bool { matches!(self, PredicateVerdict::Pass) }
    pub fn failure_class(&self) -> Option<PredicateFailureClass> {
        match self {
            PredicateVerdict::Pass => None,
            PredicateVerdict::Fail(c) => Some(*c),
        }
    }
}

/// Byte-stable typed failure class (discriminants must not change; tail-additive).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[repr(u8)]
pub enum PredicateFailureClass {
    SchemaParseError   = 0,  // P1
    KindMismatch       = 1,  // P2
    UnknownSlot        = 2,  // P3
    NonMonotonic       = 3,  // P4
    TurnOutOfRange     = 4,  // P5
    LanguageMismatch   = 5,  // P6 lang ratio
    QuestionTooShort   = 6,  // P6 < 8 chars
    QuestionMissing    = 7,  // P2 sub-case: done=false + null question
    PlaybackMissing    = 8,  // P2 sub-case: done=true + null playback
    ConfidenceOutOfRange = 9, // envelope validation
}

/// All 6 per-turn predicate results bundled together.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PredicateBundle {
    pub p1_schema_parse_ok: PredicateVerdict,
    pub p2_kind_ok: PredicateVerdict,
    pub p3_slots_in_vocab: PredicateVerdict,
    pub p4_monotonic: PredicateVerdict,
    pub p5_turn_bounded: PredicateVerdict,
    pub p6_question_nonempty_lang: PredicateVerdict,
}

/// Run all 6 per-turn predicates. `prev_covered` is covered_slots from turn N-1
/// (empty Vec for turn 1). Returns a PredicateBundle (no short-circuit).
pub fn run_turn_predicates(
    payload: &TurnPayload,
    prev_covered: &[String],
    lang: Lang,
) -> PredicateBundle;

/// Session-aggregate termination gate (FC1-N9 session-aggregate variant per R2 §A3).
/// Returns Pass iff covered_slots ⊇ REQUIRED_SLOTS AND confidence ≥ 0.8 AND
/// payload.turn ≥ 4.
/// Predicate fail means: loop back (NOT L4.E). See R2 §A3.
pub fn termination_predicate(payload: &TurnPayload) -> PredicateVerdict;

// Individual predicate functions (callable in tests independently):
pub fn p2_kind_ok(payload: &TurnPayload) -> PredicateVerdict;
pub fn p3_slots_in_vocab(payload: &TurnPayload) -> PredicateVerdict;
pub fn p4_monotonic(payload: &TurnPayload, prev_covered: &[String]) -> PredicateVerdict;
pub fn p5_turn_bounded(payload: &TurnPayload) -> PredicateVerdict;
pub fn p6_question_nonempty_lang(payload: &TurnPayload, lang: Lang) -> PredicateVerdict;
```

### W5 — `src/bin/turingos/spec_capsule.rs` (schema_id constants + GrillAttemptOutcome)

```rust
pub const SPEC_GRILL_TURN_CAPSULE_SCHEMA_ID: &str = "turingos-spec-grill-turn-v1";
pub const SPEC_GRILL_SESSION_CAPSULE_SCHEMA_ID: &str = "turingos-spec-grill-session-v1";

/// Grill-specific outcome enum (R2 §A1). Byte-stable discriminants. Tail-additive.
/// NOT an extension of AttemptOutcome (R2 §A1 hard rule).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[repr(u8)]
pub enum GrillAttemptOutcome {
    PredicatesPassed   = 0,
    SchemaParseFailed  = 1,
    KindMismatch       = 2,
    UnknownSlot        = 3,
    NonMonotonic       = 4,
    TurnOutOfRange     = 5,
    LanguageMismatch   = 6,
    LlmApiError        = 7,
    DoubleRetryFailed  = 8,
    TerminationGated   = 9,
    // R2 A5: tail-append only after W4.5 lands
    // TriageNonRelevant = 10,
}
```

---

## Task 5: Class-4 Surface Non-Touch Confirmation

| Atom | Files touched |
|---|---|
| W0 | `handover/directives/2026-05-18_TISR_PHASE6_3_X_W0_PLAN.md` (this file) |
| W1 | `assets/prompts/grill_meta_v1.md`, `assets/prompts/grill_synthesis_zh.md`, `assets/prompts/grill_synthesis_en.md`, `assets/prompts/grill_triage_blackbox_v1.md` |
| W2 | `src/runtime/grill_envelope.rs` (NEW), `src/runtime/mod.rs` (+1 pub mod line) |
| W3 | `src/runtime/grill_predicates.rs` (NEW), `src/runtime/mod.rs` (+1 pub mod line), `tests/grill_predicates_p1_p6.rs`, `tests/grill_predicates_termination.rs` |
| W4 | `src/bin/turingos/cmd_llm.rs` (extend +complete), `tests/cmd_llm_complete_stub.rs` |
| W4.5 | `src/bin/turingos/cmd_llm.rs` (extend +triage), `assets/prompts/grill_triage_blackbox_v1.md`, `tests/cmd_llm_triage_stub.rs` |
| W5 | `src/bin/turingos/spec_capsule.rs` (extend), `tests/grill_turn_capsule_write_read.rs`, `tests/grill_session_capsule.rs` |
| W6 | `src/bin/turingos/cmd_spec.rs` (extend +driven), `tests/cmd_spec_driven_mode_stub.rs`, `tests/spec_grill_byte_compat.rs` |
| W7 | `src/web/spec.rs`, `src/web/ws.rs`, `src/web/router.rs`, `src/web/mod.rs`, `tests/web_spec_turn_endpoint.rs` |
| W8 | `frontend/src/components/spec-grill.ts`, `frontend/src/types/spec.ts` |
| W9 | `handover/evidence/stage_phase6_3_x_grill_driven_<ts>/` (evidence only) |
| W10 | `handover/audits/AUDITOR_TISR_PHASE6_3_X_R2_IMPL.md` (audit record only) |

**Forbidden file union check — none of the above paths intersect**:
- `src/state/sequencer.rs` — not touched
- `src/state/typed_tx.rs` — not touched
- `src/bottom_white/cas/schema.rs` — not touched
- `src/runtime/prompt_capsule.rs` — not touched (only called, not modified)
- `src/runtime/attempt_telemetry.rs` — not touched (R2 §A1 hard rule)
- `genesis_payload.toml` — not touched
- `Cargo.toml` / `Cargo.lock` — not touched
- `src/kernel.rs` / `src/bus.rs` / `src/sdk/tools/wallet.rs` — not touched

**R2 §A9 forbidden-file regex check** (orchestrator runs after each atom commit):

```bash
git diff --name-only <base>..HEAD | rg \
  '^(src/(state/(sequencer|typed_tx)\.rs|kernel\.rs|bus\.rs|sdk/tools/wallet\.rs|bottom_white/cas/schema\.rs|runtime/(prompt_capsule|attempt_telemetry)\.rs)|genesis_payload\.toml|Cargo\.(toml|lock))$' \
  && echo "FORBIDDEN FILE TOUCHED — ABORT ATOM" && exit 1
```

If this regex matches **any** line of output, the orchestrator aborts that atom, dispatches a fresh agent with the gate-fail message + original spec, and does not proceed to the next atom.
