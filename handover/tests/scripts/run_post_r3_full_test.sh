#!/usr/bin/env bash
# TB-16 post-R3 — full FC1+FC2+FC3 constitutional conformance test battery.
#
# 5 problems × N=3 swarm × MAX_TX=10 with per-problem env-var probes
# exercising each TB's wired-in flowchart element (TB-1..TB-16 R3).
#
# Per-problem invariants checked:
#   FC1: audit_tape PROCEED + replay byte-identical (every externalized
#        proposal in L4/L4.E; predicate evidence resolves from CAS)
#   FC2: genesis_report.json + initial_q_state.json + replay reconstructs Q₀
#        + total_supply conserved per-block (id=40)
#   FC3: MarkovEvidenceCapsule chained to TB-15 head + tamper 3/3 +
#        sandbox-prefix walker (id=41) covers L4+L4.E
#
# Outputs to: handover/evidence/tb_16_real_llm_arena_2026-05-04/post_r3_full_test/

set -uo pipefail
cd /home/zephryj/projects/turingosv4

OUT_BASE="handover/evidence/tb_16_real_llm_arena_2026-05-04/post_r3_full_test"
mkdir -p "$OUT_BASE"

EVALUATOR_BIN="./target/release/evaluator"
AUDIT_TAPE_BIN="./target/release/audit_tape"
AUDIT_TAPE_TAMPER_BIN="./target/release/audit_tape_tamper"
GEN_MARKOV_BIN="./target/release/generate_markov_capsule"
AUDIT_DASHBOARD_BIN="./target/release/audit_dashboard"

LLM_PROXY_URL="${LLM_PROXY_URL:-http://localhost:18080}"
N_SWARM="${N_SWARM:-3}"
MAX_TX="${MAX_TX:-10}"

# 5 problems × probe env-vars (1-line each: <id>|<problem.lean>|<extra-env>)
PROBLEMS=(
  "P1_baseline|mathd_algebra_171.lean|"
  "P2_challenge|mathd_algebra_11.lean|TURINGOS_FORCE_CHALLENGER=Agent_2"
  "P3_completeset|mathd_algebra_96.lean|TURINGOS_COMPLETE_SET_SEED=Agent_user_0:1000000"
  "P4_bankruptcy|mathd_numbertheory_961.lean|TURINGOS_FORCE_BANKRUPTCY=Agent_0"
  "P5_aime_hard|aime_1997_p9.lean|"
)

START_TS=$(date +%s)
echo "════════════════════════════════════════════════════════════════════"
echo "TB-16 post-R3 full FC conformance test battery"
echo "════════════════════════════════════════════════════════════════════"
echo "  N_SWARM=$N_SWARM  MAX_TX=$MAX_TX  LLM_PROXY=$LLM_PROXY_URL"
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

  # Step 1: real-LLM evaluator on this problem
  T0=$(date +%s)
  env $PROBE_PREFIX \
    TURINGOS_USER_TASK_MODE=1 \
    TURINGOS_USER_TASK_BOUNTY_MICRO=200000 \
    TURINGOS_CHAINTAPE_PATH="$PROBLEM_DIR/runtime_repo" \
    TURINGOS_CAS_PATH="$PROBLEM_DIR/cas" \
    TURINGOS_RUN_ID="tb16-post-r3-full-$PID" \
    LLM_PROXY_URL="$LLM_PROXY_URL" \
    MAX_TRANSACTIONS="$MAX_TX" \
    CONDITION="n${N_SWARM}" \
    "$EVALUATOR_BIN" "$PFILE" 2> "$PROBLEM_DIR/evaluator.stderr" 1> "$PROBLEM_DIR/evaluator.stdout"
  RC=$?
  T1=$(date +%s)
  ELAPSED=$((T1 - T0))
  echo "    evaluator: rc=$RC  elapsed=${ELAPSED}s"

  # Extract PputResult (last line that starts with PPUT_RESULT:)
  grep "^PPUT_RESULT:" "$PROBLEM_DIR/evaluator.stdout" | tail -1 > "$PROBLEM_DIR/pput_result.json"
  if [[ -s "$PROBLEM_DIR/pput_result.json" ]]; then
    sed -i 's/^PPUT_RESULT://' "$PROBLEM_DIR/pput_result.json"
  fi

  # Step 2: audit_tape (FC1+FC2+FC3 invariant check)
  echo "    audit_tape..."
  "$AUDIT_TAPE_BIN" \
    --runtime-repo "$PROBLEM_DIR/runtime_repo" \
    --cas-dir "$PROBLEM_DIR/cas" \
    --agent-pubkeys "$PROBLEM_DIR/runtime_repo/agent_pubkeys.json" \
    --pinned-pubkeys "$PROBLEM_DIR/runtime_repo/pinned_pubkeys.json" \
    --genesis genesis_payload.toml \
    --constitution constitution.md \
    --markov-pointer handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt \
    --alignment-dir handover/alignment \
    --out "$PROBLEM_DIR/verdict.json" 2>&1 | tail -1

  # Step 3: replay determinism check (audit_tape twice → byte-identical)
  echo "    audit_tape replay..."
  "$AUDIT_TAPE_BIN" \
    --runtime-repo "$PROBLEM_DIR/runtime_repo" \
    --cas-dir "$PROBLEM_DIR/cas" \
    --agent-pubkeys "$PROBLEM_DIR/runtime_repo/agent_pubkeys.json" \
    --pinned-pubkeys "$PROBLEM_DIR/runtime_repo/pinned_pubkeys.json" \
    --genesis genesis_payload.toml \
    --constitution constitution.md \
    --markov-pointer handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt \
    --alignment-dir handover/alignment \
    --out "$PROBLEM_DIR/verdict_replay.json" 2>&1 | tail -1
  if cmp -s "$PROBLEM_DIR/verdict.json" "$PROBLEM_DIR/verdict_replay.json"; then
    echo "    ✓ replay byte-identical"
  else
    echo "    ✗ replay diverged"
  fi

  # Step 4: tamper harness (FC1 invariant: detect chain corruption)
  echo "    audit_tape_tamper..."
  "$AUDIT_TAPE_TAMPER_BIN" \
    --runtime-repo "$PROBLEM_DIR/runtime_repo" \
    --cas-dir "$PROBLEM_DIR/cas" \
    --agent-pubkeys "$PROBLEM_DIR/runtime_repo/agent_pubkeys.json" \
    --pinned-pubkeys "$PROBLEM_DIR/runtime_repo/pinned_pubkeys.json" \
    --genesis genesis_payload.toml \
    --constitution constitution.md \
    --markov-pointer handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt \
    --alignment-dir handover/alignment \
    --tamper-dir "$PROBLEM_DIR/tamper" \
    --out "$PROBLEM_DIR/tamper_report.json" 2>&1 | tail -1

  # Step 5: generate_markov_capsule (FC3 invariant: chain to TB-15 head)
  echo "    generate_markov_capsule..."
  PREV_CID="$(tr -d '[:space:]' < handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt)"
  "$GEN_MARKOV_BIN" \
    --tb-id 16 \
    --out-dir "$PROBLEM_DIR" \
    --constitution-path constitution.md \
    --runtime-repo "$PROBLEM_DIR/runtime_repo" \
    --cas-dir "$PROBLEM_DIR/cas" \
    --alignment-dir handover/alignment \
    --prev-cid-hex "$PREV_CID" 2>&1 | grep -E "capsule_id|wrote" | tail -3

  # Step 6: dashboard (FC1 invariant: regeneratable view)
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
echo "Done. Total wall: ${TOTAL_ELAPSED}s. Out: $OUT_BASE"
echo "Next: bash handover/tests/scripts/aggregate_post_r3_full_test.sh"
echo "════════════════════════════════════════════════════════════════════"
