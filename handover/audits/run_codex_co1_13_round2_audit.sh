#!/usr/bin/env bash
# Codex round-2 audit on CO1.13 v1.1 (after 9-patch synthesis from r1).
# Focused-verify mode: did v1.1 close all 7 Codex r1 P0s + 2 Gemini P0s?
# Are there NEW defects in v1.1 patches? Final ship-or-escalate decision.
#
# Per Elon-mode 2-round cap: this is the FINAL audit. Per Codex r1 § E:
# if R-022 enforcement still non-functional, escalate to user (no auto-ship).
set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
OUT="${ROOT}/handover/audits/CODEX_CO1_13_ROUND2_AUDIT_2026-04-29.md"
TMP_PROMPT="$(mktemp /tmp/co1_13_codex_round2.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Codex Round-2 Audit — CO1.13 v1.1 (post-9-patch synthesis; FINAL per Elon-mode cap)

**Role**: skeptical adversarial reviewer. Independent of Gemini round-2 (parallel).

**Mandate**: round-2 audit on CO1.13 v1.1 — the patch synthesis after round-1 returned CHALLENGE/CHALLENGE (your Codex r1: 7 P0s; Gemini r1: 2 P0s; conservative merge CHALLENGE per `feedback_dual_audit_conflict`).

**Per Elon-mode 2-round cap**: this is the FINAL audit. Disposition rules:
- r2 = PASS/PASS → spec ships, implementation can begin
- r2 = CHALLENGE on R-022 ENFORCEMENT (gate itself non-functional) → ESCALATE TO USER, do NOT auto-ship-with-OBS (per your r1 § E warning)
- r2 = CHALLENGE on bounded edge cases only → ship-with-OBS allowed per § 0.5 hard-threshold policy (max 3 OBS open at once)

## Round-1 P0 disposition table (verify each closed)

Your round-1 P0s (`CODEX_CO1_13_ROUND1_AUDIT_2026-04-29.md`):

| P0 | Round-1 finding | Author claim closed in v1.1 by |
|---|---|---|
| 1 | R-022 cannot run through engine.py (no external_script support, no --rule CLI) | § 1.2: YAML check.type → custom_commit_hook tombstone; shim calls check_trace_matrix.py DIRECTLY; engine.py 5-line patch ignores trigger==pre_commit |
| 2 | .git/hooks/pre-commit not shippable (untracked local state) | § 1.2: tracked scripts/hooks/pre-commit.r022 + scripts/install_hooks.sh + CI mode --mode ci |
| 3 | Orphan fallback "§ 3" target undefined in matrix | § 1.1 (CO1.13.1 scope): NEW § G "Orphan Extensions" with table schema; § 2.1 fallback target updated |
| 4 | 5-line raw heuristic empirically wrong (86% under semantic block walk vs 69.4% raw) | § 2.1: semantic algorithm (walk /// + #[ + // + blank-line block; stop on non-doc/attr/comment/blank) |
| 5 | Escape hatch silent bypass | § 2.2: commit-message token [R-022-skip: ...] (NOT Rust comment) + structured log + REQUIRED reference (cases/Cxxx/PREREG-§/OBS_R022_*) |
| 6 | Boundary cases unspecified | NEW § 1.3 R-022 Scope Table covers pub fn/struct/enum/trait/const/mod/type/static, pub(crate), pub use exempt, cfg(test) exempt, macro-generated exempt-with-token, signature-modified WARN, backlink-removal BLOCK |
| 7 | Test plan as Rust unit tests for shell/Python/git-hook behavior | § 3: 9 shell integration tests under tests/integration/co1_13/ + 1 Rust orchestrator |

Plus your r1 § C insights (verify codified):
- R-022 also blocks REMOVAL of existing backlinks → § 2.1 + § 2.4 I-REMOVAL invariant + test 3.6 r_022_blocks_backlink_removal.sh
- pub use exempt unless re-export needs separate trace → § 1.3 scope table
- macro-generated → § 1.3 scope table (exempt with skip-token)
- legacy modified WARN not BLOCK → § 1.3 scope table

## Round-2 audit questions (4)

**Q1. Did v1.1 architecturally fix the R-022 gate (P0-1 + P0-2)?**
- Read § 1.2 v1.1 carefully. The shim calls check_trace_matrix.py directly bypassing engine.py. Tracked under scripts/hooks/. CI mode added. Engine.py 5-line patch documented.
- Is this architecturally correct now? Edge case: if developer doesn't run install_hooks.sh, R-022 doesn't fire on their commits — but CI mode catches at PR. Does the dual-layer (local hook + CI) close the gap, or are there bypasses?
- Concrete check: imagine I push a commit with a pub symbol missing backlink, with install_hooks.sh NOT run. Does CI catch it on the PR? If yes, gate is real. If no, gate is theater — R-022 enforcement still non-functional → ESCALATE per your r1 § E.

**Q2. Did v1.1 close P0-4 (5-line → semantic block walk)?**
- Read § 2.1 v1.1 algorithm. Walk back through /// + #[ + // + blank lines.
- Edge case: what if the pub symbol has a #[derive(...)] attribute on the line immediately before, with NO doc-comment? The walk continues through the attribute (matches #[) but finds no /// TRACE_MATRIX. Should that be PASS (attribute-then-pub style is valid Rust idiom) or BLOCK (attribute alone is no constitutional anchor)?
- Edge case: what if the doc-comment block has /// TRACE_MATRIX at distance 50 (like transition_ledger.rs:149 to_signing_payload from your r1 analysis)? The semantic walk allows arbitrary distance as long as the block is contiguous. Is unlimited block depth desired, or should there be a sanity max (e.g., 100 lines)?

**Q3. Did v1.1 close P0-5 (escape hatch silent bypass)?**
- § 2.2 v1.1 requires `[R-022-skip: <reason; cases/Cxxx | PREREG-§n.m | OBS_R022_*.md REQUIRED>]` in commit message. Reference-missing → BLOCK.
- Concrete check: how does check_trace_matrix.py validate that `cases/C-xxx.yaml` actually exists vs being a fabricated reference? Does spec § 2.2 mandate file-existence check on the cited reference?
- Quarterly audit cadence: who runs it? What's the action on flagged abuse?

**Q4. Are there NEW P0s introduced by v1.1 patches?**
- LoC budget jump 415 → 640 LoC. CO1.13.1 doc completion is now 200 LoC delta (was 150). Is the ~33% expansion justified, or is scope creep happening?
- New § G orphan section in TRACE_MATRIX_v3 — is it actually a separate section, or does it conflict with existing § G "Deferred Items Justification" (line 155 of v3 doc)? **VERIFY** by reading the existing matrix.
- 9 shell integration tests is a lot — are any redundant? Could 5-6 cover the same ground?
- Cycle-time target revised 2 day → 3 day. Is 3 day still Elon-mode-aligned, or has Elon-mode itself drifted?

## Verdict format

Section A: Verdict (PASS / CHALLENGE-on-bounded-edge-cases / CHALLENGE-on-R-022-ENFORCEMENT-ESCALATE) with conviction.
Section B: Per-P0 closure assessment (P0-1..P0-7 + Codex § C insights).
Section C: NEW P0s introduced by v1.1 (if any).
Section D: Final disposition recommendation (ship / ship-with-OBS / escalate).
Section E: If escalate: specific user decision needed.

Be concise. Cite spec § + line.
BRIEF_EOF

# Append v1.1 spec + r1 verdicts (context)
printf '\n\n---\n\n# XREF: CO1.13 v1.1 spec (THIS IS THE TARGET)\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/specs/CO1_13_TRACE_MATRIX_IMPL_v1_2026-04-29.md" >> "$TMP_PROMPT"

printf '\n\n---\n\n# XREF: Your round-1 verdict (Codex r1)\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/audits/CODEX_CO1_13_ROUND1_AUDIT_2026-04-29.md" >> "$TMP_PROMPT"

printf '\n\n---\n\n# XREF: Gemini round-1 verdict (parallel; for context)\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/audits/GEMINI_CO1_13_ROUND1_AUDIT_2026-04-29.md" >> "$TMP_PROMPT"

printf '\n\n---\n\n# XREF: TRACE_MATRIX_v3 (verify § G is a real section, not conflicting)\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/alignment/TRACE_MATRIX_v3_2026-04-27.md" >> "$TMP_PROMPT"

printf '\n\n---\n\nNow give your INDEPENDENT round-2 verdict.\n' >> "$TMP_PROMPT"

prompt_size=$(wc -c < "$TMP_PROMPT")
echo "[codex co1.13 r2] prompt size: ${prompt_size} chars" >&2

t0=$(date +%s)
{
  printf '# Codex CO1.13 Round-2 Audit\n'
  printf '**Date**: 2026-04-29\n'
  printf '**Target**: spec v1.1 (post-r1-9-patch-synthesis; FINAL per Elon-mode 2-round-cap)\n'
  printf '**HEAD**: %s\n' "$(git -C "$ROOT" rev-parse HEAD)"
  printf '**Prompt size**: %s chars\n\n---\n\n' "$prompt_size"
  codex exec --skip-git-repo-check < "$TMP_PROMPT" 2>&1
} > "$OUT"
t1=$(date +%s)
elapsed=$((t1 - t0))
echo "[codex co1.13 r2] API returned in ${elapsed}s" >&2
echo "[codex co1.13 r2] saved: $OUT" >&2
