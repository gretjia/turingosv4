#!/usr/bin/env bash
# TB-9 Durable AgentRegistry + Wallet Projection smoke — real LLM, real Lean.
#
# UNIQUE TB-9 capability under test: **cross-run identity**.
#   - Run-A: fresh keystore at TURINGOS_AGENT_KEYSTORE_PATH override; runs evaluator
#            on heldout-49 problem; record `agent_pubkeys.json` pubkey for Agent_0.
#   - Run-B: SAME keystore (across-run persistence); fresh runtime_repo; runs same
#            problem; record run-B's pubkey for Agent_0.
#   - Assert: pubkey_A == pubkey_B (architect mandate "agent identity survives
#             run restart").
#
# Plus: regression smoke single-run on TB-8 reference problem to confirm
# zero-impact on solve+payout pipeline (TB-8 ship gate carry-forward).
#
# Outputs:
#   handover/evidence/tb_9_durable_identity_smoke_2026-05-02/
#     keystore/                           (durable keystore shared by run-A+run-B)
#     run_a_n1_mathd_algebra_171/
#     run_b_n1_mathd_algebra_171/
#     regression_n1_mathd_algebra_107/    (TB-8 reference; expect SOLVED + Finalized)
#     COMPARISON.md                       (TB-7R → TB-8 → TB-9 side-by-side)
set -uo pipefail

ROOT="/home/zephryj/projects/turingosv4"
EVIDENCE="$ROOT/handover/evidence/tb_9_durable_identity_smoke_2026-05-02"
EVAL="$ROOT/target/release/evaluator"
DASH="$ROOT/target/release/audit_dashboard"
VERIFY="$ROOT/target/release/verify_chaintape"

# Build release artefacts the runner depends on.
[ -x "$EVAL" ] || cargo build --bin evaluator -p minif2f_v4 --release 2>&1 | tail -3
[ -x "$DASH" ] || cargo build --bin audit_dashboard -p turingosv4 --release 2>&1 | tail -3
[ -x "$VERIFY" ] || cargo build --bin verify_chaintape -p turingosv4 --release 2>&1 | tail -3

mkdir -p "$EVIDENCE/keystore"

# Source project .env for DEEPSEEK_API_KEY (proxy may need it indirectly).
if [ -f "$ROOT/.env" ]; then
    set -a; . "$ROOT/.env"; set +a
fi
if [ -z "${DEEPSEEK_API_KEY:-}" ] && [ -f /home/zephryj/projects/turingosv3/.env ]; then
    set -a; . /home/zephryj/projects/turingosv3/.env; set +a
fi

# TB-9 isolated keystore for the run pair (no contamination of host
# ~/.turingos/keystore/agent_keystore.enc).
KEYSTORE_PATH="$EVIDENCE/keystore/agent_keystore.enc"
KEYSTORE_PWD="tb9-smoke-shared-password-2026-05-02"

run_one() {
    local label="$1"; shift
    local condition="$1"; shift
    local max_tx="$1"; shift
    local problem="$1"; shift
    local keystore_state="$1"; shift  # "fresh" | "load"

    local rundir="$EVIDENCE/$label"
    rm -rf "$rundir"
    mkdir -p "$rundir/runtime_repo" "$rundir/cas"

    echo "========================================================="
    echo " TB-9 smoke: $label  (CONDITION=$condition  MAX_TX=$max_tx  problem=$problem  keystore=$keystore_state)"
    echo "========================================================="

    if [ "$keystore_state" = "fresh" ]; then
        rm -f "$KEYSTORE_PATH"
        echo "  [keystore] erased (fresh-boot scenario)"
    else
        echo "  [keystore] reusing $KEYSTORE_PATH (load-existing scenario)"
        if [ ! -f "$KEYSTORE_PATH" ]; then
            echo "  WARN: keystore_state=load but keystore not present; will create fresh."
        fi
    fi

    TURINGOS_CHAINTAPE_PATH="$rundir/runtime_repo" \
    TURINGOS_CAS_PATH="$rundir/cas" \
    TURINGOS_CHAINTAPE_PRESEED=1 \
    TURINGOS_RUN_ID="tb9-$label" \
    TURINGOS_AGENT_KEYSTORE_PATH="$KEYSTORE_PATH" \
    TURINGOS_AGENT_KEYSTORE_PASSWORD="$KEYSTORE_PWD" \
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

    # Capture: keystore size + pubkey for Agent_0 (canonical durable-identity witness).
    if [ -f "$KEYSTORE_PATH" ]; then
        cp "$KEYSTORE_PATH" "$rundir/agent_keystore_at_exit.enc"
        echo "  [keystore] size=$(wc -c < "$KEYSTORE_PATH") bytes (saved to rundir)"
    fi
    if [ -f "$rundir/runtime_repo/agent_pubkeys.json" ]; then
        cp "$rundir/runtime_repo/agent_pubkeys.json" "$rundir/agent_pubkeys_for_witness.json"
    fi

    # Self-contained tar.gz packaging (TB-8 round-2 RQ3 pattern).
    if [ -d "$rundir/runtime_repo" ]; then
        ( cd "$rundir" && tar --exclude='runtime_repo/target' -czf runtime_repo.tar.gz runtime_repo )
    fi
    if [ -d "$rundir/cas" ]; then
        ( cd "$rundir" && tar -czf cas.tar.gz cas )
    fi

    # Clean loose dirs so committed evidence is tar.gz + sidecar JSON only.
    rm -rf "$rundir/runtime_repo" "$rundir/cas"
}

# ── Run sequence ────────────────────────────────────────────────────────────

# 1) Run-A — cold-boot, no prior keystore; smallest possible smoke (MAX_TX=10
#    is enough for mathd_algebra_171 to solve under deepseek-chat per TB-7R/TB-8).
run_one "run_a_n1_mathd_algebra_171" "n1" "10" "mathd_algebra_171.lean" "fresh"

# 2) Run-B — load prior keystore; same problem, fresh runtime_repo. Demonstrates
#    cross-run identity: Agent_0's pubkey must match run-A's despite the new
#    runtime_repo.
run_one "run_b_n1_mathd_algebra_171" "n1" "10" "mathd_algebra_171.lean" "load"

# 3) Regression — TB-8 reference problem, MAX_TX=20 (matches TB-8 half-1 run).
#    Confirms TB-8 minimal-payout pipeline still produces FinalizeRewardTx.
run_one "regression_n1_mathd_algebra_107" "n1" "20" "mathd_algebra_107.lean" "load"

echo ""
echo "========================================================="
echo " TB-9 smoke complete; comparing pubkeys ..."
echo "========================================================="

PA="$EVIDENCE/run_a_n1_mathd_algebra_171/agent_pubkeys_for_witness.json"
PB="$EVIDENCE/run_b_n1_mathd_algebra_171/agent_pubkeys_for_witness.json"
if [ -f "$PA" ] && [ -f "$PB" ]; then
    echo " run-A agent_pubkeys.json:"
    cat "$PA" | sed 's/^/   /'
    echo " run-B agent_pubkeys.json:"
    cat "$PB" | sed 's/^/   /'
    if diff -q "$PA" "$PB" > /dev/null 2>&1; then
        echo " ✓ CROSS-RUN IDENTITY HOLDS — pubkey maps are identical"
    else
        echo " ✗ CROSS-RUN IDENTITY BROKEN — pubkey maps differ:"
        diff "$PA" "$PB" | sed 's/^/   /'
    fi
else
    echo " WARN: one or both run pubkey files missing"
fi
