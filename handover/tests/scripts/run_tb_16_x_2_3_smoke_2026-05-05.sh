#!/usr/bin/env bash
# TB-16.x.2.3 — CompleteSetRedeem env-var trigger smoke.
#
# Per umbrella charter `handover/tracer_bullets/TB-16.x.2_charter_2026-05-04.md`
# §2 Atom 2.3: single-problem arena exercising
# TURINGOS_COMPLETE_SET_SEED + TURINGOS_FORCE_BANKRUPTCY +
# TURINGOS_FORCE_REDEEM on a task expected to MaxTxExhaust.
#
# Why MaxTxExhaust + FORCE_BANKRUPTCY:
#   - CompleteSetRedeem requires the event to be Finalized (Yes wins) or
#     Bankrupt (No wins). The MaxTxExhausted + FORCE_BANKRUPTCY path
#     transitions task_markets_t[task_id].state to Bankrupt → NO wins
#     (sequencer.rs:1357). The OMEGA-Confirm path requires FinalizeReward
#     to drive the market to Finalized, which is not currently wired in
#     the arena driver; charter §2 Atom 2.3 default scenario uses the
#     Bankruptcy path.
#   - Provider mints YES + NO shares via TURINGOS_COMPLETE_SET_SEED at
#     evaluator setup (line ~982). After Bankrupt, provider redeems NO
#     shares 1:1 against the event collateral pool.
#
# Ship gate SG-16.x.2.3: chain contains CompleteSetRedeemTx with
# non-zero share_amount (raises 11-of-13 → 12-of-13 system-emitted tx
# kinds runtime-exercised). Pre-existing audit assertion id=22
# (conditional_shares_excluded_from_supply, Layer D) covers; no new
# assertion needed.
#
# Ship gate uses python3 JSON count guard (NOT grep on field name).
# Per .2.2 Patch B + this session's hygiene #15 forensic finding on .2.1:
# `grep '"complete_set_redeem"'` matches the literal field name in
# tx_kind_counts regardless of count value (always-pass false positive).
#
# Markov capsule = None per FC2 Boot + Markov chain genesis semantic
# (fresh runtime_repo + fresh cas; no prior chain). Per TB-16.x.fix
# (architect OBS_R022 Option α RATIFIED 2026-05-04), `--markov-pointer`
# is optional and absence ≡ genesis chain.
#
# Usage:
#   bash handover/tests/scripts/run_tb_16_x_2_3_smoke_2026-05-05.sh

set -uo pipefail
cd /home/zephryj/projects/turingosv4

OUT_BASE="${OUT_BASE:-handover/evidence/tb_16_x_2_3_smoke_2026-05-05}"
mkdir -p "$OUT_BASE"

EVALUATOR_BIN="./target/release/evaluator"
AUDIT_TAPE_BIN="./target/release/audit_tape"
AUDIT_TAPE_TAMPER_BIN="./target/release/audit_tape_tamper"
AUDIT_DASHBOARD_BIN="./target/release/audit_dashboard"

LLM_PROXY_URL="${LLM_PROXY_URL:-http://localhost:18080}"
N_SWARM="${N_SWARM:-5}"
MAX_TX="${MAX_TX:-20}"

PID="P11_complete_set_redeem"
PFILE="aime_1997_p9.lean"
# Provider = Agent_user_0 (sandbox-prefixed per CR-16.5; matches
# default_pput_preseed_pairs identifier set). Mint amount 1_000_000 μC
# gives provider 1M YES + 1M NO shares; Redeem 250_000 NO shares
# (1/4 of mint) leaves headroom and exercises a non-trivial transfer.
COMPLETE_SET_PROVIDER="${COMPLETE_SET_PROVIDER:-Agent_user_0}"
COMPLETE_SET_AMOUNT="${COMPLETE_SET_AMOUNT:-1000000}"
REDEEM_OUTCOME="${REDEEM_OUTCOME:-no}"
REDEEM_UNITS="${REDEEM_UNITS:-250000}"

PROBE_ENV="TURINGOS_COMPLETE_SET_SEED=${COMPLETE_SET_PROVIDER}:${COMPLETE_SET_AMOUNT}"
PROBE_ENV="$PROBE_ENV TURINGOS_FORCE_BANKRUPTCY=1"
PROBE_ENV="$PROBE_ENV TURINGOS_FORCE_REDEEM=${COMPLETE_SET_PROVIDER}:${REDEEM_OUTCOME}:${REDEEM_UNITS}"

PROBLEM_DIR="$OUT_BASE/$PID"
mkdir -p "$PROBLEM_DIR/runtime_repo" "$PROBLEM_DIR/cas"

echo "════════════════════════════════════════════════════════════════════"
echo "TB-16.x.2.3 — CompleteSetRedeem smoke ($PID)"
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
  TURINGOS_RUN_ID="tb16-x-2-3-$PID" \
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
grep -E "CompleteSetRedeem|CompleteSetMint|MarketSeed|TaskBankruptcy|chaintape/tb16-arena" \
  "$PROBLEM_DIR/evaluator.stderr" \
  > "$PROBLEM_DIR/redeem_trace.txt" 2>&1 || true

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
echo "Ship gate SG-16.x.2.3 — CompleteSetRedeem tx kind in chain (non-zero share_amount):"
# Python JSON count guard on verdict.json (NOT grep on field name) per
# .2.2 Patch B and hygiene #15 forensic finding (.2.1 used field-name
# grep that always passed regardless of count value).
#
# Positivity witness: stderr trace `[chaintape/tb16-arena]
# CompleteSetRedeemTx submitted by <owner> (units=N, outcome=...)`
# emitted by the FORCE_REDEEM hook BEFORE submit. Because the hook
# rejects share_units==0 with a `warn! ... skip` and never builds the
# tx, presence of the submit-trace line implies the request had units>0
# AND the sequencer admitted it (otherwise verdict.json's
# complete_set_redeem count would be 0). Two-witness gate:
#   (a) verdict.json tx_kind_counts.complete_set_redeem >= 1   AND
#   (b) stderr submit-trace shows units=<positive integer>.
GATE_RESULT=$(python3 - "$PROBLEM_DIR/verdict.json" "$PROBLEM_DIR/redeem_trace.txt" <<'PY'
import json, re, sys
verdict_path = sys.argv[1]
trace_path = sys.argv[2]
v = json.load(open(verdict_path))
counts = v.get("tx_kind_counts", {})
mint_n = int(counts.get("complete_set_mint", 0))
seed_n = int(counts.get("market_seed", 0))
bk_n = int(counts.get("task_bankruptcy", 0))
redeem_n = int(counts.get("complete_set_redeem", 0))
positive_share = 0
try:
    trace = open(trace_path).read()
except Exception:
    trace = ""
# Match the submit-trace emitted at evaluator.rs FORCE_REDEEM hook:
# "CompleteSetRedeemTx submitted by <owner> (units=N, outcome=...)"
# Take the FIRST submit (multiple is fine; gate only needs ≥1 positive).
m = re.search(r'CompleteSetRedeemTx submitted by \S+ \(units=(\d+)', trace)
if m and int(m.group(1)) > 0:
    positive_share = 1
print(f"{seed_n}|{mint_n}|{bk_n}|{redeem_n}|{positive_share}")
PY
)
SEED_N="${GATE_RESULT%%|*}"
REST="${GATE_RESULT#*|}"
MINT_N="${REST%%|*}"
REST="${REST#*|}"
BK_N="${REST%%|*}"
REST="${REST#*|}"
REDEEM_N="${REST%%|*}"
POSITIVE_SHARE="${REST#*|}"

SHIP_GATE_RC=0
echo "  Chain counts: MarketSeed=$SEED_N  CompleteSetMint=$MINT_N  TaskBankruptcy=$BK_N  CompleteSetRedeem=$REDEEM_N  positive_share=$POSITIVE_SHARE"
if [[ "$REDEEM_N" -gt 0 && "$POSITIVE_SHARE" == "1" ]]; then
  echo "  ✓ Chain contains CompleteSetRedeemTx (n=$REDEEM_N) with non-zero share_amount (witness: stderr submit-trace units>0)"
else
  echo "  ✗ CompleteSetRedeemTx missing or units=0 (count=$REDEEM_N positive=$POSITIVE_SHARE) — gate FAILED"
  SHIP_GATE_RC=1
fi
echo "════════════════════════════════════════════════════════════════════"
# Fail-closed exit per .2.2.fix.r2 Patch F1+F2: propagate ship-gate
# verdict via process exit so a CI runner observes the failure honestly.
exit $SHIP_GATE_RC
