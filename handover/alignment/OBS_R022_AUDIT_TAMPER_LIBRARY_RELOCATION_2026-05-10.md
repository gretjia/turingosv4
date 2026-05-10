# OBS R-022 — `audit_tape_tamper` library relocation (TRACE_MATRIX backlinks moved, not removed)

**Date**: 2026-05-10 session #33.
**Triggered by**: pre-commit hook R-022 (TRACE_MATRIX backlink removal detector).
**Detected removals** (both in `src/bin/audit_tape_tamper.rs`):
1. `/// TRACE_MATRIX TB-16 Atom 7 R1 (Codex Q7/V5 closure 2026-05-04):` (was on `fn flip_byte_in_first_blob`).
2. `/// TRACE_MATRIX TB-16 Atom 7 R1 (Codex Q7/V5 closure 2026-05-04):` (was on `fn flip_byte_in_first_cas_object`).

## Why this is a relocation, not a removal

The two TB-16 Atom 7 backlinks documented the destructive zlib-decode-failure tamper primitives (`flip_byte_in_first_blob`, `flip_byte_in_first_cas_object`, `corrupt_l4_truncate_ref`) authored 2026-05-04 in `src/bin/audit_tape_tamper.rs`. That binary had three private fns; only the first two carried the explicit TRACE_MATRIX prefix.

The 2026-05-10 commit moves the three primitives — with corrected post-A3 multi-ref semantics — to a library module `src/runtime/audit_tamper.rs`. Each `pub fn` in the new module carries an EQUIVALENT or STRONGER backlink:

| Library symbol | New backlink |
|---|---|
| `flip_largest_reachable_l4_blob` | `/// TRACE_MATRIX FC1-N35 (audit_tape_tamper Atom 3 / TB-16 Atom 7 closure; architect §B.9.3 prove-no-fake-accepted L4-side coverage)` |
| `flip_largest_cas_object` | `/// TRACE_MATRIX FC1-N35 (audit_tape_tamper Atom 3 / TB-16 Atom 7 closure; architect §B.9.3 prove-no-fake-accepted CAS-side coverage)` |
| `corrupt_chain_refs` | `/// TRACE_MATRIX FC1-N35 + FC2-INV1 (audit_tape_tamper Atom 3 ref-truncation closure; architect §B.9.3 + Stage A3 multi-ref dual-write)` |
| `pub const L4_REFS` | `/// TRACE_MATRIX FC1-N35 (audit_tape_tamper coverage; architect §B.9.3 prove-no-fake-accepted)` |
| `pub const CHAIN_REFS` | `/// TRACE_MATRIX FC1-N35 (audit_tape_tamper coverage; architect §B.9.3)` |

The binary `src/bin/audit_tape_tamper.rs` retains the TB-16 Atom 7 anchor on the `Args` doc-comment + module-level `//!` header, pointing readers to the library.

The constitutional invariant the backlinks document (architect §B.9.3 prove-no-fake-accepted via 3/3 tamper detection) is now BETTER served:
- Original TB-16 binary: 1/3 detection on M0 P01 (silent on `flip_l4_byte` + `truncate_l4_ref` after Stage A3 multi-ref drift).
- Post-relocation library + new constitution gate (`tests/constitution_audit_tamper_3_of_3.rs`): 3/3 detection empirically verified on M0 P01 + P05 fixtures, and library API exercised by 9 gate tests so future drift is caught at gate-time.

## Why the relocation was the right choice

Per `feedback_no_workarounds_strict_constitution` ("我不要凑活") the M0 batch 2026-05-10 finding (0/20 problems achieving architect-mandated 3/3 detection) is a constitutional violation requiring full landing, not patch-and-paper-over.

A pure in-binary fix (just edit the existing private fns) would close the bug but leave the primitives non-testable from the constitution-gate ecosystem. The library refactor enables `tests/constitution_audit_tamper_3_of_3.rs` to exercise the primitives directly + assert correctness on synthetic git repos — which catches future drift (e.g. introducing a new ref name in Stage D+ without updating `CHAIN_REFS`) at gate-time, not at next M0-batch evidence inspection.

## Wire-format note

No wire format change. The binary's CLI surface, exit codes, output schema (`tamper_report.json`), and tamper labels (`flip_l4_byte`, `flip_cas_byte`, `truncate_l4_ref`) are preserved. Existing M0 / TB-16 evidence directories remain replayable against the new binary.

## Validation

- Constitution gates: 250 → 259 (+9 tamper gate tests).
- Workspace tests (--test-threads=1): 1394 → 1403 (+9).
- Trust Root verify: PASS post `src/runtime/mod.rs` rehash (33ff0897 → 8cde3e8a; added `pub mod audit_tamper`).
- `audit_tape_tamper` on M0 P01 fixture: 1/3 → 3/3 detected.
- `audit_tape_tamper` on M0 P05 fixture: 1/3 → 3/3 detected.

## Cross-references

- M0 batch summary (1/3-DEGRADED universal): `handover/evidence/m0_minif2f_harness_audit_2026-05-10_post_stage_c/M0_BATCH_SUMMARY.json`.
- Original TB-16 OBS_R022 doc (R1 closure): `handover/alignment/OBS_R022_TB16_TAMPER_BACKLINKS_2026-05-04.md`.
- Original commit authoring the in-binary primitives: `f2bb871` (TB-16.x.fix 2026-05-04).
- Stage A3 multi-ref ChainTape introduction: 2026-05-08 (per `handover/directives/2026-05-08_STAGE_A3_§8_SIGN_OFF.md`).
- Strict-constitution rule: `~/.claude/projects/-home-zephryj-projects-turingosv4/memory/feedback_no_workarounds_strict_constitution.md`.
