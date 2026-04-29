#!/usr/bin/env python3
"""Gemini round-1 dual external audit on TB-1 (Days 1-5; commits 063b003..6c04c26).

Strategic / architectural angle: does the L4/L4.E split + monetary invariant
+ h_vppu instrumentation form a coherent foundation for TB-2 RSP-1 and the
P3 → P4 → P5 → P6 phase progression? Independent of Codex round-1
(implementer-paranoid; running in parallel).

Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.
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
OUT = REPO / "handover/audits/GEMINI_TB_1_AUDIT_2026-04-29.md"

key = None
for line in ENV_FILE.read_text().splitlines():
    if line.startswith("GEMINI_API_KEY="):
        key = line.split("=", 1)[1].strip()
        break
if not key:
    sys.exit("ERROR: no GEMINI_API_KEY in env")

PROMPT = """You are Gemini DeepThink, the strategic architectural reviewer for TuringOS v4.

This is **round-1** dual external audit on TB-1 (Days 1-5; the P1 GitTape +
P3 RSP-0 + P6 h_vppu instrumentation tracer bullet). Codex is running
**round-1** in parallel with the implementer-paranoid angle. Your angle is
**strategic / architectural / constitutional**.

Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.
Per Elon-mode 2-round cap (memory `feedback_elon_mode_policy`): round 2
runs only on r1 CHALLENGE/VETO.

## Mandate

Verify TB-1 ship readiness from the architectural / constitutional angle.

The recharter at `handover/tracer_bullets/TB-1_recharter_2026-04-29.md`
reframes TB-1 against the 9-phase roadmap (P0..P9; canonical doc
`handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md`). TB-1 advances
P1 (primary) + P3 RSP-0 (secondary) + P6 instrumentation (tertiary). Ship
gate per CF-5 lighter option: ALL Tier-A tests (1-9) green; Tier-B (10-13)
non-blocking.

The CENTRAL claim under audit:

> "TB-1 Days 1-5 establishes the structural primitives + invariant assertions
>  needed to advance to TB-2 RSP-1 (escrow_lock_tx + work_tx + yes_stake)
>  WITHOUT having committed to specific dispatch_transition wiring details
>  that would prematurely lock in P5/P6 design choices."

Your audit answers: **is this a clean architectural foundation, or are
there premature commitments / scope creep / phase-ordering violations
that will compound into expensive rework at TB-2/TB-3?**

## Round-1 audit questions (5; strategic / architectural)

**Q1. Phase-ordering integrity — is TB-1 honoring P1-before-P3-before-P5/P6?**
The 9-phase roadmap principle: "不要反过来。一开始就做开放市场、公链、AGI 科研、自治公司 = 不可控的黑盒赌场。" (Don't reverse the order: starting from open market / public chain / autonomous economy = uncontrollable black-box casino.)

TB-1 ships:
- P1 primitives (L4 ledger + L4.E rejection-evidence): foundation layer ✅
- P3 RSP-0 invariant (monetary conservation + escrow scaffold): one rung up ✅
- P6 h_vppu (Epistemic Lab metric, no flowchart anchor): top of the stack ⚠

Architectural concern: does shipping P6 instrumentation in the same TB as
P1 primitives violate the phase-ordering principle? Or is P6 specifically
allowed as "anchor evidence" per recharter § "Out-of-order TBs are allowed
only as P6 anchor evidence"?

Verify: does the h_vppu wire-up (evaluator.rs main() post-hoc stamp) create
a dependency that future P5 MetaTape ArchitectAI work will have to step
around, or is it cleanly orthogonal?

**Q2. L4 vs L4.E split — is this the RIGHT primitive separation?**
Per the audit-driven decision record (`handover/alignment/DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md`),
L4 is accepted-only; L4.E is rejection-evidence; they share NO state_root advance path.

Strategic check:
  a. Is two-ledger the right primitive count, or should it be one (with status field) / three (accepted + rejected + invalid-malformed)?
  b. Will TB-2 RSP-1's escrow_lock_tx + yes_stake_tx fit cleanly into L4? Or will the dispatch_transition wiring need a third "settlement-pending" intermediate ledger?
  c. The recharter Day-3 says rejected logs use `submit_id` (NOT `logical_t`) so they can't be confused with L4 entries. Is this disjointness mechanism sufficient, or should there be a domain-separation prefix (e.g., L4 uses `turingosv4.l4_accepted.v1` and L4.E uses `turingosv4.l4e_rejection_evidence.v1` in their canonical hashes)? Verify by reading the canonical_encode in both modules.

**Q3. Monetary invariant scope — is RSP-0 the right minimal slice for P3?**
The recharter Day-2 ships:
  - assert_no_post_init_mint (structural mint guard at tx layer)
  - assert_total_ctf_conserved (numeric conservation across 6 sub-indices)
  - assert_read_is_free (per-tx fee structural guard)
  - escrow_vault (in-memory BTreeMap; lock + release primitives)

Strategic check:
  a. Is "RSP-0 = invariants + scaffold, no live wiring" the right deliverable size for a 7-day TB? Or is it under-scoped (creates pressure to ship something larger in TB-2) or over-scoped (the invariants will need to be re-touched as RSP-1+ wiring lands)?
  b. The Tier-A test 7 (`test_p3_rsp0_exit_1_on_init_total_invariant`) tests a 5-step closed-loop redistribution. Does this test the right invariant, or should it be parameterized (e.g., quickcheck-style) over random redistribution sequences? The Day-2 unit tests inside monetary_invariant.rs DO have an N=10 deterministic sequence — should the Tier-A test also have N=10 to match?
  c. escrow_vault ships unit-tested but is NOT exercised by any Tier-A test. Is that an architectural choice (P3 RSP-0 = invariants only; escrow exercise lands at RSP-1 Tier-A), or an oversight?

**Q4. h_vppu Goodhart-shield — is the metric instrumentation safe?**
The h_vppu metric: `current_pput_verified / mean(N=1..3 prior runs of same problem)`.

Strategic / Goodhart check:
  a. The metric returns `None` when mean=0 (anti-Goodhart: never emits NaN/inf
     into JSONL). Is this the right behavior, or should it return `Some(0.0)`
     when `current=0` AND `mean=0` (both runs failed; no learning happened)?
  b. The metric INCLUDES the current run's pput_verified in the persisted
     history AFTER the query, so the next run's history includes it. Is the
     "held-out baseline" semantic correctly preserved, or does the rolling
     window mechanism eventually contaminate the baseline (e.g., after run 4,
     the history is [run2, run3, run4] — runs 2-4 are all "this session";
     no genuine held-out)?
  c. The metric is wired post-hoc in evaluator.rs main(), not inside
     `make_pput`. Is this the right wire-site, or should it be inside
     `make_pput` to stay consistent with the v2 jsonl_schema's other
     fields? The recharter Day-4 explicitly says "Wire into make_pput:
     pass history reference; stamp h_vppu field on result", but the
     implementation chose post-hoc stamping (less invasive: 14 call sites
     vs. 1). Is the divergence justified, or should it be flagged as a
     spec / impl mismatch?

**Q5. Tier-A vs Tier-B downgrade per CF-5 — is the ship gate defensible?**
The recharter (post-audit amendment) downgraded original AT-1..AT-4 from
blocking to non-blocking, calling them "P6 anchor evidence + future-RSP
placeholders". CF-5 lighter option says TB-1 ships when Tier-A 1-9 green.

Strategic check:
  a. Is this CF-5 downgrade architecturally defensible, or is it ship-pressure
     erosion of the original charter quality bar?
  b. What's the right framing for the user: "TB-1 = P1+P3 correctness;
     P6 capability metric is anchored by Day-4 evidence outside the harness"?
     Or: "TB-1 = P1+P3 only; P6 is a separate concern that will be re-charted
     as a P6 anchor TB"?
  c. Specifically: T10 (mathd_algebra_107 evaluator solve) is verified by
     out-of-band evidence (commit 50a1d67's Day-4 jsonl files in /tmp/).
     Is "out-of-band evidence" a defensible substitute for an in-harness
     test, or should TB-1 require the test to be in-harness (perhaps as a
     #[ignore]-by-default integration test that CI runs nightly)?

## Round-1 audit questions (3; constitutional alignment)

**Q6. TRACE_MATRIX coverage — are the new pub symbols correctly anchored?**
The new files (Day 2-4) introduce many new pub symbols. Each should have
a `/// TRACE_MATRIX <FC-id>: <role>` doc-comment OR an entry in
`tests/orphan_registry.md` with constitutional justification (per CLAUDE.md
Alignment Standard).

Spot-check by reading the modules:
  - `src/economy/monetary_invariant.rs`: do MonetaryError, assert_no_post_init_mint, assert_total_ctf_conserved, assert_read_is_free all carry TRACE_MATRIX backlinks?
  - `src/economy/escrow_vault.rs`: ditto for VaultEntry, VaultError, lock_escrow, release_escrow, EscrowReceipt, ReleaseOutcome.
  - `src/economy/ledger.rs`: ditto for AcceptedLedger + AcceptedEntry + LedgerError + new()/append/verify/reconstruct.
  - `src/bottom_white/ledger/rejection_evidence.rs`: ditto for RejectionEvidenceWriter + RejectedSubmissionRecord + RejectionClass + PublicRejectionView + RejectionEvidenceError.
  - `experiments/minif2f_v4/src/h_vppu_history.rs`: declared as orphan with PREREG § 5 justification — is this acceptable, or should there be a TRACE_MATRIX entry?

**Q7. Trust Root manifest hygiene — are all new src/ files registered?**
genesis_payload.toml [trust_root] should include every new src/ file.
Day 2 added monetary_invariant.rs + escrow_vault.rs.
Day 3 added economy/ledger.rs + bottom_white/ledger/rejection_evidence.rs.
Day 4 added experiments/minif2f_v4/src/h_vppu_history.rs + re-hashed evaluator.rs.

Verify each is in the manifest with the correct sha256. Are there any
new src/ pub-API files NOT in the manifest? (Anti-Goodhart: only listing
"trusted" files in the manifest creates a silent expansion vector.)

**Q8. Is there a STEP_B-protected file violation?**
The recharter explicitly forbids edits to `src/kernel.rs`, `src/bus.rs`,
`src/sdk/tools/wallet.rs` during TB-1 (per `feedback_step_b_protocol`
memory). Verify the Days 1-5 commits did NOT touch any of these files.

## Verdict format

Section A: Verdict (PASS / CHALLENGE / VETO) with conviction level (1-5).
Section B: Per-Q1-Q8 disposition (one paragraph each).
Section C: P0 list (must-fix-before-ship, if any). For each P0, give:
           file:line + concrete remediation + estimated effort.
Section D: P1 list (should-fix; can ship-with-OBS).
Section E: TB-2 readiness assessment — does TB-1 leave the codebase in a
           state where TB-2 RSP-1 can begin without rework?

Be concrete. Cite file:line. The materials below include the recharter,
the L4/L4.E decision record, the Tier-A acceptance battery, all 4
production modules (Day 2-3), the h_vppu_history module (Day 4), the
Day-4 evaluator wire-up snippet, the Day-4 live evidence, and the recent
commit log.

---

# XREF materials follow.
"""

attachments = []

def append_file(label, path, fence="rust"):
    full = REPO / path
    if not full.exists():
        return
    attachments.append(f"\n\n---\n\n## XREF: {label} — `{path}`\n\n```{fence}\n{full.read_text()}\n```\n")

def append_evidence(label, abspath, fence="json"):
    p = pathlib.Path(abspath)
    if not p.exists():
        attachments.append(f"\n\n---\n\n## XREF: {label} — `{abspath}` [evidence file missing at audit time]\n")
        return
    attachments.append(f"\n\n---\n\n## XREF: {label} — `{abspath}`\n\n```{fence}\n{p.read_text()}\n```\n")

append_file("TB-1 recharter (audit target spec)", "handover/tracer_bullets/TB-1_recharter_2026-04-29.md", fence="markdown")
append_file("L4 vs L4.E decision record", "handover/alignment/DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md", fence="markdown")
append_file("Tier-A acceptance battery (Day-5 final)", "tests/tb_1_acceptance.rs", fence="rust")
append_file("monetary_invariant.rs (Day-2)", "src/economy/monetary_invariant.rs", fence="rust")
append_file("escrow_vault.rs (Day-2)", "src/economy/escrow_vault.rs", fence="rust")
append_file("ledger.rs (L4; Day-3)", "src/economy/ledger.rs", fence="rust")
append_file("rejection_evidence.rs (L4.E; Day-3)", "src/bottom_white/ledger/rejection_evidence.rs", fence="rust")
append_file("h_vppu_history.rs (Day-4)", "experiments/minif2f_v4/src/h_vppu_history.rs", fence="rust")
append_file("9-phase roadmap (canonical phase ordering)", "handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md", fence="markdown")

# Day-4 evaluator wire-up snippet (lines ~322-385).
eval_path = REPO / "experiments/minif2f_v4/src/bin/evaluator.rs"
if eval_path.exists():
    lines = eval_path.read_text().splitlines()
    snippet = "\n".join(lines[319:395])
    attachments.append(f"\n\n---\n\n## XREF: Day-4 evaluator.rs main() wire-up (lines 320-395)\n\n```rust\n{snippet}\n```\n")

# Day-4 live evidence.
append_evidence("Day-4 RUN 1 (cold; no h_vppu)", "/tmp/tb1_day4_smoke_v2/run1.jsonl", fence="json")
append_evidence("Day-4 RUN 2 (warm; h_vppu=6.21)", "/tmp/tb1_day4_smoke_v2/run2.jsonl", fence="json")
append_evidence("Day-4 h_vppu_history.json after run 2", "/tmp/tb1_day4_smoke_v2/h_vppu_history.json", fence="json")

# Trust Root manifest (Q7).
append_file("genesis_payload.toml [trust_root] (Q7 verification)", "genesis_payload.toml", fence="toml")

# Recent commit log.
log = subprocess.run(
    ["git", "-C", str(REPO), "log", "--pretty=format:%h %s", "063b003~1..HEAD"],
    capture_output=True, text=True,
).stdout
attachments.append(f"\n\n---\n\n## XREF: Recent commit log (TB-1 Days 1-5)\n\n```\n{log}\n```\n")

full_prompt = PROMPT + "".join(attachments)
print(f"[gemini tb-1] prompt size: {len(full_prompt)} chars", file=sys.stderr)

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
    with urllib.request.urlopen(req, timeout=600) as resp:
        data = json.loads(resp.read().decode())
except urllib.error.HTTPError as e:
    print(f"HTTP {e.code}: {e.read().decode()[:500]}", file=sys.stderr)
    sys.exit(1)
elapsed = time.time() - t0
print(f"[gemini tb-1] API returned in {elapsed:.1f}s", file=sys.stderr)

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
header = f"""# Gemini TB-1 Round-1 Dual External Audit
**Date**: 2026-04-29
**Target**: TB-1 Days 1-5 ship readiness (commits 063b003..HEAD)
**HEAD**: {git_head}
**Prompt size**: {len(full_prompt)} chars
**API latency**: {elapsed:.1f}s
**Mandate**: strategic / architectural / constitutional (Q1-Q8). Independent of Codex r1 (parallel).

---

"""
OUT.write_text(header + text)
print(f"[gemini tb-1] saved: {OUT}", file=sys.stderr)
