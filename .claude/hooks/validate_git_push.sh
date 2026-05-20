#!/usr/bin/env bash
# K-HARDEN-6: PreToolUse hook to block direct push to main.
#
# Fixes L9 — haiku subagent accidentally cd'd out of its isolation worktree,
# committed on main worktree HEAD, then pushed to origin/main directly,
# bypassing PR review.
#
# This hook intercepts `git push` and denies any form that would write to
# refs/heads/main on origin. Allows push to feature branches.
#
# Forbidden forms:
#   git push origin main
#   git push origin main:main
#   git push origin HEAD:main
#   git push origin HEAD       (when local branch is main)
#   git push                   (when local branch is main and main is upstream-tracked)
#
# Allowed forms:
#   git push -u origin <feature-branch>
#   git push origin <feature-branch>
#   git push                   (when local branch is non-main)
#
# To bypass legitimately (merging a PR locally + pushing the merge commit):
#   GIT_HARDEN_ALLOW_MAIN=1 git push origin main
#
# Output contract: same as validate_git_add.sh.

set -euo pipefail

INPUT="$(cat)"
COMMAND="$(echo "$INPUT" | jq -r '.tool_input.command // ""' 2>/dev/null || echo "")"

# Only process Bash invocations that contain `git push`
if ! echo "$COMMAND" | grep -qE '\bgit push\b'; then
  exit 0
fi

# Strip heredocs + quoted strings to avoid false-positive on commit messages
STRIPPED="$(echo "$COMMAND" | python3 -c '
import sys, re
text = sys.stdin.read()
text = re.sub(r"<<[A-Z\047\042]*([A-Z]+)[\047\042]*.*?\n\1", "", text, flags=re.DOTALL)
text = re.sub(r"\047[^\047]*\047", "", text)
text = re.sub(r"\042[^\042]*\042", "", text)
print(text)
' 2>/dev/null || echo "$COMMAND")"

# Detect bypass env var
if echo "$COMMAND" | grep -qE 'GIT_HARDEN_ALLOW_MAIN=1'; then
  exit 0
fi

# Command boundary: same as validate_git_add
BOUNDARY='(^|\n|&&|\|\||;|\|)[[:space:]]*'

# Pattern 1: `git push origin main` (explicit)
# Pattern 2: `git push origin main:main` (refspec with main as target)
# Pattern 3: `git push origin HEAD:main` (refspec with HEAD->main)
# Pattern 4: `git push --all` (pushes all branches including main if local)
REASON=""
if echo "$STRIPPED" | grep -qE "${BOUNDARY}git push[[:space:]]+(-[a-zA-Z]+[[:space:]]+)*origin[[:space:]]+main(\\s|$|;|&|\\|)"; then
  REASON="git push origin main is forbidden — use PR workflow"
elif echo "$STRIPPED" | grep -qE "${BOUNDARY}git push[[:space:]]+(-[a-zA-Z]+[[:space:]]+)*origin[[:space:]]+(main|HEAD):main\\b"; then
  REASON="git push origin <ref>:main is forbidden — use PR workflow"
elif echo "$STRIPPED" | grep -qE "${BOUNDARY}git push[[:space:]]+(-[a-zA-Z]+[[:space:]]+)*--all\\b"; then
  REASON="git push --all is forbidden in this harness (would include main if currently checked out) — push specific feature branch with git push -u origin <branch>"
else
  # Pattern 5: bare `git push` (or `git push origin`) when local branch is main.
  # We need git context to check the current branch. If we're in a context
  # where the bash command can be run (i.e., a worktree), check.
  if echo "$STRIPPED" | grep -qE "${BOUNDARY}git push[[:space:]]*(origin[[:space:]]*)?(\\s|$|;|&|\\|)"; then
    CURRENT_BRANCH="$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo unknown)"
    if [ "$CURRENT_BRANCH" = "main" ]; then
      REASON="git push from main branch is forbidden — switch to feature branch first (git checkout -b feature-name)"
    fi
  fi
fi

if [ -z "$REASON" ]; then
  exit 0
fi

cat <<EOF
{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "permissionDecision": "deny",
    "permissionDecisionReason": "${REASON}. Direct push to main bypasses PR review and contamination scans (K-HARDEN-5 GitHub Action only triggers on PRs). Create a feature branch instead: git checkout -b harden/my-fix.",
    "additionalContext": "K-HARDEN-6 enforcement (closes L9 lesson). To bypass legitimately (e.g., merging a vetted local PR then pushing the merge commit): GIT_HARDEN_ALLOW_MAIN=1 git push origin main. This is logged in shell history and auditable."
  }
}
EOF
exit 0
