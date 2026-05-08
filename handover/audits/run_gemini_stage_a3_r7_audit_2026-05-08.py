#!/usr/bin/env python3
"""Gemini Stage A3 (HEAD_t C2 multi-ref ChainTape) ship-gate audit (R7).

Per STAGE_A3_HEAD_T_C2_charter_2026-05-07.md SG-A3.10 + `feedback_audit_after_evidence`:
G2 dual audit dispatched AFTER MVP gates green per CR-C0.8. Stage A3 substrate
is fully landed end-to-end with both gate-level (constitution_head_t_c2_multi_ref.rs
7 tests) AND real-LLM-load witness (A3 R5 + A3 R3.5 + B3 R6 mini-M1) at HEAD
post-381554f. This is the Gemini half of the dual audit; Codex side dispatches
in parallel via run_codex_stage_a3_r7_audit_2026-05-08.sh.

Conservative merge ranking applies per `feedback_dual_audit_conflict`:
VETO > CHALLENGE > PASS.
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
OUT = ROOT / f"handover/audits/GEMINI_STAGE_A3_R7_AUDIT_2026-05-08_{ROUND}.md"

if ROUND not in {"R1", "R2"}:
    print("[gemini A3 R7] error: TB_AUDIT_ROUND must be R1 or R2", file=sys.stderr)
    sys.exit(2)
if OUT.exists():
    print(f"[gemini A3 R7] error: {OUT} already exists; refusing to overwrite", file=sys.stderr)
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
    ]
    for p in patterns:
        m = re.search(p, text)
        if m:
            return m.group(1).upper()
    for tok in ("VETO", "CHALLENGE", "PASS"):
        if re.search(rf"\b{tok}\b", text):
            return tok
    return "UNKNOWN"


env = load_env()
if "GEMINI_API_KEY" not in env:
    print("[gemini A3 R7] GEMINI_API_KEY not found in .env files", file=sys.stderr)
    sys.exit(2)

brief = f"""# Gemini Stage A3 / HEAD_t C2 Multi-Ref ChainTape Audit — {ROUND}

You are Gemini 2.5 Pro acting as a skeptical architectural reviewer for
TuringOS v4 Stage A3 (HEAD_t C2 multi-ref ChainTape). This is the Gemini
half of the Stage A3 R7 G2 dual-audit per STAGE_A3_HEAD_T_C2_charter_2026-05-07.md
§5 R7. Codex implementation-paranoid half dispatches in parallel.

Conservative merge ranking applies per `feedback_dual_audit_conflict`:
VETO > CHALLENGE > PASS. Prefer CHALLENGE over PASS when a concrete
architectural uncertainty has an actionable remediation. Use VETO only for
ship-blocking inconsistency with the Constitution, charter, or architect-pinned
schemas.

Round cap is 2. This run is {ROUND}.

## Audit target

Stage A3 substrate at HEAD post-`381554f`:

Commit lineage:
- `72e2494` Stage A3 R1+R2+R4 — multi-ref support on transition_ledger.rs (CHAINTAPE_L4_REF / CHAINTAPE_L4E_REF / CHAINTAPE_CAS_REF + dual-write + 5 helper fns) + HeadTWitness::reconstruct_from_chaintape_refs + 6 SG-A3 tests in constitution_head_t_c2_multi_ref.rs
- `4b0062e` Stage A3 R3 — CAS root ref hook in cas/store.rs (CasStore::put auto-advances refs/chaintape/cas) + 1 integration test
- `2d3d948` Stage A3 R5 smoke — mathd_algebra_107 n1 deepseek-chat 150s; SG-A3.1+3+5 GREEN under real-LLM load; refs/chaintape/l4 dual-write 859f5021... + cas 7e8c0d3f...
- `f7a6660` Stage A3 R3.5 — wire rejection_evidence to refs/chaintape/l4e via TURINGOS_CHAINTAPE_PATH env var; smoke on mathd_algebra_113 produced **10/10 1:1 ref-to-JSONL match** for SG-A3.2
- `381554f` Stage B3 R6 mini-M1 — 8-problem batch on Stage A3 C2 substrate; 8/8 problems show l4e_jsonl_match=true (83 commits ↔ 83 jsonl lines aggregate)

Charter ship gates per STAGE_A3_HEAD_T_C2_charter_2026-05-07.md §4:
```
SG-A3-HEAD-T-C2.1  L4 head ref advances on accepted transition
SG-A3-HEAD-T-C2.2  L4.E head ref advances on rejected evidence
SG-A3-HEAD-T-C2.3  CAS root ref advances when CAS evidence added
SG-A3-HEAD-T-C2.4  Replay reconstructs HEAD_t (six-field byte equality)
SG-A3-HEAD-T-C2.5  No hidden filesystem pointer
SG-A3-HEAD-T-C2.6  cargo test --workspace GREEN (>=1181)
SG-A3-HEAD-T-C2.7  bash scripts/run_constitution_gates.sh GREEN (>=97)
SG-A3-HEAD-T-C2.8  One real-LLM smoke run produces 50/50-style invariant report
SG-A3-HEAD-T-C2.9  OBS forward-binding for any C1->C2 migration edge cases
SG-A3-HEAD-T-C2.10 Codex + Gemini dual audit dispatched AFTER MVP gates green
```

Validation at HEAD `381554f`:
- Constitution gates: 154 GREEN / 0 failed / 1 ignored (architect baseline 97 → cleared by 57)
- Workspace tests: 1287 PASS / 0 failed / 151 ignored (baseline 1181 → cleared by 106)
- Trust Root rehashed: src/runtime/mod.rs + src/bottom_white/ledger/transition_ledger.rs + src/bottom_white/cas/store.rs + src/bottom_white/ledger/rejection_evidence.rs

## Required questions

Answer Q1 through Q8 explicitly. Mark each as PASS / CHALLENGE / VETO.

Q1. Stage A3 R1 — does dual-writing refs/chaintape/l4 alongside refs/transitions/main correctly preserve the C1 baseline as backward-compat alias per CR-A3-HEAD-T-C2.6, AND does the new ref system honor CR-A3-HEAD-T-C2.4 "no change to canonical signing payload"? Verify by inspecting Git2LedgerWriter::commit ledger writer logic.

Q2. Stage A3 R3.5 — is the env-var-driven `TURINGOS_CHAINTAPE_PATH` hook (in `flush_jsonl_record`) the right architectural seam, or should it be an explicit field on `RejectionEvidenceWriter`? Argue for the chosen seam OR identify a concrete failure mode where env-var driving is brittle.

Q3. SG-A3.2 (L4.E head ref advances) — the deterministic git2 commit synthesis uses `submit_id` as time and the canonical JSONL record bytes as the tree blob. Is this byte-stable across binary versions? In particular, can the canonical_decode + JsonlRecord::from + serde_json::to_string round-trip produce non-determinism if any field has a Map/Set with unstable iteration order?

Q4. SG-A3.4 — `HeadTWitness::reconstruct_from_chaintape_refs` derives `cas_root` by sha256-hashing the git2 OID of refs/chaintape/cas. Is this the right cross-format mapping, or does it lose information? Should `cas_root` instead be the OID itself (truncated/padded)?

Q5. The B3 R6 mini-M1 smoke (8 problems) showed 0/8 solved with hit_max_tx=true. Is this a Stage A3 ship gate concern, or independent (n1 single-agent + 200 tx budget weakness)? Charter §4 only requires "real-LLM smoke produces 50/50-style invariant report"; the chain_invariant.json was NOT explicitly captured per problem in the mini-M1 evidence — does this matter for SG-A3.8 closure?

Q6. CR-A3-HEAD-T-C2.5 ("no new global filesystem pointer") — verified by tests/constitution_head_t_c2_multi_ref.rs::sg_a3_no_hidden_filesystem_pointer (grep src/ + matrix). Is the grep coverage sufficient, or could a forbidden pointer be hidden in a non-grepped surface (e.g., `target/`, build artifacts, OS-level /tmp)?

Q7. Per `feedback_real_problems_not_designed`, real-public-problem witnesses are mandatory. Stage A3 evidence binds to mathd_algebra_107 + mathd_algebra_113 + 6 other real MiniF2F problems via B3 R6 mini-M1. Is 8 problems sufficient real-problem coverage for a Class-4-STEP_B-on-transition_ledger.rs change, or should the audit demand B3 R6 full 50p × n=3 × 3-seed = 450 runs before declaring Stage A3 substrate ship-eligible?

Q8. Strategic risk — what, if anything, in the Stage A3 substrate is visibly wrong or missing that next-stage TB-18B M1/M2 full benchmark or Stage C Polymarket would expose? In particular consider: ref atomicity under concurrent writers, garbage collection of unreferenced commits in refs/chaintape/*, or replay-determinism when the runtime_repo is partially packed.

## Verdict format

Use exactly this shape:

```
Q1: PASS|CHALLENGE|VETO - <reason>
Q2: PASS|CHALLENGE|VETO - <reason>
Q3: PASS|CHALLENGE|VETO - <reason>
Q4: PASS|CHALLENGE|VETO - <reason>
Q5: PASS|CHALLENGE|VETO - <reason>
Q6: PASS|CHALLENGE|VETO - <reason>
Q7: PASS|CHALLENGE|VETO - <reason>
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
"""

for rel, lang in [
    ("handover/tracer_bullets/STAGE_A3_HEAD_T_C2_charter_2026-05-07.md", "markdown"),
    ("src/bottom_white/ledger/transition_ledger.rs", "rust"),
    ("src/state/head_t_witness.rs", "rust"),
    ("src/bottom_white/ledger/rejection_evidence.rs", "rust"),
    ("src/bottom_white/cas/store.rs", "rust"),
    ("tests/constitution_head_t_c2_multi_ref.rs", "rust"),
    ("handover/evidence/stage_a3_r5_smoke_2026-05-08T05-40-39Z/SG_A3_R5_SMOKE_SUMMARY.json", "json"),
    ("handover/evidence/stage_a3_r35_smoke_2026-05-08T06-02-28Z/SG_A3_R35_SMOKE_SUMMARY.json", "json"),
    ("handover/evidence/stage_b3_r6_minim1_2026-05-08T06-07-32Z/B3_R6_MINIM1_AGGREGATE.json", "json"),
    ("handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md", "markdown"),
    ("CLAUDE.md", "markdown"),
]:
    brief += append_file(rel, lang)

print(f"[gemini A3 R7] prompt size: {len(brief):,} chars", file=sys.stderr)

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
    print(f"[gemini A3 R7] error: {exc}", file=sys.stderr)
    sys.exit(1)

elapsed = time.time() - t0
print(f"[gemini A3 R7] API returned in {elapsed:.1f}s", file=sys.stderr)

try:
    text = data["candidates"][0]["content"]["parts"][0]["text"]
except (KeyError, IndexError) as exc:
    print(f"[gemini A3 R7] malformed API response: {exc}", file=sys.stderr)
    print(json.dumps(data, indent=2)[:4000], file=sys.stderr)
    sys.exit(1)

verdict = extract_verdict(text)
header = f"""# Gemini Stage A3 / HEAD_t C2 Multi-Ref ChainTape Audit — {ROUND}

**Round**: {ROUND}
**Date**: 2026-05-08
**Model**: gemini-2.5-pro
**Elapsed**: {elapsed:.1f}s
**Prompt size**: {len(brief):,} chars
**Final aggregate verdict**: {verdict}

---

## Verbatim Gemini Response

"""

OUT.write_text(header + text)
print(f"[gemini A3 R7] saved: {OUT}")
print(f"[gemini A3 R7] verdict: {verdict}")
