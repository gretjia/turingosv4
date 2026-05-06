#!/usr/bin/env bash
# TB-18R R7 — M0 small batch (5 non-overlapping problems) on corrected
# substrate. Per TB-18R charter §2 R7 row + §1.4 SG-18R.D.
set -uo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
OUT_DIR="$PROJECT_ROOT/handover/evidence/tb_18r_r7_m0_2026-05-06"
PROBLEMS_FILE="$OUT_DIR/r7_problems.txt"
EVALUATOR_BIN="$PROJECT_ROOT/target/release/evaluator"
AUDIT_TAPE_BIN="$PROJECT_ROOT/target/release/audit_tape"
INVARIANT_BIN="$PROJECT_ROOT/target/release/tb_18r_compute_invariant"
MAX_TX="${MAX_TX:-8}"
PER_PROBLEM_TIMEOUT_S="${PER_PROBLEM_TIMEOUT_S:-360}"

if [ -z "${DEEPSEEK_API_KEY:-}" ] && [ -f "$HOME/projects/turingosv3/.env" ]; then
  source "$HOME/projects/turingosv3/.env"
fi
export LLM_PROXY_URL="${LLM_PROXY_URL:-http://localhost:8080}"
export ACTIVE_MODEL="${ACTIVE_MODEL:-deepseek-chat}"
export CONDITION="${CONDITION:-n1}"
export MINIF2F_DIR="${MINIF2F_DIR:-/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4}"

mapfile -t PROBLEMS < "$PROBLEMS_FILE"
N=${#PROBLEMS[@]}
RUN_TS="$(date -u +%Y-%m-%dT%H-%M-%SZ)"
GIT_HEAD="$(cd "$PROJECT_ROOT" && git rev-parse HEAD)"

cat > "$OUT_DIR/R7_RUN_MANIFEST.json" <<EOM
{
  "phase": "TB-18R R7 (M0 small batch)",
  "atom_authority": "TB-18R charter §2 R7 row + §1.4 SG-18R.D",
  "preflight": "handover/ai-direct/TB-18R_R4_STEP_B_invariant.md",
  "problem_count": $N,
  "max_tx_per_problem": $MAX_TX,
  "per_problem_timeout_s": $PER_PROBLEM_TIMEOUT_S,
  "llm_proxy_url": "$LLM_PROXY_URL",
  "active_model": "$ACTIVE_MODEL",
  "condition": "$CONDITION",
  "run_timestamp_utc": "$RUN_TS",
  "git_head": "$GIT_HEAD",
  "no_overlap_with_r6": true,
  "problems": [
$(for ((i=0; i<N; i++)); do
    sep=","; [ $((i+1)) -eq "$N" ] && sep=""
    echo "    \"${PROBLEMS[$i]}\"$sep"
  done)
  ]
}
EOM

echo "[r7] starting | N=$N | MAX_TX=$MAX_TX | git=$GIT_HEAD | ts=$RUN_TS"

for ((i=0; i<N; i++)); do
  NAME="${PROBLEMS[$i]}"
  IDX="$(printf 'P%02d' "$((i+1))")"
  TAG="${IDX}_${NAME}"
  RUN_DIR="$OUT_DIR/$TAG"
  mkdir -p "$RUN_DIR/runtime_repo" "$RUN_DIR/cas"
  PROBLEM_FILE="$MINIF2F_DIR/MiniF2F/Test/${NAME}.lean"

  if [ ! -f "$PROBLEM_FILE" ]; then
    echo "[r7] [$((i+1))/$N] $NAME → MISSING; skip"; continue
  fi
  echo "[r7] [$((i+1))/$N] $NAME → $RUN_DIR"
  PROB_START="$(date +%s)"
  set +e
  TURINGOS_CHAINTAPE_PATH="$RUN_DIR/runtime_repo" \
    TURINGOS_CAS_PATH="$RUN_DIR/cas" \
    TURINGOS_CHAINTAPE_PRESEED="1" \
    EXPERIMENT_DIR="$RUN_DIR" \
    MAX_TX_OVERRIDE="$MAX_TX" \
    timeout "${PER_PROBLEM_TIMEOUT_S}" \
    "$EVALUATOR_BIN" "$PROBLEM_FILE" \
    > "$RUN_DIR/evaluator.stdout" 2> "$RUN_DIR/evaluator.stderr"
  EVAL_RC=$?
  set -e
  PROB_DUR=$(($(date +%s) - PROB_START))

  PPUT_LINE="$(grep '^PPUT_RESULT:' "$RUN_DIR/evaluator.stdout" | head -1 | sed 's/^PPUT_RESULT://')"
  if [ -n "$PPUT_LINE" ]; then
    EXPECTED="$(echo "$PPUT_LINE" | python3 -c 'import sys,json; d=json.load(sys.stdin); print(sum(d.get("tool_dist",{}).values()))' 2>/dev/null || echo "0")"
    SOLVED="$(echo "$PPUT_LINE" | python3 -c 'import sys,json; print(json.load(sys.stdin).get("solved",False))' 2>/dev/null || echo "False")"
    HITMAX="$(echo "$PPUT_LINE" | python3 -c 'import sys,json; print(json.load(sys.stdin).get("hit_max_tx",False))' 2>/dev/null || echo "False")"
    if [ "$SOLVED" = "True" ]; then HALT="OmegaAccepted"
    elif [ "$HITMAX" = "True" ]; then HALT="MaxTxExhausted"
    else HALT="MaxTxExhausted"; fi
  else
    EXPECTED="0"; HALT="ErrorHalt"
  fi

  set +e
  "$INVARIANT_BIN" \
    --runtime-repo "$RUN_DIR/runtime_repo" \
    --cas "$RUN_DIR/cas" \
    --expected-completed "$EXPECTED" \
    --halt-class "$HALT" \
    > "$RUN_DIR/chain_invariant.json" 2> "$RUN_DIR/chain_invariant.stderr"
  set -e

  set +e
  "$AUDIT_TAPE_BIN" \
    --runtime-repo "$RUN_DIR/runtime_repo" \
    --cas-dir "$RUN_DIR/cas" \
    --agent-pubkeys "$RUN_DIR/runtime_repo/agent_pubkeys.json" \
    --pinned-pubkeys "$RUN_DIR/runtime_repo/pinned_pubkeys.json" \
    --genesis "$PROJECT_ROOT/genesis_payload.toml" \
    --constitution "$PROJECT_ROOT/constitution.md" \
    --alignment-dir "$PROJECT_ROOT/handover/alignment" \
    --out "$RUN_DIR/verdict.json" \
    2> "$RUN_DIR/audit_tape.stderr"
  set -e

  VERDICT="$(grep -oE '"invariant_verdict"[[:space:]]*:[[:space:]]*"[^"]*"' "$RUN_DIR/chain_invariant.json" 2>/dev/null | head -1 || echo "(none)")"
  echo "[r7] [$((i+1))/$N] $NAME: dur=${PROB_DUR}s expected=$EXPECTED halt=$HALT $VERDICT"
done

echo "[r7] DONE"
