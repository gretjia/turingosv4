#!/usr/bin/env python3
"""Gemini audit on Whitepaper v2 — Anti-Oreo Restoration (2026-04-27).

Cross-checks v2 against:
- constitution.md (supreme alignment)
- v2 tactical alignment note
- v1 whitepaper (predecessor; diff sanity)
- 4 OBS files (sedimented novel doctrine)

Conservative-wins protocol: VETO > CHALLENGE > PASS.
"""
import json
import pathlib
import subprocess
import sys
import urllib.request
import urllib.error

REPO = pathlib.Path("/home/zephryj/projects/turingosv4")
ENV_FILE = pathlib.Path("/home/zephryj/projects/turingosv3/.env")
key = None
for line in ENV_FILE.read_text().splitlines():
    if line.startswith("GEMINI_API_KEY="):
        key = line.split("=", 1)[1].strip()
        break
if not key:
    sys.exit("ERROR: no GEMINI_API_KEY")

PROMPT = """You are Gemini DeepThink. Audit `TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md` — a tactical constitutional-level alignment document ratified 2026-04-27 by the sole human architect (user).

**Status**: v2 is NOT a constitution.md edit. constitution.md remains supreme. v2 acts as the highest校准 mirror over derivative documents (Plan v3.2 / Blueprint / v1 whitepaper / Deepthink). The user explicitly chose "tactical alignment" over "full constitutional merge" to skip Phase Z′ 6-stage rerun + TRACE_MATRIX rewrite + conformance test rebuild.

**Your mandate**: skeptical adversarial review. Conservative-wins (memory `feedback_dual_audit_conflict`): VETO > CHALLENGE > PASS. Cite §/file:line for every finding.

**Audit ten questions**:

Q1. **Anti-Oreo coherence**: Does v2 preserve constitution.md's Anti-Oreo three-layer structure (⚪⚫⚪) without weakening it? Specifically does v2 § 3 give bottom-white tools equal weight with top-white predicates?

Q2. **ChainTape-as-implementation discipline**: v2 § 公理 5 + § 13 + § 18 explicitly forbid "blockchain as the body of TuringOS." Does v2 maintain this discipline throughout? Any text that accidentally reverts to chain-as-primary?

Q3. **Constitutional consistency**: Are there any v2 statements that contradict constitution.md (Art 0 Turing fundamentalism + Art 0.1 4-element mapping + Art 0.2 Tape Canonical + Art 0.3 区块链化保留 + Art 0.4 Q_t version-controlled + Art I-V signal management + Art IV Boot + Art V Go Meta)? List each conflict precisely.

Q4. **Q_t five-root extension (§ 4)**: Does adding state_root_t / tape_view_t / ledger_root_t / budget_state_t / predicate_registry_root_t / tool_registry_root_t to Q_t violate Art 0.4? Or is this a faithful refinement (Q_t schema preserved, derived roots added)?

Q5. **创造域 vs 安全域 dual rejection (§ 7.2)**: Is this consistent with Art I.1 boolean predicate semantics? Or does it introduce a soft-kernel exception?

Q6. **Predicate visibility trinity (§ 5.1 Layer 1 + § 9.4)**: Public / Private / Commit-Reveal. Already partly implemented (CO1.5 Visibility enum). Is v2's framing consistent with Art III.4 Goodhart shielding?

Q7. **Engineering PCP (§ 6.4)**: "正确候选应尽量不被误杀，错误候选应高概率被拦截." Does this avoid math overcommitment (PCP theorem)? Is the PCP-as-engineering-doctrine framing sound?

Q8. **6-layer ChainTape (§ 5.1)**: Layer 0 Constitution Root → Layer 6 Signal Indices. Is the layering logically clean? Any layer overlap or missing layer? Does Layer 4 transition ledger correctly subsume the deferred CO1.7 atom?

Q9. **5-phase roadmap (§ 17)**: GitTape → LedgerTape → MetaTape → PermissionedChainTape → PublicSettlement. Is this consistent with Plan v3.2 ~170 atoms? The tactical alignment note declares v2's roadmap is a "narrative overlay" not a renumber. Is that workable?

Q10. **Tactical-alignment status sustainability**: Can v2 reasonably function as "highest校准 mirror above derivative docs but below constitution.md"? Or does this dual authority create ambiguity that will fail under future Wave decisions?

**Final verdict**: PASS / CHALLENGE / VETO + holistic verdict + recommendation. If CHALLENGE/VETO, list must-fix items by §.

**Format**:
# Gemini Whitepaper v2 Audit
## Q1-Q10 verdicts (with §/file:line citations)
## Holistic verdict
## Must-fix (if any)
## Recommendation: RATIFICATION HOLDS / NEEDS-V2.1-PATCH / RETRACT-AND-REWRITE
"""

DOCS = [
    ("DOC: WHITEPAPER v2 (subject)", "handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md"),
    ("XREF: constitution.md (supreme)", "constitution.md"),
    ("XREF: WHITEPAPER_v2 tactical alignment note", "handover/alignment/WHITEPAPER_v2_TACTICAL_ALIGNMENT_2026-04-27.md"),
    ("XREF: OBS dual-domain", "handover/alignment/OBS_WHITEPAPER_V2_DUAL_DOMAIN_2026-04-27.md"),
    ("XREF: OBS predicate-visibility-trinity", "handover/alignment/OBS_WHITEPAPER_V2_PREDICATE_VISIBILITY_TRINITY_2026-04-27.md"),
    ("XREF: OBS Q_t five-root extension", "handover/alignment/OBS_WHITEPAPER_V2_QT_FIVE_ROOT_EXTENSION_2026-04-27.md"),
    ("XREF: OBS InitAI placeholder", "handover/alignment/OBS_WHITEPAPER_V2_INITAI_PLACEHOLDER_2026-04-27.md"),
    ("XREF: v1 whitepaper (predecessor)", "handover/whitepapers/TURINGOS_WHITEPAPER_v1_2026-04-26.md"),
    ("XREF: Blueprint v1", "handover/whitepapers/TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md"),
]

parts = [PROMPT]
for label, rel in DOCS:
    p = REPO / rel
    if not p.exists():
        print(f"# WARN: missing {rel}", file=sys.stderr)
        continue
    parts.append(f"\n\n=== {label} ===\n\n")
    parts.append(p.read_text())
text = "".join(parts)

print("# Gemini Whitepaper v2 Audit Run")
print("- Model: gemini-2.5-pro")
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
