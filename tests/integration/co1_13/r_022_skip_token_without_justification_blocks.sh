#!/usr/bin/env bash
# Test 3.5: commit-msg [R-022-skip: <no justification ref>] → BLOCK.
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
. "$SCRIPT_DIR/_lib.sh"

enter_tmp_repo
cat > src/empty_skip.rs <<'EOF'
pub fn empty_skip_fn() -> i32 { 3 }
EOF
git add src/empty_skip.rs
rc=0
GIT_COMMIT_MSG='refactor [R-022-skip: just because]' \
    python3 scripts/check_trace_matrix.py --mode commit || rc=$?
assert_eq "2" "$rc" "exit code BLOCK on unjustified skip"
grep -q "R-022-BLOCK" rules/enforcement.log
teardown
echo "PASS: r_022_skip_token_without_justification_blocks"
