#!/usr/bin/env bash
# Codex round-4 narrow audit on CO1.1.4-pre1 v1.2.1 — verifies the 2 doc-only
# fixes from round-3 CHALLENGE. Gemini r3 already PASSed v1.2; r4 is Codex-only.
set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
OUT="${ROOT}/handover/audits/CODEX_CO1_1_4_PRE1_ROUND4_AUDIT_2026-04-28.md"
TMP_PROMPT="$(mktemp /tmp/co1_1_4_pre1_codex_round4.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Codex Round-4 Narrow Audit — CO1.1.4-pre1 v1.2.1 (doc-hygiene closure)

**Role**: skeptical reviewer (round-4 narrow-scope closure check). Round-3 returned PASS (Gemini) + CHALLENGE (Codex, 2 doc-hygiene items only). Conservative merged CHALLENGE per memory `feedback_dual_audit_conflict`. v1.2.1 (commit `33e75b8`) closes the 2 stale doc-comment references.

Per CLAUDE.md "Audit Standard" + memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.

## What changed v1.2 → v1.2.1 (your r3 must-fix items)

| Your r3 must-fix | v1.2.1 fix |
|---|---|
| `src/state/typed_tx.rs:601` doc-comment said "TerminalSummaryTx is imported from system_keypair.rs (already shipped)" — stale since v1.1 P3 migration | Updated: "All variants are defined in this module (`state::typed_tx`); v1.1 P3 migrated `TerminalSummaryTx` here from a 3-field placeholder previously in `system_keypair.rs`." |
| `src/bottom_white/ledger/system_keypair.rs:570` doc-comment said `TerminalSummaryTx::canonical_digest()` — wrong path | Updated: `TerminalSummaryTx::to_signing_payload().canonical_digest()` + the parallel paths via `FinalizeRewardSigningPayload` / `TaskExpireSigningPayload` |

## Round-4 narrow questions

**Q1**: Does `src/state/typed_tx.rs` near the `pub enum TypedTx` declaration now read correctly (no claim of import from `system_keypair`)?

**Q2**: Does `src/bottom_white/ledger/system_keypair.rs` near the `terminal_summary_emitter` mod doc-comment now reference the correct API path?

**Q3**: Are there any OTHER stale references the v1.2.1 patch missed? Grep both files for any remaining "TerminalSummaryTx in system_keypair" / "imported from system_keypair" / "canonical_digest" pointing at the wrong type.

**Q4**: cargo test --lib still passes (224/0 expected)?

## Output format

# Codex CO1.1.4-pre1 Round-4 Audit
## Q1 typed_tx.rs:601 doc-comment
## Q2 system_keypair.rs:570 doc-comment
## Q3 Other stale references
## Q4 Test status
## **VERDICT**: PASS / CHALLENGE / VETO
## Conviction (low/med/high)

This is a narrow round-4. PASS = both doc-comments fixed + no other stale residue. CHALLENGE = stale residue still present. VETO = unlikely.

---

BRIEF_EOF

echo "" >> "$TMP_PROMPT"
printf '\n# Round-3 Codex verdict (the must-fix list this round verifies)\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/audits/CODEX_CO1_1_4_PRE1_ROUND3_AUDIT_2026-04-28.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# Current src/state/typed_tx.rs (v1.2.1)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/state/typed_tx.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# Current src/bottom_white/ledger/system_keypair.rs (v1.2.1)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/bottom_white/ledger/system_keypair.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\nNow give your INDEPENDENT round-4 narrow audit.\n' >> "$TMP_PROMPT"

prompt_size=$(wc -c < "$TMP_PROMPT")
echo "[codex co1.1.4-pre1 r4] prompt size: ${prompt_size} chars" >&2

t0=$(date +%s)
{
  printf '# Codex CO1.1.4-pre1 Round-4 Narrow Audit\n'
  printf '**Date**: 2026-04-28\n'
  printf '**Target**: v1.2.1 doc-hygiene closure (round-3 must-fix)\n'
  printf '**Prompt size**: %s chars\n\n---\n\n' "$prompt_size"
  codex exec --skip-git-repo-check < "$TMP_PROMPT" 2>&1
} > "$OUT"
t1=$(date +%s)
elapsed=$((t1 - t0))
echo "[codex co1.1.4-pre1 r4] API returned in ${elapsed}s" >&2
echo "[codex co1.1.4-pre1 r4] saved: $OUT" >&2
