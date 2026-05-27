#!/usr/bin/env bash
# True-suite OSWorld computer-use current-kernel evidence runner.
#
# Uses an OSWorld-style offline sandbox snapshot. The model action is parsed
# into a structured claim, hashed into CAS, submitted as a signed WorkTx through
# current ChainTape, and replayed through public `turingos verify chaintape`.

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RUN_ID="${1:-osworld_computer_use_$(date -u +%Y%m%dT%H%M%SZ)}"
RUN_ID="${RUN_ID#handover/evidence/true_suite/}"
RUN_ROOT="$PROJECT_ROOT/handover/evidence/true_suite/$RUN_ID"
RUN_DIR="$RUN_ROOT/osworld"
LLM_PROXY_URL="${LLM_PROXY_URL:-http://localhost:8080}"
ACTIVE_MODEL="${ACTIVE_MODEL:-deepseek-chat}"

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

echo "[build] cargo build --release --bin turingos --bin verify_chaintape --bin osworld_computer_use_current_kernel --bin full_system_augment_current_kernel --bin full_system_participation_current_kernel"
(cd "$PROJECT_ROOT" && cargo build --release --bin turingos --bin verify_chaintape --bin osworld_computer_use_current_kernel --bin full_system_augment_current_kernel --bin full_system_participation_current_kernel)

TURINGOS="$PROJECT_ROOT/target/release/turingos"
HELPER="$PROJECT_ROOT/target/release/osworld_computer_use_current_kernel"
AUGMENT="$PROJECT_ROOT/target/release/full_system_augment_current_kernel"
PARTICIPATION="$PROJECT_ROOT/target/release/full_system_participation_current_kernel"
SAMPLE_JSON="$RUN_DIR/input_capsules/osworld_sample.json"

echo "[init] turingos init --project $RUN_DIR"
"$TURINGOS" init --project "$RUN_DIR" --template proof --provider deepseek
mkdir -p "$RUN_DIR/input_capsules" "$RUN_DIR/sandbox_snapshots"

if [[ -n "${OSWORLD_SAMPLE_JSON:-}" ]]; then
    cp "$OSWORLD_SAMPLE_JSON" "$SAMPLE_JSON"
else
    echo "[dataset] materializing offline OSWorld-style sandbox sample"
    python3 - "$SAMPLE_JSON" "${OSWORLD_SNAPSHOT_TEXT:-}" <<'PY'
import json
import sys
from pathlib import Path

sample_json = Path(sys.argv[1])
snapshot_override = sys.argv[2]

snapshot = snapshot_override or "\n".join([
    "Offline OSWorld-style sandbox snapshot",
    "Environment: Ubuntu desktop sandbox, network disabled",
    "Visible file tree:",
    "  /home/oai/share/notes/draft.txt",
    "  /home/oai/share/notes/archive/",
    "Task-relevant observation:",
    "  draft.txt contains a short project note ready to be renamed.",
    "No host filesystem or live desktop side effects are performed by this runner.",
])

sample = {
    "schema_version": "turingosv4.true_suite.osworld_sample.v1",
    "sample_id": "osworld:offline-rename-draft-note",
    "source_family": "OSWorld",
    "public_source": "https://arxiv.org/abs/2404.07972",
    "source_file": "offline_osworld_style_sample.json",
    "task_id": "offline-rename-draft-note",
    "instruction": "In the sandboxed desktop, rename /home/oai/share/notes/draft.txt to /home/oai/share/notes/final.txt and report completion.",
    "environment": "ubuntu_desktop_sandbox",
    "network_policy": "offline_no_network",
    "allowed_tools": ["computer_sandbox"],
    "sandbox_snapshot_text": snapshot,
    "expected_action": "rename /home/oai/share/notes/draft.txt to /home/oai/share/notes/final.txt",
    "expected_final_state": "/home/oai/share/notes/final.txt exists and /home/oai/share/notes/draft.txt is absent",
}
sample_json.write_text(json.dumps(sample, indent=2, sort_keys=True) + "\n", encoding="utf-8")
PY
fi

echo "[osworld] external LLM computer-use agent -> CAS sandbox-action claim -> signed WorkTx"
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
    --family-id "osworld_computer_use" \
    --entrypoint "scripts/run_true_suite_osworld_current_kernel.sh" \
    --runtime-repo "$RUN_DIR/runtime_repo" \
    --cas "$RUN_DIR/cas" \
    --replay-report "$RUN_DIR/replay_report.json" \
    --genesis-report "$RUN_DIR/genesis_report.json" \
    --domain-manifest "$RUN_DIR/osworld_computer_use_manifest.json" \
    --fc3-index "$RUN_DIR/governance_capsule_index.json" \
    --require-full-system \
    --out "$RUN_DIR/full_system_participation.json"

cat > "$RUN_DIR/osworld_computer_use_run_manifest.json" <<EOF
{
  "schema_version": "turingosv4.true_suite.osworld_computer_use_run.v1",
  "run_id": "$RUN_ID",
  "git_head": "$(cd "$PROJECT_ROOT" && git rev-parse HEAD)",
  "entrypoint": "scripts/run_true_suite_osworld_current_kernel.sh",
  "llm_proxy_url": "$LLM_PROXY_URL",
  "active_model": "$ACTIVE_MODEL",
  "runtime_repo": "$RUN_DIR/runtime_repo",
  "cas": "$RUN_DIR/cas",
  "genesis_report": "$RUN_DIR/genesis_report.json",
  "osworld_manifest": "$RUN_DIR/osworld_computer_use_manifest.json",
  "fc_trace_report": "$RUN_DIR/fc_trace_report.json",
  "failure_taxonomy": "$RUN_DIR/failure_taxonomy.json",
  "full_system_augmentation_manifest": "$RUN_DIR/full_system_augmentation_manifest.json",
  "governance_capsule_index": "$RUN_DIR/governance_capsule_index.json",
  "replay_report": "$RUN_DIR/replay_report.json",
  "full_system_participation": "$RUN_DIR/full_system_participation.json",
  "notes": [
    "OSWorld task is represented as an offline sandbox snapshot derived from the public OSWorld computer-use task family",
    "OSWORLD_SAMPLE_JSON can supply a pre-materialized OSWorld-style sample",
    "DeepSeek/SiliconFlow access is outside the kernel through the local LLM proxy",
    "raw provider prompt and response are not written to evidence",
    "this runner records structural sandbox action evidence only; it performs no host OS side effects"
  ],
  "closure_scope": "domain_adapter_smoke_only",
  "final_closure_possible": false
}
EOF

echo "[done] OSWorld current-kernel evidence at $RUN_DIR"
