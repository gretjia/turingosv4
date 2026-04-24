#!/usr/bin/env python3
"""Gemini adversarial review of Paper 1 v2 draft (round-2 dual-audit)."""
import os, sys, json, urllib.request, urllib.error, time
from pathlib import Path

ROOT = Path("/home/zephryj/projects/turingosv4")
V3_ENV = Path("/home/zephryj/projects/turingosv3/.env")

env = {}
for line in V3_ENV.read_text().splitlines():
    if "=" in line and not line.strip().startswith("#"):
        k, v = line.split("=", 1)
        env[k.strip()] = v.strip().strip('"').strip("'")

brief = """# Gemini Adversarial Audit — Paper 1 v2 Draft (Round 2)

**Role**: skeptical reviewer for ICLR/NeurIPS Systems/Reliability track.
**Mandate**: find claims that would not survive peer review or adversarial reader scrutiny. No sycophancy.

**Context**: TuringOS v4 solo-researcher preprint, **v2 revision** after round-1 dual-audit returned CHALLENGE.
Key changes in v2 vs v1:
- Reframed as 'prompt-heterogeneity portfolio effect', NOT 'swarm emergence'
- Pre-registered sample + seeds + multiplicity (committed BEFORE run)
- Bonferroni α=0.0125 for family-of-4 declared
- Both one-sided AND two-sided McNemar reported
- Meta-Planner mechanism demoted from 'is the mechanism' to 'highly seed-dependent contribution, aggregate positive'
- Proxy-saturation incident disclosed (serial re-run with max 2 parallel batches)
- N=4 seeds for the ablation (not N=1)
- BUILD_SHA fail-fast stamping added to every jsonl

**Your task**: emit a structured critique covering exactly these 6 categories. Be specific (§ reference or file:line); no vague complaints. Focus on what's still weak AFTER v2 fixes.

1. **Statistical challenges (STAT)** — power, multiplicity, exact-test assumptions
2. **Experimental-design challenges (DESIGN)** — sample construction, negative controls, ablation scope
3. **Causal-attribution challenges (CAUSE)** — is the portfolio framing defensible, are mechanism claims now appropriately scoped
4. **Prompt-leakage / symmetry challenges (LEAKAGE)** — token budget, content fairness between A and B
5. **Reproducibility challenges (REPRO)** — is pre-reg discipline honest, are artifact links sufficient
6. **Claim-strength challenges (CLAIM)** — does the abstract/introduction overclaim the results

Produce a markdown report. End with:
- One-line VERDICT (PASS / CHALLENGE / VETO)
- Table of required changes before submission (Priority × Category × Change × Rationale)
- Top 3 must-fix items, one specific claim you would cut

You are doing this INDEPENDENTLY — give your own verdict without conferring with Codex.

---

# Paper 1 v2 Full Draft

"""

paper_draft = (ROOT / "handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md").read_text()
proxy_finding = (ROOT / "handover/preregistration/PROXY_SATURATION_FINDING_2026-04-24.md").read_text()
prereg = (ROOT / "handover/preregistration/PREREG_E1V2_HETEROGENEITY_2026-04-23.md").read_text()
results_json = (ROOT / "handover/preregistration/E1v2_RESULTS_2026-04-24.json").read_text() \
    if (ROOT / "handover/preregistration/E1v2_RESULTS_2026-04-24.json").exists() else "(results JSON not yet generated)"

full_prompt = (
    brief +
    paper_draft +
    "\n\n---\n\n# Pre-Registration (committed BEFORE run)\n\n" + prereg +
    "\n\n---\n\n# Proxy-Saturation Incident Log\n\n" + proxy_finding +
    "\n\n---\n\n# Aggregated 4-Seed Results JSON\n\n```json\n" + results_json + "\n```\n"
    "\n\n---\n\nNow give your INDEPENDENT round-2 audit. Focus on residual weaknesses after v2 fixes."
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
out = ROOT / "handover/audits/GEMINI_PAPER1_V2_AUDIT_2026-04-24.md"
header = (f"# Gemini Paper 1 v2 Adversarial Audit (round 2)\n"
          f"**Date**: 2026-04-24\n"
          f"**Elapsed**: {elapsed:.1f}s\n"
          f"**Prompt size**: {len(full_prompt):,} chars\n\n---\n\n")
out.write_text(header + text)
print(f"[gemini] saved: {out}")
