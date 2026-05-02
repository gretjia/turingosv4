#!/usr/bin/env bash
# TB-8 ChainTape smoke runner — variety problems, real LLM, real Lean.
#
# Output dir: handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/
#
# Each run:
#   - mkdir <problem_label>/{runtime_repo,cas}
#   - run evaluator with TURINGOS_CHAINTAPE_PATH + TURINGOS_CAS_PATH +
#     TURINGOS_CHAINTAPE_PRESEED=1 + TURINGOS_RUN_ID=tb8-<label>
#   - run audit_dashboard → dashboard.txt
#   - run verify_chaintape → replay_report.json
#   - tar.gz runtime_repo/.git → runtime_repo.dotgit.tar.gz
#   - tar.gz cas/.git → cas.dotgit.tar.gz
#   - clean loose .git directories (so committed evidence is the tar.gz only)
#
# Sequencer/proxy:
#   - LLM proxy at localhost:8080
#   - Lean at /home/zephryj/.elan/bin/lean
#   - Mathlib at turingosv3/experiments/minif2f_data_lean4/.lake/build
#
# Variety: 7 distinct mathd_* / aime_* problems span the heldout-49.
#
set -uo pipefail

ROOT="/home/zephryj/projects/turingosv4"
EVIDENCE="$ROOT/handover/evidence/tb_8_minimal_payout_smoke_2026-05-02"
EVAL="$ROOT/target/debug/evaluator"
DASH="$ROOT/target/debug/audit_dashboard"
VERIFY="$ROOT/target/debug/verify_chaintape"

mkdir -p "$EVIDENCE"

# Source project .env for DEEPSEEK_API_KEY (proxy may need it indirectly).
if [ -f "$ROOT/.env" ]; then
    set -a; . "$ROOT/.env"; set +a
fi
# Fallback: turingosv3/.env carries DEEPSEEK_API_KEY per memory.
if [ -z "${DEEPSEEK_API_KEY:-}" ] && [ -f /home/zephryj/projects/turingosv3/.env ]; then
    set -a; . /home/zephryj/projects/turingosv3/.env; set +a
fi

run_one() {
    local label="$1"; shift
    local condition="$1"; shift
    local max_tx="$1"; shift
    local problem="$1"; shift

    local rundir="$EVIDENCE/$label"
    rm -rf "$rundir"
    mkdir -p "$rundir/runtime_repo" "$rundir/cas"

    echo "========================================================="
    echo " TB-8 smoke: $label  (CONDITION=$condition  MAX_TX=$max_tx  problem=$problem)"
    echo "========================================================="

    TURINGOS_CHAINTAPE_PATH="$rundir/runtime_repo" \
    TURINGOS_CAS_PATH="$rundir/cas" \
    TURINGOS_CHAINTAPE_PRESEED=1 \
    TURINGOS_RUN_ID="tb8-$label" \
    LLM_PROXY_URL="http://localhost:8080/v1/chat/completions" \
    ACTIVE_MODEL="deepseek-chat" \
    CONDITION="$condition" \
    MAX_TRANSACTIONS="$max_tx" \
    "$EVAL" "$problem" > "$rundir/evaluator.log" 2>&1
    local rc=$?
    echo "  evaluator rc=$rc; tail of log:"
    tail -10 "$rundir/evaluator.log" | sed 's/^/    /'

    "$DASH" --repo "$rundir/runtime_repo" --cas "$rundir/cas" > "$rundir/dashboard.txt" 2>&1 || true
    "$VERIFY" --repo "$rundir/runtime_repo" --cas "$rundir/cas" --out "$rundir/replay_report.json" 2>&1 | tee "$rundir/verify.log" | tail -3
    [ -f "$rundir/replay_report.json" ] || echo "  WARN: replay_report.json missing"

    # TB-8 round-2 (Codex VETO RQ3 fix): self-contained tar.gz of full
    # runtime_repo + cas directories, NOT just .git/. The .git-only tar
    # missed required verifier sidecars (pinned_pubkeys.json,
    # agent_pubkeys.json, initial_q_state.json, rejections.jsonl,
    # genesis_report.json) that verify_chaintape needs at boot.
    # Excludes target/ + working-tree files of the inner git repo to
    # keep size bounded; the .git dir IS included so checkout works.
    if [ -d "$rundir/runtime_repo" ]; then
        ( cd "$rundir" && tar czf runtime_repo.tar.gz \
            --exclude='runtime_repo/.git/objects/pack/*.pack.tmp' \
            runtime_repo )
    fi
    if [ -d "$rundir/cas" ]; then
        ( cd "$rundir" && tar czf cas.tar.gz cas )
    fi

    # Quick TB-8 indicator: did a FinalizeRewardTx land?
    if grep -q "FinalizeReward" "$rundir/dashboard.txt" 2>/dev/null; then
        echo "  TB-8 indicator: FinalizeReward observed in dashboard ✓"
    else
        echo "  TB-8 indicator: NO FinalizeReward in dashboard (UNSOLVED OR Atom-4 didn't fire)"
    fi
    if grep -q "Finalized" "$rundir/dashboard.txt" 2>/dev/null; then
        echo "  TB-8 §9 Claims: ≥1 Finalized claim row ✓"
    fi
}

# 7 runs with variety (5+ distinct problems).
run_one "single_n1_mathd_algebra_171" n1 10 mathd_algebra_171.lean
run_one "half_n1_mathd_algebra_107"   n1 20 mathd_algebra_107.lean
run_one "half_n1_mathd_algebra_359"   n1 20 mathd_algebra_359.lean
run_one "half_n1_mathd_algebra_10"    n1 20 mathd_algebra_10.lean
run_one "full_n1_mathd_algebra_11"    n1 20 mathd_algebra_11.lean
run_one "full_n1_mathd_numbertheory_961" n1 20 mathd_numbertheory_961.lean
run_one "full_n1_aime_1997_p9"        n1 20 aime_1997_p9.lean

echo
echo "========================================================="
echo " TB-8 smoke COMPLETE."
echo "========================================================="
ls -la "$EVIDENCE"
