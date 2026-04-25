#!/usr/bin/env bash
# Codex adversarial audit of PREREG_PPUT_CCL_2026-04-26 ROUND 2 (Phase A4 round 2).
#
# Round 1 (this script's predecessor): Codex CHALLENGE, Gemini CHALLENGE, merged CHALLENGE.
# Round 2: 10 fixes applied. Re-audit explicitly cites round-1 P0s and asks for closure check.
#
# Output: handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md
set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
OUT="${ROOT}/handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md"
TMP_PROMPT="$(mktemp /tmp/pput_ccl_codex_round2.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Codex Adversarial Audit — PREREG_PPUT_CCL_2026-04-26 (Phase A4 ROUND 2)

**Role**: skeptical adversarial reviewer. Independent of Gemini round-2 (do NOT read GEMINI_PPUT_CCL_AUDIT_ROUND2_*.md if it exists).

**Mandate**: this is round 2. In round 1 you returned **CHALLENGE** with 3 P0 items: (M-leak) heldout sealing operational not cryptographic; (M-measure) Gate H per-artifact measurement underdefined under single sealed eval; (M-fam) statistical family closure. Gemini independently CHALLENGEd with 3 P0s of its own: (M-stat) per-artifact guardrail tests omitted from family; (M-gate-h) RR=0 brittleness; (M-spof) human-as-meta-predicate SPOF. Conservative merge → CHALLENGE.

**This round**: the author claims to have applied all 10 round-1 fixes. The PREREG has a `## changelog` section listing each fix with cross-references to the auditor that flagged it.

**Your task — round 2 verdict**: read the revised PREREG. For EACH of YOUR round-1 P0 findings, judge whether the round-2 revision closes it. Then issue an overall **PASS / CHALLENGE / VETO**.

## Round-1 P0 items YOU raised (judge closure)

**P0-leak (your axis 3, LEAKAGE)**:
- Round-1: split JSON has raw heldout IDs in cleartext; seed + bucketing rule public; "sealed by SHA-256" misleading.
- Round-2 claim: § 2.3 reframed — "operational sealing"; SHA-256 = integrity check; new context-blacklist for PREREG/SPLITS files at agent prompt construction; new tool-call EPERM filter for path-blacklist; residual paraphrasing risk acknowledged.
- **Your check**: is the reframing honest? Does the conformance battery ACTUALLY prevent leakage? Specifically: can ArchitectAI infer heldout via the public seed string by computing buckets internally without reading any blacklisted file?

**P0-measure (your axis 5, GATE-H REACHABILITY procedural underdefinition)**:
- Round-1: § 6 E2 said "single sealed eval" but § 6 E3 said "per-artifact ΔPPUT" — protocol unclear.
- Round-2 claim: § 6 E2 specifies leave-one-out within sealed eval. k+2 sub-evals on SAME 54 problems × 3 seeds (BASELINE + ALL + k LOO). Per-artifact Δ_j = PPUT_all - PPUT_minus_j.
- **Your check**: is the LOO protocol clean? Compute budget claim of 27 wall-hours at k=10 — verify. Does "single touch" hold (no iteration based on results)? Is the artifact factorial interaction handled (LOO assumes additive contribution; what if artifacts interact)?

**P0-fam (your axis 2, STAT family closure)**:
- Round-1: family=5 declared but H1-H4 had multiple endpoints each; H5 was a deterministic gate not a test; independent unit not specified for hard-10 × N=20.
- Round-2 claim: § 5 restructured — each H1-H4 has ONE primary endpoint; H5 reclassified as deterministic gate; § 5.2 added per-artifact heldout family `4·k`; family total `4 + 4k`; § 6 C2 independent unit = (problem, seed); § 9 Holm-Bonferroni stepwise.
- **Your check**: is the family closure now complete? Is the (problem, seed) unit validly independent (or are seeds correlated within problem)? Holm-Bonferroni stepwise — is the procedure unambiguous when k is data-dependent (Phase D output)?

## Round-1 P0 items GEMINI raised (you should evaluate independently)

**P0-stat-gemini (Gemini axis 2, family-of-5 omits per-artifact guardrails)** — convergent with your P0-fam from a different angle. Round-2 claim covers it.

**P0-gate-h (Gemini axis 5, RR=0 brittleness)**:
- Round-2 claim: § 7 Gate H, j-RR test relaxed to per-problem one-sided exact binomial vs pre-registered baseline regression rate `p_0` calibrated in Phase B6/B7 on adaptation set.
- **Your check**: is `p_0` calibration protocol specified clearly enough that it cannot be tuned post-hoc to be artificially high (auto-passing test)? When is `p_0` frozen, and what's the audit trail?

**P0-spof (Gemini axis 6, human meta-predicate SPOF)**:
- Round-2 claim: 48h SLA + deferred queue + 30-day clock no-pause + ≥5-queued-48h triggers Phase D early abort.
- **Your check**: is contingency complete? Edge cases (partial availability)?

## Verdict format

For each P0 above: **closed / partial / still-open + 1-sentence reason citing § / paragraph**.

Then:
- **VERDICT**: PASS / CHALLENGE / VETO
- If CHALLENGE: top 3 still-open OR new P0
- Conviction
- Any NEW issues introduced by round-2 revisions (e.g., Holm-Bonferroni dependence on k introducing analyst degree of freedom; LOO factorial assumption; baseline `p_0` calibration timing creating new attack surface)

You MUST PASS if round-2 revisions genuinely close round-1 blockers without introducing new ones. Lock in ICLR/NeurIPS adversarial-review rigor.

---

# PREREG_PPUT_CCL_2026-04-26.md (round 2 — the document being audited)

BRIEF_EOF

cat "${ROOT}/handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# Architect v1 directive (PPUT measure-theoretic FULL PASS, 2026-04-25)\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/architect-insights/PPUT_DRIVEN_FULL_PASS_2026-04-25.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# Architect v2 directive (Gemini DeepThink ontological FULL PASS, 2026-04-26)\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/architect-insights/GEMINI_DEEPTHINK_FULL_PASS_2026-04-26.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# Three-split JSON output\n\n```json\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# Round-1 merged verdict (for cross-reference)\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/audits/DUAL_AUDIT_PPUT_CCL_VERDICT_2026-04-26.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# Your round-1 audit (verbatim, do not re-argue closed items)\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md" >> "$TMP_PROMPT"
printf '\n\n---\n\nNow give your INDEPENDENT round-2 audit. Strictly evaluate closure of round-1 P0s + check for new round-2-introduced issues. Cite § for every finding.\n' >> "$TMP_PROMPT"

prompt_size=$(wc -c < "$TMP_PROMPT")
echo "[codex round2] prompt size: ${prompt_size} chars" >&2

t0=$(date +%s)
{
  printf '# Codex PPUT-CCL PREREG Adversarial Audit (Phase A4 ROUND 2)\n'
  printf '**Date**: 2026-04-26\n'
  printf '**Target**: handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md (round 2)\n'
  printf '**Prompt size**: %s chars\n\n---\n\n' "$prompt_size"
  codex exec --skip-git-repo-check < "$TMP_PROMPT" 2>&1
} > "$OUT"
t1=$(date +%s)
elapsed=$((t1 - t0))
echo "[codex round2] API returned in ${elapsed}s" >&2
echo "[codex round2] saved: $OUT" >&2
