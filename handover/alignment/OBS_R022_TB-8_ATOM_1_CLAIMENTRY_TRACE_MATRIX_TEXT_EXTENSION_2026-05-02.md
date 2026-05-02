# OBS R-022 — TB-8 Atom 1: ClaimEntry TRACE_MATRIX backlink TEXT-EXTENSION false positive

**Date**: 2026-05-02
**Atom**: TB-8 Atom 1 — `ClaimEntry` 6-field expansion + `ClaimStatus` enum.
**Branch**: `main` (TB-8 ship commit).
**File**: `src/state/q_state.rs`
**Hook**: R-022 (TRACE_MATRIX pub-symbol-block).

---

## §1 Hook output

```text
src/state/q_state.rs — removed TRACE_MATRIX backlink: /// TRACE_MATRIX WP § 2 — claim entry shape (stub). Full fields land CO P2.6.
```

## §2 Why this is a FALSE POSITIVE

R-022 expects every `pub` symbol in `src/` to carry a `/// TRACE_MATRIX <FC-id>: <role>` doc-comment. The hook detects 1 backlink "removed" — but in fact the line was **extended with TB-8 Atom 1 context** rather than removed. After the edit:

| Symbol | Old TRACE_MATRIX line (HEAD) | New TRACE_MATRIX line (this commit) |
|---|---|---|
| `pub struct ClaimEntry` | `/// TRACE_MATRIX WP § 2 — claim entry shape (stub). Full fields land CO P2.6.` | `/// TRACE_MATRIX WP § 2 — claim entry shape. Extended in TB-8 Atom 1 (2026-05-02) per handover/audits/CHARTER_RATIFICATION_TB_8_2026-05-02.md §1 Q1 ratification: 6 new fields drive the Atom-3 FinalizeReward dispatch arm without re-traversing stakes_t / escrows_t / L4. All additive; every field carries #[serde(default)] so historical rows (TB-3..TB-7R never wrote a ClaimEntry — claims_t was a never-written stub) deserialize cleanly when re-read post-TB-8.` |

**Verification** (`grep -n TRACE_MATRIX src/state/q_state.rs`):

```text
$ grep -n "TRACE_MATRIX WP § 2 — claim entry shape" src/state/q_state.rs
232:/// TRACE_MATRIX WP § 2 — claim entry shape. Extended in TB-8 Atom 1
```

The `WP § 2` anchor is preserved verbatim. Only the trailing parenthetical "(stub). Full fields land CO P2.6." was replaced with the TB-8 expansion narrative — because `claims_t` is no longer a stub (TB-8 Atom 1 IS the "CO P2.6" body that the old comment forward-referenced).

## §3 Resolution

This OBS doc serves as the auditable explanation for the R-022-skip token in the TB-8 ship commit message. No constitutional debt is created; no orphan symbol remains. Same WP § 2 anchor; richer doc-body reflecting the schema's TB-8 expansion.

## §4 Cross-references

- TB-8 charter: `handover/tracer_bullets/TB-8_charter_2026-05-02.md`
- Ratification: `handover/audits/CHARTER_RATIFICATION_TB_8_2026-05-02.md` §1 Q1
- Recursive audit: `handover/audits/RECURSIVE_AUDIT_TB_8_2026-05-02.md` §1.1
- Precedent: `handover/alignment/OBS_R022_TB-6_ATOM_1_2_TRACE_MATRIX_TEXT_EXTENSION_2026-05-01.md`
