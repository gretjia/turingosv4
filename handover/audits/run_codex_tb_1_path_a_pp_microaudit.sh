#!/usr/bin/env bash
# Codex TB-1 Path A++ MICRO-AUDIT (narrow, closed questions only).
#
# Per user ruling 2026-04-29 (`feedback_dual_audit_conflict` merged Day-6 verdict
# = CHALLENGE; Path A++ closes Codex P0-2 + P0-3 + P0-4):
#
#   - This is NOT a full round-2 audit. No Gemini.
#   - Three closed questions ONLY. Codex does NOT re-litigate the runtime
#     enforcement gap (that is intentionally TB-2 scope).
#   - Lesson from Day-6: round-1 prompt was 154KB; this prompt aims < 20KB.
#
# Disposition expected: PASS / CHALLENGE on EACH of the three closed points.

set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
OUT="${ROOT}/handover/audits/CODEX_TB_1_PATH_A_PP_MICROAUDIT_2026-04-29.md"
TMP_PROMPT="$(mktemp /tmp/tb1_codex_microaudit.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Codex TB-1 Path A++ Micro-Audit (closed questions only)

**Role**: skeptical adversarial reviewer.
**Mandate**: NARROW. Three closed questions on three landed patches.

**Background (read once; do NOT debate)**:
- Day-6 dual audit returned Codex CHALLENGE / Gemini PASS → merged CHALLENGE
  per `feedback_dual_audit_conflict` (VETO > CHALLENGE > PASS).
- User adopted Path A++: narrow the TB-1 ship claim AND close Codex P0-2 +
  P0-3 + P0-4 (the three lowest-cost code-side P0s). Codex P0-1 (runtime
  enforcement of the L4/L4.E split through `dispatch_transition`) is
  INTENTIONALLY NOT addressed in TB-1; it is the primary scope of the renamed
  TB-2 ("P1/P3 Runtime Boundary Closure + RSP-1"). DO NOT re-litigate P0-1.

**Decision rule**: each question independently returns one of:
- **PASS** — patch closes the question; no further action.
- **CHALLENGE** — concrete defect remains; specify the smallest patch that closes it.
- (No VETO disposition for this micro-audit; if you want to escalate beyond
  the three questions, file a separate CHALLENGE on the OUT-OF-SCOPE check
  at the bottom.)

---

## Q1 (P0-2). Does Tier-A now cover all six `EconomicState` holding subindexes?

**Codex P0-2 finding (Day-6)**: Tier-A test 7 only redistributes through
balances + escrows; the all-six-subindex coverage existed only at the
unit level (`ctf_counts_all_six_holding_subindexes` in
`src/economy/monetary_invariant.rs::tests`).

**Path A++ patch**: a 10th Tier-A blocking test was added. Verify:

1. Does this test exercise ALL six holding subindexes (`balances_t`,
   `escrows_t`, `stakes_t`, `claims_t`, `task_markets_t.bounty`,
   `challenge_cases_t.bond`)?
2. Does it route through `assert_total_ctf_conserved` (the production-path
   guard) — NOT `total_supply_micro` directly?
3. Would a regression that drops ANY single subindex from
   `total_supply_micro` cause this test to FAIL with a unique, identifiable
   delta (not a generic round-trip equality)?
4. Is the test in the `Tier-A — BLOCKING` section, NOT marked `#[ignore]`,
   and NOT in Tier-B?

If all four: **PASS**. Otherwise: **CHALLENGE** with the smallest patch that
closes the gap.

## Q2 (P0-3). Does `raw_diagnostic_cid` now fail closed under raw-record serialization?

**Codex P0-3 finding (Day-6)**: `RejectedSubmissionRecord` is `pub`, derives
`Serialize`, exposes `pub raw_diagnostic_cid`, and `records()` returns full
raw refs. Shielding was convention only — any agent-facing serialization
that bypassed `PublicRejectionView` would leak the raw cid.

**Path A++ patch**: `#[serde(skip_serializing, default)]` added on
`raw_diagnostic_cid`. Verify:

1. Is `serde_json::to_value(&record)` for a `RejectedSubmissionRecord` with
   `Some(cid)` raw diagnostic now structurally missing the
   `raw_diagnostic_cid` key?
2. Is the in-memory forensic access via `RejectionEvidenceWriter::records()`
   STILL able to read `raw_diagnostic_cid.is_some()` (i.e., the shield is
   serialization-side, not destructive)?
3. Is there an explicit unit test asserting both #1 and #2 in this commit?
4. Is the limitation (capability-gated forensic API is a future TB)
   documented in the field doc-comment so a future maintainer doesn't
   re-expose the cid?

If all four: **PASS**. Otherwise: **CHALLENGE**.

(Note on hash-chain: with `#[serde(skip_serializing, default)]`, a
persist→rehydrate cycle would lose `raw_diagnostic_cid` and the chain hash
would not re-verify. RSP-0 is in-memory only, so this is acceptable for TB-1.
If you believe this creates a TB-2 footgun, raise it as an OUT-OF-SCOPE
finding, not a Q2 CHALLENGE.)

## Q3 (P0-4). Does `AcceptedLedger::load_from_path` now reject `prev_hash` / entry-`hash` / `logical_t` tampers?

**Codex P0-4 finding (Day-6)**: `load_from_path` called `reconstruct_state`
only; tampers to fields NOT checked by `reconstruct_state` (`prev_hash`,
the entry `hash` field, `logical_t` row-deletion) would load successfully.

**Path A++ patch**: `load_from_path` now calls `verify_chain(0, len)`
BEFORE `reconstruct_state`. Verify:

1. Does the new `load_from_path` body call `verify_chain` first AND propagate
   its `Err` via `?` BEFORE `reconstruct_state` runs?
2. Are there explicit unit tests in `src/economy/ledger.rs::tests` that:
   - Persist a clean chain, mutate ONLY `entries[1].prev_hash`, then assert
     `load_from_path` returns `Err(LedgerError::HashMismatch { at_index: 1 })`;
   - Persist a clean chain, mutate ONLY `entries[0].hash`, then assert
     `load_from_path` returns `Err(LedgerError::HashMismatch { at_index: 0 })`;
   - Persist a 3-entry chain, REMOVE the middle row (so `logical_t=3` ends
     up at index 1), then assert `load_from_path` returns either
     `LogicalTGap { at_index: 1 }` or `HashMismatch { at_index: 1 }`?
3. Does the existing Tier-A `test_p1_kill_1_no_wtool_bypass`
   (`resulting_state_root` mutation on the last entry) STILL pass, since
   `verify_chain` now catches it earlier than `reconstruct_state`?

If all three: **PASS**. Otherwise: **CHALLENGE**.

## Out-of-scope check (file as a separate finding only if applicable)

Did the three patches accidentally:
- Touch a STEP_B-protected file (`src/state/sequencer.rs`, `src/bus.rs`,
  `src/sdk/tools/wallet.rs`)?
- Modify `dispatch_transition` or any production sequencer logic?
- Re-pin a Trust Root manifest entry without the corresponding file change?
- Introduce a NEW `pub` API surface beyond `load_from_path_unverified`-style
  controlled escape hatches?

If any: file as a separate **OUT-OF-SCOPE** finding (not a Q1/Q2/Q3 CHALLENGE).
If none: state explicitly "no out-of-scope drift detected".

---

# Output format

```
## Q1 (P0-2): PASS|CHALLENGE
<one-paragraph reasoning + smallest closing patch if CHALLENGE>

## Q2 (P0-3): PASS|CHALLENGE
<one-paragraph reasoning + smallest closing patch if CHALLENGE>

## Q3 (P0-4): PASS|CHALLENGE
<one-paragraph reasoning + smallest closing patch if CHALLENGE>

## Out-of-scope drift: <none | finding>

## Merged disposition: PASS-ALL-THREE | CHALLENGE-ON-{Q1|Q2|Q3}
```

Be terse. No section headers beyond the four above. No re-introduction of
P0-1. No "but you should also...".
BRIEF_EOF

# Append the three patched files (whole file, not diff — Codex needs context).
append() {
  local label="$1"
  local path="$2"
  printf '\n\n---\n\n## XREF: %s\n\n```rust\n' "$label" >> "$TMP_PROMPT"
  cat "$path" >> "$TMP_PROMPT"
  printf '\n```\n' >> "$TMP_PROMPT"
}

append "tests/tb_1_acceptance.rs (Path A++ Tier-A 10/10; P0-2 closure is test #10)" \
       "${ROOT}/tests/tb_1_acceptance.rs"
append "src/bottom_white/ledger/rejection_evidence.rs (P0-3 closure: serde shield + new test)" \
       "${ROOT}/src/bottom_white/ledger/rejection_evidence.rs"
append "src/economy/ledger.rs (P0-4 closure: verify_chain default + three new tamper tests)" \
       "${ROOT}/src/economy/ledger.rs"

# Append the recharter narrowing block ONLY (so Codex sees the claim
# that's actually being shipped, not the full re-charter doc).
printf '\n\n---\n\n## XREF: TB-1_recharter_2026-04-29.md (claim narrowing — Path A++ amendment + Day-7 ship gate)\n\n```markdown\n' >> "$TMP_PROMPT"
sed -n '1,80p' "${ROOT}/handover/tracer_bullets/TB-1_recharter_2026-04-29.md" >> "$TMP_PROMPT"
printf '\n[...]\n\n' >> "$TMP_PROMPT"
sed -n '/### Day 7 — Ship/,/^### /p' "${ROOT}/handover/tracer_bullets/TB-1_recharter_2026-04-29.md" >> "$TMP_PROMPT"
printf '\n```\n' >> "$TMP_PROMPT"

# Recent commit log (Path A++ patches will be near HEAD).
printf '\n\n---\n\n## XREF: Recent commit log\n\n```\n' >> "$TMP_PROMPT"
git -C "$ROOT" log --pretty=format:'%h %s' 063b003~1..HEAD >> "$TMP_PROMPT"
printf '\n```\n' >> "$TMP_PROMPT"

prompt_size=$(wc -c < "$TMP_PROMPT")
echo "[codex tb1-microaudit] prompt size: ${prompt_size} chars" >&2

t0=$(date +%s)
{
  printf '# Codex TB-1 Path A++ MICRO-AUDIT\n'
  printf '**Date**: 2026-04-29\n'
  printf '**Target**: P0-2 / P0-3 / P0-4 closures (NARROW; no Gemini; no round-2 full audit)\n'
  printf '**HEAD**: %s\n' "$(git -C "$ROOT" rev-parse HEAD)"
  printf '**Prompt size**: %s chars (Day-6 round-1 was 154KB; aim < 20KB)\n' "$prompt_size"
  printf '\n---\n\n'
  codex exec --skip-git-repo-check < "$TMP_PROMPT" 2>&1
} > "$OUT"
t1=$(date +%s)
elapsed=$((t1 - t0))
echo "[codex tb1-microaudit] API returned in ${elapsed}s" >&2
echo "[codex tb1-microaudit] saved: $OUT" >&2
