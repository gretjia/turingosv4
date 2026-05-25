#!/usr/bin/env bash
# True-suite boot/CLI current-kernel evidence runner.
#
# Produces the `boot_cli_current_kernel_fresh` artifact shape declared in
# tests/fixtures/liveness/realworld_liveness_coverage.toml.

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RUN_ID="${1:-boot_cli_current_kernel_$(date -u +%Y%m%dT%H%M%SZ)}"
RUN_ID="${RUN_ID#handover/evidence/true_suite/}"
RUN_ROOT="$PROJECT_ROOT/handover/evidence/true_suite/$RUN_ID"
RUN_DIR="$RUN_ROOT/boot_cli"

if [[ -e "$RUN_DIR" ]]; then
    echo "ERROR: evidence directory already exists: $RUN_DIR" >&2
    exit 2
fi

mkdir -p "$RUN_ROOT"

if [[ -n "$(cd "$PROJECT_ROOT" && git status --porcelain | grep -vE '^\?\? handover/evidence/' | head -1)" ]]; then
    echo "ERROR: working tree has non-evidence changes; run /runner-preflight before evidence runners" >&2
    exit 3
fi

echo "[build] cargo build --release --bin turingos --bin verify_chaintape --bin boot_cli_current_kernel_fresh"
(cd "$PROJECT_ROOT" && cargo build --release --bin turingos --bin verify_chaintape --bin boot_cli_current_kernel_fresh)

TURINGOS="$PROJECT_ROOT/target/release/turingos"
HELPER="$PROJECT_ROOT/target/release/boot_cli_current_kernel_fresh"

echo "[init] turingos init --project $RUN_DIR"
"$TURINGOS" init --project "$RUN_DIR" --template proof --provider siliconflow

echo "[boot] current runtime boot + resume tick"
"$HELPER" \
    --runtime-repo "$RUN_DIR/runtime_repo" \
    --cas "$RUN_DIR/cas" \
    --run-id "$RUN_ID" \
    --constitution "$PROJECT_ROOT/constitution.md"

cp "$RUN_DIR/runtime_repo/genesis_report.json" "$RUN_DIR/genesis_report.json"

echo "[verify] turingos verify chaintape"
"$TURINGOS" verify chaintape \
    --repo "$RUN_DIR/runtime_repo" \
    --cas "$RUN_DIR/cas" \
    --run-id "$RUN_ID" \
    --out "$RUN_DIR/replay_report.json"

cat > "$RUN_DIR/boot_cli_current_kernel_manifest.json" <<EOF
{
  "schema_version": "turingosv4.true_suite.boot_cli_current_kernel.v1",
  "run_id": "$RUN_ID",
  "git_head": "$(cd "$PROJECT_ROOT" && git rev-parse HEAD)",
  "entrypoint": "scripts/run_true_suite_boot_cli_current_kernel.sh",
  "runtime_repo": "$RUN_DIR/runtime_repo",
  "cas": "$RUN_DIR/cas",
  "genesis_report": "$RUN_DIR/genesis_report.json",
  "replay_report": "$RUN_DIR/replay_report.json",
  "notes": [
    "turingos init is filesystem scaffold only",
    "boot helper calls current runtime ChainTape boot API",
    "resume path emits an additional system MapReduceTick",
    "verify chaintape is invoked through the public turingos CLI wrapper"
  ]
}
EOF

echo "TRUE-SUITE boot/CLI evidence: $RUN_DIR"
