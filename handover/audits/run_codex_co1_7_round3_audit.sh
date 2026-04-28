#!/usr/bin/env bash
# Codex round-3 audit on CO1.7 v1.2 — narrow closure check on the 3 round-2 must-fix.
set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
OUT="${ROOT}/handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md"
TMP_PROMPT="$(mktemp /tmp/co1_7_codex_round3.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Codex Round-3 Audit — CO1.7 transition_ledger v1.2 (narrow closure check)

**Role**: skeptical adversarial implementer-reviewer. Independent of Gemini round-3.

**Mandate**: round-3 closure-only check. v1.2 claims to close the 3 specific must-fix items from your round-2 CHALLENGE plus 1 typo. PASS/PASS unblocks CO1.7 implementation.

Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.

## Your round-2 verdict (the items v1.2 must close)

You returned **CHALLENGE** with 3 specific must-fix items:
1. **R2-C3**: actually close C3 in code or stop claiming it is closed. Add `CanonicalMessage::LedgerEntrySigning`, `canonical_digest` arm, and `sign_ledger_entry` to `system_keypair.rs`. Plus skeleton test verifying signature rejection after `parent_ledger_root` mutation.
2. **R2-K3**: fix the head_t/commit return contradiction. Either make `LedgerWriter::commit` return commit `NodeId`, or explicitly defer `head_t` mutation and remove the "CO1.7 owns head_t" claim from spec § 0/§ 3/§ 5.
3. **R2-C2-CAS**: fix the CAS object type mismatch. Add `ObjectType::Transition` to schema.rs, OR change spec § 3 to use an existing CAS `ObjectType`.

You also flagged: typo "8-field" → should be "9-field" in spec.

## What v1.2 ships (per patch log at top of spec)

| Item | v1.2 fix |
|---|---|
| R2-C3 | Wave 4-B additive extension shipped: `CanonicalMessage::LedgerEntrySigning([u8;32])` opaque-digest variant + canonical_digest match arm + new `mod transition_ledger_emitter::sign_ledger_entry`. Skeleton test 9 (`signature_round_trip_and_transplant_defense`) generates real Ed25519 keypair, signs via emitter, verifies; asserts clean verify + transplant fail (K2) + cross-epoch fail (D1). |
| R2-K3 | head_t mutation explicitly deferred to CO1.7.5+ when `Git2LedgerWriter` exists. v1.x ledger owns `ledger_root_t` only. `LedgerWriter::commit` keeps `Hash` return. Spec § 0/§ 3/§ 5 updated; "CO1.7 owns head_t" claim removed. |
| R2-C2-CAS | Spec § 3 changed `ObjectType::Transition` → `ObjectType::ProposalPayload` (existing variant — semantically correct for agent work_tx payloads; no CAS schema extension). |
| R2-typo | Spec § 0 / § 1.1 "8-field" → "9-field". |

## Round-3 closure-only questions

For each of your round-2 must-fix items, judge: **CLOSED / PARTIAL / REGRESSED / NEW-ISSUE**.

**Q-1 (R2-C3 closure)**: verify in code:
- `src/bottom_white/ledger/system_keypair.rs` has `CanonicalMessage::LedgerEntrySigning([u8; 32])` variant
- `canonical_digest()` has match arm `b"LedgerEntrySigning" + digest`
- `pub(crate) mod transition_ledger_emitter` exposes `sign_ledger_entry(keypair, signing_payload_digest)` returning `Result<SystemSignature, KeypairError>`
- Skeleton test 9 (`signature_round_trip_and_transplant_defense`) actually exercises sign + verify roundtrip and the K2/D1 defenses

**Q-2 (R2-K3 closure)**: verify in spec:
- § 0 no longer claims "CO1.7 owns head_t = NodeId(commit_sha)"
- § 3 sequencer pseudocode explicitly does NOT mutate q_w.head_t (deferral comment present)
- § 5 storage backend says head_t deferred to CO1.7.5+
- LedgerWriter trait keeps `Hash` return (not `NodeId`)

**Q-3 (R2-C2-CAS closure)**: verify in spec § 3 sequencer:
- `cas.put(...)` call uses `ObjectType::ProposalPayload` (NOT `ObjectType::Transition`)
- Comment notes the R2 fix

**Q-4 (R2-typo closure)**: verify spec § 0 says "9-field" not "8-field".

**Q-5 (NEW issues from v1.2 patches)**: did v1.2 introduce any new defects?
- Opaque-digest variant in CanonicalMessage — does it correctly preserve typed-sign-only invariant?
- transition_ledger_emitter module is `pub(crate) mod` — is the visibility appropriate?
- Skeleton test cross-epoch defense uses `payload.epoch = SystemEpoch::new(2)` then computes new digest — does this correctly model the threat?
- TR manifest hashes refreshed for system_keypair.rs + transition_ledger.rs + spec — anything else needed?

**Q-6**: independent verification —  `cargo test --lib` reports 199/0 PASS (was 198 pre-v1.2; +1 new test). Verify the structural correctness of test #9.

## Output format

# Codex CO1.7 Round-3 Audit
## Q-1 R2-C3 closure
## Q-2 R2-K3 closure
## Q-3 R2-C2-CAS closure
## Q-4 R2-typo closure
## Q-5 NEW v1.2 issues
## Q-6 Independent verification
## **VERDICT**: PASS / CHALLENGE / VETO
## Top must-fix (if CHALLENGE)
## Conviction (low/med/high)

You MUST PASS if v1.2 closes round-2 blockers without new ones. Round-3 is a closure check, NOT a fresh exploration.

If CHALLENGE: list exactly what to patch in v1.3.

---

BRIEF_EOF

echo "" >> "$TMP_PROMPT"
echo "# CO1.7 spec v1.2 (current target of round-3)" >> "$TMP_PROMPT"
echo "" >> "$TMP_PROMPT"
cat "${ROOT}/handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# Type-skeleton v1.2 (current target of round-3)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/bottom_white/ledger/transition_ledger.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# system_keypair.rs (v1.2 with CanonicalMessage extension + transition_ledger_emitter — verify R2-C3)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/bottom_white/ledger/system_keypair.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# Your round-2 audit (the must-fix items v1.2 must close)\n\n' >> "$TMP_PROMPT"
awk '/^tokens used/{exit} {print}' "${ROOT}/handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# CAS schema (verify R2-C2-CAS — ObjectType::ProposalPayload exists)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/bottom_white/cas/schema.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\nNow give your INDEPENDENT round-3 closure check.\n' >> "$TMP_PROMPT"

prompt_size=$(wc -c < "$TMP_PROMPT")
echo "[codex co1.7 r3] prompt size: ${prompt_size} chars" >&2

t0=$(date +%s)
{
  printf '# Codex CO1.7 transition_ledger Round-3 Audit\n'
  printf '**Date**: 2026-04-28\n'
  printf '**Target**: spec v1.2 + skeleton v1.2 + system_keypair.rs (closure check)\n'
  printf '**Prompt size**: %s chars\n\n---\n\n' "$prompt_size"
  codex exec --skip-git-repo-check < "$TMP_PROMPT" 2>&1
} > "$OUT"
t1=$(date +%s)
elapsed=$((t1 - t0))
echo "[codex co1.7 r3] API returned in ${elapsed}s" >&2
echo "[codex co1.7 r3] saved: $OUT" >&2
