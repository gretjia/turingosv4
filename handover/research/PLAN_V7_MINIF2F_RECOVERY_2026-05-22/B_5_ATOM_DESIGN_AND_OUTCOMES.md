# B — 5-atom design + outcomes

The 5-atom decomposition that shipped Plan v7. Dependency order: R0 → R1 → R2 → Cz → R3.

## 1. Architect directive (verbatim)

```
minif2f 时代做了大量的测试，我建议你还是尽可能把之前的代码找回来，用到现在的架构里。
不要再创新了。派 multi-agents 做一个完整的补救计划。
```

Follow-up:
```
lean_market 如果是关于 polymarket 的，也要抢救。
```

Tier 3 disposition (via AskUserQuestion):
```
(a) 不恢复（如 Plan 推荐）
```

→ **Strict no-innovation, restore Tier 1 + Tier 2 only.**

## 2. Atom R0 — `lean_market` binary (Risk class 2)

### Files

- `experiments/minif2f_v4/src/bin/lean_market.rs` ← `git show 309e026a^:...` (848 LoC verbatim)
- `experiments/minif2f_v4/Cargo.toml` — 17 lines, lean_market-only `[[bin]]`, deps `turingosv4 = { path = "../.." }` + `serde` + `serde_json`
- `experiments/minif2f_v4/src/lib.rs` — 6-line stub

### Ship

- Commit `2bf282ca` "feat(experiments): restore lean_market binary from git history (R0)"
- PR #82 (after PR #81 closed due to push race)
- §8 self-signed under delegation 2026-05-21

### Verification

- `cargo build --bin lean_market` in `experiments/minif2f_v4/`: PASS
- `lean_market --help`: exit 0
- 5 smoke tests (with `TURINGOS_BIN_DIR=$(pwd)/experiments/minif2f_v4/target/debug`): GREEN

## 3. Atom R1 — `batch_orchestrator.rs` promotion (Risk class 2)

### Files

- `src/runtime/batch_orchestrator.rs` ← `git show 309e026a^:experiments/minif2f_v4/src/batch_orchestrator.rs` (583 LoC verbatim, COPY DESTINATION promoted to trunk lib)
- `src/runtime/mod.rs` — one-line append `pub mod batch_orchestrator;`

### Ship

- Commit `6148a0cd` "feat(runtime): promote batch_orchestrator from minif2f experiments (R1)"
- PR #83 (after PR #80 closed due to push race)

### Verification

- `cargo test --lib runtime::batch_orchestrator`: PASS 6/6
- Unit tests covered: fresh-genesis-boundary, env resume flag for k>0, env omit for k=0, continuity gap detection, continuity accept, manifest carries role-assignment

### Defect introduced

R1 sub-agent worktree leaked Codex Polymarket Phase A WIP (4-line `pub mod external_market_snapshot;` block) into `src/runtime/mod.rs`. The corresponding `.rs` file was never tracked → CI broke with E0583 on main. See `C_HOTFIX_R1_LEAK_POSTMORTEM.md`.

## 4. Atom R2 — Workspace exclude (Risk class 1)

### Files

- `Cargo.toml` — single-line add `exclude = ["experiments/minif2f_v4"]` to `[workspace]`

### Why

Separate-workspace pattern: `experiments/minif2f_v4/` has its own `Cargo.lock` + `target/`. Inclusion would churn root lockfile per restoration commit. Exclusion is the historical pattern (already blessed in commit `2026-05-17 workspace exclude` for prior cleanup).

### Ship

Squash-merged with Cz into commit `7f61605d`.

## 5. Atom Cz — Trust Root rehash (Risk class 4)

### Files

- `genesis_payload.toml`:
  - Line 136: `[trust_root]."Cargo.toml"` pin `e5b61b03 → f533ed57`
  - Line 210: `[trust_root]."src/runtime/mod.rs"` pin `9a5038bf → 555c6f52`
  - Append Cz cycle 3 rehash comment (same pattern as 5 prior Cz cycles)

### Ship

- Commit `7f61605d` "chore(workspace,trust-root): exclude experiments/minif2f_v4 + Cz cycle 3 (R2+Cz)"
- §8 self-signed under existing delegation 2026-05-21 (Class 4 explicitly authorized for Cz pattern repetition)

### Verification

- `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo`: PASS

## 6. Atom R3 — End-to-end verification (Risk class 1)

No file mutations. Pure verification:

```bash
cargo build --bin turingos                                          # PASS
(cd experiments/minif2f_v4 && cargo build --bin lean_market)        # PASS

TURINGOS_BIN_DIR=$(pwd)/experiments/minif2f_v4/target/debug \
  cargo test --test cli_task_open_smoke \
             --test cli_task_view_smoke \
             --test cli_report_wallet_smoke \
             --test cli_report_positions_smoke \
             --test cli_report_bankruptcy_smoke                     # 5/5 PASS

cargo test --lib runtime::batch_orchestrator                        # 6/6 PASS
cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo  # PASS
```

R3 verification was completed **before** the CI failure was discovered. The R-022 hotfix (post-R3) added one more verification: `cargo check --lib` resolving E0583 → green.

## 7. Hotfix (commit `cff03a28`) — Risk class 1

### Files

- `src/runtime/mod.rs` — remove 4-line orphan block (pin `555c6f52 → 39c5e227`)
- `genesis_payload.toml` — Trust Root pin update (Cz cycle 3-hotfix; same pattern)
- `handover/alignment/OBS_R022_R1_EXTERNAL_MARKET_SNAPSHOT_LEAK_2026-05-22.md` — NEW (R-022 hook justification)

### Why removal (not addition) was correct

Codex has its own session (verified via `pgrep -af codex`). Committing their WIP `external_market_snapshot.rs` would race with Codex's in-progress writes. Removal restores compile to green; Codex re-adds when their PR lands the `.rs` file as a single coherent commit.

### Ship

- Commit `68adf2c6` on `hotfix/r1-mod-rs-leak` (PR #85)
- Squash-merged to `main` as `cff03a28` with `GIT_HARDEN_ALLOW_MAIN=1`
- CI green (`TB-C0 Constitution Gates` run 26253490177, 2m42s, success)

## 8. Cost summary

| Metric | Value |
|--------|-------|
| LoC restored | ~1475 (848 lean_market + 583 batch_orchestrator + ~44 small files) |
| LoC modified in trunk | 3 (one `pub mod` + one `exclude` + Trust Root pins) |
| New Cargo deps | 0 |
| Trust Root churn | 2 pin updates (`Cargo.toml`, `src/runtime/mod.rs`) + 1 hotfix re-pin |
| Sub-agent dispatches | 3 Explore (Phase A) + 3 atom dispatches (R0, R1, Cz) + hotfix self-execution |
| DeepSeek tokens | 0 (pure restoration, no LLM in loop) |
| Wall-clock | ~3 hours including PR/audit/merge cycles + hotfix |

## 9. PR-train coordination defect (recoverable)

R0+R1 first attempts failed because Codex's PR #78 (boundary hygiene runner) and PR #79 (handover update) landed concurrently:

- PR #80 (R0 first attempt): closed due to push race
- PR #81 (R1 first attempt): closed due to push race

Fix: cherry-picked commits to fresh retry branches:
- PR #82 (R0 retry): merged as `2bf282ca`
- PR #83 (R1 retry): merged as `6148a0cd`

Future autonomous PR trains should `git fetch origin main && git rebase origin/main` immediately before push to detect upstream changes.

## §8 self-sign

Class 0 (research archive). Self-signed by Claude opus 4.7 under existing user delegation (2026-05-21).
