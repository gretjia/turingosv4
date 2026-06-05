#!/usr/bin/env bash
set -euo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PACKET_REPORT="$REPO/handover/reports/TC_FULL_AUDIT_PACKET_MANIFEST_2026-06-04.md"
REPLAY_REPORT="$REPO/handover/reports/TC_CLEAN_CHECKOUT_REPLAY_2026-06-04.md"
NO_NETWORK="${TURINGOS_TC_REPLAY_NO_NETWORK:-1}"

require_file() {
  if [ ! -f "$1" ]; then
    echo "missing required file: $1" >&2
    exit 1
  fi
}

hash_file() {
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1" | awk '{print $1}'
  else
    shasum -a 256 "$1" | awk '{print $1}'
  fi
}

cargo_bin() {
  if [ -n "${CARGO:-}" ]; then
    printf '%s\n' "$CARGO"
  elif command -v cargo >/dev/null 2>&1; then
    command -v cargo
  elif [ -x "$HOME/.cargo/bin/cargo" ]; then
    printf '%s\n' "$HOME/.cargo/bin/cargo"
  else
    echo "cargo not found" >&2
    exit 1
  fi
}

require_marker() {
  local marker="$1"
  local file="$2"
  if ! grep -q "$marker" "$file"; then
    echo "missing marker '$marker' in $file" >&2
    exit 1
  fi
}

obl014_satisfied() {
  awk '
    /^## OBL-014:/ { in_obl=1 }
    /^## OBL-015:/ { in_obl=0 }
    in_obl && /- Status: satisfied/ { found=1 }
    END { exit found ? 0 : 1 }
  ' "$REPO/OBLIGATIONS.md"
}

run_detached_worktree_replay() {
  local sha tmp checkout cargo_path
  sha="$(git -C "$REPO" rev-parse HEAD)"
  tmp="$(mktemp -d "${TMPDIR:-/tmp}/turingos-tc-clean-replay.XXXXXX")"
  checkout="$tmp/checkout"
  cargo_path="$(cargo_bin)"
  cleanup() {
    git -C "$REPO" worktree remove --force "$checkout" >/dev/null 2>&1 || true
    rm -rf "$tmp"
  }
  trap cleanup EXIT

  git -C "$REPO" worktree add --detach "$checkout" "$sha" >/dev/null
  (
    cd "$checkout"
    TURINGOS_TC_REPLAY_NO_NETWORK=1 "$cargo_path" test \
      --test tc_crash_matrix \
      --test tc_universal_witnesses \
      --test tc_g0_completeness \
      --test tc_external_call_records \
      --test tc_lean_micro_state_contract \
      --test tc_audit_packet_export \
      --test tc_clean_checkout_replay_contract \
      --no-fail-fast
    bash scripts/export_tc_audit_packet.sh --check
  )
}

check_replay_contract() {
  require_file "$PACKET_REPORT"
  require_file "$REPLAY_REPORT"
  require_file "$REPO/OBLIGATIONS.md"
  require_marker "network_policy: disabled" "$REPLAY_REPORT"
  require_marker "llm_replay_policy: disabled" "$REPLAY_REPORT"
  require_marker "hash_compare: required" "$REPLAY_REPORT"
  require_marker "obligation_witness_required: OBL-ALL-CLOSED" "$REPLAY_REPORT"
  require_marker "obligation_witness_verdict: OBL-ALL-CLOSED" "$REPLAY_REPORT"
  require_marker "final_obligation_witness_verdict: OBL-ALL-CLOSED" "$PACKET_REPORT"

  if [ "$NO_NETWORK" != "1" ]; then
    echo "network must be disabled for clean checkout replay" >&2
    exit 1
  fi

  if ! obl014_satisfied; then
    echo "OBL-014 is not marked satisfied" >&2
    exit 1
  fi

  local packet_hash
  local replay_hash
  packet_hash="$(hash_file "$PACKET_REPORT")"
  replay_hash="$(hash_file "$REPLAY_REPORT")"
  test -n "$packet_hash"
  test -n "$replay_hash"

  run_detached_worktree_replay

  echo "TC-CLEAN-CHECKOUT-REPLAY-CHECK PASS"
  echo "packet_report_hash=$packet_hash"
  echo "replay_report_hash=$replay_hash"
}

case "${1:---check}" in
  --check)
    check_replay_contract
    ;;
  *)
    echo "usage: $0 --check" >&2
    exit 2
    ;;
esac
