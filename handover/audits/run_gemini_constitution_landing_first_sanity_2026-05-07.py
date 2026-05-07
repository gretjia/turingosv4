#!/usr/bin/env python3
"""Gemini Constitution Landing First sanity audit (HARNESS §1 H5).

Gemini half of the H5 dual audit. Codex implementation paranoia closed via
C-LAND-1; this runner asks Gemini 2.5 Pro for an independent architectural
sanity verdict against the b7bde23 substrate plus Wave 3 real-LLM evidence.
"""
import json
import os
import re
import sys
import time
import urllib.request
from pathlib import Path

ROOT = Path("/home/zephryj/projects/turingosv4")
ENV_FILES = [
    Path("/home/zephryj/projects/turingosv3/.env"),
    ROOT / ".env",
]
ROUND = os.environ.get("TB_AUDIT_ROUND", "R1")
OUT = ROOT / f"handover/audits/GEMINI_CONSTITUTION_LANDING_FIRST_SANITY_2026-05-07_{ROUND}.md"

if ROUND not in {"R1", "R2"}:
    print(
        "[gemini c-land sanity] error: TB_AUDIT_ROUND must be R1 or R2",
        file=sys.stderr,
    )
    sys.exit(2)

if OUT.exists():
    print(
        f"[gemini c-land sanity] error: {OUT} already exists; refusing to overwrite",
        file=sys.stderr,
    )
    sys.exit(2)


def load_env() -> dict[str, str]:
    env: dict[str, str] = {}
    for fp in ENV_FILES:
        if not fp.exists():
            continue
        for line in fp.read_text().splitlines():
            if "=" in line and not line.strip().startswith("#"):
                key, value = line.split("=", 1)
                env.setdefault(key.strip(), value.strip().strip('"').strip("'"))
    return env


def read_rel(rel: str) -> str:
    fp = ROOT / rel
    if not fp.exists():
        return f"(MISSING: {rel})"
    return fp.read_text()


def append_file(rel: str, lang: str = "") -> str:
    fp = ROOT / rel
    if not fp.exists():
        return f"\n\n---\n\n## {rel}\n\n(MISSING: expected file not found)\n"
    return f"\n\n---\n\n## {rel}\n\n```{lang}\n{fp.read_text()}\n```\n"


def extract_verdict(text: str) -> str:
    patterns = [
        r"(?im)^##\s*VERDICT:\s*(PASS|CHALLENGE|VETO)\b",
        r"(?im)^VERDICT:\s*(PASS|CHALLENGE|VETO)\b",
        r"(?im)^Final aggregate verdict:\s*(PASS|CHALLENGE|VETO)\b",
        r"(?im)^Aggregate verdict:\s*(PASS|CHALLENGE|VETO)\b",
    ]
    for pattern in patterns:
        match = re.search(pattern, text)
        if match:
            return match.group(1).upper()
    for token in ("VETO", "CHALLENGE", "PASS"):
        if re.search(rf"\b{token}\b", text):
            return token
    return "UNKNOWN"


def extract_q_breakdown(text: str) -> str:
    lines = []
    for qid in range(1, 9):
        pattern = rf"(?ims)(?:^|\n)(?:#+\s*)?(Q{qid}\b.*?)(?=\n(?:#+\s*)?Q{qid + 1}\b|\n##\s*VERDICT:|\nVERDICT:|\Z)"
        match = re.search(pattern, text)
        if match:
            block = match.group(1).strip()
            first_lines = block.splitlines()[:8]
            lines.append("\n".join(first_lines))
        else:
            lines.append(f"Q{qid}: (not machine-extracted; see verbatim response below)")
    return "\n\n".join(lines)


env = load_env()
if "GEMINI_API_KEY" not in env:
    print("[gemini c-land sanity] GEMINI_API_KEY not found in .env files", file=sys.stderr)
    sys.exit(2)

brief = f"""# Gemini Constitution Landing First Architecture Sanity Audit — {ROUND}

You are Gemini 2.5 Pro acting as a skeptical architectural reviewer for
TuringOS v4. This is the Gemini half of HARNESS.md §1 H5 dual-audit.
Codex has already completed the implementation-paranoid half via C-LAND-1
VETO -> PASS (`handover/audits/CODEX_*CLAND*`). Your job is independent
architecture sanity review, not rubber-stamping Codex.

Conservative merge ranking applies per `feedback_dual_audit_conflict`:
VETO > CHALLENGE > PASS. Prefer CHALLENGE over PASS when a concrete
architectural uncertainty has an actionable remediation. Use VETO only for
ship-blocking inconsistency with the Constitution, harness, or architect-pinned
schemas.

Round cap is 2. This run is {ROUND}. If this is R2, evaluate whether the
remaining R1 challenges were answered; do not invent new scope unless it is a
true blocker visible in the attached materials.

## Audit target

Target substrate commit: b7bde23 `Constitution Landing First -- G-009 + G-012 + G-016 substrate landed`

- `src/state/head_t_witness.rs`: Class-3 G-009 C1 six-field witness derived over QState. Architect §4.1 schema is `state_root`, `l4_head`, `l4e_head`, `cas_root`, `economic_state_root`, `run_id`.
- `src/runtime/prompt_capsule.rs`: Class-3 G-016/G-019/G-021/G-028 seven-field PromptCapsule per architect §4.3. Constructor must refuse `hidden_fields_redacted=false`.
- `cases/pcp_corpus/`: G-012 nine-class adversarial corpus. Manifest pins AttemptOutcome -> L4ERejectionClass byte-stable mapping.
- Test surfaces: `tests/constitution_head_t_witness.rs` (5 tests), `tests/constitution_pcp_corpus.rs` (7 tests), `tests/constitution_prompt_capsule.rs` (8 tests).
- Matrix rows in `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`: §A Art.0.4, §B Art.I.1.1, §D Art.III prompt-persistence all promoted to GREEN in b7bde23.

## Evidence binding

Wave 3 real-LLM evidence must be considered as binding context, not as a
separate implementation audit:

- commit ffb6ebd: `handover/evidence/wave3_diagnostic_20p_2026-05-07T13-08-06Z/WAVE3_AGGREGATE.json`; FC1 invariant 140 = 7 + 129 + 4.
- commit a612cc9: `handover/evidence/wave3_diagnostic_50p_2026-05-07T14-04-48Z/WAVE3_50P_AGGREGATE.json`; FC1 invariant 460 = 9 + 400 + 51.

## Required questions

Answer Q1 through Q8 explicitly. Mark each as PASS / CHALLENGE / VETO.

Q1. Is HEAD_t C1 as a derived view sufficient for FC2 replay assertions, or
does it need persisted state now to avoid drift?

Q2. Does PromptCapsule exactly match the architect §4.3 seven-field schema,
and is refusing `hidden_fields_redacted=false` in the constructor the correct
hard fence?

Q3. Does the PCP corpus cover the architect-pinned nine-class mutation
taxonomy? Is any mutation class missing or misclassified?

Q4. Does the 50-problem Wave 3 evidence, especially FC1 invariant
460 = 9 + 400 + 51, provide sufficient real-LLM-tape proof for PROJECT_PLAN
§3 resume condition #6: P38/P49 attempt equality green?

Q5. Is Class-3 classification for `head_t_witness` and `prompt_capsule`
consistent with `feedback_class4_cannot_hide_in_class3`? In particular, confirm
neither changes sequencer admission semantics nor canonical signing payload.

Q6. Were Constitution matrix promotions for Art.0.4, Art.I.1.1, and Art.III
prompt-persistence to GREEN earned under CR-C0.7's standard that GREEN means
real path and passes?

Q7. Is deferring Wave 1/2 cleanup defensible? LATEST session #16 argues
PROJECT_PLAN §3 closure bypassed the Wave 1/2 path. Is that a strict-alignment
win because resume conditions are what bind, or covert sequencing manipulation?

Q8. Strategic risk: what, if anything, is visibly wrong or missing in the
substrate that next-stage TB-18R Final or TB-18B benchmark scale-up would expose?

## Verdict format

Use exactly this shape so the runner can extract the result:

```
Q1: PASS|CHALLENGE|VETO - <reason>
...
Q8: PASS|CHALLENGE|VETO - <reason>

## VERDICT: PASS|CHALLENGE|VETO
Conviction: low|medium|high
Recommendation: PROCEED|FIX-THEN-PROCEED|REDESIGN
Remediations:
- <only for CHALLENGE/VETO; actionable and scoped>
```

If any Q is CHALLENGE, aggregate verdict must be CHALLENGE unless another Q is
VETO. If any Q is VETO, aggregate verdict must be VETO.

---

# Ground Truth Excerpts

## constitution.md §0.4 + §I.1.1 + §III.*

```markdown
{read_rel("constitution.md")}
```

## HARNESS.md §1 H5 and related gates

```markdown
{read_rel("HARNESS.md")}
```

## PROJECT_PLAN.md §3 resume conditions

```markdown
{read_rel("PROJECT_PLAN.md")}
```

## CLAUDE.md §4.1 + §4.2 + §4.3 architect-pinned schemas

```markdown
{read_rel("CLAUDE.md")}
```
"""

for rel, lang in [
    ("src/state/head_t_witness.rs", "rust"),
    ("src/runtime/prompt_capsule.rs", "rust"),
    ("cases/pcp_corpus/README.md", "markdown"),
    ("cases/pcp_corpus/MANIFEST.json", "json"),
    ("tests/constitution_head_t_witness.rs", "rust"),
    ("tests/constitution_pcp_corpus.rs", "rust"),
    ("tests/constitution_prompt_capsule.rs", "rust"),
    ("handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md", "markdown"),
    ("handover/evidence/wave3_diagnostic_20p_2026-05-07T13-08-06Z/WAVE3_AGGREGATE.json", "json"),
    ("handover/evidence/wave3_diagnostic_50p_2026-05-07T14-04-48Z/WAVE3_50P_AGGREGATE.json", "json"),
]:
    brief += append_file(rel, lang)

print(f"[gemini c-land sanity] prompt size: {len(brief):,} chars", file=sys.stderr)

url = (
    "https://generativelanguage.googleapis.com/v1beta/models/"
    f"gemini-2.5-pro:generateContent?key={env['GEMINI_API_KEY']}"
)
body = json.dumps(
    {
        "contents": [{"parts": [{"text": brief}]}],
        "generationConfig": {"temperature": 0.2, "maxOutputTokens": 16384},
    }
).encode()
req = urllib.request.Request(
    url,
    data=body,
    headers={"Content-Type": "application/json"},
    method="POST",
)

t0 = time.time()
try:
    with urllib.request.urlopen(req, timeout=900) as resp:
        data = json.loads(resp.read())
except Exception as exc:
    print(f"[gemini c-land sanity] error: {exc}", file=sys.stderr)
    sys.exit(1)

elapsed = time.time() - t0
print(f"[gemini c-land sanity] API returned in {elapsed:.1f}s", file=sys.stderr)

try:
    text = data["candidates"][0]["content"]["parts"][0]["text"]
except (KeyError, IndexError) as exc:
    print(f"[gemini c-land sanity] malformed API response: {exc}", file=sys.stderr)
    print(json.dumps(data, indent=2)[:4000], file=sys.stderr)
    sys.exit(1)

verdict = extract_verdict(text)
q_breakdown = extract_q_breakdown(text)
header = f"""# Gemini Constitution Landing First Sanity Audit

**Round**: {ROUND}
**Date**: 2026-05-07
**Model**: gemini-2.5-pro
**Elapsed**: {elapsed:.1f}s
**Prompt size**: {len(brief):,} chars
**Final aggregate verdict**: {verdict}

## Extracted Per-Q Breakdown

{q_breakdown}

---

## Verbatim Gemini Response

"""

OUT.write_text(header + text)
print(f"[gemini c-land sanity] saved: {OUT}")
print(f"[gemini c-land sanity] verdict: {verdict}")
