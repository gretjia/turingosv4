#!/usr/bin/env bash
# Codex TB-13 ship audit — round-6 (after round-7 closure of Codex R5
# PARTIAL-MARKER + DASHBOARD-FLOOR). Class 3 (CompleteSet + MarketSeedTx).

set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
OUT="${ROOT}/handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R6.md"
TMP_PROMPT="$(mktemp /tmp/tb13_codex_ship_r6.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

if [ -f "$OUT" ]; then
  echo "  refusing to overwrite existing $OUT" >&2
  exit 2
fi

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Codex TB-13 Ship Audit — round-6 (post round-7 R5 fix closure)

**Role**: skeptical adversarial implementer-reviewer. Independent of Gemini ship audit. Class 3 (money/collateral surface).

**Round context**: this is **round-6** in audit numbering. The project's round-7 commit `8efffa8` closes the two NEW Q9/RQ6 sub-challenges your R5 audit raised against the round-6 fix:

- **TB13-Q9/RQ6-PARTIAL-MARKER (R5)**: round-6's `tb_13_scan_lines()` short-circuited to marker-spans-only when ANY marker existed. A marker-bearing file could hide non-marker TB-13 type-use plus `f64`/AMM/price tokens outside marker spans.
- **TB13-Q9-DASHBOARD-FLOOR (R5)**: round-6 removed `src/bin/audit_dashboard.rs` from `FENCE_SCOPE_FLOOR` to dodge a Layer 2 false-positive on its negative-list test fixture. But the false-positive was Layer 2-specific; removal also disabled Layer 1 hard-import scanning on that file.

**Mandate**: per memory `feedback_dual_audit_conflict` (VETO > CHALLENGE > PASS) Codex's R5 CHALLENGE outweighed Gemini's R5 PASS. Per memory `feedback_audit_obs_bias`: both qualified for surgical fix, not OBS.

## Round-7 closures (this session) — what to re-verify

```text
8efffa8  TB-13 Atom 6 round-7 — Codex R5 remediation (PARTIAL-MARKER + DASHBOARD-FLOOR)
```

`cargo test --workspace = 794 passed / 0 failed / 150 ignored` (round-6 baseline 792 + 2 new round-7 tests: `tb_13_scan_lines_partial_marker_catches_stealth_type_use` + `audit_dashboard_in_layer_1_scope_but_not_layer_2_scope`).

### PARTIAL-MARKER fix — what to re-verify

`tests/tb_13_legacy_cpmm_forward_fence.rs` `tb_13_scan_lines()` rewritten:

- Marker-files: return marker-spans UNION non-comment lines containing any TB-13 type name (`TB_13_TYPE_NAMES`). Union via `BTreeMap<usize, String>` (deduplicates).
- Unmarked-discovered files: keep round-6 behavior (all non-comment lines).

Re-evaluate the closure:

- Does the union catch your R5 attack (marker-bearing file with stealth `CompleteSetMintTx` + `f64` outside marker spans)? `CompleteSetMintTx` matches TB_13_TYPE_NAMES, line returned, Layer 2 sees `f64`. Confirm.
- Acknowledged residual gap (per code-comment in `tb_13_scan_lines`): a TB-13 helper that uses zero TB-13 type names AND lives outside marker spans. Manual code review remains the fallback. Acceptable, or stronger fix needed?
- Different residual: `f64` on a non-marker non-type-use line in a marker-bearing file (e.g., `let x: f64 = 0.5;` with no TB-13 type name on the line). Round-7 walker would not return it. Acceptable?

### DASHBOARD-FLOOR fix — what to re-verify

Two-tier scope split:

- `effective_fence_scope()` (Layer 1) = `FENCE_SCOPE_FLOOR` ∪ `discover_tb_13_files()`. `audit_dashboard.rs` RESTORED to FLOOR.
- `effective_layer_2_scope()` (NEW) = `discover_tb_13_files()` only. Excludes `audit_dashboard.rs` until it gains TB-13 contributions.
- Layer 2 scans (`legacy_cpm_api_not_imported_by_complete_set` Layer 2 + `no_f64_in_complete_set_or_market_seed`) use `effective_layer_2_scope()`.
- Layer 1 scan (`legacy_cpm_api_not_imported_by_complete_set` Layer 1) uses `effective_fence_scope()`.

Re-evaluate:

- Does Layer 1 still scan `audit_dashboard.rs` for `HARD_BANNED_LEGACY_IMPORTS`? Yes — `effective_fence_scope` includes it via FLOOR. Confirm.
- Does Layer 2 still false-positive on `audit_dashboard.rs:1628-1629`? No — `effective_layer_2_scope` excludes it. Confirm.
- New unit test `audit_dashboard_in_layer_1_scope_but_not_layer_2_scope` asserts the tier-split shape. Sufficient?
- Tradeoff: until `audit_dashboard.rs` gains TB-13 markers or type uses, Layer 2 won't scan it. A contributor could land f64/AMM tokens in `audit_dashboard.rs` without Layer 2 catching them — but TB-13 fence's purpose is "TB-13 contributions don't pull in legacy CPMM"; tokens in non-TB-13-contributing audit_dashboard code are out of TB-13 scope. Is this right?

## Q1-Q8 + Q10-Q13

Round-7 doesn't touch dispatch arms, monetary invariants, share semantics, seed solvency, or smoke evidence. PASS expected. Re-evaluate Q9 (forward-fence) given the R7 changes.

## Specifically scrutinize

**Trust Root**: round-7 modified only `tests/`. No Rust src/ change. No manifest rehash.
**No new pub symbols**: `effective_layer_2_scope` is a private fn in the test file. No R-022 surface.
**Test count drift**: 792 → 794 (+2 from R7).

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
```

Cite file:line for every finding.

Save your audit to: handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R6.md.
BRIEF_EOF

echo "  Codex R6 audit prompt prepared at: $TMP_PROMPT" >&2
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
