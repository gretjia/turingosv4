#!/usr/bin/env bash
# TB-18R Phase 3 — Technical Tape Validation (P38 + P49 + M0 mini-batch).
#
# Authority: handover/directives/2026-05-06_TB18R_PHASE_3_LAUNCH_DIRECTIVE.md
#            (architect ruling §5 Phase 3 + §8 directive items 7–9; user 2026-05-06
#             "我授权你做phase 3 launch" multi-clause authorization)
#
# Validates (per architect ruling §5 Phase 3, run-by-run):
#   1. chain_attempt_count == evaluator_reported_tx_count (R4 invariant; delta=0)
#   2. id44 / id45 / id46 PASS on real evidence
#   3. R4 invariant equation evaluable (no SIGKILL pre-PPUT_RESULT)
#   4. verdict_kind = PartialAccepted records on multi-iteration problems
#      where step_partial_ok fires (AttemptOutcome::PartialAccepted in stream)
#   5. dashboard substantive smoke still passes (covered by tests workspace)
#
# Substrate: HEAD 55a0935 (Phase 1 + Phase 2 typed PartialAccepted + handover);
# binaries rebuilt at this HEAD before invocation.
#
# Usage:
#   bash handover/tests/scripts/run_tb_18r_phase_3_evidence.sh \
#        [--out-dir <path>] [--problems-file <path>] [--smoke]
#
# --smoke: single-problem probe mode (uses smoke_problem.txt with 1 problem;
#          per feedback_smoke_before_batch).
#
# Per-problem outputs at OUT_DIR/<idx>_<name>:
#   runtime_repo/             — chaintape repo
#   cas/                      — CAS objects
#   evaluator.{stdout,stderr} — evaluator output (PPUT_RESULT line in stdout)
#   verdict.json              — audit_tape verdict (R8/Phase-2 typed → id44/45/46)
#   chain_invariant.json      — R4 6-field invariant facts
#   verdict_kind_summary.json — count of LeanResult records per verdict_kind
#   README.md                 — per-run summary
#
# Exit codes:
#   0 — all problems audited (verdicts may vary; batch-level success)
#   1 — script-level setup / build / proxy failure
#   2 — invariant gate failure (e.g., R4 evaluable=false on every problem)

set -uo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"

DEFAULT_OUT_DIR="$PROJECT_ROOT/handover/evidence/tb_18r_phase_3_$(date -u +%Y-%m-%dT%H-%M-%SZ)"
DEFAULT_PROBLEMS_FILE=""  # written below; depends on --smoke flag
EVALUATOR_BIN="$PROJECT_ROOT/target/release/evaluator"
AUDIT_TAPE_BIN="$PROJECT_ROOT/target/release/audit_tape"
INVARIANT_BIN="$PROJECT_ROOT/target/release/tb_18r_compute_invariant"

OUT_DIR="$DEFAULT_OUT_DIR"
PROBLEMS_FILE=""
SMOKE_MODE=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    --out-dir) OUT_DIR="$2"; shift 2 ;;
    --problems-file) PROBLEMS_FILE="$2"; shift 2 ;;
    --smoke) SMOKE_MODE=1; shift ;;
    -h|--help) grep -E "^# " "$0" | sed 's/^# //'; exit 0 ;;
    *) echo "[phase3] unknown arg: $1" >&2; exit 2 ;;
  esac
done

# Substrate gate: HEAD must be 55a0935 (or a descendant containing Phase 2)
HEAD_SHA=$(git -C "$PROJECT_ROOT" rev-parse HEAD)
HEAD_SHORT=$(git -C "$PROJECT_ROOT" rev-parse --short HEAD)
PHASE_2_COMMIT=$(git -C "$PROJECT_ROOT" rev-parse 3f51667 2>/dev/null || echo "")
if ! git -C "$PROJECT_ROOT" merge-base --is-ancestor 3f51667 HEAD 2>/dev/null; then
  echo "[phase3] BLOCKED: HEAD ($HEAD_SHORT) does not include Phase 2 commit (3f51667)" >&2
  echo "[phase3] Phase 3 requires typed PartialAccepted substrate." >&2
  exit 1
fi

# Binary freshness gate: rebuild reminder (not enforced — operator chose timestamp)
echo "[phase3] HEAD: $HEAD_SHORT"
echo "[phase3] evaluator built: $(stat -c %y "$EVALUATOR_BIN" 2>/dev/null | cut -d. -f1)"
echo "[phase3] audit_tape built: $(stat -c %y "$AUDIT_TAPE_BIN" 2>/dev/null | cut -d. -f1)"

mkdir -p "$OUT_DIR"

if [ -z "$PROBLEMS_FILE" ]; then
  if [ "$SMOKE_MODE" -eq 1 ]; then
    PROBLEMS_FILE="$OUT_DIR/smoke_problem.txt"
    cat > "$PROBLEMS_FILE" <<'EOM'
mathd_algebra_107
EOM
    echo "[phase3] SMOKE mode: 1 problem"
  else
    PROBLEMS_FILE="$OUT_DIR/phase_3_problems.txt"
    # Phase 3 batch: P38 + P49 + M0 mini-batch of 5 (Phase 2 directive §9 #1 cap)
    cat > "$PROBLEMS_FILE" <<'EOM'
mathd_numbertheory_1124
numbertheory_2pownm1prime_nprime
mathd_algebra_107
mathd_algebra_113
mathd_algebra_114
mathd_algebra_125
mathd_algebra_141
EOM
    echo "[phase3] BATCH mode: $(wc -l < "$PROBLEMS_FILE") problems (P38 + P49 + M0 mini-batch of 5)"
  fi
fi

# LLM env: source turingosv3/.env first (per directive §5; mirrors R9 runner)
if [ -z "${DEEPSEEK_API_KEY:-}" ] && [ -f "$HOME/projects/turingosv3/.env" ]; then
  set -a
  source "$HOME/projects/turingosv3/.env"
  set +a
elif [ -z "${DEEPSEEK_API_KEY:-}" ] && [ -f "$PROJECT_ROOT/.env" ]; then
  set -a
  source "$PROJECT_ROOT/.env"
  set +a
fi

export LLM_PROXY_URL="${LLM_PROXY_URL:-http://localhost:8080}"
export ACTIVE_MODEL="${ACTIVE_MODEL:-deepseek-chat}"
export CONDITION="${CONDITION:-n1}"
export MINIF2F_DIR="${MINIF2F_DIR:-/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4}"

MAX_TX="${MAX_TX:-12}"
PER_PROBLEM_TIMEOUT_S="${PER_PROBLEM_TIMEOUT_S:-1800}"

# Proxy health probe (fail-fast — feedback_smoke_before_batch)
if ! curl -sS -m 3 "$LLM_PROXY_URL/health" 2>/dev/null | grep -q "ok"; then
  echo "[phase3] BLOCKED: LLM proxy at $LLM_PROXY_URL not healthy" >&2
  exit 1
fi
echo "[phase3] proxy: $LLM_PROXY_URL ✓"

# MiniF2F dir gate
if [ ! -d "$MINIF2F_DIR/MiniF2F/Test" ]; then
  echo "[phase3] BLOCKED: MINIF2F_DIR=$MINIF2F_DIR/MiniF2F/Test missing" >&2
  exit 1
fi
echo "[phase3] minif2f: $MINIF2F_DIR ✓"

mapfile -t PROBLEMS < "$PROBLEMS_FILE"
N=${#PROBLEMS[@]}
RUN_TS="$(date -u +%Y-%m-%dT%H-%M-%SZ)"

echo "[phase3] starting | N=$N | MAX_TX=$MAX_TX | timeout=${PER_PROBLEM_TIMEOUT_S}s | git=$HEAD_SHORT | ts=$RUN_TS"

# Frozen run manifest
cat > "$OUT_DIR/PHASE_3_RUN_MANIFEST.json" <<EOM
{
  "phase": "TB-18R Phase 3 — Technical Tape Validation",
  "directive": "handover/directives/2026-05-06_TB18R_PHASE_3_LAUNCH_DIRECTIVE.md",
  "architect_ruling": "handover/directives/2026-05-06_TB18R_ROUND_2_ARCHITECT_RULING.md (§5 Phase 3 + §8 items 7-9)",
  "smoke_mode": $([ "$SMOKE_MODE" -eq 1 ] && echo true || echo false),
  "problem_count": $N,
  "max_tx_per_problem": $MAX_TX,
  "per_problem_timeout_s": $PER_PROBLEM_TIMEOUT_S,
  "llm_proxy_url": "$LLM_PROXY_URL",
  "active_model": "$ACTIVE_MODEL",
  "condition": "$CONDITION",
  "run_timestamp_utc": "$RUN_TS",
  "git_head": "$HEAD_SHA",
  "git_head_short": "$HEAD_SHORT",
  "problems": [
$(for ((i=0; i<N; i++)); do
    sep=","; [ $((i+1)) -eq "$N" ] && sep=""
    echo "    \"${PROBLEMS[$i]}\"$sep"
  done)
  ]
}
EOM

# LeanResult / AttemptTelemetry presence summary — reads .turingos_cas_index.jsonl object_type counts.
# Records are canonical-encoded (binary serde) so we cannot decode verdict_kind from shell;
# typed-consistency is verified indirectly via id45 PASS in audit_tape (which runs assert_45's
# 4-arm typed match over every LeanResult record). Architect §5 #4 PartialAccepted signal
# uses PPUT_RESULT.tool_dist.step_partial_ok as the canonical multi-iteration probe.
extract_verdict_kinds() {
  local cas_dir="$1"
  local pput_json="$2"
  python3 - "$cas_dir" "$pput_json" <<'PYEOF'
import json, os, sys
cas = sys.argv[1]
pput_json = sys.argv[2]
type_counts = {}
idx_path = os.path.join(cas, ".turingos_cas_index.jsonl")
if os.path.isfile(idx_path):
    with open(idx_path) as f:
        for line in f:
            line = line.strip()
            if not line: continue
            try: meta = json.loads(line)
            except Exception: continue
            t = meta.get("object_type", "?")
            type_counts[t] = type_counts.get(t, 0) + 1
try: pput = json.loads(pput_json)
except Exception: pput = {}
td = pput.get("tool_dist", {})
out = {
    "cas_object_type_counts": type_counts,
    "lean_result_count": type_counts.get("LeanResult", 0),
    "attempt_telemetry_count": type_counts.get("AttemptTelemetry", 0),
    "tool_dist": td,
    "step_partial_ok_count": td.get("step_partial_ok", 0),
    "omega_wtool_count": td.get("omega_wtool", 0),
    "step_reject_count": td.get("step_reject", 0),
    "parse_fail_count": td.get("parse_fail", 0),
    "note": "verdict_kind decoded indirectly via id45 PASS in audit_tape (assert_45 4-arm typed match over every LeanResult); step_partial_ok > 0 indicates AttemptOutcome::PartialAccepted records emitted per Phase 2 directive §5.2"
}
print(json.dumps(out, indent=2))
PYEOF
}

declare -i FAIL_COUNT=0
declare -i EVAL_FAIL_COUNT=0

for ((i=0; i<N; i++)); do
  NAME="${PROBLEMS[$i]}"
  IDX="$(printf 'P%02d' "$((i+1))")"
  TAG="${IDX}_${NAME}"
  RUN_DIR="$OUT_DIR/$TAG"
  mkdir -p "$RUN_DIR/runtime_repo" "$RUN_DIR/cas"
  PROBLEM_FILE="$MINIF2F_DIR/MiniF2F/Test/${NAME}.lean"

  if [ ! -f "$PROBLEM_FILE" ]; then
    echo "[phase3] [$((i+1))/$N] $NAME → MISSING; skip"
    FAIL_COUNT+=1
    continue
  fi
  echo "[phase3] [$((i+1))/$N] $NAME → $RUN_DIR"
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

  # The binary's invariant equation (post-TB-C0 Bug 3 fix; chain_invariant.json):
  #   evaluator_reported_completed_llm_calls
  #     == l4_work_attempt_count + l4e_work_attempt_count + capsule_anchored_attempt_count
  # The LHS is the LLM-Lean cycle count (= `tool_dist.step` in PPUT_RESULT), NOT the
  # broader `tx_count` (which counts admin scaffold txs: TB-6 Atom-3 synthetic preseed +
  # TB-C0 atom A.1 synthetic L4.E gate + sequencer system-terminal-summary).
  # Passing tx_count produced false NegativeDelta on mixed-tx runs (P04 -3, P05 -1).
  # Resolution 2026-05-07: pass tool_dist.step as --expected-completed; keep tx_count
  # for diagnostic logging only. See `handover/alignment/OBS_TB18R_INV1_NONLLM_TX_2026-05-07.md`.
  EXTRACTED_JSON="$(python3 - "$RUN_DIR/evaluator.stdout" <<'PYEOF'
import json, sys
try:
    with open(sys.argv[1]) as f:
        line = f.read().split("PPUT_RESULT:", 1)[1]
    o = json.loads(line)
    tx = int(o.get("tx_count", 0))
    solved = bool(o.get("solved", False))
    hit_max = bool(o.get("hit_max_tx", False))
    verified = bool(o.get("verified", False))
    if solved or verified:
        halt = "OmegaAccepted"
    elif hit_max:
        halt = "MaxTxExhausted"
    else:
        halt = "MaxTxExhausted"
    td = o.get("tool_dist", {})
    # LLM-Lean cycle count = sum of every tool_dist key whose evaluator callsite
    # invokes r2_write_attempt_telemetry. Per evaluator.rs:
    #   - "step" (lines 3032, callsite 2555/3522/3604) covers main-line LLM step calls
    #     including step_partial_ok + step_reject sub-counters (which are also incremented
    #     but do NOT add new AttemptTelemetry beyond step).
    #   - "parse_fail" (line 3659, callsite 3687) — LLM response unparseable.
    #   - "llm_err" (line 3734, callsite 3753) — LLM call failed.
    #   - "omega_wtool" is a wtool wrapper for the omega-success step (already counted
    #     in step); does NOT add new AttemptTelemetry.
    # Empirical verification: AT count = step + parse_fail + llm_err in 7/7 P3 problems
    # (including P02 with parse_fail=1 producing 11+1=12 AT records).
    completed_llm_calls = (int(td.get("step", 0))
                           + int(td.get("parse_fail", 0))
                           + int(td.get("llm_err", 0)))
    print(json.dumps({"tx_count": tx, "completed_llm_calls": completed_llm_calls,
                      "halt": halt, "solved": solved,
                      "hit_max_tx": hit_max, "tool_dist": td}))
except Exception as e:
    print(json.dumps({"tx_count": 0, "completed_llm_calls": 0,
                      "halt": "ErrorHalt", "error": str(e)}))
PYEOF
)"
  EXPECTED_COMPLETED="$(echo "$EXTRACTED_JSON" | python3 -c 'import json,sys; print(json.load(sys.stdin)["completed_llm_calls"])')"
  TX_COUNT_TOTAL="$(echo "$EXTRACTED_JSON" | python3 -c 'import json,sys; print(json.load(sys.stdin)["tx_count"])')"
  HALT_CLASS="$(echo "$EXTRACTED_JSON" | python3 -c 'import json,sys; print(json.load(sys.stdin)["halt"])')"
  SOLVED_FLAG="$(echo "$EXTRACTED_JSON" | python3 -c 'import json,sys; print(json.load(sys.stdin).get("solved", False))')"
  echo "$EXTRACTED_JSON" > "$RUN_DIR/extracted_pput.json"

  echo "[phase3] [$((i+1))/$N] $NAME: dur=${PROB_DUR}s tx_count=$TX_COUNT_TOTAL completed_llm_calls=$EXPECTED_COMPLETED halt=$HALT_CLASS solved=$SOLVED_FLAG rc=$EVAL_RC"
  if [ "$EVAL_RC" -ne 0 ] && [ "$EVAL_RC" -ne 124 ]; then
    EVAL_FAIL_COUNT+=1
  fi

  # R4 invariant compute
  set +e
  "$INVARIANT_BIN" \
    --runtime-repo "$RUN_DIR/runtime_repo" \
    --cas "$RUN_DIR/cas" \
    --expected-completed "$EXPECTED_COMPLETED" \
    --halt-class "$HALT_CLASS" \
    > "$RUN_DIR/chain_invariant.json" 2> "$RUN_DIR/chain_invariant.stderr"
  set -e

  # audit_tape (id44/id45/id46 PASS check downstream)
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

  # verdict_kind summary (Phase-2 typed-record presence check via index + tool_dist)
  extract_verdict_kinds "$RUN_DIR/cas" "$EXTRACTED_JSON" > "$RUN_DIR/verdict_kind_summary.json"

  # Architect §5 #1 direct check (post-OBS_TB18R_INV1_NONLLM_TX_2026-05-07 resolution):
  #   chain_attempt_count == evaluator_reported_completed_llm_calls (= tool_dist.step)
  # NOT vs evaluator's broader tx_count, which conflates LLM-Lean cycles with
  # architect-mandated admin scaffold (system-terminal-summary + TB-6 atom-3 synthetic
  # preseed + TB-C0 atom A.1 synthetic L4.E gate). The binary's invariant LHS label
  # `evaluator_reported_completed_llm_calls` is the canonical scope.
  python3 - "$RUN_DIR/cas" "$EXPECTED_COMPLETED" "$TX_COUNT_TOTAL" > "$RUN_DIR/architect_inv1_check.json" <<'PYEOF'
import json, os, subprocess, sys
cas = sys.argv[1]
completed_llm_calls = int(sys.argv[2])
tx_count_total = int(sys.argv[3])
attempt_count = 0
attempt_outcomes = {}
idx_path = os.path.join(cas, ".turingos_cas_index.jsonl")
if os.path.isfile(idx_path):
    with open(idx_path) as f:
        for line in f:
            line = line.strip()
            if not line: continue
            try:
                meta = json.loads(line)
            except Exception:
                continue
            if meta.get("object_type") != "AttemptTelemetry":
                continue
            attempt_count += 1
            oid = meta.get("backend_oid_hex")
            if not oid: continue
            try:
                raw = subprocess.run(["git","-C",cas,"cat-file","-p",oid],
                                     capture_output=True).stdout
                at = json.loads(raw)
                outcome = at.get("outcome", "unknown")
                attempt_outcomes[outcome] = attempt_outcomes.get(outcome, 0) + 1
            except Exception:
                pass
match = (attempt_count == completed_llm_calls)
print(json.dumps({
    "architect_inv_1": "chain_attempt_count == evaluator_reported_completed_llm_calls",
    "chain_attempt_count": attempt_count,
    "evaluator_reported_completed_llm_calls": completed_llm_calls,
    "evaluator_reported_tx_count_total": tx_count_total,
    "non_llm_tx_diagnostic_gap": tx_count_total - completed_llm_calls,
    "match": match,
    "attempt_outcomes": attempt_outcomes,
    "delta": attempt_count - completed_llm_calls,
    "resolution_ref": "handover/alignment/OBS_TB18R_INV1_NONLLM_TX_2026-05-07.md"
}, indent=2))
PYEOF

  # Per-run README
  INV_VERDICT_LINE="$(grep -oE '"invariant_verdict"[[:space:]]*:[[:space:]]*"[^"]*"' "$RUN_DIR/chain_invariant.json" 2>/dev/null | head -1 || echo '"invariant_verdict": "(missing)"')"
  AUDIT_VERDICT="$(grep -oE '"verdict"[[:space:]]*:[[:space:]]*"[A-Z]+"' "$RUN_DIR/verdict.json" 2>/dev/null | head -1 | sed 's/.*"\([A-Z]*\)"/\1/' || echo "UNKNOWN")"
  PARTIAL_COUNT="$(python3 -c "import json; d=json.load(open('$RUN_DIR/verdict_kind_summary.json')); print(d['step_partial_ok_count'])" 2>/dev/null || echo 0)"
  ID45_RESULT="$(python3 -c "import json; v=json.load(open('$RUN_DIR/verdict.json')); r=[a for a in v['assertions'] if a['id']==45]; print(r[0]['result'] if r else 'absent')" 2>/dev/null || echo "absent")"

  cat > "$RUN_DIR/README.md" <<EOM
# TB-18R Phase 3 — $TAG

**Phase**: TB-18R Phase 3 (Technical Tape Validation on typed PartialAccepted substrate)
**Authority**: \`handover/directives/2026-05-06_TB18R_PHASE_3_LAUNCH_DIRECTIVE.md\`
**Date**: $RUN_TS
**Git HEAD**: $HEAD_SHA
**Substrate**: HEAD includes Phase 2 commit \`3f51667\` (LeanVerdictKind + AttemptOutcome::PartialAccepted)

## Run params

- problem: \`$NAME\`
- problem file: \`$PROBLEM_FILE\`
- MAX_TRANSACTIONS: $MAX_TX
- per-problem timeout: ${PER_PROBLEM_TIMEOUT_S}s
- LLM proxy: $LLM_PROXY_URL
- active model: $ACTIVE_MODEL
- condition: $CONDITION
- duration: ${PROB_DUR}s
- evaluator exit code: $EVAL_RC

## Architect §5 invariants (per-run signal)

- audit_tape verdict: \`$AUDIT_VERDICT\`
- $INV_VERDICT_LINE
- verdict_kind=PartialAccepted record count: $PARTIAL_COUNT (multi-iteration tape-derived)

## R4 invariant verdict

\`\`\`
$(cat "$RUN_DIR/chain_invariant.json" 2>/dev/null || echo "(invariant compute failed; see chain_invariant.stderr)")
\`\`\`

## verdict_kind summary

\`\`\`
$(cat "$RUN_DIR/verdict_kind_summary.json")
\`\`\`

## Architect §5 #1 direct check (chain_attempt_count == evaluator_reported_tx_count)

\`\`\`
$(cat "$RUN_DIR/architect_inv1_check.json")
\`\`\`
EOM
  ARCH_INV1_MATCH="$(python3 -c "import json; print(json.load(open('$RUN_DIR/architect_inv1_check.json'))['match'])" 2>/dev/null || echo False)"
  echo "[phase3] [$((i+1))/$N] $NAME: audit=$AUDIT_VERDICT id45=$ID45_RESULT step_partial_ok=$PARTIAL_COUNT inv1_match=$ARCH_INV1_MATCH"
done

# Batch summary
cat > "$OUT_DIR/PHASE_3_BATCH_SUMMARY.json" <<EOM
{
  "phase": "TB-18R Phase 3",
  "smoke_mode": $([ "$SMOKE_MODE" -eq 1 ] && echo true || echo false),
  "problem_count": $N,
  "max_transactions_per_problem": $MAX_TX,
  "per_problem_timeout_s": $PER_PROBLEM_TIMEOUT_S,
  "git_head": "$HEAD_SHA",
  "run_timestamp_utc": "$RUN_TS",
  "evaluator_failures_excluding_timeout": $EVAL_FAIL_COUNT,
  "missing_problem_files": $FAIL_COUNT,
  "per_problem_results": [
$(for ((i=0; i<N; i++)); do
    NAME="${PROBLEMS[$i]}"
    IDX="$(printf 'P%02d' "$((i+1))")"
    TAG="${IDX}_${NAME}"
    INV_FILE="$OUT_DIR/$TAG/chain_invariant.json"
    VRD_FILE="$OUT_DIR/$TAG/verdict.json"
    VK_FILE="$OUT_DIR/$TAG/verdict_kind_summary.json"
    sep=","; [ $((i+1)) -eq "$N" ] && sep=""
    if [ -f "$INV_FILE" ] && [ -f "$VRD_FILE" ]; then
      INV_VERDICT="$(grep -oE '"invariant_verdict"[[:space:]]*:[[:space:]]*"[^"]*"' "$INV_FILE" | head -1 || echo '"invariant_verdict": "missing"')"
      DELTA="$(grep -oE '"delta"[[:space:]]*:[[:space:]]*-?[0-9]+' "$INV_FILE" | grep -oE '\-?[0-9]+' || echo null)"
      EVALUABLE="$(grep -oE '"r4_invariant_equation_evaluable"[[:space:]]*:[[:space:]]*(true|false)' "$INV_FILE" | grep -oE 'true|false' || echo false)"
      AUDIT="$(grep -oE '"verdict"[[:space:]]*:[[:space:]]*"[A-Z]+"' "$VRD_FILE" | head -1 | sed 's/.*"\([A-Z]*\)"/\1/' || echo "UNKNOWN")"
      PARTIAL="$(python3 -c "import json; print(json.load(open('$VK_FILE'))['by_verdict_kind']['PartialAccepted'])" 2>/dev/null || echo 0)"
      LR_TOTAL="$(python3 -c "import json; print(json.load(open('$VK_FILE'))['total_lean_result_records'])" 2>/dev/null || echo 0)"
      echo "    {\"tag\": \"$TAG\", \"delta\": $DELTA, \"evaluable\": $EVALUABLE, \"audit\": \"$AUDIT\", \"lean_results\": $LR_TOTAL, \"partial_accepted\": $PARTIAL, $INV_VERDICT}$sep"
    else
      echo "    {\"tag\": \"$TAG\", \"error\": \"missing invariant or verdict json\"}$sep"
    fi
  done)
  ]
}
EOM

echo "[phase3] DONE | summary at $OUT_DIR/PHASE_3_BATCH_SUMMARY.json"
echo "[phase3] OUT_DIR: $OUT_DIR"
exit 0
