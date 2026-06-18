#!/usr/bin/env bash
# H-HET-1 carrier pilot (2026-06-15) — autonomous, prereg
# handover/preregistration/H_HET_1_CARRIER_PILOT_PREREG_2026-06-15.md.
# 3 arms × det band × K=3 seeds, --policy autonomous_market, equal budget NA*NR.
# Each cell: carrier run -> verify_chaintape replay-recompute gate. Resumable.
set -uo pipefail
cd "$(dirname "$0")/.."

BIN=./target/debug/lean_market_agent
VERIFY=./target/debug/verify_chaintape
OUT="${OUT:-handover/evidence/het_carrier_pilot_2026-06-15}"
BANK="${BANK:-tests/fixtures/lean_theorems_pool.jsonl}"
MATHLIB="${MATHLIB:-/Users/zephryj/work/mathlib4}"
PROXY="${PROXY:-http://localhost:8123}"
NA="${NA:-4}"; NR="${NR:-3}"
SEEDS="${SEEDS:-1 2 3}"
THEOREMS="${THEOREMS:-lm_det_mul lm_det_2x2 lm_det_zero lm_det_3x3 lm_geom_eval}"
ARMS="${ARMS:-HET DSHOMO Q397HOMO}"
JOBS="${JOBS:-4}"

# arm -> --models roster
roster_for() {
  case "$1" in
    HET)      echo "deepseek-ai/DeepSeek-V4-Pro,Qwen/Qwen3-32B,zai-org/GLM-4.5-Air,Qwen/Qwen3.5-397B-A17B" ;;
    DSHOMO)   echo "deepseek-ai/DeepSeek-V4-Pro" ;;
    Q397HOMO) echo "Qwen/Qwen3.5-397B-A17B" ;;
    *) echo "UNKNOWN_ARM_$1"; return 1 ;;
  esac
}

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
  local models; models=$(roster_for "$arm") || { echo "[err ] bad arm $arm"; return 1; }
  local rid="hetc_${thm}_${arm}_s${s}"
  local repo="$OUT/repo_${rid}" cas="$OUT/cas_${rid}"
  rm -rf "$repo" "$cas"; mkdir -p "$repo" "$cas"
  echo "[run ] $thm/$arm/s$s @ $(date +%H:%M:%S)"
  "$BIN" --runtime-repo "$repo" --cas "$cas" --run-id "$rid" \
    --problem "$thm" --policy autonomous_market --models "$models" \
    --n-agents "$NA" --n-rounds "$NR" --seed "$s" \
    --model deepseek-v4-pro --bank "$BANK" --mathlib-dir "$MATHLIB" --proxy-url "$PROXY" \
    --out "$cell.json" >> "$OUT/run_${rid}.log" 2>&1
  if [ -d "$repo" ]; then
    "$VERIFY" --repo "$repo" --cas "$cas" --run-id "$rid" --out "$cell.replay.json" >> "$OUT/run_${rid}.log" 2>&1
  fi
  local v o rc
  v=$(python3 -c "import json;print(json.load(open('$cell.json')).get('verified_count','?'))" 2>/dev/null || echo '?')
  o=$(python3 -c "import json;print(json.load(open('$cell.json')).get('omega_reached','?'))" 2>/dev/null || echo '?')
  rc=$(grep -q '"economic_state_reconstructed": true' "$cell.replay.json" 2>/dev/null && echo OK || echo FAIL)
  echo "[done] $thm/$arm/s$s verified=$v omega=$o replay=$rc"
}
export -f run_cell roster_for
export BIN VERIFY OUT NA NR BANK MATHLIB PROXY

echo "=== H-HET-1 carrier pilot -> $OUT (JOBS=$JOBS budget=$((NA*NR))/cell) ==="
echo "    theorems=[$THEOREMS] arms=[$ARMS] seeds=[$SEEDS]"
total=0
for thm in $THEOREMS; do for arm in $ARMS; do for s in $SEEDS; do
  run_cell "$thm" "$arm" "$s" &
  total=$((total+1))
  while [ "$(jobs -r | wc -l | tr -d ' ')" -ge "$JOBS" ]; do sleep 2; done
done; done; done
wait
echo "=== carrier pilot complete ($total cells) -> $OUT ==="
