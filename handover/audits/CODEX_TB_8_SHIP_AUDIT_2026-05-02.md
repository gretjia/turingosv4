# Codex TB-8 Ship Audit - Class 3

Date: 2026-05-02
Scope: TB-8 Minimal Payout / FinalizeRewardTx on `main`, TB-7R baseline `4470036`.
Mandate source: `handover/audits/run_codex_tb_8_ship_audit_2026-05-02.sh` BRIEF_EOF heredoc, read directly with `cat`; script not executed.

Verdict rule: VETO > CHALLENGE > PASS.

## Audit Inputs

- Charter: `handover/tracer_bullets/TB-8_charter_2026-05-02.md`
- Ratification: `handover/audits/CHARTER_RATIFICATION_TB_8_2026-05-02.md`
- STEP_B preflight: `handover/audits/STEP_B_PREFLIGHT_TB8_2026-05-02.md`
- Recursive audit: `handover/audits/RECURSIVE_AUDIT_TB_8_2026-05-02.md`
- Smoke evidence: `handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/`
- Targeted local test run: `cargo test --test tb_8_minimal_payout` -> 11 passed, 0 failed.
- Full workspace test result not rerun by me; user-provided gate says `cargo test --workspace` = 723 passed / 0 failed / 150 ignored.

## Executive Result

Implementation-level FinalizeReward dispatch mostly does the intended escrow-to-balance money move and has direct Q-derived anti-forgery checks. However, ship is blocked by two Class-3 issues:

1. The committed TB-8 smoke evidence is not replayable from the packaged tarballs. A clean extraction of `runtime_repo.dotgit.tar.gz` + `cas.dotgit.tar.gz` fails before replay because required verifier sidecars are absent.
2. The claim writer permits multiple Open claims for the same WorkTx / escrow row. Because `assert_claim_amount_backed_by_escrow` is per-claim rather than aggregate, duplicate Confirm VerifyTxs can be accepted and then make every finalize fail once any payout would leave the other Open claim unbacked.

## RQ1 - Anti-Forgery Enforcement

Status: PASS for the direct mismatched-wire-field question.

`emit_system_tx(SystemEmitCommand::FinalizeReward { claim_id })` accepts only `claim_id` and Q-derives `task_id`, `solver`, and `reward` from `claims_t[claim_id]` before signing (`src/state/sequencer.rs:1666-1700`). Agent ingress rejects `TypedTx::FinalizeReward(_)` pre-queue (`src/state/sequencer.rs:1545-1555`). Apply stage 1.5 re-verifies system signatures before dispatch (`src/state/sequencer.rs:1897-1917`).

Defense in depth is present at dispatch: `FinalizeReward` rejects mismatched wire `reward`, `solver`, or `task_id` against the claim row with `SettlementPredicateFailed` predicate IDs `reward_matches_q_derived`, `solver_matches_q_derived`, and `task_id_matches_q_derived` (`src/state/sequencer.rs:816-842`). A forged tx with mismatched Q-derived fields cannot be accepted through the normal path, and a bypass with a valid-looking payload is still rejected at dispatch if the fields diverge from Q.

## RQ2 - CTF Conservation

Status: PASS for the single-claim finalize path.

The post-TB-8 holding sum is explicitly four holdings: balances, escrows, stakes, and challenge bond; `claims_t.amount` and `task_markets_t.total_escrow` are intentionally excluded (`src/economy/monetary_invariant.rs:111-172`). Finalize debits `escrows_t[claim.escrow_lock_tx_id]` by `claim.amount`, credits `balances_t[claim.claimant]` by the same amount, flips claim status, and decreases `task_markets_t.total_escrow` cache by the same amount (`src/state/sequencer.rs:866-915`). It then runs `assert_total_ctf_conserved`, cache=truth, and claim-backing checks (`src/state/sequencer.rs:916-938`).

Math: `escrows_t -= reward`, `balances_t += reward`, `claims_t` metadata only, `task_markets_t.total_escrow` cache only. Holding-sum delta is zero. The targeted integration test `finalize_reward_happy_path_debits_escrow_credits_solver_conserves_ctf` passed in my local `cargo test --test tb_8_minimal_payout` run.

## RQ3 - Replay Determinism / Packaged Evidence

Status: VETO.

I extracted the committed tarball pair for `single_n1_mathd_algebra_171` into a clean temp dir and attempted to replay:

```bash
RUN=handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171
WORK=/tmp/tb8_codex_replay_single_n1_mathd_algebra_171
mkdir -p "$WORK/runtime_repo" "$WORK/cas"
tar xzf "$RUN/runtime_repo.dotgit.tar.gz" -C "$WORK/runtime_repo"
tar xzf "$RUN/cas.dotgit.tar.gz" -C "$WORK/cas"
git -C "$WORK/runtime_repo" checkout -f refs/transitions/main
./target/debug/verify_chaintape --repo "$WORK/runtime_repo" --cas "$WORK/cas" --out "$WORK/replay_report.json"
```

Failure:

```text
verify_chaintape: bootstrap failed: pinned_pubkeys.json not found at "/tmp/tb8_codex_replay_single_n1_mathd_algebra_171/runtime_repo/pinned_pubkeys.json"
```

Root cause is visible in the smoke packager: it tars only `.git` for runtime and CAS (`scripts/run_tb8_smoke_2026-05-02.sh:73-79`). `verify_chaintape` requires `<runtime_repo>/pinned_pubkeys.json` before it can replay (`src/runtime/verify.rs:234-241`) and optionally reads `<runtime_repo>/initial_q_state.json` for the correct preseeded QState (`src/runtime/verify.rs:263-272`). It also uses `<runtime_repo>/rejections.jsonl` when present for L4.E chain accounting (`src/runtime/verify.rs:285-291`) and later needs agent pubkeys for WorkTx / VerifyTx signature checks.

The extracted transition ref's worktree has only:

```text
entry_canonical
payload_cid
signature
```

There are no committed sidecars in any TB-8 smoke run directory:

```text
find handover/evidence/tb_8_minimal_payout_smoke_2026-05-02 -maxdepth 2 \
  \( -name pinned_pubkeys.json -o -name initial_q_state.json -o -name rejections.jsonl -o -name agent_pubkeys.json \)
```

returned no files. The committed `replay_report.json` files were produced before tarball packaging while those sidecars still existed in the live worktree; the packaged evidence cannot reproduce them. This fails the brief's RQ3 and charter ship gate requiring replay from `runtime_repo.dotgit.tar.gz + cas.dotgit.tar.gz`.

Required surgical fix: package enough runtime/CAS evidence to satisfy `verify_chaintape` from a clean checkout, either by including the required sidecar files next to the tarballs or by committing them into a reproducible evidence tree and updating the README recipe. Then re-run the extraction + replay diff.

## RQ4 - Intent-vs-Backing Invariant

Status: VETO due aggregate duplicate-claim gap, despite the basic per-claim invariant existing.

The basic invariant exists and fires per Open claim: missing backing escrow is treated as amount 0, and `claim.amount > backing` returns `ClaimUnbacked` (`src/economy/monetary_invariant.rs:195-217`). The Verify OMEGA writer inserts a claim only after finding a task market and one `escrow_lock_tx_id` from `task_market.escrow_lock_tx_ids` (`src/state/sequencer.rs:596-635`), then runs `assert_claim_amount_backed_by_escrow` on `q_next` (`src/state/sequencer.rs:649-652`).

The blocker is aggregate backing. The code explicitly says a second VerifyTx targeting the same WorkTx creates a second claim entry, not a collision (`src/state/sequencer.rs:580-586`). There is no implemented gate that rejects a second Confirm for the same `work_tx_id`; `claim_id` is `claim-<verify.tx_id>` (`src/state/sequencer.rs:604-610`). The invariant then checks each Open claim independently against the same escrow row, not the sum of all Open claims sharing that escrow (`src/economy/monetary_invariant.rs:199-210`).

Reachable failure:

1. WorkTx accepted with one escrow row of 100.
2. VerifyTx A Confirm creates Open claim A for 100.
3. VerifyTx B Confirm on the same WorkTx creates Open claim B for 100, same `escrow_lock_tx_id`; per-claim backing passes because each 100 <= escrow 100.
4. Finalize of either claim would debit escrow to 0 and mark only that claim Finalized. The other Open claim remains amount 100 backed by escrow 0, so the post-finalize `assert_claim_amount_backed_by_escrow(&q_next)` fails (`src/state/sequencer.rs:934-938`), rejecting the payout.

This creates a malicious or accidental denial of payout for the first money-moving system tx. It violates the "exactly one FinalizeRewardTx" settlement-loop claim and is not merely a future multi-verifier feature, because `VerifyTx` is an agent-submitted surface today.

Required surgical fix: enforce one payable claim per `work_tx_id` or per backing escrow in the Atom-1 writer, and/or strengthen the invariant to assert aggregate Open claim amount per `escrow_lock_tx_id` <= escrow amount. Add a regression test where a second Confirm against the same WorkTx is rejected or cannot block finalize.

## RQ5 - Zero-Window MVP Gate

Status: PASS with one non-blocking documentation note.

The implemented gate fires only when `challenge_window_close_logical_t > 0` and `fr.timestamp_logical <= challenge_window_close_logical_t` (`src/state/sequencer.rs:778-789`). The writer sets the field literally to 0 (`src/state/sequencer.rs:620-633`), matching the ratification that zero is the "window closed immediately" MVP marker (`handover/audits/CHARTER_RATIFICATION_TB_8_2026-05-02.md:65-73`). This avoids comparing agent-controlled Verify timestamps to sequencer-controlled Finalize timestamps.

Non-blocking note: `src/state/q_state.rs:266-268` still says the zero-window value equals the OMEGA VerifyTx timestamp. The writer and ratification are correct; that stale comment should be fixed during the patch.

## RQ6 - Best-Effort Evaluator Emit

Status: PASS for TB-8 MVP semantics.

`tb8_emit_finalize_after_verify` derives `claim-<verify_tx_id>`, polls `q_snapshot()` until the claim exists, returns `Ok(false)` on poll-budget expiry, and emits FinalizeReward only after the claim is found (`src/runtime/adapter.rs:330-357`). Both evaluator OMEGA sites log `Ok(false)` / `Err` without failing the run (`experiments/minif2f_v4/src/bin/evaluator.rs:1925-1936`, `experiments/minif2f_v4/src/bin/evaluator.rs:2402-2411`).

This can leave a solver owed but unpaid if polling expires after the Confirm VerifyTx commits. For the ratified solo-run MVP this is acceptable as best-effort settlement follow-on, provided the Open claim remains on chain and a later emit/admin path can finalize. It becomes unacceptable once a production SLA or public user wallet expectation is introduced.

## RQ7 - Smoke Evidence Variety

Status: PASS for variety and dashboard semantics, but still blocked by RQ3 replay packaging.

The evidence README states 5/7 solved with chain-backed FinalizeReward and 2/7 unsolved with no fake Finalized claim across seven distinct heldout-49 problems (`handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/README.md:15-33`). I verified every run directory has `replay_report.json`, `dashboard.txt`, `runtime_repo.dotgit.tar.gz`, `cas.dotgit.tar.gz`, and `evaluator.log`.

Solved example `single_n1_mathd_algebra_171`: dashboard has L4 `FinalizeReward` at logical_t 5 (`dashboard.txt:49-60`) and a Finalized claim with payout 100000 (`dashboard.txt:82-88`). Unsolved examples `full_n1_mathd_algebra_11` and `full_n1_aime_1997_p9` have no FinalizeReward in proposal flow and show `(no Confirm-VerifyTx observed; n/a...)` in claims (`dashboard.txt:48-56`, `dashboard.txt:76-78` in each run). No fake Finalized rows were observed in the two unsolved runs.

## Other Acceptance Checks

- Flowchart trace row exists for TB-8 in `handover/alignment/TRACE_FLOWCHART_MATRIX.md:56`.
- STEP_B preflight artifact exists and documents the restricted `sequencer.rs` protocol substitute.
- Gemini parallel audit exists, but conflict policy gives this audit's VETO precedence.

## VERDICT: VETO

- RQ3 VETO: TB-8 smoke evidence tarballs are not independently replayable; clean extraction of `runtime_repo.dotgit.tar.gz + cas.dotgit.tar.gz` fails `verify_chaintape` before replay because `pinned_pubkeys.json` and other required sidecars are absent. Blocker refs: `scripts/run_tb8_smoke_2026-05-02.sh:73-79`, `src/runtime/verify.rs:234-241`, failed replay command above.
- RQ4 VETO: duplicate Confirm VerifyTxs for the same WorkTx can create multiple Open claims against the same escrow; per-claim backing passes, but finalization of any one claim is blocked by the remaining Open claim becoming unbacked. Blocker refs: `src/state/sequencer.rs:580-586`, `src/state/sequencer.rs:604-635`, `src/economy/monetary_invariant.rs:199-210`, `src/state/sequencer.rs:934-938`.
