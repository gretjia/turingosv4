# A11 Scheduler/Search Clean-Context Audit

Date: 2026-06-06

Workspace: `/home/zephryj/projects/turingosv4-a11-scheduler-search-policies`

Branch: `codex/a11-scheduler-search-policies`

Base: `7ad21702c62788a1f1587065c48ba38a7b2b6e0a`

Task: A11 Scheduler and Search Policies.

Risk class: Class 2.

Touched FC nodes/invariants:
- FC1-N7
- FC1-N13
- FC1 routing
- L6 price projection boundary

Required invariant summary:
- Scheduler remains observe-only.
- Price is an optional policy signal, not predicate truth.
- No sequencer admission authority.
- No typed tx authority.
- No canonical signing payload, trust-root, or genesis edit.
- No real parallel writer.
- No budget/economy mutation.

## Verification Evidence Provided To Witness

- `rustfmt --edition 2021 src/runtime/agent_scheduler.rs tests/scheduler_policy_trace.rs tests/scheduler_softmax_distribution.rs tests/scheduler_parallel_isolation.rs tests/scheduler_forced_loop_bounds.rs` => exit 0
- `cargo test --test scheduler_policy_trace --no-fail-fast -- --test-threads=1` => 3 passed
- `cargo test --test scheduler_softmax_distribution --no-fail-fast -- --test-threads=1` => 3 passed
- `cargo test --test scheduler_parallel_isolation --no-fail-fast -- --test-threads=1` => 2 passed
- `cargo test --test scheduler_forced_loop_bounds --no-fail-fast -- --test-threads=1` => 2 passed
- `cargo test -p turingosv4 --lib boot::tests::verify_trust_root_passes_on_intact_repo -- --exact` => 1 passed
- `cargo test -p turingosv4 --test fc_alignment_conformance fc3_n34_readonly_guard_verify_trust_root_intact_repo -- --exact` => 1 passed
- `cargo test --test constitution_g5_scheduler --no-fail-fast` => 3 passed
- `cargo test --test constitution_g6_observe_only --no-fail-fast` => 6 passed
- `cargo test --test constitution_production_module_liveness --no-fail-fast -- --test-threads=1` => 21 passed
- `git diff --check` => exit 0
- `cargo test --test constitution_matrix_drift --no-fail-fast` => 3 passed
- `bash scripts/run_constitution_gates.sh` => `[k-1-5] total=167 failed=0`, exit 0
- `cargo test --workspace --no-fail-fast` => exit 0

## Witness Protocol

Fresh `claude` CLI invocation:

```bash
claude --print --output-format json \
  --no-session-persistence \
  --add-dir /home/zephryj/projects/turingosv4-a11-scheduler-search-policies \
  --tools "Read,Grep,Bash" \
  --disallowedTools "Edit,Write,MultiEdit" \
  --permission-mode bypassPermissions \
  --effort high \
  --max-budget-usd 1
```

The witness was instructed to perform read-only inspection only, to ignore
subjective style/performance/coverage opinions unless they were constitutional
blockers, and to use the Class 2 verdict domain:

- `NO-VIOLATION`
- `VIOLATION-FOUND <constitutional-clause> <file>:<line>`
- `RECONSTRUCTION-FAILURE <which-tape-or-cas-path-cannot-be-reconstructed>`
- `SECOND-SOURCE-DRIFT <which-derived-view-is-usurping-ground-truth>`

## Witness Result

Final verdict:

```text
NO-VIOLATION
```

Witness findings: none.

Checked invariants reported by witness:

- The A11 diff touches only `src/runtime/agent_scheduler.rs` and four new
  scheduler tests.
- No Class 4 surface is modified: no `src/lib.rs`, sequencer, typed tx, CAS
  schema, genesis, canonical signing payload, kernel, bus, or wallet changes.
- `src/runtime/agent_scheduler.rs` is not trust-root pinned.
- Scheduler helpers only write CAS `ObjectType::Generic` objects and do not
  mutate `QState`, sequencer admission, bus, or ledger append paths.
- Scheduler events are replay-reconstructable from CAS CIDs and canonical
  hashes and do not become board-as-truth or a global latest pointer.
- Price remains an optional integer policy signal and defaults to a neutral
  weight when absent.
- Money/economy paths are not mutated.
- Parallel lane views are pure public read views and shield private error
  context.
- Tests cover replay hash binding, tamper/missing fail-closed behavior,
  softmax distribution versus argmax collapse, public lane isolation, forced
  loop bounds, and sequencer/typed-tx isolation.
