#!/usr/bin/env python3
"""Gemini round-1 audit on CO1.7-impl BUNDLE (A1+A2+A3+A4) + CO1.4-extra."""
import json
import pathlib
import subprocess
import sys
import urllib.request

REPO = pathlib.Path("/home/zephryj/projects/turingosv4")
ENV_FILE = pathlib.Path("/home/zephryj/projects/turingosv3/.env")
OUT = REPO / "handover/audits/GEMINI_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md"

key = None
for line in ENV_FILE.read_text().splitlines():
    if line.startswith("GEMINI_API_KEY="):
        key = line.split("=", 1)[1].strip()
        break
if not key:
    sys.exit("ERROR: no GEMINI_API_KEY")

PROMPT = """You are Gemini DeepThink, the strategic architectural reviewer for TuringOS v4.

This is a **round-1 dual external audit** on the CO1.7-impl BUNDLE (A1+A2+A3+A4) + CO1.4-extra. Codex is running an independent round-1 in parallel (implementer angle); your angle is **strategic / constitutional / forward-sustainability**.

Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.

## What's in the bundle

| Atom | LoC | Description |
|---|---|---|
| **A1** Git2LedgerWriter | ~525 | git2-rs commit chain on `refs/transitions/main`; tree blobs; deterministic author time |
| **A2** Sequencer | ~290 | K1 dual counter; tokio mpsc; apply_one 9 stages; K3 head_t deferred to CO1.7.5+ |
| **A3** dispatch_transition | ~30 | Exhaustive 7-variant match; all NotYetImplemented stubs (CO1.7.5 fills) |
| **A4** replay_full_transition | ~140 | 9-stage full-mode replay; LedgerCasView trait; 4 conformance tests (1 #[ignore] for CO1.7.5) |
| **CO1.4-extra** | ~110 | Sidecar JSONL CAS index persistence; closes Art 0.2 cold-replay gate |

237/0 lib PASS + 1 ignored. Pre-implementation gate per CO1.7 spec § 12.

## Round-1 strategic questions

**Q1. Constitutional alignment** — does the bundle uphold:
- Art 0.1 四要素 (Tape / Input-Tape / Q / State): Sequencer is the input-tape mediator; LedgerEntry is the tape unit; replay reconstructs Q from tape — correct mapping?
- Art 0.2 Tape Canonical: replay_full_transition + CO1.4-extra together close the cold-replay gate. Sufficient?
- Art 0.4 Q_t = ⟨q_t, HEAD_t, tape_t⟩ version-controlled: head_t is NOT mutated (deferred to CO1.7.5+ wiring). Is this Art 0.4 violation, or acceptable interim state given v1.2 spec deferred K3?
- Anti-Oreo 三层: Sequencer in `state::`; ledger storage in `bottom_white::ledger::`; predicates in `top_white::predicates::`; tools in `bottom_white::tools::`. Layer purity intact?

**Q2. WP § 5.L4 vs CO1.7-impl bundle** — the L4 layer per WP § 5 has machinery (sign + commit + replay) implemented; does this faithfully match the WP's L4 vision? Anything missing that would surface as a Wave 6 #2/#3 gap?

**Q3. CO1.4-extra design choice** — sidecar JSONL was chosen per "压缩即智能" principle. Constitutional review:
- Append-only ✅ Art 0.2 alignment
- Strict-mode on corruption ✅ honest failure
- Deterministic per-line ordering ✅ replay-friendly
- Trade-off: O(N) restart cost — does it scale to Wave 6 production sizes (10K-100K CAS objects)?

**Q4. K3 head_t deferral risk** — Art 0.4 says Q_t includes head_t = git commit SHA. CO1.7-impl explicitly does NOT mutate head_t (per CO1.7 spec K3 v1.2 — deferred to CO1.7.5+ wiring). For the duration of CO1.7-impl runtime, head_t stays at QState::default empty string. Is this an acceptable interim state, or does it create observable runtime inconsistency (e.g., tests querying head_t get stale data)?

**Q5. Cross-cell isolation (§ 5.2.2)** — Sequencer is single-writer per (runtime_repo, run_id). 100-cell Phase C scenario: 100 Sequencers + 100 runtime_repos + 100 sidecar files. Is this operationally sound, or is there an O(N) explosion concern?

**Q6. CO1.7.5 unblock contract** — what does the next atom (CO1.7.5: per-kind transition function bodies + STEP_B wiring) need to deliver for the FULL L4 to be production-ready? Are there visible omissions in this bundle that CO1.7.5 will struggle with?

**Q7. Forward sustainability** — if Wave 6 #2/#3 (CO1.8 L5 materializer + CO1.9 L6 signal indices) need to extend the ledger schema (e.g., add settlement proofs), does this bundle's design (LedgerEntry's `extensions: BTreeMap<String, Vec<u8>>` + additive-only ABI) accommodate it cleanly?

**Q8. Audit cost so far + what's saved by bundling** — 4 atoms in 1 audit (vs 4 separate audits). Is the bundling defensible, or did it dilute audit focus?

**Q9. New strategic risks** introduced by this bundle that weren't in the spec-phase audits?

**Q10. Final holistic verdict on the bundle**: PASS / CHALLENGE / VETO

End with:
- Top 3 must-fix (if CHALLENGE)
- Top 3 architectural risks (if VETO)
- Conviction (low/med/high)

## Output format

# Gemini CO1.7-impl Bundle Round-1 Audit
## Q1 Constitutional alignment
## Q2 WP § 5.L4 conformance
## Q3 CO1.4-extra design
## Q4 K3 head_t deferral risk
## Q5 Cross-cell isolation scaling
## Q6 CO1.7.5 unblock contract
## Q7 Forward sustainability
## Q8 Audit bundling defense
## Q9 New strategic risks
## Q10 **VERDICT**: PASS / CHALLENGE / VETO
## Top 3 must-fix / risks
## Conviction

Be rigorous. Cite spec § + code line where possible.
"""

DOCS = [
    ("DOC: CO1.7 spec v1.2 (PASS/PASS)", "handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md"),
    ("DOC: CO1.1.4-pre1 spec v1.2.2 (PASS/PASS)", "handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md"),
    ("DOC: src/bottom_white/ledger/transition_ledger.rs (A1+A4)", "src/bottom_white/ledger/transition_ledger.rs"),
    ("DOC: src/state/sequencer.rs (A2+A3)", "src/state/sequencer.rs"),
    ("DOC: src/bottom_white/cas/store.rs (CO1.4-extra)", "src/bottom_white/cas/store.rs"),
    ("DOC: src/state/typed_tx.rs (TypedTx ABI)", "src/state/typed_tx.rs"),
    ("DOC: src/bottom_white/ledger/system_keypair.rs (signing primitives)", "src/bottom_white/ledger/system_keypair.rs"),
    ("DOC: src/state/q_state.rs (QState)", "src/state/q_state.rs"),
    ("XREF: STATE_TRANSITION_SPEC v1.4 (frozen)", "handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md"),
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
    f.write(f"# Gemini CO1.7-impl Bundle Round-1 Audit Run\n")
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
