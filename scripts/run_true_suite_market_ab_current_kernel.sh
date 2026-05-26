#!/usr/bin/env bash
# True-suite market A/B current-kernel evidence runner.
#
# This runner is a liveness bridge for the market-performance domain. It keeps
# REAL-16's candidate-only report boundary, but uses fresh current-kernel
# ChainTape/CAS arms instead of treating old G-phase dashboard reports as final
# OBL-005 evidence.

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RUN_ID="${1:-market_ab_full_system_$(date -u +%Y%m%dT%H%M%SZ)}"
RUN_ID="${RUN_ID#handover/evidence/true_suite/}"
RUN_ROOT="$PROJECT_ROOT/handover/evidence/true_suite/$RUN_ID"
RUN_DIR="$RUN_ROOT/market_ab"
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
fi

mkdir -p "$RUN_DIR/arm_config_manifests"

if [[ -n "$(cd "$PROJECT_ROOT" && git status --porcelain | grep -vE '^\?\? handover/evidence/' | head -1)" ]]; then
    echo "ERROR: working tree has non-evidence changes; run /runner-preflight before evidence runners" >&2
    exit 3
fi

if ! curl -sS --max-time 5 "$LLM_PROXY_URL/health" | grep -q '"status": "ok"'; then
    echo "ERROR: LLM proxy $LLM_PROXY_URL/health not OK" >&2
    echo "Start one with: python3 src/drivers/llm_proxy.py --port 8080" >&2
    exit 4
fi

echo "[build] cargo build --release --bin turingos --bin verify_chaintape --bin market_external_agent_current_kernel --bin full_system_augment_current_kernel --bin full_system_participation_current_kernel --bin audit_tape --bin real14_e2_candidate_verifier --bin real16_market_performance_verifier"
(cd "$PROJECT_ROOT" && cargo build --release \
    --bin turingos \
    --bin verify_chaintape \
    --bin market_external_agent_current_kernel \
    --bin full_system_augment_current_kernel \
    --bin full_system_participation_current_kernel \
    --bin audit_tape \
    --bin real14_e2_candidate_verifier \
    --bin real16_market_performance_verifier)

TURINGOS="$PROJECT_ROOT/target/release/turingos"
MARKET_HELPER="$PROJECT_ROOT/target/release/market_external_agent_current_kernel"
AUGMENT="$PROJECT_ROOT/target/release/full_system_augment_current_kernel"
PARTICIPATION="$PROJECT_ROOT/target/release/full_system_participation_current_kernel"
AUDIT_TAPE="$PROJECT_ROOT/target/release/audit_tape"
E2_VERIFIER="$PROJECT_ROOT/target/release/real14_e2_candidate_verifier"
REAL16_VERIFIER="$PROJECT_ROOT/target/release/real16_market_performance_verifier"
BIN_DIR="$PROJECT_ROOT/target/release"

cat > "$RUN_DIR/problems.pinned.txt" <<'EOF'
public_market_ab_current_kernel_probe
EOF
cat > "$RUN_DIR/model_assignment.pinned.env" <<EOF
ACTIVE_MODEL=$ACTIVE_MODEL
LLM_PROXY_URL=$LLM_PROXY_URL
EOF
cat > "$RUN_DIR/budgets.pinned.env" <<'EOF'
MAX_MARKET_ACTIONS_PER_ARM=1
FULL_SYSTEM_PARTICIPATION_REQUIRED=1
EOF
cat > "$RUN_DIR/arm_config_manifests/arm_diff_allowlist.txt" <<'EOF'
ARM
ARM_CONDITION
RUN_DIR
MARKET_PRESSURE_ENABLED
FULL_SYSTEM_PARTICIPATION_REQUIRED
EOF

PROBLEM_SET_HASH="$(sha256sum "$RUN_DIR/problems.pinned.txt" | awk '{print $1}')"
MODEL_ASSIGNMENT_HASH="$(sha256sum "$RUN_DIR/model_assignment.pinned.env" | awk '{print $1}')"
BUDGET_HASH="$(sha256sum "$RUN_DIR/budgets.pinned.env" | awk '{print $1}')"
PROMPT_TEMPLATE_HASH="$(sha256sum "$PROJECT_ROOT/src/bin/market_external_agent_current_kernel.rs" | awk '{print $1}')"
RUNTIME_CONFIG_HASH="$(
    {
        sha256sum "$PROJECT_ROOT/scripts/run_true_suite_market_ab_current_kernel.sh"
        sha256sum "$PROJECT_ROOT/src/bin/market_external_agent_current_kernel.rs"
        sha256sum "$PROJECT_ROOT/src/bin/full_system_augment_current_kernel.rs"
        sha256sum "$PROJECT_ROOT/src/bin/real16_market_performance_verifier.rs"
        sha256sum "$PROJECT_ROOT/src/runtime/market_performance_e4.rs"
    } | sha256sum | awk '{print $1}'
)"

run_arm() {
    local arm="$1"
    local market_pressure_enabled="$2"
    local condition="$3"
    local arm_dir="$RUN_DIR/arm_${arm}"
    local arm_run_id="${RUN_ID}-arm-${arm}"

    {
        printf 'ARM=%s\n' "$arm"
        printf 'ARM_CONDITION=%s\n' "$condition"
        printf 'RUN_DIR=%s\n' "$arm_dir"
        printf 'MARKET_PRESSURE_ENABLED=%s\n' "$market_pressure_enabled"
        printf 'FULL_SYSTEM_PARTICIPATION_REQUIRED=1\n'
    } > "$RUN_DIR/arm_config_manifests/arm_${arm}_toggles.env"

    echo "[arm $arm] init current-kernel ChainTape"
    "$TURINGOS" init --project "$arm_dir" --template proof --provider deepseek

    echo "[arm $arm] external agent market action"
    "$MARKET_HELPER" \
        --runtime-repo "$arm_dir/runtime_repo" \
        --cas "$arm_dir/cas" \
        --run-id "$arm_run_id" \
        --constitution "$PROJECT_ROOT/constitution.md" \
        --llm-proxy-url "$LLM_PROXY_URL" \
        --model "$ACTIVE_MODEL" \
        --out "$arm_dir/external_agent_market_manifest.json"

    echo "[arm $arm] append FC3 governance/reinit participation rows"
    "$AUGMENT" \
        --runtime-repo "$arm_dir/runtime_repo" \
        --cas "$arm_dir/cas" \
        --run-id "$arm_run_id" \
        --constitution "$PROJECT_ROOT/constitution.md" \
        --out-dir "$arm_dir"

    cp "$arm_dir/runtime_repo/genesis_report.json" "$arm_dir/genesis_report.json"

    echo "[arm $arm] verify ChainTape replay"
    TURINGOS_BIN_DIR="$BIN_DIR" "$TURINGOS" verify chaintape \
        --repo "$arm_dir/runtime_repo" \
        --cas "$arm_dir/cas" \
        --run-id "$arm_run_id" \
        --out "$arm_dir/replay_report.json"

    echo "[arm $arm] audit_tape verdict"
    "$AUDIT_TAPE" \
        --runtime-repo "$arm_dir/runtime_repo" \
        --cas-dir "$arm_dir/cas" \
        --agent-pubkeys "$arm_dir/runtime_repo/agent_pubkeys.json" \
        --pinned-pubkeys "$arm_dir/runtime_repo/pinned_pubkeys.json" \
        --genesis "$arm_dir/runtime_repo/genesis_report.json" \
        --constitution "$PROJECT_ROOT/constitution.md" \
        --out "$arm_dir/aggregate_verdict.json"

    echo "[arm $arm] E2 exact-join verifier"
    set +e
    "$E2_VERIFIER" \
        --repo "$arm_dir/runtime_repo" \
        --cas "$arm_dir/cas" \
        --json-out "$arm_dir/REAL16_ARM_${arm}_E2_VERIFIER.json" \
        --md-out "$arm_dir/REAL16_ARM_${arm}_E2_VERIFIER.md"
    local e2_exit=$?
    set -e
    if [[ ! -f "$arm_dir/REAL16_ARM_${arm}_E2_VERIFIER.json" ]]; then
        echo "ERROR: E2 verifier did not write arm $arm JSON report" >&2
        exit 5
    fi
    jq --argjson exit_code "$e2_exit" \
        '. + {e2_verifier_exit_code: $exit_code, claim_boundary: "candidate-only; VETO is recorded but does not fail liveness"}' \
        "$arm_dir/REAL16_ARM_${arm}_E2_VERIFIER.json" > "$arm_dir/REAL16_ARM_${arm}_E2_VERIFIER.with_exit.json"
    mv "$arm_dir/REAL16_ARM_${arm}_E2_VERIFIER.with_exit.json" "$arm_dir/REAL16_ARM_${arm}_E2_VERIFIER.json"

    echo "[arm $arm] derive REAL16 arm metrics"
    "$REAL16_VERIFIER" \
        --derive-arm-json \
        --arm-id "$arm" \
        --evidence-dir "$arm_dir" \
        --e2-json "$arm_dir/REAL16_ARM_${arm}_E2_VERIFIER.json" \
        --problem-set-hash "$PROBLEM_SET_HASH" \
        --model-assignment-hash "$MODEL_ASSIGNMENT_HASH" \
        --budget-hash "$BUDGET_HASH" \
        --prompt-template-hash "$PROMPT_TEMPLATE_HASH" \
        --runtime-config-hash "$RUNTIME_CONFIG_HASH" \
        --market-pressure-enabled "$market_pressure_enabled" \
        --json-out "$RUN_DIR/arm_config_manifests/arm_${arm}.json"
}

run_arm A false "current-kernel baseline market-visible"
run_arm D true "current-kernel market-pressure arm"

jq -s '{arms: .}' "$RUN_DIR/arm_config_manifests"/arm_*.json > "$RUN_DIR/REAL16_VERIFIER_INPUT.json"
set +e
"$REAL16_VERIFIER" \
    --input-json "$RUN_DIR/REAL16_VERIFIER_INPUT.json" \
    --json-out "$RUN_DIR/REAL16_MARKET_PERFORMANCE_REPORT.json" \
    --md-out "$RUN_DIR/REAL16_MARKET_PERFORMANCE_REPORT.md"
REAL16_EXIT=$?
set -e
if [[ ! -f "$RUN_DIR/REAL16_MARKET_PERFORMANCE_REPORT.json" ]]; then
    echo "ERROR: REAL16 verifier did not write report" >&2
    exit 6
fi
jq --argjson exit_code "$REAL16_EXIT" \
    '. + {real16_verifier_exit_code: $exit_code, liveness_claim_boundary: "candidate-only market performance report; VETO/CleanNegative does not fail full-system liveness"}' \
    "$RUN_DIR/REAL16_MARKET_PERFORMANCE_REPORT.json" > "$RUN_DIR/REAL16_MARKET_PERFORMANCE_REPORT.with_exit.json"
mv "$RUN_DIR/REAL16_MARKET_PERFORMANCE_REPORT.with_exit.json" "$RUN_DIR/REAL16_MARKET_PERFORMANCE_REPORT.json"

"$PARTICIPATION" \
    --run-id "$RUN_ID" \
    --family-id "market_ab_performance_current_kernel" \
    --entrypoint "scripts/run_true_suite_market_ab_current_kernel.sh" \
    --runtime-repo "$RUN_DIR/arm_D/runtime_repo" \
    --cas "$RUN_DIR/arm_D/cas" \
    --replay-report "$RUN_DIR/arm_D/replay_report.json" \
    --genesis-report "$RUN_DIR/arm_D/genesis_report.json" \
    --domain-manifest "$RUN_DIR/REAL16_MARKET_PERFORMANCE_REPORT.json" \
    --fc3-index "$RUN_DIR/arm_D/governance_capsule_index.json" \
    --require-full-system \
    --out "$RUN_DIR/full_system_participation.json"

cat > "$RUN_DIR/market_ab_run_manifest.json" <<EOF
{
  "schema_version": "turingosv4.true_suite.market_ab_current_kernel_run.v1",
  "run_id": "$RUN_ID",
  "git_head": "$(cd "$PROJECT_ROOT" && git rev-parse HEAD)",
  "entrypoint": "scripts/run_true_suite_market_ab_current_kernel.sh",
  "llm_proxy_url": "$LLM_PROXY_URL",
  "active_model": "$ACTIVE_MODEL",
  "problem_set_hash": "$PROBLEM_SET_HASH",
  "model_assignment_hash": "$MODEL_ASSIGNMENT_HASH",
  "budget_hash": "$BUDGET_HASH",
  "prompt_template_hash": "$PROMPT_TEMPLATE_HASH",
  "runtime_config_hash": "$RUNTIME_CONFIG_HASH",
  "arm_a": "$RUN_DIR/arm_A",
  "arm_d": "$RUN_DIR/arm_D",
  "real16_report": "$RUN_DIR/REAL16_MARKET_PERFORMANCE_REPORT.json",
  "real16_verifier_exit_code": $REAL16_EXIT,
  "full_system_participation": "$RUN_DIR/full_system_participation.json",
  "notes": [
    "candidate-only market performance report; no E4 achieved claim",
    "both arms are fresh current-kernel ChainTape/CAS runs",
    "root full_system_participation uses arm D as the market-pressure representative sample"
  ]
}
EOF

echo "TRUE-SUITE market A/B current-kernel evidence: $RUN_DIR"
