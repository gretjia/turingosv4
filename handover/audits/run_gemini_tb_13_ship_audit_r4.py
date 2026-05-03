#!/usr/bin/env python3
"""TB-13 round-4 Gemini ship audit (post round-5 closure of RQ3 / Q9-RQ6 / RQ5)."""
import json
import os
import sys
import time
import urllib.request
from pathlib import Path

ROOT = Path("/home/zephryj/projects/turingosv4")
ENV_FILES = [ROOT / ".env"]
ROUND = "R4"
OUT = ROOT / f"handover/audits/GEMINI_TB_13_SHIP_AUDIT_2026-05-03_{ROUND}.md"

if OUT.exists():
    print(f"[gemini tb-13 r4] error: {OUT} already exists; refusing to overwrite", file=sys.stderr)
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
    print("[gemini tb-13 r4] GEMINI_API_KEY not in .env", file=sys.stderr)
    sys.exit(2)

# ── Brief ──
brief = """# Gemini TB-13 Ship Audit — round-4 (post round-5 fix closure)

**Role**: skeptical adversarial reviewer, architectural strategic angle. Independent of Codex (impl-paranoid). Conservative merge rule (memory `feedback_dual_audit_conflict`): VETO > CHALLENGE > PASS.

**Round context**: this is **round-4** in audit numbering (the project's session-internal "round-5" closure of RQ3 / Q9-RQ6 / RQ5 from the round-3 verdict). Round-3 verdicts:
- Codex round-3: CHALLENGE / high / FIX-THEN-PROCEED. 5 challenges, no VETO.
- Gemini round-3 (you): CHALLENGE / high / FIX-THEN-PROCEED. 1 challenge (Q12 ResolutionsIndex for TB-15+), explicit non-blocking.

**Mandate**: TB-13 of TURINGOS v4 implements per architect 2026-05-03 post-TB-12 ruling Part A §4: introduce conditional-share substrate "1 locked Coin = 1 YES_E + 1 NO_E". Three new typed-tx variants (CompleteSetMintTx, CompleteSetRedeemTx, MarketSeedTx) all agent-signed. EconomicState extended 11 → 13 sub-fields.

The architect's TB-13 forbidden list (Part A §4.7) explicitly bans: AMM / CPMM / orderbook / MarketOrderTx / MarketTradeTx / PriceIndex / DPMM / pro-rata / automatic liquidity / ghost liquidity / NodeMarketEntry as canonical state / f64 in money path / import of legacy `src/prediction_market.rs`.

## Round-5 closures (this session) — what to re-verify

The fresh session declined the prior session's "ship-with-OBS for everything" recommendation and structurally fixed three of six residual challenges:

```text
edbc555  TB-13 Atom 6 round-5 — Codex RQ5 remediation: drop ResolutionRef wrapper
a4f8265  TB-13 Atom 6 round-5 — Codex Q9/RQ6 remediation: type-use forward-fence discovery
ee8bfe8  TB-13 Atom 6 round-5 — Codex RQ3 remediation: non-empty TB-13 chaintape replay smoke
```

`cargo test --workspace = 791 passed / 0 failed / 150 ignored` (round-3 baseline 789 + 1 new RQ6 unit test + 1 new RQ3 chaintape smoke).

### What changed (your re-verification target)

**RQ5 — drop ResolutionRef wrapper**: ResolutionRef struct + resolution_tx_id + claimed_outcome fields removed from CompleteSetRedeemTx + signing payload. Inner-consistency check removed from sequencer. State-mismatch path preserved via existing match arm using `redeem.outcome` directly. CompleteSetRedeemTx 9→8 fields; signing payload 8→7. Wire-format break — no production rows yet. Trust Root rehashed for typed_tx / sequencer / transition_ledger. R-022 skip token at `handover/alignment/OBS_R022_TB13_RESOLUTIONREF_REMOVED_2026-05-03.md`.

**Q9/RQ6 — type-use forward-fence discovery**: `tests/tb_13_legacy_cpmm_forward_fence.rs` now has `TB_13_TYPE_NAMES` + `discover_by_type_use` walking src/ for non-comment uses of TB-13 type names. Catches contributors who import TB-13 types without authoring markers. New unit test exercises caught/skipped/neutral cases.

**RQ3 — non-empty TB-13 chaintape replay smoke**: New file `tests/tb_13_chaintape_smoke.rs` boots Git2LedgerWriter-backed sequencer with pre-seeded initial_q, wires real AgentKeypair, submits real signed CompleteSetMint + CompleteSetRedeem, asserts post-replay `final_state_root_hex == hex(live state_root_t)` — cryptographic proof of non-empty TB-13 map reconstruction (state_root is SHA-256 chain-fold over full QState). Evidence at `handover/evidence/tb_13_chaintape_smoke_2026-05-03/`.

## Architect Part A §4 + charter §3 Atom 6 mandated audit questions (your primary mandate)

Re-evaluate Q1-Q9 against round-5 HEAD:

1. Does CompleteSetMint create or destroy money? (Must be balance↔collateral migration only.)
2. Can Redeem fire without a system-emitted resolution? (Must be sequencer-rejected with `RedeemBeforeResolution`.)
3. Can Redeem with `outcome=Yes` against a Bankrupt event bypass the outcome check? (Must be rejected with `InvalidResolutionRef`. Round-5 RQ5 simplified the gate — single state-vs-outcome match arm now; no inner-consistency check.)
4. Does the 6-holding `total_supply_micro` sum hold across all TB-13 typed_tx?
5. Does `assert_complete_set_balanced` (MIN-semantics: `min(Σ_yes, Σ_no) == collateral`) hold after every transition?
6. Can MarketSeedTx create liquidity without provider balance? (Must be rejected.)
7. Are conditional shares anywhere counted as Coin? (Must be excluded — CR-13.3 + SG-13.2.)
8. Could a malformed `ShareAmount` underflow at redeem? (`u128` type guarantee + `RedeemMoreThanOwned` gate.)
9. Forward-fence: does any new TB-13 module file import legacy `prediction_market`? (Round-5 added type-use discovery layer; re-evaluate bypass surface.)

## Architectural strategic questions (Class 3 review)

10. Does CompleteSet schema extend cleanly to TB-14 PriceIndex (long/short interest derivable from `conditional_share_balances_t` aggregates)?
11. Does the `EventId == TaskId` 1:1 simplification hold up under TB-14+ multi-event-per-task scenarios?
12. With ResolutionRef now removed (round-5 RQ5), is the redeem-mechanism-coupling concern from your round-3 Q12 CHALLENGE addressed, partially addressed, or unchanged? Specifically: does removing the resolution_ref wrapper make TB-15+ multi-resolver evolution easier (fewer wire fields to reshape) or harder (lost a future-extension hook)?
13. Is the MIN-semantics `assert_complete_set_balanced` invariant the right form? Especially for adversarial sequences (re-mint after partial redeem, repeated redeem-and-remint cycles).

## Round-5-specific risks to scrutinize

- **RQ3 smoke determinism**: pre-computes post-mint state_root via `complete_set_mint_accept_state_root(&initial_root, &mint_tx)` to chain the redeem without racing the driver. Are there any QState fields that could go non-deterministic under replay (timestamps / random salts) that would invalidate the state-root equality argument?
- **RQ5 wire-format break**: Confirm there are no CAS payloads or persisted artifacts encoding the old (9-field / 8-field) shape.
- **RQ6 false-positives**: 10 names in TB_13_TYPE_NAMES — `EventId`, `OutcomeSide`, `ShareAmount` are the most common-feeling. Confirm they don't appear in legacy code outside FENCE_SCOPE_FLOOR.
- **R-022 skip-token**: Confirm `OBS_R022_TB13_RESOLUTIONREF_REMOVED_2026-05-03.md` is sufficient justification for the missing TRACE_MATRIX backlink (the symbol is gone; its role is absorbed by `CompleteSetRedeemTx.outcome`).

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
brief += "# Reference: Charter + Architect ruling + Recursive self-audit + Round-3 audits + Round-5 OBS\n"
brief += append_file("handover/tracer_bullets/TB-13_charter_2026-05-03.md", "markdown")
brief += append_file("handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md", "markdown")
brief += append_file("handover/audits/RECURSIVE_AUDIT_TB_13_2026-05-03.md", "markdown")
brief += append_file("handover/audits/GEMINI_TB_13_SHIP_AUDIT_2026-05-03_R3.md", "markdown")
brief += append_file("handover/alignment/OBS_R022_TB13_RESOLUTIONREF_REMOVED_2026-05-03.md", "markdown")
brief += append_file("handover/alignment/OBS_TB13_AUDIT_RESIDUAL_CHALLENGES_2026-05-03.md", "markdown")
brief += append_file("handover/ai-direct/TB-13_FIX_HANDOFF_2026-05-03.md", "markdown")
brief += append_file("handover/evidence/tb_13_chaintape_smoke_2026-05-03/README.md", "markdown")
brief += append_file("handover/evidence/tb_13_chaintape_smoke_2026-05-03/replay_report.json", "json")

# ── TB-13 source files under audit (post round-5) ──
brief += "\n\n---\n\n# TB-13 source code under audit (post round-5)\n"
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
    "src/runtime/verify.rs",
    "src/runtime/agent_keypairs.rs",
    "src/runtime/mod.rs",
    "tests/tb_13_complete_set.rs",
    "tests/tb_13_legacy_cpmm_forward_fence.rs",
    "tests/tb_13_chaintape_smoke.rs",
]:
    lang = "markdown" if rel.endswith(".md") else "rust"
    brief += append_file(rel, lang)

brief += "\n---\n\nGive your INDEPENDENT TB-13 round-4 ship audit. Be paranoid about Q1-Q9 + the round-5 risk surface (RQ3 / RQ6 / RQ5). Cite file:line for every finding.\n"

print(f"[gemini tb-13 r4] prompt size: {len(brief):,} chars", file=sys.stderr)

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
    print(f"[gemini tb-13 r4] API error: {e}", file=sys.stderr)
    sys.exit(1)
elapsed = time.time() - t0
print(f"[gemini tb-13 r4] API returned in {elapsed:.1f}s", file=sys.stderr)

# Extract text
try:
    text = data["candidates"][0]["content"]["parts"][0]["text"]
except (KeyError, IndexError) as e:
    print(f"[gemini tb-13 r4] response shape error: {e}\nfull response: {json.dumps(data, indent=2)[:2000]}", file=sys.stderr)
    sys.exit(1)

header = (
    f"# Gemini TB-13 Ship Audit — CompleteSet + MarketSeedTx (Class 3)\n"
    f"**Round**: {ROUND}\n"
    f"**Date**: 2026-05-03\n"
    f"**Test baseline**: cargo test --workspace = 791 PASS / 0 FAILED / 150 ignored\n"
    f"**Recursive self-audit**: PASS (5 clauses + 12 SG + 11 G + 0/7 halts)\n"
    f"**Round-5 closures**: RQ3 (chaintape replay smoke) + Q9/RQ6 (type-use fence) + RQ5 (ResolutionRef wrapper drop)\n"
    f"**Elapsed**: {elapsed:.1f}s\n"
    f"**Prompt size**: {len(brief):,} chars\n"
    f"**Audit mode**: Class 3 dual (Codex impl-paranoid + Gemini architectural strategic; per architect Part A §4.8)\n\n---\n\n"
)
OUT.write_text(header + text)
print(f"[gemini tb-13 r4] saved: {OUT}")
