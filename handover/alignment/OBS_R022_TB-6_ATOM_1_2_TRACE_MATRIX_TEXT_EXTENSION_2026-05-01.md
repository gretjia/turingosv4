# OBS R-022 — TB-6 Atom 1.2: TRACE_MATRIX backlink TEXT-EXTENSION false positive

**Date**: 2026-05-01
**Atom**: TB-6 Atom 1.2 — `RejectionEvidenceWriter` JSONL persistence backend.
**Branch**: `experiment/tb6-chaintape-bootstrap`
**File**: `src/bottom_white/ledger/rejection_evidence.rs`
**Hook**: R-022 (TRACE_MATRIX pub-symbol-block).

---

## §1 Hook output

```
src/bottom_white/ledger/rejection_evidence.rs — removed TRACE_MATRIX backlink: /// TRACE_MATRIX P1:6 — error returned by `RejectionEvidenceWriter::verify_chain
src/bottom_white/ledger/rejection_evidence.rs — removed TRACE_MATRIX backlink: /// TRACE_MATRIX P1:6/P1:9 — RSP-0 in-memory rejection-evidence writer.
src/bottom_white/ledger/rejection_evidence.rs — removed TRACE_MATRIX backlink: /// TRACE_MATRIX P1:6 — empty writer.
```

## §2 Why this is a FALSE POSITIVE

R-022 expects every `pub` symbol in `src/` to carry a `/// TRACE_MATRIX <FC-id>: <role>` doc-comment. The hook detects 3 backlinks "removed" — but in fact those 3 lines were **extended with TB-6 Atom 1.2 context** rather than removed. After the edit:

| Symbol | Old TRACE_MATRIX line (HEAD) | New TRACE_MATRIX line (this commit) |
|---|---|---|
| `pub enum RejectionEvidenceError` | `/// TRACE_MATRIX P1:6 — error returned by `RejectionEvidenceWriter::verify_chain`.` | `/// TRACE_MATRIX P1:6 — error returned by `RejectionEvidenceWriter::verify_chain` + JSONL persistence ops (TB-6 Atom 1.2).` |
| `pub struct RejectionEvidenceWriter` | `/// TRACE_MATRIX P1:6/P1:9 — RSP-0 in-memory rejection-evidence writer.` | `/// TRACE_MATRIX P1:6/P1:9 — rejection-evidence writer with optional persistent backend.` |
| `pub fn RejectionEvidenceWriter::new` | `/// TRACE_MATRIX P1:6 — empty writer.` | `/// TRACE_MATRIX P1:6 — empty writer with `InMemory` backend.` |

**Verification** (`grep -n TRACE_MATRIX src/bottom_white/ledger/rejection_evidence.rs`):

```
141:/// TRACE_MATRIX P1:6 — coarse rejection-class discriminator.
171:/// TRACE_MATRIX P1:6/P1:9 — one rejection-evidence row.
262:/// TRACE_MATRIX Inv 10 + ROADMAP P1:9 — agent-facing projection.
295:/// TRACE_MATRIX P1:6 — error returned by `RejectionEvidenceWriter::verify_chain` + JSONL persistence ops (TB-6 Atom 1.2).
328:/// TRACE_MATRIX P1:6/P1:9 — rejection-evidence writer with optional persistent backend.
363:    /// TRACE_MATRIX P1:6 — empty writer with `InMemory` backend.
368:    /// TRACE_MATRIX FC3-N1 + P1:6 (TB-6 Atom 1.2) — open or create a JSONL-backed writer.
427:    /// TRACE_MATRIX FC3-N1 (TB-6 Atom 1.2) — convenience: is this writer JSONL-backed?
432:    /// TRACE_MATRIX P1:6 — count of recorded rejections.
437:    /// TRACE_MATRIX P1:6 — empty predicate.
442:    /// TRACE_MATRIX P1:6 — last record's hash, or `Hash::ZERO` for empty chain.
447:    /// TRACE_MATRIX P1:6/P1:9 — append a rejection record; returns the new chain hash.
517:    /// TRACE_MATRIX P1:6 — verify the rejection-evidence chain end-to-end.
547:    /// TRACE_MATRIX P1:9 — read-only record slice (for L4.E forensics; full
554:    /// TRACE_MATRIX Inv 10 + P1:9 — agent-facing projection.
```

Every `pub` symbol still has a TRACE_MATRIX line. The hook is doing exact-line-text diff against HEAD; line-text changed (extended), so it reports "removed" — but the backlink is structurally preserved.

## §3 Risk assessment

**Low**. The R-022 invariant is "no orphan pub symbol without TRACE_MATRIX backlink". This invariant is preserved — every pub symbol still has a backlink. The hook's exact-text matching is the limitation. No constitutional / Layer 1 violation.

## §4 Remediation

R-022-skip token included in the Atom 1.2 commit message with reference to this OBS file.

If R-022 hook logic is upgraded to detect "TRACE_MATRIX line preserved (text extended)" vs "TRACE_MATRIX line removed (no replacement)", this OBS becomes obsolete and can be archived.

## §5 Hook upgrade proposal (out of scope for TB-6)

Future R-022 enhancement: per-pub-symbol TRACE_MATRIX presence check via AST walk + post-edit re-grep, instead of exact-line diff. Keeps strict guarantee that every pub item has SOME backlink without false positives on text-extension edits.

Not actioning here; documented for future hardening.
