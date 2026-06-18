#!/usr/bin/env bash
# H-HET-2 Step-6 deep-theorem calibration sweep (gate #4).
#
# Per prereg §3/§5: measure per-(model, theorem) coverage for HOMOGENEOUS single-model
# arms at the experiment budget (tx/agent ≳ 20), to classify Goldilocks targets:
#   no single model one-shots ∧ models differ (some 0/K, another ≥1/K).
# Each cell = one carrier run, policy verify_ucb_price_floor with a SINGLE-model roster
# (routing degenerates to that model = the full budget to M, the BestHOMO-candidate
# measurement). Fast path (persistent Lean verify-service) so Lean is ~free; LLM
# proposals dominate. RESUMABLE: a cell with a recorded verdict is skipped (per-cell
# checkpoint — the long-run discipline). 1 calibration seed (classify; confirmatory is K≥12).
#
# Usage (env-overridable):
#   TURINGOS_LEAN_VERIFY_PYTHON=/path/venv/bin/python \
#   THEOREMS="lm_lim1 lm_deriv1" MODELS="DSHOMO Q397HOMO" NA=4 NR=20 SEEDS=42 JOBS=2 \
#   bash scripts/calibration_sweep.sh
set -uo pipefail
cd "$(dirname "$0")/.."

export TURINGOS_LEAN_VERIFY_PYTHON="${TURINGOS_LEAN_VERIFY_PYTHON:-/tmp/leanvenv/bin/python}"
BIN="${BIN:-./target/release/lean_market_agent}"
OUT="${OUT:-handover/evidence/h2_calibration_2026-06-17}"
BANK="${BANK:-tests/fixtures/lean_theorems_pool.jsonl}"
MATHLIB="${MATHLIB:-/Users/zephryj/work/mathlib4}"
PROXY="${PROXY:-http://localhost:8123}"
NA="${NA:-4}"; NR="${NR:-20}"
SEEDS="${SEEDS:-42}"
# Default = a few clearly-non-det (analysis / number-theory / inequality) PILOT targets;
# the FROZEN full non-det list is gate-#4 output (architect sign-off).
THEOREMS="${THEOREMS:-lm_lim1 lm_deriv1 lm_nt_cop_cubic lm_ineq2}"
MODELS="${MODELS:-DSHOMO Q32HOMO GLMHOMO Q397HOMO}"
JOBS="${JOBS:-2}"   # concurrent carriers; each holds its own warm Mathlib REPL (RAM-bound)

roster_for() {
  case "$1" in
    DSHOMO)   echo "deepseek-ai/DeepSeek-V4-Pro" ;;
    Q32HOMO)  echo "Qwen/Qwen3-32B" ;;
    GLMHOMO)  echo "zai-org/GLM-4.5-Air" ;;
    Q397HOMO) echo "Qwen/Qwen3.5-397B-A17B" ;;
    *) echo "UNKNOWN_ARM_$1"; return 1 ;;
  esac
}

[ -x "$BIN" ] || { echo "FATAL: $BIN not built"; exit 2; }
mkdir -p "$OUT"

run_cell() {
  local thm=$1 arm=$2 s=$3
  local cell="$OUT/${thm}__${arm}__s${s}"
  if [ -f "$cell.json" ] && grep -q '"omega_reached"' "$cell.json" 2>/dev/null; then
    echo "[skip] $thm/$arm/s$s"; return 0
  fi
  local models; models=$(roster_for "$arm") || { echo "[err ] bad arm $arm"; return 1; }
  local rid="cal_${thm}_${arm}_s${s}"
  local repo="$OUT/repo_${rid}" cas="$OUT/cas_${rid}"
  rm -rf "$repo" "$cas"; mkdir -p "$repo" "$cas"
  echo "[run ] $thm/$arm/s$s @ $(date +%H:%M:%S)"
  "$BIN" --runtime-repo "$repo" --cas "$cas" --run-id "$rid" \
    --problem "$thm" --policy verify_ucb_price_floor --models "$models" \
    --n-agents "$NA" --n-rounds "$NR" --seed "$s" \
    --bank "$BANK" --mathlib-dir "$MATHLIB" --lean-verify-service true \
    --proxy-url "$PROXY" --out "$cell.json" >> "$OUT/run_${rid}.log" 2>&1
  local o v tok
  o=$(python3 -c "import json;print(json.load(open('$cell.json')).get('omega_reached','?'))" 2>/dev/null || echo '?')
  v=$(python3 -c "import json;print(json.load(open('$cell.json')).get('verified_count','?'))" 2>/dev/null || echo '?')
  tok=$(python3 -c "import json;m=json.load(open('$cell.json'));print(m.get('total_tokens') or m.get('total_model_tokens') or '?')" 2>/dev/null || echo '?')
  echo "[done] $thm/$arm/s$s omega=$o verified=$v tokens=$tok"
}
export -f run_cell roster_for
export BIN OUT NA NR BANK MATHLIB PROXY TURINGOS_LEAN_VERIFY_PYTHON

echo "=== H-HET-2 calibration sweep -> $OUT (JOBS=$JOBS budget=NA${NA}xNR${NR}) ==="
echo "    theorems=[$THEOREMS] models=[$MODELS] seeds=[$SEEDS]"
total=0
for thm in $THEOREMS; do for arm in $MODELS; do for s in $SEEDS; do
  run_cell "$thm" "$arm" "$s" &
  total=$((total+1))
  while [ "$(jobs -r | wc -l | tr -d ' ')" -ge "$JOBS" ]; do sleep 2; done
done; done; done
wait
echo "=== sweep complete ($total cells) -> $OUT ==="
# Goldilocks classification: per theorem, omega across the model arms.
python3 - "$OUT" $THEOREMS <<'PY'
import json, sys, glob, os
out = sys.argv[1]; thms = sys.argv[2:]
print("\n=== per-(theorem,model) coverage (omega) ===")
goldilocks = []
for thm in thms:
    row = {}
    for f in sorted(glob.glob(f"{out}/{thm}__*__s*.json")):
        base = os.path.basename(f)[:-5]
        arm = base.split("__")[1]
        try: row[arm] = json.load(open(f)).get("omega_reached")
        except Exception: row[arm] = "?"
    solved = [a for a,o in row.items() if o is True]
    failed = [a for a,o in row.items() if o is False]
    tag = "GOLDILOCKS" if (solved and failed) else ("all-solve" if solved and not failed else ("all-fail" if failed and not solved else "?"))
    if solved and failed: goldilocks.append(thm)
    print(f"  {thm:24s} solved={solved} failed={failed}  -> {tag}")
print(f"\nGOLDILOCKS targets (no-single-model-one-shot ∧ models-differ): {goldilocks}")
PY
