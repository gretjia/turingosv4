#!/usr/bin/env python3
"""Gemini adversarial review of Paper 1 draft (dual-audit per CLAUDE.md)."""
import os, sys, json, urllib.request, urllib.error, time
from pathlib import Path

ROOT = Path("/home/zephryj/projects/turingosv4")
V3_ENV = Path("/home/zephryj/projects/turingosv3/.env")

env = {}
for line in V3_ENV.read_text().splitlines():
    if "=" in line and not line.strip().startswith("#"):
        k, v = line.split("=", 1)
        env[k.strip()] = v.strip().strip('"').strip("'")

brief = """# Gemini Adversarial Audit — Paper 1 Draft

**Role**: skeptical reviewer for ICLR/NeurIPS Systems/Reliability track.
**Mandate**: find claims that would not survive peer review or adversarial reader scrutiny. No sycophancy.
**Context**: TuringOS v4 solo-researcher preprint. Primary claim: prompt-only skill heterogeneity (incl Meta-Planner role) elicits swarm intelligence emergence on MiniF2F Lean 4 hard problems. 4-seed paired A/B with McNemar p=0.0195 (one-sided), easy-set negative control Δ=0, ablation removing Meta-Planner.

**Your task**: emit a structured critique covering exactly these 6 categories. Be specific (§ reference or file:line); no vague complaints.

1. **Statistical challenges (STAT)**
2. **Experimental-design challenges (DESIGN)**
3. **Causal-attribution challenges (CAUSE)**
4. **Prompt-leakage challenges (LEAKAGE)**
5. **Reproducibility challenges (REPRO)**
6. **Claim-strength challenges (CLAIM)**

Produce a markdown report. End with:
- One-line VERDICT (PASS / CHALLENGE / VETO)
- Table of required changes before submission (Priority × Category × Change × Rationale)
- Top 3 must-fix items, one specific claim you would cut

You are doing this INDEPENDENTLY — do NOT read the Codex audit (if attached). Give your own verdict.

---

# Paper 1 Full Draft

"""

paper_draft = (ROOT / "handover/ai-direct/PAPER_1_FULL_DRAFT_2026-04-23.md").read_text()
final_verdict = (ROOT / "handover/ai-direct/E1_FINAL_4SEEDS_2026-04-23.md").read_text()
internal_audit = (ROOT / "handover/evidence/ADVERSARIAL_AUDIT_2026-04-23.md").read_text()

# Sample evidence (small files)
sample_hard = (ROOT / "handover/evidence/sample_E1_hard10.txt").read_text()
sample_easy = (ROOT / "handover/evidence/sample_E1_easy10.txt").read_text()

full_prompt = (
    brief +
    paper_draft +
    "\n\n---\n\n# E1 Final 4-Seed Verdict\n\n" + final_verdict +
    "\n\n---\n\n# Our Own Adversarial Audit (for your context)\n\n" + internal_audit +
    "\n\n---\n\n# Hard-10 sample\n\n" + sample_hard +
    "\n\n---\n\n# Easy-10 sample\n\n" + sample_easy +
    "\n\n---\n\nNow give your INDEPENDENT audit. Do NOT defer to our internal audit."
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
out = ROOT / "handover/audits/GEMINI_PAPER1_AUDIT_2026-04-23.md"
header = (f"# Gemini Paper 1 Adversarial Audit\n"
          f"**Date**: 2026-04-23\n"
          f"**Elapsed**: {elapsed:.1f}s\n"
          f"**Prompt size**: {len(full_prompt):,} chars\n"
          f"**Target commit**: main@f7918a7\n\n---\n\n")
out.write_text(header + text)
print(f"[gemini] saved: {out}")
