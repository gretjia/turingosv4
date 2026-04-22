#!/usr/bin/env bash
# MiniF2F baseline monitor — detect failures fast
# Exit codes: 0 = healthy, 1 = warning, 2 = critical (process dead)

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LOG_DIR="$SCRIPT_DIR/logs"
RESULTS_FILE=$(ls -t "$LOG_DIR"/pput_oneshot_*.jsonl 2>/dev/null | head -1)
LOG_FILE=$(ls -t "$SCRIPT_DIR"/baseline_run*.log 2>/dev/null | head -1)

echo "=== MiniF2F Baseline Monitor ==="
echo "Time: $(date '+%Y-%m-%d %H:%M:%S')"
echo ""

# 1. Process alive?
BATCH_PID=$(pgrep -f "run_batch.sh" | head -1 || true)
EVAL_PID=$(pgrep -f "target/release/evaluator" | head -1 || true)

if [ -z "$BATCH_PID" ]; then
    echo "CRITICAL: run_batch.sh is NOT running!"
    # Check if it finished normally
    if [ -n "$LOG_FILE" ] && grep -q "PPUT BATCH SUMMARY" "$LOG_FILE" 2>/dev/null; then
        echo "  -> Batch completed normally (summary found in log)"
        EXIT_STATUS=0
    else
        echo "  -> Batch crashed or was killed"
        echo "  -> Last log lines:"
        tail -5 "$LOG_FILE" 2>/dev/null | sed 's/^/     /'
        EXIT_STATUS=2
    fi
else
    echo "OK: run_batch.sh running (PID $BATCH_PID)"
    EXIT_STATUS=0
fi

if [ -n "$EVAL_PID" ]; then
    EVAL_CMD=$(ps -p $EVAL_PID -o args= 2>/dev/null | sed 's|.*/||')
    echo "OK: evaluator running (PID $EVAL_PID) — $EVAL_CMD"
fi

# 2. Proxy alive?
PROXY_STATUS=$(curl -s -o /dev/null -w "%{http_code}" http://localhost:8080/health 2>/dev/null || echo "000")
if [ "$PROXY_STATUS" = "200" ]; then
    PROXY_STATS=$(curl -s http://localhost:8080/stats 2>/dev/null)
    REQ_COUNT=$(echo "$PROXY_STATS" | python3 -c "import sys,json; print(json.load(sys.stdin)['requests'])" 2>/dev/null || echo "?")
    ERR_COUNT=$(echo "$PROXY_STATS" | python3 -c "import sys,json; print(json.load(sys.stdin)['errors'])" 2>/dev/null || echo "?")
    echo "OK: LLM proxy alive (requests=$REQ_COUNT, errors=$ERR_COUNT)"
else
    echo "CRITICAL: LLM proxy DOWN (HTTP $PROXY_STATUS)"
    EXIT_STATUS=2
fi

# 3. Results progress
echo ""
if [ -n "$RESULTS_FILE" ] && [ -f "$RESULTS_FILE" ]; then
    TOTAL=$(wc -l < "$RESULTS_FILE")
    SOLVED=$(grep -c '"has_golden_path":true' "$RESULTS_FILE" 2>/dev/null || echo 0)
    FAILED=$((TOTAL - SOLVED))
    PPUT_SUM=$(python3 -c "
import json
total = 0
with open('$RESULTS_FILE') as f:
    for line in f:
        try: total += json.loads(line).get('pput', 0)
        except: pass
print(f'{total:.2f}')
" 2>/dev/null || echo "?")

    echo "Progress: $TOTAL / 244 completed ($(python3 -c "print(f'{$TOTAL/244*100:.0f}')") %)"
    echo "Solved:   $SOLVED  |  PPUT=0: $FAILED"
    echo "Sum PPUT:  ${PPUT_SUM}%/s"

    # Last result
    LAST=$(tail -1 "$RESULTS_FILE")
    LAST_PROB=$(echo "$LAST" | python3 -c "import sys,json; d=json.load(sys.stdin); print(d['problem'].split('/')[-1])" 2>/dev/null || echo "?")
    LAST_GP=$(echo "$LAST" | python3 -c "import sys,json; print('SOLVED' if json.load(sys.stdin)['has_golden_path'] else 'PPUT=0')" 2>/dev/null || echo "?")
    echo "Last:     $LAST_PROB -> $LAST_GP"

    # Stall detection: if results file not modified in 30 min, warn
    if [ "$(find "$RESULTS_FILE" -mmin +30 2>/dev/null)" ]; then
        echo "WARNING: No new results in 30+ minutes — possible stall"
        [ "$EXIT_STATUS" -lt 1 ] && EXIT_STATUS=1
    fi
else
    echo "WARNING: No results file found"
    [ "$EXIT_STATUS" -lt 1 ] && EXIT_STATUS=1
fi

# 4. Recent log errors
echo ""
if [ -n "$LOG_FILE" ]; then
    ERR_LINES=$(grep -c -iE "(error|critical|panic|crash|command not found)" "$LOG_FILE" 2>/dev/null || true)
    ERR_LINES="${ERR_LINES:-0}"
    if [ "$ERR_LINES" -gt 0 ] 2>/dev/null; then
        echo "WARNING: $ERR_LINES error lines in log:"
        grep -iE "(error|critical|panic|crash|command not found)" "$LOG_FILE" 2>/dev/null | tail -3 | sed 's/^/  /'
        [ "$EXIT_STATUS" -lt 1 ] && EXIT_STATUS=1
    else
        echo "Log: clean (no errors)"
    fi
fi

echo ""
echo "Status: $([ $EXIT_STATUS -eq 0 ] && echo 'HEALTHY' || ([ $EXIT_STATUS -eq 1 ] && echo 'WARNING' || echo 'CRITICAL'))"
exit $EXIT_STATUS
