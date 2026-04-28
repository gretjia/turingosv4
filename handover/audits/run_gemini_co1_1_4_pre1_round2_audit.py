#!/usr/bin/env python3
"""Gemini round-2 audit on CO1.1.4-pre1 v1.1 (post round-1 CHALLENGE).

Closure verification of P1-P10 patches. Independent of Codex round-2.
"""
import json
import pathlib
import subprocess
import sys
import urllib.request

REPO = pathlib.Path("/home/zephryj/projects/turingosv4")
ENV_FILE = pathlib.Path("/home/zephryj/projects/turingosv3/.env")
OUT = REPO / "handover/audits/GEMINI_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md"

key = None
for line in ENV_FILE.read_text().splitlines():
    if line.startswith("GEMINI_API_KEY="):
        key = line.split("=", 1)[1].strip()
        break
if not key:
    sys.exit("ERROR: no GEMINI_API_KEY")

PROMPT = """You are Gemini DeepThink, the strategic architectural reviewer for TuringOS v4.

This is a **round-2 closure-verification audit** on CO1.1.4-pre1 v1.1 (commit `e0e4565`). Round-1 returned CHALLENGE/CHALLENGE; v1.1 claims to close 10 patches (P1-P10) per the merged verdict at `handover/audits/CO1_1_4_PRE1_DUAL_AUDIT_VERDICT_R1_2026-04-28.md`. Codex is running an independent round-2 in parallel (implementer angle); your angle is **strategic / constitutional / closure-verification**.

Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS. Conservative wins on disagreement.

## Your previous (round-1) verdict

You returned **CHALLENGE** with high conviction. Top 3 must-fix:
1. Signature security & ambiguity (Q6, Q7) — domain separation for agent-signed payloads + clarify FinalizeRewardTx.system_signature
2. Incomplete & unlocked ABI tests (Q9) — golden fixtures must hardcode SHA-256 hex
3. Constitutional violation on cold replay (Q4) — PASS gated on CO1.4-extra commitment

## What changed v1 → v1.1

| Patch | v1 issue | v1.1 fix claim | What to verify (your angle) |
|---|---|---|---|
| **P1** | Agent-sig domain separation absent | NEW signing-payload structs (Work / Verify / Challenge / FinalizeReward / TaskExpire / TerminalSummary) each with canonical_digest() prepending b"turingosv4.<actor>.<purpose>.v1" before bincode body | Constitutional: does this satisfy "domain separation is non-negotiable"? Are domain strings stable enough to lock as part of the constitution-tier ABI? |
| **P2** | claim_id: TxId leaked impl | ClaimId(pub TxId) #[serde(transparent)] | Type-distinction sufficient at API surface? |
| **P3** | TerminalSummaryTx 3-field placeholder | Migrated to 8-field STATE § 1.5 schema in state::typed_tx; system_keypair signs opaque [u8;32] via NEW CanonicalMessage::TerminalSummarySigning variant | Constitutional: is the migration HONEST (full schema present + old removed)? bottom_white ↔ state circular-dep risk eliminated? |
| **P4** | TransitionError taxonomy incomplete | Expanded 10 → 22 variants; NotYetImplemented kept as explicit stub | Variant set covers STATE § 3 pseudocode? |
| **P5** | Golden fixtures unlocked | Hardcoded SHA-256 hex for all 7 variants; +6 new tests | Locked properly? Cross-variant non-collision actually pairwise? |
| **P6** | STATE § 2.5 wording wrong | spec § 7.1 codec wording fixed (u32 variants, u64 lengths) | Wording accurate now per actual bincode-2 behavior? |
| **P7** | D-3 divergence | RESOLVED via P3; row removed | Row fully removed (not just edited)? |
| **P8** | FinalizeRewardTx ambiguity | spec § 4.1 Q-derived discipline; § 4.2 dual-sign rationale | Q-derived rule clear at replay? Dual-sign rationale convincing — or sigs still redundant? |
| **P9** | Cold-replay Art 0.2 | spec § 0.1 cross-atom ordering gate (CO1.4-extra MUST ship before CO1.7-impl A4) | Constitutional: ordering gate strong enough? "MUST NOT" binding? |
| **P10** | TaskId vs TxId QState mismatch | spec § 9 D-4: cross-atom debt to CO P2.1 | Honest forward-migration plan? Wire-format consequence correctly assessed? |

## Round-2 strategic / constitutional questions

**Q1. P1 + Q7 round-1 closure**: do the new domain prefixes (`b"turingosv4.<actor>.<purpose>.v1"`) constitutionally close the type-confusion attack? Is the b"v1" version suffix wise (forward-compat for v2 domain rotation), or does it lock us into a bincode-style versioning hazard?

**Q2. Constitutional alignment unchanged or improved**: P3 (TerminalSummaryTx migration) re-locates a typed-tx schema across a layer boundary (bottom_white → state). Does this preserve Anti-Oreo three-layer purity? Should typed-tx schemas LIVE in state per the architectural principle, or in bottom_white where the signer is?

**Q3. P9 + Q4 round-1 closure**: spec § 0.1 says A4 (replay_full_transition) MUST NOT ship before CO1.4-extra. Strong commitment? Compare to the language pattern in CO1.7 spec for similar deferrals — is this consistent with project precedent?

**Q4. P8 dual-sign rationale**: do you accept "FinalizeRewardTx.system_signature signs the payload bytes; LedgerEntrySigningPayload signs the envelope; both needed" as a non-redundant design? Or does this still feel like belt-and-suspenders that should be simplified to a single sig (drop FinalizeRewardTx.system_signature; rely on envelope sig only)?

**Q5. v4/v4.1 boundary preservation under v1.1 changes**: do the new 6 SigningPayload structs + ClaimId + 22-variant TransitionError preserve "additive variants only" property? Could v4.1 introduce MetaTx + MetaSigningPayload without breaking these golden fixtures?

**Q6. Inv 3 (escrow conservation) interaction unchanged**: P2 ClaimId newtype + P8 Q-derived discipline don't disturb Inv 3. But P5 golden fixture for FinalizeRewardTx hardcodes a specific reward value (5_000_000 microcoin). Is this a smell that fixtures should test multiple escrow-balance scenarios, or is one fixture sufficient because the test asserts encoding stability (not economic semantics)?

**Q7. Test strategy completeness now**: do the +6 new tests (cross-variant non-collision, BTreeSet permutation, default round-trip, signing-payload domain distinctness, signing-payload-excludes-signature, TerminalSummary in round-trip+kind) close the round-1 Q9 gap entirely? Anything still missing?

**Q8. Forward sustainability**: with 6 SigningPayload structs locked in v1.1, is this enough domain coverage for CO P3 public market? Should there be a 7th `MetaSigningPayload` placeholder (even if MetaTx itself is v4.1) so the domain prefix is reserved?

**Q9. Strategic risks introduced by v1.1 (anything new)**:
- Any new strategic concern from the patch set that wasn't in round-1?
- 22-variant TransitionError forward-evolution — additive-only commitment in spec?
- Domain-prefix versioning lock-in: how to rotate domain strings without breaking existing fixtures?

**Q10. Final holistic verdict on v1.1 closure**: PASS / CHALLENGE / VETO

End with:
- Top 3 remaining must-fix (if CHALLENGE)
- Top 3 architectural risks (if VETO)
- Conviction (low/med/high)

## Output format

# Gemini CO1.1.4-pre1 Round-2 Audit
## Q1 P1 + Q7 round-1 closure
## Q2 Constitutional alignment under P3
## Q3 P9 + Q4 round-1 closure
## Q4 P8 dual-sign rationale
## Q5 v4/v4.1 boundary
## Q6 Inv 3 interaction
## Q7 Test strategy completeness
## Q8 Forward sustainability
## Q9 New strategic risks
## Q10 **VERDICT**: PASS / CHALLENGE / VETO
## Top 3 must-fix / risks
## Conviction

Be rigorous. Cite spec § + code line where possible. Per CLAUDE.md, do NOT pass on principle; do NOT veto on principle.
"""

DOCS = [
    ("DOC: CO1.1.4-pre1 spec v1.1 (target of audit)", "handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md"),
    ("DOC: src/state/typed_tx.rs v1.1 (target of audit)", "src/state/typed_tx.rs"),
    ("DOC: src/state/mod.rs v1.1 (re-exports)", "src/state/mod.rs"),
    ("DOC: src/bottom_white/ledger/system_keypair.rs v1.1 (TerminalSummary migration target)", "src/bottom_white/ledger/system_keypair.rs"),
    ("DOC: round-1 merged verdict (closure check reference)", "handover/audits/CO1_1_4_PRE1_DUAL_AUDIT_VERDICT_R1_2026-04-28.md"),
    ("XREF: STATE_TRANSITION_SPEC v1.4 (frozen)", "handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md"),
    ("XREF: CO1.7 spec v1.2 (consumes this ABI)", "handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md"),
    ("XREF: src/bottom_white/ledger/transition_ledger.rs (CO1.7-impl A1; consumes typed_tx)", "src/bottom_white/ledger/transition_ledger.rs"),
    ("XREF: WP v2.2 (Anti-Oreo restoration)", "handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md"),
    ("XREF: Constitution.md", "constitution.md"),
]

parts = [PROMPT]
for label, rel in DOCS:
    p = REPO / rel
    parts.append(f"\n\n=== {label} ===\n\n")
    parts.append(p.read_text())
text = "".join(parts)

with OUT.open("w") as f:
    f.write(f"# Gemini CO1.1.4-pre1 Round-2 Audit Run\n")
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
