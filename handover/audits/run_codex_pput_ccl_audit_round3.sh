#!/usr/bin/env bash
# Codex adversarial audit of PREREG_PPUT_CCL_2026-04-26 ROUND 3.
#
# Round 1: Codex CHALLENGE / Gemini CHALLENGE → CHALLENGE.
# Round 2: Codex CHALLENGE / Gemini PASS → CHALLENGE.
# Round 3: 3 Codex round-2 P0 fixes (R3-1 family timing, R3-2 p_0 protocol,
#          R3-3 5-layer sealing) + R3-4 marginal-contribution claim caveat.
#          Re-audit asks: closure on YOUR round-2 P0s + new issues.
#
# Output: handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md
set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
OUT="${ROOT}/handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md"
TMP_PROMPT="$(mktemp /tmp/pput_ccl_codex_round3.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Codex Adversarial Audit — PREREG_PPUT_CCL_2026-04-26 (Phase A4 ROUND 3)

**Role**: skeptical adversarial reviewer. Independent of Gemini round-3.

**Mandate**: round 3. Round-1 dual = CHALLENGE/CHALLENGE. Round-2: Gemini PASS, you (Codex) CHALLENGE. Your round-2 P0s:

- **P0-fam** (still-open): N=4+4k unknown at Gate C; (problem, seed) not independent unit.
- **P0-gate-h** (partial): p_0 calibration was new tuning surface — no protocol freeze.
- **P0-leak** (partial): seed string leaks through architect/audit files; agent could grep + run hashlib.

You also flagged P0-stat-gemini (partial), P0-measure (closed-with-caveat), P0-spof (closed).

**Round 3** applies 3 fixes (R3-1, R3-2, R3-3) + § 10 R3-4 marginal-contribution claim caveat for your closed-with-caveat P0-measure.

**Your task — round 3 verdict**: judge closure of your 3 still-open/partial P0s. Issue PASS / CHALLENGE / VETO.

## Round-2 P0s YOU raised (judge closure)

**P0-fam (your round-2 still-open, family timing + (problem, seed) independence)**:
- Round-2: Gate C in Phase C ran McNemar at Holm thresholds depending on final 4+4k; k unknown until Phase D end. Plus seeds within problem correlated.
- Round-3 R3-1: § 9 rewritten — `k_max = 10` pre-committed at A5; `N_max = 4 + 4·k_max = 44` frozen for ALL Holm decisions. Realized k > 10 = arc abort. § 6 C2 + § 9.2 changed independent unit from (problem, seed) to per-problem (seeds = noise-reducing replicates); McNemar uses n=10 paired-binary observations on hard-10. § 9.5 power recomputed.
- **Your check**: does fixing N_max = 44 (irrespective of realized M) close the timing issue? Is per-problem unit defensible? Does losing power (n=10 vs prior n=20) constitute new harm? Was k_max = 10 pre-committed responsibly (or is it itself a tuning surface)?

**P0-gate-h (your round-2 partial, p_0 tuning surface)**:
- Round-2: § 5.2 said "p_0 calibrated by ablating any single recent code change" — toggle / sample / estimator / freeze unspecified.
- Round-3 R3-2: full protocol — toggle = `--simulate-rollback-at-tx-50`; sample = full adaptation-144 × 2 seeds = 288 paired runs; estimator = per-problem worst-case across seeds, p_0 = sum/144; schedule = Phase B7 mandatory (not deferrable to Phase D); freeze = genesis_payload.toml + Trust Root SHA-256 of jsonl; ceiling = p_0 > 0.10 ABORT (not auto-pass attack); audit = dual-audit at B → C transition checkpoint.
- **Your check**: is the protocol fully specified? Does the ceiling block auto-pass attacks? Is there any remaining analyst degree of freedom?

**P0-leak (your round-2 partial, sealing leaky)**:
- Round-2: blacklist covered PREREG/SPLITS files only; agent could grep seed from architect insight files / audit echoes / tool args.
- Round-3 R3-3: § 2.3 5-layer sealing. L1 file-path read isolation (existing). L2 prompt-context blacklist EXPANDED to all seed-bearing files (architect insights, audit files, dual-audit verdict, split script, any file containing `20260426_PPUT_CCL`). L3 tool-call ARGUMENT scanner (blocks `echo "20260426_PPUT_CCL" | sha256sum`-style). L4 hashlib gate (static analysis: agent code path with hashlib AND seed access → BLOCKED; dynamic via L3). L5 MiniF2F/Test ID blacklist (agent doesn't see source pool, can't enumerate buckets even with seed).
- **Your check**: does the 5-layer defense actually close the recomputation vector? Any layers vulnerable? Residual git-history risk acknowledged honestly?

## Round-3-introduced concerns to check independently

1. **k_max = 10 cap** as new tuning surface: who decided 10? If Phase D realistically produces k > 10, arc aborts — does this give an incentive to artificially limit Phase D throughput?
2. **Per-problem aggregation power loss**: hard-10 × 1 paired-binary = effectively a 10-trial test at α = 0.00114 (Holm with N=44). Power expectation unchanged from round 2 OR materially weaker?
3. **p_0 = 0.10 ceiling**: was 0.10 pre-committed responsibly, or could a higher actual baseline on the v4-flash backbone trigger an unwarranted abort?
4. **L4 hashlib gate**: defensible against adversarial paraphrase (e.g., agent uses MD5 of (seed + pid) instead of SHA-256, recomputes a different bucket?) — does L3 catch all hash-function call sites or only sha256sum?
5. **§ 10 R3-4 claim caveat**: is the caveat strict enough?

## Verdict format

For each of your 3 round-2 P0s: **closed / partial / still-open + 1-sentence reason citing §**.

Then:
- **VERDICT**: PASS / CHALLENGE / VETO
- If CHALLENGE: top 3 still-open OR new round-3 P0
- Conviction
- Are you concerned that 3 revision rounds indicate structural instability?

You MUST PASS if round-3 revisions close round-2 blockers without new ones.

---

# PREREG_PPUT_CCL_2026-04-26.md (round 3)

BRIEF_EOF

cat "${ROOT}/handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# Three-split JSON\n\n```json\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# Round-1 merged verdict\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/audits/DUAL_AUDIT_PPUT_CCL_VERDICT_2026-04-26.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# Round-1 Codex audit (your prior)\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# Round-2 Codex audit (your prior, the still-open P0s round-3 must close)\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# Round-2 Gemini audit (Gemini PASSed; you should hold to Gemini-PASS standard)\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/audits/GEMINI_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md" >> "$TMP_PROMPT"
printf '\n\n---\n\nNow give your INDEPENDENT round-3 audit. Strictly evaluate closure of your round-2 P0s + check round-3-introduced issues. Cite § for every finding.\n' >> "$TMP_PROMPT"

prompt_size=$(wc -c < "$TMP_PROMPT")
echo "[codex round3] prompt size: ${prompt_size} chars" >&2

t0=$(date +%s)
{
  printf '# Codex PPUT-CCL PREREG Adversarial Audit (Phase A4 ROUND 3)\n'
  printf '**Date**: 2026-04-26\n'
  printf '**Target**: handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md (round 3)\n'
  printf '**Prompt size**: %s chars\n\n---\n\n' "$prompt_size"
  codex exec --skip-git-repo-check < "$TMP_PROMPT" 2>&1
} > "$OUT"
t1=$(date +%s)
elapsed=$((t1 - t0))
echo "[codex round3] API returned in ${elapsed}s" >&2
echo "[codex round3] saved: $OUT" >&2
