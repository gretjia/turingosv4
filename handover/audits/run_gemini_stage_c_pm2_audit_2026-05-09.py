#!/usr/bin/env python3
"""Gemini Stage C P-M2 (CompleteSetMergeTx rebuild) PRE-§8 audit.

Per `feedback_dual_audit` Class-4 timing rule (added 2026-05-09 from Stage C
session #27 batch §8 VETO lesson): dispatch Codex G2 + Gemini at PACKET DRAFT
time, not after architect §8 request. Conservative-wins per
`feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.

This is the Gemini half of the Phase F.1 P-M2 PRE-§8 dual audit; Codex side
dispatches in parallel via the Agent tool (subagent_type codex:codex-rescue).

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
OUT = ROOT / f"handover/audits/GEMINI_STAGE_C_PM2_AUDIT_2026-05-09_{ROUND}.md"

if ROUND not in {"R1", "R2"}:
    print("[gemini C P-M2] error: TB_AUDIT_ROUND must be R1 or R2", file=sys.stderr)
    sys.exit(2)
if OUT.exists():
    print(f"[gemini C P-M2] error: {OUT} already exists; refusing to overwrite", file=sys.stderr)
    sys.exit(2)


def load_env() -> dict[str, str]:
    env: dict[str, str] = {}
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
    print("[gemini C P-M2] GEMINI_API_KEY not found in .env files", file=sys.stderr)
    sys.exit(2)

brief = f"""# Gemini Stage C P-M2 (CompleteSetMergeTx rebuild) PRE-§8 Audit — {ROUND}

You are Gemini 2.5 Pro acting as a skeptical architectural reviewer for
TuringOS v4 Stage C Polymarket Phase F.1 P-M2 rebuild. This is the Gemini
half of the PRE-§8 dual-audit per `feedback_dual_audit` Class-4 timing rule.
Codex G2 implementation-paranoid half dispatches in parallel via Claude
Code's Agent tool with subagent_type codex:codex-rescue.

## Context

Stage C Polymarket session #27 was VETOED + fully rolled back (HEAD `01dd825`,
2026-05-09). The VETO directive cited 4 defects in the batch §8 packet:
- Defect 1 (P-M6, load-bearing): `monetary_invariant.rs` accepted `min(sum_yes,
  sum_no) == collateral` instead of strict `sum_yes == collateral && sum_no ==
  collateral` — weakens CLAUDE.md §13 economy law.
- Defect 2 (P-M6, load-bearing): `router_atomic_rollback_on_failure` test
  triggered insufficient-balance failure that was rejected before `q_next`
  mutation began → vacuous; no tape evidence of 9-step composite atomicity.
- Defect 3 (P-M2): added `timestamp_logical` field; architect §7.3 verbatim
  specifies 6 fields only.
- Defect 4 (P-M4): used `event_id_kind` where architect §7.5 verbatim specifies
  `event_id`.

Stage C VETO Phase E (mechanism additions, 2026-05-09 session #28) shipped 3
gates designed to mechanically catch each defect class. Phase F.1 P-M2 rebuild
is the FIRST per-atom Class-4 §8 cycle following the new dual-audit-PRE-§8
timing rule.

This audit is PRE-§8: verdict cycles in working tree (rollback free); only
after both Codex AND Gemini PASS does the §8 packet ascend to architect.

Conservative merge ranking applies per `feedback_dual_audit_conflict`:
VETO > CHALLENGE > PASS. Prefer CHALLENGE over PASS when a concrete
architectural uncertainty has an actionable remediation. Use VETO only for
ship-blocking inconsistency with the Constitution, charter, or architect-pinned
schemas.

Round cap is 2 per `feedback_elon_mode_policy`. This run is {ROUND}.

## Audit target

Phase F.1 P-M2 substrate at HEAD `66f4e34` (local main; not pushed to origin):

Commit lineage on top of `ff2d401` (origin/main pre-Phase-F.1):
- `9d9a33c` P-M2 SHIPPED — CompleteSetMergeTx 6-field verbatim per architect §7.3
- `57a5b07` rules/enforcement.log — append R-022-PASS trail for P-M2 pub symbols
- `7af0db1` Merge branch 'feat/p-m2-rebuild' — P-M2 STEP_B → main
- `66f4e34` P-M2 §8 packet — Phase F.1 ship request

Validation at HEAD `66f4e34`:
- Constitution gates: 198 PASS / 0 failed / 1 ignored (was 193 pre-F.1; +5)
- Workspace tests: 1331 PASS / 0 failed / 151 ignored (was 1326 pre-F.1; +5)
- Trust Root: cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo PASS
- 6 trust_root files rehashed: typed_tx.rs / sequencer.rs / transition_ledger.rs / monetary_invariant.rs / verify.rs / run_summary.rs

## Required questions

Answer Q1 through Q8 explicitly. Mark each as PASS / CHALLENGE / VETO.

Q1. **Verbatim alignment (defect 3 prevention)**: Verify `CompleteSetMergeTx`
    at `src/state/typed_tx.rs` exactly matches architect manual §7.3 6-field
    spec — `{{tx_id, parent_state_root, event_id, owner, amount, signature}}`,
    NO `timestamp_logical`. Verify `CompleteSetMergeSigningPayload` is the
    5-field projection (signature excluded). Cross-check by reading the
    BINDINGS array in `tests/constitution_architect_verbatim_struct_binding.rs`
    — both P-M2 entries (wire + signing) should be `LandingStatus::Landed`.

Q2. **Test body realism**: Each of the 5 architect-mandated test names in
    `tests/constitution_completeset_merge.rs` (merge_yes_no_returns_coin,
    merge_requires_both_sides, merge_conserves_total_coin,
    merge_reduces_collateral, merge_unavailable_after_final_redeem_if_shares_exhausted)
    should exercise the live sequencer accept arm, NOT vacuously pass against
    a fixture-forged Q state. Verify each test reaches `submit_and_apply` →
    `dispatch_transition` → `q_next` mutation. Specifically: do the negative
    tests (requires_both_sides, unavailable_after_final_redeem) FAIL through
    the actual rejection path (TransitionError::InsufficientSharesForMerge),
    not through fixture trickery?

Q3. **Sequencer admission completeness**: The accept arm at
    `src/state/sequencer.rs` (TypedTx::CompleteSetMerge) must implement the
    architect §7.3 verbatim semantics block. Verify each of the 6 lines
    ("require owner YES >= amount" / "require owner NO >= amount" /
    "burn amount YES" / "burn amount NO" / "conditional_collateral_t[event] -=
    amount" / "balances_t[owner] += amount Coin") has a corresponding
    operation in the accept arm. Are any architect clauses missing or
    extra-clauses introduced?

Q4. **Architect §7.3 has NO event-state gate** (unlike CompleteSetMint which
    requires task_markets_t[event].state == Open). Is the Phase F.1
    implementation correct in NOT adding a state gate, or should it have one
    for safety? Specifically, the test
    `merge_unavailable_after_final_redeem_if_shares_exhausted` exercises a
    Finalized-state event with both YES + NO present briefly, then redeems
    YES, then attempts merge — and merge fails on share-balance grounds,
    not state-grounds. Does this honor the architect verbatim spec, or
    does it create an attack vector (e.g., merge after resolution leaks
    collateral asymmetrically)?

Q5. **CTF conservation**: `assert_complete_set_balanced` is called in the
    accept arm. Does the merge accept arm correctly maintain the 6-holding
    CTF total invariant `sum_balances + sum_collateral + sum_other_holdings
    == total_supply_micro`? In particular, the 1 share-unit = 1 micro-Coin
    equivalence is established at CompleteSetMint time; merge must reverse
    it bit-for-bit. Are there any edge cases (zero amount, partial merge
    after MarketSeed double-credits, collateral underflow) where conservation
    could be violated?

Q6. **F-DEFERRAL-2 closure**: Per remediation directive §9, F-DEFERRAL-2
    requires extending E.1 BINDINGS with a sibling SigningPayload entry.
    Verify `tests/constitution_architect_verbatim_struct_binding.rs` BINDINGS
    array has BOTH a P-M2 wire entry AND a P-M2 signing-payload entry, and
    BOTH are `LandingStatus::Landed`. Confirm the gate test
    `architect_verbatim_struct_field_bindings` enforces strict (name, type)
    pair equality on the signing-payload entry.

Q7. **Replay-determinism**: Replay must reconstruct identically from
    genesis_report + ChainTape + CAS + agent_registry. Verify:
    - `complete_set_merge_accept_state_root` is deterministic (uses
      sha256 of canonical_encode(tx) under domain prefix
      `b"turingosv4.complete_set_merge.accept.v1"`).
    - `CompleteSetMerge` variant added to TxKind enum at position 14;
      L4 LedgerEntry replay reads back the same payload via
      `tx_payload_cid` round-trip.
    - 6 trust_root rehashes (typed_tx.rs / sequencer.rs / transition_ledger.rs
      / monetary_invariant.rs / verify.rs / run_summary.rs) ensure
      `verify_trust_root_passes_on_intact_repo` succeeds at HEAD `66f4e34`.

Q8. **Strategic risk**: What, if anything, in Phase F.1 P-M2 substrate is
    visibly wrong or missing that future Phase F.3 (P-M4 CpmmPool) or
    Phase F.5 (P-M6 Mint-and-Swap Router with strict-equality
    monetary_invariant + atomic-rollback witness) would expose? In
    particular consider: how does merge interact with future swap pools
    (post-P-M5)? Does merge introduce any precondition that a future
    router would have to reason about? Are there any subtle invariant
    breaks that pass today's narrow tests but would surface under
    real-LLM Polymarket smoke?

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
    ("handover/directives/2026-05-09_STAGE_C_POLYMARKET_PM2_§8_PACKET.md", "markdown"),
    ("tests/constitution_completeset_merge.rs", "rust"),
    ("tests/constitution_architect_verbatim_struct_binding.rs", "rust"),
    ("CLAUDE.md", "markdown"),
]:
    brief += append_file(rel, lang)

print(f"[gemini C P-M2] prompt size: {len(brief):,} chars", file=sys.stderr)

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
    print(f"[gemini C P-M2] error: {exc}", file=sys.stderr)
    sys.exit(1)

elapsed = time.time() - t0
print(f"[gemini C P-M2] API returned in {elapsed:.1f}s", file=sys.stderr)

try:
    text = data["candidates"][0]["content"]["parts"][0]["text"]
except (KeyError, IndexError) as exc:
    print(f"[gemini C P-M2] malformed API response: {exc}", file=sys.stderr)
    print(json.dumps(data, indent=2)[:4000], file=sys.stderr)
    sys.exit(1)

verdict = extract_verdict(text)
header = f"""# Gemini Stage C P-M2 (CompleteSetMergeTx rebuild) PRE-§8 Audit — {ROUND}

**Round**: {ROUND}
**Date**: 2026-05-09
**Model**: gemini-2.5-pro
**Elapsed**: {elapsed:.1f}s
**Prompt size**: {len(brief):,} chars
**HEAD**: 66f4e34
**Final aggregate verdict**: {verdict}

---

## Verbatim Gemini Response

"""

OUT.write_text(header + text)
print(f"[gemini C P-M2] saved: {OUT}")
print(f"[gemini C P-M2] verdict: {verdict}")
