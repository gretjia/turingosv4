#!/usr/bin/env bash
# True-suite SWE-bench coding-repair current-kernel evidence runner.
#
# Uses a public SWE-bench Lite sample plus a real external LLM through the
# local OpenAI-compatible proxy. The model patch is parsed into a structured
# claim, hashed into CAS, submitted as a signed WorkTx through current
# ChainTape, and replayed through public `turingos verify chaintape`.

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RUN_ID="${1:-swebench_live_coding_repair_$(date -u +%Y%m%dT%H%M%SZ)}"
RUN_ID="${RUN_ID#handover/evidence/true_suite/}"
RUN_ROOT="$PROJECT_ROOT/handover/evidence/true_suite/$RUN_ID"
RUN_DIR="$RUN_ROOT/swebench"
LLM_PROXY_URL="${LLM_PROXY_URL:-http://localhost:8080}"
ACTIVE_MODEL="${ACTIVE_MODEL:-deepseek-chat}"
SWEBENCH_DATASET_ROWS_URL="${SWEBENCH_DATASET_ROWS_URL:-https://datasets-server.huggingface.co/rows}"
SWEBENCH_DATASET="${SWEBENCH_DATASET:-princeton-nlp/SWE-bench_Lite}"
SWEBENCH_CONFIG="${SWEBENCH_CONFIG:-default}"
SWEBENCH_SPLIT="${SWEBENCH_SPLIT:-test}"
SWEBENCH_SAMPLE_INDEX="${SWEBENCH_SAMPLE_INDEX:-0}"

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

echo "[build] cargo build --release --bin turingos --bin verify_chaintape --bin swebench_live_coding_repair_current_kernel --bin full_system_augment_current_kernel --bin full_system_participation_current_kernel"
(cd "$PROJECT_ROOT" && cargo build --release --bin turingos --bin verify_chaintape --bin swebench_live_coding_repair_current_kernel --bin full_system_augment_current_kernel --bin full_system_participation_current_kernel)

TURINGOS="$PROJECT_ROOT/target/release/turingos"
HELPER="$PROJECT_ROOT/target/release/swebench_live_coding_repair_current_kernel"
AUGMENT="$PROJECT_ROOT/target/release/full_system_augment_current_kernel"
PARTICIPATION="$PROJECT_ROOT/target/release/full_system_participation_current_kernel"
SAMPLE_JSON="$RUN_DIR/repo_snapshots/swebench_sample.json"

echo "[init] turingos init --project $RUN_DIR"
"$TURINGOS" init --project "$RUN_DIR" --template proof --provider deepseek
mkdir -p "$RUN_DIR/repo_snapshots"

if [[ -n "${SWEBENCH_SAMPLE_JSON:-}" ]]; then
    cp "$SWEBENCH_SAMPLE_JSON" "$SAMPLE_JSON"
else
    echo "[dataset] downloading public SWE-bench Lite sample"
    python3 - "$RUN_DIR/repo_snapshots" "$SAMPLE_JSON" "$SWEBENCH_DATASET_ROWS_URL" "$SWEBENCH_DATASET" "$SWEBENCH_CONFIG" "$SWEBENCH_SPLIT" "$SWEBENCH_SAMPLE_INDEX" <<'PY'
import json
import sys
import urllib.parse
import urllib.request
from pathlib import Path

snapshot_dir = Path(sys.argv[1])
sample_json = Path(sys.argv[2])
rows_url = sys.argv[3]
dataset = sys.argv[4]
config = sys.argv[5]
split = sys.argv[6]
sample_index = int(sys.argv[7])

params = urllib.parse.urlencode({
    "dataset": dataset,
    "config": config,
    "split": split,
    "offset": sample_index,
    "length": 1,
})
dataset_url = f"{rows_url}?{params}"
with urllib.request.urlopen(dataset_url, timeout=30) as response:
    payload = json.load(response)
rows = payload.get("rows") or []
if not rows:
    raise SystemExit(f"SWE-bench rows API returned no rows: {dataset_url}")
row_idx = rows[0].get("row_idx", sample_index)
row = rows[0].get("row") or {}

def json_list_field(name):
    value = row.get(name)
    if isinstance(value, list):
        return [str(item) for item in value]
    if isinstance(value, str) and value.strip():
        try:
            parsed = json.loads(value)
            if isinstance(parsed, list):
                return [str(item) for item in parsed]
        except json.JSONDecodeError:
            return [value]
    return []

required = ["repo", "instance_id", "base_commit", "problem_statement", "patch", "test_patch"]
missing = [name for name in required if not str(row.get(name) or "").strip()]
if missing:
    raise SystemExit(f"SWE-bench row missing required fields: {missing}")

sample = {
    "schema_version": "turingosv4.true_suite.swebench_sample.v1",
    "sample_id": f"{dataset}:{split}:{row_idx}",
    "source_family": "SWE-bench_Lite",
    "public_source": "https://huggingface.co/datasets/princeton-nlp/SWE-bench_Lite",
    "source_file": f"datasets-server:{config}/{split}:{row_idx}",
    "repo": str(row["repo"]),
    "instance_id": str(row["instance_id"]),
    "base_commit": str(row["base_commit"]),
    "problem_statement": str(row["problem_statement"]),
    "hints_text": str(row.get("hints_text") or "") or None,
    "gold_patch": str(row["patch"]),
    "test_patch": str(row["test_patch"]),
    "fail_to_pass": json_list_field("FAIL_TO_PASS"),
    "pass_to_pass": json_list_field("PASS_TO_PASS"),
    "created_at": str(row.get("created_at") or "") or None,
    "version": str(row.get("version") or "") or None,
    "environment_setup_commit": str(row.get("environment_setup_commit") or "") or None,
}
sample_json.write_text(json.dumps(sample, indent=2, sort_keys=True) + "\n", encoding="utf-8")
(snapshot_dir / "dataset_request.json").write_text(
    json.dumps(
        {
            "dataset": dataset,
            "config": config,
            "split": split,
            "offset": sample_index,
            "length": 1,
            "url": dataset_url,
        },
        indent=2,
        sort_keys=True,
    )
    + "\n",
    encoding="utf-8",
)
PY
fi

echo "[swebench] external LLM agent -> CAS patch claim -> signed WorkTx"
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
    --family-id "swebench_live_coding_repair" \
    --entrypoint "scripts/run_true_suite_swebench_current_kernel.sh" \
    --runtime-repo "$RUN_DIR/runtime_repo" \
    --cas "$RUN_DIR/cas" \
    --replay-report "$RUN_DIR/replay_report.json" \
    --genesis-report "$RUN_DIR/genesis_report.json" \
    --domain-manifest "$RUN_DIR/swebench_live_coding_repair_manifest.json" \
    --fc3-index "$RUN_DIR/governance_capsule_index.json" \
    --require-full-system \
    --out "$RUN_DIR/full_system_participation.json"

cat > "$RUN_DIR/swebench_live_coding_repair_run_manifest.json" <<EOF
{
  "schema_version": "turingosv4.true_suite.swebench_live_coding_repair_run.v1",
  "run_id": "$RUN_ID",
  "git_head": "$(cd "$PROJECT_ROOT" && git rev-parse HEAD)",
  "entrypoint": "scripts/run_true_suite_swebench_current_kernel.sh",
  "llm_proxy_url": "$LLM_PROXY_URL",
  "active_model": "$ACTIVE_MODEL",
  "swebench_dataset_rows_url": "$SWEBENCH_DATASET_ROWS_URL",
  "swebench_dataset": "$SWEBENCH_DATASET",
  "swebench_config": "$SWEBENCH_CONFIG",
  "swebench_split": "$SWEBENCH_SPLIT",
  "swebench_sample_index": $SWEBENCH_SAMPLE_INDEX,
  "runtime_repo": "$RUN_DIR/runtime_repo",
  "cas": "$RUN_DIR/cas",
  "genesis_report": "$RUN_DIR/genesis_report.json",
  "swebench_manifest": "$RUN_DIR/swebench_live_coding_repair_manifest.json",
  "failure_taxonomy": "$RUN_DIR/failure_taxonomy.json",
  "full_system_augmentation_manifest": "$RUN_DIR/full_system_augmentation_manifest.json",
  "governance_capsule_index": "$RUN_DIR/governance_capsule_index.json",
  "replay_report": "$RUN_DIR/replay_report.json",
  "full_system_participation": "$RUN_DIR/full_system_participation.json",
  "notes": [
    "SWE-bench data comes from the public Princeton-NLP SWE-bench Lite rows API unless SWEBENCH_SAMPLE_JSON is explicitly supplied",
    "DeepSeek/SiliconFlow access is outside the kernel through the local LLM proxy",
    "raw provider prompt and response are not written to evidence",
    "gold patch and test patch are evaluation-side data only and do not enter the prompt",
    "structural patch plausibility is not final benchmark or OBL-005 closure"
  ]
}
EOF

echo "TRUE-SUITE SWE-bench coding-repair evidence: $RUN_DIR"
