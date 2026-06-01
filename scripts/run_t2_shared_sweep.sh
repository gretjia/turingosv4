#!/usr/bin/env bash
# T2 SHARED-STATE counted sweep — the confound-free price-routing experiment (2026-06-01).
#
# WHY shared-state: the per-arm run_alloc re-ran the STOCHASTIC free-bank for every arm, so each arm faced a
# DIFFERENT residual set (free-bank luck ±7) that swamped the thin routing signal (~3 repairable). Here the
# free-bank + betting + per-residual REPAIR are computed ONCE per seed; the 6 arms are deterministic allocation
# policies over the IDENTICAL (residuals, prices, repair-outcomes) state, so banked@B differs ONLY by routing
# ORDER. One binary invocation per seed emits all 6 arm manifests+tapes (t2s_<arm>_<seed>.{json,tape}).
#
# 断点续做 (resumable): a seed whose 6 arms each have a manifest + a replay-GREEN rr report is SKIPPED. Kill and
# rerun any time; only missing/replay-failed seeds recompute. Every headline cell gates on verify_market_tape.
set -uo pipefail
cd "$(dirname "$0")/.."

BIN=./target/debug/lean_hayek_market
VERIFY=./target/debug/verify_market_tape
OUT="${OUT:-handover/evidence/t2_shared_sweep_2026-06-01}"
SEEDS="${SEEDS:-8 9 10 11 12 13 14 15 16 17 18 19}"  # 12 fresh post-lock seeds; seed 7 is the pilot/smoke (not counted)
ARMS="market shuffled flatbid coordinator random index"
SUBSET="${SUBSET:-24}"
B="${B:-1000}"
MODEL="${MODEL:-deepseek-v4-flash}"
REASONER="${REASONER:-qwen3.7-max}"
MATHLIB="${MATHLIB:-/Users/zephryj/work/mathlib4}"
POOL="${POOL:-tests/fixtures/lean_theorems_pool.jsonl}"

[ -x "$BIN" ] || { echo "FATAL: $BIN not built (cargo build --bin lean_hayek_market)"; exit 2; }
[ -x "$VERIFY" ] || { echo "FATAL: $VERIFY not built (cargo build --bin verify_market_tape)"; exit 2; }
mkdir -p "$OUT"

seed_done() {  # 0 iff all 6 arms have a manifest + a replay-GREEN rr
  local s=$1 a
  for a in $ARMS; do
    [ -f "$OUT/t2s_${a}_${s}.json" ] || return 1
    [ -f "$OUT/t2s_rr_${a}_${s}.json" ] || return 1
    grep -q '"replay_clean": true' "$OUT/t2s_rr_${a}_${s}.json" || return 1
  done
  return 0
}

echo "=== T2 shared-state sweep → $OUT (B=$B subset=$SUBSET model=$MODEL reasoner=$REASONER) ==="
for s in $SEEDS; do
  if seed_done "$s"; then echo "[skip] seed $s complete + replay-green"; continue; fi
  echo "[run ] seed $s @ $(date +%H:%M:%S) ..."
  "$BIN" --task "shared:$POOL" --pool-subset "$SUBSET" --reasoner-budget-tok "$B" \
    --model "$MODEL" --reasoner-model "$REASONER" --seed "$s" \
    --mathlib-dir "$MATHLIB" --out "$OUT/run_${s}.json" >> "$OUT/sweep.log" 2>&1
  green=0
  for a in $ARMS; do
    [ -f "$OUT/t2s_${a}_${s}.tape" ] || { echo "  WARN: missing tape $a seed $s"; continue; }
    "$VERIFY" --tape "$OUT/t2s_${a}_${s}.tape" --manifest "$OUT/t2s_${a}_${s}.json" \
      --out "$OUT/t2s_rr_${a}_${s}.json" >> "$OUT/sweep.log" 2>&1
    grep -q '"replay_clean": true' "$OUT/t2s_rr_${a}_${s}.json" 2>/dev/null && green=$((green+1))
  done
  row=""
  for a in $ARMS; do
    b=$(python3 -c "import json;print(json.load(open('$OUT/t2s_${a}_${s}.json'))['banked_at_B'])" 2>/dev/null || echo '?')
    row="$row ${a}=${b}"
  done
  echo "[done] seed $s replay-green=${green}/6 banked@B:${row}"
done
echo "=== sweep complete → $OUT (aggregate: python3 scripts/analyze_t2_sweep.py --dir $OUT --prefix t2s --seeds $(echo $SEEDS|tr ' ' ',') --arms market,shuffled,flatbid,coordinator,random,index) ==="
