#!/usr/bin/env python3
"""Gemini round-2 audit on CO1.7-extra v1 (post round-1 scope split).

Strategic / architectural angle: did the Occam-driven scope split correctly
preserve constitutional alignment + WP § 5.L4 boundaries? Are M3-M5 fixes
architecturally coherent? Independent of Codex round-2 (parallel).
"""
import json
import pathlib
import subprocess
import sys
import urllib.request
import urllib.error

REPO = pathlib.Path("/home/zephryj/projects/turingosv4")
ENV_FILE = pathlib.Path("/home/zephryj/projects/turingosv3/.env")
OUT = REPO / "handover/audits/GEMINI_CO1_7_EXTRA_ROUND2_AUDIT_2026-04-29.md"

key = None
for line in ENV_FILE.read_text().splitlines():
    if line.startswith("GEMINI_API_KEY="):
        key = line.split("=", 1)[1].strip()
        break
if not key:
    sys.exit("ERROR: no GEMINI_API_KEY")

PROMPT = """You are Gemini DeepThink, the strategic architectural reviewer for TuringOS v4.

This is a **round-2 dual external audit** on CO1.7-extra v1 — a SCOPE-SPLIT atom carved out of the round-1-CHALLENGED bundled CO1.7.5 v1. Codex is running an independent round-2 in parallel (implementer-paranoid angle); your angle is **strategic / architectural / constitutional**.

Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.

## What changed since round-1

ArchitectAI executed an Occam-driven scope split (B2: split by dependency profile) per user's "无损压缩即智能" principle. The previous bundled atom is now TWO atoms:
- **CO1.7-extra (this audit target)**: D2 head_t close + D3 Sequencer entry-point wiring + 1 substrate-independent test. **No FC1/FC2 substrate dependency**. Ships now post-PASS/PASS.
- **CO1.7.5 (future)**: D1 transition bodies + 3 D4 tests + un-ignore replay test. Gated on CO P2.x family substrate atoms (per PROJECT_DECISION_MAP § 3.4).

Round-1 must-fix items disposition (per `CO1_7_5_DUAL_AUDIT_VERDICT_R1_2026-04-29.md`):
- M1 (substrate gap from Codex Q-D/H/I) → D1 deferred to future CO1.7.5; CO1.7-extra has zero substrate dep
- M2 (D1 purity violations) → moved with D1
- M3 (compile defects: TuringBus / Kernel derives / Sequencer placement) → fixed in v1
- M4 (Gemini MF1+MF3: § 0.3 process passive) → § 0.4 active reconciliation commitment + STATE v1.5 issue filing committed; downstream-supersession authority principle asserted explicitly
- M5 (Gemini Q8 vs Codex Q-B Q1 disagreement) → synthesis: default None + mandatory override + defensive test; Gemini's silent-stagnation concern + Codex's no-panic-after-commit concern both addressed

## Round-2 strategic questions

**Q1. Constitutional alignment of the SCOPE SPLIT itself**: the split places D2/D3 (substrate-independent wiring) in CO1.7-extra and D1 (substrate-dependent transition bodies) in future CO1.7.5. Verify:
- Does this split respect Anti-Oreo three-layer separation? CO1.7-extra touches `src/bottom_white/ledger/` (FC3 bottom-white), `src/state/sequencer.rs` (driver), `src/bus.rs` + `src/kernel.rs` (runtime topology). Is this layering consistent?
- Does the split respect the Wave 6 #1 closure goal? PROJECT_DECISION_MAP previously called CO1.7.5 the "final L4 atom"; now CO1.7.5 is gated and CO1.7-extra is interposed. Is this re-decomposition institutionally clean, or does it muddy the Wave-6 atomic boundary?
- Does the merged verdict's "Wave 6 #1 actual progress 30-40%, not 80%" diagnosis hold up? If yes, this is a meta-finding worth surfacing in LATEST.md.

**Q2. § 0.4 process commitment quality** (round-1 MF1+MF3 closure): spec § 0.4 commits to:
1. Filing a STATE_TRANSITION_SPEC v1.5 housekeeping issue as part of CO1.7-extra atom closure
2. Asserting the principle "later, more specific, audited spec legitimately supersedes earlier general specs within the layered boundary"

Verify:
- Is principle (2) within ArchitectAI's authority to assert, or does it require constitution-level amendment? The whitepaper § 17 / Art 0 establishes specific authority chains; downstream supersession is implicit but not explicit in the constitution.
- Is commitment (1) operationally complete? "Filing an issue" is a process action; should the spec instead include the literal v1.5 patch text as an appendix to make the commitment concrete?
- Does the carry-forward statement that "the two STATE supersessions migrate intact to future CO1.7.5" correctly handle the fact that the head_t supersession actually takes effect in CO1.7-extra D2 (not in transition bodies)? § 0.4 says the supersessions migrate to CO1.7.5; § 1.1 D2 has `q_w.head_t = NodeId(commit_oid_hex)` which IS the head_t supersession in action HERE.

**Q3. Q1 synthesis architectural soundness** (M5 closure): spec § 1.2 introduces a "default None + mandatory override + defensive test" design pattern for `head_commit_oid_hex`. Strategic question:
- This is a non-standard pattern: Rust traits can't enforce mandatory override at compile time (without sealed-trait tricks); the mandate is convention + defensive test. Is this a robust safety strategy for a **constitutional anchor** field (head_t), or does it leave a window where a refactor could re-introduce silent stagnation?
- Alternative architectural patterns: (a) trait without default (Rust does enforce override); (b) sealed trait (no third-party LedgerWriter); (c) marker trait `pub trait LedgerWriterWithCommitOid: LedgerWriter { fn head_commit_oid_hex(&self) -> Option<String>; }` with explicit dispatch. Which is most architecturally clean for a constitutional anchor?

**Q4. Combined STEP_B argument rebase** (M3c partial): spec § 2.3 rebased the combined-ceremony justification from "Phase 0 minimum sufficient version is binding" to "functional coupling" (each half is a compile-or-no-op-error without the other). Verify:
- Is functional-coupling a stronger criterion than the original Phase 0 invocation? In what cases would functional-coupling justify combining ceremonies that should remain separate?
- Does the spec's stronger argument hold for OTHER combined STEP_B candidates that may arise in future? Or is this a one-off justification for CO1.7-extra specifically?

**Q5. Sequencer placement in Kernel** (M3c full): spec § 2.2 justifies placing Sequencer in Kernel via three arguments. Strategic critique:
- Argument 1 (parallel to Tape/NodeId pattern): is Tape/NodeId genuinely "topology" or is it state-management? If state-management, the parallel weakens the case for "Kernel as pure topology".
- Argument 2 (state lives in Q_t inside Sequencer): true now, but does this create a forward-compat hazard if a future atom needs Kernel-level state-data?
- Argument 3 (doc-comment patch): is patching Kernel's self-description to fit the new placement an architectural compromise that should be flagged (and possibly rejected in favor of a runtime-layer alternative)?

**Q6. Test coverage adequacy**: CO1.7-extra ships 2 tests (`cas_payload_round_trip` + `git2_writer_returns_some_after_commit`). Strategic question:
- Is the **D2 actual code path** (q.head_t = NodeId(commit_oid_hex) after writer.commit) actually exercised by any test? cas_payload_round_trip tests CAS only; git2_writer_returns_some_after_commit tests the trait method but NOT the apply_one stage 9 patch.
- Should v1.1 add a head_t-advancement integration test that calls Sequencer::apply_one (with a NotYetImplemented-but-commit-succeeding mock writer) and asserts q.head_t advances? Or is that a CO1.7.5 concern (since apply_one needs a real transition body to commit)?
- D3 STEP_B compile-coherence: any test for the TuringBus + Kernel + Sequencer wiring graph compiling cleanly?

**Q7. Forward sustainability re-examination** (per round-1 Q3/Q5 PASS): the scope split changes the forward-roadmap shape. Verify:
- Does CO1.7-extra preserve the affordances for CO1.8 (L5 materializer) + CO1.9 (L6 signal indices) that round-1 confirmed?
- Future CO1.7.5 (transition bodies) is now gated on substrate atoms; does this change the Wave 6 #2/#3 sequencing? If CO P2.x ships before CO1.7.5, what's the right order?
- LATEST.md correction implication: "Wave 6 #1 progress 30-40% not 80%" is a project-level meta-finding. Should the spec assert this or leave it to the handover-update?

**Q8. Final holistic verdict**: PASS / CHALLENGE / VETO

End with:
- Top 3 must-fix (if CHALLENGE) — bias toward NOT requesting more rounds; the spec is small and focused
- Top 3 architectural risks (if VETO) — only if foundational design flaw
- Conviction (low/med/high)

## Output format

# Gemini CO1.7-extra Round-2 Audit
## Q1 Constitutional alignment of the scope split
## Q2 § 0.4 process commitment quality
## Q3 Q1 synthesis architectural soundness
## Q4 Combined STEP_B argument rebase
## Q5 Sequencer placement in Kernel
## Q6 Test coverage adequacy
## Q7 Forward sustainability re-examination
## Q8 **VERDICT**: PASS / CHALLENGE / VETO
## Top 3 must-fix / risks
## Conviction

Be rigorous. Cite spec § + line where possible. Per CLAUDE.md, do NOT pass on principle; do NOT veto on principle.
"""

DOCS = [
    ("DOC: CO1.7-extra v1 (target of audit)", "handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md"),
    ("XREF: round-1 merged verdict (the document driving this scope split)", "handover/audits/CO1_7_5_DUAL_AUDIT_VERDICT_R1_2026-04-29.md"),
    ("XREF: CO1.7 v1.2 spec (frozen, round-3 PASS/PASS)", "handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md"),
    ("XREF: CO1.1.4-pre1 spec (frozen, PASS/PASS)", "handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md"),
    ("XREF: STATE_TRANSITION_SPEC v1.4 (frozen, round-4 PASS/PASS) — for K3 v1.2 supersession context only", "handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md"),
    ("XREF: WP v2.2 (Anti-Oreo restoration; ratified 2026-04-27)", "handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md"),
    ("XREF: Constitution.md", "constitution.md"),
    ("XREF: shipped src/state/sequencer.rs (D2 target)", "src/state/sequencer.rs"),
    ("XREF: shipped src/bottom_white/ledger/transition_ledger.rs (D2 trait target)", "src/bottom_white/ledger/transition_ledger.rs"),
    ("XREF: shipped src/bus.rs (D3 STEP_B target — TuringBus)", "src/bus.rs"),
    ("XREF: shipped src/kernel.rs (D3 STEP_B target — Kernel)", "src/kernel.rs"),
    ("XREF: shipped src/state/q_state.rs (NodeId tuple struct + Q_t)", "src/state/q_state.rs"),
    ("XREF: shipped src/bottom_white/cas/store.rs (D4 test surfaces)", "src/bottom_white/cas/store.rs"),
    ("XREF: STEP_B_PROTOCOL (binding ceremony spec)", "handover/ai-direct/STEP_B_PROTOCOL.md"),
]

parts = [PROMPT]
for label, rel in DOCS:
    p = REPO / rel
    parts.append(f"\n\n=== {label} ===\n\n")
    parts.append(p.read_text())
text = "".join(parts)

with OUT.open("w") as f:
    f.write(f"# Gemini CO1.7-extra Round-2 Audit Run\n")
    f.write(f"- Model: gemini-2.5-pro\n")
    f.write(f"- Packet chars: {len(text)}\n")
    f.write(f"- Started: {subprocess.check_output(['date', '-Iseconds']).decode().strip()}\n")
    f.write(f"- HEAD: {subprocess.check_output(['git', '-C', str(REPO), 'rev-parse', 'HEAD']).decode().strip()}\n")
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
