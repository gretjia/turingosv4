#!/usr/bin/env bash
# True-suite market/economy current-kernel evidence runner.
#
# Uses a real external LLM agent through the local OpenAI-compatible proxy.
# The agent's parsed decision is converted into a signed BuyWithCoinRouterTx
# by the runner helper and submitted through the current ChainTape sequencer.

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RUN_ID="${1:-market_external_agent_$(date -u +%Y%m%dT%H%M%SZ)}"
RUN_ID="${RUN_ID#handover/evidence/true_suite/}"
RUN_ROOT="$PROJECT_ROOT/handover/evidence/true_suite/$RUN_ID"
RUN_DIR="$RUN_ROOT/market_action"
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

mkdir -p "$RUN_ROOT"

if [[ -n "$(cd "$PROJECT_ROOT" && git status --porcelain | grep -vE '^\?\? handover/evidence/' | head -1)" ]]; then
    echo "ERROR: working tree has non-evidence changes; run /runner-preflight before evidence runners" >&2
    exit 3
fi

if ! curl -sS --max-time 5 "$LLM_PROXY_URL/health" | grep -q '"status": "ok"'; then
    echo "ERROR: LLM proxy $LLM_PROXY_URL/health not OK" >&2
    echo "Start one with: python3 src/drivers/llm_proxy.py --port 8080" >&2
    exit 4
fi

echo "[build] cargo build --release --bin turingos --bin verify_chaintape --bin market_external_agent_current_kernel --bin full_system_augment_current_kernel --bin full_system_participation_current_kernel"
(cd "$PROJECT_ROOT" && cargo build --release --bin turingos --bin verify_chaintape --bin market_external_agent_current_kernel --bin full_system_augment_current_kernel --bin full_system_participation_current_kernel)

TURINGOS="$PROJECT_ROOT/target/release/turingos"
HELPER="$PROJECT_ROOT/target/release/market_external_agent_current_kernel"
AUGMENT="$PROJECT_ROOT/target/release/full_system_augment_current_kernel"
PARTICIPATION="$PROJECT_ROOT/target/release/full_system_participation_current_kernel"

echo "[init] turingos init --project $RUN_DIR"
"$TURINGOS" init --project "$RUN_DIR" --template proof --provider deepseek

echo "[market] external LLM agent -> signed BuyWithCoinRouterTx"
"$HELPER" \
    --runtime-repo "$RUN_DIR/runtime_repo" \
    --cas "$RUN_DIR/cas" \
    --run-id "$RUN_ID" \
    --constitution "$PROJECT_ROOT/constitution.md" \
    --llm-proxy-url "$LLM_PROXY_URL" \
    --model "$ACTIVE_MODEL" \
    --out "$RUN_DIR/external_agent_market_manifest.json"

echo "[augment] append FC3 participation rows"
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
    --family-id "market_economy_polymarket" \
    --entrypoint "scripts/run_true_suite_market_external_agent.sh" \
    --runtime-repo "$RUN_DIR/runtime_repo" \
    --cas "$RUN_DIR/cas" \
    --replay-report "$RUN_DIR/replay_report.json" \
    --genesis-report "$RUN_DIR/genesis_report.json" \
    --fc3-index "$RUN_DIR/governance_capsule_index.json" \
    --domain-manifest "$RUN_DIR/external_agent_market_manifest.json" \
    --out "$RUN_DIR/full_system_participation.json" \
    --require-full-system

cat > "$RUN_DIR/market_external_agent_run_manifest.json" <<EOF
{
  "schema_version": "turingosv4.true_suite.market_external_agent_run.v1",
  "run_id": "$RUN_ID",
  "git_head": "$(cd "$PROJECT_ROOT" && git rev-parse HEAD)",
  "entrypoint": "scripts/run_true_suite_market_external_agent.sh",
  "llm_proxy_url": "$LLM_PROXY_URL",
  "active_model": "$ACTIVE_MODEL",
  "runtime_repo": "$RUN_DIR/runtime_repo",
  "cas": "$RUN_DIR/cas",
  "genesis_report": "$RUN_DIR/genesis_report.json",
  "external_agent_market_manifest": "$RUN_DIR/external_agent_market_manifest.json",
  "full_system_augmentation_manifest": "$RUN_DIR/full_system_augmentation_manifest.json",
  "governance_capsule_index": "$RUN_DIR/governance_capsule_index.json",
  "replay_report": "$RUN_DIR/replay_report.json",
  "full_system_participation": "$RUN_DIR/full_system_participation.json",
  "notes": [
    "DeepSeek/SiliconFlow access is outside the kernel through the local LLM proxy",
    "raw provider prompt and response are not written to evidence",
    "market action is admitted only as signed typed tx on ChainTape"
  ]
}
EOF

echo "TRUE-SUITE market/economy evidence: $RUN_DIR"
