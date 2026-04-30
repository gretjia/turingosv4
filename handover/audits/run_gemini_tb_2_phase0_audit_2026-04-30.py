#!/usr/bin/env python3
"""Gemini round-1 Phase-0 dual external audit on TB-2 STEP_B preflight
(`handover/ai-direct/TB-2_SEQUENCER_RUNTIME_CLOSURE_2026-04-30.md`).

Strategic / architectural angle: is the proposed sequencer.rs runtime closure
the minimum-sufficient way to discharge P1 Exit 5/6/9 + P3 Exit 3/5 without
locking in premature P5/P6 design choices or leaking P3 substrate into P1's
lap? Independent of Codex round-1 (implementer-paranoid; running in parallel).

Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.
Per Elon-mode 2-round cap (memory `feedback_elon_mode_policy`): round 2
runs only on r1 CHALLENGE/VETO.
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
OUT = REPO / "handover/audits/GEMINI_TB_2_PHASE0_AUDIT_2026-04-30.md"

key = None
for line in ENV_FILE.read_text().splitlines():
    if line.startswith("GEMINI_API_KEY="):
        key = line.split("=", 1)[1].strip()
        break
if not key:
    sys.exit("ERROR: no GEMINI_API_KEY in env")

PROMPT = """You are Gemini DeepThink, the strategic architectural reviewer for TuringOS v4.

This is **round-1** Phase-0 dual external audit on the TB-2 STEP_B preflight
for sequencer.rs runtime closure. Codex is running **round-1** in parallel
with the implementer-paranoid angle (verifying every claim against shipped
code). Your angle is **strategic / architectural / constitutional**.

Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.

## Mandate

STEP_B_PROTOCOL Phase 0 (necessity audit) gate. You answer 4 questions:

- Is the change *necessary*? What observable behavior is broken now?
- Is a less-invasive alternative available? (same effect, no STEP_B-class
  edit, or a smaller STEP_B-class edit)
- What's the *minimum sufficient* version? (avoid scope creep)
- What's the failure mode if we don't change?

If both you and Codex say "less-invasive alternative exists" → take that
path. If both say "change as scoped is necessary" → proceed to Phase 1.
If you disagree → conservative verdict (block) wins.

## Context

**TB-1 shipped 2026-04-30** (commits `063b003..ccb01fa`) with **narrowed
central claim**: "P1/P3 RSP-0 primitives + invariant scaffolding green;
runtime dispatch enforcement deferred to TB-2." TB-1 built the L4 accepted
wrapper, L4.E rejection-evidence ledger, monetary invariants, and escrow
scaffolding — but no real `WorkTx` ever traverses `Sequencer::dispatch_transition`
at HEAD `3f06d51`. Every `TypedTx` variant returns `TransitionError::NotYetImplemented`.
`apply_one` early-returns on transition error and only `log::debug!`s it.
`submit_id` is allocated at `submit()` but never travels into `apply_one`
(queue payload is `TypedTx`, not an envelope).

**TB-2 charter** (`handover/tracer_bullets/TB-2_charter_2026-04-30.md`) is
"P1/P3 Runtime Boundary Closure + RSP-1". Goal: real WorkTx traverses
`Sequencer::dispatch_transition`; accepted → canonical L4 (`bottom_white::
ledger::transition_ledger` + `LedgerWriter`, NOT `economy::ledger::AcceptedLedger`
which stays a TB-1 RSP-0 primitive); rejected → L4.E with `submit_id`;
RSP-1 admission via existing `WorkTx.stake > 0` + seeded `EconomicState`
escrow / task-market entry (formal `task_open_tx` / `escrow_lock_tx` /
`yes_stake_tx` variants reserved for TB-3).

**STEP_B preflight under audit** is `handover/ai-direct/TB-2_SEQUENCER_RUNTIME_CLOSURE_2026-04-30.md`.
It declares:
- §0: sequencer.rs is "institution per C-031" even though not literally on
  CLAUDE.md's restricted-file path list (currently `kernel.rs` + `bus.rs`
  + `sdk/tools/wallet.rs`).
- §3: minimum-sufficient version = `SubmissionEnvelope { submit_id, tx }` +
  WorkTx pure validation in `dispatch_transition` + `apply_one` rejection
  writer error path + accepted path stays on existing `transition_ledger` +
  `LedgerWriter`.
- §5: 12-test acceptance battery (2 plumbing + 6 rejection + 4 acceptance).
- §6: deterministic 12/12 PASS as the verdict rule (NOT a SolveRate A/B,
  because no LLM in the loop yet).
- §8 red lines: no ledger I/O inside `dispatch_transition`; no `AcceptedLedger`
  as production accepted spine; no new `TypedTx` variants Day-1; no non-empty
  `exempt_tx_kinds` at runtime; no widening of WalletTool.

The CENTRAL claim under audit:

> "The proposed sequencer.rs runtime closure is the minimum-sufficient
>  edit to discharge P1 Exit 5/6/9 + P3 Exit 3/5 + P1 kill 1/2 + P3 kill
>  2/3 at the runtime spine, without (a) introducing a second accepted L4
>  ledger, (b) leaking P3 substrate (formal escrow/stake tx variants) into
>  P1's lap, (c) putting side effects into the pure transition function,
>  (d) creating a post-init mint backdoor via exempt_tx_kinds."

## Round-1 audit questions (8; strategic / architectural)

**Q1. STEP_B applicability — is sequencer.rs really institutional / C-031-class?**
The path `src/state/sequencer.rs` is not literally on CLAUDE.md's restricted
list. The preflight argues it qualifies under STEP_B's "any proposal that
touches 'institution' per C-031" trigger because it is the runtime wtool
gate (the only writer that mutates `state_root_t` / `ledger_root_t` / accepted
`logical_t`). Is this a sound application of STEP_B, or is it stretching
the protocol? Should sequencer.rs be formally added to CLAUDE.md's restricted
list (or `STEP_B_PROTOCOL.md` line 3) instead of relying on the C-031
catch-all? Or is the C-031 trigger sufficient and a CLAUDE.md edit would
be ceremonial overreach?

**Q2. A-corrected vs naive A — is the "no ledger I/O inside `dispatch_transition`"
ruling architecturally necessary?**
The preflight rejects naive A (which would put `RejectionEvidenceWriter::
append_rejected` and `AcceptedLedger::append_accepted` calls inside
`dispatch_transition`'s WorkTx arm). It argues: `dispatch_transition` must
stay pure so it is replayable from the ledger, otherwise replay creates a
chicken-and-egg loop (committing during replay re-commits). Is this argument
sound? Is there any context in which a pure transition function would
legitimately produce ledger side effects (e.g. for checkpoint emission),
or is the FC2 (state mutation) / FC3 (ledger persistence) separation absolute?

**Q3. AcceptedLedger as TB-1 primitive — is the "two-L4 spines" concern real,
or theoretical?**
`src/economy/ledger.rs::AcceptedLedger` is the TB-1 RSP-0 wrapper (in-memory
`Vec`, no real `SystemSignature`, no `Git2LedgerWriter` chain). The preflight
forbids using its `append_accepted` on the production runtime spine and
mandates that accepted WorkTx commit through the existing `transition_ledger`
+ `LedgerWriter` flow. Is this the right call? Or could `AcceptedLedger`
be promoted by upgrading its internals (swap Vec → Git2-backed) without
creating a second spine? If TB-2 keeps `AcceptedLedger` only as a primitive,
when does it get retired (after which TB), or does it stay as a forensic
test wrapper indefinitely?

**Q4. RSP-1 admission via stake>0 + seeded EconomicState escrow — is this
minimum-viable RSP-1, or "fake RSP-1" that creates an illusion of progress?**
The preflight defers `task_open_tx` / `escrow_lock_tx` / `yes_stake_tx`
formal `TypedTx` variants to TB-3. Day-1 RSP-1 admission inspects existing
`WorkTx.stake` and existing seeded `EconomicState` task-market / escrow
entries. Critical question: does this constitute genuine RSP-1 admission
(stake + escrow are economically REAL gates that real submissions must
satisfy), or is it merely typecheck-level decoration that gives RSP-1 a
green checkmark while the actual escrow/stake economy is still vapor? If
fake-RSP-1, what would the test fixture look like that exposes the gap?

**Q5. SubmissionEnvelope first atom — necessary plumbing, or premature
ABI churn?**
The preflight makes `SubmissionEnvelope { submit_id, tx }` the first runtime
code atom (Atom 2 in the charter sequence) on the grounds that L4.E
identity contract requires `submit_id` to travel into `apply_one`, and
the current queue payload `TypedTx` strands `submit_id` at `submit()`.
Could the same outcome be achieved with a less invasive change — e.g.
`tokio::sync::mpsc::channel<(u64, TypedTx)>` tuple, or `submit_id`
embedded inside `TypedTx` itself, or per-tx CAS lookup of a side-table
keyed by `tx_hash`? If `SubmissionEnvelope` is the right shape, does it
need additional fields now (timestamp_logical / submitter_id) to avoid a
second envelope rewrite in TB-3 / TB-4?

**Q6. `exempt_tx_kinds` red line — is "no non-empty `exempt_tx_kinds` at
runtime" the right fence, or does it block legitimate genesis-only mint?**
The preflight forbids non-empty `exempt_tx_kinds` argument to
`assert_total_ctf_conserved` at the runtime call sites. The charter §5
argues: "TxKind::FinalizeReward is transfer (escrow → claims), not mint,
and never qualifies." The preflight raises the option of replacing the
TxKind-based exemption with an explicit `SupplyDeltaAuthorization` enum
(`None | GenesisOnly { constitution_root, root_signature }`). Is the
TB-2 fence right (no non-empty list)? Should the API change happen now
(in TB-2) or wait until a TB explicitly needs it? If wait, is the fence
"document don't refactor" or "refactor before any TB ever uses it"?

**Q7. Acceptance battery — does 12/12 cover the claim, or are there gaps?**
Tests:
- 1-2: SubmissionEnvelope plumbing
- 3-6: rejection spine (predicate-fail / stakeless / no-escrow / post-init-mint)
- 7-8: rejection invariants (no logical_t / no state_root advance)
- 9-12: acceptance spine (state_root / ledger_root / logical_t / no L4.E)

Two ship proofs (charter §8): predicate-failed WorkTx via `Sequencer::submit`
→ exactly one L4.E row + zero state_root change; predicate-passing WorkTx
with stake+escrow → state_root + ledger_root + logical_t advance + zero
L4.E rows. **Are there cases that should be in the battery but aren't?**
Specifically:
- raw_diagnostic_cid serde-shield re-confirmed at runtime path (TB-1 P0-3
  was at the primitive level; does TB-2 need to re-verify at the runtime
  spine?)
- parent_state_root mismatch as a distinct rejection class
- queue-full backpressure (`SubmitError::QueueFull`) interaction with
  submit_id allocation (does failed `try_send` still consume a `submit_id`?)
- replay test (rebuild state.db from canonical L4 transitions only;
  L4.E records must NOT advance state_root during replay)

**Q8. Phase-ordering — TB-2 advances P1+P3 simultaneously. Does this honor
the 9-phase ordering, or does it conflate phases?**
The 9-phase roadmap principle: "P0 → P1 → P2 → P3 → ...; 不要反过来"
(don't reverse the order). TB-2 declares `phase_id=P1+P3 (primary P1; P3
RSP-1 secondary)`. Defense (charter §1, §3): RSP-1 admission is the FC2
predicate-bundle that gates P1's accepted-spine semantics, so the two
phases share the runtime spine and must be discharged together. Is this
honest co-discharge of intertwined phase contracts, or is it scope creep
that pulls P3 work into a P1-primary TB?

Specifically, the dependency graph in `ROADMAP_9_PHASE_2026-04-29.md` § 12
sequences P3 RSP-0/RSP-1 BENEATH P1, not at the same level. Does TB-2's
P1+P3 framing align with that graph? Or should TB-2 have been split into
TB-2 (P1 runtime closure with stub stake/escrow gating) followed by TB-3
(P3 RSP-1 properly)?

## Verdict format

Section A: Overall verdict (PASS / CHALLENGE / VETO) with conviction (1-5).
Section B: Per-Q1-Q8 disposition (one paragraph each + verdict tag).
Section C: P0 list (must-fix-before-Phase-1, if any). For each P0, give:
           file:line + concrete remediation + estimated effort.
Section D: P1 list (should-fix; can proceed-with-OBS).
Section E: Recommendation — proceed to STEP_B Phase-1 / revise preflight /
           reject TB-2 charter as scoped.

Be concrete. Cite file:line where applicable. The materials below include
the STEP_B preflight, TB-2 charter, the TB-1 ship row, the canonical
roadmap, and the actual `sequencer.rs` source so you can verify scope
claims independently.

---

# XREF materials follow.
"""

attachments = []

def append_file(label, path, fence="rust"):
    full = REPO / path
    if not full.exists():
        attachments.append(f"\n\n---\n\n## XREF: {label} — `{path}` [missing at audit time]\n")
        return
    attachments.append(f"\n\n---\n\n## XREF: {label} — `{path}`\n\n```{fence}\n{full.read_text()}\n```\n")

# Audit target — STEP_B preflight (PRIMARY).
append_file(
    "STEP_B preflight (audit target)",
    "handover/ai-direct/TB-2_SEQUENCER_RUNTIME_CLOSURE_2026-04-30.md",
    fence="markdown",
)
# TB-2 charter.
append_file(
    "TB-2 charter",
    "handover/tracer_bullets/TB-2_charter_2026-04-30.md",
    fence="markdown",
)
# 9-phase roadmap (Q1, Q8).
append_file(
    "9-phase canonical roadmap",
    "handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md",
    fence="markdown",
)
# Constitution (Q1, Q2 — for C-031 + Anti-Oreo).
append_file("constitution.md", "constitution.md", fence="markdown")
append_file("STEP_B_PROTOCOL.md", "handover/ai-direct/STEP_B_PROTOCOL.md", fence="markdown")
append_file("CLAUDE.md (restricted-file list source)", "CLAUDE.md", fence="markdown")
# Shipped sequencer.rs (Q2, Q3, Q5, Q7 — verify scope claims).
append_file(
    "sequencer.rs (current HEAD; runtime spine under audit)",
    "src/state/sequencer.rs",
    fence="rust",
)
# AcceptedLedger primitive (Q3).
append_file(
    "AcceptedLedger (TB-1 RSP-0 primitive; not production L4)",
    "src/economy/ledger.rs",
    fence="rust",
)
# L4.E writer (Q7).
append_file(
    "rejection_evidence.rs (L4.E writer)",
    "src/bottom_white/ledger/rejection_evidence.rs",
    fence="rust",
)
# Monetary invariant (Q6).
append_file(
    "monetary_invariant.rs (assert_total_ctf_conserved + exempt_tx_kinds)",
    "src/economy/monetary_invariant.rs",
    fence="rust",
)
# typed_tx (Q5, Q8 — see what variants exist today).
append_file("typed_tx.rs", "src/state/typed_tx.rs", fence="rust")
# q_state (Q5 — for state_root_t shape).
append_file("q_state.rs", "src/state/q_state.rs", fence="rust")
# canonical L4 writer (Q3 — what the production accepted spine looks like).
ledger_dir = REPO / "src/bottom_white/ledger"
if ledger_dir.exists():
    for child in sorted(ledger_dir.iterdir()):
        if child.is_file() and child.suffix == ".rs":
            append_file(f"bottom_white/ledger/{child.name}", f"src/bottom_white/ledger/{child.name}", fence="rust")
# TB_LOG.tsv (current state).
append_file("TB_LOG.tsv (TB-1 shipped + TB-2 active rows)", "handover/tracer_bullets/TB_LOG.tsv", fence="tsv")
# AUTO_RESEARCH_NOTEPAD (current sync state).
append_file(
    "AUTO_RESEARCH_NOTEPAD.md",
    "handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md",
    fence="markdown",
)
# Recent commit log.
log = subprocess.run(
    ["git", "-C", str(REPO), "log", "--pretty=format:%h %s", "459c747~1..HEAD"],
    capture_output=True, text=True,
).stdout
attachments.append(f"\n\n---\n\n## XREF: Recent commit log (TB-1 ship → TB-2 Day-1)\n\n```\n{log}\n```\n")

full_prompt = PROMPT + "".join(attachments)
print(f"[gemini tb-2 phase0] prompt size: {len(full_prompt)} chars", file=sys.stderr)

url = f"https://generativelanguage.googleapis.com/v1beta/models/gemini-3.1-pro-preview:generateContent?key={key}"
body = json.dumps({
    "contents": [{"parts": [{"text": full_prompt}]}],
    "generationConfig": {
        "temperature": 0.3,
        "maxOutputTokens": 65536,
    },
}).encode()
req = urllib.request.Request(
    url, data=body,
    headers={"Content-Type": "application/json"},
    method="POST",
)

t0 = time.time()
try:
    with urllib.request.urlopen(req, timeout=900) as resp:
        data = json.loads(resp.read().decode())
except urllib.error.HTTPError as e:
    print(f"HTTP {e.code}: {e.read().decode()[:500]}", file=sys.stderr)
    sys.exit(1)
elapsed = time.time() - t0
print(f"[gemini tb-2 phase0] API returned in {elapsed:.1f}s", file=sys.stderr)

candidates = data.get("candidates", [])
if not candidates:
    sys.exit(f"No candidates: {json.dumps(data)[:500]}")
parts = candidates[0].get("content", {}).get("parts", [])
text = "".join(p.get("text", "") for p in parts)

OUT.parent.mkdir(parents=True, exist_ok=True)
git_head = subprocess.run(
    ["git", "-C", str(REPO), "rev-parse", "HEAD"],
    capture_output=True, text=True,
).stdout.strip()
header = f"""# Gemini TB-2 Phase-0 Round-1 Dual External Audit
**Date**: 2026-04-30
**Target**: STEP_B preflight `handover/ai-direct/TB-2_SEQUENCER_RUNTIME_CLOSURE_2026-04-30.md`
**HEAD**: {git_head}
**Prompt size**: {len(full_prompt)} chars
**API latency**: {elapsed:.1f}s
**Mandate**: STEP_B Phase-0 necessity audit; strategic / architectural / constitutional (Q1-Q8). Independent of Codex r1 (parallel, implementer-paranoid).

---

"""
OUT.write_text(header + text)
print(f"[gemini tb-2 phase0] saved: {OUT}", file=sys.stderr)
