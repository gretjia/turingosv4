#!/usr/bin/env python3
"""Gemini round-4 audit on CO1.7-extra v1.2 (post round-3 mechanical fixes).

Strategic angle: confirm v1.2's mechanical patches don't introduce
architectural drift. Round-3 already PASSed v1.1 architecturally;
round-4 is mainly verification.
"""
import json
import pathlib
import subprocess
import sys
import urllib.request
import urllib.error

REPO = pathlib.Path("/home/zephryj/projects/turingosv4")
ENV_FILE = pathlib.Path("/home/zephryj/projects/turingosv3/.env")
OUT = REPO / "handover/audits/GEMINI_CO1_7_EXTRA_ROUND4_AUDIT_2026-04-29.md"

key = None
for line in ENV_FILE.read_text().splitlines():
    if line.startswith("GEMINI_API_KEY="):
        key = line.split("=", 1)[1].strip()
        break
if not key:
    sys.exit("ERROR: no GEMINI_API_KEY")

PROMPT = """You are Gemini DeepThink, the strategic architectural reviewer for TuringOS v4.

This is a **round-4 dual external audit** on CO1.7-extra v1.2 — applied 4 mechanical fixes (B1-B4) per `CO1_7_EXTRA_DUAL_AUDIT_VERDICT_R3_2026-04-29.md`. Codex is running an independent round-4 in parallel. Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.

## Round-3 disposition
- Codex r3: CHALLENGE/High (3 concrete patch blockers + 1 non-blocking; "no foundational design flaw")
- Gemini r3: PASS/High ("v1.1 spec is a model of post-audit closure")
- Conservative-merged: CHALLENGE (Codex wins)

## What changed since round-3 (v1.1 → v1.2)

The 4 patches are mechanical (1-line code/text fixes). NO architectural surface change.

- **B1**: stage-9 snippet `&**writer_w` → `&*writer_w` (compile error fix on `RwLockWriteGuard<dyn LedgerWriter>` deref)
- **B2**: helper `pub(crate) fn advance_head_t` → `pub fn advance_head_t` (integration test access); added FC-trace doc-comment
- **B3**: removed 2 stale Kernel references (preface line 14 single-sentence summary + § 6 pre-implementation gate)
- **B4** (non-blocking): conditional `#[serde(skip)]` comment + LoC sync

## Round-4 strategic question (single)

**Q1. Architectural integrity post-mechanical-fixes**: did the 4 patches preserve all the architectural soundness Gemini validated in round-3?
- MF4 TuringBus placement: now with B3 stale-text removal, does the architecture still read coherently end-to-end?
- MF3 required trait method: unchanged
- MF2 helper extraction: B2 widens visibility from `pub(crate)` to `pub`. Does this compromise encapsulation, or is `pub` appropriate for a constitutional-anchor helper that integration tests must reach?
- B1 + B4: pure mechanical fixes; no architectural angle

**Q2. New architectural defects in v1.2**: any?

**Q3. Final holistic verdict**: PASS / CHALLENGE / VETO

End with:
- Top issues (if CHALLENGE; bias against new findings — only flag if substantive)
- Conviction (low/med/high)

## Output format

# Gemini CO1.7-extra Round-4 Audit
## Q1 Architectural integrity post-mechanical-fixes
## Q2 New architectural defects in v1.2
## Q3 **VERDICT**: PASS / CHALLENGE / VETO
## Top issues
## Conviction

Be rigorous but proportional — round-4 is a verification audit, not a re-litigation of architecture.
"""

DOCS = [
    ("DOC: CO1.7-extra v1.2 (target of audit)", "handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md"),
    ("XREF: round-3 merged verdict", "handover/audits/CO1_7_EXTRA_DUAL_AUDIT_VERDICT_R3_2026-04-29.md"),
    ("XREF: shipped src/state/sequencer.rs", "src/state/sequencer.rs"),
    ("XREF: shipped src/bus.rs (TuringBus)", "src/bus.rs"),
    ("XREF: shipped src/kernel.rs (UNTOUCHED post-MF4 + B3)", "src/kernel.rs"),
]

parts = [PROMPT]
for label, rel in DOCS:
    p = REPO / rel
    parts.append(f"\n\n=== {label} ===\n\n")
    parts.append(p.read_text())
text = "".join(parts)

with OUT.open("w") as f:
    f.write(f"# Gemini CO1.7-extra Round-4 Audit Run\n")
    f.write(f"- Model: gemini-2.5-pro\n")
    f.write(f"- Packet chars: {len(text)}\n")
    f.write(f"- Started: {subprocess.check_output(['date', '-Iseconds']).decode().strip()}\n")
    f.write(f"- HEAD: {subprocess.check_output(['git', '-C', str(REPO), 'rev-parse', 'HEAD']).decode().strip()}\n")
    f.write("\n---\n\n")
    f.flush()

    req = json.dumps({
        "contents": [{"role": "user", "parts": [{"text": text}]}],
        "generationConfig": {"temperature": 0.2, "topP": 0.95, "maxOutputTokens": 16384},
    }).encode("utf-8")

    url = f"https://generativelanguage.googleapis.com/v1/models/gemini-2.5-pro:generateContent?key={key}"
    r = urllib.request.Request(url, data=req, headers={"Content-Type": "application/json"}, method="POST")

    try:
        raw = urllib.request.urlopen(r, timeout=900).read().decode("utf-8")
    except urllib.error.HTTPError as e:
        f.write(f"HTTPError {e.code}: {e.read().decode()[:3000]}\n")
        sys.exit(1)

    resp = json.loads(raw)
    if "candidates" in resp:
        for cand in resp["candidates"]:
            for part in cand.get("content", {}).get("parts", []):
                if "text" in part:
                    f.write(part["text"])
        f.write("\n\n---\n")
        if "usageMetadata" in resp:
            u = resp["usageMetadata"]
            f.write(f"## Usage: prompt={u.get('promptTokenCount', '?')} candidates={u.get('candidatesTokenCount', '?')} total={u.get('totalTokenCount', '?')} thoughts={u.get('thoughtsTokenCount', '?')}\n")
    f.write(f"- Finished: {subprocess.check_output(['date', '-Iseconds']).decode().strip()}\n")

print(f"saved: {OUT}")
