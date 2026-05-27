#!/usr/bin/env bash
# True-suite GAIA general-assistant current-kernel evidence runner.
#
# Uses a public GAIA dataset sample plus a real external LLM through the local
# OpenAI-compatible proxy. The model answer is parsed into a structured claim,
# hashed into CAS, submitted as a signed WorkTx through current ChainTape, and
# replayed through public `turingos verify chaintape`.

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RUN_ID="${1:-gaia_general_assistant_$(date -u +%Y%m%dT%H%M%SZ)}"
RUN_ID="${RUN_ID#handover/evidence/true_suite/}"
RUN_ROOT="$PROJECT_ROOT/handover/evidence/true_suite/$RUN_ID"
RUN_DIR="$RUN_ROOT/gaia"
LLM_PROXY_URL="${LLM_PROXY_URL:-http://localhost:8080}"
ACTIVE_MODEL="${ACTIVE_MODEL:-deepseek-chat}"
GAIA_DATASET_URL="${GAIA_DATASET_URL:-https://huggingface.co/datasets/gaia-benchmark/GAIA/resolve/main/2023/validation/metadata.parquet}"
GAIA_SOURCE_FILE="${GAIA_SOURCE_FILE:-2023/validation/metadata.parquet}"
GAIA_SAMPLE_INDEX="${GAIA_SAMPLE_INDEX:-0}"

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

echo "[build] cargo build --release --bin turingos --bin verify_chaintape --bin gaia_general_assistant_current_kernel --bin full_system_augment_current_kernel --bin full_system_participation_current_kernel"
(cd "$PROJECT_ROOT" && cargo build --release --bin turingos --bin verify_chaintape --bin gaia_general_assistant_current_kernel --bin full_system_augment_current_kernel --bin full_system_participation_current_kernel)

TURINGOS="$PROJECT_ROOT/target/release/turingos"
HELPER="$PROJECT_ROOT/target/release/gaia_general_assistant_current_kernel"
AUGMENT="$PROJECT_ROOT/target/release/full_system_augment_current_kernel"
PARTICIPATION="$PROJECT_ROOT/target/release/full_system_participation_current_kernel"
SAMPLE_JSON="$RUN_DIR/input_capsules/gaia_sample.json"

echo "[init] turingos init --project $RUN_DIR"
"$TURINGOS" init --project "$RUN_DIR" --template proof --provider deepseek
mkdir -p "$RUN_DIR/input_capsules"

if [[ -n "${GAIA_SAMPLE_JSON:-}" ]]; then
    cp "$GAIA_SAMPLE_JSON" "$SAMPLE_JSON"
else
    echo "[dataset] downloading public GAIA validation metadata sample"
    python3 - "$RUN_DIR/input_capsules" "$SAMPLE_JSON" "$GAIA_DATASET_URL" "$GAIA_SOURCE_FILE" "$GAIA_SAMPLE_INDEX" "${GAIA_HF_TOKEN:-}" <<'PY'
import json
import sys
import urllib.request
from pathlib import Path

input_dir = Path(sys.argv[1])
sample_json = Path(sys.argv[2])
dataset_url = sys.argv[3]
source_file = sys.argv[4]
sample_index = int(sys.argv[5])
hf_token = sys.argv[6]

raw_path = input_dir / "gaia_metadata"
headers = {}
if hf_token:
    headers["Authorization"] = f"Bearer {hf_token}"
req = urllib.request.Request(dataset_url, headers=headers)
try:
    with urllib.request.urlopen(req, timeout=60) as response:
        raw_path.write_bytes(response.read())
except Exception as exc:
    raise SystemExit(
        "GAIA validation metadata download failed. The official GAIA Hugging Face "
        "dataset may require accepting terms and setting GAIA_HF_TOKEN, or set "
        f"GAIA_SAMPLE_JSON to a pre-materialized sample. Underlying error: {exc}"
    )

if dataset_url.endswith(".jsonl"):
    rows = [json.loads(line) for line in raw_path.read_text(encoding="utf-8").splitlines() if line.strip()]
elif dataset_url.endswith(".json"):
    loaded = json.loads(raw_path.read_text(encoding="utf-8"))
    rows = loaded if isinstance(loaded, list) else loaded.get("rows", [])
else:
    try:
        import pyarrow.parquet as pq
    except Exception as exc:
        raise SystemExit(f"reading GAIA parquet requires pyarrow: {exc}")
    table = pq.read_table(raw_path)
    rows = table.to_pylist()
if not rows:
    raise SystemExit(f"GAIA metadata has no rows: {dataset_url}")
row = rows[sample_index % len(rows)]

def get(*names, default=""):
    lowered = {str(k).lower().replace(" ", "_"): v for k, v in row.items()}
    for name in names:
        key = name.lower().replace(" ", "_")
        if key in lowered and lowered[key] is not None:
            return str(lowered[key]).strip()
    return default

task = get("Question", "question", "task")
answer = get("Final answer", "final_answer", "answer", "correct_answer")
if not task or not answer:
    raise SystemExit(f"GAIA row lacks task/final answer fields: keys={sorted(row.keys())}")
sample = {
    "schema_version": "turingosv4.true_suite.gaia_sample.v1",
    "sample_id": get("task_id", "Task ID", "id", default=f"gaia:{sample_index}"),
    "source_family": "GAIA",
    "public_source": "https://huggingface.co/datasets/gaia-benchmark/GAIA",
    "source_file": source_file,
    "level": get("Level", "level") or None,
    "task": task,
    "expected_answer": answer,
    "allowed_tools": ["web_browsing", "file_inspection", "python_scratchpad"],
    "file_name": get("file_name", "File Name") or None,
    "canary_string": None,
}
sample_json.write_text(json.dumps(sample, indent=2, sort_keys=True) + "\n", encoding="utf-8")
raw_path.unlink(missing_ok=True)
PY
fi

echo "[gaia] external LLM agent -> CAS answer claim -> signed WorkTx"
"$HELPER" \
    --runtime-repo "$RUN_DIR/runtime_repo" \
    --cas "$RUN_DIR/cas" \
    --run-id "$RUN_ID" \
    --constitution "$PROJECT_ROOT/constitution.md" \
    --sample-json "$SAMPLE_JSON" \
    --llm-proxy-url "$LLM_PROXY_URL" \
    --model "$ACTIVE_MODEL" \
    --out-dir "$RUN_DIR"

echo "[augment] append market + FC3 participation rows to the same ChainTape"
"$AUGMENT" \
    --runtime-repo "$RUN_DIR/runtime_repo" \
    --cas "$RUN_DIR/cas" \
    --run-id "$RUN_ID" \
    --constitution "$PROJECT_ROOT/constitution.md" \
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
    --family-id "gaia_general_assistant" \
    --entrypoint "scripts/run_true_suite_gaia_general_assistant_current_kernel.sh" \
    --runtime-repo "$RUN_DIR/runtime_repo" \
    --cas "$RUN_DIR/cas" \
    --replay-report "$RUN_DIR/replay_report.json" \
    --genesis-report "$RUN_DIR/genesis_report.json" \
    --domain-manifest "$RUN_DIR/gaia_general_assistant_manifest.json" \
    --fc3-index "$RUN_DIR/governance_capsule_index.json" \
    --require-full-system \
    --out "$RUN_DIR/full_system_participation.json"

cat > "$RUN_DIR/gaia_general_assistant_run_manifest.json" <<EOF
{
  "schema_version": "turingosv4.true_suite.gaia_general_assistant_run.v1",
  "run_id": "$RUN_ID",
  "git_head": "$(cd "$PROJECT_ROOT" && git rev-parse HEAD)",
  "entrypoint": "scripts/run_true_suite_gaia_general_assistant_current_kernel.sh",
  "llm_proxy_url": "$LLM_PROXY_URL",
  "active_model": "$ACTIVE_MODEL",
  "gaia_dataset_url": "$GAIA_DATASET_URL",
  "gaia_source_file": "$GAIA_SOURCE_FILE",
  "gaia_sample_index": $GAIA_SAMPLE_INDEX,
  "runtime_repo": "$RUN_DIR/runtime_repo",
  "cas": "$RUN_DIR/cas",
  "genesis_report": "$RUN_DIR/genesis_report.json",
  "gaia_manifest": "$RUN_DIR/gaia_general_assistant_manifest.json",
  "fc_trace_report": "$RUN_DIR/fc_trace_report.json",
  "failure_taxonomy": "$RUN_DIR/failure_taxonomy.json",
  "full_system_augmentation_manifest": "$RUN_DIR/full_system_augmentation_manifest.json",
  "governance_capsule_index": "$RUN_DIR/governance_capsule_index.json",
  "replay_report": "$RUN_DIR/replay_report.json",
  "full_system_participation": "$RUN_DIR/full_system_participation.json",
  "notes": [
    "GAIA data comes from the official gaia-benchmark/GAIA validation metadata unless GAIA_SAMPLE_JSON is explicitly supplied",
    "GAIA_HF_TOKEN may be required by Hugging Face access controls after accepting the dataset terms",
    "DeepSeek/SiliconFlow access is outside the kernel through the local LLM proxy",
    "raw provider prompt and response are not written to evidence",
    "benchmark accuracy is not treated as liveness closure"
  ]
}
EOF

echo "TRUE-SUITE GAIA general-assistant evidence: $RUN_DIR"
