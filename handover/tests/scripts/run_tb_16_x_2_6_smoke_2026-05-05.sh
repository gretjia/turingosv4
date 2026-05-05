#!/usr/bin/env bash
# TB-16.x.2.6 — Combined arena run smoke (13-of-13 tx kinds).
#
# Per umbrella charter `handover/tracer_bullets/TB-16.x.2_charter_2026-05-04.md`
# §2 Atom 2.6: single arena run combining all FORCE_* env vars from
# sub-atoms 2.1 + 2.2 + 2.3 + 2.4 + 2.5 to materialize a chain
# covering 13-of-13 architect tx kinds + Boltzmann RUNTIME exercise +
# AutopsyCapsule real-bankruptcy in one continuing chain. β fully
# realized for TB-16.
#
# Per .fix r1 hardening carry-over: set -euo pipefail, RC capture,
# ALLOW_REUSE refusal, AT_LOG capture for audit_tape.
#
# Tx kinds expected (13-of-13 architect set per TB-16 charter §1):
#   work             — boltzmann seed (4×) + .2.5 seed (1×) + LLM swarm OMEGA-Confirm
#   verify           — LLM OMEGA-Confirm
#   challenge        — FORCE_CHALLENGER on OMEGA-Confirm path
#   task_open        — preseed
#   escrow_lock      — preseed
#   complete_set_mint — FORCE_COMPLETE_SET_SEED
#   complete_set_redeem — FORCE_REDEEM (after Bankrupt resolution)
#   market_seed      — FORCE_COMPLETE_SET_SEED
#   finalize_reward  — auto on OMEGA-Confirm
#   challenge_resolve — FORCE_CHALLENGE_RESOLVE
#   terminal_summary — MaxTxExhausted (always fires at run cleanup)
#   task_expire      — FORCE_EXPIRE
#   task_bankruptcy  — FORCE_BANKRUPTCY
#
# Critical dependency: verify, challenge, finalize_reward, challenge_resolve
# all REQUIRE the LLM swarm to OMEGA-Confirm at least one proof. So the
# combined run uses a known-solvable problem (mathd_algebra_171.lean from
# .2.2's evidence) — not an exhaustion problem.
#
# Budget check (Agent_user_0 preseed = 10M μC):
#   .2.3 CompleteSetSeed: 1M (collateral) + 250k (mint debit) = 1.25M
#   .2.5 seed WorkTx:     50k (stake)
#   .2.4 boltzmann seeds: 4 × 25k = 100k (stake)
#   Total:                ~1.4M μC ≪ 10M (safe headroom)
#
# Markov capsule = None (genesis chain) per OBS_R022 α ratification.
#
# Usage:
#   bash handover/tests/scripts/run_tb_16_x_2_6_smoke_2026-05-05.sh

set -euo pipefail
cd /home/zephryj/projects/turingosv4

OUT_BASE="${OUT_BASE:-handover/evidence/tb_16_x_2_6_smoke_2026-05-05}"

EVALUATOR_BIN="./target/release/evaluator"
AUDIT_TAPE_BIN="./target/release/audit_tape"
AUDIT_TAPE_TAMPER_BIN="./target/release/audit_tape_tamper"
AUDIT_DASHBOARD_BIN="./target/release/audit_dashboard"

LLM_PROXY_URL="${LLM_PROXY_URL:-http://localhost:18080}"
N_SWARM="${N_SWARM:-5}"
MAX_TX="${MAX_TX:-20}"

PID="P14_comprehensive"
PFILE="mathd_algebra_171.lean"

# Single staker for all hooks: Agent_user_0 (10M μC preseed; ample budget).
STAKER="${STAKER:-Agent_user_0}"

# .2.3 CompleteSetRedeem env vars
COMPLETE_SET_AMOUNT="${COMPLETE_SET_AMOUNT:-1000000}"
REDEEM_OUTCOME="${REDEEM_OUTCOME:-no}"
REDEEM_UNITS="${REDEEM_UNITS:-250000}"

# .2.4 Boltzmann env vars (BOLTZMANN_SEED=12345 verified diversity per .fix r1 r4)
BOLTZMANN_COUNT="${BOLTZMANN_COUNT:-4}"
BOLTZMANN_STAKE_PER="${BOLTZMANN_STAKE_PER:-25000}"
BOLTZMANN_EPSILON_NUM="${BOLTZMANN_EPSILON_NUM:-9}"
BOLTZMANN_EPSILON_DEN="${BOLTZMANN_EPSILON_DEN:-10}"
BOLTZMANN_SEED="${BOLTZMANN_SEED:-12345}"

# .2.5 AutopsyCapsule seed env var
BANKRUPTCY_AFTER_ACCEPTED_STAKE="${BANKRUPTCY_AFTER_ACCEPTED_STAKE:-50000}"

# .2.2 ChallengeResolve via challenge-window scheduler
CHALLENGER="${CHALLENGER:-Agent_3}"

# Compose the probe env. Order is independent (each FORCE_* is its own
# evaluator block) but documented top-to-bottom for readability.
PROBE_ENV=""
PROBE_ENV="$PROBE_ENV TURINGOS_COMPLETE_SET_SEED=${STAKER}:${COMPLETE_SET_AMOUNT}"
PROBE_ENV="$PROBE_ENV TURINGOS_FORCE_BANKRUPTCY=1"
PROBE_ENV="$PROBE_ENV TURINGOS_FORCE_REDEEM=${STAKER}:${REDEEM_OUTCOME}:${REDEEM_UNITS}"
PROBE_ENV="$PROBE_ENV TURINGOS_FORCE_BANKRUPTCY_AFTER_ACCEPTED=${STAKER}:${BANKRUPTCY_AFTER_ACCEPTED_STAKE}"
PROBE_ENV="$PROBE_ENV TURINGOS_FORCE_BOLTZMANN_SEED_WORKTXS=${STAKER}:${BOLTZMANN_COUNT}:${BOLTZMANN_STAKE_PER}"
PROBE_ENV="$PROBE_ENV TURINGOS_FORCE_EXPIRE=1"
PROBE_ENV="$PROBE_ENV TURINGOS_FORCE_CHALLENGER=${CHALLENGER}"
PROBE_ENV="$PROBE_ENV TURINGOS_FORCE_CHALLENGE_RESOLVE=1"

PROBLEM_DIR="$OUT_BASE/$PID"
if [[ -e "$PROBLEM_DIR/verdict.json" && "${ALLOW_REUSE:-0}" != "1" ]]; then
  echo "ERROR: $PROBLEM_DIR/verdict.json already exists. Use ALLOW_REUSE=1 to override or pass OUT_BASE=<fresh>." >&2
  exit 2
fi
mkdir -p "$PROBLEM_DIR/runtime_repo" "$PROBLEM_DIR/cas"

echo "════════════════════════════════════════════════════════════════════"
echo "TB-16.x.2.6 — Combined arena run smoke ($PID; 13-of-13 target)"
echo "════════════════════════════════════════════════════════════════════"
echo "  N_SWARM=$N_SWARM  MAX_TX=$MAX_TX  LLM_PROXY=$LLM_PROXY_URL"
echo "  Probe: $PROBE_ENV"
echo "  Out: $PROBLEM_DIR"
echo "  Start: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
echo

T0=$(date +%s)
RC=0
env $PROBE_ENV \
  BOLTZMANN_EPSILON_NUM="$BOLTZMANN_EPSILON_NUM" \
  BOLTZMANN_EPSILON_DEN="$BOLTZMANN_EPSILON_DEN" \
  BOLTZMANN_SEED="$BOLTZMANN_SEED" \
  TURINGOS_USER_TASK_MODE=1 \
  TURINGOS_CHAINTAPE_PRESEED=1 \
  TURINGOS_USER_TASK_BOUNTY_MICRO=200000 \
  TURINGOS_CHAINTAPE_PATH="$PROBLEM_DIR/runtime_repo" \
  TURINGOS_CAS_PATH="$PROBLEM_DIR/cas" \
  TURINGOS_RUN_ID="tb16-x-2-6-$PID" \
  LLM_PROXY_URL="$LLM_PROXY_URL" \
  MAX_TRANSACTIONS="$MAX_TX" \
  CONDITION="n${N_SWARM}" \
  RUST_LOG="${RUST_LOG:-info}" \
  "$EVALUATOR_BIN" "$PFILE" 2> "$PROBLEM_DIR/evaluator.stderr" 1> "$PROBLEM_DIR/evaluator.stdout" || RC=$?
T1=$(date +%s)
ELAPSED=$((T1 - T0))
echo "  evaluator: rc=$RC  elapsed=${ELAPSED}s"
if [[ "$RC" -ne 0 ]]; then
  echo "ERROR: evaluator exited with rc=$RC; aborting smoke." >&2
  exit "$RC"
fi

grep "^PPUT_RESULT:" "$PROBLEM_DIR/evaluator.stdout" | tail -1 > "$PROBLEM_DIR/pput_result.json"
if [[ -s "$PROBLEM_DIR/pput_result.json" ]]; then
  sed -i 's/^PPUT_RESULT://' "$PROBLEM_DIR/pput_result.json"
fi
grep -E "boltzmann seed|seed WorkTx|TaskBankruptcy|TaskExpire|ChallengeResolve|CompleteSetMint|CompleteSetRedeem|MarketSeed|FORCE_|chaintape/tb16-arena|tb15|autopsy|FinalizeReward|verify_tx|OMEGA-Confirm|challenge tx" \
  "$PROBLEM_DIR/evaluator.stderr" \
  > "$PROBLEM_DIR/combined_trace.txt" 2>&1 || true

echo "  audit_tape..."
AT_LOG=$("$AUDIT_TAPE_BIN" \
  --runtime-repo "$PROBLEM_DIR/runtime_repo" \
  --cas-dir "$PROBLEM_DIR/cas" \
  --agent-pubkeys "$PROBLEM_DIR/runtime_repo/agent_pubkeys.json" \
  --pinned-pubkeys "$PROBLEM_DIR/runtime_repo/pinned_pubkeys.json" \
  --genesis genesis_payload.toml \
  --constitution constitution.md \
  --alignment-dir handover/alignment \
  --out "$PROBLEM_DIR/verdict.json" 2>&1)
echo "$AT_LOG" | tail -1

echo "  audit_tape replay..."
AT_REPLAY_LOG=$("$AUDIT_TAPE_BIN" \
  --runtime-repo "$PROBLEM_DIR/runtime_repo" \
  --cas-dir "$PROBLEM_DIR/cas" \
  --agent-pubkeys "$PROBLEM_DIR/runtime_repo/agent_pubkeys.json" \
  --pinned-pubkeys "$PROBLEM_DIR/runtime_repo/pinned_pubkeys.json" \
  --genesis genesis_payload.toml \
  --constitution constitution.md \
  --alignment-dir handover/alignment \
  --out "$PROBLEM_DIR/verdict_replay.json" 2>&1)
echo "$AT_REPLAY_LOG" | tail -1
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
echo "Ship gate SG-16.x.2.6 — 13-of-13 architect tx kinds in single chain:"
GATE_RESULT=$(python3 - "$PROBLEM_DIR/verdict.json" "$PROBLEM_DIR/cas/.turingos_cas_index.jsonl" <<'PY'
import json, os, sys
v = json.load(open(sys.argv[1]))
counts = v.get("tx_kind_counts", {})
expected_13 = [
    "work", "verify", "challenge", "task_open", "escrow_lock",
    "complete_set_mint", "complete_set_redeem", "market_seed",
    "finalize_reward", "challenge_resolve", "terminal_summary",
    "task_expire", "task_bankruptcy",
]
hits = []
misses = []
for k in expected_13:
    n = int(counts.get(k, 0))
    (hits if n > 0 else misses).append((k, n))
hit_count = len(hits)
# id=43 entropy gate (Boltzmann RUNTIME exercise)
id43 = next((a for a in v.get("assertions", []) if a.get("id") == 43), {})
id43_result = id43.get("result", "Missing")
# CAS autopsy presence (.2.5 mechanism)
cas_capsules = 0
try:
    for line in open(sys.argv[2]):
        try: obj = json.loads(line)
        except: continue
        if obj.get("object_type") == "AgentAutopsyCapsule":
            cas_capsules += 1
except: pass
verdict = v.get("verdict", "?")
print(f"hit={hit_count}/13|miss={','.join(k for k,_ in misses)}|verdict={verdict}|id43={id43_result}|cas_capsules={cas_capsules}")
PY
)
HIT="${GATE_RESULT%%|*}"
REST="${GATE_RESULT#*|}"
MISS="${REST%%|*}"
REST="${REST#*|}"
VERDICT_AUDIT="${REST%%|*}"
REST="${REST#*|}"
ID43="${REST%%|*}"
CAS_CAPSULES="${REST#*|}"

SHIP_GATE_RC=0
echo "  $HIT  miss: $MISS  audit_verdict=$VERDICT_AUDIT  id43=$ID43  cas_capsules=$CAS_CAPSULES"
HIT_N="${HIT##*=}"; HIT_N="${HIT_N%%/*}"
if [[ "$HIT_N" == "13" && "$VERDICT_AUDIT" == "PROCEED" && "$ID43" == "Pass" && "$CAS_CAPSULES" -gt 0 ]]; then
  echo "  ✓ Single chain covers 13-of-13 tx kinds + audit PROCEED + id=43 Pass + autopsy emitted"
else
  echo "  ✗ SG-16.x.2.6 FAILED — see misses + verdict + id43 + cas_capsules above"
  SHIP_GATE_RC=1
fi
echo "════════════════════════════════════════════════════════════════════"
exit $SHIP_GATE_RC
