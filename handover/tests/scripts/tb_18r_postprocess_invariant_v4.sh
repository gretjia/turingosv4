#!/usr/bin/env bash
# TB-18R post-process v4 — full extraction:
#   expected = step_reject + parse_fail + llm_err + sorry_block + omega
#            + step_partial_ok
# Synthetic preseed gate Work is excluded because chain-side L4.E counting
# filters it with the same synthetic-gate predicate.
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
  EXTRACT="$(echo "$PPUT_LINE" | python3 "$PROJECT_ROOT/handover/tests/scripts/tb_18r_expected_from_pput.py" 2>/dev/null)"
  EXPECTED="$(echo "$EXTRACT" | python3 -c 'import json,sys; print(json.load(sys.stdin)["expected_completed_attempts"])')"
  HALT="$(echo "$EXTRACT" | python3 -c 'import json,sys; print(json.load(sys.stdin)["halt_class"])')"
  "$INVARIANT_BIN" \
    --runtime-repo "$RUN_DIR/runtime_repo" \
    --cas "$RUN_DIR/cas" \
    --expected-completed "$EXPECTED" \
    --halt-class "$HALT" \
    > "$RUN_DIR/chain_invariant.json" 2> "$RUN_DIR/chain_invariant.stderr"
  VERDICT="$(grep -oE '"invariant_verdict"[[:space:]]*:[[:space:]]*"[^"]*"' "$RUN_DIR/chain_invariant.json" | head -1 | sed 's/.*"\([^"]*\)".*/\1/' || echo unknown)"
  L4="$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1])).get("l4_work_attempt_count", ""))' "$RUN_DIR/chain_invariant.json")"
  L4E="$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1])).get("l4e_work_attempt_count", ""))' "$RUN_DIR/chain_invariant.json")"
  if [ "$VERDICT" = "Ok" ]; then
    PASS=$((PASS+1))
    echo "[v4] $TAG: PASS expected=$EXPECTED (pput-no-preseed) halt=$HALT l4=$L4 l4e=$L4E"
  else
    FAIL=$((FAIL+1))
    echo "[v4] $TAG: FAIL expected=$EXPECTED (pput-no-preseed) halt=$HALT l4=$L4 l4e=$L4E verdict=$VERDICT"
  fi
done
PROBLEMS_FILE="$EVIDENCE_DIR/r9_problems.txt"
if [ -f "$PROBLEMS_FILE" ]; then
  MANIFEST="$EVIDENCE_DIR/R9_RUN_MANIFEST.json"
  SUMMARY_GIT_HEAD="$(cd "$PROJECT_ROOT" && git rev-parse HEAD)"
  SUMMARY_RUN_TS="$(date -u +%Y-%m-%dT%H-%M-%SZ)"
  SUMMARY_MAX_TX="${MAX_TX:-12}"
  SUMMARY_TIMEOUT="${PER_PROBLEM_TIMEOUT_S:-1800}"
  if [ -f "$MANIFEST" ]; then
    SUMMARY_GIT_HEAD="$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1])).get("git_head", sys.argv[2]))' "$MANIFEST" "$SUMMARY_GIT_HEAD")"
    SUMMARY_RUN_TS="$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1])).get("run_timestamp_utc", sys.argv[2]))' "$MANIFEST" "$SUMMARY_RUN_TS")"
    SUMMARY_MAX_TX="$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1])).get("max_tx_per_problem", sys.argv[2]))' "$MANIFEST" "$SUMMARY_MAX_TX")"
    SUMMARY_TIMEOUT="$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1])).get("per_problem_timeout_s", sys.argv[2]))' "$MANIFEST" "$SUMMARY_TIMEOUT")"
  fi
  python3 "$PROJECT_ROOT/handover/tests/scripts/r9_batch_summary.py" \
    --out-dir "$EVIDENCE_DIR" \
    --problems-file "$PROBLEMS_FILE" \
    --max-tx "$SUMMARY_MAX_TX" \
    --per-problem-timeout-s "$SUMMARY_TIMEOUT" \
    --git-head "$SUMMARY_GIT_HEAD" \
    --run-timestamp-utc "$SUMMARY_RUN_TS"
  echo "[v4] summary refreshed: $EVIDENCE_DIR/R9_BATCH_SUMMARY.json"
fi
echo "[v4] DONE | PASS=$PASS FAIL=$FAIL NA=$NA"
