#!/usr/bin/env bash
# Codex round-3 narrow audit — verifies the 2 missing tests from r2 are now present.
set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
OUT="${ROOT}/handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND3_AUDIT_2026-04-28.md"
TMP_PROMPT="$(mktemp /tmp/co1_7_impl_codex_round3.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Codex Round-3 Narrow Audit — CO1.7-impl Bundle v1.1.1

Round-2 verdict: CHALLENGE/high. Q1 P1, Q2 P2, Q5, Q6 all PASS. CHALLENGE only because Q3+Q4 found the 2 claimed tests (replay_rejects_tx_kind_mismatch + replay_rejects_payload_decode_failure) MISSING from src despite v1.1 commit message claiming them.

v1.1.1 (commit `1bc8887`) adds both tests.

## Round-3 narrow questions

**Q1**: Does `replay_rejects_tx_kind_mismatch` test exist in `src/bottom_white/ledger/transition_ledger.rs`? Does it construct a re-signed envelope claiming a different tx_kind than the CAS payload? Does `cargo test --lib` show it passing?

**Q2**: Does `replay_rejects_payload_decode_failure` test exist? Does it put non-canonical bytes into CAS and assert PayloadDecode error? Does it pass?

**Q3**: cargo test --lib full count = 239/0 + 1 ignored (was 237/0 + 1 ignored; +2 new)?

**Q4**: Any OTHER stale claim-vs-code drift? Specifically grep the v1.1 commit message + verdict claims against actual code state.

## Output format

# Codex CO1.7-impl Bundle Round-3 Audit
## Q1 replay_rejects_tx_kind_mismatch
## Q2 replay_rejects_payload_decode_failure
## Q3 Test count
## Q4 Other drift
## **VERDICT**: PASS / CHALLENGE / VETO
## Conviction (low/med/high)

Be terse. Cite line numbers.

---

BRIEF_EOF

echo "" >> "$TMP_PROMPT"
printf '\n# Round-2 verdict (must-fix list this round verifies)\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# Current src/bottom_white/ledger/transition_ledger.rs (v1.1.1)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/bottom_white/ledger/transition_ledger.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\nNow give your INDEPENDENT round-3 narrow audit.\n' >> "$TMP_PROMPT"

prompt_size=$(wc -c < "$TMP_PROMPT")
echo "[codex co1.7-impl bundle r3] prompt size: ${prompt_size} chars" >&2

t0=$(date +%s)
{
  printf '# Codex CO1.7-impl Bundle Round-3 Narrow Audit\n'
  printf '**Date**: 2026-04-28\n'
  printf '**Target**: v1.1.1 — 2 missing tests added\n'
  printf '**Prompt size**: %s chars\n\n---\n\n' "$prompt_size"
  codex exec --skip-git-repo-check < "$TMP_PROMPT" 2>&1
} > "$OUT"
t1=$(date +%s)
elapsed=$((t1 - t0))
echo "[codex co1.7-impl bundle r3] API returned in ${elapsed}s" >&2
echo "[codex co1.7-impl bundle r3] saved: $OUT" >&2
