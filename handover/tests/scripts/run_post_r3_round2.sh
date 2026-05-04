#!/usr/bin/env bash
# TB-16 post-R3 Round 2 — comprehensive mechanism RUNTIME exercise.
#
# Round 1 (post_r3_full_test/) covered FC1/FC2/FC3 STRUCTURALLY but did
# not exercise mechanisms 5 (Boltzmann) + 6 (info shielding) + 7 (typical
# error broadcast) at RUNTIME because EscrowLock didn't reach the chain
# (likely TaskOpen state_root propagation timing on a small-MAX_TX run).
#
# Round 2 strategy:
#   - bigger MAX_TX (20) gives the preseed loop more wall-clock to
#     observe TaskOpen state_root before EscrowLock submit;
#   - bigger N (5) gives more solver attempts per problem;
#   - duplicate arena_run4-style triple-probe in P6 (FORCE_CHALLENGER +
#     COMPLETE_SET_SEED simultaneously) to confirm 7-tx-kind chain
#     reproducible on R3 binary;
#   - RUST_LOG=info captures EscrowLock + bus.submit traces;
#   - extended problem set (8) for richer aggregate.
#
# Usage:
#   bash handover/tests/scripts/run_post_r3_round2.sh

set -uo pipefail
cd /home/zephryj/projects/turingosv4

OUT_BASE="handover/evidence/tb_16_real_llm_arena_2026-05-04/post_r3_round2"
mkdir -p "$OUT_BASE"

EVALUATOR_BIN="./target/release/evaluator"
AUDIT_TAPE_BIN="./target/release/audit_tape"
AUDIT_TAPE_TAMPER_BIN="./target/release/audit_tape_tamper"
GEN_MARKOV_BIN="./target/release/generate_markov_capsule"
AUDIT_DASHBOARD_BIN="./target/release/audit_dashboard"

LLM_PROXY_URL="${LLM_PROXY_URL:-http://localhost:18080}"
N_SWARM="${N_SWARM:-5}"
MAX_TX="${MAX_TX:-20}"

# 8 problems × probe env-vars (1-line each: <id>|<problem.lean>|<extra-env>)
PROBLEMS=(
  "P1_baseline|mathd_algebra_171.lean|"
  "P2_challenge|mathd_algebra_11.lean|TURINGOS_FORCE_CHALLENGER=Agent_2"
  "P3_completeset|mathd_algebra_96.lean|TURINGOS_COMPLETE_SET_SEED=Agent_user_0:1000000"
  "P4_bankruptcy|mathd_numbertheory_961.lean|TURINGOS_FORCE_BANKRUPTCY=Agent_0"
  "P5_aime_hard|aime_1997_p9.lean|"
  "P6_triple_probe|mathd_algebra_171.lean|TURINGOS_FORCE_CHALLENGER=Agent_3 TURINGOS_COMPLETE_SET_SEED=Agent_user_0:1000000"
  "P7_baseline_b|mathd_algebra_67.lean|"
  "P8_completeset_b|amc12b_2020_p5.lean|TURINGOS_COMPLETE_SET_SEED=Agent_user_0:1500000"
  # TB-16.x.2.1 (umbrella charter §2 Atom 2.1): TURINGOS_FORCE_EXPIRE=1 fires
  # tb11_emit_expire_for_eligible(expiry_delta=0) at run cleanup so every
  # Open/Bankrupt task emits TaskExpireTx (Deadline reason solo;
  # BankruptcyTriggered when chained with FORCE_BANKRUPTCY). aime_1997_p9
  # selected because it reliably MaxTxExhausts on the round2 budget per R2 P5.
  "P9_force_expire|aime_1997_p9.lean|TURINGOS_FORCE_EXPIRE=1"
)

START_TS=$(date +%s)
echo "════════════════════════════════════════════════════════════════════"
echo "TB-16 post-R3 Round 2 — comprehensive mechanism RUNTIME exercise"
echo "════════════════════════════════════════════════════════════════════"
echo "  N_SWARM=$N_SWARM  MAX_TX=$MAX_TX  LLM_PROXY=$LLM_PROXY_URL"
echo "  Problem count: ${#PROBLEMS[@]}"
echo "  Out dir: $OUT_BASE"
echo "  Start: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
echo

for entry in "${PROBLEMS[@]}"; do
  IFS='|' read -r PID PFILE PROBE_ENV <<< "$entry"
  PROBLEM_DIR="$OUT_BASE/$PID"
  mkdir -p "$PROBLEM_DIR/runtime_repo" "$PROBLEM_DIR/cas"

  echo "▶ [$PID] $PFILE  (probe: ${PROBE_ENV:-vanilla})"
  PROBE_PREFIX=""
  if [[ -n "$PROBE_ENV" ]]; then
    PROBE_PREFIX="$PROBE_ENV "
  fi

  T0=$(date +%s)
  # TURINGOS_CHAINTAPE_PRESEED=1 is REQUIRED to enable the user-task-mode
  # TaskOpen+EscrowLock preseed path (evaluator.rs:857-975). Without it
  # the chain only contains synthetic TaskOpen + TerminalSummary; Work tx
  # all reject with escrow_missing because EscrowLock never lands.
  # Per arena_run4 reproducer (d1c1af2): preseed enables the full 7-tx
  # accepted chain (TaskOpen + EscrowLock + Work + Verify + Challenge +
  # CompleteSetMint + MarketSeed).
  env $PROBE_PREFIX \
    TURINGOS_USER_TASK_MODE=1 \
    TURINGOS_CHAINTAPE_PRESEED=1 \
    TURINGOS_USER_TASK_BOUNTY_MICRO=200000 \
    TURINGOS_CHAINTAPE_PATH="$PROBLEM_DIR/runtime_repo" \
    TURINGOS_CAS_PATH="$PROBLEM_DIR/cas" \
    TURINGOS_RUN_ID="tb16-post-r3-round2-$PID" \
    LLM_PROXY_URL="$LLM_PROXY_URL" \
    MAX_TRANSACTIONS="$MAX_TX" \
    CONDITION="n${N_SWARM}" \
    RUST_LOG="${RUST_LOG:-info}" \
    "$EVALUATOR_BIN" "$PFILE" 2> "$PROBLEM_DIR/evaluator.stderr" 1> "$PROBLEM_DIR/evaluator.stdout"
  RC=$?
  T1=$(date +%s)
  ELAPSED=$((T1 - T0))
  echo "    evaluator: rc=$RC  elapsed=${ELAPSED}s"

  grep "^PPUT_RESULT:" "$PROBLEM_DIR/evaluator.stdout" | tail -1 > "$PROBLEM_DIR/pput_result.json"
  if [[ -s "$PROBLEM_DIR/pput_result.json" ]]; then
    sed -i 's/^PPUT_RESULT://' "$PROBLEM_DIR/pput_result.json"
  fi

  # Trace EscrowLock + bus.submit decisions for diagnosis
  grep -E "EscrowLock|chaintape|escrow|tb10|d3" "$PROBLEM_DIR/evaluator.stderr" 2>&1 | head -10 > "$PROBLEM_DIR/escrow_trace.txt" || true

  echo "    audit_tape..."
  "$AUDIT_TAPE_BIN" \
    --runtime-repo "$PROBLEM_DIR/runtime_repo" \
    --cas-dir "$PROBLEM_DIR/cas" \
    --agent-pubkeys "$PROBLEM_DIR/runtime_repo/agent_pubkeys.json" \
    --pinned-pubkeys "$PROBLEM_DIR/runtime_repo/pinned_pubkeys.json" \
    --genesis genesis_payload.toml \
    --constitution constitution.md \
    --alignment-dir handover/alignment \
    --out "$PROBLEM_DIR/verdict.json" 2>&1 | tail -1

  echo "    audit_tape replay..."
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
    echo "    ✓ replay byte-identical"
  else
    echo "    ✗ replay diverged"
  fi

  echo "    audit_tape_tamper..."
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

  echo "    generate_markov_capsule..."
  # TB-16.x.fix: per-run capsule lineage is no longer sourced from a
  # global pointer file. To inherit from a prior chain, set
  # PREV_CID_HEX explicitly before invocation; otherwise generate as
  # genesis (no `--prev-cid-hex`).
  if [[ -n "${PREV_CID_HEX:-}" ]]; then
    "$GEN_MARKOV_BIN" \
      --tb-id 16 \
      --out-dir "$PROBLEM_DIR" \
      --constitution-path constitution.md \
      --runtime-repo "$PROBLEM_DIR/runtime_repo" \
      --cas-dir "$PROBLEM_DIR/cas" \
      --alignment-dir handover/alignment \
      --prev-cid-hex "$PREV_CID_HEX" 2>&1 | grep -E "capsule_id|wrote" | tail -3
  else
    "$GEN_MARKOV_BIN" \
      --tb-id 16 \
      --out-dir "$PROBLEM_DIR" \
      --constitution-path constitution.md \
      --runtime-repo "$PROBLEM_DIR/runtime_repo" \
      --cas-dir "$PROBLEM_DIR/cas" \
      --alignment-dir handover/alignment \
      2>&1 | grep -E "capsule_id|wrote" | tail -3
  fi

  echo "    audit_dashboard..."
  "$AUDIT_DASHBOARD_BIN" \
    --repo "$PROBLEM_DIR/runtime_repo" \
    --cas "$PROBLEM_DIR/cas" \
    --out "$PROBLEM_DIR/dashboard.txt" 2>&1 | tail -1
  echo
done

END_TS=$(date +%s)
TOTAL_ELAPSED=$((END_TS - START_TS))
echo "════════════════════════════════════════════════════════════════════"
echo "Round 2 done. Total wall: ${TOTAL_ELAPSED}s. Out: $OUT_BASE"
echo "Next: aggregate via aggregate_post_r3_full_test.sh-style script"
echo "════════════════════════════════════════════════════════════════════"
