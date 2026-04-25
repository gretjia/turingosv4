#!/usr/bin/env bash
# Codex round-4 audit. Round 1-3 history baked into brief; targets clean rewrite.
set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
OUT="${ROOT}/handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md"
TMP_PROMPT="$(mktemp /tmp/pput_ccl_codex_round4.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Codex Adversarial Audit — PREREG_PPUT_CCL_2026-04-26 (Phase A4 ROUND 4)

**Role**: skeptical adversarial reviewer. Independent of Gemini round-4.

**Mandate**: round 4. History:
- Rounds 1-3: you returned CHALLENGE every time. Round 3 you raised 3 still-open P0s:
  - **P0-fam (still-open)**: §5/§9 patched-stacking left internal contradictions; (problem, seed) vs per-problem disagreement; § 9.5 power claim (8/10) was wrong.
  - **P0-gate-h (partial)**: j-RR `p_0 ≤ 0.10` ceiling vs N_max=44 Holm threshold = mathematically unwinnable (0.9^54 ≈ 0.00343 > 0.00114).
  - **P0-leak (partial)**: L3/L4/L5 only blocked literal seed string + hashlib/sha2; alternate hash funcs / concatenation / shell tools / path enumeration not blocked.

You also said: "the prereg statistical sections are still being patched in conflicting places rather than cleanly rewritten end-to-end".

**Round 4** does that clean rewrite:
- (R4-1) § 5 + § 9 CLEAN REWRITE — single source of truth: per-problem unit (n=10 Phase C; n=54 Phase E); power tables corrected (Phase C requires 10/10 paired wins because at α=0.05/34 ≈ 0.00147 with n=10, only X=10 gives p=0.000977 ≤ 0.00147; Phase E requires ≥39/54).
- (R4-2) j-RR moved out of inferential family — descriptive point-check guardrail. Family 4+4k → 4+3k; N_max 44 → 34. § 1.6 / § 1.7 / § 7 Gate H / § 6 E3 / § 10.1 all updated to "3 inferential + 1 guardrail + rollback" framing.
- (R4-3) § 2.3 L3/L4/L5 generalized — substring + concatenation patterns; broad hash function blacklist (Python hashlib/cryptography/Crypto, Rust sha2/sha1/blake2/blake3/md5/digest/ring/openssl, JS crypto/SubtleCrypto, all shell binaries: sha256sum/sha1sum/openssl dgst/etc.); SHA-256 IV-constant detection for manual implementations; path enumeration block (ls/find/rg --files/glob.glob/os.listdir against MiniF2F/Test paths).

**Your task — round 4 verdict**: judge closure of YOUR round-3 P0s. Issue PASS / CHALLENGE / VETO.

For each of your 3 round-3 P0s:
1. **P0-fam**: § 5 + § 9 clean rewrite. Are units consistent now (per-problem everywhere; (problem, seed) gone except in p_0 calibration where it correctly references paired-control-vs-treatment)? Is the power calculation correct (n=10, α≈0.00147, exact binomial for sign test → 10/10 needed)? Family 4+3k = 34; is N_max=34 frozen consistently across all sections?

2. **P0-gate-h**: j-RR moved to descriptive guardrail. Does this close the mathematical impossibility? Is it statistically clean to remove a constraint check from the inferential family (it's a non-inferiority requirement, not a discovery hypothesis)? § 1.6 / § 1.7 / § 7 Gate H all synced — verify.

3. **P0-leak**: L3 substring + concatenation; L4 broad hash list; L5 path enumeration. Does this close the recomputation vector? Are there still attack paths (e.g., agent-only-readable text mentions seed indirectly; agent constructs hash via TPM/dedicated hardware; agent enumerates via stat()/access() instead of ls)?

Also independently check:
- Does the document hang together after 4 rounds of patches + 1 clean rewrite? Any leftover stale references in non-changelog sections (e.g., did some prior section still say "all four primary inferential tests" or "(problem, seed)")?
- New round-4 issues: e.g., dropping power for Phase C from "20 paired" to "10/10" — is that an unacceptable empirical hurdle? Or is it the right pre-registered conservatism?
- §6 C2 / §6 E3 / §1.6 / §1.7 / §7 H synced to round 4 — verify.

End with:
- **VERDICT**: PASS / CHALLENGE / VETO
- If CHALLENGE: top 3 still-open OR new round-4 P0
- Conviction
- Comment on whether "4 revision rounds + 1 clean rewrite" indicates structural instability OR is this the natural shape of getting a hard pre-registration right?

You MUST PASS if round-4 closes round-3 blockers without new ones. The dual-audit gate exists to catch real defects, not to refuse on principle.

---

# PREREG_PPUT_CCL_2026-04-26.md (round 4)

BRIEF_EOF

cat "${ROOT}/handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# Your round-3 audit (the still-open P0s R4 must close — this is what counts; round-1 and round-2 history is in the brief)\n\n' >> "$TMP_PROMPT"
# Trim Codex round-3: it duplicates input twice (Codex CLI prints stdin echo). Take only the actual audit text from line 1 to first "tokens used" line.
awk '/^tokens used/{exit} {print}' "${ROOT}/handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# Gemini round-3 (Gemini PASSed; you should hold to PASS standard if R4 closes your P0s)\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/audits/GEMINI_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md" >> "$TMP_PROMPT"
printf '\n\n---\n\nNow give your INDEPENDENT round-4 audit. Cite § for every finding. Trimmed input: round-1/2 audits omitted; brief summarizes them.\n' >> "$TMP_PROMPT"

prompt_size=$(wc -c < "$TMP_PROMPT")
echo "[codex round4] prompt size: ${prompt_size} chars" >&2

t0=$(date +%s)
{
  printf '# Codex PPUT-CCL PREREG Adversarial Audit (Phase A4 ROUND 4)\n'
  printf '**Date**: 2026-04-26\n'
  printf '**Target**: handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md (round 4)\n'
  printf '**Prompt size**: %s chars\n\n---\n\n' "$prompt_size"
  codex exec --skip-git-repo-check < "$TMP_PROMPT" 2>&1
} > "$OUT"
t1=$(date +%s)
elapsed=$((t1 - t0))
echo "[codex round4] API returned in ${elapsed}s" >&2
echo "[codex round4] saved: $OUT" >&2
