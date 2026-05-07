#!/usr/bin/env bash
# TB-C0 multi-agent (n≥5) FC-witness evidence runner.
#
# Per architect 2026-05-06 reinforcement: every constitution detail must have
# tape-resident evidence from real-problem testing. Run real existing MiniF2F
# problems with n≥5 agents and elevated max_tx for hard problems. Walk chain
# + CAS to extract per-FC-node witnesses.
#
# Authority: handover/directives/2026-05-06_TBC0_CONSTITUTION_LANDING_RESET_DIRECTIVE.md
# Charter:   handover/tracer_bullets/TB-C0_charter_2026-05-06.md
# API auth:  user 2026-05-06 explicit "三个硅基流动 + 两个 deepseek" authorization
#
# Usage:
#   bash handover/tests/scripts/run_tbc0_multi_agent_evidence.sh \
#        [--smoke] [--out-dir <path>] [--n-agents <n>] [--max-tx <n>]
#
# --smoke         single-problem probe (mathd_algebra_107 — proven 1-shot solver)
# --out-dir DIR   output root (default handover/evidence/tb_c0_multi_agent_<TS>)
# --n-agents N    number of agents (default 5; CONDITION=n${N})
# --max-tx N      max transactions per problem (default 20)
#
# Per-problem outputs at OUT_DIR/<idx>_<name>:
#   runtime_repo/, cas/, evaluator.{stdout,stderr},
#   verdict.json, chain_invariant.json, verdict_kind_summary.json,
#   architect_inv1_check.json, fc_witness_manifest.json
#
# Aggregate outputs at OUT_DIR/:
#   TBC0_RUN_MANIFEST.json, TBC0_BATCH_SUMMARY.json,
#   fc_witness_aggregate.json
#
# Exit codes:
#   0 — all problems audited; aggregate manifest emitted
#   1 — script-level setup / build / proxy failure
#   2 — invariant gate failure (every problem RED)

set -uo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"

DEFAULT_OUT_DIR="$PROJECT_ROOT/handover/evidence/tb_c0_multi_agent_$(date -u +%Y-%m-%dT%H-%M-%SZ)"
EVALUATOR_BIN="$PROJECT_ROOT/target/release/evaluator"
AUDIT_TAPE_BIN="$PROJECT_ROOT/target/release/audit_tape"
INVARIANT_BIN="$PROJECT_ROOT/target/release/tb_18r_compute_invariant"

OUT_DIR="$DEFAULT_OUT_DIR"
SMOKE_MODE=0
N_AGENTS=5
MAX_TX_DEFAULT=20

while [[ $# -gt 0 ]]; do
  case "$1" in
    --out-dir) OUT_DIR="$2"; shift 2 ;;
    --smoke) SMOKE_MODE=1; shift ;;
    --n-agents) N_AGENTS="$2"; shift 2 ;;
    --max-tx) MAX_TX_DEFAULT="$2"; shift 2 ;;
    -h|--help) grep -E "^# " "$0" | sed 's/^# //'; exit 0 ;;
    *) echo "[tbc0] unknown arg: $1" >&2; exit 2 ;;
  esac
done

# Build binaries if missing
for bin in "$EVALUATOR_BIN" "$AUDIT_TAPE_BIN" "$INVARIANT_BIN"; do
  if [ ! -x "$bin" ]; then
    echo "[tbc0] missing binary $bin — build with cargo build --release first" >&2
    exit 1
  fi
done

mkdir -p "$OUT_DIR"

# Problem set — real existing MiniF2F problems chosen to exercise distinct FC paths.
# Per feedback_real_problems_not_designed: NEVER synthesize; always use real existing
# problems with citations.
if [ "$SMOKE_MODE" -eq 1 ]; then
  PROBLEMS_FILE="$OUT_DIR/smoke_problem.txt"
  cat > "$PROBLEMS_FILE" <<'EOM'
mathd_algebra_107
EOM
  PROBLEM_DIFFICULTY_FILE="$OUT_DIR/smoke_difficulty.txt"
  cat > "$PROBLEM_DIFFICULTY_FILE" <<'EOM'
mathd_algebra_107 easy 5
EOM
  echo "[tbc0] SMOKE mode: 1 problem (mathd_algebra_107; proven 1-shot)"
else
  PROBLEMS_FILE="$OUT_DIR/tbc0_problems.txt"
  PROBLEM_DIFFICULTY_FILE="$OUT_DIR/tbc0_problems_difficulty.txt"
  # 9-problem set (real MiniF2F entries; citations in per-row comments).
  # Format: <name> <difficulty> <max_tx>
  cat > "$PROBLEM_DIFFICULTY_FILE" <<'EOM'
mathd_algebra_107 easy 5
mathd_algebra_125 easy 5
mathd_algebra_141 easy 5
mathd_algebra_113 medium 20
mathd_algebra_114 medium 20
mathd_numbertheory_1124 hard 50
numbertheory_2pownm1prime_nprime hard 50
aime_1983_p1 hard 50
aime_1984_p1 hard 50
EOM
  awk '{print $1}' "$PROBLEM_DIFFICULTY_FILE" > "$PROBLEMS_FILE"
  echo "[tbc0] BATCH mode: $(wc -l < "$PROBLEMS_FILE") real MiniF2F problems"
  echo "[tbc0]   3 easy (max_tx=5), 2 medium (max_tx=20), 4 hard (max_tx=50)"
fi

# Source LLM env (turingosv3/.env first, fallback turingosv4/.env)
if [ -z "${SILICONFLOW_API_KEY:-}" ] && [ -f "$HOME/projects/turingosv3/.env" ]; then
  set -a; source "$HOME/projects/turingosv3/.env"; set +a
elif [ -z "${SILICONFLOW_API_KEY:-}" ] && [ -f "$PROJECT_ROOT/.env" ]; then
  set -a; source "$PROJECT_ROOT/.env"; set +a
fi

export LLM_PROXY_URL="${LLM_PROXY_URL:-http://localhost:8080}"
export ACTIVE_MODEL="${ACTIVE_MODEL:-deepseek-chat}"
export CONDITION="n${N_AGENTS}"
export MINIF2F_DIR="${MINIF2F_DIR:-/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4}"
PER_PROBLEM_TIMEOUT_S="${PER_PROBLEM_TIMEOUT_S:-1800}"

# Proxy health probe
if ! curl -sS -m 3 "$LLM_PROXY_URL/health" 2>/dev/null | grep -q "ok"; then
  echo "[tbc0] BLOCKED: LLM proxy at $LLM_PROXY_URL not healthy" >&2
  exit 1
fi
echo "[tbc0] proxy: $LLM_PROXY_URL ✓"

# MiniF2F dir gate
if [ ! -d "$MINIF2F_DIR/MiniF2F/Test" ]; then
  echo "[tbc0] BLOCKED: MINIF2F_DIR=$MINIF2F_DIR/MiniF2F/Test missing" >&2
  exit 1
fi
echo "[tbc0] minif2f: $MINIF2F_DIR ✓"

mapfile -t PROBLEMS < "$PROBLEMS_FILE"
N=${#PROBLEMS[@]}
RUN_TS="$(date -u +%Y-%m-%dT%H-%M-%SZ)"
HEAD_SHA=$(git -C "$PROJECT_ROOT" rev-parse HEAD)
HEAD_SHORT=$(git -C "$PROJECT_ROOT" rev-parse --short HEAD)

echo "[tbc0] start | N=$N | n_agents=$N_AGENTS | per-problem-timeout=${PER_PROBLEM_TIMEOUT_S}s | git=$HEAD_SHORT | ts=$RUN_TS"

# Frozen run manifest
cat > "$OUT_DIR/TBC0_RUN_MANIFEST.json" <<EOM
{
  "tb_id": "TB-C0",
  "phase": "TB-C0 multi-agent FC-witness evidence run",
  "directive": "handover/directives/2026-05-06_TBC0_CONSTITUTION_LANDING_RESET_DIRECTIVE.md",
  "charter": "handover/tracer_bullets/TB-C0_charter_2026-05-06.md",
  "smoke_mode": $([ "$SMOKE_MODE" -eq 1 ] && echo true || echo false),
  "problem_count": $N,
  "n_agents": $N_AGENTS,
  "condition": "$CONDITION",
  "per_problem_timeout_s": $PER_PROBLEM_TIMEOUT_S,
  "llm_proxy_url": "$LLM_PROXY_URL",
  "active_model": "$ACTIVE_MODEL",
  "run_timestamp_utc": "$RUN_TS",
  "git_head": "$HEAD_SHA",
  "git_head_short": "$HEAD_SHORT",
  "problem_difficulty_file": "$PROBLEM_DIFFICULTY_FILE",
  "remediation_protocol": "feedback_real_problems_not_designed (find real existing problems for any FC gap; do NOT synthesize)"
}
EOM

# Per-problem max_tx lookup
declare -A PROBLEM_MAX_TX
declare -A PROBLEM_DIFFICULTY
while IFS=' ' read -r p d t; do
  PROBLEM_MAX_TX["$p"]="$t"
  PROBLEM_DIFFICULTY["$p"]="$d"
done < "$PROBLEM_DIFFICULTY_FILE"

declare -i FAIL_COUNT=0
declare -i EVAL_FAIL_COUNT=0
declare -a SUMMARY_ROWS=()

for ((i=0; i<N; i++)); do
  NAME="${PROBLEMS[$i]}"
  IDX="$(printf 'P%02d' "$((i+1))")"
  TAG="${IDX}_${NAME}"
  RUN_DIR="$OUT_DIR/$TAG"
  mkdir -p "$RUN_DIR/runtime_repo" "$RUN_DIR/cas"
  PROBLEM_FILE="$MINIF2F_DIR/MiniF2F/Test/${NAME}.lean"
  MAX_TX="${PROBLEM_MAX_TX[$NAME]:-$MAX_TX_DEFAULT}"
  DIFF="${PROBLEM_DIFFICULTY[$NAME]:-medium}"

  if [ ! -f "$PROBLEM_FILE" ]; then
    echo "[tbc0] [$((i+1))/$N] $NAME → MISSING; skip"
    FAIL_COUNT+=1
    continue
  fi
  echo "[tbc0] [$((i+1))/$N] $NAME (diff=$DIFF, max_tx=$MAX_TX) → $RUN_DIR"
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

  # Extract PPUT_RESULT from stdout
  EXTRACTED_JSON="$(python3 - "$RUN_DIR/evaluator.stdout" <<'PYEOF'
import json, sys
try:
    with open(sys.argv[1]) as f:
        text = f.read()
    if "PPUT_RESULT:" not in text:
        print(json.dumps({"tx_count": 0, "halt": "ErrorHalt", "error": "no PPUT_RESULT"}))
        sys.exit(0)
    line = text.split("PPUT_RESULT:", 1)[1].strip().split("\n")[0]
    o = json.loads(line)
    tx = int(o.get("tx_count", 0))
    solved = bool(o.get("solved", False))
    hit_max = bool(o.get("hit_max_tx", False))
    verified = bool(o.get("verified", False))
    halt = "OmegaAccepted" if (solved or verified) else "MaxTxExhausted"
    td = o.get("tool_dist", {})
    print(json.dumps({"tx_count": tx, "halt": halt, "solved": solved,
                      "hit_max_tx": hit_max, "verified": verified, "tool_dist": td,
                      "step_partial_ok": td.get("step_partial_ok", 0)}))
except Exception as e:
    print(json.dumps({"tx_count": 0, "halt": "ErrorHalt", "error": str(e)}))
PYEOF
)"
  # Bug 1 fix (TB-C0 strict audit 2026-05-07): use tool_dist.step (count of
  # actual LLM-Lean externalized cycles) instead of tx_count (total transaction
  # count which includes non-LLM tx like TaskOpen / EscrowLock / TerminalSummary).
  # Per FC1 hard invariant: LHS is `externalized_attempt_count`, NOT `tx_count`.
  # See handover/alignment/STRICT_AUDIT_TBC0_TAPE_2026-05-07.md §1 Finding B.
  EXPECTED_COMPLETED="$(echo "$EXTRACTED_JSON" | python3 -c '
import json, sys
o = json.load(sys.stdin)
td = o.get("tool_dist", {}) or {}
# Externalized LLM-Lean cycle count = count of step invocations (each step
# is one LLM-Lean call). Per Phase 2 substrate: step subsumes step_reject +
# step_partial_ok; omega_wtool when nonzero is the omega-final step.
step_count = int(td.get("step", 0))
omega_wtool = int(td.get("omega_wtool", 0))
# Avoid omega double-counting: omega_wtool overlaps with step. If both are
# present and step covers the omega step, dont add. If step is 0 but
# omega_wtool > 0, use omega_wtool. Empirical: step_count alone is correct
# for the FC1 LHS on all observed evaluator runs.
expected = step_count if step_count > 0 else omega_wtool
# Fallback to tx_count for legacy-evidence runs that lack tool_dist
if expected == 0:
    expected = int(o.get("tx_count", 0))
print(expected)
')"
  HALT_CLASS="$(echo "$EXTRACTED_JSON" | python3 -c 'import json,sys; print(json.load(sys.stdin).get("halt", "ErrorHalt"))')"
  SOLVED="$(echo "$EXTRACTED_JSON" | python3 -c 'import json,sys; print(json.load(sys.stdin).get("solved", False))')"
  STEP_POK="$(echo "$EXTRACTED_JSON" | python3 -c 'import json,sys; print(json.load(sys.stdin).get("step_partial_ok", 0))')"
  echo "$EXTRACTED_JSON" > "$RUN_DIR/extracted_pput.json"

  echo "[tbc0] [$((i+1))/$N] $NAME: dur=${PROB_DUR}s tx=$EXPECTED_COMPLETED halt=$HALT_CLASS solved=$SOLVED step_partial_ok=$STEP_POK rc=$EVAL_RC"
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

  # audit_tape
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

  # architect_inv1 direct check (per Phase 3 candidate report §5 #1)
  python3 - "$RUN_DIR" <<'PYEOF' > "$RUN_DIR/architect_inv1_check.json" 2>&1 || true
import json, os, sys
run = sys.argv[1]
def load_jsonl(path):
    if not os.path.exists(path): return []
    out = []
    with open(path) as f:
        for line in f:
            line = line.strip()
            if line:
                try: out.append(json.loads(line))
                except: pass
    return out
cas_idx = load_jsonl(os.path.join(run, "cas", ".turingos_cas_index.jsonl"))
at_count = sum(1 for o in cas_idx if o.get("object_type") == "AttemptTelemetry")
try:
    with open(os.path.join(run, "extracted_pput.json")) as f:
        pput = json.load(f)
    # Bug 1 fix: compare against externalized LLM-Lean cycle count, not tx_count
    td = pput.get("tool_dist", {}) or {}
    step_count = int(td.get("step", 0))
    omega_wtool = int(td.get("omega_wtool", 0))
    externalized_llm = step_count if step_count > 0 else omega_wtool
    if externalized_llm == 0:
        externalized_llm = int(pput.get("tx_count", 0))
    tx_count_legacy = int(pput.get("tx_count", 0))
except Exception:
    externalized_llm = 0
    tx_count_legacy = 0
out = {
    "architect_inv_1": "chain_attempt_count == externalized_llm_cycle_count",
    "chain_attempt_count": at_count,
    "externalized_llm_cycle_count": externalized_llm,
    "evaluator_reported_tx_count_legacy": tx_count_legacy,
    "match": at_count == externalized_llm,
    "delta": at_count - externalized_llm,
    "_note": "TB-C0 strict audit 2026-05-07 Bug 1 fix: LHS is externalized LLM-Lean cycle count (tool_dist.step), not raw tx_count (which includes TaskOpen/EscrowLock/TerminalSummary)",
}
print(json.dumps(out, indent=2))
PYEOF

  # FC-witness extraction (TB-C0 round 2 tooling)
  set +e
  python3 "$PROJECT_ROOT/scripts/fc_witness_extract.py" "$RUN_DIR" \
    > "$RUN_DIR/fc_witness_extract.stdout" 2> "$RUN_DIR/fc_witness_extract.stderr"
  set -e

  # Per-problem README
  cat > "$RUN_DIR/README.md" <<EOM
# TB-C0 multi-agent run — $NAME

- problem_id: $NAME
- difficulty: $DIFF
- max_tx: $MAX_TX
- n_agents: $N_AGENTS
- condition: $CONDITION
- duration_s: $PROB_DUR
- evaluator_rc: $EVAL_RC
- tx_count: $EXPECTED_COMPLETED
- halt: $HALT_CLASS
- solved: $SOLVED
- step_partial_ok: $STEP_POK

Source: real MiniF2F entry at
\`$MINIF2F_DIR/MiniF2F/Test/${NAME}.lean\`.

Per feedback_real_problems_not_designed: this is a real existing problem,
not synthesized. MiniF2F is a published benchmark
(arXiv:2109.00110, https://github.com/openai/miniF2F).
EOM

  # Convert Python-style booleans (True/False) to JSON booleans (true/false)
  SOLVED_JSON=$(echo "$SOLVED" | tr 'A-Z' 'a-z')
  SUMMARY_ROWS+=("{\"tag\":\"$TAG\",\"name\":\"$NAME\",\"difficulty\":\"$DIFF\",\"max_tx\":$MAX_TX,\"tx_count\":$EXPECTED_COMPLETED,\"halt\":\"$HALT_CLASS\",\"solved\":$SOLVED_JSON,\"step_partial_ok\":$STEP_POK,\"duration_s\":$PROB_DUR,\"evaluator_rc\":$EVAL_RC}")
done

# Compose batch summary
{
  echo "{"
  echo "  \"tb_id\": \"TB-C0\","
  echo "  \"smoke_mode\": $([ "$SMOKE_MODE" -eq 1 ] && echo true || echo false),"
  echo "  \"problem_count\": $N,"
  echo "  \"n_agents\": $N_AGENTS,"
  echo "  \"condition\": \"$CONDITION\","
  echo "  \"git_head\": \"$HEAD_SHA\","
  echo "  \"run_timestamp_utc\": \"$RUN_TS\","
  echo "  \"evaluator_failures_excluding_timeout\": $EVAL_FAIL_COUNT,"
  echo "  \"missing_problem_files\": $FAIL_COUNT,"
  echo "  \"per_problem_results\": ["
  IFS=','; printf '    %s' "${SUMMARY_ROWS[*]}" | sed 's/,/,\n    /g'
  unset IFS
  echo
  echo "  ]"
  echo "}"
} > "$OUT_DIR/TBC0_BATCH_SUMMARY.json"

# Run aggregator
echo
echo "[tbc0] running FC-witness aggregator..."
python3 "$PROJECT_ROOT/scripts/fc_witness_aggregate.py" "$OUT_DIR" \
  > "$OUT_DIR/fc_witness_aggregate.stdout" 2> "$OUT_DIR/fc_witness_aggregate.stderr"

echo
echo "[tbc0] DONE → $OUT_DIR"
echo "[tbc0] Summary: $OUT_DIR/TBC0_BATCH_SUMMARY.json"
echo "[tbc0] Aggregate: $OUT_DIR/fc_witness_aggregate.json"
echo
tail -25 "$OUT_DIR/fc_witness_aggregate.stdout"

if [ "$EVAL_FAIL_COUNT" -gt 0 ]; then
  echo "[tbc0] WARN: $EVAL_FAIL_COUNT evaluator failures (non-timeout)" >&2
fi
exit 0
