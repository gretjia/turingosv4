# OBS — R-022 trace-removal exception for TB-16 Atom 7 R3 closure

**Date**: 2026-05-04
**Rule**: R-022 (TRACE_MATRIX pub-symbol-block / I-REMOVAL clause)
**Files**: `src/runtime/audit_assertions.rs`
**Symbols**: `assert_a_chain_agent_ids_sandbox_prefixed` (id=41 walker)
**Skip-token**: `[R-022-skip: TB-16 R3 closure text extension; TRACE_MATRIX backlink preserved + extended; OBS_R022_TB_16_R3_TRACE_MATRIX_TEXT_EXTENSION_2026-05-04.md]`

---

## §1 Why R-022 fires

The R-022 trace-removal detector parses unified diffs line-by-line.
Any `-/// TRACE_MATRIX ...` line in the diff triggers a removal violation
**regardless of whether a corresponding `+/// TRACE_MATRIX ...` line exists
on the same symbol**.

In TB-16 Atom 7 R3 closure, I extended the doc-comment of
`assert_a_chain_agent_ids_sandbox_prefixed` (the id=41 sandbox-prefix
walker) to reflect:
- closure of Codex R3 RQ3 (walker now extracts ALL AgentId fields, not
  just `submitter_id`) and
- closure of Gemini R3 RQ3 (walker now traverses L4.E rejection records).

The extended single-line doc-comment now reads:

```rust
/// TRACE_MATRIX TB-16 Atom 7 R3 (Gemini R2 Q10 closure 2026-05-04):
/// machine-verifiable CR-16.7 ("no production user funds") via L4 +
/// L4.E walk over ALL AgentId fields. R3 closure (Gemini R3 RQ3 +
/// Codex R3 RQ3 — 2026-05-04): walker now extracts every AgentId-
/// bearing field per variant (NOT just `submitter_id`) plus walks
/// L4.E rejected records by direct `agent_id`. FC1-N34 + FC2-N31.
```

The diff replaces the original single-line `/// TRACE_MATRIX TB-16 Atom 7 R3 (Gemini R2 Q10 closure 2026-05-04):`
with this extended single-line variant, so the diff parser sees the
original line as REMOVED even though the same TRACE_MATRIX token plus
EXPANDED context is present in the new line. The pub symbol still has
a TRACE_MATRIX backlink (in fact, a stronger one — now explicitly
bound to FC1-N34 + FC2-N31).

## §2 Why the skip is justified

1. **No backlink loss**: the symbol still has a TRACE_MATRIX backlink
   in its preceding doc-comment block. The new line is strictly an
   EXTENSION of the old one (preserves "TB-16 Atom 7 R3 (Gemini R2 Q10
   closure 2026-05-04)" prefix verbatim, adds "+ R3 closure" suffix).
2. **Stronger FC binding**: the new line explicitly cites `FC1-N34 +
   FC2-N31`, which the old line did not (the FC binding was implicit
   via file-level header). This addresses Gemini R2 Q11 / R3 RQ5
   (TRACE_MATRIX precision) which itself was a CHALLENGE in the audit
   cycle — i.e., the whole reason for this edit is to MORE precisely
   track FC.
3. **Cross-fixture verification**: the R3 surgical fix the new
   doc-comment describes is verified by:
   - `cargo test --workspace` = 907 PASS / 0 FAILED / 150 ignored
   - `audit_pipeline_smoke/verdict.json` PROCEED 38/0/0/3 (R3 ids 1-41)
   - `arena_run4/tamper_report.json` 3/3 detected with R3 supplemental
     ids (id=40 + id=41) present.
4. **Audit closure**: this skip is being filed as part of a
   conservative-merge VETO closure (Codex R3 VETO×2 + Gemini R3
   CHALLENGE×2 → SHIP-WITH-OBS post surgical closure). The audit
   record is in
   `handover/audits/RECURSIVE_AUDIT_TB_16_R3_2026-05-04.md`.

## §3 Cross-references

- R3 closure doc: `handover/audits/RECURSIVE_AUDIT_TB_16_R3_2026-05-04.md`
- Audit transcripts:
  - `handover/audits/CODEX_TB_16_SHIP_AUDIT_2026-05-04_R3.md`
  - `handover/audits/GEMINI_TB_16_SHIP_AUDIT_2026-05-04_R3.md`
- SHIP_STATUS: `handover/ai-direct/TB-16_SHIP_STATUS_2026-05-04.md` §7
- TB-16 charter: `handover/tracer_bullets/TB-16_charter_2026-05-04.md`
- Architect §7: `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md`

## §4 Forward action

None — this is a one-shot exception for the TB-16 R3 closure commit.
The file post-closure has correct TRACE_MATRIX backlinks and will pass
R-022 on subsequent commits without skip tokens.
