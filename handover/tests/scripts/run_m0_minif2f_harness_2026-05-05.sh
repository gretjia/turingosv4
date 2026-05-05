#!/usr/bin/env bash
# M0 — MiniF2F harness audit (architect §9.3 verbatim M0 phase)
#
# Per 2026-05-05 architect verdict §B.9.3 + §B.11 point 11:
#   "Controlled small/medium MiniF2F can start as harness prep:
#    20 -> 50 -> 100 problems, all chain-backed, no real funds,
#    no public chain, no real-world readiness claim."
#
# This is M0 = 20-problem harness audit. Per architect §B.9.3 M0 spec:
#   - 20 known problems
#   - chain-backed
#   - no market
#   - prove no fake accepted
#
# Per architect §B.9.1 conditions (binding):
#   1. NOT real-world readiness
#   2. NO real funds
#   3. NO public settlement
#   4. NO ChainTape bypass
#   5. ALL proposal/proof/failures into ChainTape/CAS or EvidenceCapsule
#   6. Dashboard regenerable
#
# Output is ARCHITECTURE-PRESSURE DATA, NOT BENCHMARK SCORES.
# Per `feedback_minif2f_scaling_policy`: M0 results MUST NOT be cited as
# benchmark; M0 is harness-prep input for TB-18 Atom B substantive
# comprehensive_arena build.
#
# Per-problem flow (NO market hooks; NO FORCE_*; OMEGA-Confirm or MaxTxExhausted):
#   1. Fresh runtime_repo + cas dir per problem
#   2. evaluator runs with TURINGOS_CHAINTAPE_PATH + TURINGOS_CHAINTAPE_PRESEED=1
#   3. audit_tape produces verdict.json per chain
#   4. audit_tape replay → verdict_replay.json (assert byte-identical)
#   5. audit_tape_tamper → tamper_report.json (assert detected_count >= 3)
#
# Per-problem caps:
#   - MAX_TX = 20 (matches TB-16 default)
#   - per-problem timeout = 10 min wall-clock (600s)
#   - cumulative budget tracking via PROBLEMS_BUDGET_USD
#
# Aggregate output written by Claude post-run; this script just runs the batch.
#
# Exit codes:
#   0 — all 20 chains audited (any verdict) AND any per-chain hard error
#       still produces output (PROCEED OR BLOCK is a verdict, not error)
#   1 — script-level setup/build failure
#   2 — invalid args / preconditions
#
# Usage:
#   bash handover/tests/scripts/run_m0_minif2f_harness_2026-05-05.sh \
#        [--out-dir <path>] \
#        [--problems-file <path>] \
#        [--evaluator-bin <path>] \
#        [--audit-tape-bin <path>] \
#        [--audit-tape-tamper-bin <path>] \
#        [--max-tx <n>] \
#        [--per-problem-timeout-s <n>] \
#        [--llm-proxy-url <url>] \
#        [--skip-build] \
#        [--skip-llm-precheck] \
#        [--dry-run]
#
# TRACE_MATRIX FC1-N34 (audit_tape) + FC1-N36 (comprehensive_arena
# scaffold preserved; M0 does not modify it) + FC2-N31..N33 (verdict
# schema) + FC3-N44 (REAL_WORLD_READINESS_REPORT cite) + FC3-N45
# (markov_inheritance_policy tests).

set -uo pipefail

# ── Defaults ────────────────────────────────────────────────────────
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
DEFAULT_OUT_DIR="$PROJECT_ROOT/handover/evidence/m0_minif2f_harness_audit_2026-05-05"
DEFAULT_PROBLEMS_FILE="$SCRIPT_DIR/m0_problems.txt"
DEFAULT_MINIF2F_DIR="${MINIF2F_DIR:-/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4}"
DEFAULT_LLM_PROXY_URL="${LLM_PROXY_URL:-http://localhost:18080}"
DEFAULT_ACTIVE_MODEL="${ACTIVE_MODEL:-deepseek-chat}"
DEFAULT_CONDITION="${CONDITION:-n1}"

OUT_DIR="$DEFAULT_OUT_DIR"
PROBLEMS_FILE="$DEFAULT_PROBLEMS_FILE"
EVALUATOR_BIN="$PROJECT_ROOT/target/release/evaluator"
AUDIT_TAPE_BIN="$PROJECT_ROOT/target/release/audit_tape"
AUDIT_TAPE_TAMPER_BIN="$PROJECT_ROOT/target/release/audit_tape_tamper"
MAX_TX="20"
PER_PROBLEM_TIMEOUT_S="600"
LLM_PROXY_URL="$DEFAULT_LLM_PROXY_URL"
SKIP_BUILD="0"
SKIP_LLM_PRECHECK="${SKIP_LLM_PRECHECK:-0}"
DRY_RUN="0"

# ── Args ────────────────────────────────────────────────────────────
while [[ $# -gt 0 ]]; do
  case "$1" in
    --out-dir) OUT_DIR="$2"; shift 2 ;;
    --problems-file) PROBLEMS_FILE="$2"; shift 2 ;;
    --evaluator-bin) EVALUATOR_BIN="$2"; shift 2 ;;
    --audit-tape-bin) AUDIT_TAPE_BIN="$2"; shift 2 ;;
    --audit-tape-tamper-bin) AUDIT_TAPE_TAMPER_BIN="$2"; shift 2 ;;
    --max-tx) MAX_TX="$2"; shift 2 ;;
    --per-problem-timeout-s) PER_PROBLEM_TIMEOUT_S="$2"; shift 2 ;;
    --llm-proxy-url) LLM_PROXY_URL="$2"; shift 2 ;;
    --skip-build) SKIP_BUILD="1"; shift ;;
    --skip-llm-precheck) SKIP_LLM_PRECHECK="1"; shift ;;
    --dry-run) DRY_RUN="1"; shift ;;
    -h|--help)
      grep -E "^# " "$0" | sed 's/^# //'
      exit 0
      ;;
    *)
      echo "ERROR: unknown arg: $1" >&2
      echo "Run with --help for usage." >&2
      exit 2
      ;;
  esac
done

# ── Auto-load DeepSeek key from v3 .env ────────────────────────────
if [ -z "${DEEPSEEK_API_KEY:-}" ] && [ -f "$HOME/projects/turingosv3/.env" ]; then
  # shellcheck disable=SC1090
  source "$HOME/projects/turingosv3/.env"
fi
if [ -z "${DEEPSEEK_API_KEY:-}" ]; then
  echo "ERROR: DEEPSEEK_API_KEY not set (and v3 .env auto-load failed)" >&2
  exit 2
fi
export LLM_PROXY_URL ACTIVE_MODEL="${ACTIVE_MODEL:-$DEFAULT_ACTIVE_MODEL}"
export CONDITION="${CONDITION:-$DEFAULT_CONDITION}"
export MINIF2F_DIR="${MINIF2F_DIR:-$DEFAULT_MINIF2F_DIR}"

# ── Validate problem list ──────────────────────────────────────────
if [ ! -f "$PROBLEMS_FILE" ]; then
  echo "ERROR: problems file not found: $PROBLEMS_FILE" >&2
  exit 2
fi
mapfile -t PROBLEMS < "$PROBLEMS_FILE"
# strip blanks/comments
FILTERED=()
for p in "${PROBLEMS[@]}"; do
  case "$p" in
    ""|\#*) continue ;;
    *) FILTERED+=("$p") ;;
  esac
done
PROBLEMS=("${FILTERED[@]}")
N="${#PROBLEMS[@]}"
if [ "$N" -lt 1 ]; then
  echo "ERROR: empty problem list" >&2
  exit 2
fi

# ── Validate MiniF2F dataset ──────────────────────────────────────
if [ ! -d "$MINIF2F_DIR/MiniF2F/Test" ]; then
  echo "ERROR: MiniF2F/Test dir not found at $MINIF2F_DIR" >&2
  exit 2
fi
MISSING=()
for name in "${PROBLEMS[@]}"; do
  if [ ! -f "$MINIF2F_DIR/MiniF2F/Test/${name}.lean" ]; then
    MISSING+=("$name")
  fi
done
if [ "${#MISSING[@]}" -gt 0 ]; then
  echo "ERROR: ${#MISSING[@]} problems missing in $MINIF2F_DIR/MiniF2F/Test/:" >&2
  for m in "${MISSING[@]}"; do echo "  $m" >&2; done
  exit 2
fi

# ── Build (unless skipped) ────────────────────────────────────────
if [ "$SKIP_BUILD" != "1" ]; then
  echo "[m0] cargo build --release -p minif2f_v4 ..."
  (cd "$PROJECT_ROOT" && cargo build --release -p minif2f_v4 2>&1 | tail -3)
  echo "[m0] cargo build --release --bin audit_tape --bin audit_tape_tamper ..."
  (cd "$PROJECT_ROOT" && cargo build --release --bin audit_tape --bin audit_tape_tamper 2>&1 | tail -3)
fi
for bin in "$EVALUATOR_BIN" "$AUDIT_TAPE_BIN" "$AUDIT_TAPE_TAMPER_BIN"; do
  if [ ! -x "$bin" ]; then
    echo "ERROR: required binary missing/non-exec: $bin" >&2
    exit 2
  fi
done

# ── Mathlib preflight (C-012) ─────────────────────────────────────
LEAN_BIN="${LEAN_BINARY:-$HOME/.elan/toolchains/leanprover--lean4---v4.24.0/bin/lean}"
PFL=$(find "$MINIF2F_DIR/.lake/packages" \( -path "*/.lake/build/lib/lean" -o -path "*/lib/lean" \) -type d 2>/dev/null | tr '\n' ':')
if [ -z "$PFL" ]; then
  echo "ERROR: PREFLIGHT FAIL — no Mathlib under $MINIF2F_DIR/.lake/packages" >&2
  echo "  Try: (cd $MINIF2F_DIR && lake exe cache get)" >&2
  exit 2
fi
PREFLIGHT_OUT=$(printf 'import Mathlib\nexample : (1:ℝ) + 1 = 2 := by norm_num\n' | LEAN_PATH="$PFL" timeout 180 "$LEAN_BIN" --stdin 2>&1)
PFL_RC=$?
if [ $PFL_RC -ne 0 ] || echo "$PREFLIGHT_OUT" | grep -q "error:"; then
  echo "ERROR: PREFLIGHT FAIL: $(echo "$PREFLIGHT_OUT" | head -c 400)" >&2
  exit 2
fi
echo "[m0] preflight OK (Mathlib + lean $LEAN_BIN)"

# ── LLM proxy precheck (unless skipped) ───────────────────────────
if [ "$SKIP_LLM_PRECHECK" != "1" ]; then
  if ! curl -sf --max-time 5 "$LLM_PROXY_URL/health" >/dev/null 2>&1; then
    if ! curl -sf --max-time 5 "$LLM_PROXY_URL" >/dev/null 2>&1; then
      echo "WARN: LLM proxy at $LLM_PROXY_URL not responding to GET /health or /;" >&2
      echo "      proceeding anyway (the evaluator will fail per-problem if proxy down)." >&2
      echo "      Set --skip-llm-precheck to silence this warning." >&2
    fi
  else
    echo "[m0] LLM proxy at $LLM_PROXY_URL is up"
  fi
fi

# ── Out dir + run manifest ────────────────────────────────────────
mkdir -p "$OUT_DIR"
RUN_TIMESTAMP="$(date -u +%Y-%m-%dT%H-%M-%SZ)"
MANIFEST="$OUT_DIR/M0_RUN_MANIFEST.json"
{
  echo "{"
  echo "  \"phase\": \"M0\","
  echo "  \"architect_authority\": \"2026-05-05_TB17_AUDIT_VERDICT_ARCHITECT_RULING.md §B.9.3\","
  echo "  \"NOT_a_benchmark\": true,"
  echo "  \"NOT_real_world_readiness\": true,"
  echo "  \"problem_count\": $N,"
  echo "  \"max_tx_per_problem\": $MAX_TX,"
  echo "  \"per_problem_timeout_s\": $PER_PROBLEM_TIMEOUT_S,"
  echo "  \"llm_proxy_url\": \"$LLM_PROXY_URL\","
  echo "  \"active_model\": \"$ACTIVE_MODEL\","
  echo "  \"condition\": \"$CONDITION\","
  echo "  \"run_timestamp_utc\": \"$RUN_TIMESTAMP\","
  echo "  \"git_head\": \"$(cd "$PROJECT_ROOT" && git rev-parse HEAD)\","
  echo "  \"problems\": ["
  for ((i=0; i<N; i++)); do
    sep=","; [ $((i+1)) -eq "$N" ] && sep=""
    echo "    \"${PROBLEMS[$i]}\"$sep"
  done
  echo "  ]"
  echo "}"
} > "$MANIFEST"
echo "[m0] manifest written to $MANIFEST"

# ── Dry run early-exit ────────────────────────────────────────────
if [ "$DRY_RUN" = "1" ]; then
  echo "[m0] --dry-run: setup verified, $N problems queued; not running."
  exit 0
fi

# ── Per-problem batch loop ────────────────────────────────────────
PROCEED_COUNT=0
BLOCK_COUNT=0
ERROR_COUNT=0
REPLAY_OK_COUNT=0
TAMPER_OK_COUNT=0
SOLVED_COUNT=0
EXHAUST_COUNT=0
START_T="$(date +%s)"

for ((i=0; i<N; i++)); do
  NAME="${PROBLEMS[$i]}"
  IDX="$(printf 'P%02d' "$((i+1))")"
  TAG="${IDX}_${NAME}"
  RUN_DIR="$OUT_DIR/$TAG"
  mkdir -p "$RUN_DIR/runtime_repo" "$RUN_DIR/cas"

  PROBLEM_FILE="$MINIF2F_DIR/MiniF2F/Test/${NAME}.lean"
  echo "[m0] [$((i+1))/$N] $NAME → $RUN_DIR"

  # Per-problem env (chain-backed; NO market hooks; NO FORCE_*)
  PROB_START="$(date +%s)"
  EVAL_LOG="$RUN_DIR/evaluator.stdout"
  EVAL_ERR="$RUN_DIR/evaluator.stderr"
  set +e
  TURINGOS_CHAINTAPE_PATH="$RUN_DIR/runtime_repo" \
    TURINGOS_CAS_PATH="$RUN_DIR/cas" \
    TURINGOS_CHAINTAPE_PRESEED="1" \
    EXPERIMENT_DIR="$RUN_DIR" \
    MAX_TX_OVERRIDE="$MAX_TX" \
    timeout "${PER_PROBLEM_TIMEOUT_S}" \
    "$EVALUATOR_BIN" "$PROBLEM_FILE" \
    > "$EVAL_LOG" 2> "$EVAL_ERR"
  EVAL_RC=$?
  set -e
  PROB_END="$(date +%s)"
  PROB_DUR=$((PROB_END - PROB_START))

  # Detect solve outcome via PPUT_RESULT line
  if grep -q "^PPUT_RESULT:" "$EVAL_LOG" 2>/dev/null; then
    if grep "^PPUT_RESULT:" "$EVAL_LOG" | grep -q '"solved": *true\|"omega_accepted": *true\|"gp_exists": *true'; then
      SOLVED_COUNT=$((SOLVED_COUNT + 1))
      OUTCOME="solved"
    else
      EXHAUST_COUNT=$((EXHAUST_COUNT + 1))
      OUTCOME="exhausted"
    fi
  else
    OUTCOME="error_or_no_pput"
  fi

  # Audit_tape → verdict.json
  AUDIT_LOG="$RUN_DIR/audit_tape.stderr"
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
    2> "$AUDIT_LOG"
  AT_RC=$?
  set -e

  if [ -f "$RUN_DIR/verdict.json" ]; then
    if grep -q '"verdict": *"PROCEED"' "$RUN_DIR/verdict.json"; then
      PROCEED_COUNT=$((PROCEED_COUNT + 1))
      VERDICT="PROCEED"
    else
      BLOCK_COUNT=$((BLOCK_COUNT + 1))
      VERDICT="$(grep -o '"verdict": *"[A-Z]*"' "$RUN_DIR/verdict.json" | head -1 | grep -o '"[A-Z]*"$' | tr -d '"' || echo BLOCK)"
    fi
  else
    ERROR_COUNT=$((ERROR_COUNT + 1))
    VERDICT="ERROR"
  fi

  # Audit_tape replay (byte-identical check)
  set +e
  "$AUDIT_TAPE_BIN" \
    --runtime-repo "$RUN_DIR/runtime_repo" \
    --cas-dir "$RUN_DIR/cas" \
    --agent-pubkeys "$RUN_DIR/runtime_repo/agent_pubkeys.json" \
    --pinned-pubkeys "$RUN_DIR/runtime_repo/pinned_pubkeys.json" \
    --genesis "$PROJECT_ROOT/genesis_payload.toml" \
    --constitution "$PROJECT_ROOT/constitution.md" \
    --alignment-dir "$PROJECT_ROOT/handover/alignment" \
    --out "$RUN_DIR/verdict_replay.json" \
    2>> "$AUDIT_LOG"
  set -e
  if [ -f "$RUN_DIR/verdict.json" ] && [ -f "$RUN_DIR/verdict_replay.json" ]; then
    if cmp -s "$RUN_DIR/verdict.json" "$RUN_DIR/verdict_replay.json"; then
      REPLAY_OK_COUNT=$((REPLAY_OK_COUNT + 1))
      REPLAY="ok"
    else
      REPLAY="DIVERGED"
    fi
  else
    REPLAY="missing"
  fi

  # Tamper-detection
  TAMPER_LOG="$RUN_DIR/audit_tape_tamper.stderr"
  set +e
  mkdir -p "$RUN_DIR/tamper"
  "$AUDIT_TAPE_TAMPER_BIN" \
    --runtime-repo "$RUN_DIR/runtime_repo" \
    --cas-dir "$RUN_DIR/cas" \
    --agent-pubkeys "$RUN_DIR/runtime_repo/agent_pubkeys.json" \
    --pinned-pubkeys "$RUN_DIR/runtime_repo/pinned_pubkeys.json" \
    --genesis "$PROJECT_ROOT/genesis_payload.toml" \
    --constitution "$PROJECT_ROOT/constitution.md" \
    --alignment-dir "$PROJECT_ROOT/handover/alignment" \
    --tamper-dir "$RUN_DIR/tamper" \
    --out "$RUN_DIR/tamper_report.json" \
    2> "$TAMPER_LOG"
  set -e
  if [ -f "$RUN_DIR/tamper_report.json" ]; then
    DETECTED="$(grep -o '"detected_count": *[0-9]*' "$RUN_DIR/tamper_report.json" | head -1 | grep -o '[0-9]*$' || echo 0)"
    if [ "${DETECTED:-0}" -ge 3 ]; then
      TAMPER_OK_COUNT=$((TAMPER_OK_COUNT + 1))
      TAMPER="${DETECTED}/3"
    else
      TAMPER="${DETECTED:-0}/3-DEGRADED"
    fi
  else
    TAMPER="missing"
  fi

  # Per-problem one-line summary
  echo "[m0] [$((i+1))/$N] $TAG: outcome=$OUTCOME verdict=$VERDICT replay=$REPLAY tamper=$TAMPER dur=${PROB_DUR}s"

done

END_T="$(date +%s)"
TOTAL_DUR=$((END_T - START_T))

# ── Aggregate summary ────────────────────────────────────────────
SUMMARY="$OUT_DIR/M0_BATCH_SUMMARY.json"
{
  echo "{"
  echo "  \"phase\": \"M0\","
  echo "  \"NOT_a_benchmark\": true,"
  echo "  \"NOT_real_world_readiness\": true,"
  echo "  \"run_timestamp_utc\": \"$RUN_TIMESTAMP\","
  echo "  \"total_duration_s\": $TOTAL_DUR,"
  echo "  \"problem_count\": $N,"
  echo "  \"audit_verdict\": {"
  echo "    \"proceed\": $PROCEED_COUNT,"
  echo "    \"block\":   $BLOCK_COUNT,"
  echo "    \"error\":   $ERROR_COUNT"
  echo "  },"
  echo "  \"replay_byte_identical\": $REPLAY_OK_COUNT,"
  echo "  \"tamper_3_of_3_detected\": $TAMPER_OK_COUNT,"
  echo "  \"evaluator_outcome\": {"
  echo "    \"solved\":    $SOLVED_COUNT,"
  echo "    \"exhausted\": $EXHAUST_COUNT,"
  echo "    \"error_or_no_pput\": $((N - SOLVED_COUNT - EXHAUST_COUNT))"
  echo "  },"
  echo "  \"manifest\": \"$MANIFEST\""
  echo "}"
} > "$SUMMARY"

cat <<EOF

══════════════════════════════════════════════════════════════════
M0 batch complete — out: $OUT_DIR
  problems:                    $N
  audit verdict PROCEED:       $PROCEED_COUNT
  audit verdict BLOCK:         $BLOCK_COUNT
  audit ERROR:                 $ERROR_COUNT
  replay byte-identical:       $REPLAY_OK_COUNT
  tamper 3/3 detected:         $TAMPER_OK_COUNT
  solved (OMEGA):              $SOLVED_COUNT
  exhausted (MaxTxExhausted):  $EXHAUST_COUNT
  total wall-clock:            ${TOTAL_DUR}s
══════════════════════════════════════════════════════════════════

NOT a benchmark; harness-prep input only.
Architect-mandated next step: aggregate report at
  handover/evidence/m0_minif2f_harness_audit_2026-05-05/M0_HARNESS_AUDIT_REPORT.md
(written by Claude post-run).

Summary JSON: $SUMMARY
Manifest:     $MANIFEST
EOF
exit 0
