#!/usr/bin/env python3
"""Gemini round-1 audit on CO1.7.5 spec v1 (transition bodies + runtime wiring).

Design-review angle: does the architecture compose with the constitution
(Anti-Oreo + WP § 5.L4 + Art 0.2/0.4) and shipped contracts? Independent
of Codex round-1 (running in parallel).
"""
import json
import pathlib
import subprocess
import sys
import urllib.request
import urllib.error

REPO = pathlib.Path("/home/zephryj/projects/turingosv4")
ENV_FILE = pathlib.Path("/home/zephryj/projects/turingosv3/.env")
OUT = REPO / "handover/audits/GEMINI_CO1_7_5_ROUND1_AUDIT_2026-04-29.md"

key = None
for line in ENV_FILE.read_text().splitlines():
    if line.startswith("GEMINI_API_KEY="):
        key = line.split("=", 1)[1].strip()
        break
if not key:
    sys.exit("ERROR: no GEMINI_API_KEY")

PROMPT = """You are Gemini DeepThink, the strategic architectural reviewer for TuringOS v4.

This is a **round-1 dual external audit** on CO1.7.5 spec v1 DRAFT (transition bodies + runtime wiring). Codex is running an independent round-1 in parallel (implementer-review angle); your angle is **strategic / architectural / constitutional**.

Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS. Conservative wins on disagreement.

## Why a spec-only audit (no skeleton this round)

CO1.7.5 inherits frozen ABI + Sequencer machinery from CO1.7-impl (commit `2461fe6`) + CO1.4-extra (commit `b6b7574`); the only NEW skeleton would be 7 transition body fns + 1 trait method default. Per CLAUDE.md "Audit Standard" pre-implementation gate, no code lands until PASS/PASS. Spec + smoke + carry-forward inheritance is the audit surface.

## What's at stake

CO1.7.5 is the FINAL atom of Wave 6 #1 (L4 transition ledger family, 80% → 100%). Wave 6 #2/#3 + ChainTape Layer 5/6 unblock only after PASS/PASS. Pre-implementation gate per CLAUDE.md "Audit Standard": PASS/PASS required before code beyond inheritance.

## Round-1 strategic questions

**Q1. Constitutional alignment** — does CO1.7.5 design uphold:
- Art 0.1 四要素映射 (Tape / Input / Q / State)
- Art 0.2 Tape Canonical 公理 (no rejection sidecar; LedgerEntry.system_signature attests sequencer-stamped semantics; rejected tx do NOT advance ledger — verify this holds for all 7 transition bodies)
- Art 0.4 Q_t version-controlled (head_t = git commit SHA; ledger_root_t = Merkle root). The G-1 close in D2 sets `q.head_t = state::q_state::NodeId(commit_oid_hex)` post-Git2LedgerWriter.commit. Is the commit_oid_hex literal write the *correct* Art 0.4 binding, or does Art 0.4 prefer state_root-derived? (Spec § 0.3.1 declares CO1.7-K3-v1.2 supersession; verify this.)
- Anti-Oreo 三层 (transition bodies live in middle-black state mutation; ledger sits in bottom-white)

**Q2. Authority chain on STATE supersessions (§ 0.3)** — spec § 0.3 carries forward TWO STATE v1.4 § 3 supersessions:
- 0.3.1: head_t mutation deferred to Sequencer post-commit (CO1.7-K3-v1.2 supersedes STATE v1.4 § 3 line 412 `q_next.head_t = NodeId::from_state_root(...)`)
- 0.3.2: shipped 4-variant SignalKind suffices (CO1.1.4-pre1 supersedes STATE v1.4 § 3 BoolSignal/StatSignal richness)

Strategic question: can a **downstream** spec's PASS/PASS resolution legitimately supersede an **upstream** (STATE v1.4 round-4 PASS/PASS) line, OR does this constitute institutional drift requiring explicit Phase Z' check / STATE re-audit before CO1.7.5 ships? Spec § 0.3 explicitly leaves this decision to the STATE curator — is that correct delegation, or is CO1.7.5 ducking responsibility? Conservative-wins-on-disagreement: when downstream and upstream disagree, which dominates?

**Q3. Wave 6 #1 closure correctness** — CO1.7.5 declares it closes Wave 6 #1 (L4 transition ledger family) to 100%. Verify against PROJECT_DECISION_MAP / Plan v3.2:
- D1 (transition bodies) + D2 (head_t close) + D3 (runtime wiring) + D4 (un-ignore + 3 NEW tests) — does this set actually exhaust L4? Or is something silently deferred to "Wave 6 #2" that should be in CO1.7.5?
- Wave 6 #2 (CO1.8 L5 materializer) + #3 (CO1.9 L6 signal indices) — does CO1.7.5 reserve enough affordances for them, or hard-code v1 minimums in ways that block CO1.8/CO1.9 forward extension?

**Q4. Combined STEP_B ceremony** — spec § 1 D3 Q3 closure: ONE A/B unit covers `src/kernel.rs` (Sequencer field) + `src/bus.rs` (forwarder method) together. Justification: Bus forwarder meaningless without Kernel field (STEP_B Phase 0 minimum-sufficient). Strategic argument:
- Combining the ceremony reduces ceremonial overhead but increases blast radius if A/B diverges. Tradeoff defensible?
- Per `STEP_B_PROTOCOL.md` Phase 0, is "minimum sufficient" a binding criterion or advisory? If binding, is the spec's invocation correct? If advisory, is the spec abusing it?
- Sequencer ownership: spec puts Sequencer field in Kernel (not Bus, not a runtime layer above Bus). Kernel currently holds Tape/NodeId from legacy ledger (kernel.rs:8). Does adding Sequencer to Kernel violate any abstraction layer (state lives in Q_t, not Kernel)?

**Q5. SignalKind minimization forward-compat hazard** — § 0.3.2 carries forward 4-variant SignalKind (Empty/Finalize/TaskExpired/TerminalSummary). Per emit table, 4 of 7 transition bodies emit Empty. STATE v1.4 § 3 / § 3.1 / § 3.2 pseudocode emits Bool/Stat richness (AcceptedAt, VerifiedAt, ChallengeUpheld, PriceUpdate, ReputationDelta). CO1.9 will extend.
- Does emitting Empty for 4 transition bodies create observable-state loss that breaks Art 0.2 reconstructibility? (Reconstructibility from L4 alone OK if downstream L5/L6 can recompute reputation/price; verify.)
- Does adding SignalKind variants in CO1.9 require LedgerEntry schema migration (breaking CO1.1.4-pre1 ABI lock)?
- Is the v1 minimization a "safe deferral" or a "deferred hazard"?

**Q6. Hygiene OBS quality** — spec smoke phase surfaced CLAUDE.md + STEP_B_PROTOCOL.md path drift (`src/wallet.rs` does not exist; wallet relocated to `src/sdk/tools/wallet.rs`). Hygiene fix landed in commit `b2036aa`; OBS at `handover/alignment/OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md`. Strategic question:
- Is fixing CLAUDE.md inline (vs deferring to OBS-only) appropriate here? CLAUDE.md is project instructions, NOT constitution.md. Does the Alignment Standard (constitution.md hygiene → OBS not in-place) extend to CLAUDE.md, or is in-place edit fine for project instructions?
- Is the OBS sufficiently honest about what was NOT investigated (e.g., bisecting which commit moved wallet.rs)?

**Q7. Forward sustainability for Wave 6 #2/#3** — CO1.8 (L5 materializer; replaces `q_next.economic_state_t.derive_state_root()` with merkleized state root) + CO1.9 (L6 signal stream indices) come after CO1.7.5. Does the spec:
- Reserve trait/struct affordances for CO1.8's `materializer::apply` substitution?
- Reserve SignalKind extension points for CO1.9 without breaking CO1.1.4-pre1 ABI?
- Avoid hard-coding v1 minimums in transition bodies that would block CO1.8/CO1.9 plumbing?

**Q8. Q1 (open) — `head_commit_oid_hex` default impl** — only Q remaining open after self-audit. Spec proposes `default { None }` for back-compat; alternative is `unimplemented!()` forcing every impl to declare. Strategic vote + rationale:
- Default-None: hidden buggy impl risk (forgot to override = silent head_t stagnation)
- Default-unimplemented: compile-time forcing function (every impl declares; no silent failure mode)
- Memory `feedback_v3_preserve` analog: which is the conservative default for a constitutional-anchor field?

**Q9. Final holistic verdict**: PASS / CHALLENGE / VETO

End with:
- Top 3 must-fix (if CHALLENGE)
- Top 3 architectural risks (if VETO)
- Conviction (low/med/high)

## Output format

# Gemini CO1.7.5 Round-1 Audit
## Q1 Constitutional alignment
## Q2 Authority chain on STATE supersessions
## Q3 Wave 6 #1 closure correctness
## Q4 Combined STEP_B ceremony
## Q5 SignalKind minimization forward-compat
## Q6 Hygiene OBS quality
## Q7 Forward sustainability for Wave 6 #2/#3
## Q8 Q1 head_commit_oid_hex default impl recommendation
## Q9 **VERDICT**: PASS / CHALLENGE / VETO
## Top 3 must-fix / risks
## Conviction

Be rigorous. Cite spec § + line where possible. Per CLAUDE.md, do NOT pass on principle; do NOT veto on principle.
"""

DOCS = [
    ("DOC: CO1.7.5 spec v1 DRAFT (target of audit)", "handover/specs/CO1_7_5_TRANSITION_BODIES_AND_RUNTIME_WIRING_v1_2026-04-29.md"),
    ("XREF: CO1.7 spec v1.2 (frozen, round-3 PASS/PASS) — K3 v1.2 head_t supersession authority", "handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md"),
    ("XREF: CO1.1.4-pre1 spec (frozen, PASS/PASS) — ABI lock + SignalKind minimization authority", "handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md"),
    ("XREF: STATE_TRANSITION_SPEC v1.4 (frozen, round-4 PASS/PASS) — pseudocode authority + supersession source", "handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md"),
    ("XREF: WP v2.2 (Anti-Oreo restoration; ratified 2026-04-27)", "handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md"),
    ("XREF: Constitution.md", "constitution.md"),
    ("XREF: shipped src/state/sequencer.rs (target of D1+D2)", "src/state/sequencer.rs"),
    ("XREF: shipped src/state/typed_tx.rs (frozen ABI: TypedTx + 22 TransitionError + 4 SignalKind)", "src/state/typed_tx.rs"),
    ("XREF: shipped src/bottom_white/ledger/transition_ledger.rs (frozen Sequencer machinery; head_commit_oid)", "src/bottom_white/ledger/transition_ledger.rs"),
    ("XREF: shipped src/state/q_state.rs (Q_t + NodeId tuple struct)", "src/state/q_state.rs"),
    ("XREF: shipped src/bus.rs (D3 STEP_B touch surface)", "src/bus.rs"),
    ("XREF: shipped src/kernel.rs (D3 STEP_B touch surface)", "src/kernel.rs"),
    ("XREF: hygiene OBS (path drift sediment)", "handover/alignment/OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md"),
    ("XREF: STEP_B_PROTOCOL (binding ceremony spec)", "handover/ai-direct/STEP_B_PROTOCOL.md"),
]

parts = [PROMPT]
for label, rel in DOCS:
    p = REPO / rel
    parts.append(f"\n\n=== {label} ===\n\n")
    parts.append(p.read_text())
text = "".join(parts)

with OUT.open("w") as f:
    f.write(f"# Gemini CO1.7.5 transition_bodies_and_runtime_wiring Round-1 Audit Run\n")
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
