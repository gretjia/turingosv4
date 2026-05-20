#!/usr/bin/env bash
# K-HARDEN-2: PreToolUse hook to block wildcard `git add` invocations.
#
# Fixes L8 — Haiku-class subagents default to `git add .` or `git add -A`
# despite explicit prompts saying "git add <file1> <file2>". This sweeps
# auto-generated sidecar evidence (handover/evidence/dev_*/) into commits.
#
# Mechanism: PreToolUse hooks read tool_input.command via JSON stdin.
# This hook detects forbidden forms and emits a deny verdict with exit 0.
# Exit 0 + permissionDecision=deny is the Anthropic-official mechanical
# block (works even with --dangerously-skip-permissions).
#
# Forbidden forms:
#   git add .
#   git add ./...
#   git add -A
#   git add --all
#   git add -u
#   git add --update
#
# Allowed forms:
#   git add <specific-path>
#   git add <multiple-paths>
#   git add -p <path>  (interactive — won't fire in non-tty context anyway)
#   git add --intent-to-add <path>
#   git rm <path>      (not "add", different command)
#
# Output contract for Claude Code PreToolUse:
#   - exit 2 + stderr message: blocks the call, stderr shown to model
#   - exit 0 + permissionDecision=deny JSON: also blocks, with structured reason
#   - exit 0 + no special output: allows the call

set -euo pipefail

INPUT="$(cat)"
COMMAND="$(echo "$INPUT" | jq -r '.tool_input.command // ""' 2>/dev/null || echo "")"

# Only process Bash tool invocations that contain `git add`
if ! echo "$COMMAND" | grep -qE '\bgit add\b'; then
  exit 0
fi

# To avoid false-positive on `git add . / -A` strings inside heredocs and
# string literals (e.g., commit messages explaining this very rule), we only
# match when `git add ...` appears at a real command boundary:
#   - start of line (^)
#   - after && || ; |
#   - after `\n` (newline in multi-line bash)
# We strip everything inside heredocs (<<EOF ... EOF) and single/double quotes
# before pattern-matching. This is a heuristic, not a real shell parser.

# Strip heredocs: anything between <<X and X on its own line (greedy multi-line)
STRIPPED="$(echo "$COMMAND" | python3 -c '
import sys, re
text = sys.stdin.read()
# Strip heredocs of common forms
text = re.sub(r"<<[A-Z\047\042]*([A-Z]+)[\047\042]*.*?\n\1", "", text, flags=re.DOTALL)
# Strip single-quoted strings
text = re.sub(r"\047[^\047]*\047", "", text)
# Strip double-quoted strings (best-effort, no nested handling)
text = re.sub(r"\042[^\042]*\042", "", text)
print(text)
' 2>/dev/null || echo "$COMMAND")"

# Match at command boundary only:
#   ^ or after && || ; | \n, possibly with whitespace
BOUNDARY='(^|\n|&&|\|\||;|\|)[[:space:]]*'

if echo "$STRIPPED" | grep -qE "${BOUNDARY}git add[[:space:]]+\\.(\\s|$|&|;|\\|)"; then
  REASON="git add . is forbidden — use explicit file paths"
elif echo "$STRIPPED" | grep -qE "${BOUNDARY}git add[[:space:]]+(-A|--all)\\b"; then
  REASON="git add -A / --all is forbidden — use explicit file paths"
elif echo "$STRIPPED" | grep -qE "${BOUNDARY}git add[[:space:]]+(-u|--update)\\b"; then
  REASON="git add -u / --update is forbidden in this harness — explicitly name files. (Even -u stages all tracked changes broadly.)"
else
  # Allowed form (specific path), or pattern not at command boundary
  exit 0
fi

# Block: emit deny verdict
cat <<EOF
{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "permissionDecision": "deny",
    "permissionDecisionReason": "${REASON}. Use git status first to see what changed, then git add specific paths. The handover/evidence/dev_*/ directory contains auto-generated sidecar files that must NOT be committed via wildcard staging.",
    "additionalContext": "K-HARDEN-2 enforcement. See handover/architect-insights/K_HARDEN_PROPOSAL_2026-05-20.md for rationale. To bypass legitimately (initial commit of new module dir, etc.), stage with explicit recursive paths: git add src/new_module/ tests/new_module/. Never use bare . or -A."
  }
}
EOF
exit 0
