#!/usr/bin/env bash
# test_render.sh — 3 round-trip tests for the UI IR renderer
# Run from experiments/tisr_ui_spike/ directory.
# Exit 0 if all pass; non-zero if any fail.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

PASS=0
FAIL=0

run_test() {
    local num="$1"
    local desc="$2"
    local result="$3"  # "pass" or "fail"
    if [ "$result" = "pass" ]; then
        echo "TEST $num PASS: $desc"
        PASS=$((PASS + 1))
    else
        echo "TEST $num FAIL: $desc"
        FAIL=$((FAIL + 1))
    fi
}

echo "=== UI IR Spike test_render.sh ==="
echo ""

# ---------------------------------------------------------------------------
# Test 1: dashboard_sample.json renders as text, exits 0, stdout non-empty
# ---------------------------------------------------------------------------
T1_OUTPUT=$(python3 render.py --fixture fixtures/dashboard_sample.json 2>/dev/null)
T1_EXIT=$?
if [ "$T1_EXIT" -eq 0 ] && [ -n "$T1_OUTPUT" ]; then
    run_test 1 "dashboard_sample.json --format text: exit 0 + stdout non-empty" "pass"
else
    run_test 1 "dashboard_sample.json --format text: exit 0 + stdout non-empty (exit=$T1_EXIT, len=${#T1_OUTPUT})" "fail"
fi

# ---------------------------------------------------------------------------
# Test 2: agent_view_sample.json renders as text, exits 0, stdout contains agent_id
# ---------------------------------------------------------------------------
T2_OUTPUT=$(python3 render.py --fixture fixtures/agent_view_sample.json 2>/dev/null)
T2_EXIT=$?
if [ "$T2_EXIT" -eq 0 ] && echo "$T2_OUTPUT" | grep -q "agent_id"; then
    run_test 2 "agent_view_sample.json --format text: exit 0 + stdout contains 'agent_id'" "pass"
else
    run_test 2 "agent_view_sample.json --format text: exit 0 + stdout contains 'agent_id' (exit=$T2_EXIT)" "fail"
fi

# ---------------------------------------------------------------------------
# Test 3: task_view_sample.json --format json exits 0 + stdout is valid JSON
# ---------------------------------------------------------------------------
T3_OUTPUT=$(python3 render.py --fixture fixtures/task_view_sample.json --format json 2>/dev/null)
T3_EXIT=$?
if [ "$T3_EXIT" -eq 0 ] && echo "$T3_OUTPUT" | python3 -c "import sys,json; json.load(sys.stdin)" 2>/dev/null; then
    run_test 3 "task_view_sample.json --format json: exit 0 + stdout is valid JSON" "pass"
else
    run_test 3 "task_view_sample.json --format json: exit 0 + stdout is valid JSON (exit=$T3_EXIT)" "fail"
fi

# ---------------------------------------------------------------------------
# Summary
# ---------------------------------------------------------------------------
echo ""
echo "=== Results: $PASS passed, $FAIL failed ==="

if [ "$FAIL" -gt 0 ]; then
    exit 1
fi

exit 0
