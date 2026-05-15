#!/usr/bin/env bash
# REAL-8 — Formal Market A/B Benchmark.
#
# Architect conditions:
#   A: market disabled
#   B: market visible, no TaskOutcomeMarket
#   C: TaskOutcomeMarket enabled
#   D: TaskOutcomeMarket + scripted AttemptPrediction fixture
#
# This runner is descriptive evidence only. It pins the same problem set, model
# assignment, and budgets across all arms, writes chain-backed arm evidence via
# scripts/run_g_phase_batch.sh, and emits a report that explicitly forbids
# causal overclaim. Negative results are valid and documented.

set -uo pipefail

usage() {
    cat <<'USAGE'
usage: scripts/run_real8_market_ab_benchmark.sh \
  --problems <same_problem_set_manifest> \
  --models <same_model_assignment_manifest> \
  --budgets <same_budget_manifest> \
  [--tasks-per-arm <N>] \
  --arms A,B,C,D \
  --out handover/evidence/real8_market_ab_<UTC>

Model manifest format (KEY=VALUE, comments allowed):
  ACTIVE_MODEL=deepseek-chat
  AGENT_MODELS=
  PHASE_D_HETERO_OK=1
  TURINGOS_REAL5_ROLE_ASSIGNMENT=Solver,Trader,Verifier,Challenger,Observer
  TURINGOS_G_PHASE_N_AGENTS=5

Budget manifest format (KEY=VALUE, comments allowed):
  MAX_TRANSACTIONS=5
  PER_PROBLEM_TIMEOUT_S=300
  TURINGOS_REAL6A_POLL_BUDGET_MS=30000
USAGE
}

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
EVIDENCE_ROOT="$PROJECT_ROOT/handover/evidence"

PROBLEMS=""
MODELS=""
BUDGETS=""
ARMS="A,B,C,D"
OUT=""
TASKS_PER_ARM=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        --problems) PROBLEMS="${2:-}"; shift 2 ;;
        --models) MODELS="${2:-}"; shift 2 ;;
        --budgets) BUDGETS="${2:-}"; shift 2 ;;
        --tasks-per-arm) TASKS_PER_ARM="${2:-}"; shift 2 ;;
        --arms) ARMS="${2:-}"; shift 2 ;;
        --out) OUT="${2:-}"; shift 2 ;;
        -h|--help) usage; exit 0 ;;
        *) echo "ERROR: unknown arg: $1" >&2; usage >&2; exit 2 ;;
    esac
done

[[ -n "$PROBLEMS" && -f "$PROBLEMS" ]] || { echo "ERROR: --problems file required" >&2; exit 2; }
[[ -n "$MODELS" && -f "$MODELS" ]] || { echo "ERROR: --models file required" >&2; exit 2; }
[[ -n "$BUDGETS" && -f "$BUDGETS" ]] || { echo "ERROR: --budgets file required" >&2; exit 2; }
[[ -n "$OUT" ]] || { echo "ERROR: --out required" >&2; exit 2; }

case "$OUT" in
    "$PROJECT_ROOT"/*) OUT_ABS="$OUT" ;;
    /*) OUT_ABS="$OUT" ;;
    *) OUT_ABS="$PROJECT_ROOT/$OUT" ;;
esac

mkdir -p "$OUT_ABS"
cp "$PROBLEMS" "$OUT_ABS/problems.pinned.txt"
cp "$MODELS" "$OUT_ABS/model_assignment.pinned.env"
cp "$BUDGETS" "$OUT_ABS/budgets.pinned.env"

PROBLEMS_HASH="$(sha256sum "$OUT_ABS/problems.pinned.txt" | awk '{print $1}')"
MODELS_HASH="$(sha256sum "$OUT_ABS/model_assignment.pinned.env" | awk '{print $1}')"
BUDGETS_HASH="$(sha256sum "$OUT_ABS/budgets.pinned.env" | awk '{print $1}')"
PROBLEM_COUNT="$(grep -Ev '^[[:space:]]*(#|$)' "$OUT_ABS/problems.pinned.txt" | wc -l | tr -d ' ')"
if [[ "$PROBLEM_COUNT" -eq 0 ]]; then
    echo "ERROR: pinned problem manifest is empty: $PROBLEMS" >&2
    exit 2
fi
if [[ "${TURINGOS_REAL6B_LIVE_ATTEMPT_PREDICTION:-0}" == "1" ]]; then
    echo "ERROR: live REAL-6B AttemptPrediction is not ratified for REAL-8/REAL-10; use the scripted fixture only" >&2
    exit 2
fi
if [[ -n "$TASKS_PER_ARM" && "$PROBLEM_COUNT" != "$TASKS_PER_ARM" ]]; then
    echo "ERROR: --tasks-per-arm=$TASKS_PER_ARM but pinned problem manifest has $PROBLEM_COUNT tasks" >&2
    exit 2
fi

read_manifest_value() {
    local file="$1"
    local key="$2"
    awk -F= -v k="$key" '
        /^[[:space:]]*#/ { next }
        /^[[:space:]]*$/ { next }
        {
            gsub(/^[[:space:]]+|[[:space:]]+$/, "", $1)
            if ($1 == k) {
                v = substr($0, index($0, "=") + 1)
                gsub(/^[[:space:]]+|[[:space:]]+$/, "", v)
                print v
                exit
            }
        }
    ' "$file"
}

ACTIVE_MODEL_PIN="$(read_manifest_value "$MODELS" ACTIVE_MODEL)"
AGENT_MODELS_PIN="$(read_manifest_value "$MODELS" AGENT_MODELS)"
PHASE_D_HETERO_OK_PIN="$(read_manifest_value "$MODELS" PHASE_D_HETERO_OK)"
ROLE_ASSIGNMENT_PIN="$(read_manifest_value "$MODELS" TURINGOS_REAL5_ROLE_ASSIGNMENT)"
N_AGENTS_PIN="$(read_manifest_value "$MODELS" TURINGOS_G_PHASE_N_AGENTS)"
MAX_TX_PIN="$(read_manifest_value "$BUDGETS" MAX_TRANSACTIONS)"
TIMEOUT_PIN="$(read_manifest_value "$BUDGETS" PER_PROBLEM_TIMEOUT_S)"
REAL6A_POLL_PIN="$(read_manifest_value "$BUDGETS" TURINGOS_REAL6A_POLL_BUDGET_MS)"

ACTIVE_MODEL_PIN="${ACTIVE_MODEL_PIN:-deepseek-chat}"
PHASE_D_HETERO_OK_PIN="${PHASE_D_HETERO_OK_PIN:-1}"
ROLE_ASSIGNMENT_PIN="${ROLE_ASSIGNMENT_PIN:-Solver,Trader,Verifier,Challenger,Observer}"
N_AGENTS_PIN="${N_AGENTS_PIN:-5}"
MAX_TX_PIN="${MAX_TX_PIN:-5}"
TIMEOUT_PIN="${TIMEOUT_PIN:-300}"
REAL6A_POLL_PIN="${REAL6A_POLL_PIN:-30000}"

if [[ "$OUT_ABS" == "$EVIDENCE_ROOT"/* ]]; then
    RUN_TAG_PREFIX="${OUT_ABS#$EVIDENCE_ROOT/}"
else
    RUN_TAG_PREFIX="$(basename "$OUT_ABS")"
fi

REPORT="$OUT_ABS/REAL8_MARKET_AB_BENCHMARK_REPORT.md"
SUMMARY_TSV="$OUT_ABS/real8_arm_summary.tsv"
ARM_CONFIG_DIR="$OUT_ABS/arm_config_manifests"
REAL8X_SEED="${REAL8X_SEED:-real8x-fixed-seed-v1}"
mkdir -p "$ARM_CONFIG_DIR"

cat > "$ARM_CONFIG_DIR/arm_diff_allowlist.txt" <<'EOF'
# REAL-10 SG-10.4.4 — only these keys may differ across arm config manifests.
ARM
ARM_CONDITION
RUN_TAG
RUN_DIR
TURINGOS_DISABLE_MARKET_TOOLS
TURINGOS_TB_N3_AUTO_MARKET
TURINGOS_REAL6_TASK_OUTCOME_MARKET
TURINGOS_REAL7_SCRIPTED_ATTEMPT_PREDICTION_FIXTURE
EOF

cat > "$ARM_CONFIG_DIR/REAL8X_SHARED_CONFIG.env" <<EOF
PROBLEMS_HASH=$PROBLEMS_HASH
MODELS_HASH=$MODELS_HASH
BUDGETS_HASH=$BUDGETS_HASH
PROBLEM_COUNT=$PROBLEM_COUNT
TASKS_PER_ARM=${TASKS_PER_ARM:-$PROBLEM_COUNT}
REAL8X_SEED=$REAL8X_SEED
ACTIVE_MODEL=$ACTIVE_MODEL_PIN
AGENT_MODELS=$AGENT_MODELS_PIN
PHASE_D_HETERO_OK=$PHASE_D_HETERO_OK_PIN
TURINGOS_REAL5_ROLE_ASSIGNMENT=$ROLE_ASSIGNMENT_PIN
TURINGOS_G_PHASE_N_AGENTS=$N_AGENTS_PIN
MAX_TRANSACTIONS=$MAX_TX_PIN
PER_PROBLEM_TIMEOUT_S=$TIMEOUT_PIN
TURINGOS_REAL6A_POLL_BUDGET_MS=$REAL6A_POLL_PIN
TURINGOS_REAL6_SCHEDULER_OBSERVE_ONLY=1
EOF
REAL8X_SHARED_CONFIG_SHA256="$(sha256sum "$ARM_CONFIG_DIR/REAL8X_SHARED_CONFIG.env" | awk '{print $1}')"
printf "%s\n" "$REAL8X_SHARED_CONFIG_SHA256" > "$ARM_CONFIG_DIR/REAL8X_SHARED_CONFIG.env.sha256"

cat > "$REPORT" <<EOF
# REAL-8 Formal Market A/B Benchmark

This report is descriptive benchmark evidence only. It does not claim causality.
Negative result is valid and documented.

## Pinned Inputs

| Pin | SHA-256 |
| --- | --- |
| same problem set | \`$PROBLEMS_HASH\` |
| same model assignment | \`$MODELS_HASH\` |
| same budgets | \`$BUDGETS_HASH\` |
| same seed/config except arm toggles | \`$REAL8X_SHARED_CONFIG_SHA256\` |
| tasks per arm | \`${TASKS_PER_ARM:-$PROBLEM_COUNT}\` |

Forbidden claim boundary:

\`\`\`text
no forced trades
no price-as-truth
no ghost liquidity
no f64 economy
no off-tape WAL as truth
no private CoT recording
no raw-log broadcast
\`\`\`

## Arms

| Arm | Condition |
| --- | --- |
| A | market disabled |
| B | market visible, no TaskOutcomeMarket |
| C | TaskOutcomeMarket enabled |
| D | TaskOutcomeMarket + scripted AttemptPrediction fixture |

## Metrics

| Arm | exit | audit | tasks | solve_rate | wilson_ci_95 | verified_pput_mean | mean_pput_solved | false_accept_rate_mean | cost_per_verified_proof_tokens | cost_time_tokens_ms | market_tx_count | no_trade_reason_distribution | pnl_dispersion_micro | role_diversity_index | failed_branch_count | verification_latency_ms_mean | wasted_attempts | audit_failure_rate |
| --- | ---: | --- | ---: | --- | --- | ---: | ---: | ---: | --- | --- | ---: | --- | --- | ---: | ---: | ---: | ---: | ---: |
EOF

printf "arm\trun_dir\texit_code\taudit_verdict\ttask_count\tsolve_rate\twilson_ci_95\tmarket_tx_count\tfailed_branch_count\tverification_latency_ms_mean\twasted_attempts\n" > "$SUMMARY_TSV"

arm_condition() {
    case "$1" in
        A) echo "market disabled" ;;
        B) echo "market visible, no TaskOutcomeMarket" ;;
        C) echo "TaskOutcomeMarket enabled" ;;
        D) echo "TaskOutcomeMarket + scripted AttemptPrediction fixture" ;;
        *) echo "unknown" ;;
    esac
}

extract_pput_metrics() {
    local run_dir="$1"
    find "$run_dir" -path '*/evaluator.stdout' -type f -print0 \
      | xargs -0 sed -n 's/^PPUT_RESULT://p' \
      | jq -s '{
          n: length,
          solved: ([.[] | select(.solved == true)] | length),
          verified: ([.[] | select(.verified == true)] | length),
          total_tokens: ([.[] | .total_run_token_count // 0] | add // 0),
          total_wall_time_ms: ([.[] | .total_wall_time_ms // 0] | add // 0),
          failed_branch_count: ([.[] | .failed_branch_count // 0] | add // 0),
          verification_latency_ms_mean: (if length == 0 then 0 else ([.[] | .verifier_wait_ms // 0] | add // 0) / length end),
          wasted_attempts: ([.[] | select(.solved != true) | .tx_count // .failed_branch_count // 0] | add // 0),
          pput_verified_mean: (if length == 0 then 0 else ([.[] | .pput_verified // 0] | add // 0) / length end),
          mean_pput_solved: (if ([.[] | select(.solved == true)] | length) == 0 then 0 else ([.[] | select(.solved == true) | .pput_verified // .pput // 0] | add // 0) / ([.[] | select(.solved == true)] | length) end),
          false_accept_rate_mean: (if length == 0 then 0 else ([.[] | .far // 0] | add // 0) / length end),
          no_trade: ([.[] | .tool_dist // {} | to_entries[]? | select(.key | startswith("invest_no_trade_")) | "\(.key)=\(.value)"] | join(";"))
        }'
}

wilson_ci_95() {
    python3 - "$1" "$2" <<'PY'
import math
import sys
success = int(sys.argv[1])
total = int(sys.argv[2])
if total <= 0:
    print("0..0")
    raise SystemExit
z = 1.96
phat = success / total
den = 1 + z * z / total
center = (phat + z * z / (2 * total)) / den
margin = z * math.sqrt((phat * (1 - phat) + z * z / (4 * total)) / total) / den
print(f"{max(0.0, center - margin):.4f}..{min(1.0, center + margin):.4f}")
PY
}

write_arm_config_manifest() {
    local arm="$1"
    local condition="$2"
    local run_tag="$3"
    local run_dir="$4"
    local shared="$ARM_CONFIG_DIR/arm_${arm}_shared.env"
    local toggles="$ARM_CONFIG_DIR/arm_${arm}_toggles.env"
    local merged="$ARM_CONFIG_DIR/arm_${arm}_config.env"
    cat > "$shared" <<EOF
PROBLEMS_HASH=$PROBLEMS_HASH
MODELS_HASH=$MODELS_HASH
BUDGETS_HASH=$BUDGETS_HASH
PROBLEM_COUNT=$PROBLEM_COUNT
TASKS_PER_ARM=${TASKS_PER_ARM:-$PROBLEM_COUNT}
ACTIVE_MODEL=$ACTIVE_MODEL_PIN
AGENT_MODELS=$AGENT_MODELS_PIN
PHASE_D_HETERO_OK=$PHASE_D_HETERO_OK_PIN
TURINGOS_REAL5_ROLE_ASSIGNMENT=$ROLE_ASSIGNMENT_PIN
TURINGOS_G_PHASE_N_AGENTS=$N_AGENTS_PIN
REAL8X_SEED=$REAL8X_SEED
MAX_TRANSACTIONS=$MAX_TX_PIN
PER_PROBLEM_TIMEOUT_S=$TIMEOUT_PIN
TURINGOS_REAL6A_POLL_BUDGET_MS=$REAL6A_POLL_PIN
TURINGOS_REAL6_SCHEDULER_OBSERVE_ONLY=1
EOF
    cat > "$toggles" <<EOF
ARM=$arm
ARM_CONDITION=$condition
RUN_TAG=$run_tag
RUN_DIR=$run_dir
TURINGOS_DISABLE_MARKET_TOOLS=${TURINGOS_DISABLE_MARKET_TOOLS:-}
TURINGOS_TB_N3_AUTO_MARKET=${TURINGOS_TB_N3_AUTO_MARKET:-}
TURINGOS_REAL6_TASK_OUTCOME_MARKET=${TURINGOS_REAL6_TASK_OUTCOME_MARKET:-}
TURINGOS_REAL7_SCRIPTED_ATTEMPT_PREDICTION_FIXTURE=${TURINGOS_REAL7_SCRIPTED_ATTEMPT_PREDICTION_FIXTURE:-}
EOF
    cat "$shared" "$toggles" > "$merged"
    sha256sum "$shared" | awk '{print $1}' > "$shared.sha256"
}

validate_arm_config_diffs() {
    # Emits per-arm `arm_config_diff_from_A.tsv`-style evidence files:
    # `arm_A_config_diff_from_A.tsv`, `arm_B_config_diff_from_A.tsv`, ...
    python3 - "$ARM_CONFIG_DIR" <<'PY'
import json
import sys
from pathlib import Path

root = Path(sys.argv[1])
allowed = set(
    line.strip()
    for line in (root / "arm_diff_allowlist.txt").read_text().splitlines()
    if line.strip() and not line.lstrip().startswith("#")
)

def parse_env(path: Path) -> dict[str, str]:
    out = {}
    for raw in path.read_text().splitlines():
        line = raw.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        out[key] = value
    return out

configs = sorted(root.glob("arm_*_config.env"))
base_path = root / "arm_A_config.env"
if not base_path.exists() and configs:
    base_path = configs[0]
base = parse_env(base_path) if base_path.exists() else {}
disallowed_config_drift = []
diff_summary = {}

for config in configs:
    arm = config.name.removeprefix("arm_").removesuffix("_config.env")
    current = parse_env(config)
    keys = sorted(set(base) | set(current))
    rows = []
    for key in keys:
        left = base.get(key, "")
        right = current.get(key, "")
        if left == right:
            continue
        status = "ALLOW" if key in allowed else "BLOCK"
        rows.append((key, left, right, status))
        if status == "BLOCK":
            disallowed_config_drift.append(
                {"arm": arm, "key": key, "base": left, "current": right}
            )
    diff_summary[arm] = len(rows)
    with (root / f"arm_{arm}_config_diff_from_A.tsv").open("w") as fh:
        fh.write("key\tarm_A\tcurrent_arm\tstatus\n")
        for key, left, right, status in rows:
            fh.write(f"{key}\t{left}\t{right}\t{status}\n")

audit = {
    "base_config": str(base_path),
    "allowlist": sorted(allowed),
    "diff_row_count_by_arm": diff_summary,
    "disallowed_config_drift": disallowed_config_drift,
}
(root / "REAL8X_CONFIG_AUDIT.json").write_text(json.dumps(audit, indent=2, sort_keys=True) + "\n")
if disallowed_config_drift:
    print("ERROR: Only arm toggles may differ; disallowed_config_drift found", file=sys.stderr)
    for item in disallowed_config_drift:
        print(item, file=sys.stderr)
    raise SystemExit(1)
PY
}

metric_from_dashboard() {
    local dashboard="$1"
    local key="$2"
    awk -F': ' -v k="$key" '$1 ~ k { gsub(/^[[:space:]]+|[[:space:]]+$/, "", $2); print $2; exit }' "$dashboard"
}

pnl_dispersion_from_dashboard() {
    local dashboard="$1"
    awk '
        /realized=/ {
            if (match($0, /realized=-?[0-9]+/)) {
                v = substr($0, RSTART + 9, RLENGTH - 9) + 0
                if (!seen || v < min) min = v
                if (!seen || v > max) max = v
                seen = 1
            }
        }
        END {
            if (!seen) print "0"
            else print min ".." max
        }
    ' "$dashboard"
}

ARM_FAILURES=0
IFS=',' read -r -a ARM_LIST <<< "$ARMS"
for arm_raw in "${ARM_LIST[@]}"; do
    arm="$(printf '%s' "$arm_raw" | xargs)"
    [[ -n "$arm" ]] || continue
    case "$arm" in A|B|C|D) ;; *) echo "ERROR: unsupported arm: $arm" >&2; exit 2 ;; esac

    run_tag="${RUN_TAG_PREFIX}/arm_${arm}"
    run_dir="$EVIDENCE_ROOT/$run_tag"
    dashboard="$run_dir/audit_dashboard_run_report.txt"

    echo "[real8] running arm $arm: $(arm_condition "$arm")"

    export ACTIVE_MODEL="$ACTIVE_MODEL_PIN"
    export PHASE_D_HETERO_OK="$PHASE_D_HETERO_OK_PIN"
    export TURINGOS_G_PHASE_N_AGENTS="$N_AGENTS_PIN"
    export TURINGOS_REAL5_ROLE_ASSIGNMENT="$ROLE_ASSIGNMENT_PIN"
    export TURINGOS_REAL5_ROLE_VIEWS=1
    export TURINGOS_G_PHASE_DIRTY_OK=1
    export TURINGOS_G_PHASE_LOW_DISK_OK=1
    export MAX_TRANSACTIONS="$MAX_TX_PIN"
    export PER_PROBLEM_TIMEOUT_S="$TIMEOUT_PIN"
    export TURINGOS_REAL6A_POLL_BUDGET_MS="$REAL6A_POLL_PIN"
    export TURINGOS_REAL6_SCHEDULER_OBSERVE_ONLY=1
    if [[ -n "$AGENT_MODELS_PIN" ]]; then
        export AGENT_MODELS="$AGENT_MODELS_PIN"
    else
        unset AGENT_MODELS
    fi

    unset TURINGOS_DISABLE_MARKET_TOOLS
    unset TURINGOS_REAL6_TASK_OUTCOME_MARKET
    unset TURINGOS_REAL7_SCRIPTED_ATTEMPT_PREDICTION_FIXTURE
    unset TURINGOS_REAL7_SCRIPTED_TASK_OUTCOME_BUYS
    unset TURINGOS_FORCE_BOLTZMANN_SEED_WORKTXS
    unset TURINGOS_REAL7_SCRIPTED_VERIFY_CHALLENGE

    case "$arm" in
        A)
            export TURINGOS_DISABLE_MARKET_TOOLS=1
            export TURINGOS_TB_N3_AUTO_MARKET=0
            ;;
        B)
            export TURINGOS_TB_N3_AUTO_MARKET=1
            ;;
        C)
            export TURINGOS_TB_N3_AUTO_MARKET=1
            export TURINGOS_REAL6_TASK_OUTCOME_MARKET=1
            ;;
        D)
            export TURINGOS_TB_N3_AUTO_MARKET=1
            export TURINGOS_REAL6_TASK_OUTCOME_MARKET=1
            export TURINGOS_REAL7_SCRIPTED_ATTEMPT_PREDICTION_FIXTURE=1
            ;;
    esac

    write_arm_config_manifest "$arm" "$(arm_condition "$arm")" "$run_tag" "$run_dir"

    bash "$PROJECT_ROOT/scripts/run_g_phase_batch.sh" "$run_tag" "$OUT_ABS/problems.pinned.txt"
    exit_code=$?
    if [[ "$exit_code" -ne 0 ]]; then
        ARM_FAILURES=$((ARM_FAILURES + 1))
    fi

    audit_verdict="$(jq -r '.verdict // "missing"' "$run_dir/aggregate_verdict.json" 2>/dev/null || echo missing)"
    if [[ "$audit_verdict" != "PROCEED" ]]; then
        ARM_FAILURES=$((ARM_FAILURES + 1))
    fi

    cargo run --quiet --bin audit_dashboard -- --repo "$run_dir/runtime_repo" --cas "$run_dir/cas" --run-report \
        > "$dashboard" || ARM_FAILURES=$((ARM_FAILURES + 1))

    pput_json="$(extract_pput_metrics "$run_dir")"
    tasks="$(jq -r '.n // 0' <<< "$pput_json")"
    solved="$(jq -r '.solved // 0' <<< "$pput_json")"
    verified="$(jq -r '.verified // 0' <<< "$pput_json")"
    total_tokens="$(jq -r '.total_tokens // 0' <<< "$pput_json")"
    total_wall_time_ms="$(jq -r '.total_wall_time_ms // 0' <<< "$pput_json")"
    failed_branch_count="$(jq -r '.failed_branch_count // 0' <<< "$pput_json")"
    verification_latency_ms_mean="$(jq -r '.verification_latency_ms_mean // 0' <<< "$pput_json")"
    wasted_attempts="$(jq -r '.wasted_attempts // 0' <<< "$pput_json")"
    pput_mean="$(jq -r '.pput_verified_mean // 0' <<< "$pput_json")"
    mean_pput_solved="$(jq -r '.mean_pput_solved // 0' <<< "$pput_json")"
    far_mean="$(jq -r '.false_accept_rate_mean // 0' <<< "$pput_json")"
    no_trade="$(jq -r '.no_trade // ""' <<< "$pput_json")"
    [[ -n "$no_trade" ]] || no_trade="none_observed"
    if [[ "$verified" -gt 0 ]]; then
        cost_per_verified="$((total_tokens / verified))"
    else
        cost_per_verified="undefined_no_verified_proof"
    fi

    market_seed="$(jq -r '.tx_kind_counts.market_seed // 0' "$run_dir/aggregate_verdict.json" 2>/dev/null || echo 0)"
    cpmm_pool="$(jq -r '.tx_kind_counts.cpmm_pool // 0' "$run_dir/aggregate_verdict.json" 2>/dev/null || echo 0)"
    cpmm_swap="$(jq -r '.tx_kind_counts.cpmm_swap // 0' "$run_dir/aggregate_verdict.json" 2>/dev/null || echo 0)"
    router="$(jq -r '.tx_kind_counts.buy_with_coin_router // 0' "$run_dir/aggregate_verdict.json" 2>/dev/null || echo 0)"
    market_tx_count="$((market_seed + cpmm_pool + cpmm_swap + router))"

    active_roles="$(metric_from_dashboard "$dashboard" "active_role_count")"
    active_roles="${active_roles:-0}"
    pnl_dispersion="$(pnl_dispersion_from_dashboard "$dashboard")"
    if [[ "$audit_verdict" == "PROCEED" ]]; then
        audit_failure_rate=0
    else
        audit_failure_rate=1
    fi
    if [[ "$tasks" -gt 0 ]]; then
        solve_rate="${solved}/${tasks}"
        wilson_ci="$(wilson_ci_95 "$solved" "$tasks")"
    else
        solve_rate="0/0"
        wilson_ci="0..0"
    fi
    cost_time="${total_tokens}/${total_wall_time_ms}ms"

    printf "| %s | %s | %s | %s | %s | %s | %s | %s | %s | %s | %s | %s | %s | %s | %s | %s | %s | %s | %s |\n" \
        "$arm" "$exit_code" "$audit_verdict" "$tasks" "$solve_rate" "$wilson_ci" "$pput_mean" \
        "$mean_pput_solved" "$far_mean" "$cost_per_verified" "$cost_time" "$market_tx_count" \
        "$no_trade" "$pnl_dispersion" "$active_roles" "$failed_branch_count" \
        "$verification_latency_ms_mean" "$wasted_attempts" "$audit_failure_rate" \
        >> "$REPORT"
    printf "%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n" \
        "$arm" "$run_dir" "$exit_code" "$audit_verdict" "$tasks" "$solve_rate" "$wilson_ci" \
        "$market_tx_count" "$failed_branch_count" "$verification_latency_ms_mean" "$wasted_attempts" >> "$SUMMARY_TSV"
done

shared_hash_count="$(find "$ARM_CONFIG_DIR" -name 'arm_*_shared.env.sha256' -type f -exec cat {} + | sort -u | wc -l | tr -d ' ')"
if [[ "$shared_hash_count" != "1" ]]; then
    echo "ERROR: per-arm shared config hashes differ; only arm toggles may differ" >&2
    ARM_FAILURES=$((ARM_FAILURES + 1))
fi
if ! validate_arm_config_diffs; then
    ARM_FAILURES=$((ARM_FAILURES + 1))
fi

cat >> "$REPORT" <<'EOF'

## Gate Verdicts

```text
SG-8.1 Same problem set across arms: PASS (single pinned problem manifest hash above).
SG-8.2 Same model assignment: PASS (single pinned model manifest hash above).
SG-8.3 Same budgets: PASS (single pinned budget manifest hash above).
SG-8.4 Same seed/config except arm toggles: PASS iff arm shared-config hashes match and arm toggles are allowlisted.
SG-8.5 All runs chain-backed: PASS iff every arm exit=0 and audit=PROCEED.
SG-8.6 No overclaim of causality: PASS (this report is descriptive evidence only).
SG-8.7 Negative result is valid and documented: PASS (undefined/no-effect metrics are retained, not rewritten).
```

## Claim Boundary

REAL-8 does not claim that a market arm caused higher solve rate, higher PPUT,
role differentiation, or spontaneous trading. It reports chain-backed
observations under pinned A/B/C/D conditions. A negative result is a valid
scientific result and must remain in the handover evidence.
EOF

if [[ "$ARM_FAILURES" -ne 0 ]]; then
    echo "REAL-8 benchmark completed with $ARM_FAILURES arm/audit/dashboard failures; see $REPORT" >&2
    exit 1
fi

echo "REAL-8 benchmark PASS: $REPORT"
