#!/usr/bin/env bash
# TB-18R R6 — P23 + P38 + P49 rerun on corrected substrate.
#
# Per TB-18R charter §2 R6 row + §1.4 SG-18R.4 v2 (Codex Q4+Q7
# remediated; six-field exact accounting). Validates R4 invariant
# `evaluator_reported_completed_llm_calls == l4_work_attempt_count +
# l4e_work_attempt_count` end-to-end on actual LLM-Lean cycles.
#
# Outputs per problem (RUN_DIR = OUT_DIR/<idx>_<name>):
#   runtime_repo/         — chaintape repo
#   cas/                  — CAS objects
#   evaluator.stdout/err  — evaluator output
#   verdict.json          — audit_tape verdict (R5 sampler PASSes)
#   chain_invariant.json  — R4 6-field invariant facts (this script)
#   README.md             — per-run summary
#
# Usage:
#   bash handover/tests/scripts/run_tb_18r_r6_evidence.sh \
#        [--max-tx <n>] [--per-problem-timeout-s <n>]

set -uo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"

OUT_DIR="$PROJECT_ROOT/handover/evidence/tb_18r_r6_p23_p38_p49_2026-05-06"
PROBLEMS_FILE="$OUT_DIR/r6_problems.txt"
EVALUATOR_BIN="$PROJECT_ROOT/target/release/evaluator"
AUDIT_TAPE_BIN="$PROJECT_ROOT/target/release/audit_tape"
INVARIANT_BIN="$PROJECT_ROOT/target/release/tb_18r_compute_invariant"
MAX_TX="${MAX_TX:-12}"
PER_PROBLEM_TIMEOUT_S="${PER_PROBLEM_TIMEOUT_S:-600}"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --max-tx) MAX_TX="$2"; shift 2 ;;
    --per-problem-timeout-s) PER_PROBLEM_TIMEOUT_S="$2"; shift 2 ;;
    *) echo "ERROR: unknown arg: $1" >&2; exit 2 ;;
  esac
done

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

echo "[r6] starting | N=$N | MAX_TX=$MAX_TX | git=$GIT_HEAD | ts=$RUN_TS"

# Manifest
cat > "$OUT_DIR/R6_RUN_MANIFEST.json" <<EOM
{
  "phase": "TB-18R R6",
  "atom_authority": "TB-18R charter §2 R6 row + §1.4 SG-18R.3+SG-18R.4 v2",
  "preflight": "handover/ai-direct/TB-18R_R4_STEP_B_invariant.md",
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
    echo "[r6] [$((i+1))/$N] $NAME → MISSING; skip"
    continue
  fi
  echo "[r6] [$((i+1))/$N] $NAME → $RUN_DIR"
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

  # Extract evaluator-reported completed LLM calls + halt class.
  EXPECTED_COMPLETED="$(grep -oE '"completed_llm_calls"[[:space:]]*:[[:space:]]*[0-9]+' "$RUN_DIR/evaluator.stdout" | head -1 | grep -oE '[0-9]+' || echo "0")"
  if [ "$EXPECTED_COMPLETED" = "0" ]; then
    EXPECTED_COMPLETED="$(grep -oE '"externalized_llm_calls"[[:space:]]*:[[:space:]]*[0-9]+' "$RUN_DIR/evaluator.stdout" | head -1 | grep -oE '[0-9]+' || echo "0")"
  fi
  if [ "$EXPECTED_COMPLETED" = "0" ]; then
    # Fallback: count "PPUT_RESULT" tx_count or proposal_count from PPUT_RESULT line.
    EXPECTED_COMPLETED="$(grep '^PPUT_RESULT:' "$RUN_DIR/evaluator.stdout" | head -1 | grep -oE '"proposal_count"[[:space:]]*:[[:space:]]*[0-9]+' | grep -oE '[0-9]+' || echo "0")"
  fi
  HALT_CLASS="$(grep -oE '"halt_reason"[[:space:]]*:[[:space:]]*"[A-Za-z]+"' "$RUN_DIR/evaluator.stdout" | head -1 | sed 's/.*"\([A-Za-z]*\)"/\1/' || echo "MaxTxExhausted")"
  if [ -z "$HALT_CLASS" ]; then
    HALT_CLASS="MaxTxExhausted"
  fi

  echo "[r6] [$((i+1))/$N] $NAME: dur=${PROB_DUR}s expected_completed=$EXPECTED_COMPLETED halt=$HALT_CLASS"

  # R4 invariant compute.
  set +e
  "$INVARIANT_BIN" \
    --runtime-repo "$RUN_DIR/runtime_repo" \
    --cas "$RUN_DIR/cas" \
    --expected-completed "$EXPECTED_COMPLETED" \
    --halt-class "$HALT_CLASS" \
    > "$RUN_DIR/chain_invariant.json" 2> "$RUN_DIR/chain_invariant.stderr"
  INV_RC=$?
  set -e

  # Audit tape verdict.
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
# TB-18R R6 — $TAG

**Phase**: TB-18R R6 evidence (P49-class rerun on corrected substrate).
**Authority**: TB-18R charter §2 R6 row + §1.4 SG-18R.3 + SG-18R.4 v2.
**Date**: $RUN_TS
**Git HEAD**: $GIT_HEAD
**Predecessor**: TB-18R R5 SHIPPED commit 5a09e2d.

## Run params

- problem: \`$NAME\`
- problem file: \`$PROBLEM_FILE\`
- MAX_TX: $MAX_TX
- per-problem timeout: ${PER_PROBLEM_TIMEOUT_S}s
- LLM proxy: $LLM_PROXY_URL
- active model: $ACTIVE_MODEL
- condition: $CONDITION
- duration: ${PROB_DUR}s
- evaluator exit code: $EVAL_RC

## Outputs

- \`evaluator.stdout\` / \`evaluator.stderr\` — evaluator output.
- \`runtime_repo/\` — chaintape L4 + L4.E git repo.
- \`cas/\` — CAS object store (AttemptTelemetry + LeanResult +
  ProposalTelemetry + RejectedSubmissionRecord etc.).
- \`chain_invariant.json\` — R4 invariant facts (FR-18R.4 v2 6-field
  accounting + verdict).
- \`verdict.json\` — audit_tape battery verdict (R5 assertions 44/45/46
  PASS confirmation on real chain data).

## R4 invariant verdict

\`\`\`
$(cat "$RUN_DIR/chain_invariant.json" 2>/dev/null || echo "(invariant compute failed; see chain_invariant.stderr)")
\`\`\`
EOM
  echo "[r6] [$((i+1))/$N] $NAME: invariant=$(grep -oE '"invariant_verdict"[[:space:]]*:[[:space:]]*"[^"]*"' "$RUN_DIR/chain_invariant.json" | head -1 || echo unknown)"
done

# Aggregate
cat > "$OUT_DIR/R6_BATCH_SUMMARY.json" <<EOM
{
  "phase": "TB-18R R6",
  "problem_count": $N,
  "max_tx_per_problem": $MAX_TX,
  "git_head": "$GIT_HEAD",
  "run_timestamp_utc": "$RUN_TS",
  "per_problem_results": [
$(for ((i=0; i<N; i++)); do
    NAME="${PROBLEMS[$i]}"
    IDX="$(printf 'P%02d' "$((i+1))")"
    TAG="${IDX}_${NAME}"
    INV_FILE="$OUT_DIR/$TAG/chain_invariant.json"
    sep=","; [ $((i+1)) -eq "$N" ] && sep=""
    if [ -f "$INV_FILE" ]; then
      INV_VERDICT="$(grep -oE '"invariant_verdict"[[:space:]]*:[[:space:]]*"[^"]*"' "$INV_FILE" | head -1)"
      L4_COUNT="$(grep -oE '"l4_work_attempt_count"[[:space:]]*:[[:space:]]*[0-9]+' "$INV_FILE" | grep -oE '[0-9]+')"
      L4E_COUNT="$(grep -oE '"l4e_work_attempt_count"[[:space:]]*:[[:space:]]*[0-9]+' "$INV_FILE" | grep -oE '[0-9]+')"
      DELTA="$(grep -oE '"delta"[[:space:]]*:[[:space:]]*-?[0-9]+' "$INV_FILE" | grep -oE '\-?[0-9]+')"
      echo "    {\"tag\": \"$TAG\", \"l4\": $L4_COUNT, \"l4e\": $L4E_COUNT, \"delta\": $DELTA, $INV_VERDICT}$sep"
    else
      echo "    {\"tag\": \"$TAG\", \"error\": \"no chain_invariant.json\"}$sep"
    fi
  done)
  ]
}
EOM

echo "[r6] DONE | summary at $OUT_DIR/R6_BATCH_SUMMARY.json"
