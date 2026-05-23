#!/usr/bin/env bash
# TB-SOFTWARE-3-0 Atom S5.1 (2026-05-23): standalone legacy-bypass audit.
#
# Reports — does NOT enforce. Run by maintainers / audit witnesses to
# baseline whether legacy bypass patterns have re-entered the codebase.
#
# Explicitly NOT a constitution gate:
#   - Does NOT run inside scripts/run_constitution_gates.sh.
#   - Does NOT block merges or CI.
#   - Exit code is 0 iff zero violations, else 1, so it can be wired into a
#     reporting workflow later without changing this script.
#
# Patterns audited (each is a "should not appear in production code" smell):
#   1. `t_hash_*` synthesized id fallback (FNV-1a / hash-of-stdout)
#   2. `simple_hash` FNV-style helper
#   3. `// removed` / `// TODO removed` ceremonial deletion stubs
#   4. `panic!()` outside tests/ and outside `#[cfg(test)]` modules in src/
#   5. `unwrap()` in src/web/* (web layer should not panic on user input)
#   6. `feature = "compat_*"` or `feature = "legacy_*"` (legacy-feature flags)
#
# Usage:
#   bash scripts/audit_legacy_bypass.sh
#   bash scripts/audit_legacy_bypass.sh --quiet         # only print summary
#
# Output: human-readable report on stdout, exit 0 if clean, 1 if any
# violations found. Designed to be safe to run with no args at any time.

set -uo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

QUIET=0
if [[ "${1:-}" == "--quiet" ]]; then
  QUIET=1
fi

violations=0

report() {
  local label="$1"
  local count="$2"
  if [[ "$count" -gt 0 ]]; then
    violations=$((violations + count))
    if [[ "$QUIET" -eq 0 ]]; then
      echo "  [$count] $label"
    fi
  fi
}

if [[ "$QUIET" -eq 0 ]]; then
  echo "audit_legacy_bypass.sh — TB-SOFTWARE-3-0 Atom S5.1 reporting baseline"
  echo "  repo: $REPO_ROOT"
  echo
  echo "Patterns scanned:"
fi

# 1. t_hash_* synthesized id fallback in src/web (S1 removed these)
count=$(grep -rE "t_hash_[a-z0-9_]+" src/web/ 2>/dev/null \
        | grep -v "// " \
        | wc -l | tr -d ' ')
report "t_hash_* synthesized id fallback in src/web/" "$count"

# 2. simple_hash helper (FNV-style) in src/web (S1 removed)
count=$(grep -rE "fn simple_hash" src/web/ 2>/dev/null | wc -l | tr -d ' ')
report "simple_hash helper in src/web/" "$count"

# 3. ceremonial removal stubs anywhere in src/
count=$(grep -rE "^[[:space:]]*//[[:space:]]*(removed|TODO removed)" src/ 2>/dev/null | wc -l | tr -d ' ')
report "ceremonial // removed stubs in src/" "$count"

# 4. panic! in src/ outside tests
count=$(grep -rn "panic!(" src/ 2>/dev/null \
        | grep -v "^src/.*#\[cfg(test)\]" \
        | grep -vE "^src/[^:]+:[0-9]+:[[:space:]]*//" \
        | wc -l | tr -d ' ')
report "panic!() in src/ (informational — may legitimately be in test scaffolding)" "$count"

# 5. unwrap() in src/web/ (web handlers should propagate, not panic)
count=$(grep -rn "\.unwrap()" src/web/ 2>/dev/null \
        | grep -vE "^src/web/[^:]+:[0-9]+:[[:space:]]*//" \
        | wc -l | tr -d ' ')
report ".unwrap() in src/web/ handler code (informational)" "$count"

# 6. legacy-feature flags in Cargo.toml
count=$(grep -E "^[\"]?(compat_|legacy_)" Cargo.toml 2>/dev/null | wc -l | tr -d ' ')
report "compat_*/legacy_* features in Cargo.toml" "$count"

if [[ "$QUIET" -eq 0 ]]; then
  echo
  if [[ "$violations" -eq 0 ]]; then
    echo "RESULT: NO-LEGACY-BYPASS-FOUND"
  else
    echo "RESULT: $violations potential signal(s) — review the items above."
    echo "        This is a reporting baseline; NOT a constitution gate."
  fi
fi

# Exit non-zero only when violations exist, so this script can be wired
# into a reporting workflow later. By design, this script is NOT invoked
# from scripts/run_constitution_gates.sh.
if [[ "$violations" -gt 0 ]]; then
  exit 1
fi
exit 0
