#!/usr/bin/env python3
"""TB-16 Round 3 Gemini ship audit — architectural strategic review of R3
surgical fixes for Gemini's own R2 VETO + CHALLENGEs. Independent of Codex
R3 (parallel, impl-paranoid angle).

Per memory feedback_dual_audit + feedback_dual_audit_conflict: VETO > CHALLENGE > PASS.
Per feedback_audit_loop_roi_flip: stop iterating if challenges shift to
test-scaffold edges.
"""
import json
import os
import sys
import time
import urllib.request
from pathlib import Path

ROOT = Path("/home/zephryj/projects/turingosv4")
ENV_FILES = [ROOT / ".env"]
ROUND = os.environ.get("TB16_AUDIT_ROUND", "R3")
OUT = ROOT / f"handover/audits/GEMINI_TB_16_SHIP_AUDIT_2026-05-04_{ROUND}.md"

if OUT.exists():
    print(f"[gemini tb-16 r3] error: {OUT} already exists; refusing to overwrite", file=sys.stderr)
    sys.exit(2)


def load_env():
    env = {}
    for fp in ENV_FILES:
        if not fp.exists():
            continue
        for line in fp.read_text().splitlines():
            if "=" in line and not line.strip().startswith("#"):
                k, v = line.split("=", 1)
                env.setdefault(k.strip(), v.strip().strip('"').strip("'"))
    return env


env = load_env()
if "GEMINI_API_KEY" not in env:
    print("[gemini tb-16 r3] GEMINI_API_KEY not in .env", file=sys.stderr)
    sys.exit(2)

# ── Brief ──
brief = """# Gemini TB-16 Round 3 Ship Audit — Controlled Market Smoke Arena (Class 3, architectural strategic review)

**Role**: skeptical adversarial reviewer for the **TB-16 ROUND 3** convergence audit. Independent of Codex R3 (parallel, impl-paranoid angle). Conservative merge rule (memory `feedback_dual_audit_conflict`): VETO > CHALLENGE > PASS.

**Mandate**: TB-16 shipped under **Class 3 integration smoke envelope** per architect §7.7. **You are the architectural strategic reviewer**; Codex covers implementation paranoia in parallel. **Round cap = 2 has been reached at R3 in the conservative-merge cycle** (your R1 + R2 already issued; this is your R3 convergence round) — per `feedback_audit_loop_roi_flip` if your R3 challenges shift to test-scaffold edges, iteration ROI has flipped — note that explicitly.

## R1 → R2 → R3 history (for grounding)

```text
R1 (your prior round; commit 3300fe2 = Atom 6 ship pre-audit):
  Your verdict: VETO Q11 (TRACE_MATRIX precision: tamper assertions belong
  to FC1-N35, not FC1-N34) + CHALLENGE×N
  Output: handover/audits/GEMINI_TB_16_SHIP_AUDIT_2026-05-04_R1.md
  Codex parallel: VETO×5 (V2-V7)

R2 (your prior round; post Step 1+3+4):
  Your verdict: VETO×5 of which 4 STALE (audit prompt predated Step 4;
  Q5/Q8/Q9 referenced pre-Step-4 deferral framing) + 1 REAL (Q2 JSON
  byte-run privacy) + CHALLENGE×5 (Q1 per-block conservation, Q3 replay
  parity, Q6 Class 3 misclass, Q7 tamper attack-vector, Q10 machine-
  verifiable CR-16.7, Q11 file-level TRACE_MATRIX, Q12 test-count math).
  Output: handover/audits/GEMINI_TB_16_SHIP_AUDIT_2026-05-04_R2.md
  Codex parallel: skipped at R2.
  Closure analysis: handover/audits/RECURSIVE_AUDIT_TB_16_R2_2026-05-04.md

Step 1 commit 3cf4c36 (closes Codex V3-V7 + your Q11 + Codex V2-audit).
Step 3 commit 05e3e86 (evaluator arena env-var hooks).
Step 4 commit d1c1af2 (fresh real-LLM arena runs):
  • arena_run4/ (mathd_algebra_171, happy): PROCEED 7 tx kinds
  • arena_run6_exhaust/ (aime_1997_p9): PROCEED 4 tx kinds
  • Aggregate 9 of 13 architect-required tx kinds
  • TB-11 writer-pattern bug fix in src/runtime/evidence_capsule.rs
    (mirrors TB-15 R2 Codex closure)

R3 prep commit 90848bb (THIS COMMIT YOU ARE AUDITING) closes:
  • Q2 (your R2 VETO; JSON byte-run) — assert_28 now scans BOTH (a) raw
    32-byte run in canonical_encode AND (b) JSON-array decimal text form
    via serde_json::to_string(&proj). Mirrors TB-15 halt-trigger #5 (R2).
  • Q1 (your R2 CHALLENGE; per-block conservation) — NEW supplemental
    assert_d_total_supply_conserved_per_block (id=40, Layer D). Replays
    entries[..=i] for every i; asserts total_supply_micro == initial.
  • Q10 (your R2 CHALLENGE; machine-verifiable CR-16.7) — NEW supplemental
    assert_a_chain_agent_ids_sandbox_prefixed (id=41, Layer A). Walks
    every L4 entry, decodes TypedTx, asserts HasSubmitter::submitter_id()
    is sandbox-prefixed.
  • Q11 (your R2 CHALLENGE; TRACE_MATRIX precision) — file-level doc-comment
    per-layer precise (Layers A-G + supplementals → FC1-N34; Layer H →
    FC1-N35; verdict.json → FC2-N31).
  • Q12 (your R2 CHALLENGE; test-count math) — TB-16 SHIP_STATUS §3 expanded
    with per-step delta from TB-15 R3 (882) → TB-16 R3 (907).
  • Q8 evidence regen (closure) — audit_pipeline_smoke MARKOV_TB-16
    regenerated with --prev-cid-hex chained to TB-15 head f9e701b4...

R3 also POSITION-HOLDS on Q4 (sandbox HALT vs banner): "non-sandbox funds
used" HALT in architect §7.7 is read parallel-structurally as audit-time
detection (Layer A #3 manifest fence + new id=41 chain-walk), NOT a
sequencer-level admission gate. CHALLENGE this position if you disagree
— but the implementer's position is that adding sequencer admission gate
exceeds architect spec and would be Class 3+/4 risk.
```

## OBS-deferred (NOT a R3 ship-blocker; pre-existing)

`audit_tape_tamper` Round 2 hangs on the `audit_pipeline_smoke` fixture.
**Verified pre-existing on git HEAD via `git stash` test**: reproduces
without any R3 fix changes. R1 ship-time tamper_report.json (3/3 detected,
committed `3cf4c36`) carry-forward valid since tamper LOGIC in
src/bin/audit_tape_tamper.rs was UNTOUCHED in R3.

Hypothesis: post-tamper audit_tape pipeline iterates a CAS loose object
whose back-half-zero corruption decodes (via git2 zlib partial) into bytes
that a bincode length-prefix path treats as unbounded.

See: `handover/alignment/OBS_TB_16_TAMPER_R2_HANG_2026-05-04.md`

## R3 audit questions — focused on R3 convergence

You have already audited R1 + R2. Do NOT re-litigate stale R2 VETOs (Q5/Q8/Q9
were stale per RECURSIVE_AUDIT_TB_16_R2). Focus on:

**RQ1 — Q2 closure adequacy**: assert_28_projection_no_autopsy_bytes
(post R3) at `src/runtime/audit_assertions.rs:1483-1573` now does both
canonical_encode raw byte scan AND serde_json::to_string JSON-array
decimal form scan. STRATEGIC: is the JSON-array form (`[b₀,b₁,…,b₃₁]`)
the COMPLETE coverage of agent-visible serialization paths, or are there
serialization paths (base64, hex string, tuple-array, debug Display impl)
that bypass both checks? Walk `AgentVisibleProjection` serde derive +
ProjectionWriter / projection.rs paths.

**RQ2 — Q1 closure adequacy**: assert_d_total_supply_conserved_per_block
(id=40) at `src/runtime/audit_assertions.rs:1199-1259` does prefix-replay
per-step. STRATEGIC: does this address the architectural concern (per-block
conservation), or is the O(N²) replay just a brute-force version of #18
that doesn't add new invariant coverage? What about: (a) chains where
some predicate dispatch fails mid-chain — does the walker correctly halt
on the first error? (b) chains where multiple txs in the SAME logical_t
window net to zero (e.g., MintReward + Slash) — does per-block coarseness
still catch sub-block leaks?

**RQ3 — Q10 closure adequacy**: assert_a_chain_agent_ids_sandbox_prefixed
(id=41) at `src/runtime/audit_assertions.rs:657-712`. STRATEGIC: this
closes the manifest-vs-chain gap (Layer A #3 was manifest-only; id=41
walks the chain). Is the closure sufficient for CR-16.7, or does CR-16.7
also require that REJECTED submissions (L4.E) be checked? L4.E doesn't
go through HasSubmitter today — is that an audit gap?

**RQ4 — Q4 position-hold (architect §7.7 parsing)**: implementer holds
position that "non-sandbox funds used" is parallel-structurally an audit-
time HALT, NOT sequencer admission gate. STRATEGIC: parse architect §7.7
verbatim. Is "audit-time HALT" semantically equivalent to "sequencer
admission gate" given that audit-time HALT triggers post-hoc on a chain
that already accepted the offending tx? If audit-time-only is acceptable,
is the position correctly defensible as "architect spec doesn't mandate
runtime gate"?

**RQ5 — Q11 closure**: file-level doc-comment now per-layer precise.
STRATEGIC: is the per-layer FC binding now correctly granular for R-022
TRACE_MATRIX consumers? Are there per-fn doc-comments that ALSO need to
be tightened (e.g., the supplemental id=39 `assert_f_no_llm_self_narrative_in_autopsy`
might still claim FC1-N34 when it's specifically a Layer F privacy fence
that could be argued is FC1-N34 sub-layer or even its own FC node)?

**RQ6 — Q12 closure (test-count math)**: TB-16 SHIP_STATUS §3 (post R3
edit) shows the per-step table from TB-15 R3 (882) → TB-16 R3 (907).
STRATEGIC: is the table arithmetically correct? Verify TB-15 R3 baseline
of 882 against `git show eddab36 -- handover/ai-direct/TB-15_SHIP_STATUS_2026-05-04.md`.
Verify the +25 / +2 / +0 deltas. Are there subtractive deltas (test
deletions across atoms) the table omits?

**RQ7 — OBS deferral fitness**: implementer deferred `audit_tape_tamper`
R2 hang as OBS at `handover/alignment/OBS_TB_16_TAMPER_R2_HANG_2026-05-04.md`.
STRATEGIC: per `feedback_audit_obs_bias` (R3-relevant memory: "After VETOs
clear → CHALLENGE-only, do NOT bucket-OBS all residuals; cheap fixes get
fixed; only OBS-defer multi-hour future-arch"): is the tamper hang
genuinely multi-hour future-arch (justifying OBS), or is it a cheap fix
the implementer should have made instead of OBS-deferring? Cite
diagnostic depth in OBS doc — the implementer ran 4 distinct
investigations (cp -r vs std::fs::copy isolation, BISECT-disable my Q1+Q2+Q10,
git stash + clean-tree reproduction, byte-diff inspection) before OBS
deferral. Is this rigorous enough for OBS, or does it need more depth?

**RQ8 — Class 3 envelope discipline at R3**: Implementer's R3 surgical
fixes added 2 NEW pure-fn supplemental assertions (id=40, id=41) + tightened
1 existing assertion (#28). Per `feedback_risk_class_audit`: Class 3 = production
wire-up + sequencer dispatch arm change. R3 fixes are AUDIT-SIDE READ paths
only — no sequencer dispatch modification, no AgentVisibleProjection mod,
no economic state surface change. STRATEGIC: should TB-16 retro-downgrade
to Class 2 (self-audit OK) given that R3 deltas are read-only additions,
OR does the cumulative TB-16 ship (Atoms 0-7 R3) still warrant Class 3
because the Atom 6 production wire-up + Atom 7 R3 cumulative scope is
non-trivial? Take an explicit position.

**RQ9 — Convergence vs divergence**: At R3 you have your own R1 + R2 +
this round of audit history. STRATEGIC: is the R3 surgical fixes path
showing CONVERGENT improvement (each round closing more findings than it
opens), or DIVERGENT (each round revealing new architectural gaps)?
Per `feedback_audit_loop_roi_flip`: if R3 reveals new gaps that need
another round of fixes, the audit ROI has flipped — recommend ship-with-OBS
+ TB-16.x continuation rather than R4. If R3 is convergent (PASS or
CHALLENGE-only), recommend SHIP.

## Verdict format (R3 convergence-round structure)

End your audit with one of:

```text
## VERDICT: PASS
(All RQ1-RQ9 cleared at convergence; R3 surgical fixes successfully closed
R2 findings; ship is clean for Class 3 envelope.)
```

```text
## VERDICT: CHALLENGE
- RQ<id> CHALLENGE: <one-line reason + line refs>
(round-cap reached at R3; per feedback_audit_loop_roi_flip +
feedback_audit_obs_bias, evaluate ship-with-OBS fitness for residuals.)
```

```text
## VERDICT: VETO
- RQ<id> VETO: <one-line BLOCKING reason + line refs>
(VETO at R3 per feedback_dual_audit_conflict; recommend escalate to
architect ratification or revert R3 prep.)
```

Include conviction (low/medium/high) + recommendation (PROCEED to SHIP /
SHIP-WITH-OBS / FIX-THEN-PROCEED / REDESIGN / RETRO-CLASS-2-DOWNGRADE).

---

"""


def append_file(rel: str, lang: str = "rust"):
    fp = ROOT / rel
    if not fp.exists():
        return f"\n## {rel}\n\n(MISSING — file not found at expected path)\n"
    return f"\n## {rel}\n\n```{lang}\n{fp.read_text()}\n```\n"


# ── Reference: charter + architect spec + ship status + R3 closure docs ──
brief += "# Reference: Charter + Architect spec + Ship status + R2 closure + OBS\n"
brief += append_file("handover/tracer_bullets/TB-16_charter_2026-05-04.md", "markdown")
brief += append_file("handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md", "markdown")
brief += append_file("handover/ai-direct/TB-16_SHIP_STATUS_2026-05-04.md", "markdown")
brief += append_file("handover/audits/RECURSIVE_AUDIT_TB_16_R2_2026-05-04.md", "markdown")
brief += append_file("handover/alignment/OBS_TB_16_TAMPER_R2_HANG_2026-05-04.md", "markdown")

# ── R1 + R2 audits (your prior rounds — for self-grounding) ──
brief += "\n\n---\n\n# Your prior audit rounds (for self-grounding)\n"
brief += append_file("handover/audits/GEMINI_TB_16_SHIP_AUDIT_2026-05-04_R1.md", "markdown")
brief += append_file("handover/audits/GEMINI_TB_16_SHIP_AUDIT_2026-05-04_R2.md", "markdown")

# ── R3 evidence ──
brief += "\n\n---\n\n# R3 evidence (audit_pipeline_smoke regenerated post R3)\n"
brief += append_file("handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/verdict.json", "json")
brief += append_file("handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/MARKOV_TB-16_2026-05-03.json", "json")
# Step 4 fresh-arena evidence
brief += append_file("handover/evidence/tb_16_real_llm_arena_2026-05-04/arena_run4/verdict.json", "json")
brief += append_file("handover/evidence/tb_16_real_llm_arena_2026-05-04/arena_run6_exhaust/verdict.json", "json")

# ── TB-16 source code under R3 audit ──
brief += "\n\n---\n\n# TB-16 source code under R3 audit (commit 90848bb)\n"
for rel in [
    "src/runtime/audit_assertions.rs",
    "src/bin/audit_tape.rs",
    "src/bin/audit_tape_tamper.rs",
    "src/runtime/evidence_capsule.rs",
    "src/state/typed_tx.rs",  # for HasSubmitter trait
]:
    if rel.endswith(".rs"):
        lang = "rust"
    elif rel.endswith(".sh"):
        lang = "bash"
    elif rel.endswith(".md"):
        lang = "markdown"
    else:
        lang = "text"
    brief += append_file(rel, lang)

brief += "\n---\n\nGive your INDEPENDENT TB-16 R3 ship audit. Be paranoid about RQ1-RQ9 (R3 surgical fix adequacy + convergence assessment + OBS deferral fitness). Cite file:line for every finding. Conclude with VERDICT.\n"

print(f"[gemini tb-16 r3] prompt size: {len(brief):,} chars", file=sys.stderr)

# ── Call ──
url = (
    "https://generativelanguage.googleapis.com/v1beta/models/"
    f"gemini-2.5-pro:generateContent?key={env['GEMINI_API_KEY']}"
)
body = json.dumps({
    "contents": [{"parts": [{"text": brief}]}],
    "generationConfig": {"temperature": 0.2, "maxOutputTokens": 16384},
}).encode()

t0 = time.time()
req = urllib.request.Request(
    url, data=body, headers={"Content-Type": "application/json"}, method="POST"
)
try:
    with urllib.request.urlopen(req, timeout=900) as resp:
        data = json.loads(resp.read())
except Exception as e:
    print(f"[gemini tb-16 r3] API error: {e}", file=sys.stderr)
    sys.exit(1)
elapsed = time.time() - t0
print(f"[gemini tb-16 r3] API returned in {elapsed:.1f}s", file=sys.stderr)

# Extract text
try:
    text = data["candidates"][0]["content"]["parts"][0]["text"]
except (KeyError, IndexError) as e:
    print(f"[gemini tb-16 r3] response shape error: {e}\nfull response: {json.dumps(data, indent=2)[:2000]}", file=sys.stderr)
    sys.exit(1)

header = (
    f"# Gemini TB-16 R3 Ship Audit — Controlled Market Smoke Arena (Class 3 dual external audit)\n"
    f"**Round**: {ROUND}\n"
    f"**Date**: 2026-05-04\n"
    f"**Test baseline**: cargo test --workspace = 907 PASS / 0 FAILED / 150 ignored (TB-16 R3 commit 90848bb)\n"
    f"**Halt-trigger battery**: 13/13 GREEN (tests/tb_16_halt_triggers.rs)\n"
    f"**audit_pipeline_smoke verdict.json**: PROCEED 38/0/0/3 (3 skipped = Layer H tamper stubs at FC1-N35)\n"
    f"**audit_pipeline_smoke replay**: byte-identical to verdict.json\n"
    f"**Markov chain**: TB-16 capsule 737b4d22... chains to TB-15 head f9e701b4...\n"
    f"**Trust Root**: GREEN\n"
    f"**Audit envelope**: Class 3 integration smoke (architect §7.7 — external audit MANDATORY at ship)\n"
    f"**OBS-deferred**: audit_tape_tamper Round 2 hang (pre-existing on git HEAD; tamper logic untouched in R3; R1 carry-forward 3/3 detected)\n"
    f"**Elapsed**: {elapsed:.1f}s\n"
    f"**Prompt size**: {len(brief):,} chars\n"
    f"**Audit mode**: architectural strategic R3 convergence (Codex covers impl-paranoid R3 in parallel)\n\n---\n\n"
)
OUT.write_text(header + text)
print(f"[gemini tb-16 r3] saved: {OUT}")
