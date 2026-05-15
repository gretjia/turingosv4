# REAL-5 Completion And Adversarial Test Report

Date: 2026-05-14 UTC

## Scope

Risk class: Class 4 package.

Touched FC nodes:

```text
FC1 externalized role action loop
FC2 role assignment + PromptCapsule replay from genesis/batch + ChainTape/CAS
FC3 evidence/materialized view
Art. III shielding
```

## Current Verdict State

Clean-context Codex review R2 returned `VETO`:

```text
handover/audits/CODEX_REAL5_IMPLEMENTATION_REVIEW_R2.md
```

Blocking issue:

```text
Trader-first evidence assigned Agent_0=Trader, but Agent_0 still produced
VerifyTx / WorkTx through the legacy evaluator path.
```

Post-VETO remediation:

```text
legacy evaluator tools now pass through the REAL-5 typed role gateway before
the old action branches run;
role resolution during live turns is derived from startup genesis/batch role
assignment, not from a fresh per-turn env parse;
PromptCapsuleV2 visible_context_cid now stores the full visible prompt context
whose bytes hash to prompt_context_hash, while the derived role view is kept as
a read-set CID.
```

Post-VETO evidence:

```text
handover/evidence/g_phase_real_5_trader_first_b8_rolegate_20260514T192523Z
handover/evidence/g_phase_real_5_core3_b8_rolegate_20260514T192958Z
```

Both post-VETO runs have `audit_tape verdict: PROCEED`.

## Completion Status

The REAL-5 execution plan has been carried through implementation, evidence,
and verification for the scaffold-level claim:

```text
role-based generation scaffold,
tape-visible decisions/reasons,
typed gateway,
role-scoped view,
reconstructable ChainTape/CAS evidence.
```

Important non-claim:

```text
This does not prove E2/E3 live market emergence.
No live agent-generated BuyWithCoinRouterTx was observed.
No persistent role-differentiated market behavior is claimed.
```

Atom alignment:

| Atom | Status | Evidence |
| --- | --- | --- |
| Atom 0 Charter + Decision Records | Complete | `handover/tracer_bullets/REAL-5_role_based_generative_scaffolding_charter.md`; three REAL-5 decision records under `handover/alignment/` |
| Atom 1 AgentRoleAssignment | Complete | `GenesisReport.agent_role_assignment`; `RoleAssignmentManifest` CAS; batch manifest role CID |
| Atom 2 Role-scoped rtool / derive_view | Complete | `DerivedViewRequest`, `DerivedView`, `derive_role_view_with_context_bytes`; role view CAS schema `real5.derived_view.visible_context.v1` |
| Atom 3 PromptCapsule role/view upgrade | Complete | `PromptCapsuleV2`; schema `v2/prompt_capsule_role_view`; AttemptTelemetry v3 `prompt_capsule_cid` |
| Atom 4 Typed Generation Gateway | Complete for scaffold after R2 remediation | `RoleAction`, parser/route gates; production evaluator now gates legacy tools before old action branches; role overreach becomes `RoleTurnOutcome::PolicyRejected` plus `real5_role_policy_reject-*` L4.E anchor |
| Atom 5 TickBudget | Complete for scaffold | `TickBudget`, tick events, derivation tests |
| Atom 6 Trader Role Activation | Complete for reason-trace scaffold | `MarketDecisionTrace` remains canonical market action trace; new `real5.role_turn_trace.v1` captures Trader `NoTrade` by role |
| Atom 7 Verifier / Challenger Bridge | Complete for VerifyTx + reason-trace scaffold | Live VerifyTx observed; NoVerify/NoChallenge role-turn traces observed |
| Atom 8 ArchitectAI / VetoAI Scaffold | Complete for proposal/veto/sandbox status | `ToolProposal`, `VetoDecision`, canary-only activation status |
| Atom 9 REAL-5 Role-Based Smoke | Complete for scaffold | Successful true-problem bounded runs listed below |

## Implementation Delta After Previous Audit

After the prior `CODEX_REAL5_IMPLEMENTATION_REVIEW.md`, one gap was closed:

```text
Verifier/Challenger/Trader no-action outcomes are now CAS-visible via
real5.role_turn_trace.v1.
```

New production/test surfaces:

```text
src/runtime/real5_roles.rs:
  ROLE_TURN_TRACE_SCHEMA_ID
  RoleTurnTrace
  RoleTurnOutcome
  write_role_turn_trace_to_cas
  summarize_role_turn_traces_from_cas

experiments/minif2f_v4/src/bin/evaluator.rs:
  pre-turn PromptCapsuleV2 CAS anchor
  role-turn trace writer after parse/action classification
```

After `CODEX_REAL5_IMPLEMENTATION_REVIEW_R2.md` returned `VETO`, three more
gaps were closed:

```text
VETO-1 role overreach:
  real5_gate_parsed_action_for_role now runs before legacy action dispatch.
  Trader + step/append/complete/verify_peer no longer reaches proof/verify
  production branches.

CHALLENGE-1 PromptCapsuleV2 replay:
  visible_context_cid now stores full visible prompt context bytes matching
  prompt_context_hash. The derived role view is stored separately and included
  in read_set.

CHALLENGE-2 hidden role switch:
  live role selection uses the startup role_assignment manifest map. Missing
  genesis role assignment fails closed when REAL-5 role gateway is active.
```

## Verification

Recorded in `turingos_dev` run:

```text
dev_1778782264956_718239
```

Commands:

```text
command_0009:
  REAL-5 targeted tests + batch role manifest test + AttemptTelemetry v3 compatibility + Trust Root verify
  exit 0

command_0010:
  bash scripts/run_constitution_gates.sh
  cargo test --workspace --no-fail-fast -- --test-threads=1
  exit 0

command_0012:
  git diff --check
  exit 0
```

`cargo fmt --all -- --check` was also recorded as `command_0011` and failed
because the repository has broad pre-existing rustfmt drift across files outside
this REAL-5 change. It was not used as a ship gate. `git diff --check` passed.

## True Problem Experiments

### R5-CORE3-B20

Run:

```text
handover/evidence/g_phase_real_5_core3_bounded20_20260514T_FINALZ
```

Condition:

```text
n5
MAX_TRANSACTIONS=20
roles=Solver,Trader,Verifier,Challenger,Observer
market K=10
3 MiniF2F tasks
```

Evidence:

```text
audit_tape verdict: PROCEED
work=1
verify=6
challenge=0
market_seed=1
cpmm_pool=1
buy_with_coin_router=0
role_turn_trace=12
Trader NoTrade=3, all NoPool
```

Problem outcomes:

```text
mathd_algebra_107: solved=true, tx_count=1
mathd_algebra_125: solved=false, hit_max_tx=true, verify_peer=5
mathd_algebra_141: solved=false, hit_max_tx=true, verify_peer=1
```

### R5-TRADER-FIRST-B12

Run:

```text
handover/evidence/g_phase_real_5_trader_first_b12_20260514T_FINALZ
```

Condition:

```text
n5
MAX_TRANSACTIONS=12
roles=Trader,Solver,Verifier,Challenger,Observer
market K=10
3 MiniF2F tasks
```

Evidence:

```text
audit_tape verdict: PROCEED
verify=5
buy_with_coin_router=0
role_turn_trace=14
Trader NoTrade=1, NoPool
```

### R5-SEEDED-MARKET-B12

Run:

```text
handover/evidence/g_phase_real_5_seeded_markets_b12_20260514T_FINALZ
```

Condition:

```text
TURINGOS_FORCE_BOLTZMANN_SEED_WORKTXS=Agent_0:2:1000
MAX_TRANSACTIONS=12
```

Outcome:

```text
batch_exit=1
audit_tape over partial chain: PROCEED
persistence_passing=true
failure: FORCE_BOLTZMANN_SEED_WORKTXS commit await expired at iter=0
```

Interpretation:

```text
Existing force-seed hook is not a reliable market-activation mechanism for REAL-5 tests.
It failed closed, which is correct, but it should not be used as evidence of market emergence.
```

### R5-MARKET-K0-B12

Run:

```text
handover/evidence/g_phase_real_5_market_k0_b12_20260514T_FINALZ
```

Condition:

```text
MAX_TRANSACTIONS=12
market K=0
```

Evidence:

```text
audit_tape verdict: PROCEED
verify=3
buy_with_coin_router=0
role_turn_trace=9
Trader NoTrade=2, all NoPool
```

Interpretation:

```text
No observed PromptBudgetExceeded. The dominant trader bottleneck is not K=0 elision;
it is absence of visible same-task tradeable pool during Trader turns.
```

### R5-ADV5-B8

Run:

```text
handover/evidence/g_phase_real_5_adversarial5_b8_20260514T_FINALZ
```

Condition:

```text
5 MiniF2F tasks
MAX_TRANSACTIONS=8
roles=Solver,Trader,Verifier,Challenger,Observer
market K=10
```

Evidence:

```text
audit_tape verdict: PROCEED
persistence_passing=true
n_witnessed=5
work=1
verify=5
challenge=0
market_seed=1
cpmm_pool=1
buy_with_coin_router=0
role_turn_trace=21
Trader NoTrade=6, all NoPool
```

Problem outcomes:

```text
mathd_algebra_107: solved=true
mathd_algebra_125: hit_max_tx=true, verify_peer=1
mathd_algebra_141: hit_max_tx=true
mathd_algebra_113: hit_max_tx=true
mathd_algebra_114: hit_max_tx=true, verify_peer=4
```

### R5-TRADER-FIRST-B8-ROLEGATE

Run:

```text
handover/evidence/g_phase_real_5_trader_first_b8_rolegate_20260514T192523Z
```

Condition:

```text
n5
MAX_TRANSACTIONS=8
roles=Trader,Solver,Verifier,Challenger,Observer
market K=10
post-VETO role gateway enforcement
3 MiniF2F tasks
```

Evidence:

```text
batch_exit=0
audit_tape verdict: PROCEED
persistence_passing=true
n_witnessed=5
l4_entries=9
l4e_entries=26
accepted VerifyTx by Agent_0 Trader: 0
accepted WorkTx by Agent_0 Trader: 0
Agent_0 Trader role_turn outcomes: PolicyRejected=5
```

Representative CAS role-turn evidence:

```text
Agent_0 role=Trader action_kind=step
outcome=PolicyRejected("trader cannot submit proof unless role permits")
```

Interpretation:

```text
This directly remediates the R2 VETO pattern. The Trader still tends to emit
proof-solver style `step`, but the production evaluator now blocks it at the
REAL-5 role gateway instead of letting it become accepted VerifyTx/WorkTx.
```

### R5-CORE3-B8-ROLEGATE

Run:

```text
handover/evidence/g_phase_real_5_core3_b8_rolegate_20260514T192958Z
```

Condition:

```text
n5
MAX_TRANSACTIONS=8
roles=Solver,Trader,Verifier,Challenger,Observer
market K=10
post-VETO role gateway enforcement
3 MiniF2F tasks
```

Evidence:

```text
batch_exit=0
audit_tape verdict: PROCEED
persistence_passing=true
n_witnessed=4
l4_entries=8
l4e_entries=27
Trader role_turn outcomes: PolicyRejected=4
Verifier role_turn outcomes: NoVerify=1, PolicyRejected=3
buy_with_coin_router=0
```

Interpretation:

```text
Normal role order also shows role-policy enforcement. It also exposes the next
mechanism bottleneck: the base prompt/model still emits solver-style `step`
for non-solver roles, so REAL-5 scaffolding is doing protective work but does
not yet produce market action emergence.
```

## Aggregate Findings

1. REAL-5 scaffold evidence is now stronger than the earlier report:

```text
PromptCapsuleV2 exists.
Role assignment is replayable from genesis/batch evidence.
Role-turn reason/action traces are CAS-visible.
Verifier role produces live VerifyTx in true-problem runs.
Trader and Challenger abstentions are no longer silent.
Role overreach is blocked by the production evaluator before legacy action dispatch.
```

2. Verifier behavior is easier to activate than Trader behavior:

```text
Successful bounded runs observed verify counts: 6, 5, 3, 5.
All successful bounded runs observed buy_with_coin_router = 0.
```

3. Trader clean-negative is consistent:

```text
All role-turn Trader NoTrade reasons observed overnight were NoPool.
```

This points away from "the model refuses trading" as the only explanation and
toward event timing / same-task pool availability:

```text
post-accept node markets often appear after uncertainty has collapsed,
or appear in a task that exits before a Trader turn can use them,
or are filtered from later tasks by same-task isolation.
```

4. Existing forced seed hook is not a clean activation path:

```text
TURINGOS_FORCE_BOLTZMANN_SEED_WORKTXS failed closed waiting for commit.
```

5. `batch_evaluator --per-task-timeout-s` is not currently enforced:

```text
The code explicitly reserves `timeout_s` for future wait-timeout integration.
Overnight bounded runs used MAX_TRANSACTIONS + outer shell timeout instead.
```

6. Post-VETO role gateway evidence changes the market diagnosis:

```text
Before role gateway enforcement, Trader sometimes behaved like a proof/verify
agent because the legacy evaluator accepted those tools.

After role gateway enforcement, Trader proof-style output is blocked as
PolicyRejected. The remaining failure to trade is therefore not hidden
role-permission leakage; it is an activation/timing/incentive issue.
```

## Recommendation For Architect

REAL-5 can be ratified for scaffold completion, with this narrowed claim:

```text
REAL-5 proves role-based generation scaffolding and tape-visible role decisions.
REAL-5 does not prove spontaneous market trading emergence.
```

The next architecture decision should likely be REAL-6:

```text
Event Timing Redesign
TaskOutcomeMarket: task will be solved before deadline / budget.
AttemptPredictionMarket: this candidate proof will verify.
```

Reason:

```text
Current post-accept node market timing is too late for Trader turns to see
tradeable uncertainty. The clean-negative evidence says NoPool, not
PromptBudgetExceeded or AgentDeclined.
```
