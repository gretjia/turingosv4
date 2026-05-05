#!/usr/bin/env bash
# TB-16.x.2.4 — Multi-WorkTx + Boltzmann RUNTIME exercise smoke.
#
# Per umbrella charter `handover/tracer_bullets/TB-16.x.2_charter_2026-05-04.md`
# §2 Atom 2.4: single-problem arena exercising
# TURINGOS_FORCE_BOLTZMANN_SEED_WORKTXS to inject N (≥3) real-signed
# WorkTxs serially, each with ProposalTelemetry.parent_tx derived from
# boltzmann_select_parent_v2(snap.price_index, snap.mask_set, ...) on
# the current bus snapshot at proposal time, with fallback to the most
# recent produced WorkTx tx_id when the v2 selector returns None
# (price_index empty for iter 0).
#
# Closes the missing R3 "Boltzmann RUNTIME exercise" gap. Audit
# assertion id=43 (boltzmann_parent_selection_diversity, Layer E
# supplemental, added in .2.5 commit) verifies Shannon entropy ≥ 0.25
# across same-task ProposalTelemetry.parent_tx distribution.
#
# Ship gate SG-16.x.2.4: chain contains ≥3 WorkTxs with diverse
# parent_selection_entropy ≥ 0.5 (per Art II.2.1 alarm threshold 0.25;
# tighter than threshold for headroom).
#
# Markov capsule = None (genesis chain) per OBS_R022 α ratification.
#
# Class 3 = MANDATORY Codex + Gemini dual external audit before merge
# per feedback_dual_audit + feedback_risk_class_audit. Smoke is
# pre-audit; audit follows successful smoke.
#
# Usage:
#   bash handover/tests/scripts/run_tb_16_x_2_4_smoke_2026-05-05.sh

set -uo pipefail
cd /home/zephryj/projects/turingosv4

OUT_BASE="${OUT_BASE:-handover/evidence/tb_16_x_2_4_smoke_2026-05-05}"
mkdir -p "$OUT_BASE"

EVALUATOR_BIN="./target/release/evaluator"
AUDIT_TAPE_BIN="./target/release/audit_tape"
AUDIT_TAPE_TAMPER_BIN="./target/release/audit_tape_tamper"
AUDIT_DASHBOARD_BIN="./target/release/audit_dashboard"

LLM_PROXY_URL="${LLM_PROXY_URL:-http://localhost:18080}"
N_SWARM="${N_SWARM:-5}"
MAX_TX="${MAX_TX:-20}"

PID="P12_boltzmann_runtime"
PFILE="aime_1997_p9.lean"
# Agent_user_0 has 10M μC preseed; 4 WorkTxs × 25_000 μC each = 100k μC < balance.
# count=4 (≥3 + headroom for entropy diversity) per SG-16.x.2.4.
STAKER="${STAKER:-Agent_user_0}"
COUNT="${COUNT:-4}"
STAKE_MICRO_PER="${STAKE_MICRO_PER:-25000}"

PROBE_ENV="TURINGOS_FORCE_BOLTZMANN_SEED_WORKTXS=${STAKER}:${COUNT}:${STAKE_MICRO_PER}"

PROBLEM_DIR="$OUT_BASE/$PID"
mkdir -p "$PROBLEM_DIR/runtime_repo" "$PROBLEM_DIR/cas"

echo "════════════════════════════════════════════════════════════════════"
echo "TB-16.x.2.4 — Multi-WorkTx + Boltzmann RUNTIME smoke ($PID)"
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
  TURINGOS_RUN_ID="tb16-x-2-4-$PID" \
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
grep -E "boltzmann seed|FORCE_BOLTZMANN|chaintape/tb16-arena" \
  "$PROBLEM_DIR/evaluator.stderr" \
  > "$PROBLEM_DIR/boltzmann_trace.txt" 2>&1 || true

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
echo "Ship gate SG-16.x.2.4 — Multi-WorkTx + Boltzmann parent diversity:"
GATE_RESULT=$(python3 - "$PROBLEM_DIR/verdict.json" <<'PY'
import json, sys
v = json.load(open(sys.argv[1]))
counts = v.get("tx_kind_counts", {})
work_n = int(counts.get("work", 0))
id43 = next((a for a in v.get("assertions", []) if a.get("id") == 43), None)
id43_result = (id43 or {}).get("result", "Missing")
id43_detail = (id43 or {}).get("detail", "")
print(f"{work_n}|{id43_result}|{id43_detail}")
PY
)
WORK_N="${GATE_RESULT%%|*}"
REST="${GATE_RESULT#*|}"
ID43_RESULT="${REST%%|*}"
ID43_DETAIL="${REST#*|}"

SHIP_GATE_RC=0
echo "  Chain WorkTx count: $WORK_N"
echo "  id=43 boltzmann_parent_selection_diversity: $ID43_RESULT"
[[ -n "$ID43_DETAIL" ]] && echo "    detail: $ID43_DETAIL"
if [[ "$WORK_N" -ge 3 && "$ID43_RESULT" == "Pass" ]]; then
  echo "  ✓ Chain has ≥3 WorkTxs (n=$WORK_N) AND id=43 entropy gate Pass"
else
  echo "  ✗ Either WorkTx count < 3 OR id=43 not Pass — gate FAILED"
  SHIP_GATE_RC=1
fi
echo "════════════════════════════════════════════════════════════════════"
exit $SHIP_GATE_RC
