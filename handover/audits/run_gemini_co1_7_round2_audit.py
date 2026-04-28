#!/usr/bin/env python3
"""Gemini round-2 audit on CO1.7 transition_ledger v1.1 (post round-1 CHALLENGE).

Strategic / constitutional review angle: did v1.1 close the round-1 must-fix
items in a constitutionally-aligned way? Independent of Codex round-2.
"""
import json
import pathlib
import subprocess
import sys
import urllib.request

REPO = pathlib.Path("/home/zephryj/projects/turingosv4")
ENV_FILE = pathlib.Path("/home/zephryj/projects/turingosv3/.env")
OUT = REPO / "handover/audits/GEMINI_CO1_7_ROUND2_AUDIT_2026-04-28.md"

key = None
for line in ENV_FILE.read_text().splitlines():
    if line.startswith("GEMINI_API_KEY="):
        key = line.split("=", 1)[1].strip()
        break
if not key:
    sys.exit("ERROR: no GEMINI_API_KEY")

PROMPT = """You are Gemini DeepThink, the strategic architectural reviewer for TuringOS v4.

This is a **round-2 closure-verification audit** on CO1.7 transition_ledger v1.1. Round-1 returned CHALLENGE/CHALLENGE; v1.1 claims to close 11 must-fix items + 1 disagreement (D1: epoch binding). Codex is running an independent round-2 in parallel (implementer-review angle); your angle is **strategic / architectural / constitutional**.

Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS. Conservative wins on disagreement.

## Your previous (round-1) verdict

You returned **CHALLENGE** with high conviction. Your top 3 must-fix:
1. **Reconstructibility & Trust Ambiguity (Q3)**: rename replay → "chain-integrity check"; explicit "trust-the-sequencer" mode
2. **Canonical Signing Primitive Integration (Q4/DIV-1)**: choose Path A (extend CanonicalMessage) + forward-compat serialization
3. **Indirection & Availability Risk (Q2)**: spec must acknowledge L4 → L3 CAS dependency + mitigation

You also raised: forward-compat extensions field (Q9), v4/v4.1 boundary (Q7), and disagreed with Codex on Q10 (epoch binding — you said redundant; Codex said required).

## What v1.1 claims to close (per patch log)

| ID | v1 issue | v1.1 fix claim |
|---|---|---|
| C1 | replay single-mode; called I-DETHASH witness but did chain-only | Two-mode `ReplayMode` enum; I-DETHASH bound to FullTransition only |
| C2 | CAS cold-replay broken | New CO1.4-extra atom; v1 ledger doc dependency + ReplayError::CasMissing |
| C3 | signing primitive integration | Path A: extend CanonicalMessage + LedgerEntrySigningPayload + sign_ledger_entry API |
| K1 | sequencer logical_t skip race | Dual counter (next_submit_id, next_logical_t) |
| K2 | parent_ledger_root NOT bound | Field added + bound in signing payload |
| K3 | L4/L5 head_t ownership | CO1.7 owns ledger_root + head_t; CO1.8 owns state_root |
| K4 | trait mismatch | Spec aligned to skeleton (commit(&mut self) -> Hash; iter_from deferred) |
| K5 | Slash dispatch gap | Slash DROPPED for v4 (CO P2.5 atom) |
| K6 | #[repr(u8)] missing | Added with explicit discriminants |
| K7 | conformance test gap (8 vs 6) | 8 tests with stage marker |
| G1 | rigid struct, no forward-compat | extensions: BTreeMap<String, Vec<u8>> bound in signing payload |
| D1 | epoch binding (you vs Codex disagree) | Conservative: bound in signing payload (Codex security wins) |

## Round-2 strategic questions

For each round-1 finding, judge **CLOSED / PARTIAL / REGRESSED / NEW-ISSUE**:

**Q1 (was Q1: Constitutional alignment)**: did v1.1 changes preserve Anti-Oreo three-layer + Art 0.1/0.2/0.4 alignment? Specifically:
- Art 0.2 Tape Canonical: does the dual-counter design preserve "no rejection sidecar"? Where do rejections go?
- Art 0.4 Q_t version-controlled: head_t = NodeId(commit_sha) per K3 — is this stronger or weaker alignment than v1's NodeId::from_state_root?
- Anti-Oreo: ledger module in bottom_white::ledger; sign API extends bottom_white::ledger::system_keypair (also bottom_white). Boundary preserved?

**Q2 (was Q2: WP § 5.L4)**: v1.1 changed envelope from 8 → 11 fields (added parent_ledger_root, extensions, separated stored vs signed). Does the new envelope still conform to WP § 5.L4 axioms? Does the additional indirection (LedgerEntrySigningPayload separate from LedgerEntry) violate any "single canonical record" principle?

**Q3 (your top-1; C1 closure)**: replay two-mode is now explicit. Verify:
- Spec § 4 names ChainOnly + FullTransition correctly
- Spec § 6 binds I-DETHASH to FullTransition only (skeleton ChainOnly is necessary-but-not-sufficient)
- Skeleton's `replay_chain_integrity` is honestly named (not "replay" stripped)
- Documentation explicitly says "trust the sequencer" semantics for ChainOnly?

CLOSED, PARTIAL, REGRESSED, or NEW-ISSUE?

**Q4 (your top-2; C3 closure)**: Path A + LedgerEntrySigningPayload + CanonicalMessage extension. Verify:
- Forward-compat serialization clause: future variants add new CanonicalMessage::* (NOT in-place edits)
- CanonicalMessage extension is additive Wave 4-B (verify: enum has 3 existing variants + 1 NEW; canonical_digest match arm added; sign_ledger_entry method added to Ed25519Keypair impl)
- Forward-compat for LedgerEntrySigningPayload itself: when v4.x feature populates extensions BTreeMap, is the canonical digest still computable on old verifiers? (binary compat across upgrades)

CLOSED, PARTIAL, REGRESSED, or NEW-ISSUE?

**Q5 (your top-3; C2 closure)**: CAS cold-replay risk. Verify:
- Spec § 0 + § 5 acknowledge dependency on CO1.4-extra atom
- ReplayError::CasMissing variant added for FullTransition
- Is the deferral to a NEW atom (CO1.4-extra) acceptable v1.1, or should v1.1 ship CAS persistence inline?

CLOSED, PARTIAL, REGRESSED, or NEW-ISSUE?

**Q6 (was Q4: epoch binding D1 disagreement)**: conservative resolution chose Codex (bind in signing payload) over you (orthogonal). Acknowledge:
- Codex's concrete cross-epoch transplant attack defense is the load-bearing argument
- v1.1 binds epoch in LedgerEntrySigningPayload (NOT separately in ledger_root)
- Are you persuaded? Or do you maintain "redundant" stance? (your call; if you re-challenge here, conservative re-resolves Codex.)

**Q7 (was Q9: forward-compat extensions)**: extensions: BTreeMap<String, Vec<u8>> added; bound in signing payload via length-prefix iteration. Verify:
- Empty-map case binds to zero-byte length prefix (deterministic)
- v4.x can populate extensions WITHOUT changing signing structure (backwards compat)
- BUT: v4.x extensions populated → old verifier sees new entries with non-empty extensions → signing digest differs. Is this safe upgrade, or does it require a coordinated rollout?

CLOSED, PARTIAL, REGRESSED, or NEW-ISSUE?

**Q8 (was Q5: INV8 interaction)**: did v1.1 leave INV8 (DAG determinism) status unchanged? Specifically: does the new dual-counter design pre-commit to a particular INV8 resolution, or stay neutral?

**Q9 (was Q6: sequencer cell count)**: § 5.2.2 mandates disjoint runtime_repo per cell. Phase C 5x10x2 = 100 cells. Resource cost concern? v1.1 changed nothing on this axis (still 1 sequencer per cell) — accept or push back?

**Q10 (was Q7: v4/v4.1 boundary)**: v1.1 dropped TxKind::Slash for v4 + deferred runtime ArchitectAI/JudgeAI to v4.1. v1.1 also added MetaTx not-in-CO1.7 explicit (in spec § 0). Boundary clean enough for v4 ship?

**Q11 (NEW; your strategic eyes only)**: v1.1's new patch log table at top of spec is unconventional; most TuringOS specs sediment changelog as a separate appendix. Is this in-place patch log a reasonable v1.1 form, or should v1.2 move it to an appendix?

**Q12 (NEW)**: spec § 13 estimated scope expanded ~1.5-2.5 weeks (slight expansion due to CO1.4-extra). Is the expansion acceptable as a Wave 6 #1 budget addition, or does CO1.4-extra need to be a separate Wave entry?

## Final holistic verdict

**Q-final**: PASS / CHALLENGE / VETO

End with:
- For each round-1 finding: CLOSED / PARTIAL / REGRESSED / NEW-ISSUE
- Top must-fix (if CHALLENGE) — be specific; cite spec § + line
- Top architectural risk (if VETO) — round-1 issued no VETO so this would be a new escalation
- Conviction (low/med/high)

## Output format

# Gemini CO1.7 Round-2 Audit
## Q1-Q12 closure judgments
## Q-final **VERDICT**: PASS / CHALLENGE / VETO
## Top must-fix
## Conviction

Be rigorous. Per CLAUDE.md, do NOT pass on principle; do NOT veto on principle. v1.1 either closes the round-1 blockers cleanly or it doesn't.
"""

DOCS = [
    ("DOC: CO1.7 spec v1.1 (round-2 target)", "handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md"),
    ("DOC: skeleton v1.1 (round-2 target)", "src/bottom_white/ledger/transition_ledger.rs"),
    ("XREF: your round-1 audit", "handover/audits/GEMINI_CO1_7_ROUND1_AUDIT_2026-04-28.md"),
    ("XREF: Codex round-1 audit (parallel; convergent flagged 3 same items)", "handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md"),
    ("XREF: merged round-1 verdict + v1.1 patch list", "handover/audits/CO1_7_DUAL_AUDIT_VERDICT_R1_2026-04-28.md"),
    ("XREF: STATE_TRANSITION_SPEC v1.4", "handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md"),
    ("XREF: SYSTEM_KEYPAIR_SECURITY_v1", "handover/specs/SYSTEM_KEYPAIR_SECURITY_v1_2026-04-27.md"),
    ("XREF: WP v2.2 (Anti-Oreo)", "handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md"),
    ("XREF: Constitution.md", "constitution.md"),
    ("XREF: shipped system_keypair.rs", "src/bottom_white/ledger/system_keypair.rs"),
    ("XREF: shipped CAS schema + store", "src/bottom_white/cas/schema.rs"),
]

parts = [PROMPT]
for label, rel in DOCS:
    p = REPO / rel
    parts.append(f"\n\n=== {label} ===\n\n")
    parts.append(p.read_text())
text = "".join(parts)

with OUT.open("w") as f:
    f.write(f"# Gemini CO1.7 transition_ledger Round-2 Audit Run\n")
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
