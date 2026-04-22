#!/usr/bin/env bash
# Broad category probe — 18 problems covering all MiniF2F categories.
# Purpose: test whether n3 architecture value generalizes beyond easy mathd_algebra.

set -uo pipefail

if [ -z "${DEEPSEEK_API_KEY:-}" ] && [ -f "$HOME/projects/turingosv3/.env" ]; then
    source "$HOME/projects/turingosv3/.env"
fi
export LLM_PROXY_URL="${LLM_PROXY_URL:-http://localhost:8080}"
export ACTIVE_MODEL="${ACTIVE_MODEL:-deepseek-chat}"

CONDITION="${1:-oneshot}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
MINIF2F_DIR="${MINIF2F_DIR:-/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4}"
LOG_DIR="$SCRIPT_DIR/logs"
TIMESTAMP=$(date +%Y%m%dT%H%M%S)
RESULTS_FILE="$LOG_DIR/broad_${CONDITION}_${TIMESTAMP}.jsonl"
STDERR_LOG="$LOG_DIR/broad_${CONDITION}_${TIMESTAMP}.err"
mkdir -p "$LOG_DIR"

# Balanced sample: 3 per major category
PROBLEMS=(
    # mathd_algebra (known-easy for oneshot)
    mathd_algebra_141
    mathd_algebra_176
    mathd_algebra_400
    # mathd_numbertheory (0/60 in broken baseline — re-probe)
    mathd_numbertheory_12
    mathd_numbertheory_207
    mathd_numbertheory_239
    # aime
    aime_1983_p2
    aime_1984_p1
    aime_1990_p4
    # amc (0 in broken baseline)
    amc12_2000_p1
    amc12_2000_p12
    amc12a_2002_p6
    # induction (0 in broken baseline)
    induction_11div10tonmn1ton
    induction_12dvd4expnp1p20
    # imo (0 in broken baseline — hardest)
    imo_1959_p1
    imo_1960_p2
    # algebra (advanced)
    algebra_sqineq_at2malt1
    algebra_absxm1pabsxpabsxp1eqxp2_0leqxleq1
)

echo "Broad probe | Condition=$CONDITION | N=${#PROBLEMS[@]}"
echo "Results: $RESULTS_FILE"
echo "Stderr:  $STDERR_LOG"

(cd "$PROJECT_ROOT" && cargo build --release -p minif2f_v4 2>&1 | tail -1)
EVALUATOR="$PROJECT_ROOT/target/release/evaluator"

# Preflight (C-012)
LEAN_BIN="${LEAN_BINARY:-$HOME/.elan/toolchains/leanprover--lean4---v4.24.0/bin/lean}"
PFL=$(find "$MINIF2F_DIR/.lake/packages" \( -path "*/.lake/build/lib/lean" -o -path "*/lib/lean" \) -type d 2>/dev/null | tr '\n' ':')
if [ -z "$PFL" ]; then echo "PREFLIGHT FAIL: no Mathlib"; exit 2; fi
OUT=$(printf 'import Mathlib\nexample : (1:ℝ) + 1 = 2 := by norm_num\n' | LEAN_PATH="$PFL" timeout 180 "$LEAN_BIN" --stdin 2>&1)
if [ $? -ne 0 ] || echo "$OUT" | grep -q "error:"; then echo "PREFLIGHT FAIL: $OUT" | head -c 400; exit 2; fi
echo "Preflight OK."

SOLVED=0
for NAME in "${PROBLEMS[@]}"; do
    PROBLEM="$MINIF2F_DIR/MiniF2F/Test/${NAME}.lean"
    if [ ! -f "$PROBLEM" ]; then echo "[$NAME] missing file, skip"; continue; fi

    echo -n "[$NAME] ... "
    echo "=== $NAME @ $(date -Is) ===" >> "$STDERR_LOG"
    OUTPUT=$(timeout 900 env CONDITION="$CONDITION" MINIF2F_DIR="$MINIF2F_DIR" \
        EXPERIMENT_DIR="$SCRIPT_DIR" RUST_LOG=info \
        "$EVALUATOR" "$PROBLEM" 2>>"$STDERR_LOG") || true
    PPUT_JSON=$(echo "$OUTPUT" | grep "^PPUT_RESULT:" | sed 's/^PPUT_RESULT://' | head -1)
    if [ -n "$PPUT_JSON" ]; then
        echo "$PPUT_JSON" >> "$RESULTS_FILE"
        HAS_GP=$(echo "$PPUT_JSON" | python3 -c "import sys,json; print(json.load(sys.stdin)['has_golden_path'])" 2>/dev/null || echo False)
        if [ "$HAS_GP" = "True" ]; then
            SOLVED=$((SOLVED + 1))
            PPUT_VAL=$(echo "$PPUT_JSON" | python3 -c "import sys,json; print(f\"{json.load(sys.stdin)['pput']:.2f}\")")
            TIME_VAL=$(echo "$PPUT_JSON" | python3 -c "import sys,json; print(f\"{json.load(sys.stdin)['time_secs']:.0f}\")")
            echo "SOLVED (${TIME_VAL}s PPUT=${PPUT_VAL})"
        else
            TIME_VAL=$(echo "$PPUT_JSON" | python3 -c "import sys,json; print(f\"{json.load(sys.stdin)['time_secs']:.0f}\")")
            echo "FAIL (${TIME_VAL}s)"
        fi
    else
        echo "MEASUREMENT_ERROR/TIMEOUT"
    fi
done

echo ""
echo "Summary: $SOLVED / ${#PROBLEMS[@]} solved"
