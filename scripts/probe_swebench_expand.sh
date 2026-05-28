#!/usr/bin/env bash
# Run loop arm + bare arm for one instance, sequentially. Per-instance token/
# wall-clock stays clean because runs do not overlap.
set -uo pipefail
SHORT="${1:?usage: probe_swebench_expand.sh <short e.g. flask4045> <sample.json>}"
SAMPLE="${2:?need sample path}"
cd /Users/zephryj/work/turingosv4-probe-gpqa
RUN=handover/evidence/swebench_loop_20260528
PROXY=http://localhost:8123
VENVPY=/Users/zephryj/.venv-swebench/bin/python

mkdir -p "$RUN/ws_${SHORT}"
cp "$RUN/ws_flask5063/turingos.toml" "$RUN/ws_${SHORT}/turingos.toml"

echo "### [$SHORT] LOOP arm ###"
bash scripts/probe_swebench_loop.sh "$SHORT" "$SAMPLE" 3 > "$RUN/logs/loop_${SHORT}.log" 2>&1
echo "### [$SHORT] loop done; stage outcome:"
grep -E "completed [0-9]+/[0-9]+ stages" "$RUN/logs/loop_${SHORT}.log" || true

echo "### [$SHORT] BARE arm ###"
curl -sS -X POST "$PROXY/stats/reset" >/dev/null
mkdir -p "$RUN/bare_work_${SHORT}"
python3 scripts/probe_bare_v4_swebench.py "$SAMPLE" "$PROXY" "$VENVPY" "$RUN/bare_work_${SHORT}" > "$RUN/logs/bare_${SHORT}.json" 2> "$RUN/logs/bare_${SHORT}.err"
echo "### [$SHORT] bare done"
