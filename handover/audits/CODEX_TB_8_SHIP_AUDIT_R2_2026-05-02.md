# Codex TB-8 Ship Audit Round 2 - Class 3

Date: 2026-05-02
Scope: TB-8 Minimal Payout / FinalizeRewardTx post-remediation verification.
Mandate source: `handover/audits/run_codex_tb_8_round_2_audit_2026-05-02.sh` was read directly with `cat`; the shell script was not executed.

Round-1 source: `handover/audits/CODEX_TB_8_SHIP_AUDIT_2026-05-02.md`, which VETOed RQ3 and RQ4 and passed RQ1, RQ2, RQ5, RQ6, and RQ7.

## Checks Run

- Read current patch regions: `scripts/run_tb8_smoke_2026-05-02.sh:60-100`, `src/state/sequencer.rs:520-680`, `tests/tb_8_minimal_payout.rs:593-705`, and `src/state/q_state.rs:250-290`.
- Extracted `handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo.tar.gz` and `cas.tar.gz` into `/tmp/tb8_codex_r2`.
- Ran `target/debug/verify_chaintape --repo /tmp/tb8_codex_r2/runtime_repo --cas /tmp/tb8_codex_r2/cas --out /tmp/tb8_codex_r2/replay_report.json`.
- Normalized and diffed generated vs committed `replay_report.json` with `jq -S 'del(.run_id,.epoch)'`; diff exit was 0.
- Ran `CARGO_TARGET_DIR=/tmp/tb8_codex_r2_cargo_target cargo test --test tb_8_minimal_payout`; result was 13 passed, 0 failed.

I did not rerun `cargo test --workspace`. The workspace count is cited from observed repo content: the round-2 brief states `725 / 0 / 150` at lines 30 and 96, and `handover/alignment/TRACE_FLOWCHART_MATRIX.md:56` records the same count.

## RQ3 - Replay Determinism / Packaged Evidence

Status: PASS.

The round-2 packager now tars full directories: `scripts/run_tb8_smoke_2026-05-02.sh:73-87` documents the RQ3 fix and creates `runtime_repo.tar.gz` from `runtime_repo` and `cas.tar.gz` from `cas`, not `.git`-only tarballs.

The clean extraction contained the verifier sidecars that were missing in round 1: `pinned_pubkeys.json`, `agent_pubkeys.json`, `initial_q_state.json`, `rejections.jsonl`, and `genesis_report.json`, plus `agent_audit_trail.jsonl` and `synthetic_rejection_label.json`.

`verify_chaintape` exited 0 from the extracted tarballs. The generated report had `l4_entries=5`, `l4e_entries=2`, all seven replay indicators set to `true`, `economic_state_reconstructed=true`, `detail.replay_failure=null`, and `detail.initial_q_state_loaded_from_disk=true`. The normalized generated report matched the committed `single_n1_mathd_algebra_171/replay_report.json` exactly.

Round-1 RQ3 VETO is closed.

## RQ4 - Duplicate Confirm Denial of Payout

Status: PASS with one non-blocking documentation OBS.

The Atom-1 writer now has the required one-claim-per-work guard. In `src/state/sequencer.rs:596-612`, a Confirm VerifyTx computes:

```rust
let already_claimed = q
    .economic_state_t
    .claims_t
    .0
    .values()
    .any(|c| c.work_tx_id == verify.target_work_tx);
if !already_claimed {
```

The claim-creation block is inside that gate through `src/state/sequencer.rs:612-652`. The VerifyTx still accepts because the verifier stake mutation happens before the claim guard (`src/state/sequencer.rs:549-563`) and no rejection path is introduced by `already_claimed`.

The two new regressions cover the intended semantics. `I130` submits a second Confirm against the same WorkTx, asserts the Verify accepts, and asserts `claims_t` remains length 1 (`tests/tb_8_minimal_payout.rs:597-662`). `I131` repeats the duplicate Confirm attempt and then finalizes the first claim successfully (`tests/tb_8_minimal_payout.rs:664-705`). The targeted test run passed all 13 tests.

OBS: the nearby pre-existing comment at `src/state/sequencer.rs:580-586` still says a second VerifyTx targeting the same WorkTx would create a second claim entry. The executable guard and tests contradict that stale wording. This is not a ship blocker for round 2, but it should be cleaned up as documentation debt.

Round-1 RQ4 VETO is closed.

## Carry-Forward Spot Checks

RQ1 remains PASS. Current code still rejects `TypedTx::FinalizeReward(_)` at agent ingress (`src/state/sequencer.rs:1564-1569`), builds FinalizeReward from `claim_id` by Q-deriving `task_id`, `solver`, and `reward` (`src/state/sequencer.rs:1681-1704`), signs internally (`src/state/sequencer.rs:1710-1715`), and re-checks Q-derived `reward`, `solver`, and `task_id` in dispatch (`src/state/sequencer.rs:831-857`). Apply-stage system signature verification still runs before dispatch (`src/state/sequencer.rs:1912-1922`).

RQ2 remains PASS. The holding sum still counts exactly balances, escrows, stakes, and challenge bonds while excluding `claims_t.amount` and `task_markets_t.total_escrow` (`src/economy/monetary_invariant.rs:111-172`). Finalize still debits escrow, credits solver balance, flips claim status, updates the task-market cache, and runs conservation/cache/backing checks (`src/state/sequencer.rs:881-953`).

RQ5 remains PASS. The finalize gate fires only when `challenge_window_close_logical_t > 0` and the finalize logical time is still within that non-zero window (`src/state/sequencer.rs:793-803`). The stale comment called out in round 1 is fixed: `src/state/q_state.rs:266-275` now documents zero as the MVP "window-closed-immediately" marker and says agent-supplied `verify.timestamp_logical` is not used.

RQ6 remains PASS. `tb8_emit_finalize_after_verify` still polls for `claim-<verify_tx_id>`, returns `Ok(false)` if the claim does not appear within budget, and emits FinalizeReward only after the claim exists (`src/runtime/adapter.rs:330-357`). Both evaluator OMEGA sites log `Ok(false)` / `Err` as warnings rather than failing the run (`experiments/minif2f_v4/src/bin/evaluator.rs:1929-1936`, `experiments/minif2f_v4/src/bin/evaluator.rs:2404-2410`).

RQ7 remains PASS. The README records 5 solved runs with FinalizeReward / Finalized claims and 2 unsolved runs with no fake Finalized claim. Spot-check commands over current evidence found all seven replay reports with the seven replay booleans true, `economic_state_reconstructed=true`, and `replay_failure=null`; five dashboards contain `FinalizeReward`, and the two unsolved dashboards report `(no Confirm-VerifyTx observed; n/a -- claim_status / payout: n/a)`.

## Final Result

The two round-1 VETO blockers have determinate round-2 closure. RQ3 is now independently replayable from the packaged tarballs, and RQ4 no longer permits duplicate Confirm VerifyTxs to create duplicate payable claim rows for one WorkTx. The only residual finding is stale local commentary around the now-fixed duplicate-Confirm path, which is OBS-level documentation cleanup, not a functional blocker.

## VERDICT: PASS  (round-1 VETOes RQ3 + RQ4 closed by round-2 patches)
