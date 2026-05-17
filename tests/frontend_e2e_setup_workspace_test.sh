#!/usr/bin/env bash
# Usage:
#   tests/frontend_e2e_setup_workspace_test.sh
#
# Purpose:
#   Meta-test for W4.3 workspace setup/teardown machinery. Runs 7 assertions
#   that verify the setup and teardown scripts work correctly end-to-end.
#   This is the unit-test for the W4.3 setup infrastructure itself —
#   NOT the §6a Phase 7 Chrome-driven E2E test.
#
# Assertions:
#   1. Setup script exits 0 (default workspace path)
#   2. Workspace scaffold files exist (genesis_payload.toml, agent_pubkeys.json,
#      runtime_repo/, cas/)
#   3. agent_001 (Solver) is registered in the workspace
#   4. turingos task open can bootstrap a ChainTape (≥2 txs) using the workspace
#      NOTE: lean_market run-task flags used: --chaintape --problem --bounty
#            --max-tx --max-secs. The lean evaluator may fail (Lean not installed)
#            but ChainTape bootstrap (TaskOpen + at least one L4 tx) still succeeds.
#            lean_market exits with evaluator exit code; we verify the ChainTape
#            was created rather than asserting exit 0 from the shell-out.
#   5. ChainTape contains ≥2 txs (agent_audit_trail.jsonl has ≥2 lines)
#   6. Teardown exits 0 and workspace is removed
#   7. Teardown safety guard: refuses /etc with exit 1
#
# Deviation from W4.3 spec §"meta-test step 4":
#   The spec listed --workspace and --agent-id flags which do not exist in the
#   lean_market run-task CLI (lean_market ignores unknown flags but --bounty 100
#   is below the 100,000 micro minimum and would cause exit 2 before evaluator
#   spawn). We use the actual working flags. See cmd_task_open.rs + lean_market.rs.
#
# FC-trace: FC2-N16 (boot/genesis gate — verifies TaskOpen bootstrap machinery)
# Risk class: Class 0-1 (test driver bash; no Rust source changes)

set -euo pipefail

# ── Repo root ──────────────────────────────────────────────────────────────
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
SETUP_SCRIPT="${SCRIPT_DIR}/frontend_e2e_setup_workspace.sh"
TEARDOWN_SCRIPT="${SCRIPT_DIR}/frontend_e2e_teardown_workspace.sh"
TURINGOS_BIN="${REPO_ROOT}/target/debug/turingos"

# Test workspace path (under tmp/ for safety)
TEST_WORKSPACE="tmp/meta_test_workspace_$$"

# Counter
PASS=0
FAIL=0

_pass() {
  local msg="$1"
  PASS=$((PASS + 1))
  echo "[meta-test] PASS ${PASS}: ${msg}"
}

_fail() {
  local msg="$1"
  FAIL=$((FAIL + 1))
  echo "[meta-test] FAIL: ${msg}" >&2
}

_assert_eq() {
  local label="$1" expected="$2" actual="$3"
  if [[ "${actual}" == "${expected}" ]]; then
    _pass "${label}: '${actual}'"
  else
    _fail "${label}: expected '${expected}', got '${actual}'"
  fi
}

# ─────────────────────────────────────────────────────────────────────────
# Assertion 1 — Setup exits 0
# ─────────────────────────────────────────────────────────────────────────
echo ""
echo "[meta-test] === Assertion 1: setup exits 0 ==="
if bash "${SETUP_SCRIPT}" "${TEST_WORKSPACE}" >/dev/null 2>&1; then
  _pass "setup exits 0"
else
  _fail "setup exited non-zero"
fi

# ─────────────────────────────────────────────────────────────────────────
# Assertion 2 — Scaffold files exist
# ─────────────────────────────────────────────────────────────────────────
echo ""
echo "[meta-test] === Assertion 2: scaffold files exist ==="
WS_ABS="${REPO_ROOT}/${TEST_WORKSPACE}"
SCAFFOLD_OK=1
for f in "genesis_payload.toml" "agent_pubkeys.json"; do
  if [[ -f "${WS_ABS}/${f}" ]]; then
    echo "[meta-test]   found: ${f}"
  else
    _fail "missing scaffold file: ${WS_ABS}/${f}"
    SCAFFOLD_OK=0
  fi
done
for d in "runtime_repo" "cas"; do
  if [[ -d "${WS_ABS}/${d}" ]]; then
    echo "[meta-test]   found dir: ${d}"
  else
    _fail "missing scaffold dir: ${WS_ABS}/${d}"
    SCAFFOLD_OK=0
  fi
done
if [[ "${SCAFFOLD_OK}" == "1" ]]; then
  _pass "all 4 scaffold entries exist (genesis_payload.toml, agent_pubkeys.json, runtime_repo/, cas/)"
fi

# ─────────────────────────────────────────────────────────────────────────
# Assertion 3 — agent_001 is registered
# ─────────────────────────────────────────────────────────────────────────
echo ""
echo "[meta-test] === Assertion 3: agent_001 registered ==="
if "${TURINGOS_BIN}" agent view --workspace "${WS_ABS}" --id agent_001 2>/dev/null | grep -q "agent_001"; then
  _pass "agent_001 found in workspace registry"
else
  _fail "agent_001 not found in workspace (turingos agent view returned no match)"
fi

# Also verify agent_pubkeys.json contains agent_001 literally
if grep -q "agent_001" "${WS_ABS}/agent_pubkeys.json" 2>/dev/null; then
  echo "[meta-test]   agent_pubkeys.json contains 'agent_001' (confirmed)"
else
  _fail "agent_pubkeys.json does not contain agent_001 string"
fi

# ─────────────────────────────────────────────────────────────────────────
# Assertion 4 — turingos task open can bootstrap a fresh ChainTape
#
# Uses actual lean_market flags (--chaintape, --problem, --bounty, --max-tx,
# --max-secs). Creates a synthetic Lean problem file with a valid theorem
# declaration so the evaluator can parse it. The evaluator will fail at the
# Lean verify step (Lean not installed in test env) but ChainTape bootstrap
# (TaskOpen + first tx) completes before Lean is invoked.
# We accept non-zero exit from task open; the assertion is that the
# ChainTape directory was created.
# ─────────────────────────────────────────────────────────────────────────
echo ""
echo "[meta-test] === Assertion 4: turingos task open bootstraps a ChainTape ==="
CHAINTAPE_DIR="${WS_ABS}/task_tape_$$"
PROBLEM_FILE="$(mktemp /tmp/meta_test_problem_XXXXXX.lean)"
printf 'theorem meta_test_thm : 1 + 1 = 2 := by sorry\n' > "${PROBLEM_FILE}"

echo "[meta-test]   invoking: turingos task open --chaintape <ws>/task_tape --problem <tmp.lean> --bounty 100000 --max-tx 2 --max-secs 8"
# Accept non-zero exit: evaluator may fail on Lean step but ChainTape is created.
# Use timeout to cap wall-clock regardless of --max-secs. Redirect stderr to
# /dev/null to suppress evaluator rejection-evidence I/O noise.
timeout 20 "${TURINGOS_BIN}" task open \
  --chaintape "${CHAINTAPE_DIR}" \
  --problem   "${PROBLEM_FILE}" \
  --bounty    100000 \
  --max-tx    2 \
  --max-secs  8 >/dev/null 2>&1 || true

rm -f "${PROBLEM_FILE}"

# Check the ChainTape directory was created
if [[ -d "${CHAINTAPE_DIR}" ]]; then
  _pass "ChainTape directory created at ${CHAINTAPE_DIR}"
else
  _fail "ChainTape directory was NOT created — lean_market/evaluator may have failed before bootstrap"
fi

# ─────────────────────────────────────────────────────────────────────────
# Assertion 5 — ChainTape contains ≥2 txs
#
# Check agent_audit_trail.jsonl has at least 2 lines (TaskOpen + 1 more tx).
# Uses the jsonl file produced by the evaluator during ChainTape preseed.
# ─────────────────────────────────────────────────────────────────────────
echo ""
echo "[meta-test] === Assertion 5: ChainTape contains ≥2 txs ==="
TRAIL_FILE="${CHAINTAPE_DIR}/agent_audit_trail.jsonl"
if [[ -f "${TRAIL_FILE}" ]]; then
  TX_COUNT=$(wc -l < "${TRAIL_FILE}" | tr -d ' ')
  echo "[meta-test]   agent_audit_trail.jsonl has ${TX_COUNT} entries"
  # Print first two tx_ids for evidence
  grep -o '"tx_id":"[^"]*"' "${TRAIL_FILE}" | head -3 | while read -r tx; do
    echo "[meta-test]   ${tx}"
  done
  if [[ "${TX_COUNT}" -ge 2 ]]; then
    _pass "ChainTape has ${TX_COUNT} ≥ 2 txs"
  else
    _fail "ChainTape has only ${TX_COUNT} tx(s) — expected ≥2 (TaskOpen + EscrowLock or synthetic WorkTx)"
  fi
else
  # agent_audit_trail.jsonl is written by the evaluator's chaintape preseed path.
  # If it doesn't exist, fall back to checking if the chaintape dir is non-empty.
  if [[ -d "${CHAINTAPE_DIR}" ]] && [[ -n "$(ls -A "${CHAINTAPE_DIR}" 2>/dev/null)" ]]; then
    echo "[meta-test]   NOTE: agent_audit_trail.jsonl not found (evaluator may have failed before preseed write)"
    echo "[meta-test]   ChainTape dir contents: $(ls "${CHAINTAPE_DIR}" | tr '\n' ' ')"
    _fail "agent_audit_trail.jsonl missing — ChainTape preseed did not complete"
  else
    _fail "ChainTape directory is empty or missing — evaluator failed before bootstrap"
  fi
fi

# ─────────────────────────────────────────────────────────────────────────
# Assertion 6 — Teardown exits 0 and workspace is removed
# ─────────────────────────────────────────────────────────────────────────
echo ""
echo "[meta-test] === Assertion 6: teardown exits 0 and workspace removed ==="
if bash "${TEARDOWN_SCRIPT}" "${TEST_WORKSPACE}" >/dev/null 2>&1; then
  TEARDOWN_EXIT=0
else
  TEARDOWN_EXIT=$?
fi

if [[ "${TEARDOWN_EXIT}" == "0" ]]; then
  _pass "teardown exited 0"
else
  _fail "teardown exited ${TEARDOWN_EXIT} (expected 0)"
fi

if [[ ! -d "${WS_ABS}" ]]; then
  _pass "workspace directory removed by teardown"
else
  _fail "workspace directory still exists after teardown: ${WS_ABS}"
fi

# ─────────────────────────────────────────────────────────────────────────
# Assertion 7 — Teardown safety guard refuses /etc
# ─────────────────────────────────────────────────────────────────────────
echo ""
echo "[meta-test] === Assertion 7: teardown safety guard refuses /etc ==="
SAFETY_EXIT=0
bash "${TEARDOWN_SCRIPT}" /etc 2>/dev/null && SAFETY_EXIT=0 || SAFETY_EXIT=$?

if [[ "${SAFETY_EXIT}" == "1" ]]; then
  _pass "teardown correctly refused /etc with exit 1"
else
  _fail "teardown should have refused /etc with exit 1 but got exit ${SAFETY_EXIT}"
fi

# ─────────────────────────────────────────────────────────────────────────
# Summary
# ─────────────────────────────────────────────────────────────────────────
echo ""
echo "[meta-test] =============================="
echo "[meta-test] Results: ${PASS} passed, ${FAIL} failed"
if [[ "${FAIL}" -gt 0 ]]; then
  echo "[meta-test] SOME ASSERTIONS FAILED" >&2
  exit 1
fi
echo "[meta-test] all ${PASS} assertions passed"
exit 0
