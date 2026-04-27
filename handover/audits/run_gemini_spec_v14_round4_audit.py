#!/usr/bin/env python3
"""Gemini round-4 audit on STATE_TRANSITION_SPEC v1.4 (post 4 cosmetic patches)."""
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

PROMPT = """You are Gemini DeepThink, the strategic architectural reviewer for TuringOS v4.

This is the **round-4 re-audit** on STATE_TRANSITION_SPEC v1.4. Your task: confirm closure of the four PARTIAL items left from Codex round-3, and certify CO P1 launch readiness.

Codex round-3 verdict (handover/audits/CODEX_SPEC_FREEZE_AUDIT_2026-04-27.md):
- 8 CLOSED (round-2 issues fully resolved)
- 5 PARTIAL items remaining:
  * Q1.1 — TaskMarketPublishTx legacy-disposition log entry was incorrect (said "retired" but it is NEW per spec § 5.3)
  * Q2.4 — ChallengeWindow::is_open(now) helper not actually invoked in pseudocode
  * Q5 / NEW-5 — STEP_B canonical fixtures + full ABI deferred (genuine v4-ship-gate decision)
  * Q6 — sequencer tie-break under concurrent submitters
- 0 NEW issues / 0 REGRESSED previously-closed items

Claude has applied **v1.4 patches** (cosmetic-only):
1. Q1.1 — patch log corrected: TaskMarketPublishTx classified NEW (not retired)
2. Q2.4 — § 5.2.5 defines ChallengeWindow::is_open(now); both challenge_transition and finalize_reward now invoke it
3. Q6 — § 5.2.6 atomic next_logical_t() sequencer tie-break
4. Q5 / NEW-5 — § 2.5 explicit defer-ack: SERIALIZATION RULE frozen (bincode v2 big-endian + BTreeMap lex + UTF-8 length prefix); fixture corpus + full ABI surface deferred to CO1.1.4-pre1 + CO1.7

Round-4 questions:

Q1. Closure verification — did the v1.4 patches actually close the round-3 PARTIAL items?
   1.1 patch log accuracy
   1.2 ChallengeWindow::is_open(now) actually used in pseudocode (challenge_transition § 3.2 + finalize_reward § 3.4)
   1.3 next_logical_t() sequencer tie-break sufficient under concurrent submitters
   1.4 § 2.5 defer-ack acceptable as v4-ship-gate scope (NOT unresolved spec ambiguity)

Q2. Did v1.4 introduce ANY new issues vs v1.3?

Q3. Cross-spec consistency post-v1.4: GENESIS_MINIMAL_WITH_ANCHOR + SYSTEM_KEYPAIR + META_TX still aligned?

Q4. CO P1 launch readiness:
   - GO: spec is implementable; STEP_B branches A/B can begin
   - NO-GO: blocking issue remaining
   - NEEDS-FIX: cosmetic round-5 still needed

Q5. STEP_B deferral risk: § 2.5 says fixtures + ABI land in CO1.1.4-pre1 + CO1.7. If a STEP_B branch A vs B implementation drifts on bincode rule, can that drift be detected mechanically from v1.4 spec? Or are fixtures actually load-bearing for STEP_B comparison?

Q6. Final holistic verdict: PASS / CHALLENGE / VETO

Format your response:

# Gemini Spec v1.4 Round-4 Audit
## Q1.1-1.4 closure verdicts
## Q2 new-issues check
## Q3 cross-spec consistency
## Q4 CO P1 GO/NO-GO/NEEDS-FIX
## Q5 STEP_B deferral risk
## Q6 holistic verdict: PASS / CHALLENGE / VETO
## Residual concerns (if any)

Be rigorous; this is the final gate before CO P1 STEP_B launch.
"""

DOCS = [
    ("DOC: STATE_TRANSITION_SPEC v1.4 (current)", "handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md"),
    ("XREF: Codex round-3 audit (5 PARTIAL source)", "handover/audits/CODEX_SPEC_FREEZE_AUDIT_2026-04-27.md"),
    ("XREF: prior Gemini round-2 audit", "handover/audits/GEMINI_SPEC_FREEZE_AUDIT_2026-04-27.md"),
    ("XREF: GENESIS_MINIMAL_WITH_ANCHOR_v1", "handover/specs/GENESIS_MINIMAL_WITH_ANCHOR_v1_2026-04-27.md"),
    ("XREF: SYSTEM_KEYPAIR_SECURITY_v1", "handover/specs/SYSTEM_KEYPAIR_SECURITY_v1_2026-04-27.md"),
    ("XREF: META_TX_SCHEMA_v1", "handover/specs/META_TX_SCHEMA_v1_2026-04-27.md"),
    ("XREF: CO_MEGA_PLAN_v3.2", "handover/architect-insights/CO_MEGA_PLAN_v3.2_2026-04-27.md"),
    ("XREF: SPEC_WALKTHROUGH_v1", "handover/specs/SPEC_WALKTHROUGH_v1_2026-04-27.md"),
    ("XREF: Constitution", "constitution.md"),
]

parts = [PROMPT]
for label, rel in DOCS:
    p = REPO / rel
    parts.append(f"\n\n=== {label} ===\n\n")
    parts.append(p.read_text())
text = "".join(parts)

print(f"# Gemini Spec v1.4 Round-4 Audit Run")
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
