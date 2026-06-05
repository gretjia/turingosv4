# A11 Scheduler And Search Policy Preflight

Date: 2026-06-05

Parent plan:
`handover/directives/2026-06-05_AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT.md`

Atom: A11. Scheduler and Search Policies

Document role: Class 0 preflight. This document does not authorize scheduler
budget allocation, economy mutation, sequencer admission, typed transaction, or
parallel worker write-path changes by itself.

## Decision

A11 is the scheduler/search atom. It must not be confused with A08
PredicateReceipt/LeanJudge work. Scheduler implementation must wait for A04
ChainTape authority, A05 TapeEvent/projection, A09 economy projection, and A10
projection-cache semantics.

Safe work now:

- docs-only preflight
- current scheduler/economy witness inventory
- dependency and risk correction
- acceptance-command correction

Blocked until predecessors exist:

- claiming `SchedulerDecisionEvent` is tape-reconstructable
- implementing `src/scheduler/*` as production routing authority
- using existing observe-only scheduler traces as canonical events
- routing real budget/economy state from scheduler decisions
- implementing `parallel_lanes` before L2 single-writer/OCC is settled

## Hard Blockers

- A11-HB1: A11 cannot claim replayable scheduler authority before A04/A05/A09/A10
  establish ChainTape authority, TapeEvent projection, economy projection, and
  cache watermark semantics.
- A11-HB2: Existing observe-only scheduler traces cannot become canonical
  `SchedulerDecisionEvent` evidence without tape-visible events and replay
  tests.
- A11-HB3: `parallel_lanes` is blocked until L2 single-writer/OCC behavior is
  settled and tested.
- A11-HB4: Real budget or economy allocation from scheduler decisions promotes
  the atom to Class 3 and requires the corresponding clean-context audit.

## Current-State Facts

Parent-plan A11 allowed paths:

```text
src/scheduler/policy.rs
src/scheduler/non_local_tree.rs
src/scheduler/softmax.rs
src/scheduler/parallel_lanes.rs
src/scheduler/forced_loop.rs
tests/scheduler_policy_trace.rs
tests/scheduler_softmax_distribution.rs
tests/scheduler_parallel_isolation.rs
tests/scheduler_forced_loop_bounds.rs
```

Existence check:

```text
MISSING src/scheduler/policy.rs
MISSING src/scheduler/non_local_tree.rs
MISSING src/scheduler/softmax.rs
MISSING src/scheduler/parallel_lanes.rs
MISSING src/scheduler/forced_loop.rs
MISSING tests/scheduler_policy_trace.rs
MISSING tests/scheduler_softmax_distribution.rs
MISSING tests/scheduler_parallel_isolation.rs
MISSING tests/scheduler_forced_loop_bounds.rs
```

Corrected implementation path inventory:

```text
src/scheduler/mod.rs
src/scheduler/policy.rs
src/scheduler/non_local_tree.rs
src/scheduler/softmax.rs
src/scheduler/parallel_lanes.rs
src/scheduler/forced_loop.rs
src/runtime/mod.rs
src/runtime/agent_scheduler.rs
src/sdk/actor.rs
src/state/price_index.rs
src/runtime/chain_tape_lease.rs
src/bottom_white/ledger/transition_ledger.rs
src/bin/turingos/cmd_generate.rs
src/web/generate.rs
tests/constitution_g5_scheduler.rs
tests/constitution_g6_observe_only.rs
tests/scheduler_policy_trace.rs
tests/scheduler_softmax_distribution.rs
tests/scheduler_parallel_isolation.rs
tests/scheduler_forced_loop_bounds.rs
```

Dirty-path check for relevant inventory:

```text
pre-existing dirty paths include:
  src/bottom_white/ledger/transition_ledger.rs
  src/state/sequencer.rs
  src/state/typed_tx.rs
  src/economy/monetary_invariant.rs
  src/bin/turingos/cmd_generate.rs
  src/web/generate.rs
  tests/constitution_g6_observe_only.rs

Implementation must read and preserve those edits. Do not overwrite them as
part of A11 scaffolding.
```

Existing scheduler/search/economy witnesses:

```text
src/runtime/agent_scheduler.rs:1
  observe-only opportunity scheduler helper, not production routing authority
src/runtime/agent_scheduler.rs:20
  SchedulerMode supports RoundRobin and ObserveOnly
src/runtime/agent_scheduler.rs:49
  SchedulerDecisionTrace carries price_signals, pnl_signals, recommended_agent,
  and observe_only
src/runtime/agent_scheduler.rs:61
  SchedulerDecisionTrace writes to CAS as ObjectType::Generic with schema_id
tests/constitution_g5_scheduler.rs:8
  round-robin and observe-only scheduler tests
tests/constitution_g6_observe_only.rs:204
  observe-only traces do not wire into sequencer/typed-tx/predicate paths
src/state/price_index.rs:1
  PriceIndex is a derived view; price is signal, not truth
src/state/price_index.rs:164
  compute_price_index is pure over EconomicState
src/state/price_index.rs:267
  BoltzmannMaskPolicy uses integer-rational fields
src/sdk/actor.rs:16
  current selector is integer-rational parent selection; full softmax is
  deferred
src/state/q_state.rs:181
  price_index_t removed from canonical QState; compute from EconomicState
src/economy/money.rs:27
  MicroCoin is integer money
src/runtime/chain_tape_lease.rs:1
  ChainTapeLease is the current single-writer lease guard for L4 refs
```

Existing parallel fan-out is not A11 scheduler:

```text
src/bin/turingos/cmd_generate.rs:308
  --n-parallel-workers CLI flag
src/bin/turingos/cmd_generate.rs:922
  candidate generation loops over extra workers
src/web/generate.rs:317
  web generate defaults to parallel worker count

These are worker fan-out surfaces, not the planned `parallel_lanes`
SchedulerDecisionEvent isolation contract.
```

## Risk Classification

Risk floor: Class 2 for read-only scheduler policy modules and tests.

Promote to Class 3 if:

- scheduler decisions allocate real budget or mutate economy state
- policy writes become production capability routing
- price projection cache is consumed as budget/economy input
- parallel lanes make external calls or write L2 events

Promote to Class 4 if:

- sequencer admission changes
- typed tx schema or discriminants change
- canonical signing payload changes
- ChainTape writer/OCC/single-writer authority changes
- trust-root / constitution / flowchart authority changes

## Recommended Contract

A11 should implement C8 only after A05/A09/A10:

```text
SchedulerDecisionEvent {
  decision_id: EventId,
  input_tape_head: GitOid,
  price_projection_head: Option<GitOid>,
  scheduler_view_cid: Cid,
  candidate_set_cid: Cid,
  candidate_set_hash: Hash,
  policy_input_bundle_hash: Hash,
  scoped_agent_view_head: Option<GitOid>,
  policy_name: String,
  selected_agent_or_task: String,
  random_seed_or_deterministic_reason: DecisionReason,
}
```

Required invariant:

```text
decision replay reads only tape prefix + projection heads.
candidate set is loaded from CAS and verified against candidate_set_hash.
policy input bundle records policy id/version, projection heads, and the scoped
AgentView handle.
memory-only candidate construction cannot pass replay.
softmax policy must be distributional in deterministic fixtures.
parallel lanes may share public tape prefix but not private sibling context.
forced loops must have max iterations, max tokens, and max wall-clock.
```

## Atomized A11 Tasks

### A11.0 Preflight Lock

Description:
Record missing scheduler files, existing observe-only witnesses, predecessor
dependencies, and the distinction between worker fan-out and scheduler
parallel-lane isolation.

Acceptance:

```bash
for f in \
  handover/directives/2026-06-05_A11_SCHEDULER_SEARCH_PREFLIGHT.md \
  handover/directives/2026-06-05_AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT.md
do
  git diff --no-index --check /dev/null "$f" || true
done
```

Expected:

```text
no whitespace errors.
A11 preflight states A04/A05/A09/A10 are predecessors.
A11 preflight states observe-only scheduler traces are not C8 completion.
```

### A11.1 Scheduler Decision Event Shape

Description:
After A05/A09/A10 exist, add C8 data shape and trace tests. This step is
read-only policy output, not budget routing.

Acceptance:

```bash
cargo test --test scheduler_policy_trace --no-fail-fast -- --test-threads=1
cargo test --test constitution_g5_scheduler --no-fail-fast
cargo test --test constitution_g6_observe_only --no-fail-fast
git diff --check
```

Expected:

```text
every decision is reconstructable from tape prefix and projection heads.
candidate_set_cid and policy_input_bundle_hash are enough to rebuild scheduler
inputs without memory-only state.
observe-only scheduler remains non-binding.
policy output does not touch sequencer or typed tx.
```

### A11.2 Strategy Gates

Description:
Add softmax, parallel-lane, and forced-loop gates without changing L2 writer or
economy mutation rules.

Acceptance:

```bash
cargo test --test scheduler_softmax_distribution --no-fail-fast -- --test-threads=1
cargo test --test scheduler_parallel_isolation --no-fail-fast -- --test-threads=1
cargo test --test scheduler_forced_loop_bounds --no-fail-fast -- --test-threads=1
git diff --check
```

Expected:

```text
equal prices distribute over >= 3/5 candidates in deterministic fixture.
argmax-collapse positive control fails.
parallel lanes cannot read sibling private context.
forced loop stops at configured iteration/token/wall-clock bounds.
```

## Final Pre-Implementation Gate

A11 implementation may start only when all are true:

- A04 selected ChainTape-L4 authority
- A05 TapeEvent/projection contract exists
- A09 economy projection exists if price/economy signals are inputs
- A10 projection cache watermark semantics exist if cached projections are used
- the first code change is a failing scheduler test
- restricted surfaces are either untouched or explicitly ratified

Clean-context audit input for a future implementation PR:

```text
Task brief: A11 Scheduler and Search Policies.
Risk class: Class 2; promote to Class 3 if real budget/economy allocation or
production capability routing is changed.
FC nodes: FC1-N7, FC1-N13, FC1 routing, L6 price projection boundary.
Evidence: A04/A05/A09/A10 predecessor evidence, scheduler trace tests,
softmax/parallel/forced-loop tests, constitution gates, restricted-surface
diff guard.
Verdict domain: NO-VIOLATION | VIOLATION-FOUND | RECONSTRUCTION-FAILURE |
SECOND-SOURCE-DRIFT
```
