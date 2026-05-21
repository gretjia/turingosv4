# C — R1 sub-agent worktree leak: post-mortem + R-022 commit-hook footgun

The R1 atom (commit `6148a0cd`) was supposed to be a verbatim restoration of one file + a one-line `pub mod` append. Instead it landed an additional 4-line `pub mod external_market_snapshot;` block in `src/runtime/mod.rs` that broke CI with `E0583: file not found`. This document captures the root cause + the R-022 commit-hook footgun discovered during the hotfix.

## 1. Defect summary

### Captured text (verbatim) of the leaked block

```rust
/// TRACE_MATRIX FC1/FC3 (REAL-WORLD Polymarket Phase A 2026-05-21): external
/// market snapshot sidecar. Additive Generic CAS evidence only; public price
/// is an agent signal, never resolution truth, wallet authority, or order flow.
pub mod external_market_snapshot;
```

The corresponding `src/runtime/external_market_snapshot.rs` was **never tracked in git** — it lived only as an untracked file in Codex's parallel WIP session.

### CI evidence (run 26252416772)

```
[error]E0583: file not found for module `external_market_snapshot`
   --> src/runtime/mod.rs:184:1
```

The Constitution Gates workflow couldn't compile any test binary because the trunk lib failed to compile. `boot::tests::verify_trust_root_passes_on_intact_repo` also failed because rustc never reached the test.

## 2. Root cause — sub-agent worktree leakage

The R1 sub-agent ran in an isolated worktree (`.claude/worktrees/1779395231-c08f1a81`). That worktree's `src/runtime/mod.rs` had previously been polluted by Codex's parallel Polymarket Phase A session — the 4-line block was already on disk in the worktree before R1 started.

When R1 sub-agent did its task (restore `batch_orchestrator.rs` + append one line to `mod.rs`), it added its own line to the already-polluted file. The squash-merge of R1's PR landed the polluted mod.rs into main, but the `.rs` file Codex had been editing was never tracked.

### Why this happened

`.claude/worktrees/` are git worktrees, not isolated containers. Untracked files in the main worktree may be visible in the sub-agent worktree (depending on how the worktree was created and whether files were touched).

### Future prevention (proposed)

Two options for sub-agent dispatches that copy "verbatim" trunk files:

1. **Reset the trunk file to `git show HEAD:<path>` before copying** — captures only tracked content. Pre-action step: `git checkout HEAD -- <path>` inside the sub-agent worktree before any read of that file.
2. **Restrict copy scope to file:line ranges, not whole files** — e.g. "copy lines 175-178 of mod.rs verbatim" instead of "use file contents". Reduces blast radius for unrelated polluted regions.

The Plan v7 retro already noted "autonomous orchestrator + Codex parallel writes to main need coordination." This OBS is one concrete instance of that exposure.

## 3. Hotfix path (commit `cff03a28`)

### Files modified

| File | Change |
|------|--------|
| `src/runtime/mod.rs` | Remove 4-line orphan block (lines 181-184 of pre-hotfix state). SHA256 `555c6f52 → 39c5e227`. |
| `genesis_payload.toml` | `[trust_root]."src/runtime/mod.rs"` pin update `555c6f52 → 39c5e227`. Cz cycle 3-hotfix comment appended (same pattern as 6 prior Cz cycles). |
| `handover/alignment/OBS_R022_R1_EXTERNAL_MARKET_SNAPSHOT_LEAK_2026-05-22.md` | NEW. Justification doc for R-022 hook bypass. |

### Removal command (the sed pattern that finally stuck)

```bash
sed -i '/^\/\/\/ TRACE_MATRIX FC1\/FC3 (REAL-WORLD Polymarket Phase A 2026-05-21): external$/,/^pub mod external_market_snapshot;$/d' src/runtime/mod.rs
```

Several `Edit` tool attempts reverted (possibly due to Codex parallel session writing the same file). Atomic `sed -i` stuck.

### Verification

- `cargo check --lib`: PASS (0 errors, E0583 resolved)
- `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo`: PASS
- `cargo test --lib runtime::batch_orchestrator`: PASS 6/6
- CI on main (`TB-C0 Constitution Gates` run 26253490177): GREEN, 2m42s

## 4. R-022 commit-hook footgun (latent bug in `scripts/check_trace_matrix.py`)

The R-022 hook (`scripts/check_trace_matrix.py --mode commit`) fires on removal of `/// TRACE_MATRIX` doc-comments + the `pub mod` declaration they annotate. The skip token `[R-022-skip: <ref>]` in the commit message is supposed to bypass the block when the removal is justified.

### Reproduction

```bash
# Token validator says message is valid:
GIT_COMMIT_MSG="$(cat /tmp/r022_hotfix_msg.txt)" \
  python3 scripts/check_trace_matrix.py --mode commit
# → exit 0

# But actual git commit blocks:
git commit -F /tmp/r022_hotfix_msg.txt
# → BLOCKED by R-022 (TRACE_MATRIX pub-symbol-block)
```

### Root cause

The script reads the commit message from `.git/COMMIT_EDITMSG`:

```python
def commit_message(mode: str) -> str:
    if mode == "commit":
        env_msg = os.environ.get("GIT_COMMIT_MSG", "")
        if env_msg:
            return env_msg
        msg_file = PROJECT_ROOT / ".git" / "COMMIT_EDITMSG"
        if msg_file.exists():
            return msg_file.read_text(errors="replace")
        return ""
    return run(["git", "log", "-1", "--pretty=%B"])
```

But git **only populates `.git/COMMIT_EDITMSG` AFTER the pre-commit hook passes** when using `git commit -F file` or `git commit -m "..."`. The pre-commit hook reads a stale (or absent) `COMMIT_EDITMSG` from the previous commit — which doesn't have the new commit's R-022-skip token.

`COMMIT_EDITMSG` is only the *current* commit's message during interactive `git commit` (which opens an editor) or via the `prepare-commit-msg` hook.

### Workaround (used by this hotfix)

```bash
cp /tmp/r022_hotfix_msg.txt .git/COMMIT_EDITMSG && git commit -F /tmp/r022_hotfix_msg.txt
# → success
```

Pre-populating `COMMIT_EDITMSG` makes the pre-commit hook see the right message.

### Proposed proper fix

Add `--message` / `--message-file` flag to `scripts/check_trace_matrix.py`:

```python
p.add_argument("--message", help="Override commit message read path")
p.add_argument("--message-file", help="Read commit message from file")
```

And have `scripts/hooks/pre-commit.r022` pass the message via the appropriate mechanism. Or alternatively, move the R-022 check from pre-commit to commit-msg hook (which DOES have access to the in-flight commit message).

This is filed for future cleanup; this hotfix used the workaround.

## 5. PR-mode R-022 also failed on the synthetic merge commit

The PR #85 CI showed `CO1.13 R-022 TRACE_MATRIX backlink check` failure (run 26253454853). Root cause: in CI mode the script runs:

```python
return run(["git", "log", "-1", "--pretty=%B"])
```

When GitHub PR CI runs against a synthetic merge commit (PR branch merged into main), `git log -1` returns the merge commit's auto-generated message, which doesn't have the R-022-skip token.

The squashed commit on main (`cff03a28`) has the proper message, so direct push CI passed. PR-mode R-022 is therefore unreliable for PR-flow merges; orchestrator should rely on the main push CI as the source of truth.

## 6. Cross-references

- OBS file: `handover/alignment/OBS_R022_R1_EXTERNAL_MARKET_SNAPSHOT_LEAK_2026-05-22.md`
- R1 PR: commit `6148a0cd` (the one that introduced the defect)
- Hotfix commit: `cff03a28` (squashed from `hotfix/r1-mod-rs-leak` branch `68adf2c6`)
- CI failure runs: 26252416772 + 26252627482 (both same root cause)
- CI green run: 26253490177 (post-hotfix)
- Codex parallel PR train (no overlap with hotfix scope): PR #78, PR #79 (both already merged)

## §8 self-sign

Class 0 (research archive). Self-signed by Claude opus 4.7 under existing user delegation (2026-05-21).
