#!/usr/bin/env bash
# Test 3.1: NEW pub fn without TRACE_MATRIX backlink → BLOCK (exit 2).
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
. "$SCRIPT_DIR/_lib.sh"

enter_tmp_repo
cat > src/test_module.rs <<'EOF'
pub fn unbacked_function() -> i32 { 42 }
EOF
git add src/test_module.rs
rc=0
GIT_COMMIT_MSG="adding new pub fn" python3 scripts/check_trace_matrix.py --mode commit || rc=$?
assert_eq "2" "$rc" "exit code BLOCK"
grep -q "R-022-BLOCK" rules/enforcement.log
teardown
echo "PASS: r_022_blocks_missing_backlink"
