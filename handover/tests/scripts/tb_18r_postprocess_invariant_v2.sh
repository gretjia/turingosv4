#!/usr/bin/env bash
# TB-18R post-process v2 — uses failed_branch_count + (1 if solved else 0)
# instead of tool_dist sum (which double-counts {step, step_reject} pairs).
set -uo pipefail
EVIDENCE_DIR="${1:?usage: $0 <evidence_dir>}"
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
INVARIANT_BIN="$PROJECT_ROOT/target/release/tb_18r_compute_invariant"

PASS=0; FAIL=0; NA=0
for RUN_DIR in "$EVIDENCE_DIR"/P[0-9]*_*/; do
  RUN_DIR="${RUN_DIR%/}"
  TAG="$(basename "$RUN_DIR")"
  STDOUT="$RUN_DIR/evaluator.stdout"
  if [ ! -f "$STDOUT" ]; then continue; fi
  PPUT_LINE="$(grep '^PPUT_RESULT:' "$STDOUT" | head -1 | sed 's/^PPUT_RESULT://')"
  if [ -z "$PPUT_LINE" ]; then
    echo "[v2] $TAG: PPUT-absent (SIGKILL); skip"
    NA=$((NA+1)); continue
  fi
  EXTRACT="$(echo "$PPUT_LINE" | python3 -c '
import sys, json
d = json.load(sys.stdin)
fbc = d.get("failed_branch_count", 0)
solved = bool(d.get("solved", False))
hit_max = bool(d.get("hit_max_tx", False))
expected = fbc + (1 if solved else 0)
if solved: halt = "OmegaAccepted"
elif hit_max: halt = "MaxTxExhausted"
else: halt = "MaxTxExhausted"
print(f"{expected} {halt} {fbc} {solved}")
' 2>/dev/null)"
  EXPECTED="$(echo "$EXTRACT" | awk '{print $1}')"
  HALT="$(echo "$EXTRACT" | awk '{print $2}')"
  FBC="$(echo "$EXTRACT" | awk '{print $3}')"
  SOLVED="$(echo "$EXTRACT" | awk '{print $4}')"

  "$INVARIANT_BIN" \
    --runtime-repo "$RUN_DIR/runtime_repo" \
    --cas "$RUN_DIR/cas" \
    --expected-completed "$EXPECTED" \
    --halt-class "$HALT" \
    > "$RUN_DIR/chain_invariant.json" 2> "$RUN_DIR/chain_invariant.stderr"

  VERDICT="$(grep -oE '"invariant_verdict"[[:space:]]*:[[:space:]]*"[^"]*"' "$RUN_DIR/chain_invariant.json" | head -1 | sed 's/.*"\([^"]*\)".*/\1/' || echo unknown)"
  L4="$(grep -oE '"l4_work_attempt_count"[[:space:]]*:[[:space:]]*[0-9]+' "$RUN_DIR/chain_invariant.json" | grep -oE '[0-9]+')"
  L4E="$(grep -oE '"l4e_work_attempt_count"[[:space:]]*:[[:space:]]*[0-9]+' "$RUN_DIR/chain_invariant.json" | grep -oE '[0-9]+')"
  if [ "$VERDICT" = "Ok" ]; then
    PASS=$((PASS+1))
    echo "[v2] $TAG: PASS expected=$EXPECTED halt=$HALT l4=$L4 l4e=$L4E (fbc=$FBC solved=$SOLVED)"
  else
    FAIL=$((FAIL+1))
    echo "[v2] $TAG: FAIL expected=$EXPECTED halt=$HALT l4=$L4 l4e=$L4E (fbc=$FBC solved=$SOLVED) verdict=$VERDICT"
  fi
done
echo "[v2] DONE | PASS=$PASS FAIL=$FAIL NA=$NA"
