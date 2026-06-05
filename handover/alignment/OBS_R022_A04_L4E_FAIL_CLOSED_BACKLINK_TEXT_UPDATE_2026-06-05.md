# OBS R-022 - A04 L4.E fail-closed TRACE_MATRIX text update

**Date**: 2026-06-05
**Rule**: R-022 (TRACE_MATRIX pub-symbol-block / exact-line removal detector)
**Authority**: `APPROVE-A04-SECTION8-CHAINTAPE-L4`
**File**: `src/bottom_white/ledger/rejection_evidence.rs`
**Symbol**: `advance_l4e_ref_for_record`
**Skip-token**: `[R-022-skip: A04 L4E fail-closed backlink text update; TRACE_MATRIX anchor preserved; OBS_R022_A04_L4E_FAIL_CLOSED_BACKLINK_TEXT_UPDATE_2026-06-05.md]`

---

## Section 1 - Why R-022 fires

The A04 ChainTape-L4 hardening changes the TRACE_MATRIX doc-comment for
`advance_l4e_ref_for_record` from a best-effort/non-propagating description to
a fail-closed/canonical-ref description.

The old line said failures were logged and did not propagate. That is no longer
true under A04: when `TURINGOS_CHAINTAPE_PATH` is set, `refs/chaintape/l4e`
advance failure must make the append fail, and the JSONL backing file must not
remain replayable as an accepted second source.

R-022 sees the removed `-/// TRACE_MATRIX ...` line in the unified diff and
blocks the commit even though the replacement line keeps the same Stage A3 /
HEAD_t C2 R3.5 / SG-A3.2 TRACE_MATRIX anchor on the same function.

## Section 2 - Why the skip is justified

1. No public symbol was removed, renamed, or moved.
2. The TRACE_MATRIX backlink remains on `advance_l4e_ref_for_record`.
3. The wording change is semantically required: preserving the old best-effort
   text would contradict the A04 ChainTape-L4 fail-closed behavior.
4. The change narrows authority rather than expanding it: a failed canonical ref
   advance is now rejected by the append path.

## Section 3 - Verification

Regression coverage added in `tests/tc_chaintape_ledger_hardening.rs` verifies:

- accepted L4 commits advance `refs/chaintape/l4` and keep the C1 alias aligned;
- divergent C1 transition aliases are repaired from canonical L4;
- logical_t gaps fail without advancing the canonical L4 head;
- L4.E canonical-ref failure returns the previous hash, keeps memory length at
  zero, leaves the JSONL file empty, and reopens with zero replayed records.

Targeted verification also covered:

- `cargo test --test constitution_l4e_body_integrity --no-fail-fast -- --test-threads=1`
- `cargo test --test tb_6_l4e_jsonl_persistence --no-fail-fast -- --test-threads=1`
- `cargo test --test tb_6_verify_chaintape --no-fail-fast -- --test-threads=1`
- `cargo test --test constitution_g1_resume --no-fail-fast -- --test-threads=1`
- `cargo test --test dual_substrate_disjointness --no-fail-fast`
- `cargo test --test constitution_matrix_drift --no-fail-fast`
- `bash scripts/run_constitution_gates.sh`

## Section 4 - Scope boundary

This OBS is only for the TRACE_MATRIX text update caused by A04 L4.E
fail-closed hardening. It does not authorize TDMA runtime work, sequencer
admission changes, typed transaction schema changes, signing payload changes, or
constitution/flowchart edits.
