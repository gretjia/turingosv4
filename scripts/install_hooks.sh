#!/usr/bin/env bash
# CO1.13.2 + K-HARDEN-7 — install tracked git hooks.
#
# Idempotent: removes existing .git/hooks/pre-commit + commit-msg + pre-push
# if present (warns on non-symlink so user can rescue local content); creates
# symlinks
#   .git/hooks/pre-commit -> ../../scripts/hooks/pre-commit.r022
#   .git/hooks/commit-msg -> ../../scripts/hooks/commit-msg.r022
#   .git/hooks/pre-push   -> ../../scripts/hooks/pre-push.harden
#
# Run as part of dev-setup. CI does NOT run this; CI uses
# `scripts/check_trace_matrix.py --mode ci` directly via
# .github/workflows/co1_13_r022_ci.yml.
#
# R022_HOOK_FIX_2026-05-22: R-022 trace-matrix backlink check moved from
# pre-commit to commit-msg because pre-commit cannot read the in-flight
# commit message for `git commit -m` / `-F` (git writes COMMIT_EDITMSG only
# after pre-commit succeeds). pre-commit.r022 now does only the K-HARDEN-2
# sidecar contamination block.
#
# K-HARDEN-7 (2026-05-20) addition: pre-push hook blocks direct push to main
# universally (any agent runtime — Claude / Codex / Gemini / human). Closes
# L9 push-to-main bypass. PR-only workflow now mandatory.

set -euo pipefail

PROJECT_ROOT="$(git rev-parse --show-toplevel)"
HOOK_DIR="$PROJECT_ROOT/.git/hooks"

install_hook() {
    local hook_name="$1"      # e.g. pre-commit
    local target_relpath="$2" # e.g. ../../scripts/hooks/pre-commit.r022
    local target_abspath="$PROJECT_ROOT/$(echo "$target_relpath" | sed 's|^\.\./\.\./||')"
    local link="$HOOK_DIR/$hook_name"

    if [ -e "$link" ] || [ -L "$link" ]; then
        if [ -L "$link" ]; then
            rm "$link"
        else
            echo "warn: $link exists and is NOT a symlink; backing up to ${link}.bak"
            mv "$link" "${link}.bak"
        fi
    fi

    ln -s "$target_relpath" "$link"
    chmod +x "$target_abspath"
    echo "installed: $link -> $target_relpath"
}

mkdir -p "$HOOK_DIR"

# CO1.13.2 — K-HARDEN-2 sidecar contamination block
install_hook "pre-commit" "../../scripts/hooks/pre-commit.r022"

# CO1.13.2 — R-022 trace-matrix backlink check (commit-msg phase so the
# in-flight commit message is reachable via $1 for `git commit -m` / `-F`)
install_hook "commit-msg" "../../scripts/hooks/commit-msg.r022"

# K-HARDEN-7 — universal pre-push (any-agent block on push-to-main)
install_hook "pre-push" "../../scripts/hooks/pre-push.harden"

echo ""
echo "K-HARDEN-7 note: enable GitHub branch protection on main as the"
echo "ultimate server-side enforcement (works across all agents incl. agents"
echo "that bypass local hooks with --no-verify):"
echo "  bash scripts/setup_branch_protection.sh"
