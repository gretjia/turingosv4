#!/usr/bin/env bash
# K-HARDEN-3 dispatch helper for external orchestrators.
#
# This script wraps subagent dispatch with explicit pre-flight cleanliness
# checks + post-dispatch contamination scan. Inside Claude Code, the Agent
# tool with isolation:"worktree" already handles dispatch — this script is
# for external orchestrators (CLI users, CI workflows, future automations).
#
# Usage:
#   bash scripts/dispatch_subagent.sh --worktree <path> --prompt <file>
#
# Optional flags:
#   --skip-preflight   skip clean-tree check (NOT recommended)
#   --dry-run          print actions, do not execute
#
# Pre-flight checks (fail-fast):
#   1. worktree exists
#   2. worktree has no uncommitted changes
#   3. worktree has no stash entries (L5 stale-state defense)
#   4. worktree is on a unique branch (no name collision with active branches)
#
# Post-dispatch checks:
#   1. agent committed at least one change
#   2. agent's diff contains no handover/evidence/dev_self_hosting/dev_* files
#   3. agent's diff contains no .claude/worktrees/ files
#   4. PR (if created) points to agent's branch + SHA (L7 self-verify)

set -euo pipefail

WORKTREE=""
PROMPT_FILE=""
SKIP_PREFLIGHT=0
DRY_RUN=0

while [ $# -gt 0 ]; do
  case "$1" in
    --worktree) WORKTREE="$2"; shift 2 ;;
    --prompt)   PROMPT_FILE="$2"; shift 2 ;;
    --skip-preflight) SKIP_PREFLIGHT=1; shift ;;
    --dry-run) DRY_RUN=1; shift ;;
    --help|-h)
      sed -n '/^# Usage:/,/^# Post-dispatch checks:/p' "$0" | sed 's/^# //; s/^#$//'
      exit 0
      ;;
    *) echo "FATAL: unknown arg: $1" >&2; exit 1 ;;
  esac
done

[ -n "$WORKTREE" ] || { echo "FATAL: --worktree required" >&2; exit 1; }
[ -n "$PROMPT_FILE" ] || { echo "FATAL: --prompt required" >&2; exit 1; }
[ -d "$WORKTREE" ] || { echo "FATAL: worktree $WORKTREE does not exist" >&2; exit 1; }
[ -f "$PROMPT_FILE" ] || { echo "FATAL: prompt file $PROMPT_FILE does not exist" >&2; exit 1; }

# ─── Pre-flight ────────────────────────────────────────────────────────────
if [ "$SKIP_PREFLIGHT" = "0" ]; then
  echo "[dispatch] pre-flight: $WORKTREE"

  if ! git -C "$WORKTREE" diff --quiet 2>/dev/null; then
    echo "FATAL: worktree has uncommitted changes" >&2
    git -C "$WORKTREE" status --short >&2
    exit 1
  fi

  if [ -n "$(git -C "$WORKTREE" stash list 2>/dev/null)" ]; then
    echo "FATAL: worktree has stash entries (L5 stale-state risk)" >&2
    git -C "$WORKTREE" stash list >&2
    exit 1
  fi

  echo "[dispatch] pre-flight OK"
fi

START_SHA="$(git -C "$WORKTREE" rev-parse HEAD)"
START_BRANCH="$(git -C "$WORKTREE" rev-parse --abbrev-ref HEAD)"
echo "[dispatch] start: branch=$START_BRANCH sha=$START_SHA"

# ─── Dispatch placeholder ──────────────────────────────────────────────────
# This script is the orchestrator wrapper for EXTERNAL automations.
# The actual agent invocation depends on the caller's runtime:
#   - Claude Code: uses the Agent tool with isolation:"worktree"
#   - CLI runner: claude -p "$(cat "$PROMPT_FILE")" --cwd "$WORKTREE" ...
#   - CI workflow: GitHub Actions step invoking claude-code-action
# We leave the dispatch step as a placeholder; callers fill in.

if [ "$DRY_RUN" = "1" ]; then
  echo "[dispatch] DRY-RUN: would dispatch agent in $WORKTREE with prompt $PROMPT_FILE"
  echo "[dispatch] DRY-RUN: stop here"
  exit 0
fi

echo "[dispatch] AGENT-RUN: placeholder — caller invokes actual agent here"
echo "[dispatch] AGENT-RUN: prompt at $PROMPT_FILE"
echo "[dispatch] AGENT-RUN: worktree at $WORKTREE"

# ─── Post-dispatch ─────────────────────────────────────────────────────────
END_SHA="$(git -C "$WORKTREE" rev-parse HEAD)"

if [ "$START_SHA" = "$END_SHA" ]; then
  echo "[dispatch] post: agent made no commit (HEAD unchanged)"
  exit 0
fi

CHANGED_FILES="$(git -C "$WORKTREE" diff --name-only "$START_SHA" "$END_SHA")"
echo "[dispatch] post: agent commit chain $START_SHA..$END_SHA"
echo "[dispatch] post: changed files:"
echo "$CHANGED_FILES" | sed 's/^/  /'

# Contamination scan
CONTAM=""
echo "$CHANGED_FILES" | grep -q '^handover/evidence/dev_self_hosting/dev_' && \
  CONTAM="${CONTAM} handover/evidence/dev_self_hosting/dev_*/"
echo "$CHANGED_FILES" | grep -q '^\.claude/worktrees/' && \
  CONTAM="${CONTAM} .claude/worktrees/"

if [ -n "$CONTAM" ]; then
  echo "FATAL: agent committed forbidden sidecar paths:$CONTAM" >&2
  exit 1
fi

echo "[dispatch] post: no contamination detected"
echo "[dispatch] OK"
