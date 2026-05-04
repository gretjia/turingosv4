# OBS R-022 — TB-16 tamper assertion backlinks re-attributed to FC1-N35

**Date**: 2026-05-04
**Source**: Gemini TB-16 R1 ship audit Q11 finding.
**Symbols**: `assert_36_tamper_l4_flip_detected`,
`assert_37_tamper_cas_flip_detected`, `assert_38_tamper_l4_remove_detected`
in `src/runtime/audit_assertions.rs`.

**Change**: doc-comment TRACE_MATRIX backlink replaced from
`FC1-N34 + FC2-N31` (audit_tape battery) → `FC1-N35` (audit_tape_tamper
binary). The original attribution was incorrect — these assertions
are SKIPPED inside `audit_tape` and exercised exclusively by
`audit_tape_tamper`. Gemini Q11 caught the mis-attribution.

**Justification**: this is a CORRECTION, not a removal. Each symbol
gains a more accurate (and stricter) FC binding. R-022's removal-
detection heuristic flagged the change; the new backlink is in fact
strictly stronger (more specific FC node).

**No structural change**: function signatures, return types, and
behavior are unchanged.
