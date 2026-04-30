#!/usr/bin/env python3
"""Gemini Phase-1c diff dual external audit on TB-2 experiment branch.

Strategic / architectural angle: did the implementation honor every constraint
the preflight v3 committed to? In particular: no ledger I/O inside
`dispatch_transition`; no `AcceptedLedger` on the production accepted spine;
`assert_total_ctf_conserved(..., &[])` only at runtime; P0-B option (a) bridge
marked for TB-3 deletion; named-struct vs tuple choice; replay invariant
actually proven by I13.

Independent of Codex Phase-1c (implementer-paranoid; running in parallel).
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
WORKTREE = pathlib.Path(
    "/home/zephryj/projects/turingosv4/.claude/worktrees/stepb-tb2-sequencer-runtime-closure"
)
ENV_FILE = pathlib.Path("/home/zephryj/projects/turingosv3/.env")
OUT = REPO / "handover/audits/GEMINI_TB_2_PHASE1C_AUDIT_2026-04-30.md"

key = None
for line in ENV_FILE.read_text().splitlines():
    if line.startswith("GEMINI_API_KEY="):
        key = line.split("=", 1)[1].strip()
        break
if not key:
    sys.exit("ERROR: no GEMINI_API_KEY in env")

PROMPT = """You are Gemini DeepThink, the strategic architectural reviewer for TuringOS v4.

This is **Phase-1c diff dual external audit** on the TB-2 experiment branch
`experiment/tb2-sequencer-runtime-closure`. The branch implements 5 atoms
(Atom 2 → Atom 6) on top of `f9ace5e` (which contains preflight v3 + charter
v3 + Phase-0 r1+r2 audit artifacts). Codex is running Phase-1c in parallel
with the implementer-paranoid angle. Your angle is **strategic /
architectural / constitutional**.

Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.

## Mandate

STEP_B_PROTOCOL Phase-1c gate (`handover/ai-direct/STEP_B_PROTOCOL.md` § 1c):
external auditors review the diff only. You answer 4 questions:

- Is the change *minimal*?
- Are tests sufficient?
- Does it introduce new constitutional debt?
- Any risk the diff itself is a Trojan (side-effects beyond scope)?

If both you and Codex PASS, the branch is merge-eligible. If either says
VETO, merge is blocked. If you disagree, conservative verdict (block) wins.

## Architectural constraints the implementation MUST honor (from preflight v3)

The preflight v3 (`handover/ai-direct/TB-2_SEQUENCER_RUNTIME_CLOSURE_2026-04-30.md`)
committed to the following hard constraints. Your job is to verify each is
honored in the actual diff:

1. **No ledger I/O inside `dispatch_transition`** (§3 + §8 line 1). The
   function returns `(q_next, signals)` or `Err(TransitionError)` only.
   Replay (Art IV Boot / P1:8) requires the transition function to be pure.
2. **No use of `economy::ledger::AcceptedLedger::append_accepted` on the
   production accepted spine** (§8 line 2). `AcceptedLedger` stays a TB-1
   RSP-0 primitive wrapper.
3. **No new `TypedTx` variants** (§8 line 3). Only TWO new `TransitionError`
   variants permitted: `EscrowMissing` + `MonetaryInvariantViolation`. The
   preflight rev'd this DOWN from r1's three-variant proposal — `StaleParent`
   already exists and must be reused.
4. **No non-empty `exempt_tx_kinds`** at the runtime call sites of
   `assert_total_ctf_conserved` (§8 line 4). Production must pass `&[]`.
5. **No widening of WalletTool mutation surface** (§8 line 5).
6. **No P5/P6/h_vppu/capability-metric work** inside this STEP_B branch
   (§8 line 6).
7. **No edits to `src/kernel.rs` / `src/bus.rs` / `src/sdk/tools/wallet.rs`**
   (§8 line 7) — the formal STEP_B-restricted set per CLAUDE.md.
8. **No use of `EscrowVault`** inside the WorkTx-arm escrow lookup (§8 line 8).
   Runtime reads from `q.economic_state_t.{escrows_t, task_markets_t}` only,
   via the `TxId(tx.task_id.0.clone())` bridge.
9. **Bridge line MUST carry an inline deletion comment** (§8 line 9) marking
   it for TB-3 deletion when `task_open_tx` lands.
10. **`Sequencer.rejection_writer` field shape**: `Arc<RwLock<RejectionEvidenceWriter>>`
    (preflight v3 §3.2 P0-1 r2). Plain `Arc<...>` would not compile because
    `append_rejected` is `&mut self`.
11. **TaskId vs TxId resolution = option (a) — bridge at lookup site**
    (§3.3 step 5 P0-B; user decision 2026-04-30). Not option (b) (q_state.rs
    index change) or (c) (`EscrowVault` injection).
12. **Acceptance battery 16 tests** (§5): 3 in-crate unit (U1-U3) + 13
    integration (I1-I13). Test 6 (post-init mint via WorkTx) DROPPED per
    Codex r1 P0-C (WorkTx carries no economic-delta field).

## Round-1 audit questions (8; strategic / architectural)

**Q1. Minimality vs scope**: does the diff exceed §3 minimum-sufficient
version? The preflight §3.1–§3.7 enumerates exactly what should land in
`src/state/sequencer.rs`. Are there any new helpers, abstractions, or
indirections beyond what §3 specified? Cite specific functions / structs.
The diff stat shows ~544 LOC added to sequencer.rs, ~13 LOC to typed_tx.rs,
~21 LOC to transition_ledger.rs, ~726 LOC for the new test file. Is that
proportional to the spec, or is there scope creep?

**Q2. `dispatch_transition` purity**: walk the WorkTx arm. Does it
contain any I/O, lock acquisition, writer call, CAS put, or other
side-effecting operation? Or is it a pure validation pipeline that
returns `(q_next, signals)` or `Err(TransitionError)`? If it touches any
locks (e.g. `q.read()`, `cas.write()`), that's a CHALLENGE — purity
must be preserved for replay determinism. Inspect the WorkTx arm
verbatim and quote evidence.

**Q3. `apply_one` rejection-path discipline**: on `Err(transition_err)`,
the path performs CAS-put + L4.E append. Verify:
- Does it advance `next_logical_t`? (Must NOT — K1 contract.)
- Does it write to `q` or `ledger_writer`? (Must NOT — Inv 7.)
- Does it use `envelope.submit_id` (not `next_logical_t`) as the L4.E key?
- Does it call `dispatch_transition` AGAIN after the rejection (e.g. to
  compute a different state)? (Must NOT.)
Quote relevant lines.

**Q4. `assert_total_ctf_conserved` call shape**: every runtime call site
inside the WorkTx arm MUST pass `&[]` as the third argument (§8 line 4).
Are there any non-empty exempt-list calls in the diff? Note: tests/
allowing non-empty exempt is OK; production runtime must not.

**Q5. New `TransitionError` variants — minimal set?**: the preflight rev'd
the new-variant count DOWN from 3 to 2 (`EscrowMissing` + `MonetaryInvariantViolation`)
because `StaleParent` already exists. Verify: does the diff add exactly 2
new variants? Are the corresponding `Display` arms also added (the impl is
exhaustive)? Does any test or runtime path use the obsolete names
`StaleParentRoot` / `PostInitMint`?

**Q6. `EscrowVault` non-use red line (§8 line 8)**: `grep -rn "EscrowVault\|escrow_vault" src/state/` should return zero hits. The bridge MUST read only from `q.economic_state_t.{escrows_t, task_markets_t}`. Verify in the diff.

**Q7. P0-B option (a) deletion-target comment (§8 line 9)**: the bridge line
in `dispatch_transition`'s WorkTx arm should carry an inline comment marking
it for TB-3 deletion. Quote the exact comment. If missing or vague (does not
mention TB-3 or `task_open_tx`), that's a CHALLENGE — preflight §8 line 9
explicitly required this.

**Q8. Replay invariant — does I13 actually prove P1:8 / Art IV Boot?**:
The test should:
- Submit one accepted WorkTx + one rejected WorkTx through `Sequencer::submit`.
- Capture the live sequencer's post-submission `state_root_t` and `ledger_root_t`.
- Reconstruct `QState` from canonical L4 transitions ALONE (via
  `replay_full_transition` reading the writer's entries).
- Assert reconstructed roots == sequencer's post roots.
- Confirm L4.E records did NOT influence the reconstruction.

Verify: does I13 (`tests/tb_2_runtime_boundary.rs::runtime_replay_from_l4_only_ignores_l4e`)
actually exercise this? Are there any cheats (e.g. test reads from L4.E
under the hood, or reuses sequencer's q_snapshot)?

## Verdict format

Section A: Overall verdict (PASS / CHALLENGE / VETO) with conviction (1-5).
PASS means the branch is cleared for merge to main pending Codex's verdict.

Section B: Per-Q1-Q8 disposition (one paragraph each + verdict tag + cite
file:line where possible).

Section C: New CONSTITUTIONAL DEBT introduced (if any). Each entry: file:line
+ what the debt is + why it matters + suggested remediation (BEFORE merge or
TB-3+).

Section D: TROJAN / scope-creep findings (if any). Each entry: file:line +
what's outside scope + why.

Section E: Recommendation — merge cleared / revise diff / abort branch.

Be direct. Cite file:line. The materials below include: the full diff
(`git diff f9ace5e..cf32735`), the preflight v3 (commitment), the charter
v3, the live source files post-diff (sequencer.rs, typed_tx.rs,
transition_ledger.rs), the integration battery, the smoke evidence.

---

# XREF materials follow.
"""

attachments = []

def append_file(label, path, fence="rust", root=WORKTREE):
    full = root / path
    if not full.exists():
        attachments.append(f"\n\n---\n\n## XREF: {label} — `{path}` [missing]\n")
        return
    attachments.append(f"\n\n---\n\n## XREF: {label} — `{path}`\n\n```{fence}\n{full.read_text()}\n```\n")

# Diff vs base.
diff = subprocess.run(
    ["git", "-C", str(WORKTREE), "diff", "f9ace5e..cf32735"],
    capture_output=True, text=True,
).stdout
attachments.append(f"\n\n---\n\n## XREF: diff `git diff f9ace5e..cf32735` (Phase-1c audit target)\n\n```diff\n{diff}\n```\n")

# Spec / preflight (commitment) — read from main repo (preflight is committed there).
append_file(
    "Preflight v3 (the commitment under audit)",
    "handover/ai-direct/TB-2_SEQUENCER_RUNTIME_CLOSURE_2026-04-30.md",
    fence="markdown",
    root=REPO,
)
append_file(
    "TB-2 charter v3",
    "handover/tracer_bullets/TB-2_charter_2026-04-30.md",
    fence="markdown",
    root=REPO,
)
append_file(
    "STEP_B protocol (Phase-1c contract)",
    "handover/ai-direct/STEP_B_PROTOCOL.md",
    fence="markdown",
    root=REPO,
)
# Source files — POST-DIFF state from the worktree (Phase-1c reviews implementation as it stands).
append_file("Post-diff sequencer.rs (worktree HEAD)", "src/state/sequencer.rs", fence="rust")
append_file("Post-diff typed_tx.rs (worktree HEAD)", "src/state/typed_tx.rs", fence="rust")
append_file(
    "Post-diff transition_ledger.rs (worktree HEAD; only the modified test arm changed)",
    "src/bottom_white/ledger/transition_ledger.rs",
    fence="rust",
)
append_file(
    "Integration battery (NEW file in this branch)",
    "tests/tb_2_runtime_boundary.rs",
    fence="rust",
)
# Pre-existing reference files (constraints).
append_file(
    "rejection_evidence.rs (existing — Sequencer integrates with this)",
    "src/bottom_white/ledger/rejection_evidence.rs",
    fence="rust",
    root=REPO,
)
append_file(
    "monetary_invariant.rs (existing — assert_* fns called by WorkTx arm)",
    "src/economy/monetary_invariant.rs",
    fence="rust",
    root=REPO,
)
append_file(
    "q_state.rs (existing — EconomicState + EscrowsIndex shape)",
    "src/state/q_state.rs",
    fence="rust",
    root=REPO,
)
# Smoke evidence.
append_file(
    "Phase-1 smoke evidence (pre-audit harness sanity)",
    "handover/evidence/tb_2_phase1_smoke_2026-04-30/README.md",
    fence="markdown",
)
# Commit log on the experiment branch.
log = subprocess.run(
    ["git", "-C", str(WORKTREE), "log", "--pretty=format:%h %s%n%n%b%n----", "f9ace5e..cf32735"],
    capture_output=True, text=True,
).stdout
attachments.append(f"\n\n---\n\n## XREF: Atom commit log (5 atoms; experiment branch)\n\n```\n{log}\n```\n")

full_prompt = PROMPT + "".join(attachments)
print(f"[gemini tb-2 phase1c] prompt size: {len(full_prompt)} chars", file=sys.stderr)

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
print(f"[gemini tb-2 phase1c] API returned in {elapsed:.1f}s", file=sys.stderr)

candidates = data.get("candidates", [])
if not candidates:
    sys.exit(f"No candidates: {json.dumps(data)[:500]}")
parts = candidates[0].get("content", {}).get("parts", [])
text = "".join(p.get("text", "") for p in parts)

OUT.parent.mkdir(parents=True, exist_ok=True)
git_head = subprocess.run(
    ["git", "-C", str(WORKTREE), "rev-parse", "HEAD"],
    capture_output=True, text=True,
).stdout.strip()
header = f"""# Gemini TB-2 Phase-1c Diff Dual External Audit
**Date**: 2026-04-30
**Target**: experiment branch `experiment/tb2-sequencer-runtime-closure` HEAD `{git_head}`
**Base**: `f9ace5e` (preflight v3 + charter v3 + Phase-0 audits)
**Prompt size**: {len(full_prompt)} chars
**API latency**: {elapsed:.1f}s
**Mandate**: STEP_B Phase-1c diff audit; strategic / architectural / constitutional (Q1-Q8). Independent of Codex Phase-1c (parallel, implementer-paranoid).

---

"""
OUT.write_text(header + text)
print(f"[gemini tb-2 phase1c] saved: {OUT}", file=sys.stderr)
