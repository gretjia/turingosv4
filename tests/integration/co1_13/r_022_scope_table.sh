#!/usr/bin/env bash
# Test 3.7: scope table policy (§ 1.3): pub use exempt; pub type/static block.
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
. "$SCRIPT_DIR/_lib.sh"

# Subtest A: pub use is EXEMPT
enter_tmp_repo
cat > src/lib.rs <<'EOF'
mod inner { pub fn anchor() {} }
pub use inner::anchor;
EOF
git add src/lib.rs
rc=0
GIT_COMMIT_MSG="add re-export" python3 scripts/check_trace_matrix.py --mode commit || rc=$?
assert_eq "0" "$rc" "pub use exempt"
teardown

# Subtest B: pub type IS load-bearing → BLOCK if missing
enter_tmp_repo
cat > src/types.rs <<'EOF'
pub type AgentId = String;
EOF
git add src/types.rs
rc=0
GIT_COMMIT_MSG="add type alias" python3 scripts/check_trace_matrix.py --mode commit || rc=$?
assert_eq "2" "$rc" "pub type blocks if missing"
teardown

# Subtest C: pub static IS load-bearing → BLOCK if missing
enter_tmp_repo
cat > src/state.rs <<'EOF'
pub static MAX_AGENTS: usize = 8;
EOF
git add src/state.rs
rc=0
GIT_COMMIT_MSG="add static" python3 scripts/check_trace_matrix.py --mode commit || rc=$?
assert_eq "2" "$rc" "pub static blocks if missing"
teardown

echo "PASS: r_022_scope_table"
