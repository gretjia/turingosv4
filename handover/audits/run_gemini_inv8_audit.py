#!/usr/bin/env python3
"""Gemini audit on INV8 DAG determinism spike pre-draft."""
import json, pathlib, subprocess, sys, urllib.request

REPO = pathlib.Path("/home/zephryj/projects/turingosv4")
ENV_FILE = pathlib.Path("/home/zephryj/projects/turingosv3/.env")
key = None
for line in ENV_FILE.read_text().splitlines():
    if line.startswith("GEMINI_API_KEY="):
        key = line.split("=", 1)[1].strip()
        break
if not key:
    sys.exit("ERROR: no GEMINI_API_KEY")

PROMPT = """You are Gemini DeepThink. Audit `INV8_DAG_DETERMINISM_SPEC_v1_2026-04-27.md` — CO P2.4.0 spike pre-draft. Plan v3.2 marks this spike as BLOCKING for any AttributionEngine implementation. Your v3.2 Q3 CHALLENGE made this mandatory.

Per Tri-Model Protocol § 9 Hard rule 2: this spec was authored by Claude (orchestrator); Gemini auditing is permitted.

**Your verdict on each**:

Q1. Algorithm soundness: does `compute_contribution_dag` (§ 2.2) produce byte-identical adjacency lists regardless of map iteration order?
Q2. 7 hostile cases (§ 3 / § 4) — right 7? Any missing? Each closed?
Q3. Concurrent-write parent ambiguity — tie-break deterministic under all witnessed-orderings? (Cf. spec v1.4 § 5.2.6 next_logical_t().)
Q4. Multi-parent merge weighting — conservation? Anti-double-counting?
Q5. Citation transitivity — A → B → C propagation without infinite loops?
Q6. Cycle detection — self-reference + adversarial cycle handled?
Q7. Edge type discrimination — builds-on vs cites vs reuses disambiguated?
Q8. Reproducibility test plan sufficient for fuzz/diff?
Q9. 1-page algorithm form held? (Your own Top-3 fix #2 demand.)
Q10. Ready to implement in Rust by CO P2.4.1?

**Final**: PASS / CHALLENGE / VETO + holistic verdict + recommendation (CO P2.4.0 spike CLEARED / NEEDS-REVISION / FAIL).

**Format**:
# Gemini INV8 Audit
## Q1-Q10 verdicts
## Holistic verdict
## Must-fix (if any)
"""

DOCS = [
    ("DOC: INV8 DAG SPEC v1 (subject)", "handover/specs/INV8_DAG_DETERMINISM_SPEC_v1_2026-04-27.md"),
    ("XREF: STATE_TRANSITION_SPEC v1.4 (TxId / WorkTx fields)", "handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md"),
    ("XREF: WP economic (Inv 8 origin)", "handover/whitepapers/TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md"),
    ("XREF: CO_MEGA_PLAN v3.2 (where P2.4.0 sits)", "handover/architect-insights/CO_MEGA_PLAN_v3.2_2026-04-27.md"),
    ("XREF: Constitution", "constitution.md"),
]

parts = [PROMPT]
for label, rel in DOCS:
    p = REPO / rel
    parts.append(f"\n\n=== {label} ===\n\n")
    parts.append(p.read_text())
text = "".join(parts)

print(f"# Gemini INV8 Audit Run")
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
