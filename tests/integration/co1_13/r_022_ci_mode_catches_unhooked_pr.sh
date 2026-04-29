#!/usr/bin/env bash
# Test 3.9: CI mode catches a PR-style commit that bypassed install_hooks.sh.
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
. "$SCRIPT_DIR/_lib.sh"

enter_tmp_repo
# Establish main baseline + branch off (all inside the tmp repo)
git -c commit.gpgsign=false commit -q --allow-empty -m "main baseline"
git checkout -q -b feature

cat > src/leaked.rs <<'EOF'
pub fn leaked_pub() -> i32 { 8 }
EOF
git add src/leaked.rs
git -c commit.gpgsign=false commit -q -m "feature: add leaked pub"

# Run CI mode (HEAD..base-ref); should BLOCK
rc=0
python3 scripts/check_trace_matrix.py --mode ci --base-ref main || rc=$?
assert_eq "2" "$rc" "CI mode catches unhooked PR"
grep -q "R-022-BLOCK" rules/enforcement.log
teardown
echo "PASS: r_022_ci_mode_catches_unhooked_pr"
