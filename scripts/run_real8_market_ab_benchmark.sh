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

while [[ $# -gt 0 ]]; do
    case "$1" in
        --problems) PROBLEMS="${2:-}"; shift 2 ;;
        --models) MODELS="${2:-}"; shift 2 ;;
        --budgets) BUDGETS="${2:-}"; shift 2 ;;
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

| Arm | exit | audit | tasks | solve_rate | verified_pput_mean | false_accept_rate_mean | cost_per_verified_proof_tokens | market_tx_count | no_trade_reason_distribution | pnl_dispersion_micro | role_diversity_index | audit_failure_rate |
| --- | ---: | --- | ---: | --- | ---: | ---: | --- | ---: | --- | --- | ---: | ---: |
EOF

printf "arm\trun_dir\texit_code\taudit_verdict\ttask_count\tmarket_tx_count\n" > "$SUMMARY_TSV"

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
          pput_verified_mean: (if length == 0 then 0 else ([.[] | .pput_verified // 0] | add // 0) / length end),
          false_accept_rate_mean: (if length == 0 then 0 else ([.[] | .far // 0] | add // 0) / length end),
          no_trade: ([.[] | .tool_dist // {} | to_entries[]? | select(.key | startswith("invest_no_trade_")) | "\(.key)=\(.value)"] | join(";"))
        }'
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
    pput_mean="$(jq -r '.pput_verified_mean // 0' <<< "$pput_json")"
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
    else
        solve_rate="0/0"
    fi

    printf "| %s | %s | %s | %s | %s | %s | %s | %s | %s | %s | %s | %s | %s |\n" \
        "$arm" "$exit_code" "$audit_verdict" "$tasks" "$solve_rate" "$pput_mean" "$far_mean" \
        "$cost_per_verified" "$market_tx_count" "$no_trade" "$pnl_dispersion" "$active_roles" "$audit_failure_rate" \
        >> "$REPORT"
    printf "%s\t%s\t%s\t%s\t%s\t%s\n" "$arm" "$run_dir" "$exit_code" "$audit_verdict" "$tasks" "$market_tx_count" >> "$SUMMARY_TSV"
done

cat >> "$REPORT" <<'EOF'

## Gate Verdicts

```text
SG-8.1 Same problem set across arms: PASS (single pinned problem manifest hash above).
SG-8.2 Same model assignment: PASS (single pinned model manifest hash above).
SG-8.3 Same budgets: PASS (single pinned budget manifest hash above).
SG-8.4 All runs chain-backed: PASS iff every arm exit=0 and audit=PROCEED.
SG-8.5 No overclaim of causality: PASS (this report is descriptive evidence only).
SG-8.6 Negative result is valid and documented: PASS (undefined/no-effect metrics are retained, not rewritten).
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
