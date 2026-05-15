# Codex REAL-6D Implementation Review R4

Scope: clean-context implementation review for REAL-6D Opportunity Scheduler
Observe-Only R4. I did not modify production code. This review inspects the
R4 source/evidence package to determine whether the R3 CHALLENGE findings are
closed and whether the REAL-6D atom can proceed.

## Findings

No blocking findings.

The two R3 CHALLENGE findings are closed for the REAL-6D semantic surface:

1. Production evaluator CAS emission is now present. The evaluator defines a
   fail-closed `write_scheduler_decision_trace_to_cas_or_exit` helper at
   `experiments/minif2f_v4/src/bin/evaluator.rs:642-663`, gates the feature on
   `TURINGOS_REAL6_SCHEDULER_OBSERVE_ONLY` at `:673-678`, derives price signals
   from canonical `QState` pools at `:684-701`, and writes one trace per enabled
   turn at `:3275-3328`. The stable schema/CAS writer lives at
   `src/runtime/agent_scheduler.rs:14` and `:61-75`; `CasStore::put` appends the
   CAS sidecar and advances `refs/chaintape/cas` at
   `src/bottom_white/cas/store.rs:266-312`. R4 evidence confirms five persisted
   `real6.scheduler_decision_trace.v1` objects in the run CAS at
   `handover/evidence/dev_self_hosting/dev_1778833170928_1564703/artifacts/command_0019_stdout.txt:1-5`.

2. Dashboard TaskOutcomeMarket price visibility is now present. The dashboard
   `CpmmPool` walker includes `task-*` and `task_outcome:*` alongside
   `node_survive:*` at `src/bin/audit_dashboard.rs:2311-2331`. The R4 dashboard
   over the smoke evidence renders `price_signals: 1` and the task-market sample
   at `handover/evidence/dev_self_hosting/dev_1778833170928_1564703/artifacts/command_0017_stdout.txt:407-425`.

## Gate Checklist

- SG-6D.1 Scheduler trace includes price/PnL signals: PASS. The required trace
  fields are present in `SchedulerDecisionTrace` at
  `src/runtime/agent_scheduler.rs:49-58`. R4 dashboard evidence shows
  `price_signals: 1`, `pnl_signals: 13`, and a task market sample at
  `command_0017_stdout.txt:414-424`. I also spot-checked CAS blobs referenced by
  `command_0019_stdout.txt:1` and `:5`; both decode to observe-only scheduler
  traces with `visible_nodes=["task-n5_numbertheory_2pownm1prime_nprime_1778833925889"]`,
  price `100000/200000`, one PnL signal, and `observe_only=true`.
- SG-6D.2 observe_only=true: PASS. `build_observe_only_scheduler_trace` hard
  sets `observe_only: true` at `src/runtime/agent_scheduler.rs:130-150`; R4
  dashboard evidence renders `observe_only: true` at `command_0017_stdout.txt:411`.
- SG-6D.3 Recommendation does not change sequencer admission: PASS for the
  REAL-6D scoped source package. `agent_scheduler` is a pure helper and states
  it does not mutate QState or replace sequencer admission at
  `src/runtime/agent_scheduler.rs:1-5`; the scheduler source gate forbids
  admission/sequencer tokens at `tests/constitution_g6_observe_only.rs:205-220`.
  The R4 scoped diff artifact contains production source changes only for
  `experiments/minif2f_v4/src/bin/evaluator.rs`,
  `src/bin/audit_dashboard.rs`, and `src/runtime/agent_scheduler.rs`, plus the
  test/report/genesis files; it has no `src/state/sequencer.rs` or
  `src/state/typed_tx.rs` source diff headers
  (`handover/evidence/dev_self_hosting/dev_1778833170928_1564703/artifacts/diff.patch:1`,
  `:4863`, `:5064`, `:5274`, `:5449`, `:5656`, `:5873`).
- SG-6D.4 Price does not affect L4/L4.E: PASS for this atom. The test gate
  verifies predicates do not read `cpmm_pools_t`, `MarketDecisionTrace`, or
  `price_index` at `tests/constitution_g6_observe_only.rs:69-81`; the scheduler
  renderer explicitly says the recommendation does not change admission or
  predicates at `src/runtime/agent_scheduler.rs:155-157`.
- SG-6D.5 Dashboard shows scheduler recommendation as non-binding: PASS. The
  renderer prints "non-binding materialized view" and "price is signal, not
  truth" at `src/runtime/agent_scheduler.rs:153-166`; R4 dashboard evidence
  shows those rows at `command_0017_stdout.txt:407-409`.

## Verification Evidence

I inspected the recorded R4 harness instead of rerunning cargo/smoke commands,
because this reviewer turn was instructed not to modify production code and only
to write this audit file.

- R4 manifest: Class 4 atom, FC1-N7 / FC2-HEAD_t / FC3-N43 / Art-0.4, allowed
  paths include the audit file and scoped REAL-6D files:
  `handover/evidence/dev_self_hosting/dev_1778833170928_1564703/DevTaskManifest.json:3-42`.
- Formatting red/green: `events.jsonl:2-3` records initial `cargo fmt --all -- --check`
  failure then success after the scoped format fix.
- Targeted REAL-6D test: `command_0011_stdout.txt` records 7/7 pass, including
  runtime-CAS-emission, chain-backed CAS evidence, observe-only, and no-admission
  gates.
- Trust Root: `command_0012_stdout.txt` records
  `boot::tests::verify_trust_root_passes_on_intact_repo ... ok`.
- Constitution gates: `command_0013_stdout.txt` ends with `Totals: 436 passed,
  0 failed, 1 ignored` and `PASS: all gates GREEN`.
- Workspace tests: `command_0014_stdout.txt` ends green with no failing tests.
- Real smoke: `command_0016_stdout.txt:32-43` records `audit_tape`
  `verdict=PROCEED passed=41 failed=0 halted=0 skipped=11` and persistence
  `is_passing=true n_witnessed=3`.
- Dashboard regeneration/markers: `command_0017_stdout.txt:400-428` renders the
  scheduler section; `command_0018_stdout.txt:1` records
  `dashboard_scheduler_markers_ok`.

## Safety Review

- No price-as-truth / ghost liquidity / f64 money path was introduced by the
  REAL-6D scheduler path. Runtime and dashboard prices are integer-rational
  strings from pool units (`evaluator.rs:688-699`,
  `audit_dashboard.rs:2311-2331`), and PnL/scheduler budget fields are integer
  values (`src/runtime/agent_scheduler.rs:36-43`). Existing legacy `f64` uses in
  `experiments/minif2f_v4/src/bin/evaluator.rs` are outside the REAL-6D
  scheduler hook and were not introduced by this closure.
- Trust Root hashes match current pinned bytes for the modified pinned files:
  `genesis_payload.toml:164` pins the evaluator hash and
  `genesis_payload.toml:243` pins `audit_dashboard`; `command_0012_stdout.txt`
  verifies the intact repo hash check passed.
- The broader worktree remains very dirty, including restricted files outside
  this review's semantic surface. That is not certified by this REAL-6D verdict.
  The broad Trust Root normalization was previously documented as dirty-tree
  preservation rather than semantic review at
  `handover/evidence/real6_task_outcome/TRUST_ROOT_REHASH_REAL6A_WORKSPACE_NORMALIZATION.md:6-11`
  and `:108-114`. Any branch-level ship packet still needs intentional staging
  and the applicable prior Class-4 audit boundaries.

## Non-Blocking Observations

- The regenerated dashboard's synthetic `SchedulerDecisionTrace` row still shows
  `visible_nodes: 0` for the task-only smoke at
  `command_0017_stdout.txt:412-420`, because the dashboard count is still based
  on the node-market seed map. The persisted production CAS traces are stronger
  evidence here: the decoded CAS objects contain the task market in
  `visible_nodes`. This is a dashboard polish gap, not an SG-6D blocker.
- The REAL-6D report says command_0013 had 431 passed gates, while the recorded
  command artifact shows 436 passed. The command exit and all-gates-green result
  are unambiguous; the count drift is documentation-only.

## Paths Changed By This Review

- `handover/audits/CODEX_REAL6D_IMPLEMENTATION_REVIEW_R4.md`

PROCEED

## Post-Review Addendum

The documentation-only gate-count drift noted above has been resolved. The
REAL-6D report now states `command_0013` as `436 passed / 0 failed / 1 ignored`,
matching `command_0013_stdout.txt`; the harness records this correction in
`command_0024` with `real6d_report_gate_count_ok`.

Verdict remains: PROCEED

PROCEED
