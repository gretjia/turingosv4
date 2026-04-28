#!/usr/bin/env bash
# Codex round-2 audit on CO1.1.4-pre1 v1.1 (post round-1 CHALLENGE).
# Closure verification of P1-P10 patches.
set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
OUT="${ROOT}/handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md"
TMP_PROMPT="$(mktemp /tmp/co1_1_4_pre1_codex_round2.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Codex Round-2 Audit — CO1.1.4-pre1 Typed Tx ABI v1.1 (post round-1 CHALLENGE)

**Role**: skeptical adversarial implementer-reviewer. Independent of Gemini round-2 (running in parallel).

**Mandate**: round-2 closure-verification audit on v1.1 joint artifact (spec + impl + 17 tests). Round-1 returned CHALLENGE/CHALLENGE (verdict at `handover/audits/CO1_1_4_PRE1_DUAL_AUDIT_VERDICT_R1_2026-04-28.md`); v1.1 (commit `e0e4565`) claims to close 10 patches (P1-P10).

Per CLAUDE.md "Audit Standard": Generator ≠ Evaluator. Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.

## What changed v1 → v1.1 (per patch log)

| ID | v1 issue (round-1 finding) | v1.1 fix claim | What to verify |
|---|---|---|---|
| **P1** (C-1) | Agent-sig domain separation missing | NEW signing-payload structs + b"turingosv4.<actor>.<purpose>.v1" domain prefixes; to_signing_payload() projection on each tx | Cross-domain digests truly distinct? Domain prefix actually included in SHA-256 input? Signature excluded from signed digest? |
| **P2** (C-3 Codex Q-B) | claim_id: TxId leaked impl | NEW ClaimId(pub TxId) #[serde(transparent)] newtype | Wire-identical to TxId? Type-distinction at API surface? |
| **P3** (C-3 Codex Q-C) | TerminalSummaryTx 3-field placeholder | Migrated to 8-field STATE § 1.5 schema in state::typed_tx; system_keypair signs opaque [u8;32] via NEW CanonicalMessage::TerminalSummarySigning variant | Full schema present? bottom_white ↔ state circular dep eliminated? Old struct fully removed? Sig API correct? |
| **P4** (CX-1 Codex Q-G) | TransitionError 10 variants insufficient | Expanded to 22 variants per STATE § 3 pseudocode | All variants STATE § 3 invokes covered? NotYetImplemented retained as stub sentinel only? |
| **P5** (C-2 both auditors) | Golden fixtures unlocked + TerminalSummary missing | Hardcoded SHA-256 hex for all 7 variants; +6 new tests (cross-variant non-collision, BTreeSet permutation, default round-trip, signing-payload domain distinctness, signing-payload-excludes-signature, TerminalSummary in round-trip+kind) | Hex actually locked (not just length=64)? All 7 variants covered? Permutation test catches HashMap-style hazard? |
| **P6** (CX-2 Codex Q-D) | STATE § 2.5 wording wrong | spec § 7.1 codec wording fixed (u32 BE variants, u64 BE lengths); codec unchanged | Wording accurate now? Cross-references valid? Decision to keep u32/u64 (not force u8) defensible? |
| **P7** (C-3 followup) | D-3 divergence | RESOLVED via P3 | D-3 row fully removed from § 9 (not just edited)? |
| **P8** (C-3 + GM-2) | FinalizeRewardTx Q-derived discipline + dual-sign rationale | NEW § 4.1 Q-derived (task_id/solver/reward authoritative from Q at replay; royalty NOT on wire); NEW § 4.2 dual-sign (this struct's sig binds payload bytes; envelope sig binds sequencer-stamped bytes; both needed) | Q-derived discipline clearly stated? Replay rule (CO1.7-impl A4) committed? Dual-sign rationale convincing or still redundant? |
| **P9** (GM-1 Gemini Q4) | Cold-replay Art 0.2 | NEW § 0.1 cross-atom ordering gate: v1.1 PASS contingent on CO1.4-extra shipping BEFORE CO1.7-impl A4 | Ordering gate explicit + binding? Constitutional commitment strong enough? |
| **P10** (CX-3 Codex Q-J) | TaskId vs TxId QState mismatch | NEW § 9 D-4: cross-atom debt assigned to CO P2.1; no retrofit in this atom | D-4 sufficiently honest? Future-migration plan clear? Wire-format consequence correctly assessed (none)? |

## Your previous (round-1) verdict

You returned **CHALLENGE** with high conviction. Top 3 must-fix:
1. Replace TerminalSummaryTx with 8-field STATE schema + add tests/fixtures
2. Fix canonical serialization documentation/tests: hardcode golden hex, include all variants, document actual bincode u32 enum/u64 length behavior
3. Add ClaimId, complete TransitionError, define signature-domain signing payloads excluding signature fields

Per-question PASS items: Q-A (D-1 TxStatus elision PASS w/ patch note); Q-H (HasSubmitter correct); Q-I (atom scope creep PASS w/ caveat).

## Round-2 closure verification questions

**Q-1. P1 closure (agent-sig domain separation)**: Cite typed_tx.rs lines for the 6 SigningPayload structs + canonical_digest impls. Is `domain_prefixed_digest` correct (sha256(domain || canonical_encode(self)))? Does signing_payload_excludes_signature test prove what it should? Cross-domain non-collision (signing_payload_domains_are_distinct) sufficient or trivial-pass? Any new defect?

**Q-2. P2 closure (ClaimId newtype)**: `#[serde(transparent)]` actually wire-identical to TxId? FinalizeRewardTx.claim_id field updated? Fixture updated to use ClaimId::new()? Any other missed call site?

**Q-3. P3 closure (TerminalSummaryTx migration)**: Old 3-field struct fully removed from system_keypair.rs (not just commented out)? New 8-field struct in typed_tx.rs has correct serde derives? CanonicalMessage::TerminalSummarySigning([u8;32]) variant added correctly + canonical_digest match arm updated? terminal_summary_emitter::sign_terminal_summary takes [u8;32] (not the struct)? bottom_white ↔ state circular dep verifiably absent (grep)?

**Q-4. P4 closure (TransitionError taxonomy)**: 22 variants enough? Each STATE § 3.1-3.7 pseudocode error invocation has a corresponding variant? Are payloads (PredicateId for *PredicateFailed) right or should they be richer (PredicateResultsBundle)?

**Q-5. P5 closure (golden fixtures + new tests)**: Hex constants actually load-bearing (assertion fails on diff)? cargo test --lib confirms 17/17 PASS? TerminalSummary in round_trip_all_variants + tx_kind_projection + golden? Cross-variant non-collision pairwise (7×7) or just smoke? BTreeSet permutation test would catch a HashMap-style codec bug?

**Q-6. P6 closure (codec wording)**: spec § 7.1 wording now matches bincode-2.0.1 source citations? Decision to keep u32/u64 (not force u8) — is that the right call given >256 variant ceiling never likely to be hit? Any cascaded impact on shipping CO1.7 LedgerEntrySigningPayload digest stability? (CO1.7 commit a03cc52 used same canonical_encode helper.)

**Q-7. P8 + P9 closure (Q-derived + dual-sign + cold-replay gate)**: Spec § 4.1 commits replay (CO1.7-impl A4) to Q-derived discipline — is this binding enough? Spec § 0.1 cross-atom ordering gate phrased as "MUST NOT ship before CO1.4-extra" — strong constitutional commitment or hedged language?

**Q-8. P10 closure (D-4 TaskId/TxId)**: Adequate honesty about the cross-atom debt? Any wire-format consequence I'm missing? CO P2.1 the right atom owner?

**Q-9. New defects in v1.1 (independent of P1-P10)**:
- Test gaps: any test class still missing (e.g. signing-payload round-trip, signing-payload golden hex)?
- Type errors that cargo check missed?
- Spec ↔ code parity drift introduced by v1.1 patches?
- Imports: does `use sha2::{Digest, Sha256}` collision-free with existing usage?
- Missing Default impls causing panics in #[derive(Default)] expansion?

**Q-10. Implementation gating (overall)**: with v1.1 closure, is CO1.7-impl A2 (TypedTx + Sequencer + dispatch_transition) implementable end-to-end against this ABI surface? Specific blockers to call out.

## Output format

# Codex CO1.1.4-pre1 Round-2 Audit
## Q-1 P1 closure
## Q-2 P2 closure
## Q-3 P3 closure
## Q-4 P4 closure
## Q-5 P5 closure
## Q-6 P6 closure
## Q-7 P8+P9 closure
## Q-8 P10 closure
## Q-9 New defects
## Q-10 Implementation gating
## **VERDICT**: PASS / CHALLENGE / VETO
## Top must-fix (if CHALLENGE) — be specific
## Conviction (low/med/high)

Be rigorous. Cite spec § + code line numbers. Per memory `feedback_dual_audit_conflict`: do NOT pass on principle; do NOT veto on principle. Real defects = CHALLENGE; foundational design flaw = VETO; clean closure = PASS.

---

BRIEF_EOF

echo "" >> "$TMP_PROMPT"
echo "# CO1.1.4-pre1 spec v1.1 (target of audit)" >> "$TMP_PROMPT"
echo "" >> "$TMP_PROMPT"
cat "${ROOT}/handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# Implementation v1.1: src/state/typed_tx.rs (target of audit)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/state/typed_tx.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# Supporting v1.1: src/state/mod.rs (re-exports)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/state/mod.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# Supporting v1.1: src/bottom_white/ledger/system_keypair.rs (TerminalSummary migration target)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/bottom_white/ledger/system_keypair.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# Round-1 merged verdict (closure check reference)\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/audits/CO1_1_4_PRE1_DUAL_AUDIT_VERDICT_R1_2026-04-28.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# XREF: STATE_TRANSITION_SPEC v1.4 (frozen)\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# XREF: shipped src/bottom_white/ledger/transition_ledger.rs (consumes typed_tx)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/bottom_white/ledger/transition_ledger.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\nNow give your INDEPENDENT round-2 closure-verification audit. Cite spec § + code line where possible.\n' >> "$TMP_PROMPT"

prompt_size=$(wc -c < "$TMP_PROMPT")
echo "[codex co1.1.4-pre1 r2] prompt size: ${prompt_size} chars" >&2

t0=$(date +%s)
{
  printf '# Codex CO1.1.4-pre1 Round-2 Audit\n'
  printf '**Date**: 2026-04-28\n'
  printf '**Target**: spec v1.1 + impl v1.1 + 17 tests joint artifact (closure verification)\n'
  printf '**Prompt size**: %s chars\n\n---\n\n' "$prompt_size"
  codex exec --skip-git-repo-check < "$TMP_PROMPT" 2>&1
} > "$OUT"
t1=$(date +%s)
elapsed=$((t1 - t0))
echo "[codex co1.1.4-pre1 r2] API returned in ${elapsed}s" >&2
echo "[codex co1.1.4-pre1 r2] saved: $OUT" >&2
