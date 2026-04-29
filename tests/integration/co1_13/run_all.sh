#!/usr/bin/env bash
# CO1.13 integration test suite — runs all 9 R-022 shell tests.
# Invoked by tests/r_022_integration_orchestrator.rs for cargo discoverability.
set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

failed=0
for t in r_022_*.sh; do
    if bash "$t" >/tmp/r022_$$.log 2>&1; then
        echo "ok  $t"
    else
        echo "FAIL $t"
        cat /tmp/r022_$$.log
        failed=$((failed + 1))
    fi
    rm -f /tmp/r022_$$.log
done

if [ "$failed" -gt 0 ]; then
    echo "FAILED: $failed test(s)"
    exit 1
fi
echo "ALL PASS"
exit 0
