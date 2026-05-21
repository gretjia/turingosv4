# A1 — Deletion map (commit 309e026a) and Tier classification

Phase A research output #1. Maps every file deleted by commit `309e026a` ("Phase C3 + minif2f cleanup") to one of three tiers based on whether current trunk has a consumer / equivalent.

## Scope of deletion

```
commit 309e026a: Phase C3 + minif2f cleanup
  57 files changed, ~26,880 lines removed (39 .rs files + tests + scripts + configs)
```

All under `experiments/minif2f_v4/` (its own previously-separate Cargo workspace).

## Tier 1 — must restore (orphan consumers in current trunk)

These deletions caused **runtime breakage in current trunk** because trunk binaries shell to a removed binary.

### Restored files (R0, commit `2bf282ca`)

| File | LoC | Reason |
|------|-----|--------|
| `experiments/minif2f_v4/src/bin/lean_market.rs` | 848 | 7-subcommand CLI. `src/bin/turingos/common.rs:68` declares `TASK_RUNNER_BIN = "lean_market"`. 7 orphan call sites in `src/bin/turingos/`: `cmd_task_open`, `cmd_report_positions`, `cmd_report_wallet`, `cmd_report_bankruptcy`, `cmd_task_view`, `cmd_task_tick`, `cmd_replay`. |
| `experiments/minif2f_v4/Cargo.toml` | 17 | Restored as a stripped-down version (lean_market `[[bin]]` only, deps reduced to `turingosv4` + `serde` + `serde_json`). Dropped `evaluator` / `batch_evaluator` `[[bin]]` entries (Tier 3). |
| `experiments/minif2f_v4/src/lib.rs` | 6 | Minimal stub (no `pub mod`s, since `lean_market.rs` imports only `turingosv4::*`). |

### Smoke tests resurrected (5 tests, formerly ENOENT)

After R0 landed and `TURINGOS_BIN_DIR=$(pwd)/experiments/minif2f_v4/target/debug` was set, the 5 broken smoke tests turn green:

- `cli_task_open_smoke`
- `cli_task_view_smoke`
- `cli_report_wallet_smoke`
- `cli_report_positions_smoke`
- `cli_report_bankruptcy_smoke`

## Tier 2 — architectural gap (no current trunk equivalent)

These deletions removed **the only implementation** of a published Plan v7 directive. Restored to trunk lib (promoted out of `experiments/`).

### Restored files (R1, commit `6148a0cd`)

| File | LoC | Reason |
|------|-----|--------|
| `experiments/minif2f_v4/src/batch_orchestrator.rs` → `src/runtime/batch_orchestrator.rs` | 583 | Multi-task `prior_outcome` carry-forward + `verify_chain_continuity`. `BatchContinuationManifest` exists in `src/runtime/` but only as post-facto tracking, never drove resumption. Promoted to trunk lib (not kept in `experiments/`) because it's authoritative runtime, not experimental. |

### Adapter changes — none required

R1 imports verified stable in current trunk:
- `turingosv4::runtime::chain_tape_lease`
- `turingosv4::runtime::resume_preflight`
- `turingosv4::runtime::batch_continuation_manifest::{BatchContinuationManifest, TaskContinuationEntry}`

The internal `TaskOutcome` working buffer maps cleanly to `TaskContinuationEntry` inside `write_manifest_skeleton()` (lines 380-445); no rewriting needed.

## Tier 3 — NOT restored (already migrated in-process)

Per architect's choice (option a, AskUserQuestion 2026-05-22): no current consumer = no restoration. Listed here for grep-ability so future sessions don't re-investigate.

### Migrated equivalents

| Deleted file | Replacement in current trunk |
|--------------|------------------------------|
| `experiments/minif2f_v4/src/chain_runtime.rs` | `src/runtime/mod.rs::ChaintapeBundle` (env-var bootstrap) |
| `experiments/minif2f_v4/src/bin/comprehensive_arena.rs` | `src/state/sequencer.rs` + REAL5 modules |
| `experiments/minif2f_v4/src/drive_task.rs` | in-process evaluator loop in `src/runtime/` |
| `experiments/minif2f_v4/src/agent_models.rs` | `src/runtime/real5_roles.rs` |

### Utilities with no current consumer

```
experiments/minif2f_v4/src/{experiment_mode,budget_regime,per_call_budget,
  cost_aggregator,fc_trace,h_vppu_history,jsonl_schema,lean4_oracle,
  post_hoc_verifier,rollback_sim,wall_clock,chaintape_mode_gate,run_id}.rs
```

### Large binaries deferred

| File | LoC | Why deferred |
|------|-----|--------------|
| `experiments/minif2f_v4/src/bin/evaluator.rs` | 9931 | No current product surface. `lean_market run-task` ENOENTs on this when invoked, which the architect accepted (smoke tests don't exercise `run-task`). |
| `experiments/minif2f_v4/src/bin/batch_evaluator.rs` | — | Same — no current consumer. |

### Tests + scripts + configs

All 14 deleted test files under `experiments/minif2f_v4/tests/`, all scripts under `experiments/minif2f_v4/scripts/`, `examples/`, `bench/` — not restored.

## When restoration was chosen

Tier 1 + Tier 2 only, because:

1. **Tier 1** — current trunk binaries hard-coded `TASK_RUNNER_BIN = "lean_market"` at `src/bin/turingos/common.rs:68`. ENOENT on every `turingos task open` / `view` / `tick` invocation until restored. 5 smoke tests in trunk shell out to this binary.
2. **Tier 2** — `batch_orchestrator.rs`'s `prior_outcome` carry-forward had no current equivalent. `BatchContinuationManifest` exists but as post-facto tracking only.
3. **Tier 3** — already migrated in-process (chain_runtime → ChaintapeBundle, drive_task → evaluator loop, agent_models → real5_roles, comprehensive_arena → sequencer+REAL5). Restoring them would create **double-implementations** — direct violation of "不要再创新了".

## §8 self-sign

Class 0 (research archive). Self-signed by Claude opus 4.7 under existing user delegation (2026-05-21).
