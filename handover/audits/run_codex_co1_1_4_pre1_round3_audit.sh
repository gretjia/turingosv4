#!/usr/bin/env bash
# Codex round-3 audit on CO1.1.4-pre1 v1.2 — narrow closure check on R2 must-fix items.
set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
OUT="${ROOT}/handover/audits/CODEX_CO1_1_4_PRE1_ROUND3_AUDIT_2026-04-28.md"
TMP_PROMPT="$(mktemp /tmp/co1_1_4_pre1_codex_round3.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Codex Round-3 Audit — CO1.1.4-pre1 v1.2 (narrow closure check)

**Role**: skeptical adversarial reviewer (round-3 narrow-scope closure check). Independent of Gemini round-3.

**Mandate**: round-3 closure verification on v1.2 (commit `f4649a9`). Round-2 returned CHALLENGE/PASS → conservative CHALLENGE. v1.2 claims to close 4 must-fix + 1 secondary + 3 Gemini recommendations.

Per CLAUDE.md "Audit Standard" + memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.

## What changed v1.1 → v1.2

| ID | v1.1 issue (round-2 finding) | v1.2 fix claim | Cite line/section to verify |
|---|---|---|---|
| **P11** | `SignalKind::Finalize.claim_id: TxId` leaked old type (P2 missed call site) | claim_id: ClaimId; SignalBundle::finalize signature updated | typed_tx.rs SignalKind::Finalize + SignalBundle::finalize impl |
| **P12** | No FinalizeRewardSigning / TaskExpireSigning CanonicalMessage variants → dual-sign path not executable for 2 of 3 system txs | NEW CanonicalMessage::FinalizeRewardSigning([u8;32]) + TaskExpireSigning([u8;32]) + canonical_digest match arms + sign_finalize_reward + sign_task_expire emitter fns | system_keypair.rs CanonicalMessage enum + canonical_digest match + terminal_summary_emitter mod |
| **P13** | Spec drift: § 0/§ 6/§ 9 still referenced TerminalSummaryTx in system_keypair after P3 migration | § 0 lists state::typed_tx; § 6 inline comment updated; § 9 D-3 row REMOVED (HTML comment marker) | spec § 0 / § 6 / § 9 |
| **P14** | Domain-prefix tests not load-bearing (round-1 used different bodies) | NEW signing_payload_domain_prefix_is_load_bearing (identical body, 6 distinct digests) + extended signing_payload_excludes_signature to all 6 + NEW signing_payload_golden_digests with 6 locked hex | typed_tx.rs tests |
| **P15** | BTreeMap permutation only covered BTreeSet | NEW typed_tx_btreemap_permutation_independence using PredicateResultsBundle.acceptance | typed_tx.rs tests |
| **GR-1** | MetaTx domain not reserved | NEW DOMAIN_AGENT_META_PROPOSAL constant (#[allow(dead_code)]) | typed_tx.rs |
| **GR-2** | TransitionError additive-only commitment absent | spec § 7.2 NEW additive-only commitment for all ABI enums | spec § 7.2 |
| **GR-3** | Domain rotation process undocumented | spec § 7.3 NEW rotation process | spec § 7.3 |

## Round-3 narrow closure questions

**Q1. P11 closure**: SignalKind::Finalize.claim_id is now ClaimId? SignalBundle::finalize takes ClaimId? Any other site grep would catch (e.g. test fixtures, doc-comments) still using TxId for claim references?

**Q2. P12 closure**: CanonicalMessage has both new variants? canonical_digest match exhaustive (no compile warning on new variants)? sign_finalize_reward / sign_task_expire actually invokable + symmetric to sign_terminal_summary? Any other code path that would need updating for the new variants (e.g. a verify-side counterpart)?

**Q3. P13 closure**: § 0 line 47 fixed? § 6 line 210 fixed? § 9 D-3 row actually removed (HTML comment present, no D-3 row visible)? Any OTHER stale TerminalSummaryTx-in-system_keypair reference that grep would catch?

**Q4. P14 closure**: signing_payload_domain_prefix_is_load_bearing test — does it ACTUALLY use identical body bytes (verify by reading the test)? signing_payload_excludes_signature now covers all 6 signed tx kinds? signing_payload_golden_digests has 6 locked EXPECTED_SIGNING_HEX_* constants matching the runtime computation?

**Q5. P15 closure**: typed_tx_btreemap_permutation_independence test uses PredicateResultsBundle.acceptance? Three different insertion orders produce byte-identical encoding?

**Q6. GR-1/2/3 closures**: DOMAIN_AGENT_META_PROPOSAL constant present? Spec § 7.2 + § 7.3 sections present and clear?

**Q7. NEW defects introduced by v1.2**: anything broken or unintentionally regressed? cargo test --lib still 224/0?

**Q8. PASS gate**: does v1.2 close all R2 findings cleanly enough for CO1.7-impl A2 unblock?

## Output format

# Codex CO1.1.4-pre1 Round-3 Audit
## Q1 P11 closure
## Q2 P12 closure
## Q3 P13 closure
## Q4 P14 closure
## Q5 P15 closure
## Q6 GR-1/2/3 closures
## Q7 New defects
## Q8 PASS gate
## **VERDICT**: PASS / CHALLENGE / VETO
## Top must-fix (if CHALLENGE)
## Conviction (low/med/high)

Be terse. Cite line numbers. PASS = all R2 must-fix closed; CHALLENGE = at least one specific patch-mechanical defect remaining; VETO = foundational design flaw (unlikely at this stage).

---

BRIEF_EOF

echo "" >> "$TMP_PROMPT"
echo "# CO1.1.4-pre1 spec v1.2 (target)" >> "$TMP_PROMPT"
cat "${ROOT}/handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# Implementation v1.2: src/state/typed_tx.rs\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/state/typed_tx.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# Supporting v1.2: src/bottom_white/ledger/system_keypair.rs (P12 target)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/bottom_white/ledger/system_keypair.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# Round-2 merged verdict (closure check reference)\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/audits/CO1_1_4_PRE1_DUAL_AUDIT_VERDICT_R2_2026-04-28.md" >> "$TMP_PROMPT"
printf '\n\n---\n\nNow give your INDEPENDENT round-3 narrow-scope audit. Cite line numbers.\n' >> "$TMP_PROMPT"

prompt_size=$(wc -c < "$TMP_PROMPT")
echo "[codex co1.1.4-pre1 r3] prompt size: ${prompt_size} chars" >&2

t0=$(date +%s)
{
  printf '# Codex CO1.1.4-pre1 Round-3 Audit\n'
  printf '**Date**: 2026-04-28\n'
  printf '**Target**: spec v1.2 + impl v1.2 + 20 tests joint artifact (closure check)\n'
  printf '**Prompt size**: %s chars\n\n---\n\n' "$prompt_size"
  codex exec --skip-git-repo-check < "$TMP_PROMPT" 2>&1
} > "$OUT"
t1=$(date +%s)
elapsed=$((t1 - t0))
echo "[codex co1.1.4-pre1 r3] API returned in ${elapsed}s" >&2
echo "[codex co1.1.4-pre1 r3] saved: $OUT" >&2
