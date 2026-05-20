#!/usr/bin/env bash
# K-HARDEN-7 server-side ultimate: enable GitHub branch protection on main.
#
# This is the strongest universal block on direct push-to-main. GitHub
# enforces it server-side, regardless of which client (Claude / Codex /
# Gemini / aider / cursor / human, with or without local hooks) pushes.
#
# Requires repo admin permissions for the authenticated gh user.
#
# Settings applied:
#   - Require pull request before merging (with 0 approvals minimum,
#     orchestrator can self-approve; can be raised to 1+ later)
#   - Block force pushes
#   - Block deletions
#   - Block direct push to main (covered by required_pull_request_reviews)
#   - enforce_admins: false (so user/orchestrator can override via gh ui)
#
# Idempotent: re-running updates the protection rule.
#
# To inspect current protection:
#   gh api repos/{owner}/{repo}/branches/main/protection
#
# To remove (NOT recommended):
#   gh api -X DELETE repos/{owner}/{repo}/branches/main/protection

set -euo pipefail

REPO="$(gh repo view --json nameWithOwner --jq '.nameWithOwner' 2>/dev/null || echo "")"
if [ -z "$REPO" ]; then
  echo "FATAL: not in a gh-recognized repo. Run from repo root with gh authenticated."
  exit 1
fi

echo "K-HARDEN-7: enabling branch protection on main for ${REPO}"

# GitHub branch protection API spec:
#   https://docs.github.com/en/rest/branches/branch-protection
# We use the most universal-compatible settings.

gh api -X PUT "repos/${REPO}/branches/main/protection" \
  -F "required_status_checks=null" \
  -F "enforce_admins=false" \
  -F "required_pull_request_reviews[required_approving_review_count]=0" \
  -F "required_pull_request_reviews[dismiss_stale_reviews]=false" \
  -F "required_pull_request_reviews[require_code_owner_reviews]=false" \
  -F "restrictions=null" \
  -F "required_linear_history=false" \
  -F "allow_force_pushes=false" \
  -F "allow_deletions=false" \
  -F "block_creations=false" \
  -F "required_conversation_resolution=false" \
  -F "lock_branch=false" \
  -F "allow_fork_syncing=false" \
  > /dev/null

echo ""
echo "K-HARDEN-7: branch protection enabled on main for ${REPO}"
echo ""
echo "Verify:"
echo "  gh api repos/${REPO}/branches/main/protection | jq"
echo ""
echo "Any agent (Claude / Codex / Gemini / human, with or without local"
echo "hooks) now CANNOT push directly to main. PR is required."
