#!/usr/bin/env bash
# Codex adversarial audit of PREREG_PPUT_CCL_2026-04-26 (Phase A4 round 1).
# Mirror of run_gemini_pput_ccl_audit.py but invokes Codex CLI (logged-in ChatGPT auth).
# Conservative ruling per VETO > CHALLENGE > PASS. PASS/PASS gates Phase B.
#
# Codex must run independently of Gemini. Brief explicitly forbids consulting
# the Gemini verdict file. Both run in parallel before merge.
#
# Output: handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md
set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
OUT="${ROOT}/handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md"
TMP_PROMPT="$(mktemp /tmp/pput_ccl_codex_brief.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Codex Adversarial Audit — PREREG_PPUT_CCL_2026-04-26 (Phase A4 round 1)

**Role**: skeptical adversarial reviewer for an ICLR/NeurIPS Systems track. Independent of Gemini (do NOT read GEMINI_PPUT_CCL_AUDIT_*.md if it exists; it is being written in parallel).

**Mandate**: find pre-registration weaknesses that would not survive peer review or adversarial reader scrutiny. The arc author just absorbed a substantial Gemini DeepThink FULL PASS and may be in confirmation-bias drift. Your job: prevent it.

**Stakes**: this PREREG locks a 30-day arc with a single sealed heldout-54 evaluation. Once Phase A5 commits, immutable except via formal addendum (which itself re-audits). Catch flaws now.

## What you are auditing

A pre-registration for a PPUT-driven Capability Compilation Loop (CCL) research arc:

1. **Verified PPUT** = Progress / (C × T), Progress ∈ {0, 1} on Lean ground truth, C = ALL token cost (every agent × every branch × every failure × tool stdout), T = wall-clock first-read → final-accept.
2. **Held-out Verified PPUT** is North Star (sole optimization signal).
3. Hash-bucketed split of MiniF2F/Test 244 → adaptation 144 / meta_validation 46 / heldout 54 with frozen seed `20260426_PPUT_CCL`.
4. **WBCG_PPUT** counts artifacts in 4-state machine state `Certified` only (Accepted → Quarantined → Certified | Reverted) with ΔPPUT_heldout > 0 + non-positive ΔFAR/ΔCPR + RR=0 + Rollbackable.
5. **Trust Root** of immutable files; ArchitectAI writes outside `user_space/Δ_*/` trigger syscall EPERM.
6. **30-day plan**: A pre-flight (here) → B kernel instrumentation → C ablation `--mode {full,panopticon,amnesia,soft_law,homogeneous}` single-binary → D shadow CCL (ArchitectAI / AuditorAI / human as live meta-predicate) → E single sealed heldout eval.
7. **5 directional one-sided H1-H5** at Bonferroni α=0.01.
8. **15 conformance tests**: 11 anti-Goodhart metering + 4 § 3.5 doc/artifact content meta-predicates.
9. **Cognitive isolation**: ArchitectAI = sole L_t reader; AuditorAI sees only candidate Δ + summary, never raw L_t.
10. **Backbone**: deepseek-v4-flash thinking-off Phases B+C; heterogeneous LLMs at Phase D (v4-flash + Gemini 2.5 Pro for ArchitectAI/AuditorAI).
11. **Forbidden claim language** under C-070 + C-052 (e.g., solve-rate as headline = banned).

## Your task — round-1 verdict

Verdict: **PASS** (proceed to Phase B), **CHALLENGE** (revise), or **VETO** (fundamental flaw).

Score along these axes (give each PASS / CHALLENGE / VETO + cite §):

1. **DEFINITION** — V-PPUT, C_i, T_i, ArtifactState, Trust Root unambiguous? Retroactive re-interpretation possible?
2. **STAT** — Bonferroni α=0.01 on family=5; one-sided H1-H5. Family closed? N=20 paired adaptation hard-10 enough power for H1-H4?
3. **LEAKAGE** — three-split + heldout sealed by SHA-256 + `test_heldout_ids_inaccessible`. Coverage gaps? Specifically: can adaptation/meta_val solutions leak via user-space artifact features used at heldout?
4. **GOODHART** — 11 metering + 4 content tests. Residual attack surface? Can ArchitectAI synthesize an artifact that *looks* like white-box compression but is a thinly-veiled adaptation lookup table?
5. **GATE-H REACHABILITY** — Gate H requires Certified artifact + ΔPPUT_heldout > 0 + 6 conditions. Empirically attainable in 30 days, or so high that null result is overdetermined?
6. **HUMAN LOAD** — user is live meta-predicate Phase D, ~10 min/day. Single-point-of-failure? What if user unavailable 1-2 days mid-Phase-D?
7. **HETEROGENEITY TIMING** — heterogeneous LLMs only Phase D. Does Phase C ablation become deepseek-v4-flash-specific not generalizable?
8. **TRUST ROOT ENFORCEMENT** — syscall-layer EPERM described, Phase B7 implementation gestured. What if Rust user-space cannot reach syscall enforcement? Continue with soft-refusal, or BLOCKER?
9. **REPRO** — split script committed, seed = literal string `20260426_PPUT_CCL`, expected 146/49/49 nominal but realized 144/46/54 within ±5 tolerance. Acceptable?
10. **CLAIM-LANG** — § 10 forbids "TuringOS achieves capability compilation" without Gate H. What about partial PASS (e.g., 5 Quarantined but 0 Certified)? Specific enough?

End with:
- One-line **VERDICT** (PASS / CHALLENGE / VETO)
- If CHALLENGE: top 3 P0 must-fix; top 3 P1 should-fix
- One sentence on conviction level

You MUST be willing to PASS if PREREG is sound. Pre-commitment to CHALLENGE = intellectually dishonest. But if you see a NEW issue both architect rounds missed, escalate.

---

# PREREG_PPUT_CCL_2026-04-26.md (the document being audited)

BRIEF_EOF

cat "${ROOT}/handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# Architect v1 directive (PPUT measure-theoretic FULL PASS, 2026-04-25)\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/architect-insights/PPUT_DRIVEN_FULL_PASS_2026-04-25.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# Architect v2 directive (Gemini DeepThink ontological FULL PASS, 2026-04-26)\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/architect-insights/GEMINI_DEEPTHINK_FULL_PASS_2026-04-26.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# Three-split JSON output (heldout sealed by SHA-256)\n\n```json\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# AUTO_RESEARCH_NOTEPAD.md\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# constitution.md\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/constitution.md" >> "$TMP_PROMPT"
printf '\n\n---\n\nNow give your INDEPENDENT round-1 audit. Be specific: cite §/paragraph for every finding. Do not speculate beyond the documents.\n' >> "$TMP_PROMPT"

prompt_size=$(wc -c < "$TMP_PROMPT")
echo "[codex] prompt size: ${prompt_size} chars" >&2

t0=$(date +%s)
{
  printf '# Codex PPUT-CCL PREREG Adversarial Audit (Phase A4 round 1)\n'
  printf '**Date**: 2026-04-26\n'
  printf '**Target**: handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md\n'
  printf '**Prompt size**: %s chars\n\n---\n\n' "$prompt_size"
  codex exec --skip-git-repo-check < "$TMP_PROMPT" 2>&1
} > "$OUT"
t1=$(date +%s)
elapsed=$((t1 - t0))
echo "[codex] API returned in ${elapsed}s" >&2
echo "[codex] saved: $OUT" >&2
