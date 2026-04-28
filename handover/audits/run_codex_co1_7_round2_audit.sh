#!/usr/bin/env bash
# Codex round-2 audit on CO1.7 transition_ledger v1.1 (post round-1 CHALLENGE).
# Closure verification of the 11 must-fix items + 1 disagreement (D1).
set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
OUT="${ROOT}/handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md"
TMP_PROMPT="$(mktemp /tmp/co1_7_codex_round2.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Codex Round-2 Audit — CO1.7 transition_ledger v1.1 (post round-1 CHALLENGE)

**Role**: skeptical adversarial implementer-reviewer. Independent of Gemini round-2 (running in parallel).

**Mandate**: round 2 closure-verification audit on v1.1 joint artifact (spec + skeleton). Round-1 returned CHALLENGE/CHALLENGE; v1.1 claims to close 11 must-fix + 1 disagreement (D1).

Per CLAUDE.md "Audit Standard": Generator ≠ Evaluator. Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.

## What changed v1 → v1.1 (per patch log at top of spec)

| ID | v1 issue | v1.1 fix claim |
|---|---|---|
| C1 | replay single-mode | Two-mode `ReplayMode::ChainOnly` (skeleton) vs `FullTransition` (CO1.7.5+); I-DETHASH bound to Full only |
| C2 | shipped CasStore::open inits empty in-memory index → cold-replay impossible | New CO1.4-extra atom (separate); v1 doc + ReplayError::CasMissing |
| C3 | signing primitive integration unspecified | LedgerEntrySigningPayload struct + CanonicalMessage::LedgerEntrySigning(_) variant + sign_ledger_entry API |
| K1 | sequencer fetch_add before accept skips logical_t | Dual counter: next_submit_id (submit) + next_logical_t (commit only) |
| K2 | signature did not bind parent_ledger_root | Added field + bound in signing payload + new test |
| K3 | L4/L5 head_t ownership inconsistent | CO1.7 owns ledger_root + commit-chain head_t; CO1.8 owns state_root |
| K4 | spec/skeleton trait mismatch | Spec aligned to skeleton: `commit(&mut self) -> Hash`; iter_from deferred |
| K5 | TxKind::Slash dispatch gap | Slash DROPPED for v4 (CO P2.5 atom) |
| K6 | tx_kind cast without #[repr(u8)] | #[repr(u8)] + explicit discriminants |
| K7 | spec promised 8 tests, skeleton had 6 | 8 tests with stage marker (4 skeleton + 4 CO1.7.5+) |
| G1 | LedgerEntry rigid; no forward-compat slot | extensions: BTreeMap<String, Vec<u8>> bound in signing payload |
| D1 | epoch binding disagreement | Conservative: bound in signing payload (Codex security wins) |

## Your previous (round-1) verdict

You returned **CHALLENGE** with high conviction. Your top 3 must-fix:
1. Sequencer logical time: no skipped accepted-entry; clear submit-vs-accept ordering
2. Redesign ledger signing payload to bind epoch + parent ledger root; exclude only derived root and signature
3. Replay honestly two-mode + ensure full replay can recover CAS payloads cold

You also raised: parent_ledger_root binding (NEW), CAS cold-replay (CasStore index empty), spec/skeleton trait mismatch, TxKind::Slash dispatch gap, #[repr(u8)] missing, conformance test gap.

## Round-2 audit questions

For each round-1 finding, judge **CLOSED / PARTIAL / REGRESSED / NEW-ISSUE**:

**Q-A1** (C1 replay two-mode): Does v1.1 ReplayMode enum + replay_chain_integrity rename + I-DETHASH binding to FullTransition only fully close the trust ambiguity? Or is documentation insufficient?

**Q-A2** (C2 CAS cold-replay): Is CO1.4-extra atom plan acceptable, or does spec need to ship CAS persistence in CO1.7 itself? Is `ReplayError::CasMissing` adequate when the dependent atom hasn't shipped?

**Q-A3** (C3 signing integration): Does `LedgerEntrySigningPayload` + `CanonicalMessage::LedgerEntrySigning` extension correctly close DIV-1? Verify: (a) the canonical_digest method binds the right 9 fields with deterministic byte layout; (b) the sign API extension is additive (Wave 4-B not breaking); (c) the forward-compat clause for future ledger-side variants is sound.

**Q-A4** (K1 sequencer): Is dual-counter design (next_submit_id at submit, next_logical_t at commit) correct? Verify spec § 3 apply_one ordering: stage 4 logical_t assignment AFTER stage 2 dispatch_transition success. Does anything else break (e.g., does submit() returning submit_id without logical_t still satisfy I-LOGTIME ordering for the agent's perspective)?

**Q-A5** (K2 transplant): parent_ledger_root field added + bound in signing payload + new test. Verify: (a) skeleton tests/replay_rejects_parent_ledger_tamper actually exercises the transplant defense path; (b) signing payload digest includes parent_ledger_root before the signature is computed; (c) any other transplant-vector still open?

**Q-A6** (K3 L4/L5 boundary): Verify spec § 3 apply_one no longer mutates head_t via from_state_root. Verify spec § 5 says head_t = NodeId(commit_sha) only. Is the boundary documentation sufficient, or does CO1.8 need a stub trait reference now?

**Q-A7** (K4 trait): commit(&mut self) -> Hash matches skeleton. iter_from deferred. Does this leave a hole for FullTransition replay? (it does — but flagged as CO1.7.5+ stage, OK?)

**Q-A8** (K5 Slash drop): Verify TxKind has 7 variants (no Slash); dispatch enum-match has no Slash arm. Is "drop until CO P2.5" cleaner than "stub now + impl later"? Any forward-compat hazard?

**Q-A9** (K6 #[repr(u8)]): Verify enum has #[repr(u8)] + Work=0..TerminalSummary=6. Verify canonical_digest uses `tx_kind as u8` safely. Any related serde-derive concern (TxKind has no serde derive in v1.1)?

**Q-A10** (K7 conformance): 8 tests now (4 skeleton + 4 CO1.7.5+). Verify the 4 skeleton-stage tests are actually present in skeleton (not the spec only). The CO1.7.5+ stage tests are deferred — is that OK, or should v1.1 ship at least stubbed unimplemented!() test functions?

**Q-A11** (G1 extensions): BTreeMap<String, Vec<u8>> field added; bound in signing payload via length-prefix iteration. Is the canonical_digest extension-binding correct (prevents collision attacks via length-prefix)? Forward-compat: when a v4.x feature populates extensions, will old verifiers reject the new entries? (yes — by design — but is this safe for upgrade?)

**Q-A12** (D1 resolution): epoch bound in signing payload (your security argument won). Verify spec § 1.1 + skeleton both have epoch in canonical_digest. Any cross-epoch transplant residual still open? (e.g., what if old-epoch private key compromised AND attacker can re-sign with current pubkey claim?)

## NEW v1.1 issues (independent of round-1)

**Q-B1**: review the new sequencer apply_one ordering in spec § 3. Specifically:
- Stage 3 puts payload to CAS BEFORE stage 4 assigns logical_t. The tentative CAS metadata `created_at_logical_t = self.next_logical_t.load() + 1` is racy — what if multiple submitters succeed before any commit?
- The CAS put's `creator` field is `format!("sequencer-{}", self.epoch.get())` — does this leak the epoch identity to CAS metadata? Is that intentional?

**Q-B2**: § 1.2 forward-compat clause says "future ledger-side variants add new CanonicalMessage::*". But CanonicalMessage is in shipped Wave 4-B code; does v4 vs v4.1 boundary policy permit any module to add variants, or only ledger module?

**Q-B3**: § 4 ReplayError has 8 variants now (4 ChainOnly + 4 FullTransition). Are the FullTransition-mode-only variants reachable from ChainOnly mode? If so → invalid state. Should ReplayError be split into 2 enums per mode?

**Q-B4**: skeleton's `to_signing_payload(&self) -> LedgerEntrySigningPayload` clones `extensions`. For very large extensions (multi-KB blobs), this is O(N) work per replay step. Is this an acceptable cost, or should `to_signing_payload` return a borrowed view?

## Independent verification

**Q-C1**: review `cargo test --lib bottom_white::ledger::transition_ledger::` output (8 tests PASS reported in commit message). Are the test names + assertions structurally consistent with the closures claimed?

**Q-C2**: spot-check the spec § 1 LedgerEntry struct vs skeleton struct: are field counts (11), order, types all matching?

**Q-C3**: TR manifest update — refreshed hashes for spec + skeleton. Anything else needs TR tracking that wasn't?

## Output format

# Codex CO1.7 Round-2 Audit
## Q-A1..A12 Round-1 closure judgments (CLOSED / PARTIAL / REGRESSED / NEW-ISSUE)
## Q-B1..B4 New v1.1 issues
## Q-C1..C3 Independent verification
## **VERDICT**: PASS / CHALLENGE / VETO
## Top must-fix (if CHALLENGE) — be specific; cite spec § + skeleton line
## Conviction (low/med/high)

You MUST PASS if v1.1 closes round-1 blockers without new ones. The audit gate exists to catch real defects, not to refuse on principle. Do NOT challenge on issues already addressed or on style preferences.

If CHALLENGE: list exactly what to patch in v1.2.
If VETO: explain why a foundational design flaw remains (round-1 issued no VETO, so this would be a regression).
If PASS: explicit GO for CO1.7 implementation start (CO1.7.5 etc.).

---

BRIEF_EOF

echo "" >> "$TMP_PROMPT"
echo "# CO1.7 spec v1.1 (current target of round-2 audit)" >> "$TMP_PROMPT"
echo "" >> "$TMP_PROMPT"
cat "${ROOT}/handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# Type-skeleton v1.1 (current target of round-2 audit)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/bottom_white/ledger/transition_ledger.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# Your round-1 audit verdict (the must-fix items v1.1 must close)\n\n' >> "$TMP_PROMPT"
awk '/^tokens used/{exit} {print}' "${ROOT}/handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# Gemini round-1 audit verdict (parallel; convergent flagged 3 same items)\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/audits/GEMINI_CO1_7_ROUND1_AUDIT_2026-04-28.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# Merged round-1 verdict + v1.1 patch list (the architect synthesis)\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/audits/CO1_7_DUAL_AUDIT_VERDICT_R1_2026-04-28.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# XREF: STATE_TRANSITION_SPEC v1.4 (frozen)\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# XREF: shipped system_keypair.rs (round-1 ground truth)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/bottom_white/ledger/system_keypair.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\nNow give your INDEPENDENT round-2 audit. For each of your round-1 must-fix items, explicitly judge CLOSED/PARTIAL/REGRESSED.\n' >> "$TMP_PROMPT"

prompt_size=$(wc -c < "$TMP_PROMPT")
echo "[codex co1.7 r2] prompt size: ${prompt_size} chars" >&2

t0=$(date +%s)
{
  printf '# Codex CO1.7 transition_ledger Round-2 Audit\n'
  printf '**Date**: 2026-04-28\n'
  printf '**Target**: spec v1.1 + skeleton v1.1 joint artifact\n'
  printf '**Prompt size**: %s chars\n\n---\n\n' "$prompt_size"
  codex exec --skip-git-repo-check < "$TMP_PROMPT" 2>&1
} > "$OUT"
t1=$(date +%s)
elapsed=$((t1 - t0))
echo "[codex co1.7 r2] API returned in ${elapsed}s" >&2
echo "[codex co1.7 r2] saved: $OUT" >&2
