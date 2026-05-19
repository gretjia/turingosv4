#!/bin/bash
# Usage: ./turn.sh <turn_idx> <answer>
SID="pi4_mrs_chen_1779149881_91621"
EVID=/Users/zephryj/work/turingosv4/handover/evidence/phase6_3_x_universality_1779111375/pi4/mrs_chen
TURN=$1
ANSWER=$2
T0=$(date +%s)
BODY_JSON=$(python3 -c "import json,sys; print(json.dumps({'session_id':'$SID','user_answer':sys.argv[1],'lang':'zh'}))" "$ANSWER")
RESP=$(curl -sS -w "\n__STATUS__%{http_code}" -X POST http://127.0.0.1:8080/api/spec/turn -H 'Content-Type: application/json' -d "$BODY_JSON")
T1=$(date +%s)
ELAPSED=$((T1-T0))
STATUS=$(echo "$RESP" | tail -1 | sed 's/__STATUS__//')
BODY=$(echo "$RESP" | sed '$d')
echo "=== TURN $TURN  STATUS=$STATUS  ELAPSED=${ELAPSED}s ==="
echo "$BODY" | python3 -m json.tool 2>&1
ANSWER_JSON=$(python3 -c "import json,sys; print(json.dumps(sys.argv[1]))" "$ANSWER")
printf '{"turn":%s,"user_answer":%s,"status":%s,"elapsed_s":%s,"response":%s}\n' "$TURN" "$ANSWER_JSON" "$STATUS" "$ELAPSED" "$BODY" >> "$EVID/session_log.jsonl"
