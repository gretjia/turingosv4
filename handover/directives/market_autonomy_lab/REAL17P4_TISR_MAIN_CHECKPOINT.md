# REAL-17 P4 TISR-Main Checkpoint

Date: 2026-05-17

Status:

```text
checkpoint / not claim-bearing
```

## Context

REAL-17 was rebased onto latest `origin/main` after the TISR Phase 6.3 /
CAS Git commit-chain update. P4 then attempted to rerun a hard10 D-arm with
direct PromptCapsule provenance on the TISR-main worktree.

Current forward label remains:

```text
market emergence candidate -- final audit PROCEED, hardening pending
```

This checkpoint does not upgrade any E-level label.

## P4 Launch Failure

Evidence directory:

```text
handover/evidence/market_autonomy_lab_real17P4_tisr_main_hard10_direct_prompt_provenance_20260517T220627Z
```

Result:

```text
CONTAMINATED / NOT CLAIM-BEARING
```

Reason: the new TISR-main worktree shell environment did not have
`DEEPSEEK_API_KEY` loaded, so the runner failed before producing arm-D aggregate
evidence.

## P4b Partial Run

Evidence directory:

```text
handover/evidence/market_autonomy_lab_real17P4b_tisr_main_hard10_direct_prompt_provenance_20260517T220717Z/arm_D
```

Run log summary:

```text
problem_count       10
git_head            c430d744be152e9b5974d02b1af62894fe4f7c1f
batch_exit          1
audit_exit          2
persistence_exit    2
```

The batch reached 7 tasks and stopped at task 6 with evaluator exit code 3:

```text
EventResolve YES emit FAIL-CLOSED:
per-tactic WorkTx accept poll expired before VerifyTx/FinalizeReward
```

Post-run `audit_tape` and persistence report then failed because a stale
`cas/.turingos_cas_chain.lock` semaphore remained after the fail-closed exit.

## Diagnostic Verifier

A posthoc diagnostic verifier was run against a scratch copy of P4b CAS with the
stale lock excluded. This does not turn P4b into a complete hard10 run.

Verifier output:

```text
REAL17P4B_POSTHOC_E2_VERIFIER_COPYCAS.json
REAL17P4B_POSTHOC_E2_VERIFIER_COPYCAS.md
```

Observed diagnostic counts:

```text
verdict                                      PROCEED
exact_join_count                             10
l4_router_tx_count                           10
submitted_trace_tx_count                     10
scripted_fixture_tx_count                    0
policy_counts_for_e2                         false
direct_prompt_capsule_provenance_count       10
missing_direct_prompt_capsule_provenance     0
```

Claim boundary:

```text
diagnostic only; not a full hard10 replication; candidate-only boundary;
not proof of market emergence.
```

## Root Cause And Fix

Two in-envelope fixes were prepared before the replacement run:

1. REAL-6A success-path WorkTx accept polling now uses
   `TURINGOS_REAL6A_POLL_BUDGET_MS` instead of hard-coded `5000` milliseconds.
2. `scripts/run_g_phase_batch.sh` now performs runner-local stale CAS lock
   cleanup after `batch_evaluator` exits and before post-run audit/persistence.
   The cleanup is exact-path only, logs to `cas_stale_lock_cleanup.log`, checks
   the recorded `pid=...` with `kill -0`, preserves live locks, and does not
   delete or rewrite ChainTape/CAS/index evidence.

Trust Root was rehashed for the touched pinned evaluator file inside
`MARKET_AUTONOMY_LAB_RESEARCH_ENVELOPE_V2`.

## Verification Before Replacement Run

Fresh checks:

```text
cargo test --test constitution_real6_task_outcome_market -- --test-threads=1
  22 passed

cargo test --test constitution_real17_evaluator_prompt_provenance_wire \
  --test constitution_real17_market_decision_provenance_link \
  --test constitution_real14_e2_candidate_verifier \
  --test constitution_librarian_market_no_trade \
  --test constitution_real6_task_outcome_market -- --test-threads=1
  passed

cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo -- --test-threads=1
  passed

git diff --check
  passed

bash -n scripts/run_g_phase_batch.sh
  passed
```

## Next Step

Run replacement P4c in a new evidence directory after committing the fix so that
the evidence `git_head` binds to the source actually used by the hard10 runner.

Required claim boundary for P4c:

```text
E2 candidate pending audit
```

only if a live non-scripted exact-join agent tx exists with ChainTape/CAS
evidence, direct PromptCapsule provenance or equivalent direct link,
audit_tape PROCEED, voluntary action, price-as-signal only, real collateral, and
PolicyTrader/scripted action excluded from the candidate count.
