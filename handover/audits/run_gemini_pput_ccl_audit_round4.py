#!/usr/bin/env python3
"""Gemini round-4 audit. Round 1-3 history baked into brief; targets clean rewrite."""
import os, sys, json, urllib.request, urllib.error, time
from pathlib import Path

ROOT = Path("/home/zephryj/projects/turingosv4")
V3_ENV = Path("/home/zephryj/projects/turingosv3/.env")

env = {}
for line in V3_ENV.read_text().splitlines():
    if "=" in line and not line.strip().startswith("#"):
        k, v = line.split("=", 1)
        env[k.strip()] = v.strip().strip('"').strip("'")

brief = """# Gemini Adversarial Audit — PREREG_PPUT_CCL_2026-04-26 (Phase A4 ROUND 4)

**Role**: skeptical adversarial reviewer. Independent of Codex round-4.

**Mandate**: round 4. History:
- Round 1: Gemini CHALLENGE / Codex CHALLENGE → CHALLENGE.
- Round 2: Gemini PASS / Codex CHALLENGE → CHALLENGE.
- Round 3: Gemini PASS / Codex CHALLENGE → CHALLENGE.

You PASSed rounds 2 and 3. Codex round-3 raised 3 still-open or new P0s that R4 addresses:
- (R4-1) STAT unit/power inconsistency: clean rewrite of § 5 + § 9 with single source of truth (per-problem unit; n=10 Phase C; n=54 Phase E); power tables corrected (Phase C requires 10/10 paired wins because at α=0.00147 with n=10, only X=10 clears).
- (R4-2) j-RR mathematically unwinnable in inferential family (0.9^54 ≈ 0.00343 > smallest Holm threshold). Round 4 moves j-RR out of inferential family — descriptive point-check guardrail. Family shrinks 4+4k → 4+3k; N_max 44 → 34.
- (R4-3) Hash defense too literal in round 3 (only blocked literal seed string + hashlib/sha2). Round 4 generalizes: substring + concatenation patterns; broad hash function blacklist (Python hashlib/cryptography/Crypto, Rust sha2/sha1/blake2/blake3/md5/digest/ring/openssl, JS crypto/SubtleCrypto, all shell hash binaries); path enumeration block on MiniF2F/Test (ls/find/rg --files/glob.glob blocked).

**Your task — round 4 verdict**:

YOU passed round 3. The R4 changes were prompted by Codex's round-3 still-open items. Did R4 hold your round-3 PASS, or introduce something new?

For each of R4-1, R4-2, R4-3:
- Closure check: does the round-4 fix close Codex's round-3 P0?
- New-issue check: does the round-4 fix introduce any NEW issues?

Specifically verify:
1. **R4-1 power calculation**: at α = 0.05/34 ≈ 0.00147 on n=10 paired-binary, exact one-sided binomial: X=10/10 gives p = 1/1024 ≈ 0.000977 ≤ 0.00147 (passes). X=9/10 gives p = 11/1024 ≈ 0.0107 (fails). Is the round-4 power table § 9.4 + § 9.5 correct? (It declares Phase C requires 10/10; Phase E requires ≥39/54 at smallest threshold.)
2. **R4-2 j-RR descriptive guardrail**: § 5.4 redefines j-RR as point check `RR_j ≤ p_0` (no α correction, not in family). § 1.6 + § 1.7 + § 7 Gate H + § 6 E3 + § 10.1 all updated to "3 inferential + 1 guardrail + rollback". Is the certification logic sound? Is removing j-RR from the inferential family statistically defensible (constraint vs hypothesis)?
3. **R4-3 sealing generalization**: § 2.3 L3/L4/L5 broadened — does it now actually close the recomputation vector across all reasonable attack patterns?

Plus internal consistency: does the document hang together with 4 rounds of patches + 1 clean rewrite of § 5 + § 9? Any leftover stale references? Any new contradictions?

End with:
- **VERDICT**: PASS / CHALLENGE / VETO
- If CHALLENGE: top 3 still-open or new P0
- Conviction
- Are 4 revision rounds + 1 clean rewrite a sign of structural instability or normal pre-registration tightening?

You MUST be willing to PASS if R4 closes round-3 blockers without new ones.

---

# PREREG_PPUT_CCL_2026-04-26.md (round 4)

"""

prereg = (ROOT / "handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md").read_text()
splits_json = (ROOT / "handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json").read_text()
round1_verdict = (ROOT / "handover/audits/DUAL_AUDIT_PPUT_CCL_VERDICT_2026-04-26.md").read_text()
round2_gemini = (ROOT / "handover/audits/GEMINI_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md").read_text()
round3_gemini = (ROOT / "handover/audits/GEMINI_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md").read_text()
round3_codex = (ROOT / "handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md").read_text()

full_prompt = (
    brief +
    prereg +
    "\n\n---\n\n# Three-split JSON\n\n```json\n" + splits_json + "\n```\n" +
    "\n\n---\n\n# Round-1 merged verdict\n\n" + round1_verdict +
    "\n\n---\n\n# Your prior PASSes\n\n## Round 2\n\n" + round2_gemini + "\n\n## Round 3\n\n" + round3_gemini +
    "\n\n---\n\n# Codex round-3 (the still-open P0s R4 must close)\n\n" + round3_codex +
    "\n\n---\n\nNow give your INDEPENDENT round-4 audit. Cite § for every finding."
)

print(f"[gemini round4] prompt size: {len(full_prompt):,} chars", file=sys.stderr)

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
    print(f"[gemini round4] HTTP {e.code}: {e.read().decode(errors='replace')[:2000]}", file=sys.stderr)
    sys.exit(1)

elapsed = time.time() - t0
print(f"[gemini round4] API returned in {elapsed:.1f}s", file=sys.stderr)

text = data["candidates"][0]["content"]["parts"][0]["text"]
out = ROOT / "handover/audits/GEMINI_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md"
header = (f"# Gemini PPUT-CCL PREREG Adversarial Audit (Phase A4 ROUND 4)\n"
          f"**Date**: 2026-04-26\n"
          f"**Target**: handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md (round 4)\n"
          f"**Elapsed**: {elapsed:.1f}s\n"
          f"**Prompt size**: {len(full_prompt):,} chars\n\n---\n\n")
out.write_text(header + text)
print(f"[gemini round4] saved: {out}")
