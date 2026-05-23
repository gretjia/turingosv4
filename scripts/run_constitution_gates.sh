#!/usr/bin/env bash
# Constitution gate runner — auto-discover + manifest cross-check (K-1.5).
# Hand-curated amendment history: handover/architect-insights/RUN_CONSTITUTION_GATES_HISTORY.md
# Authority: handover/directives/2026-05-06_TBC0_CONSTITUTION_LANDING_RESET_DIRECTIVE.md

set -euo pipefail
REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO"
mkdir -p target
OUT_TXT="target/constitution_gates_output.txt"
REPORT_JSON="target/constitution_gate_report.json"
REPORT_MD="target/constitution_gate_report.md"

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

# Run all discovered gates in one Cargo invocation. This preserves the full
# gate set while avoiding 100+ cold-runner cargo startups in CI.
CARGO_ARGS=()
for g in $DISCOVERED; do
  CARGO_ARGS+=(--test "$g")
done

# Serialize cargo's test threads so process-global env-var mutations in tests
# (e.g. TURINGOS_TEST_ROUTER_FAIL_AT_STEP in constitution_router_buy_with_coin)
# cannot leak into peer tests within the same binary. CI mirrors this via
# .github/workflows/constitution_gates.yml so local `bash
# scripts/run_constitution_gates.sh` and `make constitution` share the same
# isolation guarantee.
FAIL=0
if ! RUST_TEST_THREADS=1 cargo test "${CARGO_ARGS[@]}" --no-fail-fast 2>&1 | tee "$OUT_TXT"; then
  FAIL=$(grep -c "test result: FAILED" "$OUT_TXT" || true)
  if [ "$FAIL" -eq 0 ]; then
    FAIL=1
  fi
fi

TOTAL=$(echo "$DISCOVERED" | wc -w)
SUMMARY="[k-1-5] total=$TOTAL failed=$FAIL"
echo "$SUMMARY"

cat > "$REPORT_JSON" <<EOF
{"total":$TOTAL,"failed":$FAIL,"summary":"$SUMMARY"}
EOF

{
  echo "# Constitution Gate Report"
  echo
  echo "- total: $TOTAL"
  echo "- failed: $FAIL"
  echo "- output: $OUT_TXT"
} > "$REPORT_MD"

[ "$FAIL" -eq 0 ]
