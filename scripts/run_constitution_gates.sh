#!/usr/bin/env bash
# Constitution gate runner — auto-discover + manifest cross-check (K-1.5).
# Hand-curated amendment history: handover/architect-insights/RUN_CONSTITUTION_GATES_HISTORY.md
# Authority: handover/directives/2026-05-06_TBC0_CONSTITUTION_LANDING_RESET_DIRECTIVE.md

set -euo pipefail
REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO"
mkdir -p target

# Discover gates from test files
DISCOVERED=$(ls tests/constitution_*.rs 2>/dev/null | xargs -n1 basename | sed 's/\.rs$//' | sort)

# Extract gates from manifest
MANIFEST=$(grep -oP '^name = "\K[^"]+' scripts/constitution_gates.manifest.toml | sort)

# Cross-check: gates discovered but missing from manifest
ONLY_DISC=$(comm -23 <(echo "$DISCOVERED") <(echo "$MANIFEST"))
if [ -n "$ONLY_DISC" ]; then
  echo "[k-1-5] FAIL: gates discovered but not in manifest:" >&2
  echo "$ONLY_DISC" >&2
  exit 1
fi

# Cross-check: gates in manifest but test file missing
ONLY_MANI=$(comm -13 <(echo "$DISCOVERED") <(echo "$MANIFEST"))
if [ -n "$ONLY_MANI" ]; then
  echo "[k-1-5] FAIL: gates in manifest but test file missing:" >&2
  echo "$ONLY_MANI" >&2
  exit 1
fi

# Run all discovered gates
FAIL=0
for g in $DISCOVERED; do
  if ! cargo test --test "$g" --no-fail-fast 2>&1 | tail -5 | grep -q "0 failed"; then
    echo "[k-1-5] FAIL: $g" >&2
    FAIL=$((FAIL+1))
  fi
done

echo "[k-1-5] total=$(echo "$DISCOVERED" | wc -w) failed=$FAIL"
[ $FAIL -eq 0 ]
