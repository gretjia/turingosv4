#!/usr/bin/env python3
"""Gemini round-3 audit on CO1.7-extra v1.1 (post round-2 patches).

Strategic / architectural angle: are the 10 r2 patches architecturally
coherent? Does v1.1 introduce any new architectural drift? Independent
of Codex round-3 (parallel).
"""
import json
import pathlib
import subprocess
import sys
import urllib.request
import urllib.error

REPO = pathlib.Path("/home/zephryj/projects/turingosv4")
ENV_FILE = pathlib.Path("/home/zephryj/projects/turingosv3/.env")
OUT = REPO / "handover/audits/GEMINI_CO1_7_EXTRA_ROUND3_AUDIT_2026-04-29.md"

key = None
for line in ENV_FILE.read_text().splitlines():
    if line.startswith("GEMINI_API_KEY="):
        key = line.split("=", 1)[1].strip()
        break
if not key:
    sys.exit("ERROR: no GEMINI_API_KEY")

PROMPT = """You are Gemini DeepThink, the strategic architectural reviewer for TuringOS v4.

This is a **round-3 dual external audit** on CO1.7-extra v1.1 — applied 10 r2 patches per `CO1_7_EXTRA_DUAL_AUDIT_VERDICT_R2_2026-04-29.md`. Codex is running an independent round-3 in parallel (implementer-paranoid angle); your angle is **strategic / architectural / constitutional**.

Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.

## What changed since round-2

ArchitectAI applied all 10 r2 must-fix items. v1.1 commits: `25564d7` + `a3952cf`. Smoke verified at HEAD `a3952cf` with 11/11 PASS.

Architecturally significant changes (vs v1):
- **MF4**: Sequencer placement migrated from Kernel to TuringBus. Kernel UNTOUCHED. STEP_B becomes single-file ceremony on bus.rs. Cleaner layering — Kernel preserves "pure topology" doctrine; runtime drivers (Sequencer + future) live at TuringBus level.
- **MF3**: trait method `head_commit_oid_hex` becomes REQUIRED (no default impl). Compiler-enforced. Both audits' safety arguments satisfied via this third-option synthesis (Gemini r1 voted `unimplemented!()`; Codex r1 voted `default { None }+convention`; r2 both rejected default-None as fragile; v1.1 adopts no-default).
- **MF2**: D2 logic extracted to `advance_head_t(q, writer)` helper → directly testable via mock writer (without injecting dispatch_transition).

Smaller patches: § 0.4 disposition table (MF1), test harness flat-naming (MF5), manual Sequencer Debug (MF6), inline LedgerEntry in tests (MF7), stale-comment landing checklist (MF8), atomicity wording (MF9), LoC estimate (MF10).

**Open Q count after v1.1**: 0.

## What's at stake

CO1.7-extra is the bridge atom for Wave 6 #1 incremental closure. Round-3 PASS unblocks implementation. Round-3 CHALLENGE triggers v1.2 patch round.

## Round-3 strategic questions

**Q1. Architectural coherence of the 10 patches collectively**: do MF1-MF10 form a coherent set, or do any patches conflict/overlap in subtle ways?
- MF4 (TuringBus placement) + MF1 (§ 0.4 disposition) + MF2 (helper extraction): are these three patches mutually consistent? E.g., does the helper extraction match the TuringBus placement, and does the disposition table match the helper's behavior?
- Specifically: § 0.4 says head_t supersession is "enacted in CO1.7-extra D2". § 1.1 D2 calls `advance_head_t` from apply_one stage 9. apply_one is in `src/state/sequencer.rs`, which is owned by `Sequencer`. `Sequencer` is owned by `TuringBus` (per MF4). Trace: TuringBus → Sequencer.apply_one → advance_head_t → q.head_t = NodeId(commit_oid_hex). Is this trace constitutionally sound?

**Q2. Sequencer placement at TuringBus (MF4) architectural soundness**: spec § 2.1 + § 2.2 rewrites place Sequencer in TuringBus directly. Verify:
- Does this respect Anti-Oreo three-layer separation? TuringBus is at the runtime-orchestrator layer; Sequencer is a state-mutation-driver. Are these the right layers for these responsibilities?
- Forward-compat: does the TuringBus placement leave room for a future "Runtime" layer above TuringBus (per Gemini r2 Q5 hypothetical), or does it foreclose that option?
- Compare: original v1 had `Kernel.sequencer`, v1.1 has `TuringBus.sequencer`. Is the v1.1 placement strictly better, or just different?

**Q3. Required trait method (MF3) constitutional soundness**: removing the default impl makes `head_commit_oid_hex` compiler-required. Verify:
- For a constitutional anchor field (`head_t` per Art 0.4), is compiler-enforcement the appropriate guarantee level?
- Does requiring third-party LedgerWriter impls to declare violate any "open extension" principle (per Codex r2 Q3 noting that sealed-trait would be over-restrictive)?
- The "third option" (no default) was explicitly rejected by both audits in r1 (Gemini=unimplemented; Codex=default-None). Both r2 audits reversed and converged on no-default. Is this convergence stable, or could a future audit re-litigate?

**Q4. Helper extraction (MF2) testability vs API surface**: `advance_head_t` is now a `pub(crate)` helper in `src/state/sequencer.rs`. Verify:
- Does adding this helper expand the public API surface in undesirable ways?
- Is `pub(crate)` the right visibility (vs `pub`, vs private with re-export, vs `#[cfg(test)]`)?
- The new test § 3.3 directly exercises `advance_head_t` — does this test belong at the integration test level (`tests/`) or at the unit test level (`#[cfg(test)] mod tests`)?

**Q5. § 0.4 disposition table correctness (MF1)**: spec § 0.4 now has explicit table — head_t enacted HERE (D2); SignalKind migrates. Verify:
- Is the framing "later, more specific, audited spec legitimately supersedes earlier general spec" still a coherent principle assertion in v1.1?
- Does "filing STATE_TRANSITION_SPEC v1.5 housekeeping issue" remain a robust commitment? Or should v1.1 include the literal patch text as appendix per Gemini r2 Q2 alternative?

**Q6. Smoke 11/11 PASS reliability**: the spec footer reports smoke 11/11 PASS at HEAD `a3952cf`. Verify:
- Does the smoke methodology actually validate the v1.1 architectural claims?
- Are any v1.1 claims NOT covered by smoke (e.g., claims about post-implementation behavior)?

**Q7. Forward sustainability (re-examination post-MF4)**: the architectural shift to TuringBus-owned Sequencer affects the project roadmap. Verify:
- Does CO1.8 (L5 materializer) integrate cleanly with TuringBus.sequencer? Or does it require a Runtime-layer abstraction?
- Does CO1.9 (L6 signal indices) integrate cleanly?
- Future CO1.7.5 (transition bodies) is gated on CO P2.x substrate — does the TuringBus placement affect substrate-atom design?

**Q8. Final holistic verdict**: PASS / CHALLENGE / VETO

Round-3 expects PASS unless v1.1 missed a closure or introduced regression. End with:
- Top issues (if CHALLENGE)
- Top architectural risks (if VETO; only foundational flaws warrant)
- Conviction (low/med/high)

## Output format

# Gemini CO1.7-extra Round-3 Audit
## Q1 Architectural coherence of 10 patches
## Q2 Sequencer placement at TuringBus
## Q3 Required trait method constitutional soundness
## Q4 Helper extraction testability vs API surface
## Q5 § 0.4 disposition table correctness
## Q6 Smoke 11/11 PASS reliability
## Q7 Forward sustainability (post-MF4)
## Q8 **VERDICT**: PASS / CHALLENGE / VETO
## Top issues / risks
## Conviction

Be rigorous. Cite spec § + line where possible. Per CLAUDE.md, do NOT pass on principle; do NOT veto on principle.
"""

DOCS = [
    ("DOC: CO1.7-extra v1.1 (target of audit)", "handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md"),
    ("XREF: round-2 merged verdict (drove the v1.1 patches)", "handover/audits/CO1_7_EXTRA_DUAL_AUDIT_VERDICT_R2_2026-04-29.md"),
    ("XREF: round-1 merged verdict (drove the v1 scope split)", "handover/audits/CO1_7_5_DUAL_AUDIT_VERDICT_R1_2026-04-29.md"),
    ("XREF: CO1.7 v1.2 spec (frozen, round-3 PASS/PASS)", "handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md"),
    ("XREF: WP v2.2 (Anti-Oreo restoration)", "handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md"),
    ("XREF: Constitution.md", "constitution.md"),
    ("XREF: shipped src/state/sequencer.rs", "src/state/sequencer.rs"),
    ("XREF: shipped src/bottom_white/ledger/transition_ledger.rs", "src/bottom_white/ledger/transition_ledger.rs"),
    ("XREF: shipped src/bus.rs (TuringBus — D3 target)", "src/bus.rs"),
    ("XREF: shipped src/kernel.rs (UNTOUCHED post-MF4)", "src/kernel.rs"),
    ("XREF: shipped src/state/q_state.rs (NodeId + Q_t)", "src/state/q_state.rs"),
    ("XREF: shipped src/bottom_white/ledger/system_keypair.rs (Ed25519Keypair derives — for MF6)", "src/bottom_white/ledger/system_keypair.rs"),
    ("XREF: STEP_B_PROTOCOL", "handover/ai-direct/STEP_B_PROTOCOL.md"),
]

parts = [PROMPT]
for label, rel in DOCS:
    p = REPO / rel
    parts.append(f"\n\n=== {label} ===\n\n")
    parts.append(p.read_text())
text = "".join(parts)

with OUT.open("w") as f:
    f.write(f"# Gemini CO1.7-extra Round-3 Audit Run\n")
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
