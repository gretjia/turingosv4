#!/usr/bin/env bash
# Test 3.8: legacy untraced pub symbols at HEAD do NOT cause spurious R-022 block
# on subsequent unrelated edits (forward-only enforcement, I-FORWARD invariant).
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
. "$SCRIPT_DIR/_lib.sh"

enter_tmp_repo
cat > src/legacy.rs <<'EOF'
pub fn legacy_untraced() -> i32 { 99 }
EOF
git add src/legacy.rs
git -c commit.gpgsign=false commit -q -m "seed legacy"

# Now an unrelated change that does NOT touch any pub items in src/
cat > README.md <<'EOF'
# README change
EOF
git add README.md
rc=0
GIT_COMMIT_MSG="docs only" python3 scripts/check_trace_matrix.py --mode commit || rc=$?
assert_eq "0" "$rc" "no spurious block on unrelated edit"
teardown
echo "PASS: r_022_no_legacy_block"
