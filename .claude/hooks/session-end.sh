#!/usr/bin/env bash
# TuringOS v4 — Session End Hook (Stop)
# Checks for uncommitted changes in critical files. Advisory only (always exit 0).

PROJECT_ROOT="$(git rev-parse --show-toplevel 2>/dev/null)"
[ -z "$PROJECT_ROOT" ] && exit 0

CRITICAL_FILES="kernel.rs bus.rs wallet.rs prediction_market.rs CLAUDE.md constitution.md"
DIRTY=""
for f in $CRITICAL_FILES; do
    if git -C "$PROJECT_ROOT" status --porcelain 2>/dev/null | grep -q "$f"; then
        DIRTY="$DIRTY $f"
    fi
done

if [ -n "$DIRTY" ]; then
    echo "WARNING: Uncommitted changes in critical files:$DIRTY"
    echo "Consider committing or running /handover-update before ending."
fi

# Report rule engine activity
LOG="$PROJECT_ROOT/rules/enforcement.log"
if [ -f "$LOG" ]; then
    COUNT=$(wc -l < "$LOG" 2>/dev/null || echo 0)
    echo "Rule engine: $COUNT total triggers logged."
fi

exit 0
