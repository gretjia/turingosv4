#!/usr/bin/env python3
"""TB-13 round-6 Gemini ship audit (post round-7 closure of Codex R5 PARTIAL-MARKER + DASHBOARD-FLOOR)."""
import json
import os
import sys
import time
import urllib.request
from pathlib import Path

ROOT = Path("/home/zephryj/projects/turingosv4")
ENV_FILES = [ROOT / ".env"]
ROUND = "R6"
OUT = ROOT / f"handover/audits/GEMINI_TB_13_SHIP_AUDIT_2026-05-03_{ROUND}.md"

if OUT.exists():
    print(f"[gemini tb-13 r6] error: {OUT} already exists; refusing to overwrite", file=sys.stderr)
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
    print("[gemini tb-13 r6] GEMINI_API_KEY not in .env", file=sys.stderr)
    sys.exit(2)

# ── Brief ──
brief = """# Gemini TB-13 Ship Audit — round-6 (post round-7 Codex R5 closure)

**Role**: skeptical adversarial reviewer, architectural strategic angle. Independent of Codex (impl-paranoid). Conservative merge rule (memory `feedback_dual_audit_conflict`): VETO > CHALLENGE > PASS.

**Round context**: this is **round-6** in audit numbering (the project's session-internal "round-7" closure of Codex round-5's two NEW Q9/RQ6 challenges PARTIAL-MARKER + DASHBOARD-FLOOR). Round-5 verdicts:
- Codex round-5: CHALLENGE / high / FIX-THEN-PROCEED. 2 NEW Q9/RQ6 sub-challenges (PARTIAL-MARKER + DASHBOARD-FLOOR). Explicit "No VETO". RQ3 confirmed PASS at R5 (manual_replay_from_disk passes adversarial scrutiny).
- Gemini round-5 (you): PASS / high / PROCEED to SHIP. Both R4 fixes (Q9/RQ6 Layer 2 + RQ3 direct map equality) endorsed as airtight.

Per `feedback_dual_audit_conflict` Codex CHALLENGE wins; per `feedback_audit_obs_bias` both R5 challenges qualified for surgical fix → round-7 commit `8efffa8` addresses both.

**Mandate**: TB-13 of TURINGOS v4 implements per architect 2026-05-03 post-TB-12 ruling Part A §4: introduce conditional-share substrate "1 locked Coin = 1 YES_E + 1 NO_E". Three new typed-tx variants (CompleteSetMintTx, CompleteSetRedeemTx, MarketSeedTx) all agent-signed. EconomicState extended 11 → 13 sub-fields.

The architect's TB-13 forbidden list (Part A §4.7) explicitly bans: AMM / CPMM / orderbook / MarketOrderTx / MarketTradeTx / PriceIndex / DPMM / pro-rata / automatic liquidity / ghost liquidity / NodeMarketEntry as canonical state / f64 in money path / import of legacy `src/prediction_market.rs`.

## Round-7 closures (this session) — what to re-verify

Round-6 commit `d3473bb` (Q9/RQ6 Layer 2 + RQ3 direct map equality) addressed your R4 PASS-confirmed Codex R4 challenges. Codex R5 then surfaced two NEW sub-challenges in the round-6 fence-mechanism fix:

- **TB13-Q9/RQ6-PARTIAL-MARKER (R5)**: round-6's `tb_13_scan_lines()` short-circuited to marker-spans-only when ANY marker existed. A marker-bearing file could hide stealth TB-13 type-use + f64/AMM tokens outside marker spans. Codex flagged this as a real attack vector.
- **TB13-Q9-DASHBOARD-FLOOR (R5)**: round-6 removed `src/bin/audit_dashboard.rs` from `FENCE_SCOPE_FLOOR` to dodge a Layer 2 false-positive on its negative-list test fixture. But the false-positive was Layer 2-specific; removal also disabled Layer 1 hard-import scanning on that file.

Round-7 commit `8efffa8` (HEAD) closes both:

```text
8efffa8  TB-13 Atom 6 round-7 — Codex R5 remediation (PARTIAL-MARKER + DASHBOARD-FLOOR)
```

**PARTIAL-MARKER fix** (`tests/tb_13_legacy_cpmm_forward_fence.rs`): rewritten `tb_13_scan_lines()`. For marker-files: return marker-spans UNION non-comment lines containing TB-13 type names. Catches stealth type-use because any line referencing a TB-13-introduced type IS a TB-13 contribution by definition. Unmarked-discovered files keep round-6 behavior (all non-comment lines).

**DASHBOARD-FLOOR fix**: two-tier scope split.
- `effective_fence_scope()` (Layer 1) = FLOOR ∪ discovered. `audit_dashboard.rs` RESTORED to FLOOR.
- `effective_layer_2_scope()` (NEW) = discovered only. Excludes `audit_dashboard.rs` until it gains TB-13 contributions.
- `legacy_cpm_api_not_imported_by_complete_set` Layer 2 + `no_f64_in_complete_set_or_market_seed` now use Layer 2 scope.

Two new tests:
- `tb_13_scan_lines_partial_marker_catches_stealth_type_use`: marker-file with stealth `CompleteSetMintTx` + `f64` outside marker-spans → returned.
- `audit_dashboard_in_layer_1_scope_but_not_layer_2_scope`: asserts the tier-split shape.

Acknowledged residual gap (in code-comment): a TB-13 helper that uses zero TB-13 type names AND lives outside marker spans (e.g., a generic math helper called only by TB-13 code). Manual code review remains the fallback halt-trigger guard for that pattern.

`cargo test --workspace = 794 passed / 0 failed / 150 ignored` (round-6 baseline 792 + 2 new round-7 tests).

### Round-5 closures (carried into round-6)

**RQ5 — drop ResolutionRef wrapper**: ResolutionRef struct removed; CompleteSetRedeemTx 9→8 fields; redeem.outcome is now the sole claim source. Trust Root rehashed. R-022 skip token at `handover/alignment/OBS_R022_TB13_RESOLUTIONREF_REMOVED_2026-05-03.md`. (Your R4 PASS already confirmed.)

**Q9/RQ6 (R3)** — type-use forward-fence discovery added to `tests/tb_13_legacy_cpmm_forward_fence.rs`.

**RQ3 (R3)** — `tests/tb_13_chaintape_smoke.rs` boots Git2LedgerWriter-backed sequencer with pre-seeded initial_q, submits real signed CompleteSetMint + CompleteSetRedeem.

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

## Round-7-specific risks to scrutinize

- **PARTIAL-MARKER fix**: `tb_13_scan_lines()` for marker-files returns marker-spans UNION non-comment lines containing TB-13 type names. Verify: (a) is this the right closure for the attack Codex R5 raised? Specifically: a marker-bearing file that adds non-marker code which BOTH uses a TB-13 type AND has `f64` on the same line — caught (line returned, both spans-pass and type-use-pass agree). (b) But what if `f64` is on a DIFFERENT non-marker line that has NO TB-13 type name on it? My read of the round-7 fix: that line is NOT returned (no marker, no type-name match). Codex would still find this attack via static review. Is this acknowledged residual gap acceptable, or does it deserve a stronger fix (e.g., scan all non-comment lines of marker-files in some restricted way)?
- **DASHBOARD-FLOOR two-tier scope**: Layer 1 stays broad (FLOOR ∪ discovered) including `audit_dashboard.rs`; Layer 2 narrows to discovered-only. Verify: (a) does this preserve Layer 1's hard-banned-import enforcement on `audit_dashboard.rs`? (b) Could a contributor sneak f64 / AMM into `audit_dashboard.rs` without it being scanned by Layer 2 (because Layer 2 only sees discovered files)? — yes, until `audit_dashboard.rs` gains TB-13 markers or type uses, Layer 2 won't scan it. This is the explicit tradeoff: false-negative on `audit_dashboard.rs` Layer 2 today vs. false-positive on its negative-list test fixture (`"price_yes"`/`"price_no"` strings at line 1628-1629). Is the tradeoff right? When `audit_dashboard.rs` does start contributing TB-13 (planned for TB-14), it should auto-enter Layer 2 via marker or type-use discovery.
- **Round-7 unit-test coverage**: new tests `tb_13_scan_lines_partial_marker_catches_stealth_type_use` + `audit_dashboard_in_layer_1_scope_but_not_layer_2_scope` exercise the new behavior. Are they sufficient, or are there other adversarial patterns worth covering?

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
brief += append_file("handover/audits/GEMINI_TB_13_SHIP_AUDIT_2026-05-03_R4.md", "markdown")
brief += append_file("handover/audits/GEMINI_TB_13_SHIP_AUDIT_2026-05-03_R5.md", "markdown")
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

print(f"[gemini tb-13 r6] prompt size: {len(brief):,} chars", file=sys.stderr)

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
    print(f"[gemini tb-13 r6] API error: {e}", file=sys.stderr)
    sys.exit(1)
elapsed = time.time() - t0
print(f"[gemini tb-13 r6] API returned in {elapsed:.1f}s", file=sys.stderr)

# Extract text
try:
    text = data["candidates"][0]["content"]["parts"][0]["text"]
except (KeyError, IndexError) as e:
    print(f"[gemini tb-13 r6] response shape error: {e}\nfull response: {json.dumps(data, indent=2)[:2000]}", file=sys.stderr)
    sys.exit(1)

header = (
    f"# Gemini TB-13 Ship Audit — CompleteSet + MarketSeedTx (Class 3)\n"
    f"**Round**: {ROUND}\n"
    f"**Date**: 2026-05-03\n"
    f"**Test baseline**: cargo test --workspace = 794 PASS / 0 FAILED / 150 ignored\n"
    f"**Recursive self-audit**: PASS (5 clauses + 12 SG + 11 G + 0/7 halts)\n"
    f"**Round-5 closures**: RQ3 + Q9/RQ6 + RQ5 (R3 challenges).\n"
    f"**Round-6 closures**: Q9/RQ6 Layer 2 + RQ3 direct map equality (R4 challenges).\n"
    f"**Round-7 closures**: PARTIAL-MARKER + DASHBOARD-FLOOR (R5 challenges).\n"
    f"**Elapsed**: {elapsed:.1f}s\n"
    f"**Prompt size**: {len(brief):,} chars\n"
    f"**Audit mode**: Class 3 dual (Codex impl-paranoid + Gemini architectural strategic; per architect Part A §4.8)\n\n---\n\n"
)
OUT.write_text(header + text)
print(f"[gemini tb-13 r6] saved: {OUT}")
