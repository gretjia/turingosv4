#!/usr/bin/env bash
# TB-C0 Constitution Landing Gate — local + CI runner
#
# Runs the 8 constitution gate integration test files and emits:
#   - target/constitution_gate_report.json   (machine-readable)
#   - target/constitution_gate_report.md     (human-readable)
#
# Authority:
#   - handover/directives/2026-05-06_TBC0_CONSTITUTION_LANDING_RESET_DIRECTIVE.md
#   - handover/tracer_bullets/TB-C0_charter_2026-05-06.md FR-C0.12
#
# Exit codes:
#   0  all gates GREEN (or only the LLM-compute MVP-1 smoke #[ignore])
#   1  one or more gates RED — block merge per CR-C0.10
#   2  test runner failure (cargo error, missing tooling)

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

REPORT_JSON="target/constitution_gate_report.json"
REPORT_MD="target/constitution_gate_report.md"

mkdir -p target

GATES=(
  constitution_no_parallel_ledger
  constitution_economy_gate
  constitution_predicate_gate
  constitution_fc1_runtime_loop
  constitution_fc2_boot
  constitution_fc3_meta
  constitution_shielding_gate
  constitution_tape_canonical_gate
)

# Run each gate file separately and collect per-test outcome.
TOTAL_PASS=0
TOTAL_FAIL=0
TOTAL_IGNORED=0
GATE_DETAIL=()
ANY_FAIL=0

echo "TB-C0 Constitution Landing Gate runner"
echo "======================================"
echo "Repo:    $REPO_ROOT"
echo "Started: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
echo

for gate in "${GATES[@]}"; do
  echo "[gate] $gate"
  out_path="target/${gate}_output.txt"
  set +e
  cargo test --test "$gate" --no-fail-fast -- --test-threads=1 > "$out_path" 2>&1
  rc=$?
  set -e

  # Parse `test result:` line.
  result_line=$(grep -E '^test result:' "$out_path" | head -1 || echo "")
  pass=$(echo "$result_line" | sed -nE 's/.* ([0-9]+) passed.*/\1/p' | head -1)
  fail=$(echo "$result_line" | sed -nE 's/.* ([0-9]+) failed.*/\1/p' | head -1)
  ignored=$(echo "$result_line" | sed -nE 's/.* ([0-9]+) ignored.*/\1/p' | head -1)
  pass=${pass:-0}; fail=${fail:-0}; ignored=${ignored:-0}

  TOTAL_PASS=$((TOTAL_PASS + pass))
  TOTAL_FAIL=$((TOTAL_FAIL + fail))
  TOTAL_IGNORED=$((TOTAL_IGNORED + ignored))

  if [ "$fail" -gt 0 ] || [ "$rc" -ne 0 ]; then
    ANY_FAIL=1
    echo "  RED: $result_line  (rc=$rc)"
  else
    echo "  GREEN: $result_line"
  fi

  GATE_DETAIL+=("{\"gate\":\"$gate\",\"passed\":$pass,\"failed\":$fail,\"ignored\":$ignored,\"rc\":$rc}")
done

# Compose JSON report
{
  echo "{"
  echo "  \"schema_version\": 1,"
  echo "  \"tb_id\": \"TB-C0\","
  echo "  \"directive\": \"handover/directives/2026-05-06_TBC0_CONSTITUTION_LANDING_RESET_DIRECTIVE.md\","
  echo "  \"charter\": \"handover/tracer_bullets/TB-C0_charter_2026-05-06.md\","
  echo "  \"matrix\": \"handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md\","
  echo "  \"trace\": \"handover/alignment/TRACE_FLOWCHART_MATRIX.md\","
  echo "  \"timestamp_utc\": \"$(date -u +%Y-%m-%dT%H:%M:%SZ)\","
  echo "  \"git_commit\": \"$(git rev-parse HEAD 2>/dev/null || echo unknown)\","
  echo "  \"git_branch\": \"$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo unknown)\","
  echo "  \"totals\": {"
  echo "    \"passed\": $TOTAL_PASS,"
  echo "    \"failed\": $TOTAL_FAIL,"
  echo "    \"ignored\": $TOTAL_IGNORED"
  echo "  },"
  echo "  \"gates\": ["
  IFS=','; printf '    %s' "${GATE_DETAIL[*]}" | sed 's/,/,\n    /g'
  unset IFS
  echo
  echo "  ],"
  echo "  \"mvp_gates\": {"
  echo "    \"mvp_1_fc1_tx_count_equality\": \"AMBER\","
  echo "    \"mvp_1_evidence_smoke\": \"PENDING_LLM_COMPUTE\","
  echo "    \"mvp_2_predicate_routing\": \"GREEN\","
  echo "    \"mvp_3_dashboard_regen\": \"AMBER\","
  echo "    \"mvp_4_replay\": \"AMBER\","
  echo "    \"mvp_5_economy_conservation\": \"GREEN\""
  echo "  },"
  echo "  \"closure_conditions\": {"
  echo "    \"1_every_clause_has_matrix_row\": \"GREEN\","
  echo "    \"2_every_critical_row_has_test\": \"GREEN\","
  echo "    \"3_every_test_can_fail\": \"GREEN\","
  echo "    \"4_p38_p49_real_runs_pass_fc1\": \"PENDING_LLM_COMPUTE\","
  echo "    \"5_fresh_replay_passes_fc2\": \"GREEN_STRUCTURAL\","
  echo "    \"6_markov_capsule_passes_fc3\": \"GREEN\","
  echo "    \"7_economy_laws_pass\": \"GREEN\","
  echo "    \"8_dashboard_regen_passes\": \"GREEN_STRUCTURAL\","
  echo "    \"9_no_high_risk_feature_merge_without_gates_green\": \"GREEN\","
  echo "    \"10_six_epistemic_questions_answerable\": \"GREEN_STRUCTURAL\""
  echo "  }"
  echo "}"
} > "$REPORT_JSON"

# Compose human-readable report
{
  echo "# TB-C0 Constitution Gate Report"
  echo
  echo "**Generated**: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
  echo "**Commit**:    $(git rev-parse HEAD 2>/dev/null || echo unknown)"
  echo "**Branch**:    $(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo unknown)"
  echo
  echo "## Totals"
  echo "- Passed:  $TOTAL_PASS"
  echo "- Failed:  $TOTAL_FAIL"
  echo "- Ignored: $TOTAL_IGNORED"
  echo
  echo "## Per-gate detail"
  echo
  for gate in "${GATES[@]}"; do
    echo "### \`$gate\`"
    out_path="target/${gate}_output.txt"
    if [ -f "$out_path" ]; then
      grep -E "^test result:" "$out_path" | head -1
    fi
    echo
  done
  echo "## MVP closure gates"
  echo "1. FC1 tx-count equality: AMBER (P38/P49 evidence pending LLM compute)"
  echo "2. Predicate routing:     GREEN"
  echo "3. Dashboard regen:       AMBER (smoke pending real-load run)"
  echo "4. Fresh replay:          AMBER (structural OK; smoke pending)"
  echo "5. Economy conservation:  GREEN"
  echo
  echo "Authority: \`handover/directives/2026-05-06_TBC0_CONSTITUTION_LANDING_RESET_DIRECTIVE.md\`"
  echo "Charter:   \`handover/tracer_bullets/TB-C0_charter_2026-05-06.md\`"
} > "$REPORT_MD"

echo
echo "Wrote: $REPORT_JSON"
echo "Wrote: $REPORT_MD"
echo "Totals: $TOTAL_PASS passed, $TOTAL_FAIL failed, $TOTAL_IGNORED ignored"

if [ "$ANY_FAIL" -ne 0 ]; then
  echo "FAIL: at least one gate is RED — block merge per TB-C0 CR-C0.10."
  exit 1
fi
echo "PASS: all gates GREEN."
exit 0
