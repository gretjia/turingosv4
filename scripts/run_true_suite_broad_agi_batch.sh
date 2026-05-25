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
SELECTED_RUNNERS="${BROAD_TRUE_SUITE_RUNNERS:-boot_cli_current_kernel_fresh,replay_cas_tamper_repair_current,market_external_agent_fresh,generate_artifact_chain_fresh,tdma_real_proof_fresh,fc3_governance_reinit_fresh}"

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
}

family_runner_status = {
    "market_economy_polymarket": "domain_runner_installed_evidence_required",
    "math_formal_proof": "substrate_runner_installed_benchmark_adapter_pending",
    "gaia_general_assistant": "substrate_runner_installed_benchmark_adapter_pending",
    "memory_feedback_reinit": "domain_runner_installed_evidence_required",
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
    if final_present:
        status = "fresh_artifacts_present_unscored"
    elif installed_runner and mode == "execute-installed" and task_id in selected_set:
        status = "selected_runner_failed_or_incomplete"
    elif installed_runner and task_id in selected_set:
        status = "installed_runner_requires_execution"
    elif installed_runner:
        status = "installed_runner_not_selected"
    else:
        status = "runner_pending"
    domain_results.append(
        {
            "kind": "realworld_domain",
            "id": task_id,
            "problem_type": task["problem_type"],
            "status": status,
            "entrypoint": task["entrypoint"],
            "fc_blocks": fc_blocks(task["constitutional_paths"]),
            "final_artifacts_present": final_present,
            "final_artifacts": task["final_evidence_artifacts"],
        }
    )

family_results = []
for family in broad_manifest["family"]:
    family_id = family["id"]
    final_present = artifacts_present(family["final_evidence_artifacts"])
    if final_present:
        status = "fresh_artifacts_present_unscored"
    else:
        status = family_runner_status.get(family_id, "benchmark_adapter_pending")
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
            "final_artifacts": family["final_evidence_artifacts"],
        }
    )

all_results = domain_results + family_results
fc_seen = sorted({block for row in all_results for block in row["fc_blocks"]})
required_fc = broad_manifest["required_fc_blocks"]
pending_results = [
    row
    for row in all_results
    if row["status"] != "fresh_artifacts_present_unscored"
]
final_closure_possible = (
    mode == "execute-installed"
    and not pending_results
    and set(fc_seen) >= set(required_fc)
)

batch_manifest = {
    "schema_version": "turingosv4.true_suite.broad_agi_batch.v1",
    "run_id": run_id,
    "git_head": git_head(),
    "mode": mode,
    "closure_status": "OPEN_REAL_WORLD_COVERAGE_PENDING",
    "final_closure_possible": final_closure_possible,
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
    ],
}

aggregate = {
    "schema_version": "turingosv4.true_suite.broad_agi_fc_trace_report.v1",
    "run_id": run_id,
    "mode": mode,
    "required_fc_blocks": required_fc,
    "fc_blocks_seen": fc_seen,
    "all_required_fc_blocks_declared": set(fc_seen) >= set(required_fc),
    "realworld_domain_count": len(domain_results),
    "broad_family_count": len(family_results),
    "installed_domain_runner_count": len(installed),
    "fresh_artifact_result_count": sum(
        1 for row in all_results if row["status"] == "fresh_artifacts_present_unscored"
    ),
    "pending_result_count": len(pending_results),
    "pending_ids": [row["id"] for row in pending_results],
    "final_closure_possible": final_closure_possible,
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
print(f"final_closure_possible={str(final_closure_possible).lower()}")
PY
