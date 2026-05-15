# REAL-6D Opportunity Scheduler Observe-Only Report

## Scope

Architect target:

```text
REAL-6D — Opportunity Scheduler Observe-Only
目标

价格 / PnL 进入调度观察，但不执行 admission change。
```

Implemented:

- Added `SchedulerDecisionTrace` and `SchedulerPnlSignal` to
  `src/runtime/agent_scheduler.rs`.
- Added `SCHEDULER_DECISION_TRACE_SCHEMA_ID` plus CAS writer/reader helpers:
  `write_scheduler_decision_trace_to_cas` and
  `read_scheduler_decision_trace_from_cas`.
- Added `build_observe_only_scheduler_trace` with `observe_only=true`.
- Added `render_scheduler_trace_section` with an explicit non-binding
  materialized-view interpretation.
- Wired `audit_dashboard --run-report` to render `## §J.1 Opportunity
  Scheduler recommendation (observe-only)` from ChainTape-derived market
  activity and PnL rows.
- Wired the production evaluator prompt loop behind
  `TURINGOS_REAL6_SCHEDULER_OBSERVE_ONLY=1` so each real run turn writes a
  `SchedulerDecisionTrace` into the run CAS. Missing QState or ChainTape bundle
  fails closed instead of emitting dashboard-only evidence.
- R1 audit closure: dashboard `head_t` is no longer `tx_count:*`; it is a
  `HEAD_t(...)` witness label derived from L4 head, L4.E last hash, CAS merkle
  root, replay state root, and run_id.
- R3 audit closure: dashboard scheduler price signals now include REAL-6A
  TaskOutcomeMarket pools (`task-*` / future `task_outcome:*`), not only
  TB-N3 `node_survive:*` markets.
- Rehashed Trust Root for the modified pinned
  `src/bin/audit_dashboard.rs` and
  `experiments/minif2f_v4/src/bin/evaluator.rs`.

Not changed:

- No sequencer admission change.
- No L4 / L4.E predicate change.
- No typed transaction schema, discriminant, or signing payload change.
- No price-as-truth.
- No ghost liquidity.
- No f64 money path.

## Ship Gates

| Gate | Evidence |
| --- | --- |
| SG-6D.1 Scheduler trace includes price/PnL signals | `tests/constitution_g6_observe_only.rs::sg_6d_1_and_6d_2_scheduler_trace_carries_price_and_pnl_observe_only`; CAS/ChainTape evidence path pinned by `sg_6d_1_scheduler_trace_is_chain_backed_cas_evidence` |
| SG-6D.2 observe_only=true | same test asserts `trace.observe_only` |
| SG-6D.3 Recommendation does not change sequencer admission | source gate checks `src/runtime/agent_scheduler.rs` for admission/sequencer mutation tokens |
| SG-6D.4 Price does not affect L4/L4.E | existing G6 predicate source gate plus scheduler source gate |
| SG-6D.5 Dashboard shows scheduler recommendation as non-binding | test checks renderer; R4 harness command_0017 renders dashboard over REAL-6D smoke evidence and command_0018 asserts non-binding markers |

## Harness Evidence

R1 audit found the first harness package incomplete:

```text
dev_1778828648899_1429709
  superseded by R2 because allowed_paths only captured src/runtime/agent_scheduler.rs,
  SchedulerDecisionTrace had no CAS/ChainTape writer, and dashboard head_t used tx_count.
```

R2 clean harness attempt:

```text
dev_1778831233293_1499773
  superseded by R3 because command_0009 recorded a malformed grep invocation
  (shell quoting error in evidence collection, not a source/test failure).
```

R3 clean harness:

```text
dev_1778831813745_1527867

command_0002: cargo fmt --all -- --check
  exit 0

command_0003: cargo test --test constitution_g6_observe_only
  exit 0

command_0004: cargo test --test constitution_g5_scheduler
  exit 0

command_0005: cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo
  exit 0

command_0006: bash scripts/run_constitution_gates.sh
  exit 0, 436 passed / 0 failed / 1 ignored

command_0007: cargo test --workspace --no-fail-fast -- --test-threads=1
  exit 0

command_0008: cargo run --bin audit_dashboard -- --repo
  handover/evidence/g_phase_real_6a_task_outcome_smoke_r10_20260515T0442Z/runtime_repo
  --cas handover/evidence/g_phase_real_6a_task_outcome_smoke_r10_20260515T0442Z/cas
  --run-report
  exit 0

command_0009..0012: grep Opportunity Scheduler / observe_only /
  non-binding / HEAD_t marker rows from command_0008 dashboard output
  exit 0

command_0013: scoped git status
  exit 0

command_0014: sha256sum relevant files
  exit 0
```

R3 clean-context implementation audit:

```text
handover/audits/CODEX_REAL6D_IMPLEMENTATION_REVIEW_R3.md
  verdict=CHALLENGE
  finding 1: SchedulerDecisionTrace still lacked production run CAS emission.
  finding 2: Dashboard scheduler price_signals ignored REAL-6A TaskOutcomeMarket pools.
```

R4 clean harness:

```text
dev_1778833170928_1564703

command_0002: cargo fmt --all -- --check
  exit 1, expected formatting failure captured for src/runtime/agent_scheduler.rs

command_0003: cargo fmt --all -- --check
  exit 0 after scoped formatting fix

command_0004: cargo test --test constitution_g6_observe_only
  exit 0

command_0005: cargo test --test constitution_g5_scheduler
  exit 0

command_0006: cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo
  exit 0

command_0007: bash scripts/run_constitution_gates.sh
  exit 1 because minif2f_v4::constitution_g1_2_subprocess_resume exposed an
  evaluator compile-scope error (`bundle` unavailable in the new REAL-6D hook)

command_0008: cargo test -p minif2f_v4 --test constitution_g1_2_subprocess_resume -- --nocapture
  exit 101, reproduced the evaluator compile error

command_0009: cargo fmt --all -- --check
  exit 0 after moving the hook to use chaintape_bundle.as_ref() with fail-closed
  handling

command_0010: cargo test -p minif2f_v4 --test constitution_g1_2_subprocess_resume -- --nocapture
  exit 0

command_0011: cargo test --test constitution_g6_observe_only
  exit 0

command_0012: cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo
  exit 0

command_0013: bash scripts/run_constitution_gates.sh
  exit 0, 436 passed / 0 failed / 1 ignored

command_0014: cargo test --workspace --no-fail-fast -- --test-threads=1
  exit 0

command_0015: REAL-6D smoke launch without low-disk override
  exit 4, runner-preflight blocked because / had 19G free vs architect 20G

command_0016: REAL-6D smoke with TURINGOS_G_PHASE_LOW_DISK_OK=1
  exit 0
  evidence:
    handover/evidence/g_phase_real_6d_scheduler_observe_only_r4b_20260515T0830Z
  audit_tape:
    verdict=PROCEED passed=41 failed=0 halted=0 skipped=11
  persistence:
    is_passing=true n_witnessed=3

command_0017: audit_dashboard --run-report over the R4 smoke
  exit 0

command_0018: dashboard marker assertion
  exit 0
  observed:
    price_signals=1
    pnl_signals=13
    persisted_scheduler_trace_cas_count=5
    observe_only=true
    non-binding label present
    HEAD_t(...) present

command_0019: grep real6.scheduler_decision_trace.v1 in R4 run CAS index
  exit 0
  observed:
    5 SchedulerDecisionTrace CAS objects, one per Agent_0..Agent_4 turn
```

## Interpretation

REAL-6D lets price and PnL become scheduler-visible signals. It does not make
the scheduler authoritative. The recommendation is a dashboard/report witness
only, and does not affect transaction admission or predicate outcomes. The
trace type now has an explicit CAS evidence path; CAS writes advance
`refs/chaintape/cas` through `CasStore::put`. The R4 smoke demonstrates that
the production evaluator emits those traces into the run CAS when
`TURINGOS_REAL6_SCHEDULER_OBSERVE_ONLY=1`, while the dashboard remains a
regenerated materialized view over ChainTape/CAS evidence.
