#!/usr/bin/env python3
"""Gemini adversarial review of PREREG_PPUT_CCL_2026-04-26 ROUND 2 (Phase A4 round 2).

Round 1 (this script's predecessor `run_gemini_pput_ccl_audit.py`):
  Gemini → CHALLENGE; Codex → CHALLENGE; merged → CHALLENGE per VETO>CHALLENGE>PASS.

Round 2: 10 fixes applied (M1-M7 + H1-H2 + TR). Same auditor instance asked
to revisit; brief explicitly cites round-1 P0 items and asks if each is closed.
"""
import os, sys, json, urllib.request, urllib.error, time
from pathlib import Path

ROOT = Path("/home/zephryj/projects/turingosv4")
V3_ENV = Path("/home/zephryj/projects/turingosv3/.env")

env = {}
for line in V3_ENV.read_text().splitlines():
    if "=" in line and not line.strip().startswith("#"):
        k, v = line.split("=", 1)
        env[k.strip()] = v.strip().strip('"').strip("'")

brief = """# Gemini Adversarial Audit — PREREG_PPUT_CCL_2026-04-26 (Phase A4 ROUND 2)

**Role**: skeptical adversarial reviewer for an ICLR/NeurIPS Systems track. Independent of Codex (which is auditing in parallel; do NOT read CODEX_PPUT_CCL_AUDIT_*.md).

**Mandate**: this is round 2. Round 1 (your prior independent assessment) returned **CHALLENGE**, citing 3 P0 items: (M-stat) family-of-5 omits per-artifact guardrail tests; (M-gate-h) RR=0 too brittle; (M-spof) human-as-live-meta-predicate single-point-of-failure without contingency. Codex independently CHALLENGEd, citing 3 P0s: (M-leak) heldout sealing is operational not cryptographic; (M-measure) Gate H per-artifact measurement underdefined under single sealed eval; (M-fam) statistical family closure incomplete (this overlapped with your M-stat from a different angle). Conservative merge → CHALLENGE.

**This round**: the author claims to have applied all 10 round-1 fixes (7 P0s from union of both auditors + 3 P1s elevated to P0 per conservative rule + housekeeping). The PREREG has a `## changelog` section listing each fix as a row.

**Your task — round 2 verdict**: read the revised PREREG. For EACH of the round-1 P0 findings, judge whether the round-2 revision closes it (`closed`), partially closes it (`partial`), or leaves it open (`still-open`). Then issue an overall **PASS / CHALLENGE / VETO**.

## Round-1 P0 items to check closure on

### From your round-1 audit:

**P0-stat (your axis 2, family-of-5 omits per-artifact guardrails)**:
- Round-1 finding: § 9 declared family=5 but per-artifact ΔFAR/RR/ΔCPR were inferential too, missing from family.
- Round-2 claim: § 9.1-9.5 rewritten — family = 4 (Phase C) + 4·k (Phase E); Holm-Bonferroni stepwise procedure; H5 reclassified as deterministic gate (not inferential); independent unit = (problem, seed); power expectation calibrated for hard-10 × N=20.
- **Your check**: does this fully close the multiplicity gap? Are the test directions / endpoints unambiguous? Does Holm-Bonferroni at family-wise α=0.05 give acceptable FWER for the realistic k ≤ 10 range?

**P0-gate-h (your axis 5, RR=0 brittleness)**:
- Round-1 finding: Gate H requires RR=0 on heldout-54; even one stochastic edge case kills artifact; null result overdetermined.
- Round-2 claim: § 7 Gate H, j-RR test relaxed to per-problem one-sided exact binomial vs pre-registered baseline regression rate `p_0` calibrated in Phase B6/B7 on adaptation set.
- **Your check**: is `p_0` calibration protocol specified clearly enough that it cannot be tuned post-hoc? Is the binomial test specified completely (n, sample size, α taken from family)?

**P0-spof (your axis 6, human meta-predicate single-point-of-failure)**:
- Round-1 finding: § 6 D5 user reviews each candidate; no contingency for unavailability.
- Round-2 claim: § 6 D5 added 48h SLA, deferred queue, 30-day clock no-pause, ≥5 queued >48h triggers Phase D early abort.
- **Your check**: is the contingency complete? What if user is partially available (e.g., 1 review per day vs 5)? Is the abort threshold (≥5 queued) calibrated reasonably given expected Phase D throughput?

### From Codex's round-1 audit (you should evaluate independently):

**P0-leak (Codex axis 3, operational vs cryptographic sealing)**:
- Round-1 finding: heldout IDs in cleartext in repo + public seed = "sealed" misleading.
- Round-2 claim: § 2.3 reframed — "operational sealing" explicit; SHA-256 = integrity check not access control; new context-blacklist; new tool-call EPERM filter; residual paraphrasing risk acknowledged.
- **Your check**: is the reframe honest? Does the conformance battery actually prevent heldout from leaking into agent prompt context, given that ArchitectAI is the sole L_t reader and human user reviews artifacts (could user accidentally paste heldout context in)?

**P0-measure (Codex axis 5, Gate H per-artifact measurement)**:
- Round-1 finding: § 6 E2-E3 single sealed eval + per-artifact ΔPPUT contradicted; protocol unclear.
- Round-2 claim: § 6 E2 specifies leave-one-out within sealed eval — k+2 sub-evals on SAME 54 problems × 3 seeds (BASELINE + ALL + k LOO); per-artifact Δ_j = PPUT_all - PPUT_minus_j.
- **Your check**: is the LOO protocol valid statistically? Does it preserve single-touch discipline on heldout? Are the k+2 sub-evals counting as independent draws or as paired observations on the same sample (and is the multiplicity correction handling that correctly)?

## Your task — round 2 verdict format

For each P0 above, answer: **closed / partial / still-open + 1-sentence reason citing the revised PREREG paragraph**.

Then end with:

- **VERDICT**: PASS / CHALLENGE / VETO
- If CHALLENGE: top 3 still-open or new P0 items
- One sentence on conviction
- One sentence: are you concerned about any NEW issues introduced by the round-2 revisions themselves (e.g., Holm-Bonferroni introducing new analyst-degree-of-freedom; LOO protocol over-counting sample independence)?

You MUST be willing to PASS if the round-2 revisions genuinely close the round-1 blockers without introducing new ones. Pre-commitment to CHALLENGE = intellectually dishonest. But do not lower the bar.

---

# PREREG_PPUT_CCL_2026-04-26.md (round 2 — the document being audited)

"""

prereg = (ROOT / "handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md").read_text()
architect_v1 = (ROOT / "handover/architect-insights/PPUT_DRIVEN_FULL_PASS_2026-04-25.md").read_text()
architect_v2 = (ROOT / "handover/architect-insights/GEMINI_DEEPTHINK_FULL_PASS_2026-04-26.md").read_text()
splits_json = (ROOT / "handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json").read_text()
round1_verdict = (ROOT / "handover/audits/DUAL_AUDIT_PPUT_CCL_VERDICT_2026-04-26.md").read_text()
round1_gemini = (ROOT / "handover/audits/GEMINI_PPUT_CCL_AUDIT_2026-04-26.md").read_text()

full_prompt = (
    brief +
    prereg +
    "\n\n---\n\n# Architect v1 directive (PPUT measure-theoretic FULL PASS, 2026-04-25)\n\n" + architect_v1 +
    "\n\n---\n\n# Architect v2 directive (Gemini DeepThink ontological FULL PASS, 2026-04-26)\n\n" + architect_v2 +
    "\n\n---\n\n# Three-split JSON output\n\n```json\n" + splits_json + "\n```\n" +
    "\n\n---\n\n# Round-1 merged verdict + your round-1 audit (for cross-reference, not for re-arguing)\n\n" + round1_verdict +
    "\n\n---\n\n# Your round-1 audit (verbatim)\n\n" + round1_gemini +
    "\n\n---\n\nNow give your INDEPENDENT round-2 audit. Cite §/paragraph for every finding. Strictly evaluate closure of round-1 P0s + check for new round-2-introduced issues."
)

print(f"[gemini round2] prompt size: {len(full_prompt):,} chars", file=sys.stderr)

url = f"https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-pro:generateContent?key={env['GEMINI_API_KEY']}"
body = json.dumps({
    "contents": [{"parts": [{"text": full_prompt}]}],
    "generationConfig": {"temperature": 0.2, "maxOutputTokens": 16384},
}).encode()
headers = {"Content-Type": "application/json"}

t0 = time.time()
req = urllib.request.Request(url, data=body, headers=headers, method="POST")
try:
    with urllib.request.urlopen(req, timeout=900) as resp:
        data = json.loads(resp.read())
except urllib.error.HTTPError as e:
    print(f"[gemini round2] HTTP {e.code}: {e.read().decode(errors='replace')[:2000]}", file=sys.stderr)
    sys.exit(1)

elapsed = time.time() - t0
print(f"[gemini round2] API returned in {elapsed:.1f}s", file=sys.stderr)

text = data["candidates"][0]["content"]["parts"][0]["text"]
out = ROOT / "handover/audits/GEMINI_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md"
header = (f"# Gemini PPUT-CCL PREREG Adversarial Audit (Phase A4 ROUND 2)\n"
          f"**Date**: 2026-04-26\n"
          f"**Target**: handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md (round 2)\n"
          f"**Elapsed**: {elapsed:.1f}s\n"
          f"**Prompt size**: {len(full_prompt):,} chars\n\n---\n\n")
out.write_text(header + text)
print(f"[gemini round2] saved: {out}")
