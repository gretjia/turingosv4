#!/usr/bin/env bash
# True-suite Mind2Web browser-action current-kernel evidence runner.
#
# Uses a public Mind2Web offline webpage snapshot plus a real external LLM
# through the local OpenAI-compatible proxy. The model's browser action is
# parsed into a structured claim, hashed into CAS, submitted as a signed WorkTx
# through current ChainTape, and replayed through public `turingos verify
# chaintape`. This runner never drives a live website or account.

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RUN_ID="${1:-mind2web_browser_action_$(date -u +%Y%m%dT%H%M%SZ)}"
RUN_ID="${RUN_ID#handover/evidence/true_suite/}"
RUN_ROOT="$PROJECT_ROOT/handover/evidence/true_suite/$RUN_ID"
RUN_DIR="$RUN_ROOT/mind2web"
LLM_PROXY_URL="${LLM_PROXY_URL:-http://localhost:8080}"
ACTIVE_MODEL="${ACTIVE_MODEL:-deepseek-chat}"
MIND2WEB_DATASET_URL="${MIND2WEB_DATASET_URL:-https://huggingface.co/datasets/osunlp/Mind2Web/resolve/main/data/train/train_0.json}"
MIND2WEB_SOURCE_FILE="${MIND2WEB_SOURCE_FILE:-data/train/train_0.json}"
MIND2WEB_TASK_INDEX="${MIND2WEB_TASK_INDEX:-0}"
MIND2WEB_ACTION_INDEX="${MIND2WEB_ACTION_INDEX:-0}"

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

echo "[build] cargo build --release --bin turingos --bin verify_chaintape --bin mind2web_browser_action_current_kernel --bin full_system_augment_current_kernel --bin full_system_participation_current_kernel"
(cd "$PROJECT_ROOT" && cargo build --release --bin turingos --bin verify_chaintape --bin mind2web_browser_action_current_kernel --bin full_system_augment_current_kernel --bin full_system_participation_current_kernel)

TURINGOS="$PROJECT_ROOT/target/release/turingos"
HELPER="$PROJECT_ROOT/target/release/mind2web_browser_action_current_kernel"
AUGMENT="$PROJECT_ROOT/target/release/full_system_augment_current_kernel"
PARTICIPATION="$PROJECT_ROOT/target/release/full_system_participation_current_kernel"
SAMPLE_JSON="$RUN_DIR/input_capsules/mind2web_sample.json"

echo "[init] turingos init --project $RUN_DIR"
"$TURINGOS" init --project "$RUN_DIR" --template proof --provider deepseek
mkdir -p "$RUN_DIR/input_capsules" "$RUN_DIR/page_snapshots"

if [[ -n "${MIND2WEB_SAMPLE_JSON:-}" ]]; then
    cp "$MIND2WEB_SAMPLE_JSON" "$SAMPLE_JSON"
else
    echo "[dataset] downloading public Mind2Web offline webpage-action sample"
    python3 - "$RUN_DIR/input_capsules" "$SAMPLE_JSON" "$MIND2WEB_DATASET_URL" "$MIND2WEB_SOURCE_FILE" "$MIND2WEB_TASK_INDEX" "$MIND2WEB_ACTION_INDEX" <<'PY'
import json
import sys
import urllib.request
from pathlib import Path

input_dir = Path(sys.argv[1])
sample_json = Path(sys.argv[2])
dataset_url = sys.argv[3]
source_file = sys.argv[4]
task_index = int(sys.argv[5])
action_index = int(sys.argv[6])
raw_path = input_dir / "mind2web_source.json"

urllib.request.urlretrieve(dataset_url, raw_path)
with raw_path.open(encoding="utf-8") as fh:
    rows = json.load(fh)
if not rows:
    raise SystemExit("Mind2Web source has no tasks")
task = rows[task_index % len(rows)]
actions = task.get("actions") or []
if not actions:
    raise SystemExit("Mind2Web task has no actions")
action = actions[action_index % len(actions)]
pos = action.get("pos_candidates") or []
neg = action.get("neg_candidates") or []
if not pos:
    raise SystemExit("Mind2Web action has no positive candidates")
sample = {
    "schema_version": "turingosv4.true_suite.mind2web_sample.v1",
    "sample_id": f"{task.get('annotation_id')}:{action_index % len(actions)}",
    "source_family": "Mind2Web",
    "public_source": "https://huggingface.co/datasets/osunlp/Mind2Web",
    "source_file": source_file,
    "website": task.get("website") or "",
    "domain": task.get("domain") or "",
    "subdomain": task.get("subdomain") or "",
    "annotation_id": task.get("annotation_id") or f"task-{task_index}",
    "confirmed_task": task.get("confirmed_task") or "",
    "action_index": action_index % len(actions),
    "action_repr": (task.get("action_reprs") or [""])[action_index % len(actions)],
    "cleaned_html": action.get("cleaned_html") or "",
    "operation": action.get("operation") or {},
    "pos_candidates": pos[:8],
    "neg_candidates": neg[:40],
}
sample_json.write_text(json.dumps(sample, indent=2, sort_keys=True) + "\n", encoding="utf-8")
raw_path.unlink(missing_ok=True)
PY
fi

echo "[mind2web] external LLM agent -> CAS browser action claim -> signed WorkTx"
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
    --family-id "mind2web_open_web" \
    --entrypoint "scripts/run_true_suite_mind2web_current_kernel.sh" \
    --runtime-repo "$RUN_DIR/runtime_repo" \
    --cas "$RUN_DIR/cas" \
    --replay-report "$RUN_DIR/replay_report.json" \
    --genesis-report "$RUN_DIR/genesis_report.json" \
    --domain-manifest "$RUN_DIR/mind2web_browser_action_manifest.json" \
    --fc3-index "$RUN_DIR/governance_capsule_index.json" \
    --require-full-system \
    --out "$RUN_DIR/full_system_participation.json"

cat > "$RUN_DIR/mind2web_browser_action_run_manifest.json" <<EOF
{
  "schema_version": "turingosv4.true_suite.mind2web_browser_action_run.v1",
  "run_id": "$RUN_ID",
  "git_head": "$(cd "$PROJECT_ROOT" && git rev-parse HEAD)",
  "entrypoint": "scripts/run_true_suite_mind2web_current_kernel.sh",
  "llm_proxy_url": "$LLM_PROXY_URL",
  "active_model": "$ACTIVE_MODEL",
  "mind2web_dataset_url": "$MIND2WEB_DATASET_URL",
  "mind2web_source_file": "$MIND2WEB_SOURCE_FILE",
  "mind2web_task_index": $MIND2WEB_TASK_INDEX,
  "mind2web_action_index": $MIND2WEB_ACTION_INDEX,
  "runtime_repo": "$RUN_DIR/runtime_repo",
  "cas": "$RUN_DIR/cas",
  "genesis_report": "$RUN_DIR/genesis_report.json",
  "mind2web_manifest": "$RUN_DIR/mind2web_browser_action_manifest.json",
  "fc_trace_report": "$RUN_DIR/fc_trace_report.json",
  "failure_taxonomy": "$RUN_DIR/failure_taxonomy.json",
  "full_system_augmentation_manifest": "$RUN_DIR/full_system_augmentation_manifest.json",
  "governance_capsule_index": "$RUN_DIR/governance_capsule_index.json",
  "replay_report": "$RUN_DIR/replay_report.json",
  "full_system_participation": "$RUN_DIR/full_system_participation.json",
  "notes": [
    "Mind2Web data comes from the public osunlp/Mind2Web offline snapshot unless MIND2WEB_SAMPLE_JSON is explicitly supplied",
    "DeepSeek/SiliconFlow access is outside the kernel through the local LLM proxy",
    "raw provider prompt and response are not written to evidence",
    "this runner records offline browser action selection over a static webpage snapshot, not live website side effects",
    "benchmark exact-match is not treated as liveness closure"
  ]
}
EOF

echo "TRUE-SUITE Mind2Web browser-action evidence: $RUN_DIR"
