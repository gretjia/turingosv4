#!/usr/bin/env python3
"""Gemini re-audit on STATE_TRANSITION_SPEC v1.2 (post-NEEDS-FIX patches)."""
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

This is a **re-audit** following your CHALLENGE/NEEDS-FIX verdict on STATE_TRANSITION_SPEC v1.1 (your prior audit at `handover/audits/GEMINI_SPEC_FREEZE_AUDIT_2026-04-27.md`).

Claude has applied **v1.2 patches** (commit `6d0e6d9`). The spec at `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md` is now v1.2.

Your prior 3 must-fix items:
1. Add stake/bounty refund logic (I-STAKE-RETURN + I-BOUNTY-REFUND invariants + new task_expire_transition)
2. Specify agent/reputation initialization (I-AGENT-INIT)
3. Clarify v4 predicate bootstrap path

Codex's parallel audit identified additional must-fix items (canonical serialization, concurrency rules, hidden inputs expansion, MicroCoin promotion, false-challenge contradiction, etc.). All bundled into v1.2.

Your re-audit task:

Q1. Did v1.2 close YOUR Top-3 must-fix items?
   1.1 I-STAKE-RETURN: spec § 4 + § 3.4 stage 3a
   1.2 I-BOUNTY-REFUND: spec § 4 + § 3.6 (NEW task_expire_transition)
   1.3 I-AGENT-INIT: spec § 4 + § 3.6.5 (NEW agent_implicit_init)
   1.4 v4 predicate bootstrap path: explicitly addressed?

Q2. Did v1.2 introduce any NEW issues that weren't there in v1.1?

Q3. Are the 27 invariants now sufficient for whole-system requirements (12 economic + 24 V tape canonical + Const Art I-V)? Any STILL missing?

Q4. Cross-spec consistency post-v1.2: is GENESIS_MINIMAL_WITH_ANCHOR + SYSTEM_KEYPAIR + META_TX still aligned with spec v1.2?

Q5. Plan v3.2-fix2 promoted MicroCoin (CO P2.0a) → CO1.0a P1 prerequisite. Does this actually fix the migration trap Codex flagged?

Q6. STEP_B comparison metric: § 2.5 canonical serialization (bincode v2 big-endian + BTreeMap lex + UTF-8 length prefix). Strong enough for STEP_B branches A vs B? Any sleeper risks?

Q7. Concurrency § 5.2 (L4 sequencer + cross-cell isolation + finalize batch order). Strong enough? Any sleeper risks under Phase C 5-mode parallel runs?

Q8. False-challenge contradiction (§ 5.1): v1.2 says "v4 fixed = 0; NOT configurable". This resolves the contradiction by removing configurability. Is removing the option the right call, or should it be implementable later?

Q9. CO P1 launch readiness: GO / NO-GO / STILL-NEEDS-FIX

Q10. Codex defer items still relevant? (canonical digests / WASM sandbox / materializer logic / dependency tree)

Format:
# Gemini Spec v1.2 Re-Audit
## Q1-Q10 with verdicts
## Holistic re-verdict: PASS / CHALLENGE / VETO
## Recommendation: CO P1 GO / NO-GO / NEEDS-FIX
## Residual concerns (if any)

Be rigorous; final gate before CO P1 launch.
"""

DOCS = [
    ("DOC: STATE_TRANSITION_SPEC v1.2 (post-patch)", "handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md"),
    ("XREF: prior Gemini audit", "handover/audits/GEMINI_SPEC_FREEZE_AUDIT_2026-04-27.md"),
    ("XREF: prior Codex audit", "handover/audits/CODEX_SPEC_FREEZE_AUDIT_2026-04-27.md"),
    ("XREF: GENESIS_MINIMAL_WITH_ANCHOR_v1", "handover/specs/GENESIS_MINIMAL_WITH_ANCHOR_v1_2026-04-27.md"),
    ("XREF: SYSTEM_KEYPAIR_SECURITY_v1", "handover/specs/SYSTEM_KEYPAIR_SECURITY_v1_2026-04-27.md"),
    ("XREF: META_TX_SCHEMA_v1", "handover/specs/META_TX_SCHEMA_v1_2026-04-27.md"),
    ("XREF: CO_MEGA_PLAN_v3.2 (post-fix2)", "handover/architect-insights/CO_MEGA_PLAN_v3.2_2026-04-27.md"),
    ("XREF: SPEC_WALKTHROUGH_v1", "handover/specs/SPEC_WALKTHROUGH_v1_2026-04-27.md"),
    ("XREF: WP architecture (post-revision)", "handover/whitepapers/TURINGOS_WHITEPAPER_v1_2026-04-26.md"),
    ("XREF: WP economic (post-revision)", "handover/whitepapers/TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md"),
    ("XREF: Constitution", "constitution.md"),
]

parts = [PROMPT]
for label, rel in DOCS:
    p = REPO / rel
    parts.append(f"\n\n=== {label} ===\n\n")
    parts.append(p.read_text())
text = "".join(parts)

print(f"# Gemini Spec v1.2 Re-Audit Run")
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
