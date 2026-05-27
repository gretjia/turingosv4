#!/usr/bin/env bash
# True-suite WebArena web-agent current-kernel evidence runner.
#
# Uses the official public WebArena task configuration plus an offline browser
# observation snapshot. The model answer is parsed into a structured claim,
# hashed into CAS, submitted as a signed WorkTx through current ChainTape, and
# replayed through public `turingos verify chaintape`.

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RUN_ID="${1:-webarena_web_agent_$(date -u +%Y%m%dT%H%M%SZ)}"
RUN_ID="${RUN_ID#handover/evidence/true_suite/}"
RUN_ROOT="$PROJECT_ROOT/handover/evidence/true_suite/$RUN_ID"
RUN_DIR="$RUN_ROOT/webarena"
LLM_PROXY_URL="${LLM_PROXY_URL:-http://localhost:8080}"
ACTIVE_MODEL="${ACTIVE_MODEL:-deepseek-chat}"
WEBARENA_CONFIG_URL="${WEBARENA_CONFIG_URL:-https://raw.githubusercontent.com/web-arena-x/webarena/main/config_files/test.raw.json}"
WEBARENA_SOURCE_FILE="${WEBARENA_SOURCE_FILE:-config_files/test.raw.json}"
WEBARENA_TASK_INDEX="${WEBARENA_TASK_INDEX:-${WEBARENA_SAMPLE_INDEX:-0}}"

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

echo "[build] cargo build --release --bin turingos --bin verify_chaintape --bin webarena_web_agent_current_kernel --bin full_system_augment_current_kernel --bin full_system_participation_current_kernel"
(cd "$PROJECT_ROOT" && cargo build --release --bin turingos --bin verify_chaintape --bin webarena_web_agent_current_kernel --bin full_system_augment_current_kernel --bin full_system_participation_current_kernel)

TURINGOS="$PROJECT_ROOT/target/release/turingos"
HELPER="$PROJECT_ROOT/target/release/webarena_web_agent_current_kernel"
AUGMENT="$PROJECT_ROOT/target/release/full_system_augment_current_kernel"
PARTICIPATION="$PROJECT_ROOT/target/release/full_system_participation_current_kernel"
SAMPLE_JSON="$RUN_DIR/input_capsules/webarena_sample.json"

echo "[init] turingos init --project $RUN_DIR"
"$TURINGOS" init --project "$RUN_DIR" --template proof --provider deepseek
mkdir -p "$RUN_DIR/input_capsules" "$RUN_DIR/browser_traces"

if [[ -n "${WEBARENA_SAMPLE_JSON:-}" ]]; then
    cp "$WEBARENA_SAMPLE_JSON" "$SAMPLE_JSON"
else
    echo "[dataset] materializing official WebArena task config sample"
    python3 - "$RUN_DIR/input_capsules" "$SAMPLE_JSON" "$WEBARENA_CONFIG_URL" "$WEBARENA_SOURCE_FILE" "$WEBARENA_TASK_INDEX" "${WEBARENA_OBSERVATION_HTML:-}" <<'PY'
import hashlib
import json
import sys
import urllib.request
from pathlib import Path

input_dir = Path(sys.argv[1])
sample_json = Path(sys.argv[2])
config_url = sys.argv[3]
source_file = sys.argv[4]
task_index = int(sys.argv[5])
observation_html_path = sys.argv[6]

try:
    with urllib.request.urlopen(config_url, timeout=60) as response:
        raw_bytes = response.read()
except Exception as exc:
    raise SystemExit(
        "WebArena public config download failed. Set WEBARENA_SAMPLE_JSON to a "
        f"pre-materialized sample if the network is unavailable. Underlying error: {exc}"
    )

loaded = json.loads(raw_bytes.decode("utf-8"))
if not isinstance(loaded, list) or not loaded:
    raise SystemExit(f"WebArena config has no task rows: {config_url}")
row = loaded[task_index % len(loaded)]

def reference_answer(eval_obj):
    answers = (eval_obj or {}).get("reference_answers") or {}
    if isinstance(answers, str):
        return answers.strip()
    if isinstance(answers, dict):
        for key in ("exact_match", "fuzzy_match", "must_include"):
            value = answers.get(key)
            if isinstance(value, str) and value.strip():
                return value.strip()
            if isinstance(value, list) and value:
                joined = ", ".join(str(item).strip() for item in value if str(item).strip())
                if joined:
                    return joined
    raw = (eval_obj or {}).get("reference_answer_raw_annotation")
    if isinstance(raw, str) and raw.strip():
        return raw.strip()
    raise SystemExit("WebArena row lacks a usable reference answer")

sites = row.get("sites") or []
if not isinstance(sites, list):
    sites = [str(sites)]
intent = str(row.get("intent") or "").strip()
start_url = str(row.get("start_url") or "").strip()
task_id = str(row.get("task_id") if row.get("task_id") is not None else task_index)
if not intent or not start_url:
    raise SystemExit(f"WebArena row lacks intent/start_url: task_id={task_id}")

if observation_html_path:
    observation_html = Path(observation_html_path).read_text(encoding="utf-8")
else:
    observation_html = "\n".join([
        "<html><body>",
        "<h1>Offline WebArena task snapshot</h1>",
        f"<p>Task id: {task_id}</p>",
        f"<p>Sites: {', '.join(sites) if sites else 'none declared'}</p>",
        f"<p>Start URL: {start_url}</p>",
        f"<p>Intent: {intent}</p>",
        "<p>No live website or account side effects were performed by this runner.</p>",
        "</body></html>",
    ])

sample = {
    "schema_version": "turingosv4.true_suite.webarena_sample.v1",
    "sample_id": f"webarena:{task_id}",
    "source_family": "WebArena",
    "public_source": "https://github.com/web-arena-x/webarena/blob/main/config_files/test.raw.json",
    "source_file": source_file,
    "source_config_sha256": hashlib.sha256(raw_bytes).hexdigest(),
    "task_id": task_id,
    "intent": intent,
    "start_url": start_url,
    "sites": sites,
    "allowed_tools": ["browser_sandbox"],
    "observation_html": observation_html,
    "reference_answer": reference_answer(row.get("eval")),
}
sample_json.write_text(json.dumps(sample, indent=2, sort_keys=True) + "\n", encoding="utf-8")
PY
fi

echo "[webarena] external LLM web agent -> CAS browser-action claim -> signed WorkTx"
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
    --family-id "webarena_web_agent" \
    --entrypoint "scripts/run_true_suite_webarena_current_kernel.sh" \
    --runtime-repo "$RUN_DIR/runtime_repo" \
    --cas "$RUN_DIR/cas" \
    --replay-report "$RUN_DIR/replay_report.json" \
    --genesis-report "$RUN_DIR/genesis_report.json" \
    --domain-manifest "$RUN_DIR/webarena_web_agent_manifest.json" \
    --fc3-index "$RUN_DIR/governance_capsule_index.json" \
    --require-full-system \
    --out "$RUN_DIR/full_system_participation.json"

cat > "$RUN_DIR/webarena_web_agent_run_manifest.json" <<EOF
{
  "schema_version": "turingosv4.true_suite.webarena_web_agent_run.v1",
  "run_id": "$RUN_ID",
  "git_head": "$(cd "$PROJECT_ROOT" && git rev-parse HEAD)",
  "entrypoint": "scripts/run_true_suite_webarena_current_kernel.sh",
  "llm_proxy_url": "$LLM_PROXY_URL",
  "active_model": "$ACTIVE_MODEL",
  "webarena_config_url": "$WEBARENA_CONFIG_URL",
  "webarena_source_file": "$WEBARENA_SOURCE_FILE",
  "webarena_task_index": $WEBARENA_TASK_INDEX,
  "runtime_repo": "$RUN_DIR/runtime_repo",
  "cas": "$RUN_DIR/cas",
  "genesis_report": "$RUN_DIR/genesis_report.json",
  "webarena_manifest": "$RUN_DIR/webarena_web_agent_manifest.json",
  "fc_trace_report": "$RUN_DIR/fc_trace_report.json",
  "failure_taxonomy": "$RUN_DIR/failure_taxonomy.json",
  "full_system_augmentation_manifest": "$RUN_DIR/full_system_augmentation_manifest.json",
  "governance_capsule_index": "$RUN_DIR/governance_capsule_index.json",
  "replay_report": "$RUN_DIR/replay_report.json",
  "full_system_participation": "$RUN_DIR/full_system_participation.json",
  "notes": [
    "WebArena task metadata comes from web-arena-x/webarena config_files/test.raw.json unless WEBARENA_SAMPLE_JSON is explicitly supplied",
    "WEBARENA_OBSERVATION_HTML can supply an offline browser snapshot; the default snapshot records task metadata only",
    "DeepSeek/SiliconFlow access is outside the kernel through the local LLM proxy",
    "raw provider prompt and response are not written to evidence",
    "no live website or account side effects are performed by this runner",
    "benchmark accuracy is not treated as liveness closure"
  ]
}
EOF

echo "TRUE-SUITE WebArena web-agent evidence: $RUN_DIR"
