# REAL-7 V3-Equivalent Structural Smoke Report

## Scope

REAL-7 does not claim v3-identical volume or spontaneous market emergence. It
tests the architect-required structural pressure pattern:

```text
>= 5 agents
>= 3 roles active
>= 3 tasks
>= 1 TaskOutcomeMarket
>= 1 scripted AttemptPredictionMarket
>= 1 BuyYesWithCoinRouterTx
>= 1 BuyNoWithCoinRouterTx or Short equivalent
>= 1 VerifyTx
>= 1 ChallengeTx or NoChallengeReason
>= 1 EventResolveTx
>= 1 PnL delta
>= 1 AutopsyCapsule if loss occurs
```

## Planned Evidence Command

```bash
PHASE_D_HETERO_OK=1 \
TURINGOS_G_PHASE_DIRTY_OK=1 \
TURINGOS_G_PHASE_LOW_DISK_OK=1 \
TURINGOS_G_PHASE_N_AGENTS=5 \
TURINGOS_REAL5_ROLE_ASSIGNMENT=Solver,Trader,Verifier,Challenger,Observer \
TURINGOS_REAL5_ROLE_VIEWS=1 \
TURINGOS_REAL6_TASK_OUTCOME_MARKET=1 \
TURINGOS_REAL6_SCHEDULER_OBSERVE_ONLY=1 \
TURINGOS_REAL7_SCRIPTED_TASK_OUTCOME_BUYS=Agent_1:Agent_2:10000 \
TURINGOS_REAL7_SCRIPTED_ATTEMPT_PREDICTION_FIXTURE=1 \
TURINGOS_FORCE_BOLTZMANN_SEED_WORKTXS=Agent_0:1:1000 \
TURINGOS_REAL7_SCRIPTED_VERIFY_CHALLENGE=Agent_2:Agent_3 \
PER_PROBLEM_TIMEOUT_S=300 \
bash scripts/run_g_phase_batch.sh g_phase_real_7_structural_smoke_<UTC> \
  handover/evidence/real7_structural_smoke/REAL7_SMOKE_PROBLEMS.txt
```

## Results

Primary evidence:

```text
handover/evidence/g_phase_real_7_structural_smoke_r11_20260515T1032Z/
```

Command outcome:

```text
batch_exit=0
audit_exit=0
audit_verdict=PROCEED
persistence_exit=0
persistence_passing=true
persistence_n_witnessed=5
```

ChainTape/CAS totals from `aggregate_verdict.json`:

```text
L4 entries: 36
L4.E entries: 21
CAS objects: 194
tx_kind_counts:
  task_open=3
  escrow_lock=3
  market_seed=3
  cpmm_pool=3
  buy_with_coin_router=6
  work=3
  verify=3
  challenge=3
  finalize_reward=3
  terminal_summary=3
  event_resolve=3
```

Batch continuity:

```text
task 0 mathd_algebra_107: chain 0 -> 12, exit 0
task 1 mathd_algebra_125: chain 12 -> 24, exit 0
task 2 mathd_algebra_141: chain 24 -> 36, exit 0
```

Dashboard regenerated from ChainTape + CAS:

```text
handover/evidence/g_phase_real_7_structural_smoke_r11_20260515T1032Z/audit_dashboard_run_report.txt
§K G7 structural smoke:
  minimum_tier_green: true
  clean_negative: false
  forward_tb_stub_required: false
  one_runtime_repo: true
  multi_agent: true
  persistent_state: true
  agent_count: 15
  active_role_count: 5
  task_count: 3
  task_outcome_market_count: 3
  scripted_attempt_prediction_market_count: 3
  buy_yes_router_count: 3
  buy_no_or_short_count: 6
  verify_tx_count: 3
  challenge_tx_or_no_challenge_reason_count: 3
  event_resolve_count: 3
  pnl_delta_count: 6
  autopsy_if_loss_satisfied: true
  no_forced_live_investment: true
  market_actions_chain_visible: true
  no_ghost_liquidity: true
  clean_v3_comparison: true
  does_not_claim_identical_v3_equivalence: true
  no_trade_reason_count: 5
  g7_guard_cas_count: 3
  aggregate_audit_guard_source: handover/evidence/g_phase_real_7_structural_smoke_r11_20260515T1032Z/aggregate_verdict.json
```

## Failed Attempts And Fix Evidence

```text
r1: release build failed because REAL-7 route error logging formatted
    InvestRouteError with Display. Fixed to Debug formatting.

r2: Trust Root blocked boot because evaluator.rs hash changed after r1 fix.
    Rehashed genesis_payload.toml and verified Trust Root.

r3/r4: batch reached ChainTape evidence but stopped after task 0 because
    scripted VerifyTx/ChallengeTx overlapped later OMEGA Verify finalization.
    Changed the REAL-7 scripted fixture to close its own
    VerifyTx -> FinalizeReward -> EventResolve path.

r5/r6: hard3 batch exposed resumed-task preseed bug. TaskOpen/EscrowLock
    used ZERO or pre-existing root semantics that were only valid for fresh
    genesis. Fixed TaskOpen parent root and settle barrier to use the
    pre-submit state root, so task_k>0 continues the same ChainTape.

r7: 3-task hard3 smoke completed with audit_tape PROCEED and dashboard §K
    minimum_tier_green=true, but clean-context Codex later challenged that
    dashboard safety/equivalence flags were hardcoded instead of derived from
    ChainTape/CAS evidence.

r8/r9: after adding CAS-backed G7StructuralGuard and dashboard derivation,
    hard3/simple smoke exposed a resumed-task interaction: the scripted
    REAL-7 fixture resolved the task outcome before later live solver turns,
    so a late post-resolution WorkTx could not advance the state root and was
    incorrectly treated as missing Verify/Finalize evidence.

r10: ordinary rerun after the post-resolution guard fix completed 3 tasks,
     audit_tape PROCEED, persistence passing, dashboard §K
     minimum_tier_green=true, and g7_guard_cas_count=3.

r11: final harness-recorded evidence completed 3 tasks with the same structural
     shape and dashboard guard regeneration.
```

## Claim Boundary

- No forced live LLM investment claim.
- No price-as-truth claim.
- No ghost liquidity claim.
- No E2/E3 spontaneous emergence claim from scripted fixture activity.
- This is structural v3-equivalent pressure evidence, not v3-identical volume.
- Dashboard and this report are materialized views; ChainTape/CAS remains
  authoritative.
