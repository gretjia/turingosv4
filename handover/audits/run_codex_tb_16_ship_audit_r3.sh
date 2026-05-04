#!/usr/bin/env bash
# Codex TB-16 R3 ship audit — implementation-paranoid review of R3 surgical
# fixes for Gemini R2 VETO + CHALLENGEs. Independent of Gemini R3 (parallel).
# Per memory feedback_dual_audit_conflict: VETO > CHALLENGE > PASS.
# Per memory feedback_audit_loop_roi_flip: stop iterating if challenges shift
# to test-scaffold edges.

set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
ROUND="${TB16_AUDIT_ROUND:-R3}"
OUT="${ROOT}/handover/audits/CODEX_TB_16_SHIP_AUDIT_2026-05-04_${ROUND}.md"
TMP_PROMPT="$(mktemp /tmp/tb16_codex_ship_r3.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

if [ -e "$OUT" ]; then
  echo "[codex tb-16 r3] error: $OUT already exists; refusing to overwrite" >&2
  exit 2
fi

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Codex TB-16 Round 3 Ship Audit — implementation-paranoid

**Role**: skeptical adversarial implementer-reviewer for the **TB-16
ROUND 3** (R3) of the Class 3 ship-gate dual external audit. Independent
of Gemini R3 (parallel; architectural strategic angle).

**Mandate**: per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.
**This is the convergence round** — R1 saw VETO×5 from you; R2 was Codex-skipped;
R3 audits R3 surgical fixes. Per `feedback_audit_loop_roi_flip`: if your
R3 challenges shift to test-scaffold edges (heuristic classifiers, marker-
discipline subtleties), iteration ROI has flipped — note that explicitly.

## R1 → R2 → R3 history (for grounding)

```text
R1 (your prior round; commit 3300fe2 = Atom 6 ship pre-audit):
  VETO×5 (V2 sandbox-canonical, V3 audit_pipeline_smoke fixture,
  V4 BLOCK exit, V5 destructive tamper, V6 conservation drift,
  V7 Markov chain link)
  Output: handover/audits/CODEX_TB_16_SHIP_AUDIT_2026-05-04_R1.md

R2 (Gemini-only; Codex skipped):
  Gemini VETO×5 (4/5 stale post-Step 4: Q5/Q8/Q9; 1 real Q2 JSON byte-run)
  Gemini CHALLENGE×5 (Q1 per-block conservation, Q3 replay parity,
  Q6 Class 3 misclass, Q7 tamper attack-vector coverage,
  Q10 machine-verifiable CR-16.7, Q11 TRACE_MATRIX precision,
  Q12 test-count math)
  Output: handover/audits/GEMINI_TB_16_SHIP_AUDIT_2026-05-04_R2.md
  Closure: handover/audits/RECURSIVE_AUDIT_TB_16_R2_2026-05-04.md

Step 1 commit 3cf4c36 (closes V3-V7 + Q11 + V2-audit):
  • Audit calls production `monetary_invariant::total_supply_micro`
    directly (V6 closure; eliminates 4-vs-5 holding drift)
  • `sandbox_prefix` accepts `Agent_<digit>` canonical preseed (V2-audit)
  • #18 conservation: FINAL == INITIAL (per-chain, not hardcoded 30M)
  • Tamper: pre-tamper PROCEED baseline + destructive zero-back-half
    corruption (V5 closure; 3/3 TRUE detection on PROCEED-baseline)
  • Strip `|| true`; runner exits non-zero on BLOCK / replay divergence (V4)
  • `--prev-cid-hex` from LATEST_MARKOV_CAPSULE.txt (V7 closure)
  • Q11: tamper assertions #36-#38 backlinks → FC1-N35

Step 3 commit 05e3e86 (evaluator arena hooks):
  • 3 env-var triggers in evaluator: TURINGOS_FORCE_CHALLENGER (FR-16.3) +
    TURINGOS_COMPLETE_SET_SEED (FR-16.4) + TURINGOS_FORCE_BANKRUPTCY (FR-16.7)
  • 3 new real-signature constructors in src/runtime/adapter.rs:
    ChallengeTx + MarketSeed + CompleteSetMint

Step 4 commit d1c1af2 (fresh real-LLM arena runs + TB-11 writer-pattern bug fix):
  • arena_run4/ (mathd_algebra_171, happy): PROCEED 7 tx kinds (Work +
    Verify + Challenge + TaskOpen + EscrowLock + CompleteSetMint + MarketSeed)
  • arena_run6_exhaust/ (aime_1997_p9): PROCEED 4 tx kinds (TaskOpen +
    EscrowLock + TerminalSummary + TaskBankruptcy)
  • Aggregate: 9 of 13 architect-required tx kinds
  • CRITICAL FIX: src/runtime/evidence_capsule.rs::write_evidence_capsule
    had the same writer-pattern bug Codex caught in TB-15 R2 (capsule_id =
    sha256(prelim) but cas.put stored populated bytes; cas.get(capsule_id)
    always failed). Forward-only fix per feedback_no_retroactive_evidence_rewrite.
  • Discovered live in arena_run5 audit Layer E #27 halt; verified by run6 PROCEED.

R3 prep commit 90848bb (THIS COMMIT YOU ARE AUDITING):
  • Q2 (Gemini R2 VETO; JSON byte-run) — assert_28_projection_no_autopsy_bytes
    now scans BOTH (a) raw 32-byte run in canonical_encode AND (b) JSON-array
    decimal text form via serde_json::to_string(&proj). Mirrors TB-15
    halt-trigger #5 (R2).
  • Q1 (Gemini R2 CHALLENGE; per-block conservation) — NEW
    assert_d_total_supply_conserved_per_block (id=40, Layer D supplemental).
    Replays entries[..=i] for every i; asserts total_supply_micro == initial
    at every step. O(N²) tolerable for chain sizes 5-30.
  • Q10 (Gemini R2 CHALLENGE; machine-verifiable CR-16.7) — NEW
    assert_a_chain_agent_ids_sandbox_prefixed (id=41, Layer A supplemental).
    Walks every L4 entry, decodes TypedTx, calls HasSubmitter::submitter_id(),
    asserts sandbox_prefix.
  • Q11 (Gemini R2 CHALLENGE; TRACE_MATRIX precision) — file-level
    doc-comment per-layer precise (Layers A-G + supplementals → FC1-N34;
    Layer H → FC1-N35; verdict.json → FC2-N31).
  • Q12 (Gemini R2 CHALLENGE; test-count math) — TB-16 SHIP_STATUS §3
    expanded with per-step delta from TB-15 R3 (882) → TB-16 R3 (907).
  • Q8 evidence regen (closure) — audit_pipeline_smoke MARKOV_TB-16
    regenerated with --prev-cid-hex chained to TB-15 head f9e701b4...
  • Q4 position held (carry-forward to architect ratification): "non-sandbox
    funds used" HALT is parallel-structurally audit-time per architect §7.7,
    NOT sequencer-level admission gate.
```

## OBS-deferred (NOT a R3 ship-blocker; pre-existing)

`audit_tape_tamper` Round 2 hangs on the `audit_pipeline_smoke` fixture.
**Verified pre-existing on git HEAD via `git stash` test**: reproduces
without any R3 fix changes. R1 ship-time tamper_report.json (3/3 detected,
committed `3cf4c36`) carry-forward valid since tamper LOGIC in
src/bin/audit_tape_tamper.rs was UNTOUCHED in R3.

Hypothesis: post-tamper audit_tape pipeline iterates a CAS loose object
whose back-half-zero corruption decodes (via git2 zlib partial) into bytes
that a bincode length-prefix path treats as unbounded. Triage to TB-16.x.

See: `handover/alignment/OBS_TB_16_TAMPER_R2_HANG_2026-05-04.md`

## R3 audit questions — focused on R3 surgical fixes

You have already audited R1. Do NOT re-litigate R1 questions. Focus on:

**RQ1 — Q2 JSON-form check completeness**: `assert_28_projection_no_autopsy_bytes`
in `src/runtime/audit_assertions.rs:1483-1573` (post R3) now does:
- (a) raw 32-byte window scan in `canonical_encode(&q.tape_view_t)`
- (b) full JSON-array decimal text form scan in `serde_json::to_string(&q.tape_view_t)`

CHALLENGE: is the full-array form `[b₀,b₁,…,b₃₁]` the ONLY way Cid bytes
could leak through serde_json? What about (i) base64-encoded form (some
serde_json variants), (ii) hex-string form, (iii) tuple-array
`(b₀,b₁,…,b₃₁)`? Walk `AgentVisibleProjection`'s serde derive +
`ProjectionWriter` impl in `src/runtime/projection.rs` (or wherever it
lives). Confirm the only `Cid` serialization path on the agent-visible
boundary is the JSON-array decimal form. If not, what other paths leak?

**RQ2 — Q1 per-block walker semantics**: `assert_d_total_supply_conserved_per_block`
(id=40) in `src/runtime/audit_assertions.rs:1199-1259`. Replays
`entries[..=i]` for every i. CHALLENGE: (a) does this catch supply drift
that NETS to zero across the prefix (e.g., +5M μC at index 2, -5M μC at
index 4)? Yes, because every PARTIAL prefix is checked. Confirm. (b) is
the O(N²) replay actually safe — does `replay_full_transition` have any
non-deterministic state (RNG seed, env-var read, time-dependent path)
that could produce different total_supply_micro on re-replay? (c) for
chains where some predicate dispatch fails (e.g., `Err(StateRootMismatch)`
mid-chain), does the walker correctly halt on the first error vs silently
propagating? Check the early-return path.

**RQ3 — Q10 sandbox-prefix walker correctness**: `assert_a_chain_agent_ids_sandbox_prefixed`
(id=41) in `src/runtime/audit_assertions.rs:657-712`. Walks L4, decodes
TypedTx, calls `HasSubmitter::submitter_id()`. CHALLENGE: (a) for system-
emitted tx (FinalizeReward, TaskExpire, TerminalSummary, ChallengeResolve,
TaskBankruptcy, Reuse), `submitter_id()` returns `None` and walker SKIPS
the check. Is that correct, or should system-emitted tx ALSO be checked
for sandbox prefix on whatever AgentId field they DO carry (e.g.,
`FinalizeRewardTx.target_agent`)? (b) `sandbox_prefix` admits "system" —
but `submitter_id()` returns `None` for system tx, so the "system"
admission rule is dead code at this call site. Audit. (c) the walker
returns `Skipped` if no agent-signed tx in tape — should this be `Halt`
instead (architect §7.7 "non-sandbox funds used" → halt-on-empty)?

**RQ4 — Q4 position-hold**: I (the implementer) HELD POSITION that
"non-sandbox funds used" HALT in architect §7.7 is parallel-structurally
audit-time (Layer A #3 manifest fence + new id=41 chain-walk), NOT a
sequencer-level admission gate. Codex R1 V2 + Gemini R2 Q4 read it as
runtime gate. CHALLENGE this position. Cite architect §7.7 verbatim
parsing. Is "audit-time HALT" semantically equivalent to "sequencer
admission gate" given that audit-time HALT only triggers post-hoc?

**RQ5 — Q12 test-count math accuracy**: TB-16 SHIP_STATUS §3 (post R3
edit) shows the per-step delta table from TB-15 R3 ship (882) → TB-16
SHIPPED pre-audit (905) → R1 Step 1 (907) → R3 (907). CHALLENGE:
verify the +25 / +2 deltas are arithmetically correct. Are there any
SUBTRACTIVE deltas (test deletions) that the table omits? Run `cargo
test --workspace --no-fail-fast` mentally against `tests/` + `src/` test
counts. (You may verify against `git log -- tests/` if needed.)

**RQ6 — OBS deferral justification**: `OBS_TB_16_TAMPER_R2_HANG_2026-05-04.md`
defers tamper Round 2 hang. CHALLENGE: (a) is the "pre-existing on git
HEAD via stash test" verification rigorous, or could the hang be triggered
ONLY by R3-introduced state in the smoke fixture's CAS dir (e.g., my
regenerated MARKOV_TB-16 capsule's bytes happen to corrupt into a
hang-triggering state)? (b) is the carry-forward of R1 tamper_report.json
(3/3 detected) actually valid evidence at R3 ship, given that the smoke
fixture's CAS state has changed (my MARKOV_TB-16 regen wrote new bytes to
cas/.git/objects/e8/...)? Per `feedback_no_retroactive_evidence_rewrite`:
is the carry-forward grandfathered or invalidated?

**RQ7 — Supplemental id collisions**: NEW supplemental ids 40 + 41 added
alongside existing 39 (`assert_f_no_llm_self_narrative_in_autopsy`). Is
the id namespace correctly partitioned (1-38 = primary, 39+ = supplemental
appended)? Are downstream consumers (verdict.json schema, dashboard
render, R-022 tooling) tolerant of supplemental ids, or is there a hard-
coded `if id <= 38` somewhere that would silently drop the new walkers?

**RQ8 — TB-11 writer-pattern bug retro impact**: Step 4 commit `d1c1af2`
fixed `src/runtime/evidence_capsule.rs::write_evidence_capsule` (TB-11
latent bug since 2026-05-02). All chains that ever fired
TerminalSummaryTx + EvidenceCapsule pre-fix have `cas.get(capsule.capsule_id)`
returning Err. CHALLENGE: per `feedback_no_retroactive_evidence_rewrite`
the fix is forward-only, but what about chains that need to be REPLAYED
for verification post-fix (e.g., R3 audit re-replay of Step 4 evidence)?
Does the audit_tape battery handle the "old chain stored buggy bytes,
new fix expects correct bytes" mismatch via grandfathering, or does it
HALT on Layer E #27 for old chains? Check.

## Verdict format (R3 convergence-round structure)

End your audit with one of:

```text
## VERDICT: PASS
(All RQ1-RQ8 cleared at convergence; R3 surgical fixes successfully closed
R2 findings; ship is clean for Class 3 envelope.)
```

```text
## VERDICT: CHALLENGE
- RQ<id> CHALLENGE: <one-line reason + line refs>
(round-cap reached at R3; per feedback_audit_loop_roi_flip +
feedback_audit_obs_bias, evaluate ship-with-OBS fitness for residuals.)
```

```text
## VERDICT: VETO
- RQ<id> VETO: <one-line BLOCKING reason + line refs>
(VETO at R3 per feedback_dual_audit_conflict; recommend escalate to
architect ratification or revert R3 prep.)
```

Include conviction (low/medium/high) + recommendation (PROCEED to SHIP /
SHIP-WITH-OBS / FIX-THEN-PROCEED / REDESIGN / RETRO-CLASS-4-PROMOTION).

Cite `file:line` for every finding.

Save your audit to: handover/audits/CODEX_TB_16_SHIP_AUDIT_2026-05-04_R3.md.

BRIEF_EOF

echo "[codex tb-16 r3] prompt prepared at: $TMP_PROMPT" >&2
echo "[codex tb-16 r3] output target: $OUT" >&2
echo "[codex tb-16 r3] round: $ROUND" >&2
echo "[codex tb-16 r3] invoking codex exec..." >&2

cat "$TMP_PROMPT" | codex exec --skip-git-repo-check --sandbox read-only --color never - > "$OUT.raw" 2>&1
EXIT=$?

if [ $EXIT -ne 0 ]; then
  echo "[codex tb-16 r3] codex exec returned exit code $EXIT" >&2
  echo "[codex tb-16 r3] partial output saved to $OUT.raw" >&2
fi

mv "$OUT.raw" "$OUT"
echo "[codex tb-16 r3] audit saved: $OUT" >&2
exit $EXIT
