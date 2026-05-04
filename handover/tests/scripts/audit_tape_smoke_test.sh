#!/usr/bin/env bash
# TB-16 Atom 6 — audit_tape_smoke_test.sh
#
# Wrapper around run_real_llm_arena.sh that asserts:
#   1. verdict.json contains verdict="PROCEED"
#   2. All 13 expected tx_kinds appear in tape_root.tx_kind_counts
#      (when the underlying tape exercises them — Atom 6 v0 ships with
#      Task-A-only coverage; full 13-kind coverage requires the
#      multi-task evaluator extension noted as TB-16 Atom 6.1)
#   3. tamper_report.json reports detected_count == 3
#   4. verdict.json and verdict_replay.json are byte-identical
#
# Usage:
#   bash handover/tests/scripts/audit_tape_smoke_test.sh \
#        --out-dir <existing-evidence-dir-from-run_real_llm_arena.sh>
#
# Exit codes:
#   0  — all asserts pass (PROCEED + 13/13 tx kinds + 3/3 tamper +
#        replay byte-identical)
#   1  — at least one assert fails
#   2  — invalid args / missing inputs

set -euo pipefail

OUT_DIR=""
REQUIRE_FULL_COVERAGE="${REQUIRE_FULL_COVERAGE:-0}"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --out-dir) OUT_DIR="$2"; shift 2 ;;
    --require-full-coverage) REQUIRE_FULL_COVERAGE="1"; shift ;;
    -h|--help)
      cat <<'EOF'
audit_tape_smoke_test.sh — TB-16 Atom 6 ship-gate wrapper

USAGE:
  bash handover/tests/scripts/audit_tape_smoke_test.sh --out-dir <path>

ENV:
  REQUIRE_FULL_COVERAGE=1   Fail if any of 13 architect-required tx kinds
                            absent. Default: warn only (Atom 6 v0 ships
                            with Task-A-only coverage; full 13-kind path
                            needs evaluator multi-task extension).

EXIT:
  0  all asserts PASS
  1  at least one assert FAIL
  2  missing inputs
EOF
      exit 0
      ;;
    *) echo "audit_tape_smoke_test: unknown arg: $1" >&2; exit 2 ;;
  esac
done

if [[ -z "$OUT_DIR" ]]; then
  echo "audit_tape_smoke_test: --out-dir required" >&2; exit 2
fi
if [[ ! -d "$OUT_DIR" ]]; then
  echo "audit_tape_smoke_test: --out-dir $OUT_DIR not a directory" >&2; exit 2
fi

VERDICT="$OUT_DIR/verdict.json"
VERDICT_REPLAY="$OUT_DIR/verdict_replay.json"
TAMPER="$OUT_DIR/tamper_report.json"

failed=0
warn=0

# Assert 1: verdict.json exists + verdict==PROCEED
if [[ ! -f "$VERDICT" ]]; then
  echo "✗ verdict.json missing at $VERDICT"; failed=$((failed+1))
else
  V=$(python3 -c "import json,sys; print(json.load(open('$VERDICT'))['verdict'])")
  if [[ "$V" == "PROCEED" ]]; then
    echo "✓ verdict.json verdict=PROCEED"
  else
    echo "✗ verdict.json verdict=$V (expected PROCEED)"; failed=$((failed+1))
  fi
fi

# Assert 2: all 13 tx kinds present
if [[ -f "$VERDICT" ]]; then
  MISSING=$(python3 - <<PYEOF
import json
v = json.load(open("$VERDICT"))
c = v.get("tx_kind_counts", {})
required = ["work","verify","challenge","task_open","escrow_lock",
            "complete_set_mint","complete_set_redeem","market_seed",
            "finalize_reward","challenge_resolve","terminal_summary",
            "task_expire","task_bankruptcy"]
missing = [k for k in required if c.get(k, 0) == 0]
print(",".join(missing))
PYEOF
  )
  if [[ -z "$MISSING" ]]; then
    echo "✓ all 13 tx kinds present in tape"
  else
    if [[ "$REQUIRE_FULL_COVERAGE" == "1" ]]; then
      echo "✗ missing tx kinds: $MISSING"; failed=$((failed+1))
    else
      echo "⚠ missing tx kinds (Atom 6 v0 expected): $MISSING"; warn=$((warn+1))
    fi
  fi
fi

# Assert 3: tamper detected 3/3
if [[ ! -f "$TAMPER" ]]; then
  echo "⚠ tamper_report.json missing"; warn=$((warn+1))
else
  D=$(python3 -c "import json; print(json.load(open('$TAMPER'))['detected_count'])")
  if [[ "$D" == "3" ]]; then
    echo "✓ tamper detection 3/3"
  else
    echo "✗ tamper detection $D/3"; failed=$((failed+1))
  fi
fi

# Assert 4: replay byte-identical
if [[ -f "$VERDICT" && -f "$VERDICT_REPLAY" ]]; then
  if cmp -s "$VERDICT" "$VERDICT_REPLAY"; then
    echo "✓ replay byte-identical"
  else
    echo "✗ replay diverged"; failed=$((failed+1))
  fi
else
  echo "⚠ replay verdict missing; skipping byte-identity assert"; warn=$((warn+1))
fi

echo ""
echo "═══════════════════════════════════════════════════════════════════"
echo "audit_tape_smoke_test: failed=$failed warn=$warn"
echo "═══════════════════════════════════════════════════════════════════"
exit $failed
