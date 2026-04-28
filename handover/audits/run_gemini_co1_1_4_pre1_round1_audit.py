#!/usr/bin/env python3
"""Gemini round-1 audit on CO1.1.4-pre1 Typed Tx ABI surface — spec + impl + tests.

Strategic / constitutional angle: does the typed-tx ABI design uphold Anti-Oreo
+ Art 0.2 (tape canonicality) + Inv 3 (escrow conservation) + the v4/v4.1 boundary?
Independent of Codex round-1 (running in parallel).
"""
import json
import pathlib
import subprocess
import sys
import urllib.request

REPO = pathlib.Path("/home/zephryj/projects/turingosv4")
ENV_FILE = pathlib.Path("/home/zephryj/projects/turingosv3/.env")
OUT = REPO / "handover/audits/GEMINI_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md"

key = None
for line in ENV_FILE.read_text().splitlines():
    if line.startswith("GEMINI_API_KEY="):
        key = line.split("=", 1)[1].strip()
        break
if not key:
    sys.exit("ERROR: no GEMINI_API_KEY in /home/zephryj/projects/turingosv3/.env")

PROMPT = """You are Gemini DeepThink, the strategic architectural reviewer for TuringOS v4.

This is a **round-1 dual external audit** on CO1.1.4-pre1 Typed Tx ABI surface (spec + impl + 11 tests). Codex is running an independent round-1 in parallel (implementer-review angle); your angle is **strategic / constitutional / forward-sustainability**.

Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS. Conservative wins on disagreement.

## Why this atom exists

When CO1.7-impl A1 (Git2LedgerWriter, commit `a03cc52`) shipped the production storage backend, the downstream A2 (Sequencer + `dispatch_transition` + `replay_full_transition`) discovered that ~30 supporting schema types were specified in STATE_TRANSITION_SPEC § 1 but **none of them existed in code** (only `MicroCoin`). The architect chose option (b): split the ABI surface into its own atom (CO1.1.4-pre1), audit it under an independent gate, then unblock A2-A4. The alternative (a) was to absorb the ABI work into CO1.7-impl scope (~+1500 LoC); rejected because it would inflate the audit blast radius and conflate spec / impl decisions. Memory `feedback_phased_checkpoint`: atomic atoms get cleaner audits.

## What's at stake

CO1.1.4-pre1 PASS unblocks: (1) CO1.7-impl A2-A4; (2) CO1.7.5 (transition function bodies will use these types); (3) CO P2.x economy atoms (TaskId / AgentSignature / RejectionClass / SlashEvidenceCid are reused). It is also a constitutional load-bearing artifact: the `WorkTx` field set IS the wire format. Once locked, every future ledger entry is bincode-canonical against it.

## Round-1 strategic questions

**Q1. Constitutional alignment** — does the ABI uphold:
- Art 0.1 四要素 (Tape / Input-Tape / Q / State): is `TypedTx` the correct primary input-tape unit?
- Art 0.2 Tape Canonical 公理 (no rejection sidecar; no replay-equivalent state outside the tape): the spec § 9 D-1 divergence ELIDES `TxStatus` from WorkTx wire bytes (TxStatus tracked in `q_t.q_t.agents[id]` + ClaimsIndex). Is this CONSISTENT WITH or VIOLATES Art 0.2 (tape must be sufficient for replay)?
- Art 0.4 Q_t version-controlled state: TypedTx is the tx unit; LedgerEntry (CO1.7) is the envelope. The ABI cleanly separates payload vs envelope. Right structure?
- Anti-Oreo 三层 (top-white predicates / middle-black agents / bottom-white tools): typed_tx lives in `src/state/`, not bottom_white. Is `src/state/` semantic-correct for "the typed tape unit"?

**Q2. Inv 3 (escrow conservation) interaction** — economy invariant 3 demands monetary conservation: `Σ balances + Σ escrows + Σ stakes = const`. The ABI introduces:
- `StakeMicroCoin(MicroCoin)` newtype on stake fields — distinct from balance MicroCoin
- WorkTx.stake / VerifyTx.bond / ChallengeTx.stake all use StakeMicroCoin
- FinalizeRewardTx.reward uses bare MicroCoin (NOT StakeMicroCoin) — correct, this is a credit event
- Does the type system actually enforce Inv 3 at the wire level, or is this still purely runtime book-keeping?
- Should `EscrowMicroCoin` be a third newtype, or is the StakeMicroCoin/MicroCoin two-way distinction sufficient?

**Q3. v4 / v4.1 boundary preservation** — D-VETO-4=B ratified "defer MetaTape runtime to v4.1". Spec § 0 says "MetaTx + ancillaries (PredicatePatch / JudgeSignature / etc.) — v4.1 only". The ABI:
- has 7-variant `TypedTx` (no MetaTx variant)
- ↳ Adding `TypedTx::MetaTx(MetaTx)` in v4.1 is bincode-additive (new variant index = 7) but breaks any LedgerEntry digest fixtures that include the variant index — does the spec adequately commit to "additive variants only, never reorder"?
- does the absence of MetaTx accidentally leak into TransitionError / SignalBundle (e.g., a new TransitionError variant required for MetaTx wouldn't be additive-compatible if it slots into the middle of the enum)?

**Q4. WP § 5.L4 conformance + envelope/payload split** — WP § 5.L4 defines L4 as the typed transition tape. CO1.7 LedgerEntry is the envelope (wraps a CAS-stored TypedTx via `tx_payload_cid`). CO1.1.4-pre1 defines the payloads.
- Payloads (TypedTx) are CAS-stored opaque blobs as far as L4 is concerned; envelope (LedgerEntry) signs over the digest of the SIGNING_PAYLOAD struct (LedgerEntrySigningPayload, CO1.7).
- This is a 2-step indirection: L4 → CAS → typed payload. Is this consistent with Art 0.2 (tape canonicality), or is it a "rejection sidecar"-like architectural sin where payload data isn't on the tape itself?
- If CAS bytes are lost (cold replay before CO1.4-extra ships), the LedgerEntry is still verifiable but the underlying TypedTx is not retrievable — is this a "trust mode" ambiguity that violates Art 0.2?

**Q5. Reconstructibility (Art 0.2) interaction with TxStatus elision** — D-1 says TxStatus is runtime book-keeping. But Art 0.2 demands that Q_t MUST be reconstructible from the L4 transition ledger replay. So the question is: can `q_t.q_t.agents[id].last_accepted_tx` and ClaimsIndex be DERIVED from running `dispatch_transition` over each LedgerEntry's payload? If yes, D-1 is sound. If not, D-1 is a constitutional violation.
- Specifically: if a WorkTx is rejected, the rejected work_tx does NOT advance state_root_t (per I-PRED-GATE). Is rejection STATUS itself observable from L4 alone?
- If a WorkTx is accepted, then later challenged & slashed, that's a state-machine progression: Accepted → FinalizedSlash. Is this progression FULLY captured by the sequence of accepted typed-tx variants in L4?

**Q6. FinalizeRewardTx derivation** — spec § 4 derives the schema from § 3.4 call-site analysis. Strategic concerns:
- Should the schema include the `royalty_graph_t` snapshot at finalize time (so replay doesn't need full Q_t)?
- Should `claim_id` be a typed `ClaimId` newtype rather than reused `TxId`? The reuse leaks the implementation (claims are TxId-keyed in ClaimsIndex) into the wire format.
- The system_signature signs over what exactly? (Spec leaves canonical_digest construction to CO1.7's LedgerEntrySigningPayload, which signs the ENVELOPE not the payload — so FinalizeRewardTx's own system_signature might be REDUNDANT.)
  - If redundant: should the field be DROPPED in v1.1?
  - If non-redundant: spec must explicitly distinguish "agent-to-runtime sign" from "runtime-to-ledger sign".

**Q7. AgentSignature security model** — `AgentSignature(#[serde(with = "serde_bytes_64")] [u8; 64])` reuses the same serde adapter as SystemSignature. Wire bytes byte-identical for same content.
- Does this open a confusion attack where a system-signed payload is replayed as agent-signed (or vice versa)?
- The intent is type-distinction at the API surface. Constitutional argument: is type-level distinction sufficient, or does Art 0.2 require domain-separation (`"v4.agent_sig"` vs `"v4.system_sig"`) in the canonical_digest pre-image?
- Does CO1.7's `LedgerEntrySigningPayload.canonical_digest()` already domain-separate (`b"turingosv4.ledger_entry_signing.v1"`)? If yes, this might be sufficient. Verify.

**Q8. Forward sustainability** — if CO P3 phase opens public AGI market (per WP § 17), does CO1.1.4-pre1 ABI need extension (settlement proofs, ZK predicates, on-chain attestation, multi-party signatures)?
- LedgerEntry has `extensions: BTreeMap<String, Vec<u8>>` (CO1.7 G1) — does TypedTx need a similar forward-compat hatch?
- Should TypedTx's enum support `Extension(String, Vec<u8>)` for protocol-evolution signaling?

**Q9. Test strategy completeness** — spec § 7 commits to I-CANON-A/B/C (round-trip + byte-stability) + I-CANON-D (golden fixtures). Implementation has 11 tests.
- The golden_*_tx_digest tests currently assert digest STABILITY but don't lock the actual hex value (deferred to "phase 1: record only"). Is this acceptable for a v1 PASS, or must the locked hex values be in v1?
- Are there missing test classes: cross-variant non-collision (encode of variant A != encode of variant B), zero-value defaults round-trip, BTreeMap permutation independence?

**Q10. Final holistic verdict**: PASS / CHALLENGE / VETO

End with:
- Top 3 must-fix (if CHALLENGE)
- Top 3 architectural risks (if VETO)
- Conviction (low/med/high)

## Output format

# Gemini CO1.1.4-pre1 Round-1 Audit
## Q1 Constitutional alignment
## Q2 Inv 3 escrow conservation interaction
## Q3 v4/v4.1 boundary preservation
## Q4 WP § 5.L4 envelope/payload split
## Q5 Art 0.2 reconstructibility + TxStatus elision
## Q6 FinalizeRewardTx derivation
## Q7 AgentSignature security model
## Q8 Forward sustainability
## Q9 Test strategy completeness
## Q10 **VERDICT**: PASS / CHALLENGE / VETO
## Top 3 must-fix / risks
## Conviction

Be rigorous. Cite spec § + code line where possible. Per CLAUDE.md, do NOT pass on principle; do NOT veto on principle.
"""

DOCS = [
    ("DOC: CO1.1.4-pre1 spec v1 (target of audit)", "handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md"),
    ("DOC: src/state/typed_tx.rs (target of audit, joint with spec above)", "src/state/typed_tx.rs"),
    ("DOC: src/state/mod.rs (re-exports)", "src/state/mod.rs"),
    ("DOC: src/economy/money.rs (+StakeMicroCoin)", "src/economy/money.rs"),
    ("DOC: src/bottom_white/cas/schema.rs (+Default on Cid)", "src/bottom_white/cas/schema.rs"),
    ("DOC: src/bottom_white/ledger/system_keypair.rs (+Default + serde_bytes_64 pub(crate))", "src/bottom_white/ledger/system_keypair.rs"),
    ("XREF: STATE_TRANSITION_SPEC v1.4 (frozen, round-4 PASS/PASS)", "handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md"),
    ("XREF: CO1.7 spec v1.2 (PASS/PASS 2026-04-28; consumes this ABI)", "handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md"),
    ("XREF: src/bottom_white/ledger/transition_ledger.rs (CO1.7-impl A1; consumer of TypedTx)", "src/bottom_white/ledger/transition_ledger.rs"),
    ("XREF: src/state/q_state.rs (existing types: AgentId / TxId / Hash / NodeId / EconomicState)", "src/state/q_state.rs"),
    ("XREF: WP v2.2 § 5.L4 (Anti-Oreo restoration; ratified 2026-04-27)", "handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md"),
    ("XREF: Constitution.md (Art 0.1 / 0.2 / 0.4 + Anti-Oreo)", "constitution.md"),
]

parts = [PROMPT]
for label, rel in DOCS:
    p = REPO / rel
    parts.append(f"\n\n=== {label} ===\n\n")
    parts.append(p.read_text())
text = "".join(parts)

with OUT.open("w") as f:
    f.write(f"# Gemini CO1.1.4-pre1 Typed Tx ABI Round-1 Audit Run\n")
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
