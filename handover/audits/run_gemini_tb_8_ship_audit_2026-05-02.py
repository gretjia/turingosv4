#!/usr/bin/env python3
"""Gemini TB-8 ship audit — Class 3 strategic / architectural angle.

Independent of Codex ship audit (parallel, implementation-paranoid).
Per memory feedback_dual_audit_conflict: VETO > CHALLENGE > PASS.

Strategic-tier model `gemini-3.1-pro-preview` first; if 429 capacity-exhausted,
caller must label this audit `degraded` per memory feedback_dual_audit.
"""
import json
import pathlib
import subprocess
import sys
import time
import urllib.error
import urllib.request

REPO = pathlib.Path("/home/zephryj/projects/turingosv4")
ENV_FILE = pathlib.Path("/home/zephryj/projects/turingosv3/.env")
OUT = REPO / "handover/audits/GEMINI_TB_8_SHIP_AUDIT_2026-05-02.md"

key = None
for line in ENV_FILE.read_text().splitlines():
    if line.startswith("GEMINI_API_KEY="):
        key = line.split("=", 1)[1].strip()
        break
if not key:
    sys.exit("ERROR: no GEMINI_API_KEY in env")

PROMPT = """You are Gemini DeepThink, the strategic architectural reviewer for TuringOS v4.

This is the **TB-8 ship-gate audit** (Minimal Payout / FinalizeRewardTx).
Class 3 (auth-crypto-money: first system-emitted variant that *moves money*
from escrows_t to balances_t). Your angle is **strategic / architectural /
constitutional**. Codex is running the implementation-paranoid angle in
parallel.

Per memory feedback_dual_audit_conflict: VETO > CHALLENGE > PASS.

## Audit target

```text
Charter:        handover/tracer_bullets/TB-8_charter_2026-05-02.md
Ratification:   handover/audits/CHARTER_RATIFICATION_TB_8_2026-05-02.md
STEP_B:         handover/audits/STEP_B_PREFLIGHT_TB8_2026-05-02.md
TB-7R baseline: 9e74195..4470036  (712/0/150)
TB-8 range:     4470036..HEAD
HEAD test:      cargo test --workspace = 723 passed / 0 failed / 150 ignored (+11 net)
```

## What TB-8 changes architecturally

1. **Closes the 5-step compile loop's settlement node.** Pre-TB-8: the
   evaluator emitted WorkTx + VerifyTx pairs but no FinalizeRewardTx ever
   landed. Solver balance never increased on solved tasks. Post-TB-8: every
   accepted L4 WorkTx with closed challenge window + no upheld challenge
   produces exactly one L4 FinalizeRewardTx that atomically debits escrow
   + credits solver.

2. **5→4 holding migration in monetary invariant.** claims_t was previously
   counted in total_supply_micro (5 holdings: balances + escrows + stakes +
   claims + bond). Per TB-8 charter §3 Atom 3: FinalizeReward dispatches
   escrows → balances **directly** (not via claims_t as intermediate
   holding). Therefore claims_t is now an INTENT REGISTRY, not a holding;
   counting it would double-mint every claim. New invariant
   `assert_claim_amount_backed_by_escrow` enforces intent-vs-backing
   integrity (claim.amount ≤ backing escrow row amount).

3. **System-emitted variant precedent extended.** TB-5 introduced the
   first system-emitted variant (ChallengeResolveTx). TB-8 adds the
   second: FinalizeRewardTx via SystemEmitCommand::FinalizeReward.
   Caller passes only claim_id; emit_system_tx Q-derives task_id /
   solver / reward from claims_t[claim_id] (anti-forgery per
   typed_tx.rs:300-304).

4. **Zero-window MVP** (per ratification §1 Q3 + §2.4 namespace
   correction): claims_t[claim_id].challenge_window_close_logical_t = 0
   at claim-creation; the dispatch-arm gate fires only when window > 0.
   Forward-compat: a future TB introducing real window timing sets window
   to a non-zero sequencer-namespace logical_t.

5. **Best-effort evaluator emit** (Atom 4): tb8_emit_finalize_after_verify
   polls q_snapshot for the new claim, then calls emit_system_tx. Failure
   does NOT fail the run — the L4 OMEGA evidence is the durable signal.

6. **Dashboard §9 Claims** (Atom 6): claim_status + payout_amount columns
   per user-minimum requirement "dashboard shows payout".

## Forbidden lines TB-8 must NOT cross (charter §4)

#1 NodeMarket trading | #2 AMM/CPMM | #3 CLOB | #4 CompleteSet |
#5 MarketSeedTx | #6 multi-solver royalty splits | #7 DAG-aware payout
splits | #8 public-chain anchoring | #9 MetaTape | #10 multi-org
settlement | #11 SettlementEngine generalization | #12 slash execution
(deferred to TB-15+) | #13 TaskExpire/TerminalSummary dispatch arms |
#14 per-tactic decomposition | #15 OBS-1 PartialOk routing | #16
wall-clock challenge-window scheduler | #17 Boltzmann masking | #18
Lamarckian Autopsy | #19 EvidenceCapsule | #20 constitution.md edits.

## Evaluation criteria (architect-strategic angle)

**Q1 — Was the 5→4 holding migration the right call?** TB-3 charter §3.2
established the cache-vs-holding distinction (task_markets_t.total_escrow
is derived cache, NOT holding). TB-8 extends the same pattern to
claims_t.amount: settlement is escrow→balance direct; claim is intent
metadata. Argument FOR: avoids double-mint at claim creation; matches the
Atom-3 dispatch shape exactly. Argument AGAINST: changes a long-standing
invariant shape (5→4 holdings) which a careless reader could mistake for
loosening conservation. Mitigation: dedicated `assert_claim_amount_backed_by_escrow`
invariant. Vote: was this the right design choice?

**Q2 — Is zero-window MVP a forward-compat dead-end or a clean MVP?**
The gate semantics is "fires only when window > 0". Forward-compat path
is documented (set window to sequencer-namespace logical_t at claim-
creation; gate becomes operative). But: the agent-controlled
verify.timestamp_logical was REJECTED as a window source (per §2.4) due
to namespace mixing. Does the architect agree this rejection is correct?

**Q3 — Best-effort evaluator emit safety.** Atom 4 returns Ok(false) on
poll-budget expiry. A solver may then have an Open claim that is never
finalized — they're owed money but not paid. Charter argues: L4 OMEGA
evidence is durable; a future admin-emit path or next session can
finalize. Is this acceptable for the FIRST money-moving variant in
production? Or should Atom 4 be fail-closed (exit 3 on emit failure)?

**Q4 — Anti-Oreo barrier integrity.** TypedTx::FinalizeReward(_) cannot
be agent-submitted (TB-5 RSP-3.0 inheritance: submit_agent_tx rejects
all 4 system-emitted variants pre-queue). Does the TB-8 wire-up preserve
this strictly? Confirm via grep that no agent-side surface constructs
FinalizeRewardTx with a forged signature.

**Q5 — Smoke variety.** 7 runs across 5+ distinct heldout-49 problems is
the user-stipulated variety bar. Per `feedback_smoke_evidence_naming`,
every run must be chain-backed (replayable from committed
runtime_repo.dotgit.tar.gz + cas.dotgit.tar.gz alone). Stdout-only paper
trails are forbidden. Verify by spot-check: extract a SOLVED run's
tar.gz pair into /tmp; run verify_chaintape; confirm replay_report.json
matches committed copy.

## Verdict format

End with one of:

```text
## VERDICT: PASS
(All Q1-Q5 cleared; architectural angle is clean.)
```

```text
## VERDICT: CHALLENGE
- <Q?> CHALLENGE: <one-line reason>
(round-2 will trigger feedback_elon_mode_policy auto-execute on
determinate-best surgical patch.)
```

```text
## VERDICT: VETO
- <Q?> VETO: <one-line BLOCKING reason>
(VETO blocks ship per feedback_dual_audit_conflict.)
```
"""

ENDPOINT = "https://generativelanguage.googleapis.com/v1beta/models/gemini-3.1-pro-preview:generateContent?key=" + key
body = json.dumps({
    "contents": [{"parts": [{"text": PROMPT}]}],
    "generationConfig": {"temperature": 0.2, "maxOutputTokens": 8192},
}).encode("utf-8")
req = urllib.request.Request(ENDPOINT, data=body, headers={"Content-Type": "application/json"})

degraded = False
attempt = 0
last_err = None
text = None
while attempt < 4:
    attempt += 1
    try:
        with urllib.request.urlopen(req, timeout=180) as resp:
            payload = json.loads(resp.read().decode("utf-8"))
            text = payload["candidates"][0]["content"]["parts"][0]["text"]
            break
    except urllib.error.HTTPError as e:
        last_err = e
        if e.code == 429:
            time.sleep(20 * attempt)
            continue
        else:
            break
    except Exception as e:
        last_err = e
        time.sleep(10 * attempt)

if text is None:
    # Fallback to gemini-2.5-pro-preview if strategic tier exhausted.
    degraded = True
    fb_endpoint = "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-pro-preview-06-05:generateContent?key=" + key
    req2 = urllib.request.Request(fb_endpoint, data=body, headers={"Content-Type": "application/json"})
    try:
        with urllib.request.urlopen(req2, timeout=180) as resp:
            payload = json.loads(resp.read().decode("utf-8"))
            text = payload["candidates"][0]["content"]["parts"][0]["text"]
    except Exception as e:
        sys.exit(f"ERROR: both strategic + fallback Gemini calls failed: strategic={last_err} fallback={e}")

degraded_marker = "**Degraded label**: this audit ran on the fallback `gemini-2.5-pro-preview` after the strategic tier `gemini-3.1-pro-preview` returned 429. Per memory feedback_dual_audit, the audit is labeled `degraded`.\n\n" if degraded else ""
header = f"# Gemini TB-8 Ship Audit\n\n**Date**: 2026-05-02\n**Model**: {'gemini-2.5-pro-preview-06-05 (degraded)' if degraded else 'gemini-3.1-pro-preview (strategic)'}\n**Audit type**: Class 3 strategic / architectural angle (parallel with Codex impl-paranoid).\n\n{degraded_marker}---\n\n"
OUT.write_text(header + text + "\n")
print(f"Gemini audit saved to: {OUT}")
print(f"Degraded: {degraded}")
