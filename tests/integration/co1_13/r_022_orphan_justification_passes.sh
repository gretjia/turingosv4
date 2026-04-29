#!/usr/bin/env bash
# Test 3.3: pub fn registered in § J with valid justification → PASS.
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
. "$SCRIPT_DIR/_lib.sh"

enter_tmp_repo
cat > src/orphan.rs <<'EOF'
pub fn orphan_fn() -> i32 { 1 }
EOF
git add src/orphan.rs
rc=0
GIT_COMMIT_MSG="orphan registered" python3 scripts/check_trace_matrix.py --mode commit || rc=$?
assert_eq "0" "$rc" "exit code PASS via § J"
grep -q "§ J orphan" rules/enforcement.log
teardown
echo "PASS: r_022_orphan_justification_passes"
