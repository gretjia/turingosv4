#!/usr/bin/env bash
# CO1.13.2 — install tracked git hooks.
#
# Idempotent: removes existing .git/hooks/pre-commit if present (warns on
# non-symlink so user can rescue local content); creates symlink
# .git/hooks/pre-commit -> ../../scripts/hooks/pre-commit.r022.
#
# Run as part of dev-setup. CI does NOT run this; CI uses
# `scripts/check_trace_matrix.py --mode ci` directly via
# .github/workflows/co1_13_r022_ci.yml.
set -euo pipefail

PROJECT_ROOT="$(git rev-parse --show-toplevel)"
HOOK_DIR="$PROJECT_ROOT/.git/hooks"
HOOK_TARGET="../../scripts/hooks/pre-commit.r022"
HOOK_LINK="$HOOK_DIR/pre-commit"

mkdir -p "$HOOK_DIR"

if [ -e "$HOOK_LINK" ] || [ -L "$HOOK_LINK" ]; then
    if [ -L "$HOOK_LINK" ]; then
        rm "$HOOK_LINK"
    else
        echo "warn: $HOOK_LINK exists and is NOT a symlink; backing up to ${HOOK_LINK}.bak"
        mv "$HOOK_LINK" "${HOOK_LINK}.bak"
    fi
fi

ln -s "$HOOK_TARGET" "$HOOK_LINK"
chmod +x "$PROJECT_ROOT/scripts/hooks/pre-commit.r022"
echo "installed: $HOOK_LINK -> $HOOK_TARGET"
