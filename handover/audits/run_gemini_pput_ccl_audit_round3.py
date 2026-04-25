#!/usr/bin/env python3
"""Gemini adversarial review of PREREG_PPUT_CCL_2026-04-26 ROUND 3.

Round 1: Gemini CHALLENGE / Codex CHALLENGE → CHALLENGE.
Round 2: Gemini PASS / Codex CHALLENGE → CHALLENGE.
Round 3: 3 Codex round-2 P0 fixes applied (R3-1 family timing + per-problem unit;
         R3-2 p_0 calibration protocol; R3-3 5-layer operational sealing) + R3-4
         marginal-contribution claim caveat. Re-audit asks: closure + new issues.
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

brief = """# Gemini Adversarial Audit — PREREG_PPUT_CCL_2026-04-26 (Phase A4 ROUND 3)

**Role**: skeptical adversarial reviewer for an ICLR/NeurIPS Systems track. Independent of Codex.

**Mandate**: round 3. Round 1 = Gemini CHALLENGE + Codex CHALLENGE → merged CHALLENGE. Round 2 = Gemini PASS + Codex CHALLENGE → merged CHALLENGE. Codex round-2 raised 3 still-open or new P0s:

- (P0-fam): Holm-Bonferroni N depended on final 4+4k, but k unknown at Phase C Gate C; (problem, seed) not independent unit.
- (P0-gate-h): p_0 baseline regression rate was a new tuning surface (no protocol, no freeze, no audit trail).
- (P0-leak): heldout sealing leaky under recomputation — agent could grep seed string from architect/audit files and run hashlib themselves.

**Round 3** applies 3 fixes (R3-1, R3-2, R3-3) plus a § 10 claim caveat (R3-4). The PREREG `## changelog` lists each.

**Your task — round 3 verdict**:

YOU passed round 2. The round-3 changes were prompted by Codex's still-open items. Your job: did the round-3 revisions hold your round-2 PASS, OR did they introduce something you should now CHALLENGE?

Specifically:
1. **R3-1 (family timing)**: Holm now uses fixed `N_max = 4 + 4·k_max = 44` regardless of realized k. Independent unit changed from (problem, seed) to per-problem (n=10 for hard-10). Power recomputed. Are these changes statistically valid? Any new issues?
2. **R3-2 (p_0 protocol)**: full toggle / sample / estimator / schedule / freeze point / audit trail / ceiling spec. Does this close the tuning-surface attack?
3. **R3-3 (sealing 5 layers)**: L1 file-path / L2 prompt-context / L3 tool-call args / L4 hashlib gate / L5 MiniF2F/Test ID blacklist. Is the sealing now operationally complete (within the publicly-acknowledged residual of git-history exposure)?
4. **R3-4 (claim caveat)**: § 10 row added stating per-artifact ΔPPUT from LOO is "marginal contribution given Q\\{j}", not standalone efficacy.

Also: **does the larger PREREG (now ~700 lines with 3 rounds of patches) still hang together internally**? Conflicts between sections? Inconsistencies between § 5 / § 9 / § 7? Anything that made sense in round 1 but is now contradicted?

End with:
- **VERDICT**: PASS / CHALLENGE / VETO
- If PASS: confirm specifically that all round-3 changes are sound and did not regress your round-2 PASS items.
- If CHALLENGE: top 3 items + cite §
- One sentence on conviction
- One sentence: are you concerned that this PREREG has gone through 3 revision rounds? Is that a sign of structural instability, or normal pre-registration tightening?

---

# PREREG_PPUT_CCL_2026-04-26.md (round 3)

"""

prereg = (ROOT / "handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md").read_text()
splits_json = (ROOT / "handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json").read_text()
round1_verdict = (ROOT / "handover/audits/DUAL_AUDIT_PPUT_CCL_VERDICT_2026-04-26.md").read_text()
round1_gemini = (ROOT / "handover/audits/GEMINI_PPUT_CCL_AUDIT_2026-04-26.md").read_text()
round2_gemini = (ROOT / "handover/audits/GEMINI_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md").read_text()
round2_codex = (ROOT / "handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md").read_text()

full_prompt = (
    brief +
    prereg +
    "\n\n---\n\n# Three-split JSON\n\n```json\n" + splits_json + "\n```\n" +
    "\n\n---\n\n# Round-1 merged verdict\n\n" + round1_verdict +
    "\n\n---\n\n# Round-1 Gemini audit (your prior)\n\n" + round1_gemini +
    "\n\n---\n\n# Round-2 Gemini audit (your prior PASS)\n\n" + round2_gemini +
    "\n\n---\n\n# Round-2 Codex audit (the round you must check round-3 closure on)\n\n" + round2_codex +
    "\n\n---\n\nNow give your INDEPENDENT round-3 audit. Cite § for every finding."
)

print(f"[gemini round3] prompt size: {len(full_prompt):,} chars", file=sys.stderr)

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
    print(f"[gemini round3] HTTP {e.code}: {e.read().decode(errors='replace')[:2000]}", file=sys.stderr)
    sys.exit(1)

elapsed = time.time() - t0
print(f"[gemini round3] API returned in {elapsed:.1f}s", file=sys.stderr)

text = data["candidates"][0]["content"]["parts"][0]["text"]
out = ROOT / "handover/audits/GEMINI_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md"
header = (f"# Gemini PPUT-CCL PREREG Adversarial Audit (Phase A4 ROUND 3)\n"
          f"**Date**: 2026-04-26\n"
          f"**Target**: handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md (round 3)\n"
          f"**Elapsed**: {elapsed:.1f}s\n"
          f"**Prompt size**: {len(full_prompt):,} chars\n\n---\n\n")
out.write_text(header + text)
print(f"[gemini round3] saved: {out}")
