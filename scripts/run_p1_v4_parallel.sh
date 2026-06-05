#!/usr/bin/env bash
# P1 v4 CORRECTED sweep — same per-cell logic as run_p1_realvalue.sh, but with a bounded
# concurrency pool (JOBS) so the multi-hundred-cell decisive run finishes in hours, not days.
# Cells are fully independent (separate repo/cas dirs, separate Lean temp files keyed by pid).
#
# Honest discipline (unchanged from run_p1_realvalue.sh):
#  - EQUAL BUDGET: every arm gets NA*NR proposals (single auto-compensated in the bin).
#  - REPLAY-RECOMPUTE: every counted cell must pass verify_chaintape (economic_state from L4) ELSE excluded.
#  - 断点续做: a cell with a manifest + a replay-clean report is skipped (resumable).
#  - CAS preserved (KEEP_CAS=1 default) so cracked proofs can be extracted + axiom-shown post-hoc.
set -uo pipefail
cd "$(dirname "$0")/.."

BIN=./target/debug/lean_market_agent
VERIFY=./target/debug/verify_chaintape
OUT="${OUT:-handover/evidence/p1_v4_2026-06-02}"
THEOREMS="${THEOREMS:?set THEOREMS}"
ARMS="${ARMS:?set ARMS}"
SEEDS="${SEEDS:-1 2 3}"
NA="${NA:-4}"; NR="${NR:-6}"
MODEL="${MODEL:-deepseek-v4-pro}"
BANK="${BANK:-tests/fixtures/lean_theorems_pool.jsonl}"
MATHLIB="${MATHLIB:-/Users/zephryj/work/mathlib4}"
PROXY="${PROXY:-http://localhost:8123}"
JOBS="${JOBS:-6}"
KEEP_CAS="${KEEP_CAS:-1}"

[ -x "$BIN" ] || { echo "FATAL: $BIN not built"; exit 2; }
[ -x "$VERIFY" ] || { echo "FATAL: $VERIFY not built"; exit 2; }
mkdir -p "$OUT"

run_cell() {
  local thm=$1 arm=$2 s=$3
  local cell="$OUT/${thm}__${arm}__s${s}"
  if [ -f "$cell.json" ] && [ -f "$cell.replay.json" ] \
     && grep -q '"economic_state_reconstructed": true' "$cell.replay.json"; then
    echo "[skip] $thm/$arm/s$s"; return 0
  fi
  local rid="p1v4_${thm}_${arm}_s${s}"
  local repo="$OUT/repo_${rid}" cas="$OUT/cas_${rid}"
  rm -rf "$repo" "$cas"; mkdir -p "$repo" "$cas"
  echo "[run ] $thm/$arm/s$s @ $(date +%H:%M:%S)"
  "$BIN" --runtime-repo "$repo" --cas "$cas" --run-id "$rid" \
    --problem "$thm" --policy "$arm" --n-agents "$NA" --n-rounds "$NR" --seed "$s" \
    --model "$MODEL" --bank "$BANK" --mathlib-dir "$MATHLIB" --proxy-url "$PROXY" \
    --out "$cell.json" >> "$OUT/run_${rid}.log" 2>&1
  if [ -d "$repo" ]; then
    "$VERIFY" --repo "$repo" --cas "$cas" --run-id "$rid" --out "$cell.replay.json" >> "$OUT/run_${rid}.log" 2>&1
  fi
  local v o rc
  v=$(python3 -c "import json;print(json.load(open('$cell.json')).get('verified_count','?'))" 2>/dev/null || echo '?')
  o=$(python3 -c "import json;print(json.load(open('$cell.json')).get('omega_reached','?'))" 2>/dev/null || echo '?')
  rc=$(grep -q '"economic_state_reconstructed": true' "$cell.replay.json" 2>/dev/null && echo OK || echo FAIL)
  echo "[done] $thm/$arm/s$s verified=$v omega=$o replay=$rc"
  # keep CAS for crack extraction; on KEEP_CAS=0 reclaim only replay-clean cells
  [ "$KEEP_CAS" = 1 ] || { [ "$rc" = OK ] && rm -rf "$repo" "$cas"; }
}
export -f run_cell
export BIN VERIFY OUT NA NR MODEL BANK MATHLIB PROXY KEEP_CAS

echo "=== P1 v4 parallel sweep -> $OUT (JOBS=$JOBS model=$MODEL budget=$((NA*NR))/cell) ==="
echo "    theorems=[$THEOREMS] arms=[$ARMS] seeds=[$SEEDS]"
total=0
for thm in $THEOREMS; do for arm in $ARMS; do for s in $SEEDS; do
  run_cell "$thm" "$arm" "$s" &
  total=$((total+1))
  while [ "$(jobs -r | wc -l | tr -d ' ')" -ge "$JOBS" ]; do sleep 2; done
done; done; done
wait
echo "=== P1 v4 sweep complete ($total cells dispatched) -> $OUT ==="
