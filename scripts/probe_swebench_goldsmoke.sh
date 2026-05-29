#!/usr/bin/env bash
# Gold-patch smoke: confirm the verifier resolves the instance with the official
# gold patch. Establishes the verifier is valid for this instance.
set -uo pipefail
INSTANCE="${1:?usage: probe_swebench_goldsmoke.sh <instance_id> [run_tag]}"
TAG="${2:-g}"
cd /Users/zephryj/work/turingosv4-probe-gpqa
PY=/Users/zephryj/.venv-swebench/bin/python
WORK=handover/evidence/swebench_loop_20260528/goldsmoke_${TAG}
mkdir -p "$WORK"
echo "=== GOLD SMOKE: $INSTANCE (full env) ==="
( cd "$WORK" && "$PY" -m swebench.harness.run_evaluation \
    --dataset_name princeton-nlp/SWE-bench_Lite \
    --predictions_path gold \
    --instance_ids "$INSTANCE" \
    --run_id "${TAG}_${INSTANCE}" \
    --namespace none --max_workers 1 --cache_level instance ) 2>&1 | tail -25
echo "=== report.json ==="
find "$WORK" -name report.json -path "*${INSTANCE}*" -exec cat {} \; 2>/dev/null | python3 -m json.tool 2>/dev/null || echo "NO report.json found"
