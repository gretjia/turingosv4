#!/usr/bin/env python3
"""Gemini round-2 audit on CO1.13 v1.1 (post-r1 patch synthesis; FINAL).

Strategic / architectural angle: did Gemini's r1 P0s (engine.py bypass +
escape-hatch token) close cleanly? Does v1.1 introduce strategic regressions?
Final ship-or-escalate per Elon-mode 2-round-cap.

Per Codex r1 § E: if R-022 enforcement still non-functional, escalate; do NOT
auto-ship-with-OBS for the gate itself.
"""
import json
import pathlib
import subprocess
import sys
import urllib.request
import urllib.error

REPO = pathlib.Path("/home/zephryj/projects/turingosv4")
ENV_FILE = pathlib.Path("/home/zephryj/projects/turingosv3/.env")
OUT = REPO / "handover/audits/GEMINI_CO1_13_ROUND2_AUDIT_2026-04-29.md"

key = None
for line in ENV_FILE.read_text().splitlines():
    if line.startswith("GEMINI_API_KEY="):
        key = line.split("=", 1)[1].strip()
        break
if not key:
    sys.exit("ERROR: no GEMINI_API_KEY")

PROMPT = """You are Gemini DeepThink, the strategic architectural reviewer for TuringOS v4.

This is **round-2** dual external audit on CO1.13 v1.1 — the patch synthesis after round-1 returned CHALLENGE/CHALLENGE. Codex is running independent round-2 in parallel (implementer-paranoid angle); your angle is **strategic / architectural / constitutional**.

Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.
Per Elon-mode 2-round-cap: this is the FINAL audit.

## Round-1 P0 disposition

Your round-1 (`GEMINI_CO1_13_ROUND1_AUDIT_2026-04-29.md`) flagged 2 P0s:

| P0 | Round-1 finding | Author claim closed in v1.1 by |
|---|---|---|
| G1 | engine.py architectural mismatch (per-file, no diff awareness) | § 1.2: YAML check.type → custom_commit_hook tombstone; shim calls check_trace_matrix.py DIRECTLY bypassing engine.py |
| G2 | Escape hatch as Rust code comment pollutes source | § 2.2: changed to commit-message token [R-022-skip: ...] |

Plus your r1 strategic insights (verify codified):
- Q4 OBS hard-threshold (max 3 unresolved) → § 0.5 #1
- Q5 form vs substance two-layer model → NEW § 2.5
- Q7 CO1.13-extra MUST schedule before Phase D → § 0.5 #3

Your r1 also confirmed Codex's findings, so v1.1 incorporates Codex's 7 P0s as well (P1-P7 in the patch log + P8 backlink-removal block + P9 Elon-mode policy refinement).

## Round-2 audit questions (5; strategic)

**Q1. Did v1.1 close the engine.py architectural mismatch (P0-G1)?**
Read § 1.2 v1.1 carefully:
- engine.py BYPASSED for R-022 (correct — addresses your P0)
- 5-line patch to engine.py to gracefully ignore trigger==pre_commit rules (clean — doesn't break existing rules)
- YAML rule becomes declarative tombstone (defensible — preserves 30-rule cap accounting + documentation surface)

But: is the dual-layer architecture (rules engine for pre-edit; standalone scripts for pre-commit) still architecturally clean, or is it now a fragmented enforcement system? Strategic perspective: would a future maintainer understand which rules go through engine.py vs which bypass?

**Q2. Did v1.1 close the escape hatch concern (P0-G2)?**
Read § 2.2 v1.1:
- Commit-message token [R-022-skip: <reason; cases/Cxxx | PREREG-§n.m | OBS_R022_*.md REQUIRED>]
- Required reference; missing → BLOCK
- Structured log entry mandatory

But: Codex P0-5 added "structured logging + reference required". Is the v1.1 escape hatch now over-engineered? Could a developer legitimately need to skip without an existing cases/Cxxx file (e.g., during the same atom that creates the case)? Should there be a "create cases/Cxxx + reference it in same commit" pattern documented?

**Q3. Are the 3 strategic policy items (Q4 OBS hard threshold + Q5 form/substance + Q7 CO1.13-extra-before-Phase-D) codified correctly?**
Read § 0.5 + § 2.5:
- § 0.5 #1: OBS max 3 unresolved → ✅ codified
- § 0.5 #2: ship-with-OBS NOT for R-022 gate itself → ✅ codified per Codex § E
- § 0.5 #3: CO1.13-extra before Phase D → ✅ codified
- § 2.5: form/substance two-layer model → ✅ codified

Strategic gaps to verify:
- § 0.5 #1 says "max 3 unresolved OBS" — does this apply across ALL atom types, or just CO1.13-related? Anti-Oreo separation suggests project-wide.
- § 2.5 promises "future CO1.13-extra-extra: semantic alignment checker". Is this a real commitment or hand-wave? Should it have a deferred-marker in PROJECT_DECISION_MAP § 3.6?

**Q4. Strategic regressions in v1.1?**
LoC budget 415 → 640 (+54%); cycle-time target 2 day → 3 day (+50%). The expansion is driven by audit findings, not scope creep — but at what point does v1.1 become "too big to ship as one atom"?
- Should CO1.13 be split into CO1.13.1 (TRACE_MATRIX_v3 doc closure) + CO1.13.2 (R-022 hook) as TWO sequential atoms? This would test the Elon-mode "factory" hypothesis on smaller atoms first.
- Or is the bundled atom architecturally correct (single coherent factory unit)?

**Q5. Final disposition recommendation**:
- If R-022 enforcement clearly works per § 1.2 v1.1 architecture: PASS or CHALLENGE-on-edge-cases → ship-or-ship-with-OBS allowed
- If R-022 enforcement still has fundamental gaps: CHALLENGE-on-ENFORCEMENT → ESCALATE per Codex § E
- Your call. Be specific.

## Verdict format

Section A: Verdict (PASS / CHALLENGE-bounded / CHALLENGE-ESCALATE) with conviction.
Section B: Per-P0 closure (G1, G2, plus your strategic Q4/Q5/Q7 codification).
Section C: NEW strategic concerns introduced by v1.1.
Section D: Final disposition recommendation.
Section E: Forward-sustainability notes.

Be concrete. Cite spec § + line.

---

# Spec attachment + reference materials follow.
"""

# Build context
attachments = []

def append_file(label, path, fence="markdown"):
    full = REPO / path
    if not full.exists():
        return
    attachments.append(f"\n\n---\n\n## XREF: {label} — `{path}`\n\n```{fence}\n{full.read_text()}\n```\n")

append_file("CO1.13 v1.1 spec (TARGET)", "handover/specs/CO1_13_TRACE_MATRIX_IMPL_v1_2026-04-29.md")
append_file("Gemini round-1 verdict (yours)", "handover/audits/GEMINI_CO1_13_ROUND1_AUDIT_2026-04-29.md")
append_file("Codex round-1 verdict (parallel; context)", "handover/audits/CODEX_CO1_13_ROUND1_AUDIT_2026-04-29.md")
append_file("TRACE_MATRIX_v3 (verify § G doesn't conflict with existing structure)", "handover/alignment/TRACE_MATRIX_v3_2026-04-27.md")

full_prompt = PROMPT + "".join(attachments)
print(f"[gemini co1.13 r2] prompt size: {len(full_prompt)} chars", file=sys.stderr)

# POST to Gemini (gemini-3.1-pro-preview)
url = f"https://generativelanguage.googleapis.com/v1beta/models/gemini-3.1-pro-preview:generateContent?key={key}"
body = json.dumps({
    "contents": [{"parts": [{"text": full_prompt}]}],
    "generationConfig": {
        "temperature": 0.3,
        "maxOutputTokens": 65536,
    },
}).encode()

req = urllib.request.Request(url, data=body, headers={"Content-Type": "application/json"}, method="POST")

import time
t0 = time.time()
try:
    with urllib.request.urlopen(req, timeout=300) as resp:
        data = json.loads(resp.read().decode())
except urllib.error.HTTPError as e:
    print(f"HTTP {e.code}: {e.read().decode()[:500]}", file=sys.stderr)
    sys.exit(1)
elapsed = time.time() - t0
print(f"[gemini co1.13 r2] API returned in {elapsed:.1f}s", file=sys.stderr)

# Extract text
candidates = data.get("candidates", [])
if not candidates:
    sys.exit(f"No candidates: {json.dumps(data)[:500]}")
parts = candidates[0].get("content", {}).get("parts", [])
text = "".join(p.get("text", "") for p in parts)

OUT.parent.mkdir(parents=True, exist_ok=True)
header = f"""# Gemini CO1.13 Round-2 Audit
**Date**: 2026-04-29
**Target**: spec v1.1 (post-r1 9-patch synthesis; FINAL per Elon-mode 2-round-cap)
**HEAD**: {subprocess.run(['git', '-C', str(REPO), 'rev-parse', 'HEAD'], capture_output=True, text=True).stdout.strip()}
**Prompt size**: {len(full_prompt)} chars
**API latency**: {elapsed:.1f}s

---

"""
OUT.write_text(header + text)
print(f"[gemini co1.13 r2] saved: {OUT}", file=sys.stderr)
