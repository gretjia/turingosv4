#!/usr/bin/env python3
"""TB-13 round-5 Gemini ship audit (post round-6 closure of Codex R4 Q9/RQ6 + RQ3)."""
import json
import os
import sys
import time
import urllib.request
from pathlib import Path

ROOT = Path("/home/zephryj/projects/turingosv4")
ENV_FILES = [ROOT / ".env"]
ROUND = "R5"
OUT = ROOT / f"handover/audits/GEMINI_TB_13_SHIP_AUDIT_2026-05-03_{ROUND}.md"

if OUT.exists():
    print(f"[gemini tb-13 r5] error: {OUT} already exists; refusing to overwrite", file=sys.stderr)
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
    print("[gemini tb-13 r5] GEMINI_API_KEY not in .env", file=sys.stderr)
    sys.exit(2)

# ── Brief ──
brief = """# Gemini TB-13 Ship Audit — round-5 (post round-6 Codex R4 closure)

**Role**: skeptical adversarial reviewer, architectural strategic angle. Independent of Codex (impl-paranoid). Conservative merge rule (memory `feedback_dual_audit_conflict`): VETO > CHALLENGE > PASS.

**Round context**: this is **round-5** in audit numbering (the project's session-internal "round-6" closure of Codex round-4's two NEW challenges Q9/RQ6 Layer-2 + RQ3 overclaim). Round-4 verdicts:
- Codex round-4: CHALLENGE / high / FIX-THEN-PROCEED. 2 NEW challenges (TB13-Q9/RQ6 Layer 2 marker-scoping + TB13-RQ3 state-root overclaim). Explicit "No VETO".
- Gemini round-4 (you): PASS / high / PROCEED to SHIP. Q12 ResolutionsIndex challenge fully resolved by RQ5 ResolutionRef removal.

Per `feedback_dual_audit_conflict` Codex CHALLENGE wins; per `feedback_audit_obs_bias` both R4 challenges qualified for surgical fix (not OBS) → round-6 commit `d3473bb` addresses both.

**Mandate**: TB-13 of TURINGOS v4 implements per architect 2026-05-03 post-TB-12 ruling Part A §4: introduce conditional-share substrate "1 locked Coin = 1 YES_E + 1 NO_E". Three new typed-tx variants (CompleteSetMintTx, CompleteSetRedeemTx, MarketSeedTx) all agent-signed. EconomicState extended 11 → 13 sub-fields.

The architect's TB-13 forbidden list (Part A §4.7) explicitly bans: AMM / CPMM / orderbook / MarketOrderTx / MarketTradeTx / PriceIndex / DPMM / pro-rata / automatic liquidity / ghost liquidity / NodeMarketEntry as canonical state / f64 in money path / import of legacy `src/prediction_market.rs`.

## Round-6 closures (this session) — what to re-verify

Round-5 commits (`edbc555` RQ5, `a4f8265` Q9/RQ6, `ee8bfe8` RQ3) closed the three round-3 challenges.

Round-4 audit (Codex) found two NEW substantive defects in the round-5 fixes themselves:
- **TB13-Q9/RQ6 (R4)**: type-use discovery added unmarked TB-13 files to scope, but Layer 2 (FORBIDDEN_LEGACY_TOKENS + f64) still walked marker-scoped `tb_13_spans()`. An unmarked file importing `CompleteSetMintTx` and using `f64` would be discovered into scope but never scanned by Layer 2.
- **TB13-RQ3 (R4)**: round-5 README + test claimed `final_state_root_hex == hex(live state_root_t)` was "cryptographic proof of map equality." But state-root mutators hash `domain || prev_root || canonical_tx`, NOT the full QState. State-root match proves chain determinism; map equality follows by dispatch-purity inference, not directly.

Round-6 commit `d3473bb` (HEAD) closes both:

```text
887537f  TB-13 Atom 6 round-5 audit artifacts (R3 rename + R4 audit run)
d3473bb  TB-13 Atom 6 round-6 — Codex R4 remediation (Q9/RQ6 Layer 2 + RQ3 direct map equality)
```

**Q9/RQ6 fix** (`tests/tb_13_legacy_cpmm_forward_fence.rs`): new `tb_13_scan_lines()` helper. Marker-files → return spans (preserves doc-xref skip); unmarked-discovered files → return all non-comment lines. Wired into both `legacy_cpm_api_not_imported_by_complete_set` Layer 2 and `no_f64_in_complete_set_or_market_seed`. Side-effect: removed `src/bin/audit_dashboard.rs` from `FENCE_SCOPE_FLOOR` (0 TB-13 markers + 0 type uses; would false-positive on negative-list test fixture). New unit test `tb_13_scan_lines_handles_marker_and_unmarked_files` covers both branches.

**RQ3 fix** (`tests/tb_13_chaintape_smoke.rs`): added `manual_replay_from_disk()` helper that opens Git2LedgerWriter + loads `initial_q_state.json` + decodes `pinned_pubkeys.json` + opens CasStore + calls `replay_full_transition` (pub API). The smoke now asserts `replayed_q.economic_state_t.conditional_collateral_t == live` (byte-equal), `... .conditional_share_balances_t == live` (byte-equal), and full `economic_state_t == live` — direct map-equality evidence with no inference from dispatch-determinism. README + module docstring revised to claim what's actually proven.

`cargo test --workspace = 792 passed / 0 failed / 150 ignored` (round-5 baseline 791 + 1 new `tb_13_scan_lines_handles_marker_and_unmarked_files`).

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

## Round-6-specific risks to scrutinize

- **Q9/RQ6 (R4) Layer 2 fix**: `tb_13_scan_lines()` returns marker-spans for marker-files (preserves doc-xref skip) and ALL non-comment lines for unmarked-but-discovered files. Verify the helper logic at `tests/tb_13_legacy_cpmm_forward_fence.rs:165..183`: is the conditional `source.lines().any(is_tb_13_authoring_marker)` the right test, or could a partially-marked file (e.g., one with a single TB-13 marker but TB-12 contributions in non-marker spans) get incorrectly classified? Also: is removing `src/bin/audit_dashboard.rs` from FENCE_SCOPE_FLOOR safe? It currently has 0 TB-13 markers + 0 type uses, so the auto-discovery walk gives it no scope; if Atom 4 dashboard ships before TB-14 with TB-13 contributions but without TB-13 markers, the file would re-enter scope only via type-use discovery. Acceptable?
- **RQ3 (R4) direct map-equality fix**: `manual_replay_from_disk()` opens `Git2LedgerWriter` + loads `initial_q_state.json` + decodes pinned pubkeys + opens CasStore + calls `replay_full_transition` (pub). Then asserts byte-equal `economic_state_t` against live. Is this evidence airtight? Specifically: (a) does `replay_full_transition`'s reconstructed QState match the LIVE QState in all sub-fields, not just TB-13 ones; (b) is there any subtle non-determinism (BTreeMap ordering, MicroCoin internals, hashmap iteration if any) that could make replay differ from live; (c) is the manual replay a faithful mirror of `verify_chaintape`'s internal step 6 — i.e., does the same input produce the same output?
- **R-022 skip-token continuity**: `OBS_R022_TB13_RESOLUTIONREF_REMOVED_2026-05-03.md` justifies the round-5 ResolutionRef removal. Round-6 doesn't introduce new pub-symbol removals. Confirm no new R-022 surface.

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

print(f"[gemini tb-13 r5] prompt size: {len(brief):,} chars", file=sys.stderr)

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
    print(f"[gemini tb-13 r5] API error: {e}", file=sys.stderr)
    sys.exit(1)
elapsed = time.time() - t0
print(f"[gemini tb-13 r5] API returned in {elapsed:.1f}s", file=sys.stderr)

# Extract text
try:
    text = data["candidates"][0]["content"]["parts"][0]["text"]
except (KeyError, IndexError) as e:
    print(f"[gemini tb-13 r5] response shape error: {e}\nfull response: {json.dumps(data, indent=2)[:2000]}", file=sys.stderr)
    sys.exit(1)

header = (
    f"# Gemini TB-13 Ship Audit — CompleteSet + MarketSeedTx (Class 3)\n"
    f"**Round**: {ROUND}\n"
    f"**Date**: 2026-05-03\n"
    f"**Test baseline**: cargo test --workspace = 792 PASS / 0 FAILED / 150 ignored\n"
    f"**Recursive self-audit**: PASS (5 clauses + 12 SG + 11 G + 0/7 halts)\n"
    f"**Round-5 closures**: RQ3 + Q9/RQ6 + RQ5 (R3 challenges).\n"
    f"**Round-6 closures**: Q9/RQ6 Layer 2 + RQ3 direct map equality (R4 challenges).\n"
    f"**Elapsed**: {elapsed:.1f}s\n"
    f"**Prompt size**: {len(brief):,} chars\n"
    f"**Audit mode**: Class 3 dual (Codex impl-paranoid + Gemini architectural strategic; per architect Part A §4.8)\n\n---\n\n"
)
OUT.write_text(header + text)
print(f"[gemini tb-13 r5] saved: {OUT}")
