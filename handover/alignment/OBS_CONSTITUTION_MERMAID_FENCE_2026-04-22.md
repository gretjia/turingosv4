# Observation: FC-2 / FC-3 mermaid fence missing in `constitution.md`

**Date**: 2026-04-22
**Severity**: cosmetic (does not affect code alignment; affects GitHub rendering)
**Discovered by**: Stage 1 flowchart extractor agent (Phase Z')
**Constitutional status**: FILED FOR HUMAN ARCHITECT (Claude does NOT modify `constitution.md`, per user directive 宪法不能改)

---

## Facts

`grep -n '```' constitution.md` yields:

- Line 325: ` ```mermaid ` (FC-1 opener) ✓
- Line 379: ` ``` ` (FC-1 closer) ✓
- Line 530: ` ``` ` (FC-2 closer) ← **opener missing**
- Line 714: ` ``` ` (FC-3 closer) ← **opener missing**

FC-2 starts at line 441 with `    flowchart TD` (4-space indent) but has no preceding ` ```mermaid ` fence.
FC-3 starts at line 670 with `    graph TB` (4-space indent) but also has no preceding fence.

## Consequence

Only FC-1 renders correctly on GitHub / Notion / any markdown viewer. FC-2 and FC-3 appear as plain (indented) text with only an unbalanced closing backtick.

## Impact on Phase Z′ alignment

Zero. Stage 1 agent parsed all three flowcharts by treating the indented text as if properly fenced. Element extraction + code mapping proceeded successfully. The TRACE_MATRIX is not affected.

## Recommended fix (for human architect only)

Add ` ```mermaid ` before line 441 and before line 670. Keep all content unchanged. After fix, `grep -c '```mermaid' constitution.md` should return 3.

## Why Claude does not auto-fix

Per 2026-04-22 user directive: 宪法约定的不能改. Even cosmetic markdown-fence repair on `constitution.md` is human-architect territory. This observation is filed for reference; no runtime blocker.

## Related

- TRACE_MATRIX_v0 § 7 Constitutional-document hygiene note
- C-069 Constitutional Alignment Audit Protocol (ruling item 6)
