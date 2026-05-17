#!/usr/bin/env bash
# TB-18R G2 round-2 R9 — P38 + P49 rerun on R8-fix-applied source.
#
# Per G2 verdict §3 Blocker 2: R6 P02/P03 were SIGKILL'd before
# PPUT_RESULT emit, leaving r4_invariant_equation_evaluable=false.
# Remediation: rerun with PER_PROBLEM_TIMEOUT_S=1800 AND a properly-
# enforced MAX_TRANSACTIONS=12 (the r6 runner's MAX_TX_OVERRIDE was a
# no-op — evaluator reads MAX_TRANSACTIONS).
#
# Outputs per problem (RUN_DIR = OUT_DIR/<idx>_<name>):
#   runtime_repo/         — chaintape repo
#   cas/                  — CAS objects
#   evaluator.stdout/err  — evaluator output (PPUT_RESULT line in stdout)
#   verdict.json          — audit_tape verdict (R8-fix → 38 PASS / 0 FAIL)
#   chain_invariant.json  — R4 6-field invariant facts (evaluable=true)
#   README.md             — per-run summary
#
# Usage: bash handover/tests/scripts/run_tb_18r_r9_evidence.sh

set -uo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"

OUT_DIR="${OUT_DIR:-$PROJECT_ROOT/handover/evidence/tb_18r_r9_p38_p49_2026-05-06}"
PROBLEMS_FILE="$OUT_DIR/r9_problems.txt"
EVALUATOR_BIN="$PROJECT_ROOT/target/release/evaluator"
AUDIT_TAPE_BIN="$PROJECT_ROOT/target/release/audit_tape"
INVARIANT_BIN="$PROJECT_ROOT/target/release/tb_18r_compute_invariant"
MAX_TX="${MAX_TX:-12}"
PER_PROBLEM_TIMEOUT_S="${PER_PROBLEM_TIMEOUT_S:-1800}"

mkdir -p "$OUT_DIR"
cat > "$PROBLEMS_FILE" <<'EOM'
mathd_numbertheory_1124
numbertheory_2pownm1prime_nprime
EOM

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

echo "[r9] starting | N=$N | MAX_TX=$MAX_TX | timeout=${PER_PROBLEM_TIMEOUT_S}s | git=$GIT_HEAD | ts=$RUN_TS"

cat > "$OUT_DIR/R9_RUN_MANIFEST.json" <<EOM
{
  "phase": "TB-18R G2 round-2 R9",
  "atom_authority": "G2 verdict §3 Blocker 2 + §6 R9; charter §2 R6 row + §1.4 SG-18R.4 v2",
  "predecessor_evidence": "handover/evidence/tb_18r_r6_p23_p38_p49_2026-05-06/ (P38/P49 SIGKILL'd)",
  "fix": "MAX_TRANSACTIONS=$MAX_TX (correct env var; r6 runner used MAX_TX_OVERRIDE which is a no-op) + PER_PROBLEM_TIMEOUT_S=$PER_PROBLEM_TIMEOUT_S (3x prior)",
  "problem_count": $N,
  "max_tx_per_problem": $MAX_TX,
  "per_problem_timeout_s": $PER_PROBLEM_TIMEOUT_S,
  "llm_proxy_url": "$LLM_PROXY_URL",
  "active_model": "$ACTIVE_MODEL",
  "condition": "$CONDITION",
  "run_timestamp_utc": "$RUN_TS",
  "git_head": "$GIT_HEAD",
  "problems": [
$(for ((i=0; i<N; i++)); do
    sep=","; [ $((i+1)) -eq "$N" ] && sep=""
    echo "    \"${PROBLEMS[$i]}\"$sep"
  done)
  ]
}
EOM

for ((i=0; i<N; i++)); do
  NAME="${PROBLEMS[$i]}"
  IDX="$(printf 'P%02d' "$((i+1))")"
  TAG="${IDX}_${NAME}"
  RUN_DIR="$OUT_DIR/$TAG"
  mkdir -p "$RUN_DIR/runtime_repo" "$RUN_DIR/cas"
  PROBLEM_FILE="$MINIF2F_DIR/MiniF2F/Test/${NAME}.lean"

  if [ ! -f "$PROBLEM_FILE" ]; then
    echo "[r9] [$((i+1))/$N] $NAME → MISSING; skip"
    continue
  fi
  echo "[r9] [$((i+1))/$N] $NAME → $RUN_DIR"
  PROB_START="$(date +%s)"
  set +e
  TURINGOS_CHAINTAPE_PATH="$RUN_DIR/runtime_repo" \
    TURINGOS_CAS_PATH="$RUN_DIR/cas" \
    TURINGOS_CHAINTAPE_PRESEED="1" \
    EXPERIMENT_DIR="$RUN_DIR" \
    MAX_TRANSACTIONS="$MAX_TX" \
    timeout "${PER_PROBLEM_TIMEOUT_S}" \
    "$EVALUATOR_BIN" "$PROBLEM_FILE" \
    > "$RUN_DIR/evaluator.stdout" 2> "$RUN_DIR/evaluator.stderr"
  EVAL_RC=$?
  set -e
  PROB_DUR=$(($(date +%s) - PROB_START))

  EXPECTED_COMPLETED="$(grep -oE '"completed_llm_calls"[[:space:]]*:[[:space:]]*[0-9]+' "$RUN_DIR/evaluator.stdout" | head -1 | grep -oE '[0-9]+' || echo "0")"
  if [ "$EXPECTED_COMPLETED" = "0" ]; then
    EXPECTED_COMPLETED="$(grep -oE '"externalized_llm_calls"[[:space:]]*:[[:space:]]*[0-9]+' "$RUN_DIR/evaluator.stdout" | head -1 | grep -oE '[0-9]+' || echo "0")"
  fi
  if [ "$EXPECTED_COMPLETED" = "0" ]; then
    EXPECTED_COMPLETED="$(grep '^PPUT_RESULT:' "$RUN_DIR/evaluator.stdout" | head -1 | grep -oE '"proposal_count"[[:space:]]*:[[:space:]]*[0-9]+' | grep -oE '[0-9]+' || echo "0")"
  fi
  HALT_CLASS="$(grep -oE '"halt_reason"[[:space:]]*:[[:space:]]*"[A-Za-z]+"' "$RUN_DIR/evaluator.stdout" | head -1 | sed 's/.*"\([A-Za-z]*\)"/\1/' || echo "MaxTxExhausted")"
  if [ -z "$HALT_CLASS" ]; then
    HALT_CLASS="MaxTxExhausted"
  fi

  echo "[r9] [$((i+1))/$N] $NAME: dur=${PROB_DUR}s expected_completed=$EXPECTED_COMPLETED halt=$HALT_CLASS rc=$EVAL_RC"

  set +e
  "$INVARIANT_BIN" \
    --runtime-repo "$RUN_DIR/runtime_repo" \
    --cas "$RUN_DIR/cas" \
    --expected-completed "$EXPECTED_COMPLETED" \
    --halt-class "$HALT_CLASS" \
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

  cat > "$RUN_DIR/README.md" <<EOM
# TB-18R G2 round-2 R9 — $TAG

**Phase**: TB-18R G2 round-2 R9 evidence (P38/P49 evaluable rerun on R8-fix substrate).
**Authority**: G2 verdict §3 Blocker 2 + §6 R9; charter §2 R6 row + §1.4 SG-18R.4 v2.
**Date**: $RUN_TS
**Git HEAD**: $GIT_HEAD
**Predecessor**: R6 SIGKILL evidence (handover/evidence/tb_18r_r6_p23_p38_p49_2026-05-06/$TAG/).

## Run params

- problem: \`$NAME\`
- problem file: \`$PROBLEM_FILE\`
- MAX_TRANSACTIONS: $MAX_TX (was MAX_TX_OVERRIDE in r6 runner — a no-op)
- per-problem timeout: ${PER_PROBLEM_TIMEOUT_S}s (3x prior 600s)
- LLM proxy: $LLM_PROXY_URL
- active model: $ACTIVE_MODEL
- condition: $CONDITION
- duration: ${PROB_DUR}s
- evaluator exit code: $EVAL_RC

## R4 invariant verdict

\`\`\`
$(cat "$RUN_DIR/chain_invariant.json" 2>/dev/null || echo "(invariant compute failed; see chain_invariant.stderr)")
\`\`\`
EOM
  echo "[r9] [$((i+1))/$N] $NAME: invariant=$(grep -oE '"invariant_verdict"[[:space:]]*:[[:space:]]*"[^"]*"' "$RUN_DIR/chain_invariant.json" | head -1 || echo unknown)"
done

python3 "$PROJECT_ROOT/handover/tests/scripts/r9_batch_summary.py" \
  --out-dir "$OUT_DIR" \
  --problems-file "$PROBLEMS_FILE" \
  --max-tx "$MAX_TX" \
  --per-problem-timeout-s "$PER_PROBLEM_TIMEOUT_S" \
  --git-head "$GIT_HEAD" \
  --run-timestamp-utc "$RUN_TS"

echo "[r9] DONE | summary at $OUT_DIR/R9_BATCH_SUMMARY.json"
