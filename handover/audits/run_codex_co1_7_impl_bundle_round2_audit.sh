#!/usr/bin/env bash
# Codex round-2 narrow audit on CO1.7-impl bundle v1.1 — closure check on R1 must-fix items.
set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
OUT="${ROOT}/handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md"
TMP_PROMPT="$(mktemp /tmp/co1_7_impl_codex_round2.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Codex Round-2 Narrow Audit — CO1.7-impl Bundle v1.1 (closure check)

**Role**: skeptical reviewer (round-2 narrow-scope closure). Round-1 returned CHALLENGE/CHALLENGE; v1.1 (commit `1a921e5`) closes 4 patches addressing Codex's R1 must-fix list.

Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.

## What changed v1.0 → v1.1 (your r1 must-fix items)

| Your r1 must-fix | v1.1 patch | Cite to verify |
|---|---|---|
| C-1: replay_full_transition accepts only roots; fabricates QState::genesis() | P1: signature changed to `replay_full_transition(genesis: &QState, ...) -> Result<QState, ReplayError>`; caller provides full QState; replay returns reconstructed final QState | transition_ledger.rs replay_full_transition signature + caller updates |
| C-2: K1 violated by infra failure — fetch_add before sign+commit | P2: apply_one uses `next_logical_t.load(SeqCst) + 1` for tentative; `next_logical_t.store(logical_t, SeqCst)` happens ONLY AFTER writer.commit succeeds; doc-comment justifies single-writer assumption | sequencer.rs apply_one stage 4-9 |
| C-3: replay never asserts tx_kind match | P3: NEW ReplayError::TxKindMismatch variant + stage 6.5 assertion + test replay_rejects_tx_kind_mismatch | transition_ledger.rs ReplayError enum + replay_full_transition stage 6.5 + tests |
| C-3-secondary: decode errors reported as CasMissing | P4: NEW ReplayError::PayloadDecode { at, reason } variant; replay stage 6 distinguishes decode from CAS-miss; test replay_rejects_payload_decode_failure | transition_ledger.rs ReplayError + stage 6 |

## Round-2 narrow questions

**Q1**: Does P1 (replay signature) actually return `Result<QState, ReplayError>` instead of two-roots-tuple? Does it correctly use `genesis: &QState` for state-root + ledger-root + downstream consumer fields?

**Q2**: Does P2 (logical_t ordering) correctly defer `next_logical_t.store(...)` to AFTER `writer.commit(&entry)?`? On commit Err, is fetch_add NOT called (preserving K1 under infra failure)?

**Q3**: Does P3 (tx_kind match) correctly reject the case where envelope claims one kind but CAS payload decodes as another? Test exercises the case with a re-signed tampered envelope?

**Q4**: Does P4 (PayloadDecode) cleanly separate decode failure from CAS miss? Test exercises decode failure with non-canonical bytes?

**Q5**: cargo test --lib still passes (237/0 + 1 ignored expected)?

**Q6**: Any OTHER stale references / patches missed? Specifically grep both files for "fetch_add(1, Ordering::SeqCst)" / "Result<(Hash, Hash)" / "genesis_state_root, genesis_ledger_root" residue.

## Output format

# Codex CO1.7-impl Bundle Round-2 Audit
## Q1 P1 replay signature
## Q2 P2 logical_t ordering
## Q3 P3 tx_kind match
## Q4 P4 PayloadDecode separation
## Q5 Test status
## Q6 Other stale residue
## **VERDICT**: PASS / CHALLENGE / VETO
## Top must-fix (if CHALLENGE)
## Conviction (low/med/high)

Be terse. Cite line numbers.

---

BRIEF_EOF

echo "" >> "$TMP_PROMPT"
printf '\n# Round-1 verdict (must-fix list this round verifies)\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/audits/CO1_7_IMPL_BUNDLE_DUAL_AUDIT_VERDICT_R1_2026-04-28.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# Current src/bottom_white/ledger/transition_ledger.rs (v1.1)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/bottom_white/ledger/transition_ledger.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# Current src/state/sequencer.rs (v1.1)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/state/sequencer.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\nNow give your INDEPENDENT round-2 narrow audit.\n' >> "$TMP_PROMPT"

prompt_size=$(wc -c < "$TMP_PROMPT")
echo "[codex co1.7-impl bundle r2] prompt size: ${prompt_size} chars" >&2

t0=$(date +%s)
{
  printf '# Codex CO1.7-impl Bundle Round-2 Narrow Audit\n'
  printf '**Date**: 2026-04-28\n'
  printf '**Target**: v1.1 closure (P1-P4)\n'
  printf '**Prompt size**: %s chars\n\n---\n\n' "$prompt_size"
  codex exec --skip-git-repo-check < "$TMP_PROMPT" 2>&1
} > "$OUT"
t1=$(date +%s)
elapsed=$((t1 - t0))
echo "[codex co1.7-impl bundle r2] API returned in ${elapsed}s" >&2
echo "[codex co1.7-impl bundle r2] saved: $OUT" >&2
