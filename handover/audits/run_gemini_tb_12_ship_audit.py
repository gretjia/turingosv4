#!/usr/bin/env python3
"""TB-12 Node Exposure Index — Gemini Class 3 ship audit (architect 2026-05-03 §5)."""
import json
import os
import sys
import time
import urllib.request
from pathlib import Path

ROOT = Path("/home/zephryj/projects/turingosv4")
ENV_FILES = [ROOT / ".env"]
ROUND = os.environ.get("TB12_AUDIT_ROUND", "R1")
OUT = ROOT / f"handover/audits/GEMINI_TB_12_SHIP_AUDIT_2026-05-03_{ROUND}.md"

if OUT.exists():
    print(f"[gemini tb-12] error: {OUT} already exists; refusing to overwrite", file=sys.stderr)
    sys.exit(2)


def load_env():
    env = {}
    for fp in ENV_FILES:
        if not fp.exists():
            continue
        for line in fp.read_text().splitlines():
            if "=" in line and not line.strip().startswith("#"):
                k, v = line.split("=", 1)
                env.setdefault(k.strip(), v.strip().strip('"').strip("'"))
    return env


env = load_env()
if "GEMINI_API_KEY" not in env:
    print("[gemini tb-12] GEMINI_API_KEY not in .env", file=sys.stderr)
    sys.exit(2)

# ── Brief ──
brief = """# Gemini TB-12 Ship Audit — Node Exposure Index (Class 3 architectural strategic review)

**Role**: skeptical adversarial reviewer. Independent of Codex. Conservative merge rule (memory `feedback_dual_audit_conflict`): VETO > CHALLENGE > PASS.

**Mandate**: TB-12 of TURINGOS v4 implements per architect 2026-05-03 ruling: "Node exposure index — NOT trading market. Records WorkTx.stake → FirstLong + ChallengeTx.stake → ChallengeShort. NO price, NO trading, NO AMM, NO CompleteSet, NO ghost liquidity. NodePosition is IMMUTABLE EXPOSURE RECORD, NOT active position balance (architect §10)."

The architect explicitly chose **flat NodePositionsIndex** over nested NodeMarketEntry (architect §3 ruling: NodeMarketEntry is TB-14 derived view; flat is canonical to avoid second source-of-truth, mirroring TaskMarket.total_escrow precedent).

**Architect §8 Atom 6 mandated audit questions** (these are YOUR primary mandate):

1. Does NodePosition create a second money ledger? (Should NOT.)
2. Does replay reconstruct positions deterministically?
3. Does VerifyTx bond avoid market classification?
4. Does NodePosition avoid total supply counting?
5. Does TB-12 accidentally implement trading?

**Plus architectural strategic questions** (Class 3 review):

6. Does the flat NodePositionsIndex extend cleanly to TB-13 CompleteSet (which will introduce real YES/NO claims) without schema collision?
7. Does the flat-not-nested decision (architect §3 ruling) hold up at TB-14 PriceIndex when long/short aggregation IS needed?
8. Are the architect's halting triggers (CTF conservation failure / WorkTx-Challenge position mismatch / NodePosition counted as Coin / replay divergence / VETO) genuinely impossible given the TB-12 implementation?

**Codex round-1 verdict**: CHALLENGE × 2 (Q4 doc-drift on holding count; Q5 legacy CPMM scope question). Both resolved as documentation/scope clarifications via §10 of recursive self-audit. NO VETO. NO forge-or-replay vector found.

**Iteration cap**: 72h. **Sync mode (Q6 ii.5)**: STOP after dual audit; user reviews verdict before SHIP.

End your audit with:
- **VERDICT**: PASS / CHALLENGE / VETO
- **Conviction**: low / medium / high
- **Recommendation**: PROCEED to SHIP / FIX-THEN-PROCEED / REDESIGN

Cite file:line for every finding.

---

"""


def append_file(rel: str, lang: str = "rust"):
    fp = ROOT / rel
    if not fp.exists():
        return ""
    return f"\n## {rel}\n\n```{lang}\n{fp.read_text()}\n```\n"


# ── Reference docs ──
brief += "# Reference: Charter + Architect ruling + Recursive self-audit\n"
brief += append_file("handover/tracer_bullets/TB-12_charter_2026-05-03.md", "markdown")
brief += append_file("handover/directives/2026-05-03_TB12_NODE_EXPOSURE_INDEX_ARCHITECT_RULING.md", "markdown")
brief += append_file("handover/audits/RECURSIVE_AUDIT_TB_12_2026-05-03.md", "markdown")
brief += append_file("handover/audits/CODEX_TB_12_SHIP_AUDIT_2026-05-03.md", "markdown")

brief += "\n\n---\n\n# TB-11 carry-forward context (preceding TB; provides SubstrateEvidenceCapsule + RunExhausted + TaskBankruptcy)\n"
brief += append_file("handover/directives/2026-05-02_TB11_EPISTEMIC_EXHAUST_ARCHITECT_RULING.md", "markdown")
brief += append_file("handover/directives/2026-05-02_TB11_TO_TB17_SUPPLEMENTARY_DIRECTIVE.md", "markdown")

# ── TB-12 source files ──
brief += "\n\n---\n\n# TB-12 source code under audit\n"
for rel in [
    "src/state/typed_tx.rs",
    "src/state/q_state.rs",
    "src/state/sequencer.rs",
    "src/state/mod.rs",
    "src/economy/monetary_invariant.rs",
    "src/bin/audit_dashboard.rs",
    "experiments/minif2f_v4/src/bin/evaluator.rs",
    "experiments/minif2f_v4/src/bin/lean_market.rs",
    "tests/tb_12_node_exposure_index.rs",
    "src/runtime/evidence_capsule.rs",
    "src/runtime/adapter.rs",
]:
    brief += append_file(rel, "rust")

brief += "\n---\n\nGive your INDEPENDENT TB-12 ship audit. Be paranoid about Q1 + Q4 (the architect's critical halting triggers). Cite file:line for every finding.\n"

print(f"[gemini tb-12] prompt size: {len(brief):,} chars", file=sys.stderr)

# ── Call ──
url = (
    "https://generativelanguage.googleapis.com/v1beta/models/"
    f"gemini-2.5-pro:generateContent?key={env['GEMINI_API_KEY']}"
)
body = json.dumps({
    "contents": [{"parts": [{"text": brief}]}],
    "generationConfig": {"temperature": 0.2, "maxOutputTokens": 16384},
}).encode()

t0 = time.time()
req = urllib.request.Request(
    url, data=body, headers={"Content-Type": "application/json"}, method="POST"
)
try:
    with urllib.request.urlopen(req, timeout=900) as resp:
        data = json.loads(resp.read())
except Exception as e:
    print(f"[gemini tb-12] API error: {e}", file=sys.stderr)
    sys.exit(1)
elapsed = time.time() - t0
print(f"[gemini tb-12] API returned in {elapsed:.1f}s", file=sys.stderr)

# Extract text
try:
    text = data["candidates"][0]["content"]["parts"][0]["text"]
except (KeyError, IndexError) as e:
    print(f"[gemini tb-12] response shape error: {e}\nfull response: {json.dumps(data, indent=2)[:2000]}", file=sys.stderr)
    sys.exit(1)

header = (
    f"# Gemini TB-12 Ship Audit — Node Exposure Index (Class 3)\n"
    f"**Round**: {ROUND}\n"
    f"**Date**: 2026-05-03\n"
    f"**Test baseline**: cargo test --workspace = 757 PASS / 0 FAILED / 150 ignored\n"
    f"**Codex round-1 verdict**: CHALLENGE × 2 (Q4 doc-drift, Q5 legacy CPMM scope) — both resolved via recursive self-audit §10\n"
    f"**Elapsed**: {elapsed:.1f}s\n"
    f"**Prompt size**: {len(brief):,} chars\n"
    f"**Audit mode**: Class 3 dual (Codex impl-paranoid + Gemini architectural strategic; per architect §5)\n\n---\n\n"
)
OUT.write_text(header + text)
print(f"[gemini tb-12] saved: {OUT}")
