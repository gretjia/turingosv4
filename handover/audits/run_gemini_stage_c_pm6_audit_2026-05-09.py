#!/usr/bin/env python3
"""Gemini Stage C P-M6 (BuyWithCoinRouter rebuild) PRE-§8 audit.

Per `feedback_dual_audit` Class-4 timing rule (added 2026-05-09 from Stage C
session #27 batch §8 VETO lesson): dispatch Codex G2 + Gemini at PACKET DRAFT
time, not after architect §8 request. Conservative-wins per
`feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.

This is the Gemini half of the Phase F.5 P-M6 PRE-§8 dual audit; Codex side
dispatches in parallel via Claude Code's Agent tool with subagent_type
codex:codex-rescue, OR via direct Bash invocation of `codex exec` per
`feedback_codex_bash_exec_direct_dispatch` if Skill rejected.

Round cap is 2 per `feedback_elon_mode_policy`. R3 needs explicit user
authorization.
"""
import json
import os
import re
import sys
import time
import urllib.request
from pathlib import Path

ROOT = Path("/home/zephryj/projects/turingosv4")
ENV_FILES = [
    Path("/home/zephryj/projects/turingosv3/.env"),
    ROOT / ".env",
]
ROUND = os.environ.get("TB_AUDIT_ROUND", "R1")
OUT = ROOT / f"handover/audits/GEMINI_STAGE_C_PM6_AUDIT_2026-05-09_{ROUND}.md"

if ROUND not in {"R1", "R2"}:
    print("[gemini C P-M6] error: TB_AUDIT_ROUND must be R1 or R2", file=sys.stderr)
    sys.exit(2)
if OUT.exists():
    print(f"[gemini C P-M6] error: {OUT} already exists; refusing to overwrite", file=sys.stderr)
    sys.exit(2)


def load_env() -> dict:
    env = {}
    for fp in ENV_FILES:
        if not fp.exists():
            continue
        for line in fp.read_text().splitlines():
            if "=" in line and not line.strip().startswith("#"):
                key, value = line.split("=", 1)
                env.setdefault(key.strip(), value.strip().strip('"').strip("'"))
    return env


def append_file(rel: str, lang: str = "") -> str:
    fp = ROOT / rel
    if not fp.exists():
        return f"\n\n---\n\n## {rel}\n\n(MISSING: expected file not found)\n"
    return f"\n\n---\n\n## {rel}\n\n```{lang}\n{fp.read_text()}\n```\n"


def extract_verdict(text: str) -> str:
    patterns = [
        r"(?im)^##\s*VERDICT:\s*(PASS|CHALLENGE|VETO)\b",
        r"(?im)^VERDICT:\s*(PASS|CHALLENGE|VETO)\b",
        r"(?im)^Final aggregate verdict:\s*(PASS|CHALLENGE|VETO)\b",
    ]
    for p in patterns:
        m = re.search(p, text)
        if m:
            return m.group(1).upper()
    for tok in ("VETO", "CHALLENGE", "PASS"):
        if re.search(rf"\b{tok}\b", text):
            return tok
    return "UNKNOWN"


env = load_env()
if "GEMINI_API_KEY" not in env:
    print("[gemini C P-M6] GEMINI_API_KEY not found in .env files", file=sys.stderr)
    sys.exit(2)

brief = f"""# Gemini Stage C P-M6 (BuyWithCoinRouter rebuild) PRE-§8 Audit — {ROUND}

You are Gemini 2.5 Pro acting as a skeptical architectural reviewer for
TuringOS v4 Stage C Polymarket Phase F.5 P-M6 rebuild. This is the Gemini
half of the PRE-§8 dual-audit per `feedback_dual_audit` Class-4 timing rule.
Codex G2 implementation-paranoid half dispatches in parallel.

## Context

Stage C Polymarket session #27 was VETOED + fully rolled back (HEAD `01dd825`,
2026-05-09). The VETO directive cited 4 defects in the batch §8 packet, two
of which targeted P-M6 specifically:
- Defect 1 (P-M6, load-bearing): `monetary_invariant.rs` accepted `min(sum_yes,
  sum_no) == collateral` instead of strict `sum_yes == collateral && sum_no ==
  collateral` — weakens CLAUDE.md §13 economy law.
- Defect 2 (P-M6, load-bearing): `router_atomic_rollback_on_failure` test
  triggered insufficient-buyer-balance failure that was rejected before
  `q_next` mutation began → vacuous; no tape evidence of 9-step composite
  atomicity.

Phases F.1 (P-M2 SHIPPED FINAL session #29) + F.2 (P-M3 SHIPPED session #30) +
F.3 (P-M4 SHIPPED FINAL session #31) + F.4 (P-M5 SHIPPED session #32) all
landed; this is the FIFTH per-atom Class-4 §8 cycle (only Class-4 atom in
Phase F outside P-M2 + P-M4; per remediation directive §1.C row 5).

This audit is PRE-§8: verdict cycles in working tree (rollback free); only
after both Codex AND Gemini PASS does the §8 packet ascend to architect.

Conservative merge ranking applies per `feedback_dual_audit_conflict`:
VETO > CHALLENGE > PASS. Prefer CHALLENGE over PASS when a concrete
architectural uncertainty has an actionable remediation. Use VETO only for
ship-blocking inconsistency with the Constitution, charter, or architect-pinned
schemas.

Round cap is 2 per `feedback_elon_mode_policy`. This run is {ROUND}.

## Audit target

Phase F.5 P-M6 substrate at HEAD `0e0df18` (local branch `feat/p-m6-rebuild`;
NOT pushed to origin). Commit lineage on top of `3f72383` (origin/main
post-P-M5):
- `b03df48` P-M6 SHIPPED — BuyWithCoinRouter Class-4 STEP_B rebuild (Phase F.5)
- `0e0df18` P-M6 §8 packet — Phase F.5 ship request (CANDIDATE)

Validation at HEAD `0e0df18`:
- Constitution gates: 223 PASS / 0 failed / 1 ignored (was 213 pre-F.5; +10)
- Workspace tests: 1356 PASS / 0 failed / 151 ignored (was 1346 pre-F.5; +10)
- Trust Root: cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo PASS
- 6 trust_root files rehashed: typed_tx.rs / sequencer.rs / transition_ledger.rs / verify.rs / run_summary.rs / monetary_invariant.rs

## Required questions

Answer Q1 through Q9 explicitly. Mark each as PASS / CHALLENGE / VETO.

Q1. **Architect §7.7 verbatim compliance — 9-step composite**: Verify the
    BuyWithCoinRouter admission arm at `src/state/sequencer.rs::dispatch_
    transition::TypedTx::BuyWithCoinRouter` implements the 9-step composite
    from architect manual §7.7 lines 871-883 verbatim. In particular: each
    architect-numbered step (1-9) should have a corresponding code block;
    BuyDirection::BuyYes should map "Debit buyer Coin / Lock collateral /
    synthetic mint / transfer YES / swap into pool / receive outY / transfer
    outY YES / cumulative getY"; BuyNo should be the symmetric mirror.
    Are any architect-numbered steps missing OR are extra non-architect
    steps introduced?

Q2. **Architect §7.7 integer formula correctness**: Verify the formula
    `outY = floor(payC * pool_yes / (pool_no + payC))` (BuyYes) +
    `outN = floor(payC * pool_no / (pool_yes + payC))` (BuyNo) is implemented
    via u128 checked_mul + checked_add + integer division `numer / denom`.
    The architect §7.7 invariant `pool_yes1 * pool_no1 >= pool_yes * pool_no`
    (`>=` because integer floor leaves dust in pool) should hold for all
    accepted router txs. No f64/f32 anywhere in the admission arm. Cross-
    check with the source-grep `buy_yes_no_f64` test.

Q3. **Defect-1 patch (STRICT equality monetary invariant)**: Verify that
    `assert_complete_set_balanced` in `src/economy/monetary_invariant.rs`
    enforces strict `sum_yes == sum_no == collateral` on the symmetric
    branch (post-Phase E.3 split). The router accept arm calls
    `assert_complete_set_balanced(&q_next.economic_state_t)` and
    `assert_total_ctf_conserved` with empty exempt-list — both must pass for
    a healthy router post-state. Verify test `buy_yes_mints_complete_set`
    + `buy_yes_no_ghost_liquidity` directly assert this strict property.
    Is the symmetric branch reachable from the router, or could a router
    leave the post-state on the asymmetric branch?

Q4. **Defect-2 patch (mid-mutation atomic-rollback witness)**: Verify the
    cfg(debug_assertions) failure-injection hook
    `check_router_test_failure_injection(step)` in
    `src/state/sequencer.rs` reads `TURINGOS_TEST_ROUTER_FAIL_AT_STEP` env
    var; the hook is called BEFORE EACH of the 9 architect-numbered steps
    so that injection at step N causes failure AT step N (post step N-1
    mutations, pre step N mutation). Verify cfg(not(debug_assertions))
    branch is a no-op (#[inline(always)]) so production --release builds
    cannot have router admission influenced by env var (replay determinism).
    Verify test `router_atomic_rollback_on_failure` AT
    `tests/constitution_router_buy_with_coin.rs` injects step 5 failure
    AND asserts ALL of: state_root unchanged, buyer Coin balance unchanged,
    collateral unchanged, pool reserves unchanged, buyer's step-4 YES gain
    reverted. Verify defense-in-depth test
    `router_atomic_rollback_witnessed_at_every_step` exhaustively injects
    at steps 1..=9 and asserts state_root unchanged each time.
    Is the cfg gate appropriate? Are the assertions exhaustive enough?

Q5. **Tx schema (implementation-defined)**: Architect §7.7 specifies the
    9-step composite + 9 mandated tests + integer formulas, NOT a tx schema.
    The implementation chose 8-field shape:
    `{{tx_id, parent_state_root, event_id, buyer, direction, pay_coin,
       min_out_shares, signature}}`. Verify NO `timestamp_logical` (defect
    3 prevention; matches CpmmPool / CpmmSwap minimal pattern). Verify
    `event_id` (NOT `event_id_kind`; defect 4 prevention). Cross-check by
    reading the BINDINGS array in
    `tests/constitution_architect_verbatim_struct_binding.rs` — both P-M6
    entries (wire + signing) should be `LandingStatus::Landed`. Is the
    schema choice defendable under strict-spec scrutiny?

Q6. **F-DEFERRAL-2 closure (sibling SigningPayload binding)**: Per
    remediation directive §9, F-DEFERRAL-2 requires extending E.1 BINDINGS
    with a sibling SigningPayload entry. Verify
    `BuyWithCoinRouterSigningPayload` 7-field projection (8 wire fields
    minus signature) is bound at `LandingStatus::Landed`. Verify
    `to_signing_payload` projection in `src/state/typed_tx.rs` matches the
    binding fields. Confirm domain-prefixed digest uses
    `b"turingosv4.agent_sig.buy_with_coin_router.v1"`.

Q7. **CTF + total-Coin conservation**: Coin moves: `balances_t -= pay_coin`
    (step 1) AND `conditional_collateral_t += pay_coin` (step 2). Both
    balances and conditional_collateral are in the 6-holding
    `total_supply_micro` sum. Verify `assert_total_ctf_conserved` with
    empty exempt-list passes across the router tx (Coin neither minted
    nor burned; symmetric movement). Verify `assert_no_post_init_mint`
    has `TypedTx::BuyWithCoinRouter` in allow-list. Edge cases to check:
    pay_coin == 0 (rejected pre-mutation by `RouterZeroPay`), pay_coin
    overflow (checked_sub / checked_add at steps 1/2 trap with
    MonetaryInvariantViolation), pool drained (out_shares == pool_other —
    impossible for positive pay_coin per arithmetic, but verify
    boundary).

Q8. **Replay-determinism**: Replay must reconstruct identically from
    genesis_report + ChainTape + CAS + agent_registry. Verify:
    - `buy_with_coin_router_accept_state_root` is deterministic
      (sha256 of canonical_encode(tx) under domain prefix
      `b"turingosv4.buy_with_coin_router.accept.v1"`).
    - `BuyWithCoinRouter` variant added to TxKind enum at position 17;
      L4 LedgerEntry replay reads back the same payload via
      `tx_payload_cid` round-trip.
    - 6 trust_root rehashes ensure
      `verify_trust_root_passes_on_intact_repo` succeeds.
    - cfg(debug_assertions) failure-injection hook does NOT influence
      production --release replay (no env var read in release builds).

Q9. **Strategic risk**: What, if anything, in Phase F.5 P-M6 substrate is
    visibly wrong or missing that future Phase F.6 (P-M7 PriceIndex) or
    Phase F.7 (P-M8 audit views) or Phase F.8 (P-M9 controlled market
    smoke) would expose? In particular consider: how does the router
    interact with future PriceIndex queries (architect §7.8 effective
    price = quote_payC / quote_getY)? Does the router introduce any
    precondition that audit views (architect §7.10) would have to reason
    about? Are there subtle invariant breaks that pass today's narrow
    tests but would surface under real-LLM Polymarket smoke? In particular:
    - Pool drain attack: can a sufficiently large payC drain the pool
      output side completely? (out_shares > 0 check protects but is
      `out >= pool_other` reachable?)
    - Slippage griefing: can a third party manipulate pool ratio between
      quote and submission to force `RouterSlippageExceeded`?
    - Dust accumulation: floor rounding leaves dust in pool — does this
      drift the pool's k upward in a way that distorts later swaps?

## Verdict format

Use exactly this shape:

```
Q1: PASS|CHALLENGE|VETO - <reason>
Q2: PASS|CHALLENGE|VETO - <reason>
Q3: PASS|CHALLENGE|VETO - <reason>
Q4: PASS|CHALLENGE|VETO - <reason>
Q5: PASS|CHALLENGE|VETO - <reason>
Q6: PASS|CHALLENGE|VETO - <reason>
Q7: PASS|CHALLENGE|VETO - <reason>
Q8: PASS|CHALLENGE|VETO - <reason>
Q9: PASS|CHALLENGE|VETO - <reason>

## VERDICT: PASS|CHALLENGE|VETO
Conviction: low|medium|high
Recommendation: PROCEED|FIX-THEN-PROCEED|REDESIGN
Remediations:
- <only for CHALLENGE/VETO; actionable and scoped>
```

If any Q is CHALLENGE, aggregate verdict must be CHALLENGE unless another Q is
VETO. If any Q is VETO, aggregate verdict must be VETO.

---

# Ground Truth Excerpts
"""

for rel, lang in [
    ("handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md", "markdown"),
    ("handover/directives/2026-05-09_STAGE_C_POLYMARKET_VETO_REMEDIATION_DIRECTIVE.md", "markdown"),
    ("handover/directives/2026-05-09_STAGE_C_POLYMARKET_PM6_§8_PACKET.md", "markdown"),
    ("tests/constitution_router_buy_with_coin.rs", "rust"),
    ("tests/constitution_architect_verbatim_struct_binding.rs", "rust"),
    ("tests/constitution_class4_atomic_rollback_witness.rs", "rust"),
    ("CLAUDE.md", "markdown"),
]:
    brief += append_file(rel, lang)

print(f"[gemini C P-M6] prompt size: {len(brief):,} chars", file=sys.stderr)

url = (
    "https://generativelanguage.googleapis.com/v1beta/models/"
    f"gemini-2.5-pro:generateContent?key={env['GEMINI_API_KEY']}"
)
body = json.dumps(
    {
        "contents": [{"parts": [{"text": brief}]}],
        "generationConfig": {"temperature": 0.2, "maxOutputTokens": 16384},
    }
).encode()
req = urllib.request.Request(
    url,
    data=body,
    headers={"Content-Type": "application/json"},
    method="POST",
)

t0 = time.time()
try:
    with urllib.request.urlopen(req, timeout=900) as resp:
        data = json.loads(resp.read())
except Exception as exc:
    print(f"[gemini C P-M6] error: {exc}", file=sys.stderr)
    sys.exit(1)

elapsed = time.time() - t0
print(f"[gemini C P-M6] API returned in {elapsed:.1f}s", file=sys.stderr)

try:
    text = data["candidates"][0]["content"]["parts"][0]["text"]
except (KeyError, IndexError) as exc:
    print(f"[gemini C P-M6] malformed API response: {exc}", file=sys.stderr)
    print(json.dumps(data, indent=2)[:4000], file=sys.stderr)
    sys.exit(1)

verdict = extract_verdict(text)
header = f"""# Gemini Stage C P-M6 (BuyWithCoinRouter rebuild) PRE-§8 Audit — {ROUND}

**Round**: {ROUND}
**Date**: 2026-05-09
**Model**: gemini-2.5-pro
**Elapsed**: {elapsed:.1f}s
**Prompt size**: {len(brief):,} chars
**HEAD**: 0e0df18
**Final aggregate verdict**: {verdict}

---

## Verbatim Gemini Response

"""

OUT.write_text(header + text)
print(f"[gemini C P-M6] saved: {OUT}")
print(f"[gemini C P-M6] verdict: {verdict}")
