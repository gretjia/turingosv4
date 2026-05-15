#!/usr/bin/env bash
# REAL-11 Atom 2 — Router positive-control evidence runner.
#
# This is a scripted fixture runner. It proves the BuyWithCoinRouter substrate
# can route accepted and rejected economic actions through L4/L4.E, but it is
# not E2. E2 requires live, non-scripted, agent-generated router/short action.

set -euo pipefail

usage() {
    cat <<'USAGE'
usage: scripts/run_real11_router_positive_control.sh [--out handover/evidence/real11_router_positive_control_<UTC>]

Always writes an evidence directory containing:
  manifest.json
  aggregate_verdict.json
  REAL11_ROUTER_POSITIVE_CONTROL_VERDICT.json
  REAL11_ROUTER_POSITIVE_CONTROL_REPORT.md
  cargo_test.stdout
  cargo_test.stderr
  runtime_repo/
  cas/
  audit_dashboard_run_report.txt

There is intentionally no evidence opt-out mode.
USAGE
}

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
EVIDENCE_ROOT="$PROJECT_ROOT/handover/evidence"
OUT=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        --out) OUT="${2:-}"; shift 2 ;;
        -h|--help) usage; exit 0 ;;
        *) echo "ERROR: unknown arg: $1" >&2; usage >&2; exit 2 ;;
    esac
done

if [[ -z "$OUT" ]]; then
    OUT="$EVIDENCE_ROOT/real11_router_positive_control_$(date -u +%Y%m%dT%H%M%SZ)"
elif [[ "$OUT" != /* ]]; then
    OUT="$PROJECT_ROOT/$OUT"
fi

case "$OUT" in
    "$EVIDENCE_ROOT"/real11_router_positive_control_*) ;;
    *)
        echo "ERROR: --out must be under handover/evidence/real11_router_positive_control_<UTC>" >&2
        exit 2
        ;;
esac

mkdir -p "$OUT"

COMMAND=(cargo test --test constitution_real11_router_positive_control -- --test-threads=1)
STARTED_AT="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
set +e
(
    cd "$PROJECT_ROOT"
    "${COMMAND[@]}"
) > "$OUT/cargo_test.stdout" 2> "$OUT/cargo_test.stderr"
TEST_EXIT=$?
set -e

RUNTIME_EXIT=0
DASHBOARD_EXIT=0
AUDIT_VERDICT="not_run"
BUY_WITH_COIN_ROUTER=0
MARKET_SEED=0
CPMM_POOL=0
if [[ "$TEST_EXIT" -eq 0 ]]; then
    set +e
    (
        cd "$PROJECT_ROOT"
        TURINGOS_TB_N3_AUTO_MARKET=1 \
        TURINGOS_REAL6_TASK_OUTCOME_MARKET=1 \
        TURINGOS_REAL7_SCRIPTED_TASK_OUTCOME_BUYS="Agent_1:Agent_2:1000" \
        TURINGOS_REAL7_SCRIPTED_ATTEMPT_PREDICTION_FIXTURE=0 \
        TURINGOS_REAL6B_LIVE_ATTEMPT_PREDICTION=0 \
        TURINGOS_REAL5_ROLE_ASSIGNMENT="${TURINGOS_REAL5_ROLE_ASSIGNMENT:-Solver,Trader,Verifier,Challenger,Observer}" \
        TURINGOS_REAL5_ROLE_VIEWS=1 \
        TURINGOS_G_PHASE_N_AGENTS="${TURINGOS_G_PHASE_N_AGENTS:-5}" \
        MAX_TRANSACTIONS="${MAX_TRANSACTIONS:-5}" \
        PER_PROBLEM_TIMEOUT_S="${PER_PROBLEM_TIMEOUT_S:-300}" \
        TURINGOS_G_PHASE_DIRTY_OK="${TURINGOS_G_PHASE_DIRTY_OK:-1}" \
        TURINGOS_G_PHASE_LOW_DISK_OK="${TURINGOS_G_PHASE_LOW_DISK_OK:-1}" \
        bash scripts/run_g_phase_batch.sh "$(basename "$OUT")" mini
    ) > "$OUT/runtime_batch.stdout" 2> "$OUT/runtime_batch.stderr"
    RUNTIME_EXIT=$?
    if [[ "$RUNTIME_EXIT" -eq 0 ]]; then
        (
            cd "$PROJECT_ROOT"
            cargo run --quiet --bin audit_dashboard -- --repo "$OUT/runtime_repo" --cas "$OUT/cas" --run-report
        ) > "$OUT/audit_dashboard_run_report.txt" 2> "$OUT/audit_dashboard.stderr"
        DASHBOARD_EXIT=$?
    fi
    set -e
    if [[ -f "$OUT/aggregate_verdict.json" ]]; then
        AUDIT_VERDICT="$(jq -r '.verdict // "missing"' "$OUT/aggregate_verdict.json" 2>/dev/null || echo missing)"
        BUY_WITH_COIN_ROUTER="$(jq -r '.tx_kind_counts.buy_with_coin_router // 0' "$OUT/aggregate_verdict.json" 2>/dev/null || echo 0)"
        MARKET_SEED="$(jq -r '.tx_kind_counts.market_seed // 0' "$OUT/aggregate_verdict.json" 2>/dev/null || echo 0)"
        CPMM_POOL="$(jq -r '.tx_kind_counts.cpmm_pool // 0' "$OUT/aggregate_verdict.json" 2>/dev/null || echo 0)"
    fi
fi
FINISHED_AT="$(date -u +%Y-%m-%dT%H:%M:%SZ)"

REPORT="$OUT/REAL11_ROUTER_POSITIVE_CONTROL_REPORT.md"
ROOT_REPORT="$PROJECT_ROOT/handover/reports/REAL11_ROUTER_POSITIVE_CONTROL_REPORT.md"
MANIFEST="$OUT/manifest.json"
AGGREGATE="$OUT/REAL11_ROUTER_POSITIVE_CONTROL_VERDICT.json"

python3 - "$MANIFEST" "$AGGREGATE" "$OUT" "$STARTED_AT" "$FINISHED_AT" "$TEST_EXIT" "$RUNTIME_EXIT" "$DASHBOARD_EXIT" "$AUDIT_VERDICT" "$BUY_WITH_COIN_ROUTER" "$MARKET_SEED" "$CPMM_POOL" <<'PY'
import json
import pathlib
import sys

manifest_path = pathlib.Path(sys.argv[1])
aggregate_path = pathlib.Path(sys.argv[2])
out = pathlib.Path(sys.argv[3])
started_at = sys.argv[4]
finished_at = sys.argv[5]
test_exit = int(sys.argv[6])
runtime_exit = int(sys.argv[7])
dashboard_exit = int(sys.argv[8])
audit_verdict = sys.argv[9]
buy_with_coin_router = int(sys.argv[10])
market_seed = int(sys.argv[11])
cpmm_pool = int(sys.argv[12])
runtime_pass = (
    test_exit == 0
    and runtime_exit == 0
    and dashboard_exit == 0
    and audit_verdict == "PROCEED"
    and buy_with_coin_router >= 2
    and market_seed > 0
    and cpmm_pool > 0
)
verdict = "PROCEED" if runtime_pass else "VETO"

data = {
    "schema_version": "real11.router_positive_control.manifest.v1",
    "atom": "REAL-11 Atom 2 router positive-control",
    "risk_class": 3,
    "fc_nodes": [
        "FC1 router action to L4/L4E",
        "FC2 mandatory evidence directory",
        "FC3 scripted-not-E2 claim boundary",
        "economy gates",
    ],
    "source_evidence_path": str(out),
    "started_at_utc": started_at,
    "finished_at_utc": finished_at,
    "unit_command": "cargo test --test constitution_real11_router_positive_control -- --test-threads=1",
    "runtime_command": "TURINGOS_REAL7_SCRIPTED_TASK_OUTCOME_BUYS=Agent_1:Agent_2:1000 bash scripts/run_g_phase_batch.sh <run_tag> mini",
    "test_exit_code": test_exit,
    "runtime_exit_code": runtime_exit,
    "dashboard_exit_code": dashboard_exit,
    "audit_tape_verdict": audit_verdict,
    "runtime_repo": str(out / "runtime_repo"),
    "cas": str(out / "cas"),
    "audit_dashboard_report": str(out / "audit_dashboard_run_report.txt"),
    "runtime_buy_with_coin_router": buy_with_coin_router,
    "runtime_market_seed": market_seed,
    "runtime_cpmm_pool": cpmm_pool,
    "scripted_positive_control_is_not_e2": True,
    "live_non_scripted_agent_router_tx_observed": False,
    "e2_claim": False,
    "mandatory_artifacts": [
        "manifest.json",
        "REAL11_ROUTER_POSITIVE_CONTROL_VERDICT.json",
        "REAL11_ROUTER_POSITIVE_CONTROL_REPORT.md",
        "cargo_test.stdout",
        "cargo_test.stderr",
        "runtime_repo/",
        "cas/",
        "aggregate_verdict.json",
        "audit_dashboard_run_report.txt",
        "runtime_batch.stdout",
        "runtime_batch.stderr",
    ],
    "sg_coverage": {
        "SG-11.2.1_scripted_buy_yes_l4": "covered_by_test",
        "SG-11.2.2_buy_no_short_equivalent_l4_or_l4e": "covered_by_test",
        "SG-11.2.3_insufficient_balance_l4e": "covered_by_test",
        "SG-11.2.4_missing_pool_l4e": "covered_by_test",
        "SG-11.2.5_ctf_conserved": "covered_by_test",
        "SG-11.2.6_no_ghost_liquidity": "covered_by_test",
        "SG-11.2.7_no_f64_money_path": "covered_by_test",
    },
    "audit_ready": {
        "aggregate_verdict": verdict,
        "clean_context_audit_verdict": None,
        "designed_for_later_audit_PROCEED_record": True,
        "audit_record_slot": "clean_context_audit_verdict",
    },
}
manifest_path.write_text(json.dumps(data, indent=2, sort_keys=True) + "\n")

aggregate = {
    "schema_version": "real11.router_positive_control.aggregate_verdict.v1",
    "verdict": verdict,
    "test_exit_code": test_exit,
    "runtime_exit_code": runtime_exit,
    "dashboard_exit_code": dashboard_exit,
    "audit_tape_verdict": audit_verdict,
    "runtime_buy_with_coin_router": buy_with_coin_router,
    "runtime_market_seed": market_seed,
    "runtime_cpmm_pool": cpmm_pool,
    "scripted_positive_control_is_not_e2": True,
    "live_non_scripted_agent_router_tx_observed": False,
    "source_evidence_path": str(out),
    "unit_command": data["unit_command"],
    "runtime_command": data["runtime_command"],
}
aggregate_path.write_text(json.dumps(aggregate, indent=2, sort_keys=True) + "\n")
PY

if [[ "$TEST_EXIT" -eq 0 && "$RUNTIME_EXIT" -eq 0 && "$DASHBOARD_EXIT" -eq 0 && "$AUDIT_VERDICT" == "PROCEED" && "$BUY_WITH_COIN_ROUTER" -ge 2 ]]; then
    TEST_STATUS="PASS"
    SG_STATUS="PASS"
else
    TEST_STATUS="FAIL"
    SG_STATUS="BLOCKED"
fi

cat > "$REPORT" <<EOF
# REAL-11 Router Positive-Control Report

source evidence path: \`$OUT\`

This scripted positive control is not E2. It proves router wiring only.
E2 remains false unless a live, non-scripted, agent-generated router/short
action is observed on ChainTape/CAS.

## Command

\`\`\`text
${COMMAND[*]}
\`\`\`

test_exit_code: \`$TEST_EXIT\`
runtime_exit_code: \`$RUNTIME_EXIT\`
dashboard_exit_code: \`$DASHBOARD_EXIT\`
audit_tape verdict: \`$AUDIT_VERDICT\`
test_status: \`$TEST_STATUS\`

## Runtime Chain Evidence

\`\`\`text
runtime_repo: $OUT/runtime_repo
CAS path: $OUT/cas
audit_dashboard: $OUT/audit_dashboard_run_report.txt
market_seed: $MARKET_SEED
cpmm_pool: $CPMM_POOL
buy_with_coin_router: $BUY_WITH_COIN_ROUTER
scripted_task_outcome_buys: Agent_1:Agent_2:1000
\`\`\`

This runtime evidence uses scripted TaskOutcomeMarket buys as a positive
control. It proves the normal ChainTape/CAS/audit-dashboard path can carry
router actions; it is explicitly not E2.

## Claim Boundary

\`\`\`text
scripted fixture == not E2
no forced trade
no price-as-truth
no ghost liquidity
no f64 economy
no private CoT recording
no raw-log broadcast
dashboard/report is a materialized view, not source of truth
\`\`\`

## SG Coverage

| Gate | Status | Evidence |
| --- | --- | --- |
| SG-11.2.1 scripted BuyYesWithCoinRouterTx enters L4 | $SG_STATUS | \`cargo_test.stdout\` + \`aggregate_verdict.json\` |
| SG-11.2.2 scripted BuyNo / short-equivalent enters L4 or explicit L4.E | $SG_STATUS | \`cargo_test.stdout\` + \`aggregate_verdict.json\` |
| SG-11.2.3 insufficient balance routes L4.E / pre-submit classification | $SG_STATUS | \`cargo_test.stdout\` / \`cargo_test.stderr\` |
| SG-11.2.4 missing pool routes NoPool / L4.E | $SG_STATUS | \`cargo_test.stdout\` / \`cargo_test.stderr\` |
| SG-11.2.5 CTF conserved | $SG_STATUS | \`cargo_test.stdout\` / \`cargo_test.stderr\` |
| SG-11.2.6 no ghost liquidity | $SG_STATUS | \`cargo_test.stdout\` / \`cargo_test.stderr\` |
| SG-11.2.7 no f64 money path | $SG_STATUS | \`cargo_test.stdout\` / \`cargo_test.stderr\` |

## Audit Readiness

positive_control_verdict: \`$([[ "$TEST_STATUS" == "PASS" ]] && echo PROCEED || echo VETO)\`

The manifest records aggregate verdict now and keeps a separate slot for later
clean-context audit:
\`audit_ready.clean_context_audit_verdict\`.
EOF

if [[ "$TEST_STATUS" == "PASS" ]]; then
    cp "$REPORT" "$ROOT_REPORT"
fi

echo "REAL-11 router positive-control evidence: $OUT"
echo "test_exit_code=$TEST_EXIT"
echo "runtime_exit_code=$RUNTIME_EXIT"
echo "audit_verdict=$AUDIT_VERDICT"
echo "buy_with_coin_router=$BUY_WITH_COIN_ROUTER"
if [[ "$TEST_STATUS" == "PASS" ]]; then
    exit 0
fi
exit 1
