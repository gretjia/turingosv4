#!/usr/bin/env bash
# True-suite generate/artifact current-kernel evidence runner.
#
# Uses a real external LLM through the local OpenAI-compatible proxy, then
# drives the public CLI path:
#   turingos init -> turingos spec -> turingos generate --from-capsule
#   -> turingos verify chaintape.

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RUN_ID="${1:-generate_artifact_current_kernel_$(date -u +%Y%m%dT%H%M%SZ)}"
RUN_ID="${RUN_ID#handover/evidence/true_suite/}"
RUN_ROOT="$PROJECT_ROOT/handover/evidence/true_suite/$RUN_ID"
RUN_DIR="$RUN_ROOT/generate_artifact"
LLM_PROXY_URL="${LLM_PROXY_URL:-http://localhost:8080}"
LLM_ENDPOINT="${LLM_ENDPOINT:-${LLM_PROXY_URL%/}/v1/chat/completions}"
INIT_PROVIDER="${INIT_PROVIDER:-deepseek}"
N_PARALLEL_WORKERS="${N_PARALLEL_WORKERS:-1}"

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

mkdir -p "$RUN_DIR"

if [[ -n "$(cd "$PROJECT_ROOT" && git status --porcelain | grep -vE '^\?\? handover/evidence/' | head -1)" ]]; then
    echo "ERROR: working tree has non-evidence changes; run /runner-preflight before evidence runners" >&2
    exit 3
fi

if ! curl -sS --max-time 5 "$LLM_PROXY_URL/health" | grep -q '"status": "ok"'; then
    echo "ERROR: LLM proxy $LLM_PROXY_URL/health not OK" >&2
    echo "Start one with: python3 src/drivers/llm_proxy.py --port 8080" >&2
    exit 4
fi

echo "[build] cargo build --release --bin turingos --bin verify_chaintape"
(cd "$PROJECT_ROOT" && cargo build --release --bin turingos --bin verify_chaintape)

TURINGOS="$PROJECT_ROOT/target/release/turingos"

echo "[init] turingos init --project $RUN_DIR --provider $INIT_PROVIDER"
"$TURINGOS" init --project "$RUN_DIR" --template proof --provider "$INIT_PROVIDER"

cat > "$RUN_DIR/answers.json" <<'EOF'
[
  "I need a small browser tool for comparing three project launch plans. I keep doing this in notes and losing the tradeoffs.",
  "A lightweight decision matrix app, but without accounts, databases, or external services.",
  "It should remember the three plans, criteria weights, and the current score table inside the page state while I am using it.",
  "I open one page, see three plan columns, edit scores from 1 to 5, and immediately see weighted totals and a recommended plan.",
  "Blank scores, equal totals, long plan names, and accidental non-number input should not break it.",
  "No login, no backend, no cloud sync, no collaboration, no payments, no external packages.",
  "After a month, I know it works if I can paste a launch scenario into the page, compare options in under five minutes, and explain the final choice.",
  "Build a self-contained HTML launch-plan decision matrix with editable scores, weighted totals, clear recommendation, and no external runtime dependencies."
]
EOF

echo "[spec] provider-backed spec synthesis -> CAS spec capsule"
SPEC_OUTPUT="$(
    TURINGOS_SILICONFLOW_ENDPOINT="$LLM_ENDPOINT" \
    "$TURINGOS" spec \
        --workspace "$RUN_DIR" \
        --answers-file "$RUN_DIR/answers.json" \
        --lang en
)"
printf '%s\n' "$SPEC_OUTPUT" > "$RUN_DIR/spec_output.txt"
SPEC_CAPSULE_CID="$(printf '%s\n' "$SPEC_OUTPUT" | sed -n 's/^  CAS capsule CID    -> //p' | tail -1)"
if [[ -z "$SPEC_CAPSULE_CID" ]]; then
    echo "ERROR: could not parse spec capsule CID" >&2
    exit 5
fi

echo "[generate] provider-backed artifact generation -> CAS bundle + ChainTape WorkTx"
GENERATE_OUTPUT="$(
    TURINGOS_SILICONFLOW_ENDPOINT="$LLM_ENDPOINT" \
    "$TURINGOS" generate \
        --workspace "$RUN_DIR" \
        --from-capsule \
        --entrypoint index.html \
        --n-parallel-workers "$N_PARALLEL_WORKERS"
)"
printf '%s\n' "$GENERATE_OUTPUT" > "$RUN_DIR/generate_output.txt"
ARTIFACT_BUNDLE_CID="$(printf '%s\n' "$GENERATE_OUTPUT" | sed -n 's/^artifact_bundle_cid=//p' | tail -1)"
if [[ -z "$ARTIFACT_BUNDLE_CID" ]]; then
    echo "ERROR: could not parse artifact bundle CID" >&2
    exit 6
fi
CHAIN_RUN_ID="$(sed -n 's/.*"run_id": "\(.*\)".*/\1/p' "$RUN_DIR/runtime_repo/pinned_pubkeys.json" | head -1)"
if [[ -z "$CHAIN_RUN_ID" ]]; then
    echo "ERROR: could not parse ChainTape run_id from pinned_pubkeys.json" >&2
    exit 7
fi

cat > "$RUN_DIR/artifact_bundle_cid.json" <<EOF
{
  "schema_version": "turingosv4.true_suite.generate_artifact_bundle_cid.v1",
  "run_id": "$RUN_ID",
  "chain_run_id": "$CHAIN_RUN_ID",
  "spec_capsule_cid": "$SPEC_CAPSULE_CID",
  "artifact_bundle_cid": "$ARTIFACT_BUNDLE_CID",
  "workspace": "$RUN_DIR",
  "runtime_repo": "$RUN_DIR/runtime_repo",
  "cas": "$RUN_DIR/cas"
}
EOF

cp "$RUN_DIR/runtime_repo/genesis_report.json" "$RUN_DIR/genesis_report.json"

echo "[verify] turingos verify chaintape"
"$TURINGOS" verify chaintape \
    --repo "$RUN_DIR/runtime_repo" \
    --cas "$RUN_DIR/cas" \
    --run-id "$CHAIN_RUN_ID" \
    --out "$RUN_DIR/replay_report.json"

cat > "$RUN_DIR/generate_artifact_run_manifest.json" <<EOF
{
  "schema_version": "turingosv4.true_suite.generate_artifact_current_kernel.v1",
  "run_id": "$RUN_ID",
  "chain_run_id": "$CHAIN_RUN_ID",
  "git_head": "$(cd "$PROJECT_ROOT" && git rev-parse HEAD)",
  "entrypoint": "scripts/run_true_suite_generate_artifact_current_kernel.sh",
  "llm_proxy_url": "$LLM_PROXY_URL",
  "llm_endpoint": "$LLM_ENDPOINT",
  "init_provider": "$INIT_PROVIDER",
  "n_parallel_workers": $N_PARALLEL_WORKERS,
  "runtime_repo": "$RUN_DIR/runtime_repo",
  "cas": "$RUN_DIR/cas",
  "genesis_report": "$RUN_DIR/genesis_report.json",
  "artifact_bundle_cid": "$RUN_DIR/artifact_bundle_cid.json",
  "replay_report": "$RUN_DIR/replay_report.json",
  "notes": [
    "DeepSeek/SiliconFlow access is outside the kernel through the local LLM proxy",
    "spec synthesis is anchored as a CAS spec capsule",
    "generate reads the spec from CAS via --from-capsule",
    "accepted artifact work lands as typed ChainTape entries and replays through public verify"
  ]
}
EOF

echo "TRUE-SUITE generate/artifact evidence: $RUN_DIR"
