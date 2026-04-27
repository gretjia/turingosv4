#!/usr/bin/env python3
"""Gemini cross-review on Plan v3.2 + spec docs (post-Codex T+S re-review).

Codex already reviewed at the philosophical/T+S level and code-grounded level.
Gemini's role is strategic cross-section coherence on the new spec artifacts.
"""
import json
import os
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

You audited v3.1 yesterday (2 runs). Codex then audited v3.1 (verdict: Plan v3.1 VETO + Amendment v1 VETO) and re-audited a T+S re-review (verdict on Claude's revisions: D-VETO-1=CHALLENGE, D-VETO-3=CHALLENGE, D-VETO-4=VETO, B-1=PASS, D-VETO-6-retry=CHALLENGE).

Claude has now produced 4 new artifacts in response to Codex:

1. STATE_TRANSITION_SPEC_v1 — typed binding form for D-VETO-1 (typed Q_t / WorkTx / VerifyTx / ChallengeTx / RejectedAttemptSummary / TerminalSummaryTx / MetaTx schemas + 16 named invariants + deterministic pseudocode + hidden-input classification + optional TLA+ skeleton)
2. GENESIS_MINIMAL_WITH_ANCHOR_v1 — 8-field minimal genesis (replaces 5-line) with content-hash anchors for amendment_predicate + initial registries
3. ART_0_2_REINTERPRETATION — proposes Reading Y for Art 0.2 line 64 ("failure SIGNAL on tape, not raw rejected payloads"), 4 options A/B/C/X
4. CO_MEGA_PLAN_v3.2 — patch overlay on v3.1 (NOT full rewrite); 6 surgical changes, +26 atoms (~159 total), 20-25 weeks, $520-1100

YOUR strategic audit on these four documents:

Q1. Does STATE_TRANSITION_SPEC_v1 § 1 type system actually cover EVERY tx variant the white papers describe? Especially: WorkTx vs MetaTx vs ReuseTx vs TerminalSummaryTx — are there any economic-chapter or architecture-chapter transitions that fall through the gaps?

Q2. Are the 16 named invariants in STATE_TRANSITION_SPEC § 4 actually orthogonal? Are there hidden dependencies / overlaps that mean "passing 16 tests" doesn't actually prove what it claims?

Q3. The hidden-input classification in STATE_TRANSITION_SPEC § 2 retires `wall_clock` + `TAPE_ECONOMY_V2` env + `FOUNDER_GRANT_GAMMA` env + `system_lp_amount` config + `bus.graveyard` sidecar. Is this classification COMPLETE, or are there other hidden inputs in src/bus.rs / src/kernel.rs / experiments/.../src/bin/evaluator.rs that the spec misses?

Q4. GENESIS_MINIMAL_WITH_ANCHOR § 5 boot.rs verify_constitution_root function: does it actually anchor against bootstrap circularity? Specifically: if attacker swaps PINNED_CREATOR_PUBKEY constant in source, the whole anchor falls. What's the TRUST EXTENSION mechanism beyond this single file?

Q5. ART_0_2_REINTERPRETATION proposes Option B (cosmetic edit) as default. Is the proposed edited text actually clearer, or does it introduce new ambiguity (e.g., "any system-signed mechanism is allowed" — too permissive?). Compare to Option C (new sub-section).

Q6. CO_MEGA_PLAN_v3.2 § 5 critical path: is CO1.SPEC.0 actually a meaningful BLOCKING gate, or could a sufficiently careless implementation in CO1.1.4 still pass spec conformance while breaking the bus's actual contracts? What's the strongest test that would catch implementation gaming?

Q7. CO_MEGA_PLAN_v3.2 reverts D-VETO-4 from "permanent abandon" back to "defer v4.1 + Phase 3 prep" per Codex VETO. Is "Phase 3 prep" a substantive deliverable (could be audited) or is it weasel wording (could be empty docs)? What concrete artifacts should v4 ship to count as honest Phase 3 prep?

Q8. CO_MEGA_PLAN_v3.2 atom count went 132 → 159 (+26) and wall clock 17-21wk → 20-25wk. Is this honest given the new scope, or is there evidence Claude is still under-estimating somewhere? Where would Gemini bet a 4th overshoot lands?

Q9. The retry-metadata system-stamping (CO1.7.0 + CO1.7.0b) introduces a new system-signing keypair. Where is this keypair generated, where is it stored, who can rotate it, and what stops a compromised system instance from forging fake retry metadata?

Q10. Cross-section coherence: read all 4 new artifacts as a single bundle. What's the BIGGEST contradiction or tension between any two artifacts? (Examples: spec § 1.4 RejectedAttemptSummary vs Art 0.2 Reading Y choice; genesis schema vs spec § 1 Q_t fields; Plan v3.2 atoms vs spec § 7 deferred items.)

Format:
# Gemini v3.2 Cross-Review Audit
## Q1-Q10 with verdict tags [PASS/CHALLENGE/VETO]
## Cross-cutting concerns (anything not covered by Q1-Q10)
## Holistic verdict per artifact:
- STATE_TRANSITION_SPEC_v1: PASS / CHALLENGE / VETO
- GENESIS_MINIMAL_WITH_ANCHOR_v1: PASS / CHALLENGE / VETO
- ART_0_2_REINTERPRETATION: PASS / CHALLENGE / VETO
- CO_MEGA_PLAN_v3.2: PASS / CHALLENGE / VETO
## Top-3 must-fix before user accepts the bundle
## What you'd want Codex to verify next (deep-dive code questions)

Be rigorous. The user has been burned multiple times. Do not flatter Claude. If you see weak reasoning, say so.

"""

DOCS = [
    ("DOC 1: STATE_TRANSITION_SPEC_v1", "handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md"),
    ("DOC 2: GENESIS_MINIMAL_WITH_ANCHOR_v1", "handover/specs/GENESIS_MINIMAL_WITH_ANCHOR_v1_2026-04-27.md"),
    ("DOC 3: ART_0_2_REINTERPRETATION", "handover/specs/ART_0_2_REINTERPRETATION_2026-04-27.md"),
    ("DOC 4: CO_MEGA_PLAN_v3.2", "handover/architect-insights/CO_MEGA_PLAN_v3.2_2026-04-27.md"),
    ("XREF: Codex T+S re-review", "handover/audits/CODEX_T_S_REVIEW_2026-04-27.md"),
    ("XREF: Codex CO P0.7 audit", "handover/audits/CODEX_CO_P0_AUDIT_2026-04-26.md"),
    ("XREF: Constitution", "constitution.md"),
    ("XREF: WP architecture", "handover/whitepapers/TURINGOS_WHITEPAPER_v1_2026-04-26.md"),
    ("XREF: WP economic", "handover/whitepapers/TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md"),
    ("XREF: Plan v3.1 (predecessor)", "handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md"),
]

parts = [PROMPT]
for label, rel in DOCS:
    p = REPO / rel
    parts.append(f"\n\n=== {label} ===\n\n")
    parts.append(p.read_text())
text = "".join(parts)

print(f"# Gemini v3.2 Cross-Review")
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
