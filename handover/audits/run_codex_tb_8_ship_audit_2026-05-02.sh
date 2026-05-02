#!/usr/bin/env bash
# Codex TB-8 ship audit — Class 3 (auth-crypto-money: first system-emitted
# variant that *moves money* from escrows_t to balances_t).
# Implementation-paranoid angle. Independent of Gemini ship audit (parallel,
# architectural angle). Per memory feedback_dual_audit_conflict: VETO > CHALLENGE > PASS.

set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
OUT="${ROOT}/handover/audits/CODEX_TB_8_SHIP_AUDIT_2026-05-02.md"
TMP_PROMPT="$(mktemp /tmp/tb8_codex_ship.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Codex TB-8 Ship Audit — implementation-paranoid

**Role**: skeptical adversarial implementer-reviewer for the TB-8 (Minimal
Payout / FinalizeRewardTx) ship-gate dual external audit. Independent of
Gemini ship audit (parallel, architectural angle).

**Mandate**: Class 3 (auth-crypto-money — first system-emitted variant that
moves money from `escrows_t` to `balances_t`). Per memory
`feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS. Round cap = 2 per
`feedback_elon_mode_policy`.

## Audit target

Charter: `handover/tracer_bullets/TB-8_charter_2026-05-02.md`.
Ratification: `handover/audits/CHARTER_RATIFICATION_TB_8_2026-05-02.md`.
STEP_B preflight: `handover/audits/STEP_B_PREFLIGHT_TB8_2026-05-02.md`.

```text
TB-7R ship range:  9e74195..4470036  (TB-7R baseline 712/0/150)
TB-8 range:        4470036..HEAD     (this audit)
HEAD: cargo test --workspace = 723 passed / 0 failed / 150 ignored (+11 net)
```

Net-new for TB-8 (line ranges to anchor your reads):

1. **Atom 1 — claims_t writer + ClaimEntry schema**:
   - `src/state/q_state.rs:230-310` ClaimEntry 6-field expansion + ClaimStatus enum.
   - `src/state/sequencer.rs:540-600` Verify dispatch arm: OMEGA-Confirm path
     creates ClaimEntry { status: Open, amount: total_escrow, claimant: solver,
     escrow_lock_tx_id, work_tx_id, verify_tx_id, challenge_window_close_logical_t=0 }.
   - `src/economy/monetary_invariant.rs` 5→4 holding migration: claims_t
     removed from total_supply_micro (now intent registry). NEW
     `assert_claim_amount_backed_by_escrow` invariant + `ClaimUnbacked` variant.

2. **Atom 2 — SystemEmitCommand::FinalizeReward ingress**:
   - `src/state/sequencer.rs:935-960` SystemEmitCommand::FinalizeReward variant.
   - `src/state/sequencer.rs:1310-1410` build_signed_system_tx FinalizeReward arm
     (Q-derives task_id/solver/reward from claims_t[claim_id]; signs via runtime keypair).
   - `src/state/sequencer.rs:1430-1450` verify_emitted_system_tx_signature
     FinalizeReward arm (defense-in-depth: live verify against pinned pubkeys).
   - `src/state/sequencer.rs:990-1000` EmitSystemError::ClaimNotFound.

3. **Atom 3 — TypedTx::FinalizeReward dispatch arm**:
   - `src/state/sequencer.rs:632-790` 9-step dispatch body: parent-root match
     → claim lookup → idempotency (ClaimAlreadyFinalized / AlreadySlashed)
     → ChallengeWindow gate (window>0 strict-inequality; zero-window MVP no-op)
     → upheld-challenge gate (target_work_tx UpheldDeferred → SettlementPredicateFailed)
     → Q-derived consistency (reward/solver/task_id match wire) → escrow lookup +
     sufficiency → atomic mutation (escrow -= reward, balance += reward,
     status=Finalized, task_market.total_escrow -= reward) → 4 invariants
     (no_post_init_mint / total_ctf_conserved / cache=truth /
     claim_amount_backed_by_escrow) → state_root advance via FINALIZE_REWARD_DOMAIN_V1.
   - `src/state/typed_tx.rs:1024-1035` TransitionError::ClaimAlreadyFinalized.
   - `src/state/sequencer.rs:240-245` rejection_class_for: ClaimAlreadyFinalized
     + ChallengeWindowStillOpen → PolicyViolation (charter §4.5).

4. **Atom 4 — Evaluator OMEGA-branch caller**:
   - `src/runtime/adapter.rs:264-320` tb8_emit_finalize_after_verify(seq, vid, budget_ms)
     poll-then-emit helper (5s default budget; best-effort).
   - `experiments/minif2f_v4/src/bin/evaluator.rs` 2 OMEGA emit sites
     (full-proof + per-tactic) call the helper after VerifyTx commits.

5. **Atom 5 — ChainTape smoke evidence (real LLM, variety)**:
   - `handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/` — 7 runs across
     5+ distinct heldout-49 problems (mathd_algebra_171/107/359/10/11,
     mathd_numbertheory_961, aime_1997_p9). Each run: replay_report.json +
     dashboard.txt + runtime_repo.dotgit.tar.gz + cas.dotgit.tar.gz +
     evaluator.log. New TB-8 indicator: `claim_finalized_chain_backed`
     observable as ≥1 `Finalized` row in dashboard.txt §9 Claims AND
     ≥1 FinalizeReward row in dashboard.txt §5 Proposal flow.

6. **Atom 6 — Audit-dashboard claim_status + payout_amount**:
   - `src/bin/audit_dashboard.rs:95-110` ClaimAuditRow struct.
   - `src/bin/audit_dashboard.rs:680-740` §9 TB-8 Claims section in render_text.

7. **Atom 7 — this audit (you).**

## Acceptance contract (charter §5 ship gates)

```text
1. cargo test --workspace = 723 passed / 0 failed / 150 ignored
   (+11 TB-8 tests vs 712 baseline; canonical reporting per
    feedback_workspace_test_canonical).

2. STEP_B preflight artifact: handover/audits/STEP_B_PREFLIGHT_TB8_2026-05-02.md
   exists; documents Phase-0 necessity audit + Phase-1 implementation-audit
   checkpoints + Phase-2 statistical-validity substitute.

3. ChainTape smoke evidence: 7 runs in handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/.
   Per SOLVED run: ≥1 FinalizeReward L4 row + ≥1 Finalized claim row in
   dashboard §9 + replay_report.json all 7 indicators GREEN +
   replayable from committed runtime_repo.dotgit.tar.gz + cas.dotgit.tar.gz.

4. Conservation invariant holds:
   - assert_total_ctf_conserved(&[]) PASS at FinalizeReward dispatch
     (escrow -reward + balance +reward = 0 delta on the holding sum;
      claims_t is intent registry post-TB-8).
   - assert_claim_amount_backed_by_escrow PASS post-mutation.

5. Idempotency:
   - re-emit FinalizeReward for same claim_id → ClaimAlreadyFinalized
     (test I118 in tests/tb_8_minimal_payout.rs).
   - replay determinism: same chain → same claims_t snapshot
     (test I103).

6. Anti-Oreo barrier preserved:
   - TypedTx::FinalizeReward(_) cannot be agent-submitted via
     submit_agent_tx (test I121: SubmitError::SystemTxForbiddenOnAgentIngress
     + foundation TB-5 RSP-3.0 inheritance).
   - emit_system_tx is the only construction path; all signatures derived
     from runtime keypair internally.

7. Class 3 dual external audit at strategic tier (this + Gemini parallel).

8. No regression on TB-7R 4-clause acceptance:
   - clause 1 still GREEN under three-node taxonomy.
   - clause 2 still GREEN (predicate evidence resolves).
   - clause 3 still GREEN (failed shielded).
   - clause 4 still GREEN (dashboard regeneratable from ChainTape + CAS).

9. Flowchart-trace declared: TB-8 row added at Atom 8 to
   handover/alignment/TRACE_FLOWCHART_MATRIX.md.
```

## Specifically scrutinize (RQ1-RQ7)

**RQ1 — Anti-forgery enforcement**: A forged FinalizeRewardTx where
wire `reward / solver / task_id` differ from claims_t[claim_id] — does the
dispatch arm reject? Trace src/state/sequencer.rs Atom-3 step 5.

**RQ2 — CTF conservation**: Walk the math. At FinalizeReward dispatch:
- escrows_t row decreases by `reward`.
- balances_t row increases by `reward`.
- claims_t.amount unchanged (only status flips to Finalized; not in supply sum).
- task_markets_t.total_escrow decreases by `reward` (cache=truth).
- 4 holdings sum: pre = post = pre. PASS?

**RQ3 — Replay determinism**: Two replays of the same chain must produce
byte-identical claims_t + balances_t + escrows_t. Verify via the smoke
evidence: extract runtime_repo.dotgit.tar.gz to a fresh dir, run
verify_chaintape, diff vs committed replay_report.json. Demonstrate
end-to-end CID resolution.

**RQ4 — Intent-vs-backing invariant**: assert_claim_amount_backed_by_escrow
fires when an Open claim's amount > backing escrow row amount. Does the
TB-8 5→4 holding migration introduce any path where claim is created
without a backing escrow row? Charter §3 Atom 1: writer requires
escrow_lock_tx_id resolves at claim-creation; if not, claim NOT created.

**RQ5 — Window-namespace correctness** (per ratification §2.4): the gate
fires only when window > 0 AND fr.timestamp_logical <= window. For TB-8
MVP (window=0), gate is no-op. Is this safe? Argument: structural ordering
(claim must exist for finalize to dispatch) replaces the time-based gate
in MVP. Forward-compat: a future TB introducing real window timing sets
window to a non-zero sequencer-namespace logical_t at claim-creation.

**RQ6 — Best-effort evaluator emit**: `tb8_emit_finalize_after_verify`
returns `Ok(false)` on poll-budget expiry without failing the run. Is this
safe? Argument: L4 OMEGA evidence is the durable signal; the
FinalizeReward emit is a follow-on settlement. A run with OMEGA but no
FinalizeReward is not a regression — the next session (or a future
admin-emit path) can finalize from the on-chain claim. **But** does this
create a path where solver is OWED money but never paid? Yes — the claim
stays Open. Is this acceptable for TB-8 MVP? Charter says yes (single-
solver MVP, no royalty, no time pressure). Confirm.

**RQ7 — Smoke evidence variety**: 7 runs spanning 5+ distinct heldout-49
problems is the user-required variety. Verify each run dir exists +
contains the 4-file shape (replay_report.json, dashboard.txt,
runtime_repo.dotgit.tar.gz, cas.dotgit.tar.gz). Verify ≥1 SOLVED run has
the new TB-8 §9 §5 chain-backed FinalizeReward indicator. Verify
UNSOLVED runs do NOT show fake Finalized claims (no fake accepted node).

## Verdict format

End your audit with one of:

```text
## VERDICT: PASS
(All RQ1-RQ7 cleared; ship is clean.)
```

```text
## VERDICT: CHALLENGE
- <claim id> CHALLENGE: <one-line reason + line refs>
- <repeat per challenge>
(round-2 will trigger feedback_elon_mode_policy auto-execute on
determinate-best surgical patch.)
```

```text
## VERDICT: VETO
- <claim id> VETO: <one-line BLOCKING reason + line refs>
(VETO blocks ship per feedback_dual_audit_conflict; VETO > CHALLENGE > PASS.)
```

Save your audit to: handover/audits/CODEX_TB_8_SHIP_AUDIT_2026-05-02.md.

BRIEF_EOF

# Pipe through codex-runtime per the codex plugin contract.
echo "  Codex audit prompt prepared at: $TMP_PROMPT"
echo "  Output target: $OUT"
echo "  Run via: cat \"$TMP_PROMPT\" | codex-runtime > \"$OUT\""
echo ""
echo "  (Manual invocation expected; this script prepares the prompt only.)"
