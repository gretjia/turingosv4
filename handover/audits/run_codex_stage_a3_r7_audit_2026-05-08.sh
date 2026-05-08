#!/usr/bin/env bash
# Codex Stage A3 (HEAD_t C2 multi-ref ChainTape) ship-gate audit (R7).
#
# Per STAGE_A3_HEAD_T_C2_charter_2026-05-07.md SG-A3.10 + `feedback_audit_after_evidence`:
# G2 dual audit dispatched AFTER MVP gates green. Codex implementation-paranoid
# half. Independent of Gemini sanity audit.
# Per `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.

set -uo pipefail

ROOT=/home/zephryj/projects/turingosv4
ROUND="${TB_AUDIT_ROUND:-R1}"
OUT="${ROOT}/handover/audits/CODEX_STAGE_A3_R7_AUDIT_2026-05-08_${ROUND}.md"
TMP_PROMPT="$(mktemp /tmp/a3_r7_codex.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

if [ -e "$OUT" ]; then
  echo "[codex A3 R7] error: $OUT already exists; refusing to overwrite" >&2
  exit 2
fi

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Codex Stage A3 / HEAD_t C2 Multi-Ref ChainTape Audit — R1

You are Codex acting as the implementation-paranoid half of the Stage A3 R7 G2
dual audit per STAGE_A3_HEAD_T_C2_charter_2026-05-07.md §5 R7. Gemini 2.5 Pro
ran the strategic / architectural half in parallel (verdict: CHALLENGE; details
at handover/audits/GEMINI_STAGE_A3_R7_AUDIT_2026-05-08_R1.md).

Per feedback_dual_audit_conflict: VETO > CHALLENGE > PASS conservative ranking.
Round cap = 2.

## Your scope (implementation-paranoid)

Inspect each commit in the Stage A3 substrate lineage. Look for:

1. Subtle bugs in dual-write semantics (refs/transitions/main vs refs/chaintape/l4)
2. Bugs in the new advance_chaintape_l4e_to / advance_chaintape_cas_to helpers
3. Bugs in HeadTWitness::reconstruct_from_chaintape_refs (especially the cas_root sha256-of-OID derivation)
4. Bugs in the rejection_evidence flush_jsonl_record env-var hook (advance_l4e_ref_for_record)
5. Bugs in the CasStore::put hook
6. Test correctness in tests/constitution_head_t_c2_multi_ref.rs (do tests actually exercise the gates they claim?)
7. Trust Root rehash correctness in genesis_payload.toml

## Substrate at HEAD `90376ae`

Commit lineage:
- 72e2494 Stage A3 R1+R2+R4 (multi-ref support + HeadTWitness::reconstruct_from_chaintape_refs + 6 SG-A3 tests)
- 4b0062e Stage A3 R3 (CAS root ref hook in cas/store.rs)
- 2d3d948 Stage A3 R5 smoke (mathd_algebra_107)
- f7a6660 Stage A3 R3.5 wire + smoke (10/10 1:1 ref-to-JSONL match)
- 381554f Stage B3 R6 mini-M1 (8 problems × n=1; 8/8 l4e_jsonl_match=true; 83 commits aggregate)
- 90376ae Stage A3 R7 Gemini R1 audit + Q5 closure (10/10 chain_invariant.json Ok delta=0)

Gemini R1 raised concrete remediations:
- Q5 (chain_invariant.json missing) — CLOSED in 90376ae (10/10 verdict=Ok delta=0)
- Q2 (env-var seam preference) — FORWARD-BIND to Stage A3.6
- Q8 (refs/chaintape/cas GC + concurrency) — FORWARD-BIND to Stage A3.6

## Your questions (implementation-paranoid)

Q1. Read `src/bottom_white/ledger/transition_ledger.rs::Git2LedgerWriter::commit`. Is the dual-write to refs/chaintape/l4 atomically consistent with the C1 refs/transitions/main update? Specifically: if refs/chaintape/l4 ref-update fails AFTER refs/transitions/main commit succeeded, does the writer leave inconsistent state? Should the dual-write be reordered (chaintape/l4 first, then transitions/main as alias)?

Q2. Read `src/bottom_white/ledger/rejection_evidence.rs::flush_jsonl_record` + `advance_l4e_ref_for_record`. The git2 commit author/committer time uses `submit_id` cast to i64. submit_id is u64. Is there an overflow concern at very large submit_ids (>2^63)? Realistic? Defensible?

Q3. Read `src/state/head_t_witness.rs::reconstruct_from_chaintape_refs`. The cas_root mapping uses `Sha256::new() update("chaintape/cas/oid") update(o.as_bytes())`. Is this domain separator correctly disambiguating from other Hash usages in the codebase? Search src/ for any other Sha256 update with similar prefix that could collide.

Q4. Read `src/bottom_white/cas/store.rs::CasStore::put` (post-A3 R3). The advance_chaintape_cas_to call is `let _ = ...`. If this fails silently mid-batch, does the next put recover? Verify by tracing the recovery path.

Q5. Read `tests/constitution_head_t_c2_multi_ref.rs` all 7 tests. Do they actually fail under their stated kill conditions? Specifically: pick `sg_a3_no_hidden_filesystem_pointer` and identify any forbidden pointer name that should be on the list but isn't (potential coverage hole).

Q6. Read `genesis_payload.toml` for the entries that were rehashed by Stage A3 (`src/bottom_white/ledger/transition_ledger.rs`, `src/bottom_white/cas/store.rs`, `src/bottom_white/ledger/rejection_evidence.rs`, `src/runtime/mod.rs`). Are the rehash predecessor chains documented correctly? Any hash drift?

Q7. Stage A3 substrate intersects with rejection_evidence.rs (Class-3 per `feedback_class4_cannot_hide_in_class3` analysis: "rejection_evidence.rs is NOT in CLAUDE.md §12 STEP_B restricted list"). Verify: does the A3 R3.5 change to flush_jsonl_record cross any Class-4 line? In particular: does it modify RejectedSubmissionRecord schema, compute_hash byte stream, or sequencer admission semantics? If yes → VETO.

Q8. Run `cargo test --workspace --no-fail-fast` and `bash scripts/run_constitution_gates.sh` mentally. Do the claimed totals match (1287 PASS / 0 failed; 154 GREEN / 0 failed)? Any tests that are passing vacuously per CR-C0.1 / closure-3?

## Verdict format

```
Q1: PASS|CHALLENGE|VETO - <reason>
Q2: PASS|CHALLENGE|VETO - <reason>
Q3: PASS|CHALLENGE|VETO - <reason>
Q4: PASS|CHALLENGE|VETO - <reason>
Q5: PASS|CHALLENGE|VETO - <reason>
Q6: PASS|CHALLENGE|VETO - <reason>
Q7: PASS|CHALLENGE|VETO - <reason>
Q8: PASS|CHALLENGE|VETO - <reason>

## VERDICT: PASS|CHALLENGE|VETO
Conviction: low|medium|high
Recommendation: PROCEED|FIX-THEN-PROCEED|REDESIGN
Remediations:
- <only for CHALLENGE/VETO; actionable and scoped>
```

If any Q is VETO, aggregate verdict must be VETO. If any Q is CHALLENGE (no VETO), aggregate is CHALLENGE.

Save your audit verbatim. Do not edit src/. Read-only review.
BRIEF_EOF

echo "[codex A3 R7] dispatching codex exec ..." >&2
echo "[codex A3 R7] prompt size: $(wc -c < "$TMP_PROMPT") chars" >&2

cat "$TMP_PROMPT" | codex exec --skip-git-repo-check --sandbox read-only --color never - > "$OUT.raw" 2>&1
EXIT=$?
echo "[codex A3 R7] codex exec exit: $EXIT" >&2

# Strip codex's wrapper output to keep just the verdict
if [ -s "$OUT.raw" ]; then
    cp "$OUT.raw" "$OUT"
    rm -f "$OUT.raw"
fi

if [ -f "$OUT" ]; then
    VERDICT=$(grep -oE 'VERDICT:\s*(PASS|CHALLENGE|VETO)' "$OUT" | head -1 | awk '{print $2}')
    echo "[codex A3 R7] verdict: ${VERDICT:-UNKNOWN}" >&2
    echo "[codex A3 R7] saved: $OUT" >&2
fi
exit $EXIT
