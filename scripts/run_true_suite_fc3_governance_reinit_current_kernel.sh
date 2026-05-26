#!/usr/bin/env bash
# True-suite FC3 governance/re-init current-kernel evidence runner.
#
# Drives the existing typed FC3 runtime path and verifies it through public
# ChainTape replay:
#   turingos init -> fc3_governance_reinit_current_kernel
#   -> turingos verify chaintape.
#
# This is not an external PR ceremony, dashboard proof, or handover-file proof.

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RUN_ID="${1:-fc3_governance_reinit_$(date -u +%Y%m%dT%H%M%SZ)}"
RUN_ID="${RUN_ID#handover/evidence/true_suite/}"
RUN_ROOT="$PROJECT_ROOT/handover/evidence/true_suite/$RUN_ID"
RUN_DIR="$RUN_ROOT/fc3"
INIT_PROVIDER="${INIT_PROVIDER:-deepseek}"

if [[ -e "$RUN_DIR" ]]; then
    echo "ERROR: evidence directory already exists: $RUN_DIR" >&2
    exit 2
fi

mkdir -p "$RUN_DIR"

if [[ -n "$(cd "$PROJECT_ROOT" && git status --porcelain | grep -vE '^\?\? handover/evidence/' | head -1)" ]]; then
    echo "ERROR: working tree has non-evidence changes; run /runner-preflight before evidence runners" >&2
    exit 3
fi

echo "[build] cargo build --release --bin turingos --bin verify_chaintape --bin fc3_governance_reinit_current_kernel --bin full_system_augment_current_kernel --bin full_system_participation_current_kernel"
(cd "$PROJECT_ROOT" && cargo build --release --bin turingos --bin verify_chaintape --bin fc3_governance_reinit_current_kernel --bin full_system_augment_current_kernel --bin full_system_participation_current_kernel)

TURINGOS="$PROJECT_ROOT/target/release/turingos"
HELPER="$PROJECT_ROOT/target/release/fc3_governance_reinit_current_kernel"
AUGMENT="$PROJECT_ROOT/target/release/full_system_augment_current_kernel"
PARTICIPATION="$PROJECT_ROOT/target/release/full_system_participation_current_kernel"

echo "[init] turingos init --project $RUN_DIR --provider $INIT_PROVIDER"
"$TURINGOS" init --project "$RUN_DIR" --template proof --provider "$INIT_PROVIDER"

echo "[fc3] current-kernel typed FC3 governance/re-init sequence"
"$HELPER" \
    --runtime-repo "$RUN_DIR/runtime_repo" \
    --cas "$RUN_DIR/cas" \
    --run-id "$RUN_ID" \
    --constitution "$PROJECT_ROOT/constitution.md" \
    --out-dir "$RUN_DIR"

cp "$RUN_DIR/runtime_repo/genesis_report.json" "$RUN_DIR/genesis_report.json"

echo "[full-system] append FC1/market/FC3 participation rows to same ChainTape"
"$AUGMENT" \
    --runtime-repo "$RUN_DIR/runtime_repo" \
    --cas "$RUN_DIR/cas" \
    --run-id "$RUN_ID" \
    --constitution "$PROJECT_ROOT/constitution.md" \
    --out-dir "$RUN_DIR" \
    --skip-fc3

cp "$RUN_DIR/runtime_repo/genesis_report.json" "$RUN_DIR/genesis_report.json"

echo "[verify] turingos verify chaintape"
"$TURINGOS" verify chaintape \
    --repo "$RUN_DIR/runtime_repo" \
    --cas "$RUN_DIR/cas" \
    --run-id "$RUN_ID" \
    --out "$RUN_DIR/fc3_replay_report.json"

"$PARTICIPATION" \
    --run-id "$RUN_ID" \
    --family-id "memory_feedback_reinit" \
    --entrypoint "scripts/run_true_suite_fc3_governance_reinit_current_kernel.sh" \
    --runtime-repo "$RUN_DIR/runtime_repo" \
    --cas "$RUN_DIR/cas" \
    --replay-report "$RUN_DIR/fc3_replay_report.json" \
    --genesis-report "$RUN_DIR/genesis_report.json" \
    --fc3-index "$RUN_DIR/governance_capsule_index.json" \
    --require-full-system \
    --out "$RUN_DIR/full_system_participation.json"

cat > "$RUN_DIR/fc3_governance_reinit_run_manifest.json" <<EOF
{
  "schema_version": "turingosv4.true_suite.fc3_governance_reinit_run.v1",
  "run_id": "$RUN_ID",
  "git_head": "$(cd "$PROJECT_ROOT" && git rev-parse HEAD)",
  "entrypoint": "scripts/run_true_suite_fc3_governance_reinit_current_kernel.sh",
  "init_provider": "$INIT_PROVIDER",
  "runtime_repo": "$RUN_DIR/runtime_repo",
  "cas": "$RUN_DIR/cas",
  "genesis_report": "$RUN_DIR/genesis_report.json",
  "chaintape_jsonl": "$RUN_DIR/chaintape.jsonl",
  "governance_capsule_index": "$RUN_DIR/governance_capsule_index.json",
  "replay_report": "$RUN_DIR/fc3_replay_report.json",
  "full_system_augmentation_manifest": "$RUN_DIR/full_system_augmentation_manifest.json",
  "full_system_participation": "$RUN_DIR/full_system_participation.json",
  "notes": [
    "FC3 evidence is typed ChainTape/CAS runtime evidence, not handover or dashboard evidence",
    "This FC3 true-suite sample also appends FC1 WorkTx and market participation rows on the same ChainTape",
    "ArchitectAI and Veto-AI are represented by runtime system txs",
    "ReinitRequest and ReinitBoot are tape-visible and replay-verified"
  ]
}
EOF

echo "TRUE-SUITE FC3 governance/re-init evidence: $RUN_DIR"
