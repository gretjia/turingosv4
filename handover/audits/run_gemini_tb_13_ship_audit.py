#!/usr/bin/env python3
"""TB-13 CompleteSet + MarketSeedTx — Gemini Class 3 ship audit (architect Part A §4.8 + feedback_dual_audit)."""
import json
import os
import sys
import time
import urllib.request
from pathlib import Path

ROOT = Path("/home/zephryj/projects/turingosv4")
ENV_FILES = [ROOT / ".env"]
ROUND = os.environ.get("TB13_AUDIT_ROUND", "R1")
OUT = ROOT / f"handover/audits/GEMINI_TB_13_SHIP_AUDIT_2026-05-03_{ROUND}.md"

if OUT.exists():
    print(f"[gemini tb-13] error: {OUT} already exists; refusing to overwrite", file=sys.stderr)
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
    print("[gemini tb-13] GEMINI_API_KEY not in .env", file=sys.stderr)
    sys.exit(2)

# ── Brief ──
brief = """# Gemini TB-13 Ship Audit — CompleteSet + MarketSeedTx (Class 3 architectural strategic review)

**Role**: skeptical adversarial reviewer. Independent of Codex. Conservative merge rule (memory `feedback_dual_audit_conflict`): VETO > CHALLENGE > PASS.

**Mandate**: TB-13 of TURINGOS v4 implements per architect 2026-05-03 post-TB-12 ruling Part A §4: introduce conditional-share substrate "1 locked Coin = 1 YES_E + 1 NO_E". Three new typed-tx variants (CompleteSetMintTx, CompleteSetRedeemTx, MarketSeedTx) all agent-signed. EconomicState extended 11 → 13 sub-fields (+conditional_collateral_t Coin holding per CR-13.4; +conditional_share_balances_t claims NOT in supply per CR-13.3 + SG-13.2).

The architect's TB-13 forbidden list (Part A §4.7) explicitly bans: AMM / CPMM / orderbook / MarketOrderTx / MarketTradeTx / PriceIndex / DPMM / pro-rata / automatic liquidity / ghost liquidity / NodeMarketEntry as canonical state / f64 in money path / import of legacy `src/prediction_market.rs`.

**Architect Part A §4 + charter §3 Atom 6 mandated audit questions** (these are YOUR primary mandate):

1. Does CompleteSetMint create or destroy money? (Must be balance↔collateral migration only.)
2. Can Redeem fire without a system-emitted resolution? (Must be sequencer-rejected with `RedeemBeforeResolution`.)
3. Can Redeem with `outcome=Yes` and a TaskBankruptcy-style resolution_ref bypass the outcome check? (Must be rejected with `InvalidResolutionRef`.)
4. Does the 6-holding `total_supply_micro` sum hold across all TB-13 typed_tx?
5. Does `assert_complete_set_balanced` (MIN-semantics: `min(Σ_yes, Σ_no) == collateral`) hold after every transition?
6. Can MarketSeedTx create liquidity without provider balance? (Must be rejected with `InsufficientBalanceForMint` or `InsufficientCollateral`.)
7. Are conditional shares anywhere counted as Coin? (Must be excluded — CR-13.3 + SG-13.2.)
8. Could a malformed `ShareAmount` underflow at redeem? (`u128` type guarantee + `RedeemMoreThanOwned` gate.)
9. Forward-fence: does any new TB-13 module file import legacy `prediction_market`?

**Plus architectural strategic questions** (Class 3 review beyond impl-paranoid):

10. Does CompleteSet schema extend cleanly to TB-14 PriceIndex (long/short interest derivable from `conditional_share_balances_t` aggregates)?
11. Does the `EventId == TaskId` 1:1 simplification hold up under TB-14+ multi-event-per-task scenarios?
12. Is the `ResolutionRef` model robust to multi-resolver scenarios in TB-15+?
13. Is the MIN-semantics `assert_complete_set_balanced` invariant the right form (vs. strict equality), particularly for adversarial patterns: e.g., re-mint after partial redeem, or repeated redeem-and-remint cycles? Trace through specific transition sequences.

**Recursive self-audit verdict**: PASS with no halting triggers fired. 12/12 SG-13.0..8 + 11/11 G ship gates pass. 13 integration tests + 8 unit tests pass. Real-LLM regression smoke: 7/7 chaintape replay indicators GREEN; EconomicState 13 sub-fields confirmed end-to-end.

**Iteration cap**: 72h. **Sync mode**: STOP after dual audit; user reviews verdict before SHIP.

End your audit with:
- **VERDICT**: PASS / CHALLENGE / VETO
- **Conviction**: low / medium / high
- **Recommendation**: PROCEED to SHIP / FIX-THEN-PROCEED / REDESIGN

Cite file:line for every finding. Be paranoid about Q1-Q9 (architect's mandated questions). Q10-Q13 are strategic — flag schema-evolution risk if you see it but don't VETO on theoretical-only concerns.

---

"""


def append_file(rel: str, lang: str = "rust"):
    fp = ROOT / rel
    if not fp.exists():
        return ""
    return f"\n## {rel}\n\n```{lang}\n{fp.read_text()}\n```\n"


# ── Reference docs ──
brief += "# Reference: Charter + Architect ruling + Recursive self-audit + Smoke evidence\n"
brief += append_file("handover/tracer_bullets/TB-13_charter_2026-05-03.md", "markdown")
brief += append_file("handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md", "markdown")
brief += append_file("handover/audits/RECURSIVE_AUDIT_TB_13_2026-05-03.md", "markdown")
brief += append_file("handover/evidence/tb_13_real_llm_smoke_2026-05-03/README.md", "markdown")

brief += "\n\n---\n\n# TB-12 carry-forward context (preceding TB; provides NodePosition exposure record substrate)\n"
brief += append_file("handover/directives/2026-05-03_TB12_NODE_EXPOSURE_INDEX_ARCHITECT_RULING.md", "markdown")

brief += "\n\n---\n\n# TB-11 carry-forward context (provides EvidenceCapsule + TaskBankruptcyTx + TerminalSummaryTx system-emitted resolution anchors)\n"
brief += append_file("handover/directives/2026-05-02_TB11_EPISTEMIC_EXHAUST_ARCHITECT_RULING.md", "markdown")

# ── TB-13 source files under audit ──
brief += "\n\n---\n\n# TB-13 source code under audit\n"
for rel in [
    "src/state/typed_tx.rs",
    "src/state/q_state.rs",
    "src/state/sequencer.rs",
    "src/state/mod.rs",
    "src/economy/monetary_invariant.rs",
    "src/prediction_market.rs",
    "src/kernel.rs",
    "src/bottom_white/ledger/transition_ledger.rs",
    "src/runtime/run_summary.rs",
    "tests/tb_13_complete_set.rs",
    "tests/tb_13_legacy_cpmm_forward_fence.rs",
    "handover/alignment/OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03.md",
]:
    lang = "markdown" if rel.endswith(".md") else "rust"
    brief += append_file(rel, lang)

brief += "\n---\n\nGive your INDEPENDENT TB-13 ship audit. Be paranoid about Q1-Q9 (the architect's mandated halting triggers + the resolution-gating + conservation + complete-set-balanced invariants). Cite file:line for every finding.\n"

print(f"[gemini tb-13] prompt size: {len(brief):,} chars", file=sys.stderr)

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
    print(f"[gemini tb-13] API error: {e}", file=sys.stderr)
    sys.exit(1)
elapsed = time.time() - t0
print(f"[gemini tb-13] API returned in {elapsed:.1f}s", file=sys.stderr)

# Extract text
try:
    text = data["candidates"][0]["content"]["parts"][0]["text"]
except (KeyError, IndexError) as e:
    print(f"[gemini tb-13] response shape error: {e}\nfull response: {json.dumps(data, indent=2)[:2000]}", file=sys.stderr)
    sys.exit(1)

header = (
    f"# Gemini TB-13 Ship Audit — CompleteSet + MarketSeedTx (Class 3)\n"
    f"**Round**: {ROUND}\n"
    f"**Date**: 2026-05-03\n"
    f"**Test baseline**: cargo test --workspace = 783 PASS / 0 FAILED / 150 ignored\n"
    f"**Recursive self-audit**: PASS (5 clauses + 12 SG + 11 G + 0/7 halts)\n"
    f"**Real-LLM regression smoke**: PASS (7/7 chaintape replay indicators GREEN)\n"
    f"**Elapsed**: {elapsed:.1f}s\n"
    f"**Prompt size**: {len(brief):,} chars\n"
    f"**Audit mode**: Class 3 dual (Codex impl-paranoid + Gemini architectural strategic; per architect Part A §4.8)\n\n---\n\n"
)
OUT.write_text(header + text)
print(f"[gemini tb-13] saved: {OUT}")
