#!/usr/bin/env bash
# K-HARDEN-1: WorktreeCreate hook
#
# Fixes L5 — branch entanglement when multiple parallel haiku subagents
# share refs DB. Anthropic confirmed bugs: #51596 stale-branch-reuse,
# #43535 wrong-base, #34645 .git/config.lock race, #33045 silent isolation skip.
#
# Mechanism: replace Claude Code's default `git worktree add` with controlled
# script that:
# 1. Generates collision-resistant branch name: worktree-agent-${TIMESTAMP}-${RAND}
# 2. Fetches origin/main to be current
# 3. Uses --lock (no auto-prune race) + --no-track (no upstream config write race)
#    + explicit origin/main start point (no wrong-base heuristic)
# 4. Runs git clean -fdx in new worktree (paranoia against any state pollution)
# 5. Verifies branch matches before returning path

set -euo pipefail

INPUT="$(cat)"

# Hook contract: receive JSON on stdin; output path on stdout if success
BASE_PATH="$(echo "$INPUT" | jq -r '.base_path // empty')"
if [ -z "$BASE_PATH" ]; then
  BASE_PATH="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
fi
REPO_ROOT="$BASE_PATH"

TIMESTAMP="$(date +%s)"
RAND="$(openssl rand -hex 4 2>/dev/null || date +%N | head -c 8)"
BRANCH="worktree-agent-${TIMESTAMP}-${RAND}"
WORKTREE_DIR="${REPO_ROOT}/.claude/worktrees/${TIMESTAMP}-${RAND}"

mkdir -p "${REPO_ROOT}/.claude/worktrees"

# Refresh remote (best-effort; ignore network errors)
git -C "$REPO_ROOT" fetch origin main --quiet 2>&1 >&2 || true

# Create worktree:
#   --lock: prevents auto-prune by `git gc`
#   --no-track: avoids upstream branch config write → fewer .git/config.lock races
#   -b BRANCH: create new branch
#   "origin/main": explicit start point (fixes #43535 wrong-base bug)
git -C "$REPO_ROOT" worktree add \
  --lock \
  --no-track \
  -b "$BRANCH" \
  "$WORKTREE_DIR" \
  "origin/main" \
  >&2

# Paranoia: clean any state that might have leaked
git -C "$WORKTREE_DIR" clean -fdx 2>&1 >&2 || true

# Verify branch matches what we asked for
ACTUAL_BRANCH="$(git -C "$WORKTREE_DIR" rev-parse --abbrev-ref HEAD)"
if [ "$ACTUAL_BRANCH" != "$BRANCH" ]; then
  echo "FATAL: K-HARDEN-1 worktree branch mismatch: expected $BRANCH, got $ACTUAL_BRANCH" >&2
  exit 1
fi

# Output path for Claude Code to use
echo "$WORKTREE_DIR"
exit 0
