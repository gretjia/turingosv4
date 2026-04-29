#!/usr/bin/env bash
# Test 3.2: NEW pub fn WITH /// TRACE_MATRIX backlink → PASS (exit 0).
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
. "$SCRIPT_DIR/_lib.sh"

enter_tmp_repo
cat > src/traced_module.rs <<'EOF'
/// TRACE_MATRIX FC3-Test: synthetic test symbol.
pub fn traced_function() -> i32 { 7 }
EOF
git add src/traced_module.rs
rc=0
GIT_COMMIT_MSG="adding traced pub fn" python3 scripts/check_trace_matrix.py --mode commit || rc=$?
assert_eq "0" "$rc" "exit code PASS"
grep -q "R-022-PASS" rules/enforcement.log
teardown
echo "PASS: r_022_passes_traced_pub"
