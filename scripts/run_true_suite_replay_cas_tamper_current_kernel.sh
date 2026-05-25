#!/usr/bin/env bash
# True-suite replay/CAS tamper current-kernel evidence runner.
#
# Produces the `replay_cas_tamper_repair_current` artifact shape declared in
# tests/fixtures/liveness/realworld_liveness_coverage.toml.

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RUN_ID="${1:-replay_cas_tamper_current_$(date -u +%Y%m%dT%H%M%SZ)}"
RUN_ID="${RUN_ID#handover/evidence/true_suite/}"
RUN_ROOT="$PROJECT_ROOT/handover/evidence/true_suite/$RUN_ID"
RUN_DIR="$RUN_ROOT/replay_cas"

if [[ -e "$RUN_DIR" ]]; then
    echo "ERROR: evidence directory already exists: $RUN_DIR" >&2
    exit 2
fi

mkdir -p "$RUN_ROOT"

if [[ -n "$(cd "$PROJECT_ROOT" && git status --porcelain | grep -vE '^\?\? handover/evidence/' | head -1)" ]]; then
    echo "ERROR: working tree has non-evidence changes; run /runner-preflight before evidence runners" >&2
    exit 3
fi

echo "[build] cargo build --release --bin turingos --bin verify_chaintape --bin boot_cli_current_kernel_fresh --bin audit_tape_tamper"
(cd "$PROJECT_ROOT" && cargo build --release \
    --bin turingos \
    --bin verify_chaintape \
    --bin boot_cli_current_kernel_fresh \
    --bin audit_tape_tamper)

TURINGOS="$PROJECT_ROOT/target/release/turingos"
BOOT_HELPER="$PROJECT_ROOT/target/release/boot_cli_current_kernel_fresh"
TAMPER="$PROJECT_ROOT/target/release/audit_tape_tamper"
BIN_DIR="$PROJECT_ROOT/target/release"

echo "[init] turingos init --project $RUN_DIR"
"$TURINGOS" init --project "$RUN_DIR" --template proof --provider siliconflow

echo "[boot] current runtime ChainTape boot + resume tick"
"$BOOT_HELPER" \
    --runtime-repo "$RUN_DIR/runtime_repo" \
    --cas "$RUN_DIR/cas" \
    --run-id "$RUN_ID" \
    --constitution "$PROJECT_ROOT/constitution.md"

cp "$RUN_DIR/runtime_repo/genesis_report.json" "$RUN_DIR/genesis_report.json"

# System-only true-suite runs have no deployed agents, but audit_tape_tamper
# requires the modern manifest envelope shape.
printf '{"agents":{}}\n' > "$RUN_DIR/runtime_repo/agent_pubkeys.json"

echo "[verify] public turingos verify chaintape"
TURINGOS_BIN_DIR="$BIN_DIR" "$TURINGOS" verify chaintape \
    --repo "$RUN_DIR/runtime_repo" \
    --cas "$RUN_DIR/cas" \
    --run-id "$RUN_ID" \
    --out "$RUN_DIR/replay_report.json"

echo "[tamper] audit_tape_tamper over temp forks"
"$TAMPER" \
    --runtime-repo "$RUN_DIR/runtime_repo" \
    --cas-dir "$RUN_DIR/cas" \
    --agent-pubkeys "$RUN_DIR/runtime_repo/agent_pubkeys.json" \
    --pinned-pubkeys "$RUN_DIR/runtime_repo/pinned_pubkeys.json" \
    --genesis "$RUN_DIR/runtime_repo/genesis_report.json" \
    --constitution "$PROJECT_ROOT/constitution.md" \
    --alignment-dir "$PROJECT_ROOT/handover/alignment" \
    --tamper-dir "$RUN_DIR/tamper_work" \
    --out "$RUN_DIR/tamper_report.json"

echo "[verify] original tape still verifies after tamper forks"
TURINGOS_BIN_DIR="$BIN_DIR" "$TURINGOS" verify chaintape \
    --repo "$RUN_DIR/runtime_repo" \
    --cas "$RUN_DIR/cas" \
    --run-id "$RUN_ID" \
    --out "$RUN_DIR/post_tamper_replay_report.json"

python3 - "$PROJECT_ROOT" "$RUN_DIR" <<'PY'
import hashlib
import json
import sys
from pathlib import Path

project = Path(sys.argv[1])
run_dir = Path(sys.argv[2])

def load(name):
    with (run_dir / name).open("r", encoding="utf-8") as f:
        return json.load(f)

replay = load("replay_report.json")
post = load("post_tamper_replay_report.json")
tamper = load("tamper_report.json")
genesis = load("genesis_report.json")
agent_pubkeys = json.loads((run_dir / "runtime_repo" / "agent_pubkeys.json").read_text())

for label, report in [("replay", replay), ("post_tamper_replay", post)]:
    if report.get("l4_entries", 0) < 3:
        raise SystemExit(f"{label}: expected at least 3 L4 entries")
    for key in [
        "ledger_root_verified",
        "system_signatures_verified",
        "state_reconstructed",
        "economic_state_reconstructed",
        "cas_payloads_retrievable",
        "agent_signatures_verified",
        "proposal_telemetry_cas_retrievable",
    ]:
        if report.get(key) is not True:
            raise SystemExit(f"{label}: replay indicator {key} did not pass")

if tamper.get("detected_count") != 3 or tamper.get("expected") != 3:
    raise SystemExit("tamper report did not detect 3/3 corruptions")
if tamper.get("all_detected") is not True:
    raise SystemExit("tamper report all_detected is not true")
for row in tamper.get("tamper_results", []):
    if row.get("detected") is not True:
        raise SystemExit(f"tamper row did not detect corruption: {row.get('label')}")

constitution_hash = hashlib.sha256((project / "constitution.md").read_bytes()).hexdigest()
if genesis.get("constitution_hash") != constitution_hash:
    raise SystemExit("genesis_report constitution_hash does not match live constitution.md")
if agent_pubkeys != {"agents": {}}:
    raise SystemExit("runtime_repo/agent_pubkeys.json is not an explicit empty agent manifest")
PY

cat > "$RUN_DIR/replay_cas_run_manifest.json" <<EOF
{
  "schema_version": "turingosv4.true_suite.replay_cas_tamper_current.v1",
  "run_id": "$RUN_ID",
  "git_head": "$(cd "$PROJECT_ROOT" && git rev-parse HEAD)",
  "entrypoint": "scripts/run_true_suite_replay_cas_tamper_current_kernel.sh",
  "runtime_repo": "$RUN_DIR/runtime_repo",
  "cas": "$RUN_DIR/cas",
  "genesis_report": "$RUN_DIR/genesis_report.json",
  "replay_report": "$RUN_DIR/replay_report.json",
  "tamper_report": "$RUN_DIR/tamper_report.json",
  "post_tamper_replay_report": "$RUN_DIR/post_tamper_replay_report.json",
  "notes": [
    "fresh evidence is generated through public turingos init",
    "current runtime ChainTape boot helper emits boot/resume L4 rows",
    "public turingos verify chaintape reconstructs the original tape before and after tamper forks",
    "audit_tape_tamper corrupts only temporary forks and must detect 3/3 corruptions"
  ]
}
EOF

echo "TRUE-SUITE replay/CAS tamper evidence: $RUN_DIR"
