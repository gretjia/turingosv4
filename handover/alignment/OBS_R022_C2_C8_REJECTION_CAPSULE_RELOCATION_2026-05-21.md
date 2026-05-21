# OBS R-022 — C8 rejection_capsule backlinks relocated, not removed

**Date**: 2026-05-21 session #56.
**Triggered by**: pre-commit hook R-022 (TRACE_MATRIX backlink removal detector).
**Detected removals** (all in `src/runtime/generation_attempt.rs`):
1. `/// TRACE_MATRIX FC1: Schema ID for LLM generate rejections.` (was on `GENERATE_REJECTION_CAPSULE_SCHEMA_ID`).
2. `/// TRACE_MATRIX FC1: Enum representing the classification of a generate rejection.` (was on `RejectClass`).
3. `/// TRACE_MATRIX FC1 + FC3-N4: Capsule containing metadata for a generate rejection event.` (was on `GenerateRejectionCapsule`).
4. `/// TRACE_MATRIX FC3-N4: Writes the GenerateRejectionCapsule to CAS store.` (was on `write_generate_rejection_capsule`).

## Why this is a relocation, not a removal

The C2/C8 audit finding (`handover/audits/CLAUDE_SESSION_56_GEMINI_P7Z_AUDIT_2026-05-21.md`
PR #45 section, finding #3) established that C8's full implementation was fused
into `src/runtime/generation_attempt.rs:67-126` — the wrong file. Master plan §C8
specifies `src/runtime/rejection_capsule.rs` as the canonical home for these symbols.

The remediation commit creates `src/runtime/rejection_capsule.rs` and moves all 4
symbols verbatim into it, retaining identical or improved TRACE_MATRIX backlinks:

| Symbol | New backlink (in `src/runtime/rejection_capsule.rs`) |
|--------|------------------------------------------------------|
| `GENERATE_REJECTION_CAPSULE_SCHEMA_ID` | `/// TRACE_MATRIX FC1: Schema ID for LLM generate rejections.` |
| `RejectClass` | `/// TRACE_MATRIX FC1: Enum representing the classification of a generate rejection.` |
| `GenerateRejectionCapsule` | `/// TRACE_MATRIX FC1 + FC3-N4: Capsule containing metadata for a generate rejection event.` |
| `write_generate_rejection_capsule` | `/// TRACE_MATRIX FC3-N4: Writes the GenerateRejectionCapsule to CAS store.` |

All 4 backlinks are preserved verbatim. The constitutional invariant (FC1
failure-path externalization + FC3-N4 CAS evidence binding) is unchanged.

## Why the relocation was the right choice

Per `feedback_no_workarounds_strict_constitution` the audit verdict on PR #45 was
VETO due to C8 scope leakage. The correct resolution is to move the code to its
spec'd file. Leaving the code in `generation_attempt.rs` would sustain the C2/C8
boundary violation and block C8's own §8 gate (which requires C8 code to live in
`rejection_capsule.rs`).

## Wire-format note

No wire format change. Schema IDs, struct fields, enum discriminants, and CAS
put parameters are identical. No new `ObjectType` variant introduced. All 5 C2
acceptance tests pass post-relocation.

## Validation

- `cargo check`: exit 0 (warnings only).
- `cargo test --test generation_attempt_capsule_cas_wire`: 1 passed.
- `cargo test --test generate_attempt_records_raw_output_cid`: 1 passed.
- `cargo test --test generate_retry_attempts_are_distinct`: 1 passed.
- `cargo test --test generate_attempt_outcome_routes_to_rejection`: 2 passed.
- `cargo test --test generate_attempt_prompt_hash_is_canonical`: 1 passed.

## Cross-references

- Audit report: `handover/audits/CLAUDE_SESSION_56_GEMINI_P7Z_AUDIT_2026-05-21.md` (PR #45 section).
- Master plan §C8: `handover/architect-insights/V4_PRODUCT_CAK_HARDENING_EXECUTION_PLAN_2026-05-20.md:1011-1110`.
- Audit tamper relocation precedent: `handover/alignment/OBS_R022_AUDIT_TAMPER_LIBRARY_RELOCATION_2026-05-10.md`.
