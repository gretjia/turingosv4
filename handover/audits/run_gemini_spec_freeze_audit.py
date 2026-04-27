#!/usr/bin/env python3
"""Gemini independent audit on STATE_TRANSITION_SPEC v1.1 FINAL FREEZE.

CO1.SPEC.0.5 gate before CO P1 launch. Codex is auditing in parallel.
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

User triggered path A: WP finalization tag signed; constitution amendments unfrozen; this is the FINAL spec freeze gate (CO1.SPEC.0.5) before CO Phase 1 launch.

You audited STATE_TRANSITION_SPEC v1 in the v3.2 cross-review (2026-04-27). Your CHALLENGE was incomplete pseudocode (only WorkTx). Claude responded with v1.1 adding VerifyTx + ChallengeTx + ReuseTx + finalize_reward + terminal_summary pseudocode + 4 new invariants. Plus walk-through found 4 gaps; v1.1 also added stage 4e + royalty cap + 2 more invariants. Total: 22 named invariants, 5 transition functions.

Codex is auditing in parallel from the code-grounded angle. Your audit is the strategic / cross-section angle.

Q1. **Spec completeness for ship gate**: is v1.1 truly ready as the BINDING contract for STEP_B branch A vs branch B comparison? Any concerns vs your prior CHALLENGE?

Q2. **22 invariants vs whole-system requirements**: cross-reference 22 transition invariants against:
- 12 economic invariants (WP § 18) — covered?
- 24 V tape canonical violations (Const Art 0.2) — covered?
- Constitution Art I-V philosophical principles — covered?
- Phase 3 prep deliverables (META_TX_SCHEMA / meta_validator) — covered?
What invariants are MISSING that v4 ship would expose?

Q3. **5 transition functions sufficiency**: work / verify / challenge / reuse / finalize_reward / terminal_summary — is this set complete for v4 scope? E.g.:
- Stake withdrawal / refund (when does Solver get stake back if no challenge)?
- Bounty refund (if task expires without acceptance)?
- Agent reputation explicit init (when Agent first joins)?
- Predicate registry update (covered by MetaTx? but MetaTx is v4.1 runtime...)

Q4. **Cross-spec consistency**: spec v1.1 references:
- GENESIS_MINIMAL_WITH_ANCHOR_v1 (boot)
- SYSTEM_KEYPAIR_SECURITY_v1 (system signing)
- META_TX_SCHEMA_v1 (Phase 3 prep)
- MetaTransitionInterface (v4.1 trait)
Are all references coherent? Any chained type that fails to type-check across specs?

Q5. **Determinism guarantees**: spec § 3 promises "byte-identical output for same inputs". Cross-platform determinism (Linux/macOS x86_64/aarch64) — is f64 IEEE 754 enough? Any sleeper non-determinism? (Codex is checking this but you also assess.)

Q6. **TaskMarket config defaults** (gap fixes): false_challenge=0, verifier_bond=Return, royalty_cap=0.10, quorum=1. Are these defaults game-theoretically sound for the 5-mode ablation experiments planned in Phase C?

Q7. **Walk-through gap closure**: did v1.1 ACTUALLY close all 4 gaps from SPEC_WALKTHROUGH_v1, or are they shifted to "configurable per TaskMarket" (deferring to runtime config)?

Q8. **STEP_B feasibility prediction**: bus.rs + kernel.rs are ~870 LOC of intertwined state. Spec v1.1 + STEP_B parallel branches would have Claude-A and Codex-B both generating ~800-1000 LOC each. Realistic prediction: how many spec ambiguities will surface during STEP_B? What's the audit cost of resolving them?

Q9. **Holistic verdict on v1.1 freeze**:
- STATE_TRANSITION_SPEC v1.1: PASS / CHALLENGE / VETO
- Ready for CO P1 launch: GO / NO-GO / NEEDS-FIX
- Should v1.2 patches happen first?

Q10. **What you'd want Codex to check next**: areas you can't validate strategically; final defer.

Format:
# Gemini Spec Freeze Audit (CO1.SPEC.0.5)
## Q1-Q10 with verdicts [PASS/CHALLENGE/VETO]
## Cross-cutting concerns
## Holistic verdict: PASS / CHALLENGE / VETO
## Top-3 must-fix before CO P1 launch (if any)
## Recommendation: CO P1 GO / NO-GO / NEEDS-FIX

Be rigorous; do not flatter.
"""

DOCS = [
    ("DOC: STATE_TRANSITION_SPEC v1.1 (FINAL FREEZE candidate)",
     "handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md"),
    ("XREF: GENESIS_MINIMAL_WITH_ANCHOR_v1", "handover/specs/GENESIS_MINIMAL_WITH_ANCHOR_v1_2026-04-27.md"),
    ("XREF: SYSTEM_KEYPAIR_SECURITY_v1", "handover/specs/SYSTEM_KEYPAIR_SECURITY_v1_2026-04-27.md"),
    ("XREF: META_TX_SCHEMA_v1", "handover/specs/META_TX_SCHEMA_v1_2026-04-27.md"),
    ("XREF: META_TRANSITION_INTERFACE_v1", "handover/specs/META_TRANSITION_INTERFACE_v1_2026-04-27.md"),
    ("XREF: SPEC_WALKTHROUGH_v1", "handover/specs/SPEC_WALKTHROUGH_v1_2026-04-27.md"),
    ("XREF: STATE_TRANSITION_SPEC_TLA (skeleton)", "handover/specs/STATE_TRANSITION_SPEC_TLA_2026-04-27.tla"),
    ("XREF: TRACE_MATRIX_v3 (~80 conformance tests target)", "handover/alignment/TRACE_MATRIX_v3_2026-04-27.md"),
    ("XREF: WP architecture (post-revision)", "handover/whitepapers/TURINGOS_WHITEPAPER_v1_2026-04-26.md"),
    ("XREF: WP economic (post-revision)", "handover/whitepapers/TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md"),
    ("XREF: Constitution", "constitution.md"),
    ("XREF: Plan v3.2-fix1 (atom dependency)", "handover/architect-insights/CO_MEGA_PLAN_v3.2_2026-04-27.md"),
    ("XREF: Sprint dep graph", "handover/architect-insights/SPRINT_DEPENDENCY_GRAPH_v1_2026-04-27.md"),
]

parts = [PROMPT]
for label, rel in DOCS:
    p = REPO / rel
    parts.append(f"\n\n=== {label} ===\n\n")
    parts.append(p.read_text())
text = "".join(parts)

print(f"# Gemini Spec Freeze Audit Run")
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
