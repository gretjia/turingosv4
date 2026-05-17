#!/usr/bin/env bash
# Usage:
#   tests/frontend_e2e_teardown_workspace.sh [WORKSPACE_DIR]
#
# Purpose:
#   Removes the TuringOS workspace directory created by
#   frontend_e2e_setup_workspace.sh. Includes a strict safety check that
#   refuses to delete any path outside the repo's tmp/ subdirectory or /tmp/.
#
# Arguments:
#   $1  Workspace directory path. Default: tmp/phase7_workspace (relative to
#       repo root). Must resolve to a path under <repo>/tmp/ or /tmp/.
#
# Safety guard:
#   If the resolved absolute path does NOT start with <repo>/tmp/ or /tmp/,
#   the script REFUSES and exits with code 1. This prevents accidental
#   deletion of home directories, system files, or arbitrary paths.
#
# FC-trace: FC2-N16 (cleanup complement to setup; no ChainTape write)
# Risk class: Class 0-1 (bash cleanup; no Rust changes)

set -euo pipefail

# ── Repo root ──────────────────────────────────────────────────────────────
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

# ── Workspace dir ──────────────────────────────────────────────────────────
WORKSPACE_ARG="${1:-tmp/phase7_workspace}"

# ─────────────────────────────────────────────────────────────────────────
# Step 1 — Verify the directory exists
# ─────────────────────────────────────────────────────────────────────────
case "${WORKSPACE_ARG}" in
  /*) WORKSPACE_UNRESOLVED="${WORKSPACE_ARG}" ;;
  *)  WORKSPACE_UNRESOLVED="${REPO_ROOT}/${WORKSPACE_ARG}" ;;
esac

if [[ ! -e "${WORKSPACE_UNRESOLVED}" ]]; then
  echo "[teardown] nothing to remove: ${WORKSPACE_UNRESOLVED} does not exist"
  exit 0
fi

# ─────────────────────────────────────────────────────────────────────────
# Step 2 — Resolve to canonical absolute path and safety-check
# ─────────────────────────────────────────────────────────────────────────
# Use realpath to resolve symlinks and ../ components.
WORKSPACE="$(realpath "${WORKSPACE_UNRESOLVED}")"

ALLOWED_REPO_TMP="${REPO_ROOT}/tmp"
ALLOWED_SYSTEM_TMP="/tmp"

_safe_to_remove() {
  local path="$1"
  # Allow paths under <repo>/tmp/ (trailing slash ensures prefix match on dir)
  if [[ "${path}" == "${ALLOWED_REPO_TMP}" || "${path}" == "${ALLOWED_REPO_TMP}/"* ]]; then
    return 0
  fi
  # Allow paths under /tmp/ (system temp)
  if [[ "${path}" == "${ALLOWED_SYSTEM_TMP}" || "${path}" == "${ALLOWED_SYSTEM_TMP}/"* ]]; then
    return 0
  fi
  return 1
}

if ! _safe_to_remove "${WORKSPACE}"; then
  echo "[teardown] REFUSED: '${WORKSPACE}' is not under ${ALLOWED_REPO_TMP}/ or ${ALLOWED_SYSTEM_TMP}/" >&2
  echo "[teardown] Teardown only removes paths inside the repo's tmp/ directory or /tmp/." >&2
  echo "[teardown] To remove a custom workspace, delete it manually." >&2
  exit 1
fi

# ─────────────────────────────────────────────────────────────────────────
# Additional guard: never remove the allowed root itself (e.g. /tmp alone)
# ─────────────────────────────────────────────────────────────────────────
if [[ "${WORKSPACE}" == "${ALLOWED_REPO_TMP}" || "${WORKSPACE}" == "${ALLOWED_SYSTEM_TMP}" ]]; then
  echo "[teardown] REFUSED: refusing to remove root temp directory '${WORKSPACE}'" >&2
  echo "[teardown] Pass a subdirectory path, not the root temp dir." >&2
  exit 1
fi

# ─────────────────────────────────────────────────────────────────────────
# Step 3 — Remove
# ─────────────────────────────────────────────────────────────────────────
echo "[teardown] removing: ${WORKSPACE}"
rm -rf "${WORKSPACE}"

# ─────────────────────────────────────────────────────────────────────────
# Step 4 — Confirm
# ─────────────────────────────────────────────────────────────────────────
echo "[teardown] removed ${WORKSPACE}"
exit 0
