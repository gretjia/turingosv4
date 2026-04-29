#!/usr/bin/env bash
# Test 3.6: REMOVAL of existing /// TRACE_MATRIX line → BLOCK unless skip-token.
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
. "$SCRIPT_DIR/_lib.sh"

enter_tmp_repo
cat > src/with_trace.rs <<'EOF'
/// TRACE_MATRIX FC3-Test: keeper symbol.
pub fn keeper_fn() -> i32 { 4 }
EOF
git add src/with_trace.rs
git -c commit.gpgsign=false commit -q -m "seed traced symbol"

# Now remove the TRACE_MATRIX line
cat > src/with_trace.rs <<'EOF'
pub fn keeper_fn() -> i32 { 4 }
EOF
git add src/with_trace.rs
rc=0
GIT_COMMIT_MSG="cleanup" python3 scripts/check_trace_matrix.py --mode commit || rc=$?
assert_eq "2" "$rc" "exit code BLOCK on removal"
grep -q "trace_removal" rules/enforcement.log
teardown
echo "PASS: r_022_blocks_backlink_removal"
