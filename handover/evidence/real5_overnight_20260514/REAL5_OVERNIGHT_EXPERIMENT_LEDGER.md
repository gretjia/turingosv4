# REAL-5 Overnight Experiment Ledger

Date: 2026-05-14 UTC

Objective:

```text
Ensure the REAL-5 execution plan has tape-visible role-turn evidence, then run
true MiniF2F problem tests under bounded adversarial conditions to collect data
for architect review.
```

Invariant boundaries:

```text
No forced trade.
No price-as-truth.
No private CoT recording.
No raw-log broadcast.
No ghost liquidity.
No automatic tool/predicate mutation.
```

## Attempts

| Attempt | Run tag | Condition | Problem set | Prompt/role knobs | Outcome | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| R5-SMOKE-PARTIAL-ROLETURN | `g_phase_real_5_role_smoke_roleturn_20260514T_FINALZ` | n5, unbounded evaluator | `mini` | role views on; `Solver,Trader,Verifier,Challenger,Observer` | intentionally terminated after discovering `PER_PROBLEM_TIMEOUT_S` is not enforced by `batch_evaluator`; partial CAS already contains role-turn traces | `handover/evidence/g_phase_real_5_role_smoke_roleturn_20260514T_FINALZ` |
| R5-CORE3-B20 | `g_phase_real_5_core3_bounded20_20260514T_FINALZ` | n5, `MAX_TRANSACTIONS=20` | `problems_core3.txt` | role views on; `Solver,Trader,Verifier,Challenger,Observer`; market K=10 | PASS; audit_tape `PROCEED`; 3 true tasks; 6 VerifyTx; 12 role-turn traces; 0 router buys | `handover/evidence/g_phase_real_5_core3_bounded20_20260514T_FINALZ` |
| R5-TRADER-FIRST-B12 | `g_phase_real_5_trader_first_b12_20260514T_FINALZ` | n5, `MAX_TRANSACTIONS=12` | `problems_core3.txt` | role views on; `Trader,Solver,Verifier,Challenger,Observer`; market K=10 | PASS; audit_tape `PROCEED`; role order adversary; 5 VerifyTx; 14 role-turn traces; 0 router buys | `handover/evidence/g_phase_real_5_trader_first_b12_20260514T_FINALZ` |
| R5-SEEDED-MARKET-B12 | `g_phase_real_5_seeded_markets_b12_20260514T_FINALZ` | n5, `MAX_TRANSACTIONS=12` | `problems_core3.txt` | role views on; normal roles; `TURINGOS_FORCE_BOLTZMANN_SEED_WORKTXS=Agent_0:2:1000` | FAIL-CLOSED diagnostic; batch exit 1 while audit_tape over partial chain `PROCEED`; forced seed hook timed out waiting for commit at iter 0 | `handover/evidence/g_phase_real_5_seeded_markets_b12_20260514T_FINALZ` |
| R5-MARKET-K0-B12 | `g_phase_real_5_market_k0_b12_20260514T_FINALZ` | n5, `MAX_TRANSACTIONS=12` | `problems_core3.txt` | role views on; normal roles; market K=0 | PASS; audit_tape `PROCEED`; 3 VerifyTx; 9 role-turn traces; no `PromptBudgetExceeded`; Trader NoTrade stayed `NoPool` | `handover/evidence/g_phase_real_5_market_k0_b12_20260514T_FINALZ` |
| R5-ADV5-B8 | `g_phase_real_5_adversarial5_b8_20260514T_FINALZ` | n5, `MAX_TRANSACTIONS=8` | `problems_adversarial5.txt` | role views on; normal roles; market K=10 | PASS; audit_tape `PROCEED`; 5 true tasks; 5 VerifyTx; 21 role-turn traces; 0 router buys | `handover/evidence/g_phase_real_5_adversarial5_b8_20260514T_FINALZ` |
| R5-TRADER-FIRST-B8-ROLEGATE | `g_phase_real_5_trader_first_b8_rolegate_20260514T192523Z` | n5, `MAX_TRANSACTIONS=8` | `problems_core3.txt` | role views on; `Trader,Solver,Verifier,Challenger,Observer`; market K=10; post-VETO role gateway enforcement | PASS; audit_tape `PROCEED`; persistence passing; Agent_0 Trader role emitted 5 `PolicyRejected` role-turn traces and no accepted WorkTx/VerifyTx | `handover/evidence/g_phase_real_5_trader_first_b8_rolegate_20260514T192523Z` |
| R5-CORE3-B8-ROLEGATE | `g_phase_real_5_core3_b8_rolegate_20260514T192958Z` | n5, `MAX_TRANSACTIONS=8` | `problems_core3.txt` | role views on; `Solver,Trader,Verifier,Challenger,Observer`; market K=10; post-VETO role gateway enforcement | PASS; audit_tape `PROCEED`; persistence passing; Trader role emitted 4 `PolicyRejected` traces; no live router buys | `handover/evidence/g_phase_real_5_core3_b8_rolegate_20260514T192958Z` |

## Known Harness Observation

`batch_evaluator` currently accepts `--per-task-timeout-s`, but the implementation
documents it as reserved and does not kill the evaluator subprocess. Bounded
overnight tests therefore use `MAX_TRANSACTIONS` and outer shell `timeout`.

## Aggregate Observation

Across successful bounded runs, REAL-5 produced role-scoped PromptCapsuleV2 and
`real5.role_turn_trace.v1` CAS evidence. Verifier behavior appears before trader
market behavior: live VerifyTx counts were non-zero in bounded runs, while
`buy_with_coin_router` stayed zero. Every Trader NoTrade reason observed in
role-turn traces was `NoPool`, pointing at event timing / same-task pool
availability rather than prompt top-K elision.

Post-VETO role-gateway runs add a sharper result: proof-solver style `step`
outputs from a `Trader` no longer enter the legacy proof/verify production path.
They are classified as `RoleTurnOutcome::PolicyRejected` and anchored by
`real5_role_policy_reject-*` L4.E WorkTx witnesses. This supports scaffold
completion for role-policy enforcement, while preserving the non-claim that no
spontaneous router trade emerged.
