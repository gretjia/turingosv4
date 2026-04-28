#!/usr/bin/env python3
"""Gemini round-3 audit on CO1.7 v1.2 — narrow re-affirmation since Gemini round-2 PASSed.

Strategic re-check whether v1.2 patches preserved the round-2 PASS verdict.
"""
import json
import pathlib
import subprocess
import sys
import urllib.request

REPO = pathlib.Path("/home/zephryj/projects/turingosv4")
ENV_FILE = pathlib.Path("/home/zephryj/projects/turingosv3/.env")
OUT = REPO / "handover/audits/GEMINI_CO1_7_ROUND3_AUDIT_2026-04-28.md"

key = None
for line in ENV_FILE.read_text().splitlines():
    if line.startswith("GEMINI_API_KEY="):
        key = line.split("=", 1)[1].strip()
        break
if not key:
    sys.exit("ERROR: no GEMINI_API_KEY")

PROMPT = """You are Gemini DeepThink, the strategic architectural reviewer for TuringOS v4.

This is a **round-3 narrow re-affirmation** on CO1.7 v1.2. Round-2 you returned PASS (high conviction); Codex returned CHALLENGE (3 narrow patch blockers); conservative merged CHALLENGE. v1.2 closes those 3 Codex must-fix items + 1 typo. Round-3 verifies the v1.1 → v1.2 patches did NOT break your round-2 PASS-worthy aspects.

Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.

## What v1.2 changed (vs v1.1 you PASSed in round-2)

| ID | Change | Strategic angle |
|---|---|---|
| R2-C3 | Wave 4-B additive extension shipped: `CanonicalMessage::LedgerEntrySigning([u8;32])` + canonical_digest match arm + `transition_ledger_emitter::sign_ledger_entry`. Skeleton test 9 actually signs + verifies. | Was your round-2 question — verifies that the design you PASSed is now real |
| R2-K3 | head_t mutation deferred to CO1.7.5+. v1.x ledger owns `ledger_root_t` only. Spec § 0/§ 3/§ 5 updated. | Was your round-2 acceptable; this is tighter scope |
| R2-C2-CAS | `ObjectType::Transition` → `ObjectType::ProposalPayload` (existing variant) | Cosmetic |
| R2-typo | "8-field" → "9-field" | Cosmetic |

**Your round-2 verdict**: PASS, all 6 round-1 findings CLOSED, withdrew D1 objection. Top must-fix: None.

## Round-3 questions

**Q1**: did v1.2's `CanonicalMessage::LedgerEntrySigning([u8;32])` opaque-digest design (NOT carrying full payload struct) preserve the constitutional alignment + forward-compat clause you accepted in round-2? Specifically:
- Constitutional: still typed-sign-only invariant (no raw digest signer escapes)
- Forward-compat: future ledger-side variants still add new `CanonicalMessage::*` variants (NOT in-place edit)
- Cycle prevention: does the opaque-digest carry compromise any property?

**Q2**: head_t deferral to CO1.7.5+. Is deferring head_t mutation a tighter scope (your round-2 said "boundary clean enough") or does it leave a constitutional gap (Art 0.4 Q_t version-controlled)?

**Q3**: ObjectType change is minor — just verify spec consistency.

**Q4**: did v1.2 introduce any new strategic concerns:
- The opaque-digest design now means `transition_ledger.rs` is the single source of truth for what a LedgerEntry's signing payload looks like. Is that a desirable concentration?
- The new test case `signature_round_trip_and_transplant_defense` actually exercises real Ed25519 sign/verify. Does this lift implementation confidence to the level you'd want for PASS?

**Q5**: Holistic re-affirmation — does v1.2 still warrant your PASS? Or did anything regress?

## Output format

# Gemini CO1.7 Round-3 Audit
## Q1 CanonicalMessage opaque-digest design
## Q2 head_t deferral
## Q3 ObjectType cosmetic
## Q4 NEW v1.2 strategic concerns
## Q5 Holistic re-affirmation
## **VERDICT**: PASS / CHALLENGE / VETO
## Top must-fix (if CHALLENGE)
## Conviction (low/med/high)

Be concise — round-3 is closure-only, not fresh exploration.
"""

DOCS = [
    ("DOC: CO1.7 spec v1.2 (round-3 target)", "handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md"),
    ("DOC: skeleton v1.2 (round-3 target)", "src/bottom_white/ledger/transition_ledger.rs"),
    ("DOC: system_keypair.rs (v1.2 with extension)", "src/bottom_white/ledger/system_keypair.rs"),
    ("XREF: your round-2 audit (PASS)", "handover/audits/GEMINI_CO1_7_ROUND2_AUDIT_2026-04-28.md"),
    ("XREF: Codex round-2 audit (CHALLENGE; 3 must-fix items v1.2 closes)", "handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md"),
    ("XREF: Constitution.md", "constitution.md"),
]

parts = [PROMPT]
for label, rel in DOCS:
    p = REPO / rel
    parts.append(f"\n\n=== {label} ===\n\n")
    parts.append(p.read_text())
text = "".join(parts)

with OUT.open("w") as f:
    f.write(f"# Gemini CO1.7 transition_ledger Round-3 Audit Run\n")
    f.write(f"- Model: gemini-2.5-pro\n")
    f.write(f"- Packet chars: {len(text)}\n")
    f.write(f"- Started: {subprocess.check_output(['date', '-Iseconds']).decode().strip()}\n")
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
