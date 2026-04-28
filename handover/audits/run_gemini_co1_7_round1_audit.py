#!/usr/bin/env python3
"""Gemini round-1 audit on CO1.7 transition_ledger spec v1 + type-skeleton.

Design-review angle: does the architecture compose with the constitution
(Anti-Oreo + WP § 5.L4 + Art 0.2/0.4) and shipped contracts? Independent
of Codex round-1 (running in parallel).
"""
import json
import pathlib
import subprocess
import sys
import urllib.request

REPO = pathlib.Path("/home/zephryj/projects/turingosv4")
ENV_FILE = pathlib.Path("/home/zephryj/projects/turingosv3/.env")
OUT = REPO / "handover/audits/GEMINI_CO1_7_ROUND1_AUDIT_2026-04-28.md"

key = None
for line in ENV_FILE.read_text().splitlines():
    if line.startswith("GEMINI_API_KEY="):
        key = line.split("=", 1)[1].strip()
        break
if not key:
    sys.exit("ERROR: no GEMINI_API_KEY")

PROMPT = """You are Gemini DeepThink, the strategic architectural reviewer for TuringOS v4.

This is a **round-1 dual external audit** on CO1.7 transition_ledger spec v1 + type-skeleton joint artifact. Codex is running an independent round-1 in parallel (implementer-review angle); your angle is **strategic / architectural / constitutional**.

Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS. Conservative wins on disagreement.

## Why a joint artifact, not paper-only

Per memory `feedback_smoke_before_batch`: plan audit ≠ runtime check. Skeleton smoke already caught 1 spec bug (Q9) + 5 DIV cataloged (DIV-1..5) before this audit. Paper-review-only would have missed those. Round-1 inspects both paper (spec) + code (370-line skeleton, 196/0 lib PASS, TR-tracked).

## What's at stake

CO1.7 is Wave 6 #1 priority per LATEST line 79: single-point bottleneck blocking ChainTape L4-L6, Git runtime_repo wire-up, and 6 economic transition functions. Pre-implementation gate per CLAUDE.md "Audit Standard": PASS/PASS required before code beyond skeleton.

## Round-1 strategic questions

**Q1. Constitutional alignment** — does CO1.7 design uphold:
- Art 0.1 四要素映射 (Tape / Input / Q / State)
- Art 0.2 Tape Canonical 公理 (no rejection sidecar; LedgerEntry.system_signature attests sequencer-stamped semantics; rejected tx do NOT advance ledger)
- Art 0.4 Q_t version-controlled (head_t = git commit SHA; ledger_root_t = Merkle root)
- Anti-Oreo 三层结构 (top-white predicates / middle-black agents / bottom-white tools — ledger sits in bottom_white, correct?)

**Q2. WP § 5.L4 conformance** — WP defines L4 as 12-field WorkTx (tx_id / parent_state_root / agent_id / task_id / read_set / write_set / proposal_cid / predicate_results / stake / signature / timestamp / status). CO1.7 LedgerEntry is an 8-field ENVELOPE (logical_t / parent_state_root / tx_kind / tx_payload_cid / resulting_state_root / resulting_ledger_root / timestamp_logical / epoch / system_signature) wrapping a CAS-stored payload. Does the envelope-vs-inline distinction:
- preserve WP § 5.L4 axioms?
- create a 2-step indirection vulnerability (CAS unavailable → ledger entries unreadable)?
- correctly map "agent_signature" vs "system_signature" — the agent's WorkTx.signature is inside payload; LedgerEntry.system_signature is sequencer-stamped. Right structure?

**Q3. Reconstructibility (Art 0.2)** — Art 0.2 demands tape canonical reconstructibility. Skeleton replay v1 does parent_state_root + ledger_root chain check ONLY, deferring re-running pure transitions to CO1.7.5+. Is this:
- a defensible v1 deliverable (chain integrity is necessary-but-not-sufficient)?
- a partial Art 0.2 implementation that should NOT be called "I-DETHASH witness" until full?
- creating a "trust mode" ambiguity (chain-only-trust vs full-replay-trust)?

**Q4. CanonicalMessage extension (Q8 / DIV-1)** — existing system_keypair.rs has `CanonicalMessage` enum with 3 fixed variants. LedgerEntry is not among them. Two paths:
  (a) extend enum with `LedgerEntry(LedgerEntry)` variant — touches Wave 4-B shipped code (additive)
  (b) introduce sibling sign primitive specifically for LedgerEntry
- Constitutional argument for/against each?
- Is "single canonical sign primitive" a property worth preserving (analogous to "single tape" axiom)?
- If (a): does it create a forward-compat hazard for future LedgerEntry schema changes?

**Q5. INV8 + economic invariant interaction** — STATE_TRANSITION_SPEC § 4 has 27 invariants, but per LATEST INV8 (DAG determinism) is currently VETOed pending v2 revision. Does CO1.7 design accidentally pre-commit to a particular INV8 resolution? Or is it INV8-neutral?

**Q6. Sequencer single-writer assumption** — § 5.2.1 mandates one sequencer per (runtime_repo, run_id). Cross-cell isolation per § 5.2.2 mandates disjoint runtime_repo. But Phase C runs 5 modes × 10 problems × 2 seeds = 100 cells. Is it operationally tractable to spawn 100 sequencers + 100 runtime_repos? What's the resource cost in disk/inode? Is this an inadvertent O(N²) explosion?

**Q7. v4 / v4.1 boundary** — D-VETO-4=B ratified "defer MetaTape runtime to v4.1; v4 ships Phase 3 prep". Spec § 0 says "MetaTx full schema deferred to v4.1; v4 emits MetaProposalDraft to L3 CAS". Skeleton TxKind enum has variants Work/Verify/Challenge/Reuse/FinalizeReward/TaskExpire/TerminalSummary/Slash but NO Meta variant. Correct boundary? Will adding `TxKind::Meta` in v4.1 break LedgerEntry binary compat?

**Q8. Open Q1-Q11 strategic recommendations**:
- Q1 SubmissionQueue type (tokio mpsc / crossbeam / std mpsc) — strategic dep weight argument?
- Q3 Sequencer-vs-(LedgerWriter+OrderingCoordinator) split — abstraction boundary argument?
- Q5 enum-match dispatch vs MetaTransitionInterface trait — v4/v4.1 cleanliness argument?
- Q7 genesis ledger_root_t (Hash::ZERO vs sha256 of genesis_payload.toml) — constitutional anchor argument?
- Q10 epoch field (added by skeleton) — should also bind to ledger_root_t somehow?

**Q9. Forward sustainability** — if CO P3 phase opens public AGI market (per WP § 17), does CO1.7 ledger schema need extension (e.g., on-chain settlement proofs, ZK predicates)? What forward-compat affordances should v1 reserve?

**Q10. Final holistic verdict**: PASS / CHALLENGE / VETO

End with:
- Top 3 must-fix (if CHALLENGE)
- Top 3 architectural risks (if VETO)
- Conviction (low/med/high)

## Output format

# Gemini CO1.7 Round-1 Audit
## Q1 Constitutional alignment
## Q2 WP § 5.L4 conformance
## Q3 Reconstructibility (Art 0.2)
## Q4 CanonicalMessage extension
## Q5 INV8 interaction
## Q6 Sequencer cell explosion
## Q7 v4/v4.1 boundary
## Q8 Open Q recommendations
## Q9 Forward sustainability
## Q10 **VERDICT**: PASS / CHALLENGE / VETO
## Top 3 must-fix / risks
## Conviction

Be rigorous. Cite spec § + line where possible. Per CLAUDE.md, do NOT pass on principle; do NOT veto on principle.
"""

DOCS = [
    ("DOC: CO1.7 spec v1 (target of audit)", "handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md"),
    ("DOC: type-skeleton (joint artifact target)", "src/bottom_white/ledger/transition_ledger.rs"),
    ("XREF: STATE_TRANSITION_SPEC v1.4 (frozen, round-4 PASS/PASS)", "handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md"),
    ("XREF: SYSTEM_KEYPAIR_SECURITY_v1 (frozen, CO1.7.0a-f)", "handover/specs/SYSTEM_KEYPAIR_SECURITY_v1_2026-04-27.md"),
    ("XREF: META_TRANSITION_INTERFACE_v1", "handover/specs/META_TRANSITION_INTERFACE_v1_2026-04-27.md"),
    ("XREF: WP v2.2 (Anti-Oreo restoration; ratified 2026-04-27)", "handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md"),
    ("XREF: Constitution.md", "constitution.md"),
    ("XREF: shipped system_keypair.rs (for DIV-1 ground truth)", "src/bottom_white/ledger/system_keypair.rs"),
    ("XREF: shipped CO1.4 CAS schema", "src/bottom_white/cas/schema.rs"),
    ("XREF: shipped CO1.4 CAS store", "src/bottom_white/cas/store.rs"),
    ("XREF: shipped Q_t struct (CO1.2)", "src/state/q_state.rs"),
]

parts = [PROMPT]
for label, rel in DOCS:
    p = REPO / rel
    parts.append(f"\n\n=== {label} ===\n\n")
    parts.append(p.read_text())
text = "".join(parts)

# Open output file for streaming write
with OUT.open("w") as f:
    f.write(f"# Gemini CO1.7 transition_ledger Round-1 Audit Run\n")
    f.write(f"- Model: gemini-2.5-pro\n")
    f.write(f"- Packet chars: {len(text)}\n")
    f.write(f"- Started: {subprocess.check_output(['date', '-Iseconds']).decode().strip()}\n")
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
