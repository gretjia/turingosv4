#!/usr/bin/env bash
# TB-18R post-process v4 — full extraction:
#   expected = step_reject + parse_fail + omega + preseed
#            (step_partial_ok EXCLUDED per R3 §1.3 amended CAS-only path)
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
    echo "[v4] $TAG: PPUT-absent (SIGKILL); skip"
    NA=$((NA+1)); continue
  fi
  PRESEED_WORK=$(grep -c '"agent_id":"tb6-smoke-agent"' "$RUN_DIR/runtime_repo/rejections.jsonl" 2>/dev/null || echo 0)
  EXTRACT="$(echo "$PPUT_LINE" | python3 -c '
import sys, json
d = json.load(sys.stdin)
td = d.get("tool_dist", {})
step_reject = td.get("step_reject", 0)
parse_fail = td.get("parse_fail", 0)
llm_err = td.get("llm_err", 0)
sorry_block = td.get("sorry_block", 0)
omega = td.get("omega_wtool", 0) + td.get("complete", 0) + td.get("complete_via_tape", 0)
runtime_expected = step_reject + parse_fail + llm_err + sorry_block + omega
solved = bool(d.get("solved", False))
hit_max = bool(d.get("hit_max_tx", False))
print(f"{runtime_expected} {solved} {hit_max} {step_reject} {parse_fail} {omega}")
' 2>/dev/null)"
  RUNTIME_EXPECTED="$(echo "$EXTRACT" | awk '{print $1}')"
  SOLVED="$(echo "$EXTRACT" | awk '{print $2}')"
  HITMAX="$(echo "$EXTRACT" | awk '{print $3}')"
  EXPECTED=$((RUNTIME_EXPECTED + PRESEED_WORK))
  if [ "$SOLVED" = "True" ]; then HALT="OmegaAccepted"
  elif [ "$HITMAX" = "True" ]; then HALT="MaxTxExhausted"
  else HALT="MaxTxExhausted"; fi
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
    echo "[v4] $TAG: PASS expected=$EXPECTED (runtime=$RUNTIME_EXPECTED + preseed=$PRESEED_WORK) halt=$HALT l4=$L4 l4e=$L4E"
  else
    FAIL=$((FAIL+1))
    echo "[v4] $TAG: FAIL expected=$EXPECTED (runtime=$RUNTIME_EXPECTED + preseed=$PRESEED_WORK) halt=$HALT l4=$L4 l4e=$L4E verdict=$VERDICT"
  fi
done
echo "[v4] DONE | PASS=$PASS FAIL=$FAIL NA=$NA"
