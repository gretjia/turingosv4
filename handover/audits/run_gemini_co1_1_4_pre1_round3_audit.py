#!/usr/bin/env python3
"""Gemini round-3 audit on CO1.1.4-pre1 v1.2 — narrow closure check.

Gemini PASSed v1.1; round-3 verifies v1.2 didn't introduce strategic regressions
while closing Codex's round-2 patch-mechanical CHALLENGEs.
"""
import json
import pathlib
import subprocess
import sys
import urllib.request

REPO = pathlib.Path("/home/zephryj/projects/turingosv4")
ENV_FILE = pathlib.Path("/home/zephryj/projects/turingosv3/.env")
OUT = REPO / "handover/audits/GEMINI_CO1_1_4_PRE1_ROUND3_AUDIT_2026-04-28.md"

key = None
for line in ENV_FILE.read_text().splitlines():
    if line.startswith("GEMINI_API_KEY="):
        key = line.split("=", 1)[1].strip()
        break
if not key:
    sys.exit("ERROR: no GEMINI_API_KEY")

PROMPT = """You are Gemini DeepThink, the strategic architectural reviewer for TuringOS v4.

This is a **round-3 closure-verification audit** on CO1.1.4-pre1 v1.2 (commit `f4649a9`).

Round-2 returned: **Codex CHALLENGE/high** (4 patch-mechanical defects) + **YOU returned PASS/high**. Per memory `feedback_dual_audit_conflict`: conservative merged CHALLENGE → v1.2 patch round.

You PASSed v1.1; this round-3 is a **narrow-scope verification** that v1.2 closed Codex's CHALLENGE without introducing strategic regressions or weakening your previous PASS basis.

## What changed v1.1 → v1.2

| Patch | v1.1 issue (Codex round-2 finding) | v1.2 fix claim |
|---|---|---|
| **P11** | SignalKind::Finalize.claim_id leaked TxId | claim_id: ClaimId throughout |
| **P12** | No CanonicalMessage variants for FinalizeReward / TaskExpire signing → dual-sign rationale not executable for 2 of 3 system txs | NEW FinalizeRewardSigning + TaskExpireSigning variants + emitter fns symmetric to TerminalSummarySigning |
| **P13** | Spec drift around TerminalSummary placement (§ 0 / § 6 / § 9 D-3) | All 3 stale references cleaned |
| **P14** | Round-1 domain-prefix test was non-load-bearing (different bodies) | NEW signing_payload_domain_prefix_is_load_bearing (identical body, 6 distinct domains, 6 distinct digests) + extended signature-exclusion tests to all 6 + locked signing-payload golden hex |
| **P15** | BTreeMap permutation only tested BTreeSet | NEW BTreeMap permutation test using PredicateResultsBundle.acceptance |
| **GR-1** (your recommendation) | MetaTx domain not reserved | NEW DOMAIN_AGENT_META_PROPOSAL constant for v4.1 namespace reservation |
| **GR-2** (your recommendation) | TransitionError additive-only commitment absent | spec § 7.2 NEW additive-only commitment for ALL ABI enums (TransitionError / TxStatus / RejectionClass / VerifyVerdict / RunOutcome / SafetyOrCreation / SignalKind / CanonicalMessage / TxKind) |
| **GR-3** (your recommendation) | Domain rotation process undocumented | spec § 7.3 NEW rotation process (parallel `.v2` add, runtime accepts both during quiescence) |

## Round-3 strategic verification questions

**Q1. P11 (ClaimId completeness)**: does the SignalBundle migration to ClaimId preserve constitutional alignment (Inv 3 escrow conservation derives finalize-reward via claim_id; right typing here protects against accidental cross-claim reuse)?

**Q2. P12 (symmetric system signing)**: does adding 2 more CanonicalMessage variants strengthen or weaken the "all sign goes through CanonicalMessage" invariant? Are 5 variants (RejectedAttemptSummary + TerminalSummarySigning + FinalizeRewardSigning + TaskExpireSigning + EpochRotationProof + LedgerEntrySigning = 6 actually) the right surface, or is this getting variant-heavy?

**Q3. P13 (spec drift cleanup)**: any architectural risk from leaving an HTML-comment marker in § 9 D-3's slot vs deleting the row entirely? (Project pattern: which approach honors "audit honesty"?)

**Q4. P14 + P15 (test completeness)**: is the v1.2 test surface (24 typed_tx tests up from 17) now sufficient to lock the ABI? Anything you'd still recommend adding before declaring the ABI "frozen"?

**Q5. GR-1/2/3 implementation**: did your 3 recommendations get implemented in spirit (not just literally)? Any drift?

**Q6. NEW strategic risks introduced by v1.2**: anything from the patch set that wasn't in your round-2 PASS analysis? Specifically:
- 22 → 22 TransitionError variants unchanged but additive-only commitment now binding
- 4 → 6 CanonicalMessage variants (added 2)
- Domain rotation process — does the documented process actually preserve forward replay-compatibility?

**Q7. Final holistic verdict on v1.2 closure**: PASS / CHALLENGE / VETO

End with:
- Top 3 remaining must-fix (if CHALLENGE)
- Conviction

## Output format

# Gemini CO1.1.4-pre1 Round-3 Audit
## Q1 P11 ClaimId completeness
## Q2 P12 symmetric system signing
## Q3 P13 spec drift cleanup
## Q4 P14+P15 test completeness
## Q5 GR-1/2/3 implementation fidelity
## Q6 New strategic risks
## Q7 **VERDICT**: PASS / CHALLENGE / VETO
## Top 3 must-fix (if CHALLENGE)
## Conviction

Be rigorous but terse — this is a narrow-scope round-3.
"""

DOCS = [
    ("DOC: CO1.1.4-pre1 spec v1.2 (target)", "handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md"),
    ("DOC: src/state/typed_tx.rs v1.2", "src/state/typed_tx.rs"),
    ("DOC: src/bottom_white/ledger/system_keypair.rs v1.2 (P12 target)", "src/bottom_white/ledger/system_keypair.rs"),
    ("DOC: round-2 merged verdict (closure check reference)", "handover/audits/CO1_1_4_PRE1_DUAL_AUDIT_VERDICT_R2_2026-04-28.md"),
    ("XREF: round-1 merged verdict (historical context)", "handover/audits/CO1_1_4_PRE1_DUAL_AUDIT_VERDICT_R1_2026-04-28.md"),
    ("XREF: STATE_TRANSITION_SPEC v1.4 (frozen)", "handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md"),
]

parts = [PROMPT]
for label, rel in DOCS:
    p = REPO / rel
    parts.append(f"\n\n=== {label} ===\n\n")
    parts.append(p.read_text())
text = "".join(parts)

with OUT.open("w") as f:
    f.write(f"# Gemini CO1.1.4-pre1 Round-3 Audit Run\n")
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
