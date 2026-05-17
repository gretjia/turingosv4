#!/usr/bin/env bash
# Usage:
#   tests/frontend_e2e_setup_workspace.sh [WORKSPACE_DIR]
#
# Purpose:
#   Idempotent setup script that creates a TuringOS workspace for the §6a
#   Chrome-driven verifier. Produces all state required for the verifier to:
#     1. Start turingos_web with TURINGOS_WEB_WORKSPACE=<DIR>
#     2. Verify Page 2 DOM contains "agent_001" (Solver role)
#     3. Trigger a write-path that shells out to `lean_market run-task`
#        (ChainTape advances by at least 2 txs: TaskOpen + EscrowLock)
#
# Arguments:
#   $1  Workspace directory path. Default: tmp/phase7_workspace (relative to
#       repo root). MUST be under tmp/ or /tmp/ for teardown safety.
#
# Outputs (printed to stdout at exit):
#   TURINGOS_WEB_WORKSPACE=<absolute-path>
#   TURINGOS_WEB_BIN=<absolute-path-to-turingos_web>
#
# FC-trace: FC2-N16 (boot/genesis gate — workspace creation is the canonical
#   on_init precondition for TaskOpen + EscrowLock chain advancement)
# Risk class: Class 0-1 (bash driver; no Rust source changes; no production
#   write path modified)

set -euo pipefail

# ── Repo root ──────────────────────────────────────────────────────────────
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

# ── Workspace dir ──────────────────────────────────────────────────────────
WORKSPACE_ARG="${1:-tmp/phase7_workspace}"
# Resolve to absolute path (the dir may not exist yet)
case "${WORKSPACE_ARG}" in
  /*) WORKSPACE="${WORKSPACE_ARG}" ;;
  *)  WORKSPACE="${REPO_ROOT}/${WORKSPACE_ARG}" ;;
esac

# ── Binary paths ──────────────────────────────────────────────────────────
TURINGOS_BIN="${REPO_ROOT}/target/debug/turingos"
TURINGOS_WEB_BIN="${REPO_ROOT}/target/debug/turingos_web"
FRONTEND_DIST="${REPO_ROOT}/frontend/dist/main.js"

echo "[setup] workspace target: ${WORKSPACE}"

# ─────────────────────────────────────────────────────────────────────────
# Step 1 — Build the CLI binary (skip if target is newer than source)
# ─────────────────────────────────────────────────────────────────────────
_needs_rebuild_cli() {
  if [[ ! -f "${TURINGOS_BIN}" ]]; then return 0; fi
  # If any src/bin/turingos* file is newer than the binary, rebuild
  if find "${REPO_ROOT}/src/bin" -name "*.rs" -newer "${TURINGOS_BIN}" \
       -not -path "*/target/*" | grep -q .; then
    return 0
  fi
  return 1
}

if _needs_rebuild_cli; then
  echo "[setup] building turingos CLI..."
  (cd "${REPO_ROOT}" && cargo build --bin turingos --quiet)
  echo "[setup] turingos CLI built: ${TURINGOS_BIN}"
else
  echo "[setup] turingos CLI up to date: ${TURINGOS_BIN}"
fi

# ─────────────────────────────────────────────────────────────────────────
# Step 2 — Build the web binary
# ─────────────────────────────────────────────────────────────────────────
_needs_rebuild_web() {
  if [[ ! -f "${TURINGOS_WEB_BIN}" ]]; then return 0; fi
  if find "${REPO_ROOT}/src" -name "*.rs" -newer "${TURINGOS_WEB_BIN}" \
       -not -path "*/target/*" | grep -q .; then
    return 0
  fi
  return 1
}

if _needs_rebuild_web; then
  echo "[setup] building turingos_web..."
  (cd "${REPO_ROOT}" && cargo build --bin turingos_web --features web --quiet)
  echo "[setup] turingos_web built: ${TURINGOS_WEB_BIN}"
else
  echo "[setup] turingos_web up to date: ${TURINGOS_WEB_BIN}"
fi

# ─────────────────────────────────────────────────────────────────────────
# Step 3 — Build the frontend (skip if dist/main.js exists and is newer
#           than the TypeScript sources)
# ─────────────────────────────────────────────────────────────────────────
_needs_rebuild_frontend() {
  if [[ ! -f "${FRONTEND_DIST}" ]]; then return 0; fi
  if find "${REPO_ROOT}/frontend/src" -name "*.ts" -newer "${FRONTEND_DIST}" \
       2>/dev/null | grep -q .; then
    return 0
  fi
  return 1
}

if _needs_rebuild_frontend; then
  echo "[setup] building frontend..."
  (cd "${REPO_ROOT}/frontend" && npm run build --silent)
  echo "[setup] frontend built: ${FRONTEND_DIST}"
else
  echo "[setup] frontend up to date: ${FRONTEND_DIST}"
fi

# ─────────────────────────────────────────────────────────────────────────
# Step 4 — Create workspace dir or integrity-check if it already exists
# ─────────────────────────────────────────────────────────────────────────
if [[ -d "${WORKSPACE}" ]]; then
  echo "[setup] workspace exists at ${WORKSPACE} — checking integrity"
  # Verify the four canonical scaffold files are present
  MISSING=()
  [[ -d "${WORKSPACE}/runtime_repo" ]] || MISSING+=("runtime_repo/")
  [[ -d "${WORKSPACE}/cas" ]]          || MISSING+=("cas/")
  [[ -f "${WORKSPACE}/genesis_payload.toml" ]] || MISSING+=("genesis_payload.toml")
  [[ -f "${WORKSPACE}/agent_pubkeys.json" ]]   || MISSING+=("agent_pubkeys.json")
  if [[ ${#MISSING[@]} -gt 0 ]]; then
    echo "[setup] ERROR: workspace exists but is missing scaffold files:" >&2
    for f in "${MISSING[@]}"; do echo "  missing: ${WORKSPACE}/${f}" >&2; done
    echo "[setup] Remove ${WORKSPACE} manually and re-run, or pass a fresh path." >&2
    exit 1
  fi
  echo "[setup] integrity OK — all scaffold files present"
  # Verify agent_001 is registered
  if "${TURINGOS_BIN}" agent view --workspace "${WORKSPACE}" --id agent_001 >/dev/null 2>&1; then
    echo "[setup] agent_001 already registered — workspace ready"
    echo ""
    echo "TURINGOS_WEB_WORKSPACE=${WORKSPACE}"
    echo "TURINGOS_WEB_BIN=${TURINGOS_WEB_BIN}"
    exit 0
  else
    echo "[setup] WARNING: workspace exists but agent_001 not found — re-deploying"
  fi
else
  # ── Step 4 (new workspace): init ─────────────────────────────────────
  echo "[setup] creating workspace dir: ${WORKSPACE}"
  mkdir -p "$(dirname "${WORKSPACE}")"

  # Step 5 — turingos init --project <DIR>
  # NOTE: exact flag is --project (discovered from cmd_init.rs; NOT --workspace)
  echo "[setup] running: turingos init --project ${WORKSPACE} --template multi-agent"
  "${TURINGOS_BIN}" init --project "${WORKSPACE}" --template multi-agent
  echo "[setup] workspace initialized"
fi

# ─────────────────────────────────────────────────────────────────────────
# Step 6 — Deploy agent_001 (Solver role)
#
# turingos agent deploy requires --pubkey (64-char hex ed25519 public key).
# For E2E testing, we use a deterministic synthetic pubkey. The §6a verifier
# requires the AGENT REGISTRY to contain "agent_001" — the pubkey's validity
# is not checked at the filesystem level (sequencer admission would check it
# during a real on-chain run).
#
# Exact flags discovered from cmd_agent.rs:
#   --workspace <PATH>  --id <ID>  --pubkey <64HEX>  --role <ROLE>
# ─────────────────────────────────────────────────────────────────────────
AGENT_ID="agent_001"
AGENT_ROLE="Solver"
# Deterministic synthetic pubkey: 64 hex chars, all zeros except last digit = 1.
# This satisfies the 64-char hex validation in validate_pubkey() and is stable
# across re-runs (idempotent).
AGENT_PUBKEY="0000000000000000000000000000000000000000000000000000000000000001"

echo "[setup] deploying agent: --id ${AGENT_ID} --role ${AGENT_ROLE}"
"${TURINGOS_BIN}" agent deploy \
  --workspace "${WORKSPACE}" \
  --id        "${AGENT_ID}" \
  --pubkey    "${AGENT_PUBKEY}" \
  --role      "${AGENT_ROLE}"
echo "[setup] agent deployed"

# ─────────────────────────────────────────────────────────────────────────
# Step 7 — Print env vars for operator / verifier
# ─────────────────────────────────────────────────────────────────────────
echo ""
echo "[setup] === ENVIRONMENT VARS FOR VERIFIER ==="
echo "TURINGOS_WEB_WORKSPACE=${WORKSPACE}"
echo "TURINGOS_WEB_BIN=${TURINGOS_WEB_BIN}"
echo "[setup] === WORKSPACE CONTENTS ==="
ls -la "${WORKSPACE}"
echo ""
echo "[setup] DONE. Start the web server with:"
echo "  TURINGOS_WEB_WORKSPACE=${WORKSPACE} ${TURINGOS_WEB_BIN}"
exit 0
