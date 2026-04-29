#!/usr/bin/env bash
# Codex round-1 dual external audit on TB-1 (Days 1-5; commits 063b003..6c04c26).
# Implementer-paranoid angle: do the 9 Tier-A tests actually PROVE the claimed
# P1 + P3 RSP-0 properties, or are there bugs/loopholes that let a malicious
# implementer pass all 9 while violating the invariant?
#
# Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.
# Dispositions: PASS (ship Day-7) | CHALLENGE (specific patches before ship)
#              | VETO (charter must change before retry per `feedback_phased_checkpoint`).

set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
OUT="${ROOT}/handover/audits/CODEX_TB_1_AUDIT_2026-04-29.md"
TMP_PROMPT="$(mktemp /tmp/tb1_codex_audit.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Codex Audit — TB-1 Days 1-5 (P1 GitTape + P3 RSP-0 + P6 h_vppu instrumentation)

**Role**: skeptical adversarial reviewer (implementer-paranoid angle).
Independent of Gemini round-1 (running in parallel; strategic/architectural angle).

**Mandate**: verify TB-1 ship readiness. TB-1 ships when ALL Tier-A tests
(1-9) are green AND the dual external audit clears. The ship gate per CF-5
"lighter option" downgrades P6 (Tier-B) to non-blocking; only P1 + P3 RSP-0
correctness gates the ship.

**Per Elon-mode 2-round cap (memory `feedback_elon_mode_policy`)**: this is
round 1. Round 2 only runs if round 1 returns CHALLENGE/VETO. Disposition:

- PASS  → TB-1 ships Day-7 (no further audit before merge)
- CHALLENGE → specific patches required before ship
- VETO  → charter must change before retry (per `feedback_phased_checkpoint`;
          NOT permitted on Tier-A kill criteria themselves)

## What to verify

The CENTRAL claim under audit:

> "These 9 Tier-A tests, taken together, prove that:
>   (P1) the v4 GitTape kernel honors the L4 / L4.E split — accepted txs
>        advance state on L4; rejected txs go to L4.E only; deleting any row
>        on either chain breaks its hash chain; agent-facing read views are
>        structurally isolated from L4.E raw diagnostics; and state.db is
>        bit-equal reconstructable from L4 alone.
>   (P3 RSP-0) total CTF is conserved across redistribution sequences; reads
>        are free for all K5 TxKinds; any post-init mint surfaces as
>        MonetaryError::PostInitMint and routes to L4.E (not L4)."

Your audit answers: **does the test battery prove this claim, or does it have
adversarial gaps that let a malicious implementer pass all 9 tests while
breaking the invariant?**

## Round-1 audit questions (5; implementer-paranoid)

**Q1. (P1 kill 1) Is `test_p1_kill_1_no_wtool_bypass` a real bypass test?**
The test:
  1. Builds a clean L4 chain of 3 entries; persists.
  2. Tampers state.db on disk (overwrites last entry's resulting_state_root with 0xFF...FF).
  3. Asserts `load_from_path` either returns Err OR reconstructs to a state_root that differs from the canonical pre-tamper root.

Adversarial check: imagine an implementer "fixes" the test to pass by adding
`if reconstructed != canonical { reconstructed = canonical }` in
`load_from_path` (silent rebase). Would the test still pass? Or does the
test's structure prevent that bypass?

Sub-question: the test only checks one bypass shape (mutated last entry's
resulting_state_root). What about a bypass that:
  a. Inserts a fake row at position 0 (replaces genesis with a fake)?
  b. Reorders rows (swap row 1 and row 2)?
  c. Mutates parent_state_root only (chain-prev field, not chain-next)?
Are any of these caught structurally by load_from_path's verify_chain, or
would the test miss them?

**Q2. (P1 kill 2 + P3 kill 1) Is the L4 / L4.E disjointness ACTUALLY tested?**
Both tests construct L4 + L4.E side-by-side, append a rejection to L4.E only,
and assert L4 didn't advance. But these tests construct the two ledgers
independently in the test fixture — they don't exercise a real
`dispatch_transition` path that ROUTES based on predicate results.

Adversarial check: a buggy `dispatch_transition` that writes to BOTH L4 and
L4.E on rejection (or to L4 on rejection by mistake) would still let these
tests pass, because the tests don't exercise dispatch_transition. The recharter
explicitly defers WorkTx dispatch_transition body to TB-2 RSP-1 — but does
that mean TB-1 is shipping the L4/L4.E split as scaffolding only? Should
this be made loud in the disposition (e.g., "Tier-A tests prove the
PRIMITIVES; disjointness ENFORCEMENT requires TB-2 dispatch_transition")?

**Q3. (P1 kill 4) Is `test_p1_kill_4_rejected_log_isolated` real isolation?**
The test asserts the `PublicRejectionView` JSON omits raw_diagnostic_cid.
But isolation depends on every code path that materializes
RejectedSubmissionRecord into agent-facing views going through
`PublicRejectionView` — not through `RejectedSubmissionRecord` directly.

Adversarial check: grep the codebase for direct serialization of
`RejectedSubmissionRecord`. If any agent-facing path (e.g., a future
WAL stream, a future librarian board, a future bus.snapshot()) serializes
the record directly, raw_diagnostic_cid leaks. Is there a structural enforcer
(e.g., `#[serde(skip_serializing)]` on raw_diagnostic_cid, or a sealed-trait
projection-only escape hatch), or is the discipline purely convention?

**Q4. (P3 RSP-0 Exit 1) Does the 5-step redistribution sequence test the right thing?**
The test models a closed system: alice 100 → bob 30 → escrow → bob → carol → back to baseline. At every step, `assert_total_ctf_conserved(prev, next, &[])` must succeed.

Adversarial check: a bug where `total_supply_micro` undercounts a sub-index
(e.g., forgets `claims_t` or `task_markets_t.bounty`) might still pass this
test if the test sequence never touches that sub-index. Does the 5-step
sequence exercise ALL 6 sub-indices that `total_supply_micro` counts
(balances, escrows, stakes, claims, task_markets bounty, challenge_cases bond),
or only some? If only some, then a regression in the un-exercised sub-indices
would slip through.

Also verify: the sequence uses a closed-system roundtrip (end == start).
Does this test catch a bug where total_supply_micro is constant-zero (e.g.,
an empty sum)? Probably not — both sides would equal zero.

**Q5. (P3 RSP-0 Exit 2 + P3 kill 1) Is `assert_read_is_free` a real
structural guard, or just a function that's called nowhere?**

Read `src/economy/monetary_invariant.rs` Q3.b. If `assert_read_is_free` is
defined but NEVER called from a real dispatch_transition / settlement
path, then test 8 is testing the function's own logic, not the property
"reads are free in the K5 economy". A future read-tx could carry a fee
of 1, the function would correctly return ReadCharged, but the dispatch
path would never call the function — silent bypass.

Same question for `assert_no_post_init_mint` and
`assert_total_ctf_conserved`. Are these wired into a real dispatch path
today? If not, what's the call-site landing TB?

## Round-1 audit questions (3; charter-discipline)

**Q6. Is the P6 Tier-B downgrade defensible per the recharter ship gate?**
The recharter Day-5 (per CF-5 lighter option) says: "TB-1 ships when all
Tier-A tests (1-9) are green. Tier-B tests (10-13) are captured as
artifacts but do not gate ship." But Tier-B 10-13 are non-trivial:
- T10 (mathd_algebra_107 evaluator solve) — verified out-of-band by Day-4
  evidence (commit 50a1d67; both n3 runs solved=true).
- T11 (L4 entry per dispatched tx) — explicitly deferred to TB-2 RSP-1.
- T12 (h_vppu non-null on 2nd run) — verified by Day-4 commit 50a1d67;
  unit test stub registered as #[ignore] in tb_1_acceptance.rs (cannot
  import minif2f_v4 from turingosv4 integration tests).
- T13 (econ_balance_delta non-zero) — needs RSP-1 wiring.

Question: does the Day-4 live evidence actually anchor T10 + T12, or are
those screenshots / log files outside the canonical test harness? Is "out
of band evidence" a defensible substitute for an in-harness assertion?

**Q7. Did Day-3 cleanly absorb the Day-2 monetary_invariant + escrow_vault?**
Day 2 commit 451cc66 shipped monetary_invariant.rs + escrow_vault.rs but
did NOT wire them into a dispatch path; Day 3 commit 846279f shipped
ledger.rs + rejection_evidence.rs. The recharter Day-5 acceptance battery
exercises monetary_invariant via direct fn calls (good) and exercises the
L4/L4.E primitives via direct AcceptedLedger / RejectionEvidenceWriter API
calls (good). But escrow_vault is NOT exercised by any Tier-A test. Is
that intentional? Should there be a Tier-A test for escrow lock + release
+ overpayout-reject before TB-1 ships?

**Q8. Are there NEW defects introduced by Day-4 + Day-5?**
Day-4 wired h_vppu into evaluator.rs main(); re-hashed Trust Root.
Day-5 added tests/tb_1_acceptance.rs and removed tests/tb_1_p1_acceptance.rs.

- Is the Day-4 evaluator.rs wire-up Goodhart-safe? Specifically, does the
  ordering `load → query → stamp → record → save` correctly EXCLUDE the
  current run's pput_verified from the ratio (held-out semantics)? Read
  the unit test `test_record_before_query_does_not_self_reference` — does
  it actually verify the production wire-up's ordering, or just the
  HVppuHistory API in isolation?
- Day-5 deleted tb_1_p1_acceptance.rs and re-added the 6 P1 tests in
  tb_1_acceptance.rs. Are the test bodies BIT-EQUAL to the Day-3 versions,
  or did the consolidation introduce a regression?

## Verdict format

Section A: Verdict (PASS / CHALLENGE / VETO) with conviction level.
Section B: Per-Q1-Q8 disposition (one paragraph each).
Section C: P0 list (must-fix-before-ship, if any).
Section D: P1 list (should-fix; can ship-with-OBS per Elon-mode policy).
Section E: Final disposition recommendation.

Be concrete. Cite file:line. Run grep / read commands as needed (you have
filesystem access via `view` / `apply_patch` etc.). Refute or confirm each
claim against actual code, not the spec wording.

---

# XREF materials follow.
BRIEF_EOF

# Append all relevant artifacts.
append() {
  local label="$1"; local path="$2"
  if [ -f "$path" ]; then
    printf '\n\n---\n\n## XREF: %s — `%s`\n\n```\n' "$label" "${path#$ROOT/}" >> "$TMP_PROMPT"
    cat "$path" >> "$TMP_PROMPT"
    printf '\n```\n' >> "$TMP_PROMPT"
  fi
}

append "TB-1 recharter (audit target spec)" "${ROOT}/handover/tracer_bullets/TB-1_recharter_2026-04-29.md"
append "L4 vs L4.E decision record" "${ROOT}/handover/alignment/DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md"
append "Tier-A acceptance battery (Day-5 final)" "${ROOT}/tests/tb_1_acceptance.rs"
append "monetary_invariant.rs (Day-2)" "${ROOT}/src/economy/monetary_invariant.rs"
append "escrow_vault.rs (Day-2)" "${ROOT}/src/economy/escrow_vault.rs"
append "ledger.rs (L4; Day-3)" "${ROOT}/src/economy/ledger.rs"
append "rejection_evidence.rs (L4.E; Day-3)" "${ROOT}/src/bottom_white/ledger/rejection_evidence.rs"
append "h_vppu_history.rs (Day-4)" "${ROOT}/experiments/minif2f_v4/src/h_vppu_history.rs"

# Day-4 evaluator wire-up snippet (just the relevant block).
printf '\n\n---\n\n## XREF: Day-4 evaluator.rs main() wire-up (lines ~322-385)\n\n```rust\n' >> "$TMP_PROMPT"
sed -n '320,395p' "${ROOT}/experiments/minif2f_v4/src/bin/evaluator.rs" >> "$TMP_PROMPT"
printf '\n```\n' >> "$TMP_PROMPT"

# Day-4 live evidence (JSONL rows + history file).
printf '\n\n---\n\n## XREF: Day-4 live wire-up evidence (TB-1 acceptance signal)\n\n' >> "$TMP_PROMPT"
printf '### RUN 1 JSONL (cold; no h_vppu — first run, omitted via Option::is_none):\n```json\n' >> "$TMP_PROMPT"
cat /tmp/tb1_day4_smoke_v2/run1.jsonl 2>/dev/null >> "$TMP_PROMPT" || echo "[evidence file missing]" >> "$TMP_PROMPT"
printf '\n```\n\n### RUN 2 JSONL (warm; h_vppu=6.2159...):\n```json\n' >> "$TMP_PROMPT"
cat /tmp/tb1_day4_smoke_v2/run2.jsonl 2>/dev/null >> "$TMP_PROMPT" || echo "[evidence file missing]" >> "$TMP_PROMPT"
printf '\n```\n\n### h_vppu_history.json after run 2:\n```json\n' >> "$TMP_PROMPT"
cat /tmp/tb1_day4_smoke_v2/h_vppu_history.json 2>/dev/null >> "$TMP_PROMPT" || echo "[evidence file missing]" >> "$TMP_PROMPT"
printf '\n```\n' >> "$TMP_PROMPT"

# Recent commit log (for context on what landed when).
printf '\n\n---\n\n## XREF: Recent commit log (TB-1 Days 1-5)\n\n```\n' >> "$TMP_PROMPT"
git -C "$ROOT" log --pretty=format:'%h %s' 063b003~1..HEAD >> "$TMP_PROMPT"
printf '\n```\n' >> "$TMP_PROMPT"

prompt_size=$(wc -c < "$TMP_PROMPT")
echo "[codex tb-1] prompt size: ${prompt_size} chars" >&2

t0=$(date +%s)
{
  printf '# Codex TB-1 Round-1 Dual External Audit\n'
  printf '**Date**: 2026-04-29\n'
  printf '**Target**: TB-1 Days 1-5 ship readiness (commits 063b003..%s)\n' "$(git -C "$ROOT" rev-parse --short HEAD)"
  printf '**HEAD**: %s\n' "$(git -C "$ROOT" rev-parse HEAD)"
  printf '**Prompt size**: %s chars\n' "$prompt_size"
  printf '**Mandate**: implementer-paranoid (Q1-Q8). Independent of Gemini r1 (parallel).\n\n---\n\n'
  codex exec --skip-git-repo-check < "$TMP_PROMPT" 2>&1
} > "$OUT"
t1=$(date +%s)
elapsed=$((t1 - t0))
echo "[codex tb-1] API returned in ${elapsed}s" >&2
echo "[codex tb-1] saved: $OUT" >&2
