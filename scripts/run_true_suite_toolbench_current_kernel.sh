#!/usr/bin/env bash
# True-suite ToolBench API tool-use current-kernel evidence runner.
#
# Uses the public ToolBench benchmark parquet split plus a real external LLM
# through the local OpenAI-compatible proxy. The model's API selection is parsed
# into a structured claim, hashed into CAS, submitted as a signed WorkTx through
# current ChainTape, and replayed through public `turingos verify chaintape`.

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RUN_ID="${1:-toolbench_api_tool_use_$(date -u +%Y%m%dT%H%M%SZ)}"
RUN_ID="${RUN_ID#handover/evidence/true_suite/}"
RUN_ROOT="$PROJECT_ROOT/handover/evidence/true_suite/$RUN_ID"
RUN_DIR="$RUN_ROOT/toolbench"
LLM_PROXY_URL="${LLM_PROXY_URL:-http://localhost:8080}"
ACTIVE_MODEL="${ACTIVE_MODEL:-deepseek-chat}"
TOOLBENCH_DATASET_URL="${TOOLBENCH_DATASET_URL:-https://huggingface.co/datasets/tuandunghcmut/toolbench-v1/resolve/main/benchmark/g1_instruction-00000-of-00001.parquet}"
TOOLBENCH_SOURCE_SPLIT="${TOOLBENCH_SOURCE_SPLIT:-benchmark/g1_instruction}"
TOOLBENCH_SAMPLE_INDEX="${TOOLBENCH_SAMPLE_INDEX:-0}"

if [[ -e "$RUN_DIR" ]]; then
    echo "ERROR: evidence directory already exists: $RUN_DIR" >&2
    exit 2
fi

if [[ -f "$PROJECT_ROOT/.env" ]]; then
    set -a
    # shellcheck disable=SC1091
    source "$PROJECT_ROOT/.env"
    set +a
elif [[ -f "/home/zephryj/projects/turingosv4/.env" ]]; then
    set -a
    # shellcheck disable=SC1091
    source "/home/zephryj/projects/turingosv4/.env"
    set +a
fi

if [[ -n "$(cd "$PROJECT_ROOT" && git status --porcelain | grep -vE '^\?\? handover/evidence/' | head -1)" ]]; then
    echo "ERROR: working tree has non-evidence changes; run /runner-preflight before evidence runners" >&2
    exit 3
fi

if ! curl -sS --max-time 5 "$LLM_PROXY_URL/health" | grep -q '"status": "ok"'; then
    echo "ERROR: LLM proxy $LLM_PROXY_URL/health not OK" >&2
    echo "Start one with: python3 src/drivers/llm_proxy.py --port 8080" >&2
    exit 4
fi

echo "[build] cargo build --release --bin turingos --bin verify_chaintape --bin toolbench_api_tool_use_current_kernel --bin full_system_participation_current_kernel"
(cd "$PROJECT_ROOT" && cargo build --release --bin turingos --bin verify_chaintape --bin toolbench_api_tool_use_current_kernel --bin full_system_participation_current_kernel)

TURINGOS="$PROJECT_ROOT/target/release/turingos"
HELPER="$PROJECT_ROOT/target/release/toolbench_api_tool_use_current_kernel"
PARTICIPATION="$PROJECT_ROOT/target/release/full_system_participation_current_kernel"
SAMPLE_JSON="$RUN_DIR/tool_capsules/toolbench_sample.json"

echo "[init] turingos init --project $RUN_DIR"
"$TURINGOS" init --project "$RUN_DIR" --template proof --provider deepseek
mkdir -p "$RUN_DIR/tool_capsules"

if [[ -n "${TOOLBENCH_SAMPLE_JSON:-}" ]]; then
    cp "$TOOLBENCH_SAMPLE_JSON" "$SAMPLE_JSON"
else
    echo "[dataset] downloading public ToolBench benchmark sample"
    python3 - "$RUN_DIR/tool_capsules" "$SAMPLE_JSON" "$TOOLBENCH_DATASET_URL" "$TOOLBENCH_SOURCE_SPLIT" "$TOOLBENCH_SAMPLE_INDEX" <<'PY'
import json
import sys
import urllib.request
from pathlib import Path

capsule_dir = Path(sys.argv[1])
sample_json = Path(sys.argv[2])
dataset_url = sys.argv[3]
source_split = sys.argv[4]
sample_index = int(sys.argv[5])
parquet_path = capsule_dir / "toolbench_sample.parquet"

try:
    import pyarrow.parquet as pq
except Exception as exc:
    raise SystemExit(f"pyarrow is required to read ToolBench parquet: {exc}") from exc

urllib.request.urlretrieve(dataset_url, parquet_path)
table = pq.read_table(parquet_path)
if table.num_rows == 0:
    raise SystemExit("ToolBench parquet has no rows")
row = table.slice(sample_index % table.num_rows, 1).to_pylist()[0]
sample = {
    "schema_version": "turingosv4.true_suite.toolbench_sample.v1",
    "query_id": str(row["query_id"]),
    "source_family": "ToolBench/ToolLLM",
    "public_source": "https://huggingface.co/datasets/tuandunghcmut/toolbench-v1",
    "source_split": source_split,
    "query": row["query"],
    "api_list": json.loads(row["api_list"]),
    "relevant_apis": json.loads(row["relevant_apis"]),
}
sample_json.write_text(json.dumps(sample, indent=2, sort_keys=True) + "\n", encoding="utf-8")
parquet_path.unlink(missing_ok=True)
PY
fi

echo "[toolbench] external LLM agent -> CAS tool claim -> signed WorkTx"
"$HELPER" \
    --runtime-repo "$RUN_DIR/runtime_repo" \
    --cas "$RUN_DIR/cas" \
    --run-id "$RUN_ID" \
    --constitution "$PROJECT_ROOT/constitution.md" \
    --sample-json "$SAMPLE_JSON" \
    --llm-proxy-url "$LLM_PROXY_URL" \
    --model "$ACTIVE_MODEL" \
    --out-dir "$RUN_DIR"

cp "$RUN_DIR/runtime_repo/genesis_report.json" "$RUN_DIR/genesis_report.json"

echo "[verify] turingos verify chaintape"
"$TURINGOS" verify chaintape \
    --repo "$RUN_DIR/runtime_repo" \
    --cas "$RUN_DIR/cas" \
    --run-id "$RUN_ID" \
    --out "$RUN_DIR/replay_report.json"

"$PARTICIPATION" \
    --run-id "$RUN_ID" \
    --family-id "toolbench_api_tool_use" \
    --entrypoint "scripts/run_true_suite_toolbench_current_kernel.sh" \
    --runtime-repo "$RUN_DIR/runtime_repo" \
    --cas "$RUN_DIR/cas" \
    --replay-report "$RUN_DIR/replay_report.json" \
    --genesis-report "$RUN_DIR/genesis_report.json" \
    --domain-manifest "$RUN_DIR/toolbench_api_tool_use_manifest.json" \
    --out "$RUN_DIR/full_system_participation.json"

cat > "$RUN_DIR/toolbench_api_tool_use_run_manifest.json" <<EOF
{
  "schema_version": "turingosv4.true_suite.toolbench_api_tool_use_run.v1",
  "run_id": "$RUN_ID",
  "git_head": "$(cd "$PROJECT_ROOT" && git rev-parse HEAD)",
  "entrypoint": "scripts/run_true_suite_toolbench_current_kernel.sh",
  "llm_proxy_url": "$LLM_PROXY_URL",
  "active_model": "$ACTIVE_MODEL",
  "toolbench_dataset_url": "$TOOLBENCH_DATASET_URL",
  "toolbench_source_split": "$TOOLBENCH_SOURCE_SPLIT",
  "toolbench_sample_index": $TOOLBENCH_SAMPLE_INDEX,
  "runtime_repo": "$RUN_DIR/runtime_repo",
  "cas": "$RUN_DIR/cas",
  "genesis_report": "$RUN_DIR/genesis_report.json",
  "toolbench_manifest": "$RUN_DIR/toolbench_api_tool_use_manifest.json",
  "fc_trace_report": "$RUN_DIR/fc_trace_report.json",
  "failure_taxonomy": "$RUN_DIR/failure_taxonomy.json",
  "replay_report": "$RUN_DIR/replay_report.json",
  "full_system_participation": "$RUN_DIR/full_system_participation.json",
  "notes": [
    "ToolBench data comes from the public tuandunghcmut/toolbench-v1 benchmark parquet unless TOOLBENCH_SAMPLE_JSON is explicitly supplied",
    "DeepSeek/SiliconFlow access is outside the kernel through the local LLM proxy",
    "raw provider prompt and response are not written to evidence",
    "this runner records API selection and structural tool-call hashes, not live third-party API side effects",
    "benchmark exact-match is not treated as liveness closure"
  ]
}
EOF

echo "TRUE-SUITE ToolBench API tool-use evidence: $RUN_DIR"
