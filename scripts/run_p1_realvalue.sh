#!/usr/bin/env bash
# P1 REAL-VALUE experiment — non-local price-routed tree search vs honest baselines, on the REAL
# ChainTape substrate (lean_market_agent). The first experiment that tests BOTH constitutional
# dimensions at once: {loss-bearing YES/NO price} x {non-local tree search: agents read the full-chain
# price landscape and restart from any earlier node}. Substrate verified 2026-06-01 (13/16 non-local
# restarts, distinct prices, verify_chaintape replay-clean).
#
# Honest discipline (the forensic lessons, enforced here):
#  - EQUAL BUDGET: every arm gets n_agents*n_rounds proposals (single is auto-compensated in the bin).
#  - FAIR BASELINES: single (1 chain), shuffled_price (price destroyed), no_price (random node) —
#    market must beat ALL THREE, else price+non-locality are not load-bearing.
#  - REPLAY-RECOMPUTE: every counted cell must pass verify_chaintape (economic_state RECONSTRUCTED
#    from L4, not byte-only) ELSE excluded.
#  - 断点续做: a cell with a manifest + a replay-clean report is skipped.
set -uo pipefail
cd "$(dirname "$0")/.."

BIN=./target/debug/lean_market_agent
VERIFY=./target/debug/verify_chaintape
OUT="${OUT:-handover/evidence/p1_realvalue_2026-06-01}"
THEOREMS="${THEOREMS:-lm_commute_pow lm_sum_cubes lm_ineq2}"
ARMS="${ARMS:-market single shuffled_price no_price}"
SEEDS="${SEEDS:-1 2 3 4 5 6}"
NA="${NA:-4}"; NR="${NR:-6}"
MODEL="${MODEL:-deepseek-v4-pro}"
BANK="${BANK:-tests/fixtures/lean_theorems_pool.jsonl}"
MATHLIB="${MATHLIB:-/Users/zephryj/work/mathlib4}"
PROXY="${PROXY:-http://localhost:8123}"

[ -x "$BIN" ] || { echo "FATAL: $BIN not built"; exit 2; }
[ -x "$VERIFY" ] || { echo "FATAL: $VERIFY not built"; exit 2; }
mkdir -p "$OUT"

cell_done() {  # 0 iff manifest + replay-clean
  local c=$1
  [ -f "$c.json" ] || return 1
  [ -f "$c.replay.json" ] || return 1
  grep -q '"economic_state_reconstructed": true' "$c.replay.json" || return 1
  return 0
}

echo "=== P1 real-value sweep -> $OUT (model=$MODEL NA=$NA NR=$NR budget=$((NA*NR))/cell) ==="
echo "    theorems=[$THEOREMS] arms=[$ARMS] seeds=[$SEEDS]"
for thm in $THEOREMS; do for arm in $ARMS; do for s in $SEEDS; do
  cell="$OUT/${thm}__${arm}__s${s}"
  if cell_done "$cell"; then echo "[skip] $thm/$arm/s$s"; continue; fi
  rid="p1_${thm}_${arm}_s${s}"
  repo="$OUT/repo_${rid}"; cas="$OUT/cas_${rid}"
  rm -rf "$repo" "$cas"; mkdir -p "$repo" "$cas"
  echo "[run ] $thm/$arm/s$s @ $(date +%H:%M:%S)"
  "$BIN" --runtime-repo "$repo" --cas "$cas" --run-id "$rid" \
    --problem "$thm" --policy "$arm" --n-agents "$NA" --n-rounds "$NR" --seed "$s" \
    --model "$MODEL" --bank "$BANK" --mathlib-dir "$MATHLIB" --proxy-url "$PROXY" \
    --out "$cell.json" >> "$OUT/run.log" 2>&1
  if [ -d "$repo" ]; then
    "$VERIFY" --repo "$repo" --cas "$cas" --run-id "$rid" --out "$cell.replay.json" >> "$OUT/run.log" 2>&1
  fi
  v=$(python3 -c "import json;print(json.load(open('$cell.json')).get('verified_count','?'))" 2>/dev/null || echo '?')
  o=$(python3 -c "import json;print(json.load(open('$cell.json')).get('omega_reached','?'))" 2>/dev/null || echo '?')
  rc=$(grep -q '"economic_state_reconstructed": true' "$cell.replay.json" 2>/dev/null && echo OK || echo FAIL)
  echo "[done] $thm/$arm/s$s verified=$v omega=$o replay=$rc"
  # keep the ChainTape repo only if replay failed (for debugging); else reclaim disk
  [ "$rc" = OK ] && rm -rf "$repo" "$cas"
done; done; done
echo "=== P1 sweep complete -> $OUT ==="
