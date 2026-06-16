#!/bin/bash
# Repository Identity Preflight Gate (Gate A)
# Ensures:
#   - We are in the correct repository (/Users/zephryj/work/turingosv4)
#   - Prints branch and HEAD for audit
#   - If dirty, snapshots state and exits 0 (dirty is allowed but tracked)
#   - Only exits 1 on wrong repo or git error
#
# POSIX/bash compatible, no GNU-only flags (macOS compatibility)
# Usage: scripts/hooks/repo_identity_preflight.sh
# chmod +x before use

set -e

# Verify we are in the correct repository
EXPECTED_TOP_LEVEL="/Users/zephryj/work/turingosv4"
ACTUAL_TOP_LEVEL=$(git rev-parse --show-toplevel 2>/dev/null || echo "")

if [ -z "$ACTUAL_TOP_LEVEL" ]; then
  echo "ERROR: git rev-parse --show-toplevel failed. Not in a git repository." >&2
  exit 1
fi

if [ "$ACTUAL_TOP_LEVEL" != "$EXPECTED_TOP_LEVEL" ]; then
  echo "ERROR: Wrong repository." >&2
  echo "  Expected: $EXPECTED_TOP_LEVEL" >&2
  echo "  Got:      $ACTUAL_TOP_LEVEL" >&2
  exit 1
fi

# Print branch and HEAD for audit trail
BRANCH=$(git rev-parse --abbrev-ref HEAD)
HEAD=$(git rev-parse HEAD)
echo "✓ Repository identity verified: $EXPECTED_TOP_LEVEL"
echo "  Branch: $BRANCH"
echo "  HEAD:   $HEAD"

# Check if working tree is dirty
DIRTY_COUNT=$(git status --porcelain 2>/dev/null | wc -l)

if [ "$DIRTY_COUNT" -gt 0 ]; then
  # Snapshot the dirty state
  SNAPSHOT_FILE="handover/DIRTY_SNAPSHOT.md"
  TIMESTAMP="$HEAD"

  # Ensure handover directory exists
  mkdir -p "$(dirname "$SNAPSHOT_FILE")"

  # Generate snapshot (overwrite if exists — idempotent)
  {
    echo "# Dirty Tree Snapshot"
    echo ""
    echo "**Timestamp (HEAD):** \`$TIMESTAMP\`"
    echo ""
    echo "**Dirty File Count:** $DIRTY_COUNT"
    echo ""
    echo "## Modified Files"
    echo ""
    git status --porcelain | while IFS= read -r line; do
      echo "    $line"
    done
  } > "$SNAPSHOT_FILE"

  echo "⚠ DIRTY — $DIRTY_COUNT file(s) modified. Snapshot written to $SNAPSHOT_FILE"
  exit 0
fi

echo "✓ Working tree clean."
exit 0
