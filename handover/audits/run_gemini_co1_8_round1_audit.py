#!/usr/bin/env python3
"""Gemini round-1 audit on CO1.8 v1 (L5 Materializer + Agent Read View; greenfield).

Strategic / architectural angle: WP § 5.L5 fidelity, L5 vs L6 boundary, Anti-Oreo
three-layer respect, constitutional alignment with Art 0.4 (state_root semantics),
Inv 10 Goodhart shield consequences of agent_view stubbing. Independent of Codex
round-1 (parallel).
"""
import json
import pathlib
import subprocess
import sys
import urllib.request
import urllib.error

REPO = pathlib.Path("/home/zephryj/projects/turingosv4")
ENV_FILE = pathlib.Path("/home/zephryj/projects/turingosv3/.env")
OUT = REPO / "handover/audits/GEMINI_CO1_8_ROUND1_AUDIT_2026-04-29.md"

key = None
for line in ENV_FILE.read_text().splitlines():
    if line.startswith("GEMINI_API_KEY="):
        key = line.split("=", 1)[1].strip()
        break
if not key:
    sys.exit("ERROR: no GEMINI_API_KEY")

PROMPT = """You are Gemini DeepThink, the strategic architectural reviewer for TuringOS v4.

This is a **round-1 dual external audit** on CO1.8 v1 — a GREENFIELD spec for the L5 Materialized State + Agent Read View atom. Codex is running an independent round-1 in parallel (implementer-paranoid angle); your angle is **strategic / architectural / constitutional**.

Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.

## Background

CO1.7-extra (final L4 atom; STEP_B closed at `4a978f0`) just shipped 2026-04-29. CO1.8 is the determinate next atom per LATEST.md "Wave 6 #2" framing + SPRINT_DEPENDENCY_GRAPH line 109 (Materialized State, 8 atoms). This is a greenfield draft from primary sources (no prior rounds; no prior must-fix items).

## What you're reviewing

1. **Spec doc**: `handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md` (~300 lines)
2. **Primary source**: WP § 5.L5 (lines 392-413) — enumerates 6 sub-modules + `read_tool` minimal-context
3. **Constitutional reference**: `q_state.rs:27` "generic 32-byte hash (sha256)" + `q.state_root_t: Hash` field + `Hash::ZERO` genesis sentinel
4. **L4 invocation surface**: STATE_TRANSITION_SPEC v1.4 lines 399/466/560/624/700/758/852 (7 `materializer::apply` invocation sites in transition body stages 4-7)
5. **Frozen ABI**: TypedTx 7-variant (CO1.1.4-pre1)

## Round-1 strategic questions (7)

**Q1. Constitutional alignment of L5 = sha256-of-snapshot semantics**: spec § 5 Q1 is the most consequential open question — does `state_root_t: Hash` literally equal a git-tree object id (Path B), or is it sha256 of a serialized state snapshot whose backend storage is git tree?
- The author's lean (sha256-of-snapshot, with git tree as storage backend) is grounded in q_state.rs:27 explicit "generic 32-byte hash (sha256)" wording.
- STATE_TRANSITION_SPEC line 78 "Materialized state Merkle root (git tree root in Path B)" can be read EITHER as "state_root EQUALS git tree object id" OR as "state_root is computed-from / stored-in git tree". Which reading is constitutionally normative?
- Path B was ratified per AUTO_RESEARCH_NOTEPAD line 66 ("real git substrate"). Does Path B mandate git-tree-id-as-state-root (which would force sha1, not sha256), or just git-as-storage-substrate (sha256 over git-stored snapshots)?
- This question affects every CO1.8 sub-atom's storage layer + every CO1.7.5 transition body's commit shape. Constitutional alignment must precede impl.

**Q2. Anti-Oreo three-layer separation**: CO1.8 sits at FC3 bottom-white per the spec (alignment to TRACE_MATRIX § 5.L5). Verify:
- Does CO1.8 honor the Anti-Oreo separation? Specifically: does `materializer::apply(prior_root, tx)` need to KNOW about predicate semantics (FC1 top-white domain) to compute new state? Or does it dispatch on TxKind only and treat predicate results as opaque payloads?
- Does CO1.8.7 `agent_view::project_for_agent` need to consume FC2 middle-black agent identity in a way that violates the "tools are below agents" hierarchy?
- WP § 5.L5 line 408 `read_tool(agent_i, task_j, Q_t)` — is "read_tool" a bottom-white tool (FC3) OR a top-white predicate (FC1)? The naming suggests bottom-white but the semantics (visibility-filtered projection) suggests top-white predicate evaluation.

**Q3. L5 vs L6 boundary** (WP § 5.L5 line 427 "Art 0.2 重建性合规" caveat):
- WP says "L6 严格 derivable from L4 + L5"; "delete L6 不破坏 Art 0.2".
- Spec § 0.4 #3 defers `reputation_counters` (windowed delta), `price_signals` (market microstructure compression), `failure_histogram` to CO1.9 L6.
- But spec § 0.3 puts `agent_reputation_index` at L5 (CO1.8.5) and `price_signal_index` at L5 (CO1.8.6). Where exactly is the L5/L6 line for these?
- Author's reading: L5 holds *absolute* state (current reputation count for agent X = 5); L6 holds *derived* state (delta-reputation in window = +2 over last 10 tx). Is this the right cut?
- If yes: CO1.8 ships absolute counters + index-by-key; CO1.9 ships delta-windows + statistical summaries. Architecturally clean?

**Q4. Inv 10 Goodhart shield through agent_view stubbing**:
- Spec § 0.4 #4 ships `project_for_agent` as no-op filter (returns full view) until CO1.5 ships visibility tags.
- Inv 10 (Goodhart shield) was the primary motivator for L5 vs L4 separation in WP § 5.L5: agents read L5 (visibility-filtered) NOT L4 (raw ledger).
- A no-op filter means agents (in v1) effectively read raw L5 = full materialized state, including potentially-evaluator-internal fields.
- Is this acceptable for v1? Is there a downstream HARD GATE (e.g., PPUT-CCL Phase D requires Goodhart shield to be active)? If yes, the v1 stub creates a regression window.
- Alternative: ship a deny-by-default stub (returns hardcoded minimal field set) until CO1.5 ships. Trade-off: requires hardcoding what "minimal" means without PredicateRegistry guidance.

**Q5. WP § 5.L5 fidelity — module count + naming**:
- WP § 5.L5 enumerates 6 modules (lines 397-402): `current_state_db`, `task_index`, `agent_reputation_index`, `error_taxonomy_index`, `price_signal_index`, `permission_view` + `read_tool` (line 408).
- Spec § 0.3 maps these to: CO1.8.3 state_db / CO1.8.4 task_index / CO1.8.5 agent_reputation_index / CO1.8.6 (error_taxonomy + price_signal) / CO1.8.7 agent_view (= permission_view + read_tool merged) — plus CO1.8.1 mod skeleton + CO1.8.2 apply() = 8 sub-atoms.
- Author folds `permission_view` and `read_tool` into a single CO1.8.7 `agent_view`. Is this correct? Whitepaper treats them as distinct (permission_view is one of 6 modules; read_tool is the access function).
- Author folds `error_taxonomy_index` and `price_signal_index` into a single CO1.8.6. Is this acceptable, or should they be separate sub-atoms for audit-readability?

**Q6. Sub-atom independence & shippability**:
- Spec § 3 says "Sub-atoms are NOT independently shippable in v1 — they form a single compile unit (mod.rs re-exports cascade)". Sprint graph (line 152) puts "[CO1.8.*] Materializer 8 atoms (each is an independent index)" in PARALLEL GROUP C.
- These statements are partially in tension. What's the canonical reading? If 8 sub-atoms are "each an independent index", can they ship as 8 separate STEP_B-non-restricted commits (with mod.rs growing by one re-export per commit)? Or do they require a single bundled commit?
- Architectural implication: per CLAUDE.md "Audit Standard" (dual audit per phase decision), would each sub-atom need its own dual audit? Or does PASS/PASS on the v1 spec authorize all 8 as a single phase?

**Q7. Forward sustainability**:
- After CO1.8 ships, CO1.9 (L6 Signal Indices) is the immediate next atom. Spec hints at the L5/L6 cut (Q3 above) but does not freeze the L5↔L6 interface.
- Should CO1.8 v1 ship a `pub trait L5Surface { ... }` interface that CO1.9 will consume, or is that premature?
- If the L5↔L6 interface is left implicit, what prevents future CO1.9 from forcing breaking changes to CO1.8 once L6 demands additional L5 hooks?
- Per memory `project_thesis`: "Frozen 5-step compile loop". Does CO1.8 advance the loop, or is it pure substrate? If substrate, what's the latest CO1.8 can ship without delaying the loop's measurability gate?

## Verdict format

Section A: Verdict (PASS/CHALLENGE/VETO) with conviction (LOW/MED/HIGH).
Section B: P0 blockers (must-fix before round-2; cite spec § + line).
Section C: Open architectural questions raised.
Section D: Suggested patches (specific spec line/section edits).
Section E: Forward-sustainability notes.

Be concrete. Cite spec § + line where possible.

---

# Spec attachment + reference materials follow.
"""

# Build context
attachments = []

def append_file(label, path, fence="rust"):
    full = REPO / path
    if not full.exists():
        return
    attachments.append(f"\n\n---\n\n## XREF: {label} — `{path}`\n\n```{fence}\n{full.read_text()}\n```\n")

append_file("CO1.8 v1 spec (PRIMARY)", "handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md", fence="markdown")
append_file("Q_t types (Hash + state_root_t)", "src/state/q_state.rs", fence="rust")
append_file("TypedTx 7-variant ABI", "src/state/typed_tx.rs", fence="rust")
append_file("CAS store (snapshot storage surface)", "src/bottom_white/cas/store.rs", fence="rust")

# WP § 5.L5 excerpt
wp_path = REPO / "handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md"
wp_lines = wp_path.read_text().splitlines()
wp_excerpt = "\n".join(wp_lines[379:445])  # lines 380-445 covering L5 + L6 boundary
attachments.append(f"\n\n---\n\n## XREF: WP § 5.L5 (lines 380-445; primary source)\n\n```\n{wp_excerpt}\n```\n")

# STATE spec materializer::apply invocation sites
state_path = REPO / "handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md"
state_text = state_path.read_text()
result = subprocess.run(
    ["grep", "-n", "-B3", "-A3", "materializer::apply", str(state_path)],
    capture_output=True, text=True,
)
attachments.append(f"\n\n---\n\n## XREF: STATE_TRANSITION_SPEC v1.4 — 7 materializer::apply invocation sites\n\n```\n{result.stdout}\n```\n")

full_prompt = PROMPT + "".join(attachments)
print(f"[gemini co1.8 r1] prompt size: {len(full_prompt)} chars", file=sys.stderr)

# POST to Gemini
url = f"https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-thinking-exp-01-21:generateContent?key={key}"
body = json.dumps({
    "contents": [{"parts": [{"text": full_prompt}]}],
    "generationConfig": {
        "temperature": 0.3,
        "maxOutputTokens": 65536,
    },
}).encode()

req = urllib.request.Request(url, data=body, headers={"Content-Type": "application/json"}, method="POST")

import time
t0 = time.time()
try:
    with urllib.request.urlopen(req, timeout=300) as resp:
        data = json.loads(resp.read().decode())
except urllib.error.HTTPError as e:
    print(f"HTTP {e.code}: {e.read().decode()[:500]}", file=sys.stderr)
    sys.exit(1)
elapsed = time.time() - t0
print(f"[gemini co1.8 r1] API returned in {elapsed:.1f}s", file=sys.stderr)

# Extract text
candidates = data.get("candidates", [])
if not candidates:
    sys.exit(f"No candidates: {json.dumps(data)[:500]}")
parts = candidates[0].get("content", {}).get("parts", [])
text = "".join(p.get("text", "") for p in parts)

OUT.parent.mkdir(parents=True, exist_ok=True)
header = f"""# Gemini CO1.8 Round-1 Audit
**Date**: 2026-04-29
**Target**: spec v1 (greenfield)
**HEAD**: {subprocess.run(['git', '-C', str(REPO), 'rev-parse', 'HEAD'], capture_output=True, text=True).stdout.strip()}
**Prompt size**: {len(full_prompt)} chars
**API latency**: {elapsed:.1f}s

---

"""
OUT.write_text(header + text)
print(f"[gemini co1.8 r1] saved: {OUT}", file=sys.stderr)
