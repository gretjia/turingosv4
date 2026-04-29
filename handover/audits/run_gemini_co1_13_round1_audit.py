#!/usr/bin/env python3
"""Gemini round-1 audit on CO1.13 v1 (TRACE_MATRIX impl + R-022 hook).

Strategic / architectural angle: constitutional alignment fidelity, R-015 vs
R-022 defense-in-depth coherence, factory-tooling-as-spec vs as-devtools
boundary, Elon-mode round-cap policy first-application observability.
Independent of Codex round-1 (parallel).
"""
import json
import pathlib
import subprocess
import sys
import urllib.request
import urllib.error

REPO = pathlib.Path("/home/zephryj/projects/turingosv4")
ENV_FILE = pathlib.Path("/home/zephryj/projects/turingosv3/.env")
OUT = REPO / "handover/audits/GEMINI_CO1_13_ROUND1_AUDIT_2026-04-29.md"

key = None
for line in ENV_FILE.read_text().splitlines():
    if line.startswith("GEMINI_API_KEY="):
        key = line.split("=", 1)[1].strip()
        break
if not key:
    sys.exit("ERROR: no GEMINI_API_KEY")

PROMPT = """You are Gemini DeepThink, the strategic architectural reviewer for TuringOS v4.

This is a **round-1 dual external audit** on CO1.13 v1 — a 3-sub-atom factory bundle (TRACE_MATRIX_v3 doc completion + R-022 commit-time hook + reverse-map § F population). Codex is running an independent round-1 in parallel (implementer-paranoid angle); your angle is **strategic / architectural / constitutional**.

Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.
Per **NEW Elon-mode policy** (this v1 is the FIRST application = real test): round cap = 2; ship-with-OBS if r2 still CHALLENGE.

## Background

- CO1.7-extra (final L4 atom; STEP_B closed) shipped 2026-04-29 (commit `4a978f0`)
- CO1.13 is the canonical "TRACE_MATRIX impl + R-022 hook" atom per SPRINT_DEPENDENCY_GRAPH line 129 (3 atoms)
- TRACE_MATRIX_v3 is the existing 324-line doc; CO1.13.1 closes its empty/stub fields
- 15 active YAML rules already exist; CO1.13.2 lands R-022 as the 16th
- ~75% of src/ pub symbols lack TRACE_MATRIX backlinks (87/354 = 24.6% baseline coverage)
- Author selected CO1.13 over CO1.8 per Elon-mode ROI analysis (factory amortization 20-50x over 150+ remaining atoms)

## Round-1 strategic questions (7)

**Q1. Constitutional alignment of R-022 as commit-time block enforcer**:
- CLAUDE.md "Alignment Standard" requires every src/ pub symbol to map to FC1/FC2/FC3 element OR be filed orphan with explicit justification. R-022 enforces this at commit time.
- Authority chain: CLAUDE.md → TRACE_MATRIX_v3 → R-022 hook. Is this chain constitutionally sound? CLAUDE.md is project doc, not constitution. Should R-022 reference constitution.md Art V.1 directly?
- Forward-only enforcement (legacy ~250 untraced symbols exempt): is this constitutionally acceptable, or does Art 0.4 (alignment is invariant) require backlog cleanup?
- Q1 (spec § 5): forward-only vs edit-also. The author's lean is forward-only. Strategic: does signature change to a pub symbol change its CONSTITUTIONAL ROLE? If yes, edit-also is needed. If no, forward-only is sufficient.

**Q2. R-015 vs R-022 defense-in-depth coherence**:
- R-015 (existing): pre-edit warn; triggers on every pub symbol edit; warns regardless of backlink presence. Spec § 0.5 #1 keeps R-015.
- R-022 (NEW): commit-time block; triggers only on NEW pub additions; blocks if no backlink.
- These have OVERLAPPING but not IDENTICAL semantics. Is "defense in depth" the right framing, or does R-015 become NOISE under R-022 (every edit warns even when commit will block — alarm fatigue)?
- Strategic alternative: deprecate R-015 in favor of R-022 alone (spec § 5 Q5 considers this; author lean: keep both). Argue both sides.
- Operational risk: a developer learns to ignore R-015 warnings → develops habit of ignoring rule warnings generally → R-022 blocks become surprises → increased commit-cycle friction.

**Q3. Factory-tooling-as-spec vs as-devtools boundary**:
- Spec § 0.4 lists 3 devtools (scaffold_co_spec.sh / scaffold_audit_launcher.sh / rehash_trust_root.sh) as OUT of scope (no audit gate).
- These devtools ARE load-bearing for Elon-mode hypothesis ("cycle time 14d → 2d"). Removing audit gate is itself a process choice that affects subsequent atom quality.
- Strategic question: where is the right boundary?
  - Option A (spec lean): if not constitutional, no audit gate (developer experience tooling)
  - Option B (alt): anything that affects subsequent atom CYCLE TIME requires audit (because cycle-time guarantees compound)
  - Option C: per-script decision (scaffold_audit_launcher.sh produces AUDIT prompts → its quality affects every audit → IS load-bearing)
- Author's framing inherits from CO1.7-extra v1.2.2 § 2.2 amendment ("process-spec refinement, not mathematical, no audit"). Is this consistent precedent?

**Q4. Elon-mode 2-round-cap policy first application observability**:
- This v1 is the FIRST atom under the new round-cap policy.
- Strategic concern: if r1 = CHALLENGE/CHALLENGE and r2 still CHALLENGE, ship-with-OBS captures unresolved issues to handover/alignment/OBS_*.md.
- BUT: under prior policy (4-5 rounds to PASS/PASS), the same issues would be FIXED in r3-r5. Ship-with-OBS PRESERVES the issues; doesn't resolve them.
- Question: is "ship-with-OBS" honest "we know about it and accept" OR is it "we couldn't fix it in 2 rounds and gave up"? Constitutional risk: technical debt accumulating in OBS files masks rather than addresses architectural drift.
- Drift review at phase end (per user's auto-execute protocol) should catch this; spec § 8 ack #8 mentions drift review. Is the review criteria specific enough?

**Q5. Anti-Oreo three-layer compliance of R-022 enforcement**:
- R-022 enforces TRACE_MATRIX backlinks on pub symbols in src/. Anti-Oreo three layers: top-white (predicates) / middle-black (agents) / bottom-white (tools).
- Does R-022 respect the layer separation, or does it mix concerns? Specifically: backlinks span FC1/FC2/FC3 — does R-022 require domain-specific knowledge of which layer a symbol belongs to?
- v1 says: "5 lines preceding has /// TRACE_MATRIX " — that's syntactic, not semantic. The script doesn't know whether the backlink is correctly assigned to the right layer.
- Wrong assignment risk: a symbol with `/// TRACE_MATRIX FC3-N15: ...` that should actually be FC1 — passes R-022 but is constitutionally misaligned. R-022 is FORM check, not SUBSTANCE check.
- Mitigation: TRACE_MATRIX_v3 § A/§ B serves as the substance check (every Article + every § has a defined Code symbol mapping). Is this enough?

**Q6. Whitepaper coverage implications**:
- WP v2 has 21 architecture §s + 8 economic §s + RSP appendix (per TRACE_MATRIX_v3 § B).
- CO1.13.1 closes § B WP coverage. But several WP §s (e.g., L5-L6 indices, L4 transition bodies) reference modules that don't exist yet (CO1.8 / CO1.9 / future CO1.7.5).
- Strategic question: does CO1.13.1 § B closure require those modules to ship FIRST (creating circular dependency CO1.13 ← CO1.8 ← CO1.13)? Or does TRACE_MATRIX_v3 § B accept "Plan v3.2 atom: CO1.8" as valid Code symbol pointer (deferred-mapping)?
- Spec § 8 ack #4 acknowledges "TRACE_MATRIX_v3 references modules that don't yet exist" but doesn't resolve the circular dependency question.

**Q7. Forward sustainability**:
- After CO1.13 ships, 150+ atoms remain. R-022 enforces forward-only; the legacy 75% gap remains.
- Strategic question: when does the legacy gap become technical debt that compounds? Quantitative threshold: at what % coverage does the discrepancy between R-022 (block new) and current (75% gap) become indefensible?
- Author's lean: CO1.13-extra closes legacy gap (~10-15 hr; ~250 backlinks). When should this happen? Spec defers without timeline.
- Per project_thesis (Frozen 5-step compile loop): CO1.13 is infrastructure. Doesn't directly advance the loop. What's its priority vs CO1.8 (which DOES feed L5 materialization that feeds Phase D ArchitectAI)?

## Verdict format

Section A: Verdict (PASS/CHALLENGE/VETO) with conviction (LOW/MED/HIGH).
Section B: P0 blockers (must-fix before round-2; cite spec § + line).
Section C: Open architectural questions raised.
Section D: Suggested patches (specific spec line/section edits).
Section E: Forward-sustainability notes.

Be concrete. Cite spec § + line where possible.

---

# Spec attachment + reference materials follow.
"""

# Build context
attachments = []

def append_file(label, path, fence="rust"):
    full = REPO / path
    if not full.exists():
        return
    attachments.append(f"\n\n---\n\n## XREF: {label} — `{path}`\n\n```{fence}\n{full.read_text()}\n```\n")

append_file("CO1.13 v1 spec (PRIMARY)", "handover/specs/CO1_13_TRACE_MATRIX_IMPL_v1_2026-04-29.md", fence="markdown")
append_file("TRACE_MATRIX_v3 (existing 324-line doc)", "handover/alignment/TRACE_MATRIX_v3_2026-04-27.md", fence="markdown")
append_file("docs/rules.md (rule engine architecture)", "docs/rules.md", fence="markdown")
append_file("rules/SCHEMA.yaml", "rules/SCHEMA.yaml", fence="yaml")
append_file("rules/active/R-015 (existing pre-edit warn)", "rules/active/R-015_trace_matrix_pub_symbol.yaml", fence="yaml")
append_file("rules/engine.py", "rules/engine.py", fence="python")

full_prompt = PROMPT + "".join(attachments)
print(f"[gemini co1.13 r1] prompt size: {len(full_prompt)} chars", file=sys.stderr)

# POST to Gemini (gemini-3.1-pro-preview = latest strongest available 2026-04-29)
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
print(f"[gemini co1.13 r1] API returned in {elapsed:.1f}s", file=sys.stderr)

# Extract text
candidates = data.get("candidates", [])
if not candidates:
    sys.exit(f"No candidates: {json.dumps(data)[:500]}")
parts = candidates[0].get("content", {}).get("parts", [])
text = "".join(p.get("text", "") for p in parts)

OUT.parent.mkdir(parents=True, exist_ok=True)
header = f"""# Gemini CO1.13 Round-1 Audit
**Date**: 2026-04-29
**Target**: spec v1 (greenfield; first Elon-mode 2-round-cap atom)
**HEAD**: {subprocess.run(['git', '-C', str(REPO), 'rev-parse', 'HEAD'], capture_output=True, text=True).stdout.strip()}
**Prompt size**: {len(full_prompt)} chars
**API latency**: {elapsed:.1f}s

---

"""
OUT.write_text(header + text)
print(f"[gemini co1.13 r1] saved: {OUT}", file=sys.stderr)
