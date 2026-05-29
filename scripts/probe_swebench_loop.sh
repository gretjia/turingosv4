#!/usr/bin/env bash
# TuringOS verify-retry LOOP arm on one SWE-bench instance.
# Routes the SiliconFlow client through the local DeepSeek proxy (:8123).
# Real hidden-test verification via the official swebench harness; on failure
# the real failing-test names feed the next retry prompt.
set -euo pipefail

INSTANCE="${1:?usage: probe_swebench_loop.sh <instance_short> (e.g. flask5063)}"
SAMPLE="${2:?usage: probe_swebench_loop.sh <instance_short> <sample.json>}"
ATTEMPTS="${3:-3}"

cd /Users/zephryj/work/turingosv4-probe-gpqa
RUN=handover/evidence/swebench_loop_20260528

set -a; . ./.env; set +a
export DEEPSEEK_API_KEY_WORKER="${DEEPSEEK_API_KEY}"
export TURINGOS_SILICONFLOW_ENDPOINT=http://localhost:8123/v1/chat/completions

curl -sS -X POST http://localhost:8123/stats/reset >/dev/null

echo "=== LOOP arm: $INSTANCE  (max ${ATTEMPTS} attempts) ==="
./target/release/turingos tdma run \
  --workspace "$RUN/ws_${INSTANCE}" \
  --judge swebench --role meta \
  --swebench-sample "$SAMPLE" \
  --swebench-python /Users/zephryj/.venv-swebench/bin/python \
  --swebench-workdir "$RUN/loop_work_${INSTANCE}" \
  --max-attempts-per-stage "$ATTEMPTS" \
  --evidence-dir "$RUN/loop_evidence_${INSTANCE}"

echo "=== proxy token stats ($INSTANCE) ==="
curl -sS http://localhost:8123/stats | python3 -m json.tool
