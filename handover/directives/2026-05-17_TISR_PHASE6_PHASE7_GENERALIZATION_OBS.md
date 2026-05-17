# OBS: Phase 7+ Task-Runner Generalization

**Date**: 2026-05-17
**Trigger**: Architect/user observation post-Phase-6.1 ship — Phase 6.1
wrappers leaked Lean-specific naming (`lean_market`, "Lean theorem-proving",
"TB-10 workflow") into user-facing help text, contradicting TuringOS's
positioning as a general agent OS.

## What was observed

77 mentions of `lean_market` across Phase 6.1 modules at the moment of the
observation. Concretely:

1. 7 `cmd_*.rs` wrapper modules called `run_external("lean_market", ...)`
   with the binary name hard-coded.
2. `SHORT_HELP` / `FULL_HELP` strings in those modules described the
   wrappers as shell-outs to `lean_market view-X` — surfacing the wrapped
   binary's identity in `turingos <subcommand> --help` output.
3. `cmd_init.rs` stdout pointed users to "established TB-10 workflow" via
   `./target/release/lean_market`, framing the proof/Lean backend as THE
   primary path rather than ONE backend among many.

## Why this is a problem

TuringOS positions itself as a **general agent OS for the AGI era**
(constitution.md identity; `handover/research/interaction_substrate/...`
research deliverables). minif2f and Lean theorem-proving are
**development-stage testing artifacts**, not permanent kernel content.

The seven operations wrapped by Phase 6.1 are themselves **task-type
agnostic**:
- `view-wallet` — replay agent EconomicState (any task type)
- `view-positions` — NodePositionsIndex exposure record (any market)
- `view-bankruptcy` — RunExhausted / Bankruptcy condition (any agent)
- `view-replay` — 7-indicator ChainTape verify (universal)
- `view-task` — task status from ChainTape (any task type)
- `run-task` — bootstraps ChainTape + TaskOpen + EscrowLock (any task)
- `tick` — G3 carry-forward epoch advance (universal)

They live in `experiments/minif2f_v4/src/bin/lean_market.rs` because TB-10
was the first ship vehicle that needed them. That's an **accident of
history**, not a permanent architectural choice.

## Phase 6.1 immediate fix (applied)

`src/bin/turingos/common.rs` now exports a single constant:
```rust
pub(crate) const TASK_RUNNER_BIN: &str = "lean_market";
```
with extensive doc-comment explaining the Phase 7+ generalization plan.

All 7 wrapper `cmd_*.rs` modules now:
1. Call `run_external(TASK_RUNNER_BIN, ...)` (single point of change).
2. Use domain-generic `SHORT_HELP` / `FULL_HELP` text — NO mentions of
   `lean_market`, `Lean`, `TB-10`, `minif2f`, or `proof` in user-facing
   output for these 7 wrappers.
3. Describe what the operation DOES (replay wallet / replay positions /
   advance epoch / etc.), not WHICH BINARY implements it.

`cmd_init.rs` "What you can run right now" section now leads with
TuringOS commands (`turingos report wallet`, `turingos task view`, etc.)
and mentions task-type backends as a separate, generalized section.

## Phase 7+ generalization plan

When the next architect §8 ratification window opens (post G-Phase
closeout + post-REAL-13/REAL-BCAST-1/REAL-13A ship), the following
sequence will retire the Lean-specific naming entirely:

1. **Extract generic kernel ops to root crate library**: move the
   ChainTape replay / wallet / positions / bankruptcy / task-lifecycle
   functions out of `experiments/minif2f_v4/src/bin/lean_market.rs` and
   into `src/runtime/` modules of the root `turingosv4` crate.
2. **Add generic `agent_runner` binary** at `src/bin/agent_runner.rs`
   that exposes the same subcommands (`view-wallet`, `view-positions`,
   ..., `run-task`, `tick`) but is task-type agnostic — task adapter
   selection happens via flag (`--task-adapter proof|polymarket|...`).
3. **Update `common.rs::TASK_RUNNER_BIN`** to `"agent_runner"`. This is
   the single point of change — all 7 wrappers route to the new binary
   automatically.
4. **Deprecate `lean_market`** to only handle Lean-SPECIFIC orchestration
   (Lean checker subprocess, minif2f problem corpus loading, .lean file
   parsing). Generic kernel ops are no longer its responsibility.
5. **Eventually retire `lean_market`** entirely once another task-type
   backend (e.g., polymarket / multi-agent / generic-compute) is shipped
   alongside `agent_runner`.

Each step is independent and can be ratified separately. None of them
require Trust Root file changes (Cargo.toml, Cargo.lock, src/lib.rs) —
the new `agent_runner` is a new `[[bin]]` entry, and the move of
functions into `src/runtime/` is additive within an already-pinned
library module tree.

## Why this matters for the future

TuringOS Phase 9+ vision (per `handover/research/interaction_substrate/`):
a substrate for **multi-task, multi-agent, multi-modal AGI-era
collaboration**. A user-facing CLI that hard-codes "lean_market" in
every help message would be a permanent embarrassment when the
substrate hosts polymarket trading, multi-agent task arenas, generic
compute pipelines, scientific research workflows, etc.

The Phase 6.1 fix locks the abstraction NOW so that the Phase 7+ rename
is mechanically safe. The user-facing surface area today reads as
"turingos report wallet replays agent wallet balances" — and that
sentence stays true forever, regardless of what binary implements it.

## Audit trail

- Phase 6.1 initial implementation: 77 `lean_market` mentions across
  user-facing surface (commit chain `f2fbfaed` → `e4d4b46b`).
- Architect observation: 2026-05-17 (post `e4d4b46b`).
- Phase 6.1 generalization fix: applies after `e4d4b46b`, before final
  ship. Touches only `src/bin/turingos/{common.rs, cmd_init.rs,
  cmd_report_{wallet,positions,bankruptcy}.rs, cmd_replay.rs,
  cmd_task_{open,view,tick}.rs}` + this OBS doc. No new pub items.
  All 76 Phase 6.1 tests + 5 init regression tests still pass.

FC-trace: FC2-N16 (boot / genesis / tape replay view — wrappers stay
on the FC2 read-view side; the generalization is architectural hygiene,
not a flowchart-node change).
