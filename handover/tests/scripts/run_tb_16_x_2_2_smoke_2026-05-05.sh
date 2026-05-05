#!/usr/bin/env bash
# TB-16.x.2.2 — ChallengeResolve via challenge-window scheduler smoke.
#
# Per umbrella charter `handover/tracer_bullets/TB-16.x.2_charter_2026-05-04.md`
# §2 Atom 2.2: single-problem arena exercising TURINGOS_FORCE_CHALLENGER +
# TURINGOS_FORCE_CHALLENGE_RESOLVE on a task expected to OMEGA-Confirm.
#
# Why OMEGA-Confirm and not MaxTxExhaust: FORCE_CHALLENGER fires inside the
# evaluator's post-VerifyTx OMEGA-Confirm path; FORCE_CHALLENGE_RESOLVE fires
# in the pre-bundle.shutdown cleanup (deliberately OUTSIDE the
# MaxTxExhausted EvidenceCapsule conditional, since the two cleanup paths
# are mutually exclusive). Pairing them on the same chain requires the
# Challenge to be admitted (success path) before the cleanup hook runs.
#
# Ship gate SG-16.x.2.2: chain contains parent-child Challenge →
# ChallengeResolve relationship (raises 10-of-13 → 11-of-13 system-emitted
# tx kinds runtime-exercised). New audit assertion id=42
# (challenge_resolve_chain_to_challenge_tx, Layer E supplemental) verifies
# the parent-child invariant.
#
# Markov capsule = None per FC2 Boot + Markov chain genesis semantic
# (fresh runtime_repo + fresh cas; no prior chain). Per TB-16.x.fix
# (architect OBS_R022 Option α RATIFIED 2026-05-04), `--markov-pointer`
# is optional and absence ≡ genesis chain.
#
# Usage:
#   bash handover/tests/scripts/run_tb_16_x_2_2_smoke_2026-05-05.sh

set -uo pipefail
cd /home/zephryj/projects/turingosv4

OUT_BASE="${OUT_BASE:-handover/evidence/tb_16_x_2_2_smoke_2026-05-05}"
mkdir -p "$OUT_BASE"

EVALUATOR_BIN="./target/release/evaluator"
AUDIT_TAPE_BIN="./target/release/audit_tape"
AUDIT_TAPE_TAMPER_BIN="./target/release/audit_tape_tamper"
AUDIT_DASHBOARD_BIN="./target/release/audit_dashboard"

LLM_PROXY_URL="${LLM_PROXY_URL:-http://localhost:18080}"
N_SWARM="${N_SWARM:-5}"
MAX_TX="${MAX_TX:-20}"

PID="P10_challenge_resolve"
PFILE="mathd_algebra_171.lean"
# Agent_3 chosen as challenger because the swarm always includes Agent_0..Agent_4
# (5-agent default) and Agent_3 is unlikely to also be the OMEGA-confirming agent
# on any given run. Self-challenge would be skipped by the FORCE_CHALLENGER guard
# (challenger.as_str() != agent_id.as_str() at evaluator.rs ChallengeTx submit).
PROBE_ENV="TURINGOS_FORCE_CHALLENGER=Agent_3 TURINGOS_FORCE_CHALLENGE_RESOLVE=1"
PROBLEM_DIR="$OUT_BASE/$PID"
mkdir -p "$PROBLEM_DIR/runtime_repo" "$PROBLEM_DIR/cas"

echo "════════════════════════════════════════════════════════════════════"
echo "TB-16.x.2.2 — ChallengeResolve smoke ($PID)"
echo "════════════════════════════════════════════════════════════════════"
echo "  N_SWARM=$N_SWARM  MAX_TX=$MAX_TX  LLM_PROXY=$LLM_PROXY_URL"
echo "  Probe: $PROBE_ENV"
echo "  Out: $PROBLEM_DIR"
echo "  Start: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
echo

T0=$(date +%s)
env $PROBE_ENV \
  TURINGOS_USER_TASK_MODE=1 \
  TURINGOS_CHAINTAPE_PRESEED=1 \
  TURINGOS_USER_TASK_BOUNTY_MICRO=200000 \
  TURINGOS_CHAINTAPE_PATH="$PROBLEM_DIR/runtime_repo" \
  TURINGOS_CAS_PATH="$PROBLEM_DIR/cas" \
  TURINGOS_RUN_ID="tb16-x-2-2-$PID" \
  LLM_PROXY_URL="$LLM_PROXY_URL" \
  MAX_TRANSACTIONS="$MAX_TX" \
  CONDITION="n${N_SWARM}" \
  RUST_LOG="${RUST_LOG:-info}" \
  "$EVALUATOR_BIN" "$PFILE" 2> "$PROBLEM_DIR/evaluator.stderr" 1> "$PROBLEM_DIR/evaluator.stdout"
RC=$?
T1=$(date +%s)
ELAPSED=$((T1 - T0))
echo "  evaluator: rc=$RC  elapsed=${ELAPSED}s"

grep "^PPUT_RESULT:" "$PROBLEM_DIR/evaluator.stdout" | tail -1 > "$PROBLEM_DIR/pput_result.json"
if [[ -s "$PROBLEM_DIR/pput_result.json" ]]; then
  sed -i 's/^PPUT_RESULT://' "$PROBLEM_DIR/pput_result.json"
fi
grep -E "ChallengeResolve|tb16_emit_challenge_resolve|chaintape/tb16-arena" "$PROBLEM_DIR/evaluator.stderr" \
  > "$PROBLEM_DIR/challenge_resolve_trace.txt" 2>&1 || true

echo "  audit_tape..."
"$AUDIT_TAPE_BIN" \
  --runtime-repo "$PROBLEM_DIR/runtime_repo" \
  --cas-dir "$PROBLEM_DIR/cas" \
  --agent-pubkeys "$PROBLEM_DIR/runtime_repo/agent_pubkeys.json" \
  --pinned-pubkeys "$PROBLEM_DIR/runtime_repo/pinned_pubkeys.json" \
  --genesis genesis_payload.toml \
  --constitution constitution.md \
  --alignment-dir handover/alignment \
  --out "$PROBLEM_DIR/verdict.json" 2>&1 | tail -1

echo "  audit_tape replay..."
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
echo "Ship gate SG-16.x.2.2 — Challenge → ChallengeResolve parent-child:"
HAS_CHALL=0
HAS_RESOLVE=0
HAS_ID42_PASS=0
if grep -q '"challenge"' "$PROBLEM_DIR/verdict.json" 2>/dev/null \
  || grep -qi 'ChallengeTx\|challenge_tx' "$PROBLEM_DIR/dashboard.txt" 2>/dev/null; then
  HAS_CHALL=1
fi
if grep -q '"challenge_resolve"' "$PROBLEM_DIR/verdict.json" 2>/dev/null \
  || grep -qi 'ChallengeResolveTx\|challenge_resolve' "$PROBLEM_DIR/dashboard.txt" 2>/dev/null; then
  HAS_RESOLVE=1
fi
# id=42 audit assertion verifies parent-child relationship; passing ⇒ every
# ChallengeResolveTx references a prior ChallengeTx in the same chain.
if grep -q '"challenge_resolve_chain_to_challenge_tx".*"Pass"' "$PROBLEM_DIR/verdict.json" 2>/dev/null \
  || grep -q '"name":"challenge_resolve_chain_to_challenge_tx".*"result":"Pass"' "$PROBLEM_DIR/verdict.json" 2>/dev/null; then
  HAS_ID42_PASS=1
fi
if [[ "$HAS_CHALL" == "1" && "$HAS_RESOLVE" == "1" ]]; then
  echo "  ✓ Both ChallengeTx and ChallengeResolveTx detected"
  if [[ "$HAS_ID42_PASS" == "1" ]]; then
    echo "  ✓ id=42 audit assertion PASS — parent-child relationship verified"
  else
    echo "  ! id=42 audit assertion not detected as Pass — inspect verdict.json"
  fi
else
  echo "  ✗ Challenge=$HAS_CHALL  ChallengeResolve=$HAS_RESOLVE — gate FAILED"
fi
echo "════════════════════════════════════════════════════════════════════"
