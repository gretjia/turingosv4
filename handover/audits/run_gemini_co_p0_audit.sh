#!/bin/bash
# CO P0.7 Gemini DeepThink audit on Blueprint + Plan v3.1 + Protocol + Amendment v1
# Usage: bash run_gemini_co_p0_audit.sh > GEMINI_CO_P0_AUDIT_2026-04-26.md 2>&1

set -euo pipefail
cd "$(dirname "$0")/../.."

# Source key
if [ -f /home/zephryj/projects/turingosv3/.env ]; then
  set -a
  # shellcheck disable=SC1091
  source /home/zephryj/projects/turingosv3/.env
  set +a
fi

if [ -z "${GEMINI_API_KEY:-}" ]; then
  echo "ERROR: GEMINI_API_KEY not found in /home/zephryj/projects/turingosv3/.env" >&2
  exit 1
fi

PACKET=$(mktemp)
trap 'rm -f "$PACKET"' EXIT

cat > "$PACKET" <<'PROMPT_END'
You are Gemini DeepThink, the strategic architectural reviewer for TuringOS v4 per the TRI_MODEL_ORCHESTRATION_PROTOCOL.

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

Q1. **Coverage**: does the Blueprint actually cover EVERY § in both white papers? Are there any white paper concepts that lack a code symbol or test mapping?

Q2. **Anti-Oreo soundness**: are any modules (in src/{top_white,middle_black,bottom_white,economy,state,transition}) ambiguously layered? Is the Goodhart shield (private predicate visibility) actually leak-proof?

Q3. **Economic invariants completeness**: do the 12 invariants → 12 conformance tests actually catch the invariant violations they claim? Especially Inv 8 (DAG attribution) — is determinism actually achievable?

Q4. **Plan v3.1 atom dependency soundness**: do CO P1 atoms (50-65) truly precede CO P2 atoms in implementation order? Any circular dependency or blocking pair?

Q5. **STEP_B atom risk**: bus.rs + kernel.rs splits (CO1.1.4, CO1.1.5) are listed as 5-way / 3-way parallel branches. Is parallel branch the right protocol, or is sequential refactor safer?

Q6. **Tri-model protocol soundness**: per-atom workflow has Generator ≠ Evaluator. Is that achievable when the heaviest implementer (Codex) is also the heaviest reviewer? Risk of Codex marking own code PASS?

Q7. **Cost projection realism**: $435-950 — is this honest given 132 atoms × multi-model audits? Or under-estimating?

Q8. **D-decisions all-rec sanity**: D5=A full RSP increases atom count; D4=B defers MetaTape; are these compatible (i.e., can full RSP be tested without MetaTape ArchitectAI runtime)?

Q9. **Constitution Art 0.5 draft (6 axioms)**: are 6 axioms the right granularity? Should it be expanded or compressed?

Q10. **PREREG amendment v2 (D1=C MVP-pivot)**: 50 rows × 1 seed for Phase C MVP — is this enough statistical power to declare H1-H4 supported/rejected?

Format your response as:

# Gemini CO P0.7 Strategic Audit

## Q1-Q10 detailed answers (one paragraph per Q with verdict tag [PASS/CHALLENGE/VETO])

## Cross-cutting concerns (anything you noticed that the 10 questions miss)

## Holistic verdict
- Blueprint: PASS / CHALLENGE / VETO
- Plan v3.1: PASS / CHALLENGE / VETO
- Protocol: PASS / CHALLENGE / VETO
- Amendment v1: PASS / CHALLENGE / VETO

## Top-3 must-fix items (if any) before CO P1 entry

## What you're uncertain about (areas you'd defer to Codex for deep-dive)

PROMPT_END

# Append actual document contents (Gemini reads them inline)
{
  echo
  echo "=== DOCUMENT 1: TURINGOS_v4_FINAL_BLUEPRINT ==="
  cat handover/whitepapers/TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md
  echo
  echo "=== DOCUMENT 2: CO_MEGA_PLAN_v3.1 ==="
  cat handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md
  echo
  echo "=== DOCUMENT 3: TRI_MODEL_ORCHESTRATION_PROTOCOL ==="
  cat handover/architect-insights/TRI_MODEL_ORCHESTRATION_PROTOCOL_2026-04-26.md
  echo
  echo "=== DOCUMENT 4: CO_P0_AMENDMENT_v1 ==="
  cat handover/architect-insights/CO_P0_AMENDMENT_v1_2026-04-26.md
  echo
  echo "=== CROSS-REF A: TURINGOS_WHITEPAPER_v1 (architecture) ==="
  cat handover/whitepapers/TURINGOS_WHITEPAPER_v1_2026-04-26.md
  echo
  echo "=== CROSS-REF B: TURINGOS_WHITEPAPER_v1_ECONOMIC ==="
  cat handover/whitepapers/TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md
  echo
  echo "=== CROSS-REF C: constitution.md ==="
  cat constitution.md
} >> "$PACKET"

# Build JSON request
REQ=$(mktemp)
trap 'rm -f "$PACKET" "$REQ"' EXIT

python3 <<PY > "$REQ"
import json, sys
with open("$PACKET", "r") as f:
    text = f.read()
print(json.dumps({
    "contents": [{"role": "user", "parts": [{"text": text}]}],
    "generationConfig": {
        "temperature": 0.2,
        "topP": 0.95,
        "maxOutputTokens": 16384,
    }
}))
PY

echo "# Gemini CO P0.7 Audit Run"
echo
echo "- Started: $(date -Iseconds)"
echo "- Model: gemini-2.5-pro"
echo "- Packet bytes: $(wc -c < "$PACKET")"
echo
echo "---"
echo

RAW=$(mktemp)
trap 'rm -f "$PACKET" "$REQ" "$RAW"' EXIT

curl -sS \
  -H "Content-Type: application/json" \
  -X POST \
  --max-time 600 \
  -d @"$REQ" \
  "https://generativelanguage.googleapis.com/v1/models/gemini-2.5-pro:generateContent?key=${GEMINI_API_KEY}" \
  > "$RAW"

python3 - "$RAW" <<'PY'
import json, sys
raw = open(sys.argv[1]).read()
try:
    r = json.loads(raw)
except Exception as e:
    print("JSON PARSE ERROR:", e)
    print(raw[:3000])
    sys.exit(1)

if "candidates" in r and r["candidates"]:
    for cand in r["candidates"]:
        for part in cand.get("content", {}).get("parts", []):
            if "text" in part:
                print(part["text"])
    print()
    print("---")
    if "usageMetadata" in r:
        u = r["usageMetadata"]
        prompt = u.get("promptTokenCount", "?")
        cand_t = u.get("candidatesTokenCount", "?")
        total = u.get("totalTokenCount", "?")
        print("## Usage: prompt={} candidates={} total={}".format(prompt, cand_t, total))
else:
    print("ERROR: no candidates")
    print(json.dumps(r, indent=2)[:3000])
PY

echo
echo "---"
echo
echo "- Finished: $(date -Iseconds)"
