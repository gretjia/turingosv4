#!/usr/bin/env python3
"""Gemini independent audit on 2026-04-27 WP surgical revisions.

User authorized ArchitectAI (Claude) to revise white papers per ultrathink directive
「白皮书是对宪法的解释，但是不能违宪」. This audit independently verifies:
- 9 surgical edits do NOT introduce constitutional violations
- 9 edits do NOT introduce new numeric drift
- 9 edits preserve user's voice/intent (no creative additions)
- Critical missed items (if any)
"""
import json
import pathlib
import subprocess
import sys
import urllib.request

REPO = pathlib.Path("/home/zephryj/projects/turingosv4")
ENV_FILE = pathlib.Path("/home/zephryj/projects/turingosv3/.env")

key = None
for line in ENV_FILE.read_text().splitlines():
    if line.startswith("GEMINI_API_KEY="):
        key = line.split("=", 1)[1].strip()
        break
if not key:
    sys.exit("ERROR: no GEMINI_API_KEY")

PROMPT = """You are Gemini DeepThink, the strategic architectural reviewer for TuringOS v4 per the TRI_MODEL_ORCHESTRATION_PROTOCOL.

You have audited v3.1 + v3.2 + v3.2-fix1 in prior rounds. The TuringOS user (gretjia) on 2026-04-27 issued an ultrathink directive that established this authority chain:

1. constitution.md — supreme; FROZEN; cannot violate
2. white papers (architecture + economic) — interpretation of constitution; revisable; cannot violate constitution
3. spec docs — implementation of WP
4. code — implementation of spec

User authorized ArchitectAI (Claude) to revise white papers within bounds: fix numeric inconsistencies + close audit findings + reconcile with downstream specs. NO new philosophical content; NO constitutional violations.

Claude has now made 9 surgical edits across both white papers (commit e2ee141). Each edit is documented in `handover/whitepapers/REVISION_NOTES_2026-04-27.md`.

YOUR independent audit task — answer all of:

Q1. **Constitutional alignment**: for each of the 9 edits (A.1-A.4 + B.1-B.5), does it violate any Constitution Article (read constitution.md fully, especially Art 0-0.4, Art I-V)? List specific edits that pass/fail.

Q2. **Numeric drift fixes**: claude claims to have fixed 5 numeric inconsistencies (Q_t 8→9, RSP 8→9, Agent 5→6, Phase 1 substrate, v4 scope). Verify each fix is internally consistent in the post-edit WP. Are there OTHER numeric drifts NOT fixed?

Q3. **User voice preservation**: did Claude introduce new philosophical claims masquerading as "fixes"? Compare the diff at each edit location. Flag anything that adds substantive content vs. clarifying existing content.

Q4. **Cross-chapter consistency**: now that both chapters are revised, are they fully consistent with each other? Any remaining contradictions?

Q5. **Cross-spec consistency**: are the WP revisions consistent with STATE_TRANSITION_SPEC v1.1 + GENESIS_MINIMAL_WITH_ANCHOR + META_TX_SCHEMA + AmendmentFlow + V4_1_METATAPE_PLAN? Any drift?

Q6. **Missed revisions**: identify items that should have been revised but weren't. Specifically:
   - Architecture § 0 6 axioms (Claude said "deferred" — is that right?)
   - Architecture § 9 Goodhart shielding (Claude said deferred to Art 0.2 unfreeze)
   - Economic § 0 "经济不是发币" (Claude said "user's voice, don't touch")
   - Architecture § 11 Boot field reconciliation (3-source: Const Art IV + WP § 11 + spec GENESIS_MINIMAL_WITH_ANCHOR; Claude said FROZEN)
   - Are these deferral decisions correct, or is Claude over-deferring?

Q7. **Honesty check**: Claude wrote REVISION_NOTES § F "Honest Acknowledgements" + § C "What WAS NOT Changed" + § D "Items Still Unresolved". Are these accurate self-assessments, or is Claude flattering itself?

Q8. **Holistic verdict per change**:
   - A.1 (Q_t cross-ref): PASS / CHALLENGE / VETO
   - A.2 (§ 12.4 NEW v4 vs v4.1 boundary): PASS / CHALLENGE / VETO
   - A.3 (§ 17 Phase 3 prep concrete): PASS / CHALLENGE / VETO
   - A.4 (RSP appendix 8→9): PASS / CHALLENGE / VETO
   - B.1 (§ 7 title 5→6): PASS / CHALLENGE / VETO
   - B.2 (§ 20 Phase 1 substrate): PASS / CHALLENGE / VETO
   - B.3 (§ 20 v4 scope): PASS / CHALLENGE / VETO
   - B.4 (§ 19 RSP cross-ref): PASS / CHALLENGE / VETO
   - B.5 (§ 18 invariants cross-ref): PASS / CHALLENGE / VETO

Q9. **WP finalization readiness**: in your view, can the user now sign `v4-whitepaper-finalized-*` tag, or are there blockers that prevent finalization?

Q10. **What you'd want Codex to verify next**: areas you can't validate strategically; defer to Codex code-grounded check.

Format:
# Gemini WP-Revision Audit (2026-04-27)
## Q1-Q10 with [PASS/CHALLENGE/VETO] tags
## Cross-cutting concerns
## Holistic verdict on the 9 surgical edits as a bundle: PASS / CHALLENGE / VETO
## Top-3 must-fix items (if any)
## Recommendation on WP finalization: GO / NO-GO / NEEDS-FIX

Be rigorous. Do not flatter Claude.
"""

DOCS = [
    ("DOC 1: TURINGOS_WHITEPAPER_v1 (architecture, post-revision)",
     "handover/whitepapers/TURINGOS_WHITEPAPER_v1_2026-04-26.md"),
    ("DOC 2: TURINGOS_WHITEPAPER_v1_ECONOMIC (post-revision)",
     "handover/whitepapers/TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md"),
    ("DOC 3: REVISION_NOTES_2026-04-27 (Claude's self-doc of edits)",
     "handover/whitepapers/REVISION_NOTES_2026-04-27.md"),
    ("XREF: constitution.md (supreme; check no violations)", "constitution.md"),
    ("XREF: STATE_TRANSITION_SPEC_v1.1 (downstream alignment check)",
     "handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md"),
    ("XREF: GENESIS_MINIMAL_WITH_ANCHOR_v1 (downstream alignment check)",
     "handover/specs/GENESIS_MINIMAL_WITH_ANCHOR_v1_2026-04-27.md"),
    ("XREF: META_TX_SCHEMA_v1 (downstream alignment check)",
     "handover/specs/META_TX_SCHEMA_v1_2026-04-27.md"),
    ("XREF: V4_1_METATAPE_PLAN_v1 (v4.1 contract check)",
     "handover/architect-insights/V4_1_METATAPE_PLAN_v1_2026-04-27.md"),
    ("XREF: RATIFICATION_2026-04-27 (what's already user-confirmed)",
     "handover/architect-insights/RATIFICATION_2026-04-27.md"),
]

parts = [PROMPT]
for label, rel in DOCS:
    p = REPO / rel
    parts.append(f"\n\n=== {label} ===\n\n")
    parts.append(p.read_text())
text = "".join(parts)

print(f"# Gemini WP-Revision Audit Run")
print(f"- Model: gemini-2.5-pro")
print(f"- Packet chars: {len(text)}")
print(f"- Started: {subprocess.check_output(['date', '-Iseconds']).decode().strip()}")
print()
print("---")
print()
sys.stdout.flush()

req = json.dumps({
    "contents": [{"role": "user", "parts": [{"text": text}]}],
    "generationConfig": {"temperature": 0.2, "topP": 0.95, "maxOutputTokens": 16384},
}).encode("utf-8")

url = f"https://generativelanguage.googleapis.com/v1/models/gemini-2.5-pro:generateContent?key={key}"
r = urllib.request.Request(url, data=req, headers={"Content-Type": "application/json"}, method="POST")

try:
    raw = urllib.request.urlopen(r, timeout=900).read().decode("utf-8")
except urllib.error.HTTPError as e:
    print(f"HTTPError {e.code}: {e.read().decode()[:3000]}")
    sys.exit(1)

resp = json.loads(raw)
if "candidates" in resp:
    for cand in resp["candidates"]:
        for part in cand.get("content", {}).get("parts", []):
            if "text" in part:
                print(part["text"])
    print()
    print("---")
    if "usageMetadata" in resp:
        u = resp["usageMetadata"]
        print(f"## Usage: prompt={u.get('promptTokenCount', '?')} candidates={u.get('candidatesTokenCount', '?')} total={u.get('totalTokenCount', '?')} thoughts={u.get('thoughtsTokenCount', '?')}")
print(f"- Finished: {subprocess.check_output(['date', '-Iseconds']).decode().strip()}")
