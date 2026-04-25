#!/usr/bin/env python3
"""Gemini adversarial review of Paper 1 v2.1 draft (round-3 dual-audit).

Round-1 (v1, commit 2687882):       CHALLENGE / CHALLENGE
Round-2 (v2,  commit 210f19b):      CHALLENGE / CHALLENGE
This run (v2.1, commit d349a86):    round-3 — does the v2.1 revision close
the round-2 P0 blockers, or is another revision needed before arXiv?
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

brief = """# Gemini Adversarial Audit — Paper 1 v2.1 Draft (Round 3)

**Role**: skeptical reviewer for ICLR/NeurIPS Systems/Reliability track.
**Mandate**: find claims that would not survive peer review or adversarial reader scrutiny. No sycophancy. Independent verdict; do NOT confer with Codex.

## Audit history

- **Round 1** (Paper 1 v1, commit `2687882`): your verdict was CHALLENGE; Codex independently CHALLENGE. Merged → CHALLENGE.
- **Round 2** (Paper 1 v2, commit `210f19b`): your verdict was CHALLENGE; Codex independently CHALLENGE. Merged → CHALLENGE.
  - Round-2 verdict + your major catch (mathd_algebra_246 model drift) is included below in full.

## What v2.1 changed (commit `d349a86`)

The author applied **all round-2 P0 blockers (A–E)** + P1-B (easy-set demote) + P1-C (causal sharpening) + P2-A (stale footer). Specifically:

1. **DRIFT (P0-A)**: Added § 4.7 documenting the `mathd_algebra_246` drift event and a drift-robust restatement of Table 4.1 over the 9 truly-hard problems (8/36 vs 0/36, McNemar p=0.00391 unchanged).
2. **CLAIM (P0-B + P1-C)**: Cut "generic prompt heterogeneity" from abstract + § 4.1 + § 6.1; reframed as "portfolio of prompts including one meta-cognitive instruction." Added Limitation 12 explicitly labeling the meta-vs-object-level confound.
3. **HEADLINE (P0-C)**: Cut "tripled absolute solve count (3× from 4 to 12)." Replaced with "B solves 4 distinct hard problems A never solved; A solves 0 hard problems B never solved; 8/8 discordant pairs favor B."
4. **FAMILY (P0-D)**: Reconciled pre-reg family-of-4 vs executed-tests inconsistency. Closed confirmatory family at 3 hard-set McNemar tests (B-vs-A, Abl-vs-A, B-vs-Abl); demoted easy-set + per-seed to descriptive with transparent post-hoc labeling note in § 3.6.
5. **EASY-SET (P0-D + P1-B)**: Demoted v1 easy-set out of inferential family (§ 4.3) since not re-stamped with BUILD_SHA `29ab43a`; retained as descriptive historical control.
6. **ARTIFACTS (P0-E)**: Copied all 12 E1 v2 jsonl into stable `handover/evidence/v2/` (out of `.claude/worktrees/`); updated § 8.5 with explicit file list.
7. **STATUS (P2-A)**: Removed stale "data collection ~50% complete" footer.

**Explicitly DEFERRED to v2.2 (not gating round-3)**:
- P1-A: problem-cluster sensitivity analysis (cluster-bootstrap or mixed logistic)
- P1-D: per-condition token-budget table
- P1-E: Docker build/run transcript
- P2-B: Appendix C node-count + winning-agent extraction

## Your task — round-3 verdict

Independent verdict on v2.1: **PASS** (arXiv-ready), **CHALLENGE** (revision needed), or **VETO** (fundamental flaw).

Score along the same 6 axes you used in round-2:
1. **STAT** — power, multiplicity, exact-test assumptions, family closure
2. **DESIGN** — sample construction, negative controls, ablation scope
3. **CAUSE** — is the new "portfolio of prompts including one meta-cognitive" framing defensible
4. **LEAKAGE** — token budget, content fairness between A/Abl/B
5. **REPRO** — pre-reg discipline, artifact stability, BUILD_SHA enforcement
6. **CLAIM** — does the v2.1 abstract still overclaim

For each axis, judge: **closed by v2.1**, **partially closed**, or **still open**. For "still open" items, specify whether the residual is:
- a P0 (must fix before arXiv) — would block submission as a reviewer
- a P1 (should fix, but you would not block) — can be addressed in v2.2 + final-author-version
- a P2 (nice-to-have)

End with:
- One-line **VERDICT** (PASS / CHALLENGE / VETO)
- Top 3 **must-fix** items if CHALLENGE
- One sentence on whether the v2.2 deferred items (P1-A cluster, P1-D token table, P1-E Docker, P2-B Appendix C) should be promoted to round-3 P0 status

You MUST be willing to PASS if the v2.1 work genuinely closed the round-2 blockers. Pre-commitment to CHALLENGE would be intellectually dishonest. But if you see a NEW issue v1+v2 missed, escalate it as a fresh blocker.

---

# Paper 1 v2.1 Full Draft

"""

paper_draft = (ROOT / "handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md").read_text()
prereg = (ROOT / "handover/preregistration/PREREG_E1V2_HETEROGENEITY_2026-04-23.md").read_text()
proxy_finding = (ROOT / "handover/preregistration/PROXY_SATURATION_FINDING_2026-04-24.md").read_text()
results_json = (ROOT / "handover/preregistration/E1v2_RESULTS_2026-04-24.json").read_text()
round2_verdict = (ROOT / "handover/audits/DUAL_AUDIT_V2_VERDICT_2026-04-24.md").read_text()
round2_codex = (ROOT / "handover/audits/CODEX_PAPER1_V2_AUDIT_2026-04-24.md").read_text()
round2_gemini = (ROOT / "handover/audits/GEMINI_PAPER1_V2_AUDIT_2026-04-24.md").read_text()

full_prompt = (
    brief +
    paper_draft +
    "\n\n---\n\n# Pre-Registration (committed BEFORE run)\n\n" + prereg +
    "\n\n---\n\n# Proxy-Saturation Incident Log\n\n" + proxy_finding +
    "\n\n---\n\n# Aggregated 4-Seed Results JSON\n\n```json\n" + results_json + "\n```\n" +
    "\n\n---\n\n# Round-2 Merged Verdict\n\n" + round2_verdict +
    "\n\n---\n\n# Round-2 Codex Audit (for reference)\n\n" + round2_codex +
    "\n\n---\n\n# Round-2 Gemini Audit (your prior independent assessment)\n\n" + round2_gemini +
    "\n\n---\n\nNow give your INDEPENDENT round-3 audit. Did v2.1 close round-2's P0 blockers? "
    "Be specific: cite the v2.1 paragraph that closes (or fails to close) each round-2 P0 item."
)

print(f"[gemini] prompt size: {len(full_prompt):,} chars", file=sys.stderr)

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
    print(f"[gemini] HTTP {e.code}: {e.read().decode(errors='replace')[:2000]}", file=sys.stderr)
    sys.exit(1)

elapsed = time.time() - t0
print(f"[gemini] API returned in {elapsed:.1f}s", file=sys.stderr)

text = data["candidates"][0]["content"]["parts"][0]["text"]
out = ROOT / "handover/audits/GEMINI_PAPER1_V2_1_AUDIT_2026-04-25.md"
header = (f"# Gemini Paper 1 v2.1 Adversarial Audit (round 3)\n"
          f"**Date**: 2026-04-25\n"
          f"**Target commit**: d349a86\n"
          f"**Elapsed**: {elapsed:.1f}s\n"
          f"**Prompt size**: {len(full_prompt):,} chars\n\n---\n\n")
out.write_text(header + text)
print(f"[gemini] saved: {out}")
