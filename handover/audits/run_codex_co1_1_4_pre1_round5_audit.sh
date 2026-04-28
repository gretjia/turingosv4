#!/usr/bin/env bash
# Codex round-5 narrow audit on CO1.1.4-pre1 v1.2.2 — final doc-hygiene closure check.
set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
OUT="${ROOT}/handover/audits/CODEX_CO1_1_4_PRE1_ROUND5_AUDIT_2026-04-28.md"
TMP_PROMPT="$(mktemp /tmp/co1_1_4_pre1_codex_round5.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Codex Round-5 Narrow Audit — CO1.1.4-pre1 v1.2.2 (final doc-hygiene closure)

**Role**: skeptical reviewer (round-5 final doc-hygiene closure). Round-3 PASSed code; round-4 caught 1 stale doc + you (R4) flagged `system_keypair.rs:229-231`. v1.2.2 closes that + 2 preemptive WorkTx + TerminalSummaryTx doc-comment cleanups.

Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS. Round 1+2+3+4 cumulative audit cost ~$26-48; this round-5 is the final doc-hygiene gate.

## What changed v1.2.1 → v1.2.2

| Your r4 must-fix | v1.2.2 fix |
|---|---|
| `src/bottom_white/ledger/system_keypair.rs:229-231` said "full canonical_digest of the 8-field TerminalSummaryTx" — wrong API path | Updated to reference `TerminalSummarySigningPayload::canonical_digest()` (a/k/a `TerminalSummaryTx::to_signing_payload().canonical_digest()`) |
| (preemptive) `src/state/typed_tx.rs:218` WorkTx doc said signature is over `canonical_digest(&work_tx)` — incorrect since v1.1 P1 introduced WorkSigningPayload | Updated: signature is over `WorkSigningPayload::canonical_digest()` with domain prefix |
| (preemptive) `src/state/typed_tx.rs:336` TerminalSummaryTx doc said "canonical_digest is computed here" — vague | Updated: explicit API path `TerminalSummaryTx::to_signing_payload().canonical_digest()` + domain prefix `b"turingosv4.system_sig.terminal_summary.v1"` |

## Round-5 narrow questions

**Q1**: Is `system_keypair.rs:229-231` (CanonicalMessage::TerminalSummarySigning doc) now correct?

**Q2**: Is `typed_tx.rs:218` (WorkTx struct doc) now correct?

**Q3**: Is `typed_tx.rs:336` (TerminalSummaryTx struct doc) now correct?

**Q4**: Are there any OTHER stale doc-comment references this round missed? Grep both files for any "TerminalSummaryTx in system_keypair" / "imported from system_keypair" / "canonical_digest of work_tx" / "canonical_digest of TerminalSummaryTx" residue.

**Q5**: cargo test --lib still passes (224/0)?

## Output format

# Codex CO1.1.4-pre1 Round-5 Audit
## Q1 system_keypair.rs:229
## Q2 typed_tx.rs:218
## Q3 typed_tx.rs:336
## Q4 Other stale residue
## Q5 Test status
## **VERDICT**: PASS / CHALLENGE / VETO
## Conviction (low/med/high)

This is round-5 final doc-hygiene gate. PASS = all stale references closed; CHALLENGE = at least one stale residue still present.

---

BRIEF_EOF

echo "" >> "$TMP_PROMPT"
printf '\n# Round-4 verdict (must-fix list this round verifies)\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/audits/CODEX_CO1_1_4_PRE1_ROUND4_AUDIT_2026-04-28.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# Current src/state/typed_tx.rs (v1.2.2)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/state/typed_tx.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# Current src/bottom_white/ledger/system_keypair.rs (v1.2.2)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/bottom_white/ledger/system_keypair.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\nNow give your INDEPENDENT round-5 narrow audit.\n' >> "$TMP_PROMPT"

prompt_size=$(wc -c < "$TMP_PROMPT")
echo "[codex co1.1.4-pre1 r5] prompt size: ${prompt_size} chars" >&2

t0=$(date +%s)
{
  printf '# Codex CO1.1.4-pre1 Round-5 Narrow Audit\n'
  printf '**Date**: 2026-04-28\n'
  printf '**Target**: v1.2.2 final doc-hygiene closure\n'
  printf '**Prompt size**: %s chars\n\n---\n\n' "$prompt_size"
  codex exec --skip-git-repo-check < "$TMP_PROMPT" 2>&1
} > "$OUT"
t1=$(date +%s)
elapsed=$((t1 - t0))
echo "[codex co1.1.4-pre1 r5] API returned in ${elapsed}s" >&2
echo "[codex co1.1.4-pre1 r5] saved: $OUT" >&2
