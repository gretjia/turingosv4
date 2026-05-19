#!/bin/bash
# Usage:
#   ./turn.sh 0            # bootstrap greeting (user_answer: null)
#   ./turn.sh <idx> <ans>  # send user_answer for turn <idx>
SID=$(cat "$(dirname "$0")/session_id.txt")
EVID=/Users/zephryj/work/turingosv4/handover/evidence/phase6_3_x_universality_1779111375/pi4/s9_gibberish_negctrl
TURN=$1
ANSWER=$2
T0=$(date +%s)
if [ "$TURN" = "0" ]; then
  BODY_JSON=$(python3 -c "import json; print(json.dumps({'session_id':'$SID','user_answer':None,'lang':'zh'}))")
else
  BODY_JSON=$(python3 -c "import json,sys; print(json.dumps({'session_id':'$SID','user_answer':sys.argv[1],'lang':'zh'}))" "$ANSWER")
fi
RESP=$(curl -sS -w "\n__STATUS__%{http_code}" -X POST http://127.0.0.1:8080/api/spec/turn -H 'Content-Type: application/json' -d "$BODY_JSON")
T1=$(date +%s)
ELAPSED=$((T1-T0))
STATUS=$(echo "$RESP" | tail -1 | sed 's/__STATUS__//')
BODY=$(echo "$RESP" | sed '$d')
echo "=== TURN $TURN  STATUS=$STATUS  ELAPSED=${ELAPSED}s ==="
echo "$BODY" | python3 -m json.tool 2>&1
if [ "$TURN" = "0" ]; then
  ANSWER_JSON="null"
  QTAG='"greeting"'
else
  ANSWER_JSON=$(python3 -c "import json,sys; print(json.dumps(sys.argv[1]))" "$ANSWER")
  QTAG='null'
fi
printf '{"turn":%s,"q_text":%s,"user_answer":%s,"status":%s,"elapsed_s":%s,"response":%s}\n' \
  "$TURN" "$QTAG" "$ANSWER_JSON" "$STATUS" "$ELAPSED" "$BODY" >> "$EVID/session_log.jsonl"
