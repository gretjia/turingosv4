#!/usr/bin/env bash
# TuringOS UI IR validator — smoke / integration test (Phase 6.2 W2.2)
#
# Tests:
#   1. each existing fixture validates (exit 0)
#   2. a deliberately-malformed fixture fails (exit 1 with diagnostic)
#   3. invalid JSON fails (exit 2)
#   4. missing-fixture invocation fails (exit 2)
#   5. stdin mode works

set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")"

PASS=0
FAIL=0

run() {
    local name="$1"; shift
    if "$@" > /tmp/validate_test_stdout 2>/tmp/validate_test_stderr; then
        echo "PASS: $name"
        PASS=$((PASS + 1))
    else
        echo "FAIL: $name (exit $?)"
        cat /tmp/validate_test_stderr >&2
        FAIL=$((FAIL + 1))
    fi
}

run_fail() {
    local name="$1"
    local expected_exit="$2"
    shift 2
    local rc=0
    "$@" > /tmp/validate_test_stdout 2>/tmp/validate_test_stderr && rc=0 || rc=$?
    if [ "$rc" = "$expected_exit" ]; then
        echo "PASS: $name (expected exit $expected_exit)"
        PASS=$((PASS + 1))
    else
        echo "FAIL: $name (got exit $rc, expected $expected_exit)"
        cat /tmp/validate_test_stderr >&2
        FAIL=$((FAIL + 1))
    fi
}

echo "=== UI IR Spike test_validate.sh ==="
echo ""

# ---------------------------------------------------------------------------
# Test group 1: each existing fixture validates (exit 0)
# ---------------------------------------------------------------------------
for f in fixtures/*.json; do
    run "validate $f" python3 validate.py --fixture "$f" --quiet
done

# ---------------------------------------------------------------------------
# Test 2: malformed fixture — missing required 'blocks' field — exits 1
# ---------------------------------------------------------------------------
cat > /tmp/bad_ir_no_blocks.json <<'EOF'
{"id": "bad", "title": "missing-blocks"}
EOF
run_fail "malformed_no_blocks exit 1" 1 \
    python3 validate.py --fixture /tmp/bad_ir_no_blocks.json --quiet

# ---------------------------------------------------------------------------
# Test 3: malformed fixture — unknown block kind — exits 1
# ---------------------------------------------------------------------------
cat > /tmp/bad_ir_unknown_kind.json <<'EOF'
{
  "id": "bad2",
  "title": "unknown-block-kind",
  "blocks": [{"id": "b1", "kind": "bogus_kind"}]
}
EOF
run_fail "malformed_unknown_kind exit 1" 1 \
    python3 validate.py --fixture /tmp/bad_ir_unknown_kind.json --quiet

# ---------------------------------------------------------------------------
# Test 4: invalid JSON — exits 2
# ---------------------------------------------------------------------------
printf 'not valid json\n' > /tmp/bad_json.json
run_fail "invalid_json exit 2" 2 \
    python3 validate.py --fixture /tmp/bad_json.json --quiet

# ---------------------------------------------------------------------------
# Test 5: missing fixture file — exits 2
# ---------------------------------------------------------------------------
run_fail "missing_file exit 2" 2 \
    python3 validate.py --fixture /tmp/this_file_does_not_exist_12345.json --quiet

# ---------------------------------------------------------------------------
# Test 6: stdin mode — dashboard_sample via stdin, exits 0
# ---------------------------------------------------------------------------
if python3 validate.py --stdin --quiet < fixtures/dashboard_sample.json \
        > /tmp/validate_test_stdout 2>/tmp/validate_test_stderr; then
    echo "PASS: validate via stdin (dashboard_sample.json)"
    PASS=$((PASS + 1))
else
    echo "FAIL: validate via stdin (dashboard_sample.json)"
    cat /tmp/validate_test_stderr >&2
    FAIL=$((FAIL + 1))
fi

# ---------------------------------------------------------------------------
# Test 7: --help exits 0
# ---------------------------------------------------------------------------
run "help_exits_0" python3 validate.py --help

# ---------------------------------------------------------------------------
# Summary
# ---------------------------------------------------------------------------
echo ""
echo "=== Results: PASS=$PASS  FAIL=$FAIL ==="

[ "$FAIL" = "0" ]
