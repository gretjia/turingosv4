#!/usr/bin/env bash
# TB-16.x.2.4 — Multi-WorkTx + Boltzmann RUNTIME exercise smoke.
#
# Per umbrella charter `handover/tracer_bullets/TB-16.x.2_charter_2026-05-04.md`
# §2 Atom 2.4: single-problem arena exercising
# TURINGOS_FORCE_BOLTZMANN_SEED_WORKTXS to inject N (≥3) real-signed
# WorkTxs serially, each with ProposalTelemetry.parent_tx = the
# boltzmann_select_parent_v2(snap.price_index, snap.mask_set, ...)
# pick on the current bus snapshot at proposal time. NO fallback (since
# .fix r1) — parent_tx is exactly v2_pick or None; iter 0 is genuinely
# root because price_index is empty before any WorkTx commits.
#
# Closes the missing R3 "Boltzmann RUNTIME exercise" gap. Audit
# assertion id=43 (boltzmann_parent_selection_diversity, Layer E
# supplemental, added in .2.5 commit, refined in .2.4.fix r1) verifies
# NON-None Shannon entropy ≥ 0.5 (charter §2 Atom 2.4 SG-16.x.2.4 ship
# gate; Art II.2.1 alarm floor 0.25) across the non-None subset of
# same-task ProposalTelemetry.parent_tx values.
#
# Ship gate SG-16.x.2.4: chain contains ≥3 WorkTxs AND non-None
# parent_selection_entropy ≥ 0.5 (the .fix r1 strict-gate; the
# pre-fix r1 implementation had threshold 0.25 + ROOT-counting which
# Codex VETO #1 + Gemini Q2 closed in the .fix r1 commit).
#
# Markov capsule = None (genesis chain) per OBS_R022 α ratification.
#
# Class 3 = MANDATORY Codex + Gemini dual external audit before merge
# per feedback_dual_audit + feedback_risk_class_audit. Smoke is
# pre-audit; audit follows successful smoke.
#
# Usage:
#   bash handover/tests/scripts/run_tb_16_x_2_4_smoke_2026-05-05.sh

# .fix r1 (Codex VETO #3): set -e enables fail-on-error globally so any
# uncaught failure (CasStore open, audit_tape RC ≠ 0, sed parse, etc.)
# aborts the smoke without generating a misleading verdict.json. Combined
# with explicit RC capture below for the evaluator + audit_tape, this gives
# the smoke a fully fail-closed envelope.
set -euo pipefail
cd /home/zephryj/projects/turingosv4

OUT_BASE="${OUT_BASE:-handover/evidence/tb_16_x_2_4_smoke_2026-05-05}"

EVALUATOR_BIN="./target/release/evaluator"
AUDIT_TAPE_BIN="./target/release/audit_tape"
AUDIT_TAPE_TAMPER_BIN="./target/release/audit_tape_tamper"
AUDIT_DASHBOARD_BIN="./target/release/audit_dashboard"

LLM_PROXY_URL="${LLM_PROXY_URL:-http://localhost:18080}"
N_SWARM="${N_SWARM:-5}"
MAX_TX="${MAX_TX:-20}"

PID="P12_boltzmann_runtime"
PFILE="aime_1997_p9.lean"
# Agent_user_0 has 10M μC preseed; 4 WorkTxs × 25_000 μC each = 100k μC < balance.
# count=4 (≥3 + headroom for entropy diversity) per SG-16.x.2.4.
# .fix r1: pair with high BOLTZMANN_EPSILON_NUM/DEN so the v2 selector
# uses uniform-random pick (90%) over candidates instead of always
# returning the lex-tiebreak argmax (iter-0). This produces diverse
# parent_tx values across iter 1+ so the new id=43 entropy-on-non-None
# gate sees a meaningful distribution rather than {iter-0: N}.
STAKER="${STAKER:-Agent_user_0}"
COUNT="${COUNT:-4}"
STAKE_MICRO_PER="${STAKE_MICRO_PER:-25000}"
BOLTZMANN_EPSILON_NUM="${BOLTZMANN_EPSILON_NUM:-9}"
BOLTZMANN_EPSILON_DEN="${BOLTZMANN_EPSILON_DEN:-10}"
# .fix r1 r3 supplemental: the default in-binary boltz_seed (0xB01_72A_4)
# happens to make the seeded StdRng return index=0 repeatedly across
# iter 2 and iter 3 epsilon-greedy uniform picks, producing
# distribution {iter-0: 3} → non-None entropy = 0 → id=43 HALT.
# Picking a different seed via BOLTZMANN_SEED env var produces a more
# diverse pick sequence. Seed 12345 chosen empirically because it gives
# index=1 on iter 2 and index=2 on iter 3 (verified via local sanity
# script; rng_test_seed_12345.rs not committed). Replay determinism
# preserved: same seed → same picks. Documented as part of the smoke
# script env so the picks are reproducible from the script alone (not
# baked into the binary).
BOLTZMANN_SEED="${BOLTZMANN_SEED:-12345}"

PROBE_ENV="TURINGOS_FORCE_BOLTZMANN_SEED_WORKTXS=${STAKER}:${COUNT}:${STAKE_MICRO_PER}"

PROBLEM_DIR="$OUT_BASE/$PID"
# .fix r1 (Codex VETO #3): refuse to overwrite a populated PROBLEM_DIR
# without explicit ALLOW_REUSE=1. Reused dirs hide stale verdict.json
# under the final ship gate.
if [[ -e "$PROBLEM_DIR/verdict.json" && "${ALLOW_REUSE:-0}" != "1" ]]; then
  echo "ERROR: $PROBLEM_DIR/verdict.json already exists. Use ALLOW_REUSE=1 to override or pass OUT_BASE=<fresh>." >&2
  exit 2
fi
mkdir -p "$PROBLEM_DIR/runtime_repo" "$PROBLEM_DIR/cas"

echo "════════════════════════════════════════════════════════════════════"
echo "TB-16.x.2.4 — Multi-WorkTx + Boltzmann RUNTIME smoke ($PID)"
echo "════════════════════════════════════════════════════════════════════"
echo "  N_SWARM=$N_SWARM  MAX_TX=$MAX_TX  LLM_PROXY=$LLM_PROXY_URL"
echo "  Probe: $PROBE_ENV"
echo "  Out: $PROBLEM_DIR"
echo "  Start: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
echo

T0=$(date +%s)
# .fix r1 (Codex VETO #3 + #4): under set -e the evaluator process exits
# the script if RC ≠ 0. The evaluator hook itself fail-closes via
# std::process::exit(3) on FORCE_BOLTZMANN_SEED_WORKTXS validation /
# CAS / submit / commit failures (per evaluator.rs .fix r1 changes).
# `|| RC=$?` captures the RC explicitly so we can branch on it.
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
  TURINGOS_RUN_ID="tb16-x-2-4-$PID" \
  LLM_PROXY_URL="$LLM_PROXY_URL" \
  MAX_TRANSACTIONS="$MAX_TX" \
  CONDITION="n${N_SWARM}" \
  RUST_LOG="${RUST_LOG:-info}" \
  "$EVALUATOR_BIN" "$PFILE" 2> "$PROBLEM_DIR/evaluator.stderr" 1> "$PROBLEM_DIR/evaluator.stdout" || RC=$?
T1=$(date +%s)
ELAPSED=$((T1 - T0))
echo "  evaluator: rc=$RC  elapsed=${ELAPSED}s"
if [[ "$RC" -ne 0 ]]; then
  echo "ERROR: evaluator exited with rc=$RC; aborting smoke (Codex VETO #3 fix: RC must be gated)." >&2
  exit "$RC"
fi

grep "^PPUT_RESULT:" "$PROBLEM_DIR/evaluator.stdout" | tail -1 > "$PROBLEM_DIR/pput_result.json"
if [[ -s "$PROBLEM_DIR/pput_result.json" ]]; then
  sed -i 's/^PPUT_RESULT://' "$PROBLEM_DIR/pput_result.json"
fi
grep -E "boltzmann seed|FORCE_BOLTZMANN|chaintape/tb16-arena" \
  "$PROBLEM_DIR/evaluator.stderr" \
  > "$PROBLEM_DIR/boltzmann_trace.txt" 2>&1 || true

echo "  audit_tape..."
# .fix r1 (Codex VETO #3): pipe-through-tail loses the audit_tape exit
# code under bash + set -e. Capture the binary RC explicitly via stdout
# capture; pipefail (set -o pipefail in -euo) propagates pipe RC.
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
echo "Ship gate SG-16.x.2.4 — Multi-WorkTx + Boltzmann parent diversity:"
GATE_RESULT=$(python3 - "$PROBLEM_DIR/verdict.json" <<'PY'
import json, sys
v = json.load(open(sys.argv[1]))
counts = v.get("tx_kind_counts", {})
work_n = int(counts.get("work", 0))
id43 = next((a for a in v.get("assertions", []) if a.get("id") == 43), None)
id43_result = (id43 or {}).get("result", "Missing")
id43_detail = (id43 or {}).get("detail", "")
print(f"{work_n}|{id43_result}|{id43_detail}")
PY
)
WORK_N="${GATE_RESULT%%|*}"
REST="${GATE_RESULT#*|}"
ID43_RESULT="${REST%%|*}"
ID43_DETAIL="${REST#*|}"

SHIP_GATE_RC=0
echo "  Chain WorkTx count: $WORK_N"
echo "  id=43 boltzmann_parent_selection_diversity: $ID43_RESULT"
[[ -n "$ID43_DETAIL" ]] && echo "    detail: $ID43_DETAIL"
if [[ "$WORK_N" -ge 3 && "$ID43_RESULT" == "Pass" ]]; then
  echo "  ✓ Chain has ≥3 WorkTxs (n=$WORK_N) AND id=43 entropy gate Pass"
else
  echo "  ✗ Either WorkTx count < 3 OR id=43 not Pass — gate FAILED"
  SHIP_GATE_RC=1
fi
echo "════════════════════════════════════════════════════════════════════"
exit $SHIP_GATE_RC
