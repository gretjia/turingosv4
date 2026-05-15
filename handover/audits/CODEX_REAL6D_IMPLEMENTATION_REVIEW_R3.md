# Codex REAL-6D Implementation Review R3

Verdict: CHALLENGE

Scope reviewed:

- `src/runtime/agent_scheduler.rs`
- `src/bin/audit_dashboard.rs`
- `tests/constitution_g6_observe_only.rs`
- `genesis_payload.toml`
- `handover/evidence/real6_scheduler_observe_only/REAL6D_OPPORTUNITY_SCHEDULER_REPORT.md`
- Harness run `handover/evidence/dev_self_hosting/dev_1778831813745_1527867/`
- REAL-6A dashboard evidence used by command_0008:
  `handover/evidence/g_phase_real_6a_task_outcome_smoke_r10_20260515T0442Z/`

Risk / FC mapping reviewed:

- Risk class: Class 4 package due `genesis_payload.toml` Trust Root rehash.
- FC nodes/invariants: FC1-N7 scheduler signal loop, FC2 HEAD_t / Art.0.4 witness, FC3-N43 materialized audit/report view.

## Findings

### CHALLENGE-1: The real scheduler trace is still not emitted to the run CAS/ChainTape evidence path

`src/runtime/agent_scheduler.rs:61` adds
`write_scheduler_decision_trace_to_cas`, and
`tests/constitution_g6_observe_only.rs:158` proves the helper can write a
synthetic trace into a temporary CAS with schema id
`real6.scheduler_decision_trace.v1`. That is useful scaffolding, but it does
not close the R1 production/evidence defect.

The actual `audit_dashboard --run-report` scheduler trace is only built and
rendered at `src/bin/audit_dashboard.rs:2710` and
`src/bin/audit_dashboard.rs:2720`. There is no production call to
`write_scheduler_decision_trace_to_cas`; the only call site I found is the unit
test at `tests/constitution_g6_observe_only.rs:182`. I also found no
`real6.scheduler_decision_trace.v1` entry in the REAL-6A CAS index used by
command_0008.

Impact: R1 finding #1 remains open for the real evidence path. A reviewer can
read the §J.1 dashboard text, but cannot recover the actual
`SchedulerDecisionTrace` object from the run's CAS/ChainTape evidence. This is
not a sequencer-admission safety defect; it is a tape-first completion defect
against the approved REAL-6D atom "Emit SchedulerDecisionTrace to CAS/ChainTape
evidence path."

Required fix: write the same scheduler trace that is rendered in §J.1 into the
run evidence CAS/ChainTape path with the stable schema id, and surface the CID
or other replayable anchor in the report; or obtain explicit architect/user
deferral changing REAL-6D to dashboard-only.

### CHALLENGE-2: The real §J.1 scheduler trace drops TaskOutcomeMarket price signals

SG-6D.1 requires the scheduler trace to include price and PnL signals. The R3
REAL-6A evidence does contain task-outcome market activity:
`runtime_repo/run_summary.json:8` records a task-outcome `MarketSeed`, and
`runtime_repo/run_summary.json:9` records a task-outcome `CpmmPool`.
`command_0008_stdout.txt:260` and `:261` also render `MarketSeed: 1` and
`CpmmPool: 1`.

However, the scheduler price-signal collector only pushes signals for
`CpmmPool` events whose id starts with `node_survive:` at
`src/bin/audit_dashboard.rs:2301` through `src/bin/audit_dashboard.rs:2326`.
The resulting §J.1 trace over that same evidence renders `price_signals: 0`
while `pnl_signals: 13` at `command_0008_stdout.txt:392` and `:393`.

Impact: REAL-6D currently demonstrates PnL visibility, but not real
TaskOutcomeMarket price visibility in the scheduler observation loop. The unit
tests construct a synthetic trace with a price signal, but the production
dashboard/evidence path used for R3 does not carry the available REAL-6A
TaskOutcomeMarket price signal into §J.1.

Required fix: include TaskOutcomeMarket / non-`node_survive:` CPMM price
signals in the scheduler trace derived from ChainTape/CAS, or explicitly
document and ratify why SG-6D.1 does not require the REAL-6A task market price
in this atom.

## R1 Closure Status

- R1 finding #1: not closed. A CAS writer/reader exists, but the real rendered
  trace is not emitted to the run evidence CAS/ChainTape path. See
  CHALLENGE-1.
- R1 finding #2: closed enough for R3. The dashboard no longer uses
  `tx_count:*`; it builds `scheduler_head_t` from L4 head, L4.E hash, CAS
  merkle root, replay state root, and run id at
  `src/bin/audit_dashboard.rs:2679` through `src/bin/audit_dashboard.rs:2694`.
  The evidence output shows the resulting `HEAD_t(...)` row at
  `command_0008_stdout.txt:388`.
- R1 finding #3: partially closed and ready for post-audit closeout. The R3
  manifest allowed paths cover the scoped touched files plus this audit artifact
  at `DevTaskManifest.json:15` through `:22`; `events.jsonl:2` records the diff;
  command_0002 through command_0014 all have `exit_code: 0`. I do not see
  validate/close events yet, but that is expected before this R3 audit artifact
  exists. The run should record this audit, validate, and close only after the
  CHALLENGE fixes are implemented and re-audited.

## Evidence Reviewed

- Approved plan:
  `handover/directives/2026-05-15_REAL5S_REAL6_REAL7_REAL8_REAL9_EXECUTION_PLAN_APPROVED.md`
- Prior R1 audit:
  `handover/audits/CODEX_REAL6D_IMPLEMENTATION_REVIEW.md`
- R3 harness manifest and event chain:
  `handover/evidence/dev_self_hosting/dev_1778831813745_1527867/DevTaskManifest.json`,
  `events.jsonl`, `events_hash_chain.json`
- R3 diff artifact:
  `handover/evidence/dev_self_hosting/dev_1778831813745_1527867/artifacts/diff.patch`
- Command evidence:
  - command_0002 `cargo fmt --all -- --check` exit 0
  - command_0003 `cargo test --test constitution_g6_observe_only` exit 0; 6 passed
  - command_0004 `cargo test --test constitution_g5_scheduler` exit 0; 3 passed
  - command_0005 `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo` exit 0; 1 passed
  - command_0006 `bash scripts/run_constitution_gates.sh` exit 0; 436 passed / 0 failed / 1 ignored
  - command_0007 `cargo test --workspace --no-fail-fast -- --test-threads=1` exit 0
  - command_0008 `audit_dashboard --run-report` over REAL-6A evidence exit 0
  - command_0009..0012 grep markers for Opportunity Scheduler, observe_only, non-binding, HEAD_t all exit 0
  - command_0013 scoped git status exit 0
  - command_0014 sha256sum relevant files exit 0

## Gate Checklist

- SG-6D.1 Scheduler trace includes price/PnL signals: CHALLENGE. The type and
  tests include both, but the real §J.1 evidence has PnL signals and zero price
  signals despite an accepted task-outcome market, and the trace is not emitted
  to the run CAS/ChainTape path.
- SG-6D.2 observe_only=true: PASS. `build_observe_only_scheduler_trace` forces
  `observe_only: true` at `src/runtime/agent_scheduler.rs:130` through `:140`;
  command_0008 renders `observe_only: true`.
- SG-6D.3 Recommendation does not change sequencer admission: PASS for the
  scoped REAL-6D diff. I found no sequencer admission, typed transaction schema,
  canonical signing payload, kernel, bus, wallet, or CAS schema source diff in
  the R3 diff artifact.
- SG-6D.4 Price does not affect L4/L4.E: PASS for the audited files and command
  evidence. The G6 predicate source gate passed, and I found no price-as-truth
  admission/predicate path in the scoped REAL-6D changes.
- SG-6D.5 Dashboard shows scheduler recommendation as non-binding: PASS.
  `src/runtime/agent_scheduler.rs:145` through `:148` render the non-binding
  banner, and command_0008 shows the same text at `command_0008_stdout.txt:385`
  through `:388`.

## Restricted Surface / Safety Checks

The R3 diff artifact contains only:

- `genesis_payload.toml`
- `src/bin/audit_dashboard.rs`
- `src/runtime/agent_scheduler.rs`
- `tests/constitution_g6_observe_only.rs`
- `handover/evidence/real6_scheduler_observe_only/REAL6D_OPPORTUNITY_SCHEDULER_REPORT.md`

Within that scoped diff, I found no direct mutation to `src/state/sequencer.rs`,
`src/state/typed_tx.rs`, `src/kernel.rs`, `src/bus.rs`,
`src/sdk/tools/wallet.rs`, `src/bottom_white/cas/schema.rs`, or canonical
signing payload surfaces. The only restricted surface hit recorded by the R3
harness is `genesis_payload.toml`.

I also found no REAL-6D-local evidence of price-as-truth, ghost liquidity,
`f64` economy logic, off-tape WAL truth, private CoT recording, or raw-log
broadcast.

## Residual Risks / Forward Notes

- The broader working tree is very dirty and includes many modified restricted
  files outside the scoped R3 diff. I did not attribute those to REAL-6D because
  the requested R3 diff artifact is scoped, but any ship review over the whole
  branch must audit those surfaces under their own Class-4 packets.
- The `HEAD_t(...)` row is still a string label, not a typed
  `HeadTWitness`. I treated it as acceptable for R1 #2 because it is now
  L4/L4.E/CAS/state/run-derived rather than `tx_count:*`; a later hardening pass
  should prefer the typed witness shape.
- After remediation, rerun the R3 gate set or a successor run, record the audit
  verdict, then validate and close the harness run.
