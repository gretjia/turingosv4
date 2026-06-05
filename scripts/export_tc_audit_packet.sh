#!/usr/bin/env bash
set -euo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT="$REPO/handover/reports/TC_FULL_AUDIT_PACKET_MANIFEST_2026-06-04.md"

hash_file() {
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1" | awk '{print $1}'
  else
    shasum -a 256 "$1" | awk '{print $1}'
  fi
}

require_file() {
  if [ ! -f "$1" ]; then
    echo "missing required file: $1" >&2
    exit 1
  fi
}

require_marker() {
  local marker="$1"
  if ! grep -q "$marker" "$REPORT"; then
    echo "audit packet manifest missing marker: $marker" >&2
    exit 1
  fi
}

check_manifest() {
  require_file "$REPORT"
  require_file "$REPO/constitution.md"
  require_file "$REPO/OBLIGATIONS.md"
  require_file "$REPO/handover/directives/TC_002_BOOT_TRUST_ROOT_MANIFEST.md"
  require_file "$REPO/handover/reports/TC_Q_DIRTY_TREE_PRESERVATION_2026-06-04.yaml"
  require_file "$REPO/src/runtime/tc_crash_matrix.rs"
  require_file "$REPO/src/runtime/tc_universal_witness.rs"
  require_file "$REPO/src/runtime/g0_completeness.rs"
  require_file "$REPO/handover/directives/tc_prereg_2026-06-04/PARITY_SCHEMA.yaml"

  for marker in \
    "source_sha" \
    "worktree_status" \
    "constitution_hash" \
    "boot_manifest_hash" \
    "path_b_ref_schema" \
    "replay_commands" \
    "crash_matrix_results" \
    "universal_witnesses" \
    "g0_manifest" \
    "scheduler_traces" \
    "parity_schema" \
    "clean_context_audits" \
    "obligation_witness" \
    "dirty_tree_preservation"; do
    require_marker "$marker"
  done

  for excluded in RUN_STATUS.json STAGE_A_POWER_GATE.json prereg.json; do
    require_marker "$excluded"
    if grep -q "reliability_input: $excluded" "$REPORT"; then
      echo "metadata artifact appears as reliability input: $excluded" >&2
      exit 1
    fi
  done

  echo "TC-AUDIT-PACKET-CHECK PASS"
  echo "source_sha=$(git -C "$REPO" rev-parse HEAD)"
  echo "constitution_hash=$(hash_file "$REPO/constitution.md")"
  echo "boot_manifest_hash=$(hash_file "$REPO/handover/directives/TC_002_BOOT_TRUST_ROOT_MANIFEST.md")"
}

export_packet() {
  local out_dir="${1:-$REPO/target/tc_audit_packet}"
  mkdir -p "$out_dir"
  cp "$REPORT" "$out_dir/TC_FULL_AUDIT_PACKET_MANIFEST_2026-06-04.md"
  cp "$REPO/OBLIGATIONS.md" "$out_dir/OBLIGATIONS.md"
  cp "$REPO/constitution.md" "$out_dir/constitution.md"
  cp "$REPO/handover/directives/TC_OPERATIONALIZATION_FULL_EXECUTION_PLAN_2026-06-04.md" "$out_dir/TC_OPERATIONALIZATION_FULL_EXECUTION_PLAN_2026-06-04.md"
  git -C "$REPO" status --short > "$out_dir/worktree_status.txt"
  git -C "$REPO" rev-parse HEAD > "$out_dir/source_sha.txt"
  hash_file "$REPO/constitution.md" > "$out_dir/constitution.sha256"
  echo "TC-AUDIT-PACKET-EXPORT $out_dir"
}

case "${1:---check}" in
  --check)
    check_manifest
    ;;
  --export)
    check_manifest >/dev/null
    export_packet "${2:-}"
    ;;
  *)
    echo "usage: $0 --check | --export [out_dir]" >&2
    exit 2
    ;;
esac
