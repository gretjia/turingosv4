#!/usr/bin/env bash
# Codex round-3 audit on CO1.7-extra v1.1 (post round-2 patches).
# Implementer-paranoid angle: did v1.1 close all 10 r2 MFs without introducing
# new defects? Round-3 should converge to PASS unless v1.1 missed a closure
# or introduced a regression.
set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
OUT="${ROOT}/handover/audits/CODEX_CO1_7_EXTRA_ROUND3_AUDIT_2026-04-29.md"
TMP_PROMPT="$(mktemp /tmp/co1_7_extra_codex_round3.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Codex Adversarial Audit — CO1.7-extra v1.1 (Round 3; post round-2 patches)

**Role**: skeptical adversarial implementer-reviewer. Independent of Gemini round-3 (parallel).

**Mandate**: round 3 dual external audit on CO1.7-extra v1.1 — applied 10 patches (MF1-MF10) per `CO1_7_EXTRA_DUAL_AUDIT_VERDICT_R2_2026-04-29.md`. Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.

**What changed since round-2**: ArchitectAI applied all 10 r2 must-fix items. v1.1 commits: `25564d7` (v1.1 patches + audit artifacts) + `a3952cf` (smoke 11/11 PASS footer + 2 minor refinements). Smoke verified at HEAD `a3952cf`.

Round-2 must-fix items disposition:
- **MF1** § 0.4 supersession-disposition table corrected (head_t enacted in CO1.7-extra D2; SignalKind migrates) — § 0.4 patched
- **MF2** D2 testability: `advance_head_t` helper extracted from apply_one stage 9 — § 1.1 + new test § 3.3
- **MF3** trait method `head_commit_oid_hex` becomes REQUIRED (no default impl) — § 1.2
- **MF4** Sequencer placement: TuringBus directly (not nested through Kernel); STEP_B becomes single-file — § 2.1 + § 2.2
- **MF5** test harness flat-named `tests/co1_7_extra_*.rs` — § 3
- **MF6** manual Sequencer Debug `finish_non_exhaustive()` (Ed25519Keypair has no Debug) — § 2.1
- **MF7** `entry_at` helper private; tests construct LedgerEntry inline — § 3.2 (Codex r2 misidentified name as `canonical_test_entry`; v1.1 corrected to `entry_at`)
- **MF8** stale Sequencer comments at sequencer.rs:178-184 + :357-361 (spec v1.1 corrected line cites from r2's 180-184/359-361) — listed in atom landing checklist § 1.1
- **MF9** atomicity wording refined — § 1.1
- **MF10** LoC estimate 150-230 → 200-280 — § 7

**Open Q count after v1.1**: 0. Q1 (round-1) and Q1' (round-1 surfaced) both closed.

## What you're reviewing

1. **Spec doc**: `handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md` v1.1 (~520 lines)
2. **Round-2 audit artifacts** (verify closure): `CODEX_CO1_7_EXTRA_ROUND2_AUDIT_2026-04-29.md`, `GEMINI_CO1_7_EXTRA_ROUND2_AUDIT_2026-04-29.md`, `CO1_7_EXTRA_DUAL_AUDIT_VERDICT_R2_2026-04-29.md`
3. **Frozen prior specs**: CO1.7 v1.2 + CO1.1.4-pre1
4. **Shipped code**: src/state/sequencer.rs + src/bottom_white/ledger/transition_ledger.rs + src/bus.rs + src/state/q_state.rs + src/bottom_white/cas/store.rs

## Round 3 audit questions

**Q1. MF1 § 0.4 disposition table closure**: spec § 0.4 now has explicit table — head_t enacted HERE (D2); SignalKind migrates to future CO1.7.5. Verify:
- Is the table content factually correct?
- Does the principle assertion ("downstream supersedes upstream within layered boundary") still hold post-correction?
- Any residual drift?

**Q2. MF2 advance_head_t helper extraction**: spec § 1.1 introduces `advance_head_t(q, writer)` helper called from apply_one stage 9. Verify:
- Helper signature is correct + testable in isolation
- apply_one stage 9 logic preserved byte-identically (pure refactor, zero behavior change)
- New test § 3.3 actually exercises the D2 code path (calls `advance_head_t` directly with mock writer)
- Mock LedgerWriter implementation in test correctly tests both Some and None paths

**Q3. MF3 required trait method**: spec § 1.2 makes `head_commit_oid_hex` required (no default). Verify:
- The trait definition is syntactically correct
- Both `Git2LedgerWriter` and `InMemoryLedgerWriter` implementations explicitly declare
- The "compiler enforces" claim holds (a missing impl would be E0046)
- Both safety arguments (silent stagnation prevention + no-panic) actually satisfied

**Q4. MF4 Sequencer placement (TuringBus, not Kernel)**: § 2.1 + § 2.2 rewritten. Verify:
- TuringBus (NOT Bus, NOT Kernel) gets the field + constructor + forwarder
- Kernel UNTOUCHED — does the spec consistently state this? Is "pure topology" doctrine preserved?
- STEP_B is now single-file (bus.rs only) — does § 2.2 ceremony procedure match? Is the "less invasive alternative" framing correct?
- Any residual references to Kernel.sequencer that should now be TuringBus.sequencer?

**Q5. MF5 + MF6 + MF7 + MF8 + MF9 + MF10 small fixes**: verify each:
- MF5: test paths use flat naming `tests/co1_7_extra_*.rs` (§ 3.1, § 3.2, § 3.3 file paths)
- MF6: manual Sequencer Debug with `finish_non_exhaustive()` (§ 2.1) — uses correct method
- MF7: `entry_at` helper at line 813 (Codex r2 misidentification corrected)
- MF8: stale comment line cites match real source (sequencer.rs:178-184 + :357-361)
- MF9: atomicity wording uses "non-failing best-effort head binding" / "explicit no-op preservation"
- MF10: LoC estimate 200-280 in § 7

**Q6. New defects in v1.1**: did the patches introduce any new issues?
- Any internal contradictions between sections (e.g., § 0.4 table vs § 1.1 helper code)?
- Implementation-blocking ambiguities in the new helper signature or test files?
- Anything in v1.1 that compiles in spec but won't compile when implemented?
- LoC estimate 200-280 still defensible given the new test mock?

**Q7. Implementation gating**: assuming v1.1 reaches PASS, is CO1.7-extra implementable end-to-end with no further blockers?

## Output format

# Codex CO1.7-extra Round-3 Audit
## Q1 MF1 § 0.4 disposition table closure
## Q2 MF2 advance_head_t helper extraction
## Q3 MF3 required trait method
## Q4 MF4 Sequencer placement (TuringBus)
## Q5 MF5-MF10 small fixes
## Q6 New defects in v1.1
## Q7 Implementation gating
## **VERDICT**: PASS / CHALLENGE / VETO
## Top issues (if CHALLENGE)
## Conviction (low/med/high)

Be rigorous. Cite spec line numbers + source file line numbers when calling defects. Do NOT pass on principle; do NOT veto on principle. Real defects = CHALLENGE; foundational design flaw = VETO; clean spec = PASS.

---

BRIEF_EOF

echo "" >> "$TMP_PROMPT"
echo "# CO1.7-extra v1.1 (target of audit)" >> "$TMP_PROMPT"
echo "" >> "$TMP_PROMPT"
cat "${ROOT}/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# XREF: round-2 merged verdict (the document driving v1.1 patches)\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/audits/CO1_7_EXTRA_DUAL_AUDIT_VERDICT_R2_2026-04-29.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# XREF: CO1.7 v1.2 spec (frozen, round-3 PASS/PASS)\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# XREF: shipped src/state/sequencer.rs (D2 target)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/state/sequencer.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# XREF: shipped src/bottom_white/ledger/transition_ledger.rs (D2 trait target)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/bottom_white/ledger/transition_ledger.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# XREF: shipped src/bus.rs (D3 STEP_B target — TuringBus)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/bus.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# XREF: shipped src/state/q_state.rs (NodeId + Q_t)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/state/q_state.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# XREF: shipped src/bottom_white/ledger/system_keypair.rs (Ed25519Keypair derives — for MF6 verification)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/bottom_white/ledger/system_keypair.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\nNow give your INDEPENDENT round-3 audit. Cite spec § + line where possible. Round-3 expects PASS unless v1.1 missed a closure or introduced a regression.\n' >> "$TMP_PROMPT"

prompt_size=$(wc -c < "$TMP_PROMPT")
echo "[codex co1.7-extra r3] prompt size: ${prompt_size} chars" >&2

t0=$(date +%s)
{
  printf '# Codex CO1.7-extra Round-3 Audit\n'
  printf '**Date**: 2026-04-29\n'
  printf '**Target**: spec v1.1 (post round-2 patches)\n'
  printf '**HEAD**: %s\n' "$(git -C "$ROOT" rev-parse HEAD)"
  printf '**Prompt size**: %s chars\n\n---\n\n' "$prompt_size"
  codex exec --skip-git-repo-check < "$TMP_PROMPT" 2>&1
} > "$OUT"
t1=$(date +%s)
elapsed=$((t1 - t0))
echo "[codex co1.7-extra r3] API returned in ${elapsed}s" >&2
echo "[codex co1.7-extra r3] saved: $OUT" >&2
