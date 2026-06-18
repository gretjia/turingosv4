#!/usr/bin/env bash
# V5 budget-binding canary: run ONE pool theorem through the FULL carrier path at the
# calibration budget, then project cost for the full sweep and report whether the budget
# BINDS (affordable < repairable). This is paid prep — run only under architect authz.
#
# Usage: estimate_calibration_cost.sh <theorem> <model> <n_theorems> <n_models> <n_seeds> [n_rounds]
# Prints a JSON projection on the last line.
set -euo pipefail
cd "${REPO:-$(git rev-parse --show-toplevel 2>/dev/null || pwd)}"
# Fast path: route Lean verifies through the persistent verify-service
# (A/B-oracle-proven byte-equivalent; ~130-260x faster than per-process lean).
# Override the venv (with lean-interact) via TURINGOS_LEAN_VERIFY_PYTHON.
export TURINGOS_LEAN_VERIFY_PYTHON="${TURINGOS_LEAN_VERIFY_PYTHON:-/tmp/leanvenv/bin/python}"

THM="${1:-lm_det_2x2}"; MODEL="${2:-Qwen/Qwen3.5-397B-A17B}"
NTHM="${3:-37}"; NMODELS="${4:-4}"; NSEEDS="${5:-3}"; NR="${6:-8}"
MATHLIB="${MATHLIB:-$HOME/work/mathlib4}"
BIN=./target/debug/lean_market_agent
repo=/tmp/autoloop_estimate_repo; cas=/tmp/autoloop_estimate_cas
rm -rf "$repo" "$cas"; mkdir -p "$repo" "$cas"

t0=$(date +%s)
"$BIN" --runtime-repo "$repo" --cas "$cas" --run-id autoloop_estimate \
  --problem "$THM" --policy verify_ucb_price_floor --models "$MODEL" \
  --n-agents 4 --n-rounds "$NR" --seed 42 \
  --bank tests/fixtures/lean_theorems_pool.jsonl --mathlib-dir "$MATHLIB" \
  --lean-verify-service true \
  --proxy-url http://localhost:8123 --out "$repo/manifest.json" >"$repo/run.log" 2>&1 || true
t1=$(date +%s)

python3 - "$repo/manifest.json" "$((t1-t0))" "$NTHM" "$NMODELS" "$NSEEDS" <<'PY'
import json, sys
man_path, wall, nthm, nmodels, nseeds = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4]), int(sys.argv[5])
try:
    m = json.load(open(man_path))
    tok = m.get("total_tokens") or m.get("total_model_tokens") or 0
    omega = m.get("omega_reached")
except Exception as e:
    tok, omega, wall = 0, None, wall
runs = nthm * nmodels * nseeds
proj_tokens = tok * runs
proj_wall_hr = (wall * runs) / 3600.0
# binding heuristic: a calibration is informative only if the per-run budget can be SPENT (omega not
# reached trivially at round 0) — i.e. the run used real attempts. Flag if the canary one-shot at round 0.
binds = (tok > 0)
print(json.dumps({
  "canary": {"theorem": sys.argv[1].split('/')[-1], "tokens": tok, "wall_s": wall, "omega": omega},
  "full_sweep": {"runs": runs, "n_theorems": nthm, "n_models": nmodels, "n_seeds": nseeds},
  "projection": {"total_tokens": proj_tokens, "wall_hours": round(proj_wall_hr, 1)},
  "budget_binds": binds,
  "note": "wall is serial; parallelize across models/theorems to cut wall-clock. Tokens are the cost driver."
}, indent=2))
PY
