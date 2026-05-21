# A2 — Current trunk consumers of restored APIs

Phase A research output #2. Maps which current trunk files invoke / link / shell-to the restored binaries and library code, so future maintainers know the integration surface.

## R0 consumer surface — `lean_market` binary

### Resolver path

`src/bin/turingos/common.rs::run_external()` (line ~93) resolves the binary lookup chain:

```
$TURINGOS_BIN_DIR/<TASK_RUNNER_BIN>          # explicit override (preferred)
  → sibling-of-current-binary/<TASK_RUNNER_BIN>    # release/debug co-location
    → target/release/<TASK_RUNNER_BIN>
      → target/debug/<TASK_RUNNER_BIN>
        → PATH lookup
```

Constant `TASK_RUNNER_BIN = "lean_market"` lives at `src/bin/turingos/common.rs:68`.

### 7 orphan call sites (all in `src/bin/turingos/`)

After R0 (`2bf282ca`) restores `lean_market` to `experiments/minif2f_v4/target/{release,debug}/`, these resolve automatically (no code change on the resolver side):

| Call site | Subcommand invoked |
|-----------|---------------------|
| `cmd_task_open` | `lean_market run-task` |
| `cmd_task_view` | `lean_market view-task` |
| `cmd_task_tick` | `lean_market tick` |
| `cmd_replay` | `lean_market view-replay` |
| `cmd_report_positions` | `lean_market view-positions` |
| `cmd_report_wallet` | `lean_market view-wallet` |
| `cmd_report_bankruptcy` | `lean_market view-bankruptcy` |

### Env-var contract consumed by restored `lean_market.rs`

The restored binary consumes these env vars (all already populated by current trunk runtime):

- `TURINGOS_CHAINTAPE_PATH`
- `TURINGOS_CAS_PATH`
- `TURINGOS_CHAINTAPE_RESUME`
- `TURINGOS_RUN_ID`
- `TURINGOS_CHAINTAPE_QUEUE_CAPACITY`

These are emitted by `src/runtime/mod.rs::RuntimeChaintapeConfig::from_env()` bootstrap — no change needed.

### Smoke tests (5, all in trunk `tests/`)

After R0, set `TURINGOS_BIN_DIR=$(pwd)/experiments/minif2f_v4/target/debug` to make these green:

- `cli_task_open_smoke`
- `cli_task_view_smoke`
- `cli_report_wallet_smoke`
- `cli_report_positions_smoke`
- `cli_report_bankruptcy_smoke`

### Known acceptable limitation

`lean_market run-task` ENOENT on the missing `evaluator` binary (Tier 3, not restored). Architect accepted this — smoke tests don't exercise this path. Operators wanting solver loops use `turingos generate` / `turingos batch`.

## R1 consumer surface — `src/runtime/batch_orchestrator`

### Module registration

`src/runtime/mod.rs` includes `pub mod batch_orchestrator;` after R1 + hotfix (alphabetical position, preceding `batch_continuation_manifest`).

### Internal dependencies (all stable in current trunk)

- `crate::runtime::chain_tape_lease` (verified at `src/runtime/mod.rs:135`)
- `crate::runtime::resume_preflight` (verified at `src/runtime/mod.rs:125`)
- `crate::runtime::batch_continuation_manifest::{BatchContinuationManifest, TaskContinuationEntry}` (verified at `src/runtime/mod.rs:146`)

### Internal types (not wire-shaped)

- `TaskOutcome` — working buffer used during `write_manifest_skeleton()` (lines 380-445); maps cleanly to `TaskContinuationEntry` (the canonical wire shape). No rewriting needed.

### No external callers yet

R1 promotes the orchestrator to trunk lib but **no production callers have been re-wired** to consume it. Plan v7 intentionally stops at "promotion + tests green"; downstream wire-up (e.g. `cmd_batch` consuming `batch_orchestrator::prepare_task_boundary`) is a future task.

The 6 inline unit tests cover the helper logic; production wire-up will need:

- `cmd_batch` invocation of `batch_orchestrator::prepare_task_boundary(task_idx, prev_outcome)`
- env-var propagation for `TURINGOS_CHAINTAPE_RESUME` between adjacent tasks
- `verify_chain_continuity` invocation post-batch as the audit gate

## R2 consumer surface — workspace exclusion

### Root `Cargo.toml` `[workspace]` block

```toml
[workspace]
members = [".", "spike/gix_capability"]
exclude = ["experiments/minif2f_v4"]
```

### Why exclusion (vs inclusion)

`experiments/minif2f_v4/` is a separate Cargo workspace with its own `Cargo.lock` and `target/` dir. Including it in root workspace would:

1. Churn root `Cargo.lock` on every restoration commit (R0 changed `experiments/Cargo.lock`)
2. Pollute root `target/` with `lean_market` artifacts
3. Couple the two compilation domains (root + experiments share dependency resolution)

Exclusion preserves the **separate-workspace pattern** established when `experiments/minif2f_v4/` was first created (pre-Plan-v4 era).

## Cz consumer surface — Trust Root

### Pinned files

`genesis_payload.toml` `[trust_root]` pins these (relevant to Plan v7):

| File | Pre-Cz hash | Post-Cz-3 hash | Post-hotfix hash |
|------|-------------|----------------|-------------------|
| `Cargo.toml` | `e5b61b03...` | `f533ed57...` | (unchanged from Cz-3) |
| `src/runtime/mod.rs` | `9a5038bf...` | `555c6f52...` | `39c5e227...` |

### `boot::tests::verify_trust_root_passes_on_intact_repo`

This test reads `genesis_payload.toml`'s `[trust_root]` table, hashes each file, and asserts equality. Failure = Trust Root drift = boot blocked.

Post-hotfix: all hashes match → test passes → boot OK.

## §8 self-sign

Class 0 (research archive). Self-signed by Claude opus 4.7 under existing user delegation (2026-05-21).
