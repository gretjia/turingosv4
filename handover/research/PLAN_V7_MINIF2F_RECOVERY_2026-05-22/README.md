# Plan v7 MiniF2F Recovery Research Archive — 2026-05-22

| Field | Value |
|-------|-------|
| Date | 2026-05-22 |
| Trigger | Architect's directive: "minif2f 时代做了大量的测试，我建议你还是尽可能把之前的代码找回来，用到现在的架构里。**不要再创新了**" + follow-up "lean_market 如果是关于 polymarket 的，也要抢救" |
| Phase | A (research × 3) → 5-atom design (R0/R1/R2/Cz/R3) → hotfix |
| Decision outcome | Tier 1 + Tier 2 restoration only; Tier 3 NOT restored (architect chose strict no-innovation). All 5 atoms shipped + 1 hotfix shipped. |
| Final commits | R0 `2bf282ca`, R1 `6148a0cd`, R2+Cz `7f61605d`, hotfix `cff03a28` |
| Plan doc | `.claude/plans/minif2f-multi-agents-wise-scone.md` (approved Plan Mode 2026-05-22) |

## Why this archive exists

The 2026-05-22 MiniF2F recovery effort cost ~3 sub-agent Explore dispatches + 3 sub-agent atom dispatches + 1 hotfix cycle. Future TuringOS sessions revisiting:

- "Why is `experiments/minif2f_v4/` a separate Cargo workspace?"
- "What was Tier 3 and why didn't we restore it?"
- "How did the R1 sub-agent leak Codex parallel WIP into mod.rs?"
- "What's the R-022 commit-hook footgun?"

should **read this archive first** instead of re-running the audit.

## Files in this archive

| File | What it covers | When to re-read |
|------|----------------|-----------------|
| `A1_explore_deletion_map.md` | The 57 deleted files from commit `309e026a` (39 .rs, ~26880 LoC) categorized into Tier 1 (must restore), Tier 2 (architectural gap), Tier 3 (already migrated in-process). Includes the 7 orphan CLI call sites + 5 broken smoke tests. | Before re-deleting / re-migrating any `experiments/minif2f_v4/` content. |
| `A2_explore_current_consumers.md` | Mapping of which current trunk files consume which restored APIs. Confirms `ChaintapeBundle::from_env()` env-var contract is the integration point; `src/bin/turingos/common.rs::run_external()` resolves `TASK_RUNNER_BIN = lean_market` via `$TURINGOS_BIN_DIR`. | Before changing the env-var contract or moving `lean_market` again. |
| `A3_explore_api_drift_audit.md` | Per-import audit of `lean_market.rs` and `batch_orchestrator.rs` against current public APIs. All imports verified stable (no Class 1 visibility flips needed). | Before any future restoration from `git show 309e026a^:...` (the audit is the basis for "verbatim restore is safe"). |
| `B_5_ATOM_DESIGN_AND_OUTCOMES.md` | The 5-atom decomposition (R0 lean_market / R1 batch_orchestrator / R2 workspace exclude / Cz Trust Root rehash / R3 verify), dependency order, risk classes, and final ship commits. | Before designing future multi-atom restorations. |
| `C_HOTFIX_R1_LEAK_POSTMORTEM.md` | The R1 sub-agent worktree leak: Codex Polymarket Phase A WIP `external_market_snapshot.rs` declaration in `src/runtime/mod.rs` without the `.rs` file → E0583 CI failure → hotfix. Includes the R-022 commit-hook footgun (pre-commit reads `.git/COMMIT_EDITMSG` but `git commit -F` only populates COMMIT_EDITMSG *after* hooks pass). | Before any future sub-agent dispatch that copies "verbatim" trunk files (need worktree-isolation discipline) AND before any future R-022 hotfix (pre-populate COMMIT_EDITMSG via `cp`). |

## Cross-references

- Plan doc: `.claude/plans/minif2f-multi-agents-wise-scone.md`
- Hotfix OBS: `handover/alignment/OBS_R022_R1_EXTERNAL_MARKET_SNAPSHOT_LEAK_2026-05-22.md`
- Predecessor plans: `handover/research/PLAN_V6_TAPE_RELAY_2026-05-22/` (Plan v6, tape-relay validation)
- Architect directive verbatim: see `B_5_ATOM_DESIGN_AND_OUTCOMES.md §1`

## Triggers that would reopen this debate

1. **`lean_market run-task` ENOENT on `evaluator`** — Architect accepted this. If a future user needs the full solver loop (not just the 5 view subcommands), Tier 3 `evaluator.rs` (9931 LoC) becomes a candidate to restore. Smoke tests do not exercise `run-task`.
2. **Codex re-adds `external_market_snapshot.rs`** — When Codex Polymarket Phase A lands as a real PR with the `.rs` file, the `pub mod` + `/// TRACE_MATRIX` block re-appears in `src/runtime/mod.rs`. The doc-comment text in `C_HOTFIX_R1_LEAK_POSTMORTEM.md §captured-text` is the canonical source to copy-paste from.
3. **Tier 3 file resurrection** — `chain_runtime.rs`, `comprehensive_arena.rs`, `drive_task.rs`, `agent_models.rs` are confirmed already-migrated in-process. Restoring them creates double-implementations. If a future architect changes mind, re-read `A1_explore_deletion_map.md §Tier3` first — they must explicitly displace the current migrated equivalents.
4. **R-022 commit-hook fix** — `scripts/check_trace_matrix.py --mode commit` reads `.git/COMMIT_EDITMSG`, which is empty / stale during non-interactive `git commit -F file` and `git commit -m msg`. Workaround: `cp /tmp/msg.txt .git/COMMIT_EDITMSG && git commit -F /tmp/msg.txt`. A proper fix is to add `--message` / `--message-file` flag to the script. See `C_HOTFIX_R1_LEAK_POSTMORTEM.md §footgun` for the full reproduction.

## How to consume this archive on a future session

1. Read this README + `B_5_ATOM_DESIGN_AND_OUTCOMES.md` for the binding decision and atom ship facts.
2. If your current question is "what's in `experiments/minif2f_v4/`" → it's a separate Cargo workspace with `lean_market` only (R0). See `A1 §Tier1`.
3. If asking "where is `batch_orchestrator.rs`" → at `src/runtime/batch_orchestrator.rs` since R1 (`6148a0cd`); registered in `src/runtime/mod.rs::pub mod batch_orchestrator;`. See `A1 §Tier2`.
4. If asking "why is there a Trust Root pin for `Cargo.toml`" → Cz cycle 3 rehash after R2 added `exclude = ["experiments/minif2f_v4"]`. See `B §Cz`.
5. If asking "why did CI break right after R1 merged" → R1 sub-agent worktree leak; see `C_HOTFIX_R1_LEAK_POSTMORTEM.md`.
6. Only re-dispatch Explore agents if the **codebase has shifted significantly** (R0/R1 files moved, public APIs changed, new Tier 3 candidates appeared). Otherwise the A1/A2/A3 findings remain current.

## §8 self-sign

Class 0 (docs archive). Self-signed by Claude opus 4.7 under existing user delegation (2026-05-21).
