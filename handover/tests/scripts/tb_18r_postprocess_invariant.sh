#!/usr/bin/env bash
# TB-18R post-process: re-run R4 invariant compute on completed evidence
# runs using PPUT_RESULT-based extraction.
#
# Usage: bash tb_18r_postprocess_invariant.sh <evidence_dir>
#
# For each P*_<name>/ subdir: parse PPUT_RESULT line from evaluator.stdout,
# sum tool_dist for expected_completed_attempts, derive halt_class from
# solved/hit_max_tx flags, re-invoke tb_18r_compute_invariant.

set -uo pipefail
EVIDENCE_DIR="${1:?usage: $0 <evidence_dir>}"
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
INVARIANT_BIN="$PROJECT_ROOT/target/release/tb_18r_compute_invariant"

if [ ! -x "$INVARIANT_BIN" ]; then
  echo "ERROR: $INVARIANT_BIN not built" >&2
  exit 2
fi

PASS_COUNT=0
FAIL_COUNT=0
for RUN_DIR in "$EVIDENCE_DIR"/P[0-9]*_*/; do
  RUN_DIR="${RUN_DIR%/}"
  TAG="$(basename "$RUN_DIR")"
  STDOUT="$RUN_DIR/evaluator.stdout"
  if [ ! -f "$STDOUT" ]; then
    echo "[postprocess] $TAG: no evaluator.stdout; skip"
    continue
  fi

  PPUT_LINE="$(grep '^PPUT_RESULT:' "$STDOUT" | head -1 | sed 's/^PPUT_RESULT://')"
  if [ -z "$PPUT_LINE" ]; then
    echo "[postprocess] $TAG: no PPUT_RESULT; halt=ErrorHalt expected=0"
    EXPECTED=0
    HALT="ErrorHalt"
  else
    EXPECTED="$(echo "$PPUT_LINE" | python3 -c 'import sys,json; d=json.load(sys.stdin); print(sum(d.get("tool_dist",{}).values()))' 2>/dev/null || echo "0")"
    SOLVED="$(echo "$PPUT_LINE" | python3 -c 'import sys,json; print(json.load(sys.stdin).get("solved",False))' 2>/dev/null || echo "False")"
    HITMAX="$(echo "$PPUT_LINE" | python3 -c 'import sys,json; print(json.load(sys.stdin).get("hit_max_tx",False))' 2>/dev/null || echo "False")"
    if [ "$SOLVED" = "True" ]; then HALT="OmegaAccepted"
    elif [ "$HITMAX" = "True" ]; then HALT="MaxTxExhausted"
    else HALT="MaxTxExhausted"; fi
  fi

  "$INVARIANT_BIN" \
    --runtime-repo "$RUN_DIR/runtime_repo" \
    --cas "$RUN_DIR/cas" \
    --expected-completed "$EXPECTED" \
    --halt-class "$HALT" \
    > "$RUN_DIR/chain_invariant.json" 2> "$RUN_DIR/chain_invariant.stderr"

  VERDICT="$(grep -oE '"invariant_verdict"[[:space:]]*:[[:space:]]*"[^"]*"' "$RUN_DIR/chain_invariant.json" | head -1 | sed 's/.*"\([^"]*\)".*/\1/')"
  L4="$(grep -oE '"l4_work_attempt_count"[[:space:]]*:[[:space:]]*[0-9]+' "$RUN_DIR/chain_invariant.json" | grep -oE '[0-9]+')"
  L4E="$(grep -oE '"l4e_work_attempt_count"[[:space:]]*:[[:space:]]*[0-9]+' "$RUN_DIR/chain_invariant.json" | grep -oE '[0-9]+')"
  if [[ "$VERDICT" == "Ok" ]]; then
    echo "[postprocess] $TAG: PASS expected=$EXPECTED halt=$HALT l4=$L4 l4e=$L4E"
    PASS_COUNT=$((PASS_COUNT+1))
  else
    echo "[postprocess] $TAG: FAIL expected=$EXPECTED halt=$HALT l4=$L4 l4e=$L4E verdict=${VERDICT}"
    FAIL_COUNT=$((FAIL_COUNT+1))
  fi
done

echo ""
echo "[postprocess] DONE | PASS=$PASS_COUNT FAIL=$FAIL_COUNT"
