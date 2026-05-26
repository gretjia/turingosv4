#!/usr/bin/env bash
# True-suite TDMA/proof current-kernel evidence runner.
#
# Uses a real external LLM through the local OpenAI-compatible proxy, then
# drives the public CLI path:
#   turingos init -> turingos tdma run -> durable TDMA GitTapeLedger evidence.
#
# TDMA is a bounded proof-work tape, not the bottom-white L4 ChainTape. This
# runner records replay-style invariant evidence honestly as replay_report.json.

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RUN_ID="${1:-tdma_current_kernel_$(date -u +%Y%m%dT%H%M%SZ)}"
RUN_ID="${RUN_ID#handover/evidence/true_suite/}"
RUN_ROOT="$PROJECT_ROOT/handover/evidence/true_suite/$RUN_ID"
RUN_DIR="$RUN_ROOT/tdma"
LLM_PROXY_URL="${LLM_PROXY_URL:-http://localhost:8080}"
LLM_ENDPOINT="${LLM_ENDPOINT:-${LLM_PROXY_URL%/}/v1/chat/completions}"
INIT_PROVIDER="${INIT_PROVIDER:-deepseek}"
TDMA_JUDGE="${TDMA_JUDGE:-putnam_2025_b3}"
TDMA_ROLE="${TDMA_ROLE:-meta}"
TDMA_MAX_ATTEMPTS_PER_STAGE="${TDMA_MAX_ATTEMPTS_PER_STAGE:-3}"
TDMA_TEMPERATURE="${TDMA_TEMPERATURE:-0.2}"

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

echo "[build] cargo build --release --bin turingos --bin verify_chaintape --bin tdma_proof_current_kernel --bin full_system_augment_current_kernel --bin full_system_participation_current_kernel"
(cd "$PROJECT_ROOT" && cargo build --release --bin turingos --bin verify_chaintape --bin tdma_proof_current_kernel --bin full_system_augment_current_kernel --bin full_system_participation_current_kernel)

TURINGOS="$PROJECT_ROOT/target/release/turingos"
HELPER="$PROJECT_ROOT/target/release/tdma_proof_current_kernel"
AUGMENT="$PROJECT_ROOT/target/release/full_system_augment_current_kernel"
PARTICIPATION="$PROJECT_ROOT/target/release/full_system_participation_current_kernel"

echo "[init] turingos init --project $RUN_DIR --provider $INIT_PROVIDER"
"$TURINGOS" init --project "$RUN_DIR" --template proof --provider "$INIT_PROVIDER"

echo "[tdma] external LLM -> public turingos tdma run -> durable TDMA tape"
TDMA_OUTPUT="$(
    TURINGOS_SILICONFLOW_ENDPOINT="$LLM_ENDPOINT" \
    "$TURINGOS" tdma run \
        --workspace "$RUN_DIR" \
        --judge "$TDMA_JUDGE" \
        --role "$TDMA_ROLE" \
        --evidence-dir "$RUN_DIR" \
        --max-attempts-per-stage "$TDMA_MAX_ATTEMPTS_PER_STAGE" \
        --temperature "$TDMA_TEMPERATURE" \
        --tape-backend git
)"
printf '%s\n' "$TDMA_OUTPUT" > "$RUN_DIR/tdma_output.txt"

python3 - "$RUN_DIR" <<'PY'
import hashlib
import json
import subprocess
import sys
from pathlib import Path

run_dir = Path(sys.argv[1])
manifest_path = run_dir / "manifest.json"
if not manifest_path.exists():
    raise SystemExit("manifest.json missing")
manifest = json.loads(manifest_path.read_text())
tdma_tape = run_dir / "tdma_tape.git"

def sha256(path: Path) -> str:
    h = hashlib.sha256()
    h.update(path.read_bytes())
    return h.hexdigest()

checks = {
    "manifest_present": True,
    "chaintape_present": (run_dir / "chaintape.jsonl").is_file(),
    "probes_present": (run_dir / "per_attempt_probes.jsonl").is_file(),
    "tdma_git_tape_present": tdma_tape.is_dir(),
    "production_report_present": (run_dir / "ProductionTdmaReport.md").is_file(),
    "stages_completed_all": manifest.get("stages_completed") == manifest.get("stages_total"),
    "all_prompts_within_budget": manifest.get("all_prompts_within_budget") is True,
    "no_raw_stderr_leak": manifest.get("leak_in_any_prompt") is False,
    "chaintape_sha256_verified": sha256(run_dir / "chaintape.jsonl") == manifest.get("chaintape_sha256"),
    "probes_sha256_verified": sha256(run_dir / "per_attempt_probes.jsonl") == manifest.get("probes_sha256"),
}
if tdma_tape.is_dir():
    rev_count = subprocess.run(
        ["git", "--git-dir", str(tdma_tape), "rev-list", "--all", "--count"],
        check=False,
        text=True,
        capture_output=True,
    )
    verified_head = subprocess.run(
        ["git", "--git-dir", str(tdma_tape), "rev-parse", "--verify", "refs/tdma/verified_head"],
        check=False,
        text=True,
        capture_output=True,
    )
    checks["tdma_git_tape_has_commits"] = rev_count.returncode == 0 and int((rev_count.stdout or "0").strip() or "0") > 0
    checks["tdma_git_verified_head_ref_present"] = verified_head.returncode == 0
report = {
    "schema_version": "turingosv4.true_suite.tdma_replay_report.v1",
    "ok": all(checks.values()),
    "checks": checks,
    "stages_completed": manifest.get("stages_completed"),
    "stages_total": manifest.get("stages_total"),
    "total_attempts": manifest.get("total_attempts"),
    "problem_label": manifest.get("problem_label"),
    "model_label": manifest.get("model_label"),
}
(run_dir / "tdma_replay_report.json").write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
if not report["ok"]:
    raise SystemExit(json.dumps(report, indent=2, sort_keys=True))
PY

tar -C "$RUN_DIR" -czf "$RUN_DIR/tdma_tape.git.tar.gz" tdma_tape.git

echo "[bridge] TDMA proof-work evidence -> CAS summary -> signed WorkTx on canonical ChainTape"
"$HELPER" \
    --runtime-repo "$RUN_DIR/runtime_repo" \
    --cas "$RUN_DIR/cas" \
    --run-id "$RUN_ID" \
    --constitution "$PROJECT_ROOT/constitution.md" \
    --tdma-evidence-dir "$RUN_DIR" \
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
    --family-id "tdma_proof" \
    --entrypoint "scripts/run_true_suite_tdma_current_kernel.sh" \
    --runtime-repo "$RUN_DIR/runtime_repo" \
    --cas "$RUN_DIR/cas" \
    --replay-report "$RUN_DIR/replay_report.json" \
    --genesis-report "$RUN_DIR/genesis_report.json" \
    --domain-manifest "$RUN_DIR/tdma_proof_manifest.json" \
    --fc3-index "$RUN_DIR/governance_capsule_index.json" \
    --require-full-system \
    --out "$RUN_DIR/full_system_participation.json"

cat > "$RUN_DIR/tdma_run_manifest.json" <<EOF
{
  "schema_version": "turingosv4.true_suite.tdma_current_kernel.v1",
  "run_id": "$RUN_ID",
  "git_head": "$(cd "$PROJECT_ROOT" && git rev-parse HEAD)",
  "entrypoint": "scripts/run_true_suite_tdma_current_kernel.sh",
  "llm_proxy_url": "$LLM_PROXY_URL",
  "llm_endpoint": "$LLM_ENDPOINT",
  "init_provider": "$INIT_PROVIDER",
  "tdma_judge": "$TDMA_JUDGE",
  "tdma_role": "$TDMA_ROLE",
  "tdma_max_attempts_per_stage": $TDMA_MAX_ATTEMPTS_PER_STAGE,
  "tdma_temperature": "$TDMA_TEMPERATURE",
  "tdma_tape": "$RUN_DIR/tdma_tape.git",
  "tdma_tape_archive": "$RUN_DIR/tdma_tape.git.tar.gz",
  "chaintape_jsonl": "$RUN_DIR/chaintape.jsonl",
  "per_attempt_probes": "$RUN_DIR/per_attempt_probes.jsonl",
  "manifest": "$RUN_DIR/manifest.json",
  "tdma_replay_report": "$RUN_DIR/tdma_replay_report.json",
  "runtime_repo": "$RUN_DIR/runtime_repo",
  "cas": "$RUN_DIR/cas",
  "genesis_report": "$RUN_DIR/genesis_report.json",
  "tdma_proof_manifest": "$RUN_DIR/tdma_proof_manifest.json",
  "failure_taxonomy": "$RUN_DIR/failure_taxonomy.json",
  "full_system_augmentation_manifest": "$RUN_DIR/full_system_augmentation_manifest.json",
  "governance_capsule_index": "$RUN_DIR/governance_capsule_index.json",
  "replay_report": "$RUN_DIR/replay_report.json",
  "full_system_participation": "$RUN_DIR/full_system_participation.json",
  "notes": [
    "DeepSeek/SiliconFlow access is outside the kernel through the local LLM proxy",
    "turingos tdma run is the public CLI entrypoint",
    "TDMA writes a durable GitTapeLedger proof-work tape, not bottom-white L4",
    "tdma_replay_report.json verifies manifest and tape/probe hashes without historical evidence rewrite",
    "replay_report.json is canonical ChainTape replay after the TDMA proof bridge WorkTx lands"
  ]
}
EOF

echo "TRUE-SUITE TDMA/proof evidence: $RUN_DIR"
