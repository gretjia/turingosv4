#!/usr/bin/env bash
# Codex TB-8 ship audit ROUND-2 — post-remediation verification.
#
# Round-1 verdict (CODEX_TB_8_SHIP_AUDIT_2026-05-02.md): VETO with 2 blockers.
# Surgical remediation under feedback_elon_mode_policy round-2 auto-execute:
#
#   RQ3 BLOCKER FIX — smoke evidence packaging:
#     scripts/run_tb8_smoke_2026-05-02.sh:74-87 changed from tar.gz of .git/
#     only to tar.gz of full runtime_repo/ + cas/ directories. Required
#     verifier sidecars (pinned_pubkeys.json, agent_pubkeys.json,
#     initial_q_state.json, rejections.jsonl, genesis_report.json) are now
#     packaged. New filename suffix: runtime_repo.tar.gz / cas.tar.gz
#     (was runtime_repo.dotgit.tar.gz / cas.dotgit.tar.gz).
#     Spot-check: extracting the new tar.gz pair to a clean temp dir and
#     running verify_chaintape produces all-7-indicators-GREEN
#     replay_report.json with l4=5 (TaskOpen+EscrowLock+Work+Verify+
#     FinalizeReward) — matches the committed replay_report.json.
#
#   RQ4 BLOCKER FIX — duplicate-Confirm denial-of-payout:
#     src/state/sequencer.rs:540-650 Atom-1 writer now gates claim creation
#     on `q.economic_state_t.claims_t.0.values().any(|c| c.work_tx_id ==
#     verify.target_work_tx)` — a second Confirm targeting the same WorkTx
#     does NOT create a second claim row (the VerifyTx itself still accepts;
#     bond locks; verdict rides L4). 2 new regression tests in
#     tests/tb_8_minimal_payout.rs:
#       - duplicate_confirm_verify_does_not_create_second_claim (I130)
#       - finalize_succeeds_after_duplicate_confirm_attempt (I131)
#     Net new TB-8 test count: 13 (was 11; +2 regression).
#
# Workspace test count post-round-2: 725 / 0 / 150 (+13 net vs TB-7R 712).
#
# Non-blocking note from RQ5: src/state/q_state.rs:266-268 stale comment
# fixed in the round-2 patch (now correctly documents window=0 = MVP marker
# rather than = verify.timestamp_logical).

set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
OUT="${ROOT}/handover/audits/CODEX_TB_8_SHIP_AUDIT_R2_2026-05-02.md"
TMP_PROMPT="$(mktemp /tmp/tb8_codex_r2.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Codex TB-8 Round-2 Audit (post-remediation)

**Role**: same skeptical adversarial implementer-reviewer as round-1.
**Round cap**: 2 per `feedback_elon_mode_policy`. This is the final round.

## Round-1 verdict (closed-as-remediated)

Round-1 saved at `handover/audits/CODEX_TB_8_SHIP_AUDIT_2026-05-02.md`. Verdict was **VETO** with 2 blockers:

1. **RQ3** — smoke evidence tarballs not independently replayable; tar.gz of `.git/` only missed required verifier sidecars (`pinned_pubkeys.json` etc.); `verify_chaintape` failed at boot.
2. **RQ4** — duplicate Confirm VerifyTxs against the same WorkTx could create multiple Open claims sharing one escrow row; per-claim `assert_claim_amount_backed_by_escrow` passes individually, but post-finalize backing assertion fails — denial-of-payout.

## Round-2 surgical patches landed

**Patch 1 (RQ3)**: `scripts/run_tb8_smoke_2026-05-02.sh:74-87` — tar full `runtime_repo/` + `cas/` directories (was `.git/` only). New artifact filenames: `runtime_repo.tar.gz` + `cas.tar.gz`. Sidecars now packaged.

**Patch 2 (RQ4)**: `src/state/sequencer.rs:540-660` — Atom-1 writer adds one-claim-per-`work_tx_id` idempotency guard:

```rust
let already_claimed = q
    .economic_state_t
    .claims_t
    .0
    .values()
    .any(|c| c.work_tx_id == verify.target_work_tx);
if !already_claimed {
    /* existing claim-creation block */
}
```

The duplicate VerifyTx itself still accepts on L4 (bond locks); only the claim writer is suppressed.

**Patch 3 (RQ5 stale comment)**: `src/state/q_state.rs:266-275` — doc-comment now correctly describes `challenge_window_close_logical_t = 0` as the MVP marker (no longer says `= verify.timestamp_logical`).

**Regression tests** (`tests/tb_8_minimal_payout.rs:507+`):
- `duplicate_confirm_verify_does_not_create_second_claim` (I130)
- `finalize_succeeds_after_duplicate_confirm_attempt` (I131)

**Updated smoke evidence**: `handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/` — re-run with the round-2 binary; same 7 problems, same 5/7 SOLVED + 2/7 UNSOLVED shape, but now with self-contained tar.gz packaging.

**Updated README**: `handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/README.md` — references the new filename format.

## Round-2 verification mandate

Re-run only the audit checks tied to the 2 round-1 blockers + spot-check no regression elsewhere:

**RQ3 round-2**: extract `single_n1_mathd_algebra_171/runtime_repo.tar.gz + cas.tar.gz` into a clean temp dir; run `verify_chaintape`; confirm all 7 indicators GREEN + `economic_state_reconstructed=true` + `replay_failure=null`. Compare round-2 `replay_report.json` against the committed copy (modulo `run_id` + `epoch` runtime tags).

**RQ4 round-2**: read `src/state/sequencer.rs:540-660` Atom-1 writer; confirm the `already_claimed` guard exists and gates the claim-creation block. Read the 2 new regression tests (`tests/tb_8_minimal_payout.rs:I130 + I131`); confirm both pass via `cargo test --test tb_8_minimal_payout`. Confirm: a second Confirm targeting the same WorkTx accepts on L4 BUT does NOT create a second claim row.

**RQ1, RQ2, RQ5, RQ6, RQ7 carry-forward**: cite round-1 PASS; spot-check no regression. Round-2 changes are additive.

**Workspace test count gate**: cite `cargo test --workspace` = 725 / 0 / 150 (+13 net TB-8 tests vs TB-7R 712).

## Verdict format

End with one of:

```text
## VERDICT: PASS
(Round-1 VETOes RQ3 + RQ4 closed by round-2 surgical patches; ship is clean.)
```

```text
## VERDICT: CHALLENGE
(round cap exceeded; round-3 is not allowed per feedback_elon_mode_policy.
 Surface the residual concerns as ship-with-OBS proposals.)
```

```text
## VERDICT: VETO
(round cap exceeded; this is a hard block requiring user escalation.)
```

Save your audit to: handover/audits/CODEX_TB_8_SHIP_AUDIT_R2_2026-05-02.md.

BRIEF_EOF

echo "  Codex round-2 audit prompt prepared at: $TMP_PROMPT"
echo "  Output target: $OUT"
echo "  Run via: cat \"$TMP_PROMPT\" | codex-runtime > \"$OUT\""
