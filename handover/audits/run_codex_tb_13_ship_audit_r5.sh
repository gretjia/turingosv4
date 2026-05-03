#!/usr/bin/env bash
# Codex TB-13 ship audit — round-5 (after round-6 closure of Codex R4 Q9/RQ6 + RQ3).
# Class 3 (CompleteSet + MarketSeedTx). Independent of Gemini ship audit.
# Per memory feedback_dual_audit_conflict: VETO > CHALLENGE > PASS.

set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
OUT="${ROOT}/handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R5.md"
TMP_PROMPT="$(mktemp /tmp/tb13_codex_ship_r5.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

if [ -f "$OUT" ]; then
  echo "  refusing to overwrite existing $OUT" >&2
  exit 2
fi

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Codex TB-13 Ship Audit — round-5 (post round-6 R4 fix closure)

**Role**: skeptical adversarial implementer-reviewer. Independent of Gemini ship audit. Class 3 (money/collateral surface).

**Round context**: this is **round-5** in audit numbering. The project's round-6 commit `d3473bb` closes the two NEW challenges your R4 audit raised against the round-5 fixes:

- **TB13-Q9/RQ6 (R4)**: type-use discovery added unmarked TB-13 files to scope, but Layer 2 (FORBIDDEN_LEGACY_TOKENS + f64) still walked marker-scoped `tb_13_spans()`. An unmarked file importing `CompleteSetMintTx` and using `f64` would be discovered into scope but never scanned by Layer 2.
- **TB13-RQ3 (R4)**: round-5 README + test claimed `final_state_root_hex == hex(live state_root_t)` was "cryptographic proof of map equality." But state-root mutators hash `domain || prev_root || canonical_tx`, NOT the full QState.

**Mandate**: per memory `feedback_dual_audit_conflict` (VETO > CHALLENGE > PASS) Codex's R4 CHALLENGE outweighed Gemini's R4 PASS. Per memory `feedback_audit_obs_bias` (this session): both qualified for surgical fix, not OBS. Round cap = 2 per `feedback_elon_mode_policy` (this session); cap-exception via auto-execute on determinate-best surgical patch was invoked.

## Round-6 closures (this session) — what to re-verify

```text
887537f  TB-13 Atom 6 round-5 audit artifacts (R3 rename + R4 audit run)
d3473bb  TB-13 Atom 6 round-6 — Codex R4 remediation (Q9/RQ6 Layer 2 + RQ3 direct map equality)
```

`cargo test --workspace = 792 passed / 0 failed / 150 ignored` (round-5 baseline 791 + 1 new `tb_13_scan_lines_handles_marker_and_unmarked_files`).

### Q9/RQ6 (R4) fix — what to re-verify

`tests/tb_13_legacy_cpmm_forward_fence.rs`:
- New `tb_13_scan_lines(source: &str) -> Vec<(usize, String)>` helper. If the source contains any TB-13 authoring marker → return `tb_13_spans()` (preserves doc-xref skip). Otherwise → return all non-comment lines (caught the `f64` / AMM tokens that round-5's marker-scoped Layer 2 missed).
- `legacy_cpm_api_not_imported_by_complete_set` Layer 2 now uses `tb_13_scan_lines()` (was `tb_13_spans`).
- `no_f64_in_complete_set_or_market_seed` now uses `tb_13_scan_lines()`.
- `src/bin/audit_dashboard.rs` removed from `FENCE_SCOPE_FLOOR` because (a) it has 0 TB-13 markers + 0 TB-13 type uses today, (b) Atom 4 §13 dashboard is DEFERRED to TB-14, (c) the new walker would have false-positived on its negative-list test fixture (`"price_yes" / "price_no"` string literals at line 1628-1629 inside an audit dashboard test that bans those tokens). Will be auto-rediscovered by `discover_by_marker` when TB-14 ships dashboard contributions.
- New unit test `tb_13_scan_lines_handles_marker_and_unmarked_files` covers both branches: marker-file → spans + skips xref; unmarked-file → all non-comment lines + filters pure comments.

Re-evaluate: is this fix complete? Specifically:

- Does `tb_13_scan_lines` correctly classify a partially-marked file (one with a single TB-13 marker but TB-12 contributions in non-marker spans)? If a file has any TB-13 marker, the helper returns `tb_13_spans()` only (which catches only marker-spans). That means TB-12 lines outside any TB-13 span are NOT scanned. Is that correct, or should partially-marked files be treated as fully-in-scope? My read: marker-only-spans is the correct behavior (we don't want to false-positive on TB-12 baseline code that happens to share a file with TB-13 additions). Confirm.
- Are there any other places in `tests/tb_13_legacy_cpmm_forward_fence.rs` that still use `tb_13_spans()` directly and could fail to scan unmarked-discovered files? (You should search for all `tb_13_spans` callers.)
- Is removing `audit_dashboard.rs` from FLOOR safe in adversarial attack: e.g., a contributor adds TB-13 contributions to audit_dashboard.rs WITHOUT any marker AND without any of the TB_13_TYPE_NAMES — they would not be discovered by either path. Is that a real risk?

### RQ3 (R4) fix — what to re-verify

`tests/tb_13_chaintape_smoke.rs`:
- New `manual_replay_from_disk(runtime_repo_path, cas_path) -> QState` helper that mirrors `verify_chaintape`'s internal step 6: opens `Git2LedgerWriter`, reads all entries, loads `initial_q_state.json`, decodes `pinned_pubkeys.json` into `PinnedSystemPubkeys`, opens `CasStore`, calls `replay_full_transition` (pub API in `src/bottom_white/ledger/transition_ledger.rs:434`).
- The smoke now asserts (after the existing `verify_chaintape` + state-root-hex match):
  - `replayed_q.economic_state_t.conditional_collateral_t == live_q.economic_state_t.conditional_collateral_t` (byte-equal map reconstruction).
  - `replayed_q.economic_state_t.conditional_share_balances_t == live_q.economic_state_t.conditional_share_balances_t` (byte-equal map reconstruction).
  - `replayed_q.economic_state_t == live_q.economic_state_t` (full economic_state_t equality, belt-and-suspenders).
- Module docstring + README revised to claim the correct property: state-root match proves chain determinism; direct map equality proves byte-equal reconstruction.

Re-evaluate: is the strengthened evidence airtight?

- Is `replay_full_transition` (pub API) sound to call from a test? It's used in production by `verify_chaintape` internally; calling it twice (once via verify_chaintape Gate 4 + 6, once manually) on the same artifacts should be idempotent.
- Could the `replayed_q` returned by `replay_full_transition` differ from the internal QState that `verify_chaintape` computed? Both run the same dispatcher over the same entries; they should be bit-equal.
- Are there any subtle non-determinism risks in `dispatch_transition` (BTreeMap iteration order during canonical-encode, MicroCoin internals, hashmap iteration if any) that could make replay differ from live?
- Is the manual replay correctly assembling the inputs (initial_q from `initial_q_state.json`, pinned_pubkeys decoded from `pinned_pubkeys.json` hex, CasStore opened at `cas_path`)? Double-check the hex decoding loop at the helper.

## The 9 mandated audit questions

Re-evaluate Q1-Q9 against round-6 HEAD. Most of these were PASS in your R4 read; this round is mainly about re-verifying the two new fix surfaces don't introduce regressions.

**Q1-Q8**: Round-6 doesn't touch dispatch arms, monetary invariants, share semantics, or seed solvency. PASS expected.

**Q9 (forward-fence)**: Round-6 strengthens the Layer 2 coverage. Re-evaluate.

## Specifically scrutinize

**Trust Root**: round-6 modified only `tests/` and `handover/`. No Rust src/ change. Trust Root manifest unchanged. Confirm no rehash needed.

**No new pub symbols**: round-6 added `tb_13_scan_lines`, `manual_replay_from_disk` — these are private (test-only). Confirm no R-022 surface.

**Test count drift**: round-5 baseline 791 → round-6 baseline 792 (+1 = `tb_13_scan_lines_handles_marker_and_unmarked_files`). Confirm.

## Verdict format

```text
## VERDICT: PASS
- conviction: low/medium/high
- recommendation: PROCEED to SHIP
```

```text
## VERDICT: CHALLENGE
- <claim id> CHALLENGE: <one-line reason + line refs>
- conviction: low/medium/high
- recommendation: FIX-THEN-PROCEED  (if cheap fix exists) or PROCEED-WITH-OBS (if architecturally deferred)
```

```text
## VERDICT: VETO
- <claim id> VETO: <one-line BLOCKING reason + line refs>
- conviction: low/medium/high
(VETO blocks ship per feedback_dual_audit_conflict; VETO > CHALLENGE > PASS.)
```

Cite file:line for every finding.

Save your audit to: handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R5.md.
BRIEF_EOF

echo "  Codex R5 audit prompt prepared at: $TMP_PROMPT" >&2
echo "  Output target: $OUT" >&2
echo "  Invoking codex exec..." >&2

cat "$TMP_PROMPT" | codex exec --skip-git-repo-check --sandbox read-only --color never - > "$OUT.raw" 2>&1
EXIT=$?

if [ $EXIT -ne 0 ]; then
  echo "  codex exec returned exit code $EXIT" >&2
  echo "  partial output saved to $OUT.raw" >&2
fi

mv "$OUT.raw" "$OUT"
echo "  Audit saved: $OUT" >&2
exit $EXIT
