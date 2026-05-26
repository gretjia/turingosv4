#!/usr/bin/env bash
# Broad AGI true-suite batch evidence runner.
#
# This is a control plane over the narrower current-kernel true-suite runners.
# It does not score benchmarks or close OBL-005 by itself. It writes a batch
# manifest, per-family JSONL, and an FC-trace aggregate that make over-claiming
# hard: pending benchmark adapters stay pending, and plan-only mode never emits
# passed coverage.

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
MODE="plan-only"
RUN_ID="broad_agi_batch_$(date -u +%Y%m%dT%H%M%SZ)"
RUN_ROOT=""
SELECTED_RUNNERS="${BROAD_TRUE_SUITE_RUNNERS:-boot_cli_current_kernel_fresh,replay_cas_tamper_repair_current,market_external_agent_fresh,generate_artifact_chain_fresh,tdma_real_proof_fresh,fc3_governance_reinit_fresh,gpqa_science_reasoning_fresh,math_competition_reasoning_fresh,swebench_live_coding_repair_fresh,toolbench_api_tool_use_fresh,mind2web_open_web_fresh}"

while [[ $# -gt 0 ]]; do
    case "$1" in
        --plan-only)
            MODE="plan-only"
            shift
            ;;
        --execute-installed)
            MODE="execute-installed"
            shift
            ;;
        --run-id)
            RUN_ID="${2:?--run-id requires a value}"
            shift 2
            ;;
        --run-root)
            RUN_ROOT="${2:?--run-root requires a value}"
            shift 2
            ;;
        --runners)
            SELECTED_RUNNERS="${2:?--runners requires a comma-separated value}"
            shift 2
            ;;
        *)
            RUN_ID="${1#handover/evidence/true_suite/}"
            shift
            ;;
    esac
done

RUN_ID="${RUN_ID#handover/evidence/true_suite/}"
RUN_ROOT="${RUN_ROOT:-$PROJECT_ROOT/handover/evidence/true_suite/$RUN_ID}"
RUN_DIR="$RUN_ROOT/broad_batch"

if [[ -e "$RUN_DIR" ]]; then
    echo "ERROR: evidence directory already exists: $RUN_DIR" >&2
    exit 2
fi

if [[ "$MODE" == "execute-installed" ]]; then
    if [[ -n "$(cd "$PROJECT_ROOT" && git status --porcelain | grep -vE '^\?\? handover/evidence/' | head -1)" ]]; then
        echo "ERROR: working tree has non-evidence changes; run /runner-preflight before evidence runners" >&2
        exit 3
    fi

    IFS=',' read -r -a RUNNER_LIST <<< "$SELECTED_RUNNERS"
    for runner in "${RUNNER_LIST[@]}"; do
        case "$runner" in
            boot_cli_current_kernel_fresh)
                "$PROJECT_ROOT/scripts/run_true_suite_boot_cli_current_kernel.sh" "$RUN_ID"
                ;;
            replay_cas_tamper_repair_current)
                "$PROJECT_ROOT/scripts/run_true_suite_replay_cas_tamper_current_kernel.sh" "$RUN_ID"
                ;;
            market_external_agent_fresh)
                "$PROJECT_ROOT/scripts/run_true_suite_market_external_agent.sh" "$RUN_ID"
                ;;
            generate_artifact_chain_fresh)
                "$PROJECT_ROOT/scripts/run_true_suite_generate_artifact_current_kernel.sh" "$RUN_ID"
                ;;
            tdma_real_proof_fresh)
                "$PROJECT_ROOT/scripts/run_true_suite_tdma_current_kernel.sh" "$RUN_ID"
                ;;
            fc3_governance_reinit_fresh)
                "$PROJECT_ROOT/scripts/run_true_suite_fc3_governance_reinit_current_kernel.sh" "$RUN_ID"
                ;;
            gpqa_science_reasoning_fresh)
                "$PROJECT_ROOT/scripts/run_true_suite_gpqa_science_reasoning_current_kernel.sh" "$RUN_ID"
                ;;
            math_competition_reasoning_fresh)
                "$PROJECT_ROOT/scripts/run_true_suite_math_competition_current_kernel.sh" "$RUN_ID"
                ;;
            swebench_live_coding_repair_fresh)
                "$PROJECT_ROOT/scripts/run_true_suite_swebench_current_kernel.sh" "$RUN_ID"
                ;;
            toolbench_api_tool_use_fresh)
                "$PROJECT_ROOT/scripts/run_true_suite_toolbench_current_kernel.sh" "$RUN_ID"
                ;;
            mind2web_open_web_fresh)
                "$PROJECT_ROOT/scripts/run_true_suite_mind2web_current_kernel.sh" "$RUN_ID"
                ;;
            *)
                echo "ERROR: unknown broad true-suite runner id: $runner" >&2
                exit 4
                ;;
        esac
    done
    "$PROJECT_ROOT/scripts/package_true_suite_evidence.sh" --run-root "$RUN_ROOT"
elif [[ "$MODE" != "plan-only" ]]; then
    echo "ERROR: mode must be --plan-only or --execute-installed" >&2
    exit 5
fi

mkdir -p "$RUN_DIR"

python3 - "$PROJECT_ROOT" "$RUN_ROOT" "$RUN_DIR" "$RUN_ID" "$MODE" "$SELECTED_RUNNERS" <<'PY'
import json
import subprocess
import sys
from pathlib import Path

try:
    import tomllib
except ModuleNotFoundError:  # pragma: no cover - Python 3.11+ in CI.
    raise SystemExit("Python tomllib is required")

project = Path(sys.argv[1])
run_root = Path(sys.argv[2])
run_dir = Path(sys.argv[3])
run_id = sys.argv[4]
mode = sys.argv[5]
selected_runner_ids = [item for item in sys.argv[6].split(",") if item]

broad_manifest_path = project / "tests/fixtures/liveness/broad_agi_true_suite_manifest.toml"
coverage_manifest_path = project / "tests/fixtures/liveness/realworld_liveness_coverage.toml"

with broad_manifest_path.open("rb") as f:
    broad_manifest = tomllib.load(f)
with coverage_manifest_path.open("rb") as f:
    coverage_manifest = tomllib.load(f)

installed = {
    "boot_cli_current_kernel_fresh": {
        "entrypoint": "scripts/run_true_suite_boot_cli_current_kernel.sh",
        "family_ids": [],
    },
    "replay_cas_tamper_repair_current": {
        "entrypoint": "scripts/run_true_suite_replay_cas_tamper_current_kernel.sh",
        "family_ids": [],
    },
    "market_external_agent_fresh": {
        "entrypoint": "scripts/run_true_suite_market_external_agent.sh",
        "family_ids": ["market_economy_polymarket"],
    },
    "generate_artifact_chain_fresh": {
        "entrypoint": "scripts/run_true_suite_generate_artifact_current_kernel.sh",
        "family_ids": ["gaia_general_assistant"],
    },
    "tdma_real_proof_fresh": {
        "entrypoint": "scripts/run_true_suite_tdma_current_kernel.sh",
        "family_ids": ["math_formal_proof"],
    },
    "fc3_governance_reinit_fresh": {
        "entrypoint": "scripts/run_true_suite_fc3_governance_reinit_current_kernel.sh",
        "family_ids": ["memory_feedback_reinit"],
    },
    "gpqa_science_reasoning_fresh": {
        "entrypoint": "scripts/run_true_suite_gpqa_science_reasoning_current_kernel.sh",
        "family_ids": ["gpqa_science_reasoning"],
    },
    "math_competition_reasoning_fresh": {
        "entrypoint": "scripts/run_true_suite_math_competition_current_kernel.sh",
        "family_ids": ["math_formal_proof"],
    },
    "swebench_live_coding_repair_fresh": {
        "entrypoint": "scripts/run_true_suite_swebench_current_kernel.sh",
        "family_ids": ["swebench_live_coding_repair"],
    },
    "toolbench_api_tool_use_fresh": {
        "entrypoint": "scripts/run_true_suite_toolbench_current_kernel.sh",
        "family_ids": ["toolbench_api_tool_use"],
    },
    "mind2web_open_web_fresh": {
        "entrypoint": "scripts/run_true_suite_mind2web_current_kernel.sh",
        "family_ids": ["mind2web_open_web"],
    },
}

family_runner_status = {
    "market_economy_polymarket": "domain_runner_installed_evidence_required",
    "math_formal_proof": "domain_runner_installed_evidence_required",
    "swebench_live_coding_repair": "domain_runner_installed_evidence_required",
    "gaia_general_assistant": "substrate_runner_installed_benchmark_adapter_pending",
    "gpqa_science_reasoning": "domain_runner_installed_evidence_required",
    "memory_feedback_reinit": "domain_runner_installed_evidence_required",
    "toolbench_api_tool_use": "domain_runner_installed_evidence_required",
    "mind2web_open_web": "domain_runner_installed_evidence_required",
}

def git_head() -> str:
    result = subprocess.run(
        ["git", "rev-parse", "HEAD"],
        cwd=project,
        check=True,
        text=True,
        capture_output=True,
    )
    return result.stdout.strip()

def materialize(path_template: str) -> Path:
    replaced = path_template.replace("handover/evidence/true_suite/<run>", str(run_root))
    return project / replaced if not replaced.startswith("/") else Path(replaced)

def artifacts_present(templates) -> bool:
    return all(materialize(path).exists() for path in templates)

def nested_bool(value, keys) -> bool:
    cur = value
    for key in keys:
        if not isinstance(cur, dict):
            return False
        cur = cur.get(key)
    return cur is True

def nested_int(value, keys) -> int:
    cur = value
    for key in keys:
        if not isinstance(cur, dict):
            return 0
        cur = cur.get(key)
    return cur if isinstance(cur, int) and cur >= 0 else 0

def market_choice_lit(report) -> bool:
    if not nested_bool(report, ["market", "present"]):
        return False
    return (
        nested_int(report, ["market", "agent_market_action_txs"]) > 0
        or nested_int(report, ["market", "market_decision_submitted_count"]) > 0
        or nested_int(report, ["market", "market_decision_no_trade_count"]) > 0
        or nested_int(report, ["market", "market_decision_declined_count"]) > 0
    )

def full_system_report_result(templates) -> dict:
    report_templates = [
        path for path in templates if path.endswith("/full_system_participation.json")
    ]
    if not report_templates:
        return {
            "present": False,
            "lit": False,
            "verdict": "UNDECLARED",
            "missing": ["full_system_participation.json_not_declared"],
            "required_rows": {},
        }

    report_path = materialize(report_templates[0])
    if not report_path.exists():
        return {
            "present": False,
            "lit": False,
            "verdict": "MISSING",
            "missing": ["full_system_participation.json_missing"],
            "required_rows": {},
        }

    try:
        report = json.loads(report_path.read_text(encoding="utf-8"))
    except Exception as exc:  # pragma: no cover - exercised by shell diagnostics.
        return {
            "present": True,
            "lit": False,
            "verdict": "INVALID_JSON",
            "missing": [f"full_system_participation_json_parse_error:{exc}"],
            "required_rows": {},
        }

    verdict = report.get("verdict") if isinstance(report, dict) else {}
    if not isinstance(verdict, dict):
        verdict = {}
    declared_missing = verdict.get("missing")
    if not isinstance(declared_missing, list):
        declared_missing = []

    required_rows = {
        "FC1_runtime_work_or_l4e": nested_bool(report, ["fc1", "present"]),
        "FC2_boot_tick_replay": nested_bool(report, ["fc2", "present"]),
        "FC3_typed_architect_veto_feedback": nested_bool(
            report, ["fc3", "typed_meta_roles_present"]
        ),
        "FC3_reinit_semantics": nested_bool(report, ["fc3", "reinit_semantics_present"]),
        "market_economy_invest_or_visible_abstention": market_choice_lit(report),
        "replay_all_indicators_pass": nested_bool(report, ["replay", "all_indicators_pass"]),
    }
    computed_missing = [
        name for name, row_lit in required_rows.items() if not row_lit
    ]
    declared_full = verdict.get("full_system_participation") is True
    declared_verdict = verdict.get("full_system_verdict", "INVALID_REPORT")
    lit = declared_full and declared_verdict == "FULL_SYSTEM_LIT" and not computed_missing
    missing = sorted({str(item) for item in declared_missing} | set(computed_missing))
    if declared_full and computed_missing:
        declared_verdict = "INVALID_FULL_SYSTEM_REPORT"

    return {
        "present": True,
        "lit": lit,
        "verdict": declared_verdict,
        "missing": missing,
        "required_rows": required_rows,
    }

def status_for_artifacts(final_present: bool, full_system_report: dict, fallback: str) -> str:
    if final_present and full_system_report["lit"]:
        return "full_system_participation_passed"
    if final_present and full_system_report["present"]:
        return "full_system_participation_report_partial"
    if full_system_report["present"]:
        return "full_system_report_present_declared_artifacts_missing"
    if final_present:
        return "domain_artifacts_present_full_system_pending"
    return fallback

def fc_blocks(traces) -> list[str]:
    blocks = set()
    for trace in traces:
        if ":" in trace:
            blocks.add(trace.split(":", 1)[0])
    return sorted(blocks)

domain_results = []
selected_set = set(selected_runner_ids)
for task in coverage_manifest["task"]:
    task_id = task["id"]
    installed_runner = installed.get(task_id)
    final_present = artifacts_present(task["final_evidence_artifacts"])
    full_system_report = full_system_report_result(task["final_evidence_artifacts"])
    if installed_runner and mode == "execute-installed" and task_id in selected_set:
        status = "selected_runner_failed_or_incomplete"
    elif installed_runner and task_id in selected_set:
        status = "installed_runner_requires_execution"
    elif installed_runner:
        status = "installed_runner_not_selected"
    else:
        status = "runner_pending"
    status = status_for_artifacts(final_present, full_system_report, status)
    domain_results.append(
        {
            "kind": "realworld_domain",
            "id": task_id,
            "problem_type": task["problem_type"],
            "status": status,
            "entrypoint": task["entrypoint"],
            "fc_blocks": fc_blocks(task["constitutional_paths"]),
            "final_artifacts_present": final_present,
            "full_system_report_present": full_system_report["present"],
            "full_system_report_lit": full_system_report["lit"],
            "full_system_verdict": full_system_report["verdict"],
            "full_system_missing": full_system_report["missing"],
            "full_system_required_rows": full_system_report["required_rows"],
            "final_artifacts": task["final_evidence_artifacts"],
        }
    )

family_results = []
for family in broad_manifest["family"]:
    family_id = family["id"]
    final_present = artifacts_present(family["final_evidence_artifacts"])
    full_system_report = full_system_report_result(family["final_evidence_artifacts"])
    status = status_for_artifacts(
        final_present,
        full_system_report,
        family_runner_status.get(family_id, "benchmark_adapter_pending"),
    )
    family_results.append(
        {
            "kind": "broad_agi_family",
            "id": family_id,
            "source_family": family["source_family"],
            "risk_class": family["risk_class"],
            "entry_boundary": family["entry_boundary"],
            "status": status,
            "fc_blocks": fc_blocks(family["fc_trace"]),
            "failure_taxonomy": family["failure_taxonomy"],
            "final_artifacts_present": final_present,
            "full_system_report_present": full_system_report["present"],
            "full_system_report_lit": full_system_report["lit"],
            "full_system_verdict": full_system_report["verdict"],
            "full_system_missing": full_system_report["missing"],
            "full_system_required_rows": full_system_report["required_rows"],
            "final_artifacts": family["final_evidence_artifacts"],
        }
    )

all_results = domain_results + family_results
fc_seen = sorted({block for row in all_results for block in row["fc_blocks"]})
required_fc = broad_manifest["required_fc_blocks"]
per_result_fc_complete = all(set(row["fc_blocks"]) >= set(required_fc) for row in all_results)
pending_results = [
    row
    for row in all_results
    if row["status"] != "full_system_participation_passed"
]
all_declared_artifacts_present = mode == "execute-installed" and all(
    row["final_artifacts_present"] for row in all_results
)
full_system_closure_candidate = (
    mode == "execute-installed"
    and not pending_results
    and all_declared_artifacts_present
    and per_result_fc_complete
)

batch_manifest = {
    "schema_version": "turingosv4.true_suite.broad_agi_batch.v1",
    "run_id": run_id,
    "git_head": git_head(),
    "mode": mode,
    "closure_status": "OPEN_REAL_WORLD_COVERAGE_PENDING",
    "full_system_required_for_final": bool(broad_manifest.get("full_system_required_for_final", False)),
    "full_system_sample_manifest": broad_manifest.get("full_system_sample_manifest", "full_system_participation.json"),
    "per_sample_fc_union_is_not_sufficient": bool(broad_manifest.get("per_sample_fc_union_is_not_sufficient", False)),
    "market_participation_required_for_every_sample": bool(
        broad_manifest.get("market_participation_required_for_every_sample", False)
    ),
    "all_declared_artifacts_present": all_declared_artifacts_present,
    "full_system_closure_candidate": full_system_closure_candidate,
    "closure_decision_source": "OBL-005 witness after per-sample full_system_participation reports",
    "old_15_is_not_sufficient": bool(broad_manifest["old_15_is_not_sufficient"]),
    "leaderboard_score_is_not_liveness": bool(broad_manifest["leaderboard_score_is_not_liveness"]),
    "authority": "constitution.md + fresh current-kernel true-problem evidence",
    "broad_manifest": str(broad_manifest_path.relative_to(project)),
    "realworld_coverage_manifest": str(coverage_manifest_path.relative_to(project)),
    "selected_runners": selected_runner_ids,
    "installed_domain_runners": [
        {"id": key, **value} for key, value in sorted(installed.items())
    ],
    "outputs": {
        "family_results_jsonl": str(run_dir / "family_results.jsonl"),
        "aggregate_fc_trace_report": str(run_dir / "aggregate_fc_trace_report.json"),
        "evidence_package_manifest": str(run_root / "evidence_package_manifest.json"),
    },
    "no_overclaim_guards": [
        "plan-only mode cannot emit passed coverage",
        "pending benchmark adapters never count as liveness pass",
        "old 15-question evidence cannot close OBL-005",
        "leaderboard score is capability signal only, not module liveness",
        "TDMA evidence is domain tape evidence, not bottom-white L4 ChainTape",
        "provider raw prompt/response is not a valid final artifact",
        "full_system_participation.json must be parsed and FULL_SYSTEM_LIT, not merely present",
        "domain artifacts without a FULL_SYSTEM_LIT participation report remain partial runner evidence",
        "market/economy must participate even in one-agent runs via invest or tape-visible abstention",
    ],
}

aggregate = {
    "schema_version": "turingosv4.true_suite.broad_agi_fc_trace_report.v1",
    "run_id": run_id,
    "mode": mode,
    "required_fc_blocks": required_fc,
    "fc_blocks_seen": fc_seen,
    "all_required_fc_blocks_declared": set(fc_seen) >= set(required_fc),
    "per_result_required_fc_blocks_declared": per_result_fc_complete,
    "realworld_domain_count": len(domain_results),
    "broad_family_count": len(family_results),
    "installed_domain_runner_count": len(installed),
    "declared_artifact_result_count": sum(
        1 for row in all_results if row["final_artifacts_present"]
    ),
    "full_system_participation_pass_count": sum(
        1 for row in all_results if row["full_system_report_lit"]
    ),
    "pending_result_count": len(pending_results),
    "pending_ids": [row["id"] for row in pending_results],
    "partial_runner_ids": [
        row["id"] for row in all_results if not row["full_system_report_lit"]
    ],
    "all_declared_artifacts_present": all_declared_artifacts_present,
    "full_system_closure_candidate": full_system_closure_candidate,
}

run_dir.mkdir(parents=True, exist_ok=True)
(run_dir / "broad_agi_batch_manifest.json").write_text(
    json.dumps(batch_manifest, indent=2, sort_keys=True) + "\n",
    encoding="utf-8",
)
(run_dir / "aggregate_fc_trace_report.json").write_text(
    json.dumps(aggregate, indent=2, sort_keys=True) + "\n",
    encoding="utf-8",
)
with (run_dir / "family_results.jsonl").open("w", encoding="utf-8") as f:
    for row in all_results:
        f.write(json.dumps(row, sort_keys=True) + "\n")

print(f"TRUE-SUITE broad AGI batch evidence: {run_dir}")
print(f"closure_status={batch_manifest['closure_status']}")
print(f"full_system_closure_candidate={str(full_system_closure_candidate).lower()}")
PY
