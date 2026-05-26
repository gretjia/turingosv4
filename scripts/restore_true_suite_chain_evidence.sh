#!/usr/bin/env bash
# Restore packaged true-suite ChainTape/CAS stores and re-run public verifier.
#
# This is intentionally a post-package check: it proves the committed tarballs
# are enough to reconstruct the runtime repo and CAS view used by `turingos
# verify chaintape`, without depending on loose nested .git stores.

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RUN_ID=""
RUN_ROOT=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        --run-id)
            RUN_ID="${2:?--run-id requires a value}"
            shift 2
            ;;
        --run-root)
            RUN_ROOT="${2:?--run-root requires a value}"
            shift 2
            ;;
        -h|--help)
            cat <<'EOF'
restore_true_suite_chain_evidence.sh --run-id <RUN_ID>
restore_true_suite_chain_evidence.sh --run-root <PATH>

For every direct child evidence domain containing packaged runtime_repo/cas
tarballs, restore them into a temp directory and write:
  <domain>/restore_replay_report.json

For the fc3 domain, also write:
  <domain>/fc3_restore_replay_report.json
EOF
            exit 0
            ;;
        *)
            RUN_ID="${1#handover/evidence/true_suite/}"
            shift
            ;;
    esac
done

if [[ -z "$RUN_ROOT" ]]; then
    if [[ -z "$RUN_ID" ]]; then
        echo "ERROR: provide --run-id or --run-root" >&2
        exit 2
    fi
    RUN_ID="${RUN_ID#handover/evidence/true_suite/}"
    RUN_ROOT="$PROJECT_ROOT/handover/evidence/true_suite/$RUN_ID"
fi

if [[ ! -d "$RUN_ROOT" ]]; then
    echo "ERROR: run root is not a directory: $RUN_ROOT" >&2
    exit 3
fi

RUN_ID="${RUN_ID:-$(basename "$RUN_ROOT")}"
TURINGOS="$PROJECT_ROOT/target/release/turingos"
if [[ ! -x "$TURINGOS" ]]; then
    (cd "$PROJECT_ROOT" && cargo build --release --bin turingos)
fi

restored_count=0
for domain_dir in "$RUN_ROOT"/*; do
    [[ -d "$domain_dir" ]] || continue
    [[ "$(basename "$domain_dir")" != "broad_batch" ]] || continue

    runtime_git="$domain_dir/runtime_repo.dotgit.tar.gz"
    cas_git="$domain_dir/cas.dotgit.tar.gz"
    [[ -f "$runtime_git" && -f "$cas_git" ]] || continue
    replay_report="$domain_dir/replay_report.json"
    if [[ ! -f "$replay_report" && "$(basename "$domain_dir")" == "fc3" ]]; then
        replay_report="$domain_dir/fc3_replay_report.json"
    fi
    [[ -f "$replay_report" ]] || continue

    domain_run_id="$(python3 - "$replay_report" "$RUN_ID" <<'PY'
import json
import sys

path, fallback = sys.argv[1:3]
try:
    with open(path, "r", encoding="utf-8") as f:
        payload = json.load(f)
except Exception:
    print(fallback)
else:
    value = payload.get("run_id")
    print(value if isinstance(value, str) and value else fallback)
PY
)"

    out_report="$domain_dir/restore_replay_report.json"
    tmp_root="$(mktemp -d)"
    cleanup() {
        rm -rf "$tmp_root"
    }
    trap cleanup EXIT

    mkdir -p "$tmp_root/runtime_repo" "$tmp_root/cas"
    if [[ -f "$domain_dir/runtime_repo.worktree.tar.gz" ]]; then
        tar -xzf "$domain_dir/runtime_repo.worktree.tar.gz" -C "$tmp_root/runtime_repo"
    fi
    tar -xzf "$runtime_git" -C "$tmp_root/runtime_repo"

    if [[ -f "$domain_dir/cas.worktree.tar.gz" ]]; then
        tar -xzf "$domain_dir/cas.worktree.tar.gz" -C "$tmp_root/cas"
    fi
    tar -xzf "$cas_git" -C "$tmp_root/cas"

    "$TURINGOS" verify chaintape \
        --repo "$tmp_root/runtime_repo" \
        --cas "$tmp_root/cas" \
        --run-id "$domain_run_id" \
        --out "$out_report"

    if [[ "$(basename "$domain_dir")" == "fc3" ]]; then
        cp "$out_report" "$domain_dir/fc3_restore_replay_report.json"
    fi

    cleanup
    trap - EXIT
    restored_count=$((restored_count + 1))
done

echo "TRUE-SUITE restore replay reports written: $restored_count"
