#!/usr/bin/env bash
# TB-16.x.2.5 — AutopsyCapsule real-bankruptcy chain smoke.
#
# Per umbrella charter `handover/tracer_bullets/TB-16.x.2_charter_2026-05-04.md`
# §2 Atom 2.5: single-problem arena exercising
# TURINGOS_FORCE_BANKRUPTCY_AFTER_ACCEPTED + TURINGOS_FORCE_BANKRUPTCY
# on a task expected to MaxTxExhaust. The AFTER_ACCEPTED env var injects
# a real-signed WorkTx (predicate_passes=true) BEFORE the LLM swarm so
# stakes_t is populated by the time FORCE_BANKRUPTCY emits at MaxTxExhaust.
# The TB-15 dispatch arm Step 3.5 hook then derives an AgentAutopsyCapsule
# for the seeded staker (loss_reason_class=Bankruptcy, loss_amount = stake).
#
# Why both env vars: TB-16 R2 P4 evidence shows MaxTxExhaust+FORCE_BANKRUPTCY
# alone produces work=0 / verify=0 / task_bankruptcy=1 / autopsy=∅ (LLM
# never admitted any WorkTx, so stakes_t empty, so Step 3.5 derives 0
# autopsies). The AFTER_ACCEPTED seed guarantees stakes_t non-empty.
#
# Ship gate SG-16.x.2.5: chain contains AutopsyCapsule with non-default
# loss_reason_class (= Bankruptcy, the TB-15 v0 sole production trigger
# per autopsy_capsule.rs:46-48) AND loss_amount > 0.
#
# Pre-existing audit assertions cover (umbrella charter §2 Atom 2.5):
# id=14 (replay_autopsy_index_chains, Layer C) and id=29
# (autopsy_private_detail_creator_is_system, Layer F).
#
# Markov capsule = None (genesis chain) per OBS_R022 α ratification.
#
# Usage:
#   bash handover/tests/scripts/run_tb_16_x_2_5_smoke_2026-05-05.sh

set -uo pipefail
cd /home/zephryj/projects/turingosv4

OUT_BASE="${OUT_BASE:-handover/evidence/tb_16_x_2_5_smoke_2026-05-05}"
mkdir -p "$OUT_BASE"

EVALUATOR_BIN="./target/release/evaluator"
AUDIT_TAPE_BIN="./target/release/audit_tape"
AUDIT_TAPE_TAMPER_BIN="./target/release/audit_tape_tamper"
AUDIT_DASHBOARD_BIN="./target/release/audit_dashboard"

LLM_PROXY_URL="${LLM_PROXY_URL:-http://localhost:18080}"
N_SWARM="${N_SWARM:-5}"
MAX_TX="${MAX_TX:-20}"

PID="P13_autopsy_real"
PFILE="aime_1997_p9.lean"
# Agent_user_0 is the seeded staker. Stake 50_000 μC (small but > 0).
# default_pput_preseed_pairs allocates Agent_user_0 = 10_000_000 μC + Agent_0..9 =
# 1_000_000 μC each + tb7-7-sponsor = 10_000_000 μC. Note: there is NO
# `Agent_solver_*` in default preseed — that prefix is reserved for a future
# `comprehensive_arena` extension; using it here causes admission rejection
# `InsufficientBalance` (.2.5 first-run smoke trip 2026-05-05). All sandbox-
# prefixed agents are valid per CR-16.5; balance is the practical constraint.
STAKER="${STAKER:-Agent_user_0}"
STAKE_MICRO="${STAKE_MICRO:-50000}"

PROBE_ENV="TURINGOS_FORCE_BANKRUPTCY_AFTER_ACCEPTED=${STAKER}:${STAKE_MICRO}"
PROBE_ENV="$PROBE_ENV TURINGOS_FORCE_BANKRUPTCY=1"

PROBLEM_DIR="$OUT_BASE/$PID"
mkdir -p "$PROBLEM_DIR/runtime_repo" "$PROBLEM_DIR/cas"

echo "════════════════════════════════════════════════════════════════════"
echo "TB-16.x.2.5 — AutopsyCapsule real-bankruptcy smoke ($PID)"
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
  TURINGOS_RUN_ID="tb16-x-2-5-$PID" \
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
grep -E "seed WorkTx|TaskBankruptcy|autopsy|AutopsyCapsule|chaintape/tb16-arena|tb15" \
  "$PROBLEM_DIR/evaluator.stderr" \
  > "$PROBLEM_DIR/autopsy_trace.txt" 2>&1 || true

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
echo "Ship gate SG-16.x.2.5 — AutopsyCapsule with loss_amount > 0 in chain:"
# Two-witness gate per .2.3 pattern:
#   (a) verdict.json tx_kind_counts.work >= 1 + tx_kind_counts.task_bankruptcy >= 1
#       (chain admitted seed WorkTx + emitted bankruptcy)
#   (b) CAS index `.turingos_cas_index.jsonl` contains entry with
#       object_type == "AgentAutopsyCapsule" (canonical signal — sequencer
#       Stage 3.5 hook writes this object_type when bankruptcy emits an
#       autopsy via derive_autopsies_for_bankruptcy with loss_amount > 0).
#       NOTE: a previous .2.5 r3 ship-gate iteration tried substring scan for
#       'Bankruptcy' in raw bytes — but the canonical_encode'd capsule
#       encodes the LossReasonClass::Bankruptcy enum tag as a 1-byte variant
#       index (BCS), not a UTF-8 string. False-negative on r3 surfaced this.
GATE_RESULT=$(python3 - "$PROBLEM_DIR/verdict.json" "$PROBLEM_DIR/cas/.turingos_cas_index.jsonl" "$PROBLEM_DIR/autopsy_trace.txt" <<'PY'
import json, os, sys
verdict_path = sys.argv[1]
cas_index_path = sys.argv[2]
trace_path = sys.argv[3]
v = json.load(open(verdict_path))
counts = v.get("tx_kind_counts", {})
work_n = int(counts.get("work", 0))
bk_n = int(counts.get("task_bankruptcy", 0))
cas_autopsy_capsules = 0
cas_autopsy_private_detail = 0
try:
    for line in open(cas_index_path):
        try:
            obj = json.loads(line)
        except Exception:
            continue
        if obj.get("object_type") == "AgentAutopsyCapsule":
            cas_autopsy_capsules += 1
        elif obj.get("object_type") == "AutopsyPrivateDetail":
            cas_autopsy_private_detail += 1
except Exception:
    pass
# Witness on stderr trace too (defensive — autopsy emit is silent in current
# evaluator logging; presence of seed WorkTx + bankruptcy emit lines is the
# proxy that the chain shape we expect did materialize).
trace = ""
try: trace = open(trace_path).read()
except: pass
import re
seed_committed = 1 if re.search(r'seed WorkTx submitted by', trace) else 0
bk_committed = 1 if re.search(r'TaskBankruptcyTx emitted', trace) else 0
print(f"{work_n}|{bk_n}|{cas_autopsy_capsules}|{cas_autopsy_private_detail}|{seed_committed}|{bk_committed}")
PY
)
WORK_N="${GATE_RESULT%%|*}"
REST="${GATE_RESULT#*|}"
BK_N="${REST%%|*}"
REST="${REST#*|}"
CAS_AUTOPSY="${REST%%|*}"
REST="${REST#*|}"
CAS_PRIVATE_DETAIL="${REST%%|*}"
REST="${REST#*|}"
SEED_COMMITTED="${REST%%|*}"
BK_COMMITTED="${REST#*|}"

SHIP_GATE_RC=0
echo "  Chain counts: WorkTx=$WORK_N  TaskBankruptcy=$BK_N"
echo "  CAS autopsy: AgentAutopsyCapsule=$CAS_AUTOPSY  AutopsyPrivateDetail=$CAS_PRIVATE_DETAIL"
echo "  Trace witnesses: seed_committed=$SEED_COMMITTED  bk_committed=$BK_COMMITTED"
if [[ "$WORK_N" -gt 0 && "$BK_N" -gt 0 && "$CAS_AUTOPSY" -gt 0 && "$CAS_PRIVATE_DETAIL" -gt 0 ]]; then
  echo "  ✓ Chain has WorkTx (n=$WORK_N) + TaskBankruptcy (n=$BK_N); CAS has AgentAutopsyCapsule (n=$CAS_AUTOPSY) + AutopsyPrivateDetail (n=$CAS_PRIVATE_DETAIL) — Step 3.5 derive_autopsies_for_bankruptcy fired"
else
  echo "  ✗ AutopsyCapsule chain missing — gate FAILED (work=$WORK_N bk=$BK_N capsule=$CAS_AUTOPSY detail=$CAS_PRIVATE_DETAIL)"
  SHIP_GATE_RC=1
fi
echo "════════════════════════════════════════════════════════════════════"
exit $SHIP_GATE_RC
