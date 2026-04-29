#!/usr/bin/env bash
# Test 3.4: commit-msg [R-022-skip: cases/C-test ...] with valid ref → PASS+SKIP log.
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
. "$SCRIPT_DIR/_lib.sh"

enter_tmp_repo
cat > src/skipped.rs <<'EOF'
pub fn skipped_fn() -> i32 { 2 }
EOF
git add src/skipped.rs
rc=0
GIT_COMMIT_MSG='refactor: cleanup [R-022-skip: cases/C-test refactor cleanup]' \
    python3 scripts/check_trace_matrix.py --mode commit || rc=$?
assert_eq "0" "$rc" "exit code PASS via skip-token"
grep -q "R-022-SKIP" rules/enforcement.log
teardown
echo "PASS: r_022_skip_token_with_justification"
