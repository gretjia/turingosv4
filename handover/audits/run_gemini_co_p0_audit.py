#!/usr/bin/env python3
"""CO P0.7 Gemini DeepThink audit — standalone Python (avoids heredoc fragility)."""
import json
import os
import pathlib
import subprocess
import sys
import urllib.request

REPO = pathlib.Path("/home/zephryj/projects/turingosv4")
ENV_FILE = pathlib.Path("/home/zephryj/projects/turingosv3/.env")

# Load Gemini key
key = None
if ENV_FILE.exists():
    for line in ENV_FILE.read_text().splitlines():
        if line.startswith("GEMINI_API_KEY="):
            key = line.split("=", 1)[1].strip()
            break
if not key:
    print("ERROR: GEMINI_API_KEY not found", file=sys.stderr)
    sys.exit(1)

PROMPT = """You are Gemini DeepThink, the strategic architectural reviewer for TuringOS v4 per the TRI_MODEL_ORCHESTRATION_PROTOCOL.

Your role: independent strategic audit. You did NOT participate in drafting. Provide PASS / CHALLENGE / VETO + reasoning per document, then a holistic verdict.

The TuringOS user (a solo researcher with zero programming background) has lost trust in single-model execution after multiple architectural rot incidents. Your audit is the antidote — be rigorous, be honest, do not flatter.

Audit scope: 4 newly-shipped docs forming the v4 architectural foundation:
1. TURINGOS_v4_FINAL_BLUEPRINT (synthesizes architecture WP + economic WP into file-level v4 spec)
2. CO_MEGA_PLAN_v3.1 (atomization: 132 atoms, 17-21 weeks, $435-950 budget)
3. TRI_MODEL_ORCHESTRATION_PROTOCOL (Codex+Gemini as co-executors, per-atom workflow, decision matrix)
4. CO_P0_AMENDMENT_v1 (D1-D6 all-rec resolutions + cost adjustment)

Cross-reference inputs:
A. TURINGOS_WHITEPAPER_v1 (architecture chapter, user-authored 2026-04-26 ultrathink)
B. TURINGOS_WHITEPAPER_v1_ECONOMIC (economic chapter, user-authored)
C. constitution.md Art. 0-0.4 (recent amendments)

Your audit must answer:

Q1. Coverage: does the Blueprint actually cover EVERY § in both white papers? Are there any white paper concepts that lack a code symbol or test mapping?

Q2. Anti-Oreo soundness: are any modules ambiguously layered? Is the Goodhart shield (private predicate visibility) actually leak-proof?

Q3. Economic invariants completeness: do the 12 invariants -> 12 conformance tests actually catch the invariant violations they claim? Especially Inv 8 (DAG attribution) — is determinism actually achievable?

Q4. Plan v3.1 atom dependency soundness: do CO P1 atoms (50-65) truly precede CO P2 atoms in implementation order? Any circular dependency or blocking pair?

Q5. STEP_B atom risk: bus.rs + kernel.rs splits (CO1.1.4, CO1.1.5) are listed as 5-way / 3-way parallel branches. Is parallel branch the right protocol, or is sequential refactor safer?

Q6. Tri-model protocol soundness: per-atom workflow has Generator != Evaluator. Is that achievable when the heaviest implementer (Codex) is also the heaviest reviewer? Risk of Codex marking own code PASS?

Q7. Cost projection realism: $435-950 — is this honest given 132 atoms x multi-model audits? Or under-estimating?

Q8. D-decisions all-rec sanity: D5=A full RSP increases atom count; D4=B defers MetaTape; are these compatible (i.e., can full RSP be tested without MetaTape ArchitectAI runtime)?

Q9. Constitution Art 0.5 draft (6 axioms): are 6 axioms the right granularity? Should it be expanded or compressed?

Q10. PREREG amendment v2 (D1=C MVP-pivot): 50 rows x 1 seed for Phase C MVP — is this enough statistical power to declare H1-H4 supported/rejected?

Format your response as:

# Gemini CO P0.7 Strategic Audit

## Q1-Q10 detailed answers (one paragraph per Q with verdict tag [PASS/CHALLENGE/VETO])

## Cross-cutting concerns

## Holistic verdict
- Blueprint: PASS / CHALLENGE / VETO
- Plan v3.1: PASS / CHALLENGE / VETO
- Protocol: PASS / CHALLENGE / VETO
- Amendment v1: PASS / CHALLENGE / VETO

## Top-3 must-fix items (if any) before CO P1 entry

## What you're uncertain about (defer to Codex for deep-dive)

"""

DOCS = [
    ("DOC 1: TURINGOS_v4_FINAL_BLUEPRINT", "handover/whitepapers/TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md"),
    ("DOC 2: CO_MEGA_PLAN_v3.1", "handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md"),
    ("DOC 3: TRI_MODEL_ORCHESTRATION_PROTOCOL", "handover/architect-insights/TRI_MODEL_ORCHESTRATION_PROTOCOL_2026-04-26.md"),
    ("DOC 4: CO_P0_AMENDMENT_v1", "handover/architect-insights/CO_P0_AMENDMENT_v1_2026-04-26.md"),
    ("XREF A: WHITEPAPER architecture", "handover/whitepapers/TURINGOS_WHITEPAPER_v1_2026-04-26.md"),
    ("XREF B: WHITEPAPER economic", "handover/whitepapers/TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md"),
    ("XREF C: constitution.md", "constitution.md"),
]

parts = [PROMPT]
for label, rel in DOCS:
    p = REPO / rel
    parts.append(f"\n\n=== {label} ===\n\n")
    parts.append(p.read_text())
text = "".join(parts)

print(f"# Gemini CO P0.7 Audit Run\n")
print(f"- Model: gemini-2.5-pro")
print(f"- Packet chars: {len(text)}")
print(f"- Started: {subprocess.check_output(['date', '-Iseconds']).decode().strip()}")
print()
print("---")
print()
sys.stdout.flush()

req_body = json.dumps({
    "contents": [{"role": "user", "parts": [{"text": text}]}],
    "generationConfig": {
        "temperature": 0.2,
        "topP": 0.95,
        "maxOutputTokens": 16384,
    }
}).encode("utf-8")

url = f"https://generativelanguage.googleapis.com/v1/models/gemini-2.5-pro:generateContent?key={key}"
req = urllib.request.Request(url, data=req_body, headers={"Content-Type": "application/json"}, method="POST")

try:
    with urllib.request.urlopen(req, timeout=600) as resp:
        raw = resp.read().decode("utf-8")
except urllib.error.HTTPError as e:
    print(f"HTTPError {e.code}: {e.reason}")
    print(e.read().decode("utf-8")[:3000])
    sys.exit(1)
except Exception as e:
    print(f"Request error: {e}")
    sys.exit(1)

try:
    r = json.loads(raw)
except Exception as e:
    print(f"JSON parse error: {e}")
    print(raw[:3000])
    sys.exit(1)

if "candidates" not in r or not r["candidates"]:
    print("ERROR: no candidates")
    print(json.dumps(r, indent=2)[:3000])
    sys.exit(1)

for cand in r["candidates"]:
    for part in cand.get("content", {}).get("parts", []):
        if "text" in part:
            print(part["text"])

print()
print("---")
if "usageMetadata" in r:
    u = r["usageMetadata"]
    print("## Usage")
    print(f"- promptTokenCount: {u.get('promptTokenCount', '?')}")
    print(f"- candidatesTokenCount: {u.get('candidatesTokenCount', '?')}")
    print(f"- totalTokenCount: {u.get('totalTokenCount', '?')}")
    if "thoughtsTokenCount" in u:
        print(f"- thoughtsTokenCount: {u['thoughtsTokenCount']}")
print(f"- Finished: {subprocess.check_output(['date', '-Iseconds']).decode().strip()}")
