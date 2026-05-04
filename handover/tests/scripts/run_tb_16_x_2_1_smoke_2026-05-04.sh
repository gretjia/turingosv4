#!/usr/bin/env bash
# TB-16.x.2.1 — TaskExpire env-var trigger smoke.
#
# Per umbrella charter `handover/tracer_bullets/TB-16.x.2_charter_2026-05-04.md`
# §2 Atom 2.1: single-problem arena exercising TURINGOS_FORCE_EXPIRE=1 on a
# task expected to MaxTxExhaust. Ship gate SG-16.x.2.1: TaskExpire tx kind
# present in arena-produced tx kinds (raises 9-of-13 → 10-of-13).
#
# Combined-path coverage (FORCE_EXPIRE + FORCE_BANKRUPTCY) is gated by
# COMBINE_BANKRUPTCY=1 (off by default for the cleanest Deadline-reason
# reading of the smoke chain).
#
# Usage:
#   bash handover/tests/scripts/run_tb_16_x_2_1_smoke_2026-05-04.sh

set -uo pipefail
cd /home/zephryj/projects/turingosv4

OUT_BASE="${OUT_BASE:-handover/evidence/tb_16_x_2_1_smoke_2026-05-04}"
mkdir -p "$OUT_BASE"

EVALUATOR_BIN="./target/release/evaluator"
AUDIT_TAPE_BIN="./target/release/audit_tape"
AUDIT_TAPE_TAMPER_BIN="./target/release/audit_tape_tamper"
AUDIT_DASHBOARD_BIN="./target/release/audit_dashboard"

LLM_PROXY_URL="${LLM_PROXY_URL:-http://localhost:18080}"
N_SWARM="${N_SWARM:-5}"
MAX_TX="${MAX_TX:-20}"
COMBINE_BANKRUPTCY="${COMBINE_BANKRUPTCY:-0}"

# Markov capsule = None per FC2 Boot + Markov chain genesis semantic.
# This sub-atom's smoke run is constitutionally a genesis chain (fresh
# runtime_repo + fresh cas; no `previous_capsule_cid` claim in its bytes).
# TB-16.x.fix (architect OBS_R022 Option α RATIFIED 2026-05-04):
# `--markov-pointer` is now optional; absence ≡ genesis chain.
# The previous `NULL_MARKOV_POINTER` plumbing is no longer needed.

PID="P9_force_expire"
PFILE="aime_1997_p9.lean"
PROBE_ENV="TURINGOS_FORCE_EXPIRE=1"
if [[ "$COMBINE_BANKRUPTCY" == "1" ]]; then
  PID="P9_force_expire_bankrupt"
  PROBE_ENV="$PROBE_ENV TURINGOS_FORCE_BANKRUPTCY=Agent_0"
fi
PROBLEM_DIR="$OUT_BASE/$PID"
mkdir -p "$PROBLEM_DIR/runtime_repo" "$PROBLEM_DIR/cas"

echo "════════════════════════════════════════════════════════════════════"
echo "TB-16.x.2.1 — TaskExpire smoke ($PID)"
echo "════════════════════════════════════════════════════════════════════"
echo "  N_SWARM=$N_SWARM  MAX_TX=$MAX_TX  LLM_PROXY=$LLM_PROXY_URL"
echo "  Probe: $PROBE_ENV"
echo "  Out: $PROBLEM_DIR"
echo "  Start: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
echo

T0=$(date +%s)
env $PROBE_ENV \
  TURINGOS_USER_TASK_MODE=1 \
  TURINGOS_CHAINTAPE_PRESEED=1 \
  TURINGOS_USER_TASK_BOUNTY_MICRO=200000 \
  TURINGOS_CHAINTAPE_PATH="$PROBLEM_DIR/runtime_repo" \
  TURINGOS_CAS_PATH="$PROBLEM_DIR/cas" \
  TURINGOS_RUN_ID="tb16-x-2-1-$PID" \
  LLM_PROXY_URL="$LLM_PROXY_URL" \
  MAX_TRANSACTIONS="$MAX_TX" \
  CONDITION="n${N_SWARM}" \
  RUST_LOG="${RUST_LOG:-info}" \
  "$EVALUATOR_BIN" "$PFILE" 2> "$PROBLEM_DIR/evaluator.stderr" 1> "$PROBLEM_DIR/evaluator.stdout"
RC=$?
T1=$(date +%s)
ELAPSED=$((T1 - T0))
echo "  evaluator: rc=$RC  elapsed=${ELAPSED}s"

grep "^PPUT_RESULT:" "$PROBLEM_DIR/evaluator.stdout" | tail -1 > "$PROBLEM_DIR/pput_result.json"
if [[ -s "$PROBLEM_DIR/pput_result.json" ]]; then
  sed -i 's/^PPUT_RESULT://' "$PROBLEM_DIR/pput_result.json"
fi
grep -E "TaskExpire|tb11_emit_expire|chaintape/tb16-arena" "$PROBLEM_DIR/evaluator.stderr" \
  > "$PROBLEM_DIR/expire_trace.txt" 2>&1 || true

echo "  audit_tape..."
"$AUDIT_TAPE_BIN" \
  --runtime-repo "$PROBLEM_DIR/runtime_repo" \
  --cas-dir "$PROBLEM_DIR/cas" \
  --agent-pubkeys "$PROBLEM_DIR/runtime_repo/agent_pubkeys.json" \
  --pinned-pubkeys "$PROBLEM_DIR/runtime_repo/pinned_pubkeys.json" \
  --genesis genesis_payload.toml \
  --constitution constitution.md \
  --alignment-dir handover/alignment \
  --out "$PROBLEM_DIR/verdict.json" 2>&1 | tail -1

echo "  audit_tape replay..."
"$AUDIT_TAPE_BIN" \
  --runtime-repo "$PROBLEM_DIR/runtime_repo" \
  --cas-dir "$PROBLEM_DIR/cas" \
  --agent-pubkeys "$PROBLEM_DIR/runtime_repo/agent_pubkeys.json" \
  --pinned-pubkeys "$PROBLEM_DIR/runtime_repo/pinned_pubkeys.json" \
  --genesis genesis_payload.toml \
  --constitution constitution.md \
  --alignment-dir handover/alignment \
  --out "$PROBLEM_DIR/verdict_replay.json" 2>&1 | tail -1
if cmp -s "$PROBLEM_DIR/verdict.json" "$PROBLEM_DIR/verdict_replay.json"; then
  echo "  ✓ replay byte-identical"
else
  echo "  ✗ replay diverged"
fi

echo "  audit_tape_tamper..."
"$AUDIT_TAPE_TAMPER_BIN" \
  --runtime-repo "$PROBLEM_DIR/runtime_repo" \
  --cas-dir "$PROBLEM_DIR/cas" \
  --agent-pubkeys "$PROBLEM_DIR/runtime_repo/agent_pubkeys.json" \
  --pinned-pubkeys "$PROBLEM_DIR/runtime_repo/pinned_pubkeys.json" \
  --genesis genesis_payload.toml \
  --constitution constitution.md \
  --alignment-dir handover/alignment \
  --tamper-dir "$PROBLEM_DIR/tamper" \
  --out "$PROBLEM_DIR/tamper_report.json" 2>&1 | tail -1

echo "  audit_dashboard..."
"$AUDIT_DASHBOARD_BIN" \
  --repo "$PROBLEM_DIR/runtime_repo" \
  --cas "$PROBLEM_DIR/cas" \
  --out "$PROBLEM_DIR/dashboard.txt" 2>&1 | tail -1

echo
echo "Ship gate SG-16.x.2.1 — TaskExpire tx kind in chain:"
if grep -q '"task_expire"' "$PROBLEM_DIR/verdict.json" 2>/dev/null \
  || grep -qi 'TaskExpire\|task_expire' "$PROBLEM_DIR/dashboard.txt" 2>/dev/null; then
  echo "  ✓ TaskExpire tx kind detected"
else
  echo "  ✗ TaskExpire tx kind NOT detected — gate FAILED"
fi
echo "════════════════════════════════════════════════════════════════════"
