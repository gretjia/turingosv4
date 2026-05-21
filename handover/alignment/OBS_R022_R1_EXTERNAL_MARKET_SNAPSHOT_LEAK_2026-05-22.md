# OBS R-022 — Orphan `pub mod external_market_snapshot` removal from R1

| Field | Value |
|-------|-------|
| Date | 2026-05-22 |
| Triggered by | R-022 (TRACE_MATRIX pub-symbol-block) pre-commit hook blocking removal of a `/// TRACE_MATRIX` doc comment in `src/runtime/mod.rs` |
| Detected removal | `/// TRACE_MATRIX FC1/FC3 (REAL-WORLD Polymarket Phase A 2026-05-21): external market snapshot sidecar.` + 3 sibling lines + `pub mod external_market_snapshot;` declaration |
| Hotfix branch | `hotfix/r1-mod-rs-leak` |
| Risk class | 1 (CI fix on own R1 PR; 4-line removal, no schema change) |

## Why this removal is correct, not an orphan-deletion

### Background

Plan v7 R1 (`src/bin/turingos/commit 6148a0cd` "feat(runtime): promote batch_orchestrator from minif2f experiments") was a verbatim restoration of the deleted `experiments/minif2f_v4/src/batch_orchestrator.rs` (583 LoC) into `src/runtime/batch_orchestrator.rs`, plus a single-line `pub mod batch_orchestrator;` append to `src/runtime/mod.rs`.

The R1 sub-agent ran in an isolated worktree (`.claude/worktrees/1779395231-c08f1a81`). That worktree's `src/runtime/mod.rs` had previously been polluted by Codex's parallel Polymarket Phase A session — a 4-line block:

```rust
/// TRACE_MATRIX FC1/FC3 (REAL-WORLD Polymarket Phase A 2026-05-21): external
/// market snapshot sidecar. Additive Generic CAS evidence only; public price
/// is an agent signal, never resolution truth, wallet authority, or order flow.
pub mod external_market_snapshot;
```

The corresponding `src/runtime/external_market_snapshot.rs` was **never tracked in git** — it lived only as an untracked file in Codex's WIP. When R1 squash-merged into main, the polluted mod.rs landed but the .rs file did not.

### CI evidence

Run 26252416772 on commit `6148a0cd`:

```
[error]E0583: file not found for module `external_market_snapshot`
   --> src/runtime/mod.rs:184:1
```

The Constitution Gates workflow couldn't compile any test binary because the trunk lib fails to compile. Trust Root verification (boot::tests::verify_trust_root_passes_on_intact_repo) also failed because rustc never reached the test.

### Why R-022 fires

The removed 4-line block contains a `/// TRACE_MATRIX FC1/FC3 (...)` doc comment + a `pub mod external_market_snapshot;` declaration. R-022 enforces that TRACE_MATRIX-annotated pub symbols cannot be silently removed — every removal must either (a) add an equivalent backlink, (b) register in TRACE_MATRIX_v3.md §J, or (c) cite an OBS_R022_*.md file in the commit message.

This OBS file satisfies (c).

### Why removal (not addition) is the right call

The choice was:
- **A. Add Codex's `external_market_snapshot.rs` file** — would require Claude opus to take ownership of Codex's in-progress Polymarket work. Codex has its own session (verified via `pgrep -af codex`) and may be actively iterating on the file. Committing it would race / conflict.
- **B. Remove the orphan mod declaration** — restores compile to green; respects Codex's separate work boundary; if Codex lands their PR later, they re-add the mod alongside the .rs file in a single commit.

Option B is chosen. The TRACE_MATRIX backlink for the FC1/FC3 Polymarket sidecar will be re-added by Codex when they land `external_market_snapshot.rs` as a real commit (the doc comment text can be copy-pasted from this OBS or from the prior R1 commit `6148a0cd`).

## Files modified by this hotfix

- `src/runtime/mod.rs` — removed 4-line orphan block (lines 181-184 of pre-hotfix state). New SHA256: `39c5e2273565c4379e8c0f1477db80a696e2548447f36de143d17970dca0f174`. Predecessor `555c6f52`.
- `genesis_payload.toml` — `[trust_root]."src/runtime/mod.rs"` pin updated `555c6f52 → 39c5e227`. Cz cycle 3-hotfix comment appended.

## Verification (pre-merge, local)

```
cargo check --lib                                                    PASS (0 errors)
cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo PASS (1/1)
cargo test --lib runtime::batch_orchestrator                          PASS (6/6)
```

## Cross-references

- Plan v7 design doc: `.claude/plans/minif2f-multi-agents-wise-scone.md`
- R1 PR (now broken on main pending this hotfix): commit `6148a0cd`
- CI failure run: 26252416772 + 26252627482 (both failed; both same root cause)
- Codex parallel PR train (no overlap with this hotfix scope): PR #78 boundary hygiene runner, PR #79 handover update, both already merged

## Future prevention

The R1 sub-agent's worktree did not isolate untracked files from the main repo. Future sub-agent dispatches that copy "verbatim" trunk files should:

1. Reset the trunk file to `git show HEAD:<path>` before copying — captures only tracked content.
2. Or restrict copy scope to file:line ranges, not whole files.

The Plan v7 retro already noted "autonomous orchestrator + Codex parallel writes to main need coordination." This OBS is one concrete instance of that exposure.

## §8 self-sign

Class 1 (additive UX / 4-line removal). No §3.1 forbidden surface touched. Trust Root pin update is Cz cycle 3-hotfix (single-line, identical pattern to 6 prior Cz cycles documented inline in genesis_payload.toml's `Cargo.toml` and `src/runtime/mod.rs` pins). Self-signed by Claude opus 4.7 under existing user delegation (2026-05-21).
