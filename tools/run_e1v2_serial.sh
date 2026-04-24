#!/usr/bin/env bash
# Serialized E1 v2 runner: 2 parallel batches per round, 6 rounds total.
# Addresses proxy saturation finding (PROXY_SATURATION_FINDING_2026-04-24.md).
#
# Usage: bash tools/run_e1v2_serial.sh
# Output logs: /tmp/e1v2_<TAG>.log + logs/E1v2_<TAG>_n8_<ts>.jsonl
#
# Expected wallclock: ~6h (6 rounds × ~60min each)

set -uo pipefail

EXP_DIR="/home/zephryj/projects/turingosv4/.claude/worktrees/phase-8a-snapshot"
SAMPLE="$EXP_DIR/experiments/minif2f_v4/analysis/sample_E1v2_hard10_S20260423.txt"

launch_pair() {
    local tag1="$1" seed1="$2" mode1="$3"
    local tag2="$4" seed2="$5" mode2="$6"
    local round="$7"

    echo "===== ROUND $round ====="
    echo "launch: $tag1 seed=$seed1 mode=$mode1"
    echo "launch: $tag2 seed=$seed2 mode=$mode2"

    # Build env strings
    local env1=""
    case "$mode1" in
        A) env1="HOMOGENEOUS_AGENTS=1" ;;
        B) env1="" ;;
        Abl) env1="EXCLUDE_META_PLANNER=1" ;;
    esac
    local env2=""
    case "$mode2" in
        A) env2="HOMOGENEOUS_AGENTS=1" ;;
        B) env2="" ;;
        Abl) env2="EXCLUDE_META_PLANNER=1" ;;
    esac

    # Launch both in background
    cd "$EXP_DIR" && \
        TURING_STEP_ONLY=0 TEMP_LADDER=1 HAYEK_BOUNTY=1 TAPE_ECONOMY_V2=1 \
        TICK_INTERVAL=20 MAX_TRANSACTIONS=50 \
        BOLTZMANN_SEED=$seed1 $env1 \
        ACTIVE_MODEL=deepseek-chat \
        bash "$EXP_DIR/experiments/minif2f_v4/run_list.sh" n8 "$SAMPLE" "$tag1" > /tmp/e1v2_${tag1#E1v2_}.log 2>&1 &
    local pid1=$!

    cd "$EXP_DIR" && \
        TURING_STEP_ONLY=0 TEMP_LADDER=1 HAYEK_BOUNTY=1 TAPE_ECONOMY_V2=1 \
        TICK_INTERVAL=20 MAX_TRANSACTIONS=50 \
        BOLTZMANN_SEED=$seed2 $env2 \
        ACTIVE_MODEL=deepseek-chat \
        bash "$EXP_DIR/experiments/minif2f_v4/run_list.sh" n8 "$SAMPLE" "$tag2" > /tmp/e1v2_${tag2#E1v2_}.log 2>&1 &
    local pid2=$!

    echo "waiting for pair to finish (PIDs: $pid1 $pid2)..."
    wait $pid1 $pid2
    echo "round $round complete at $(date +%H:%M:%S)"
}

# 6 rounds, each 2 batches
launch_pair "E1v2_A_s141421"   141421 A     "E1v2_B_s141421"    141421 B     1
launch_pair "E1v2_Abl_s141421" 141421 Abl   "E1v2_A_s31415"     31415  A     2
launch_pair "E1v2_B_s31415"    31415  B     "E1v2_Abl_s31415"   31415  Abl   3
launch_pair "E1v2_A_s2718"     2718   A     "E1v2_B_s2718"      2718   B     4
launch_pair "E1v2_Abl_s2718"   2718   Abl   "E1v2_A_s2357"      2357   A     5
launch_pair "E1v2_B_s2357"     2357   B     "E1v2_Abl_s2357"    2357   Abl   6

echo "===== ALL ROUNDS DONE ====="
