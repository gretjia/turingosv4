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

# Mode → env-var assignment via env command (not shell-variable expansion).
run_one() {
    local tag="$1" seed="$2" mode="$3"
    local logfile="/tmp/e1v2_${tag#E1v2_}.log"
    local env_args=(
        env
        TURING_STEP_ONLY=0
        TEMP_LADDER=1
        HAYEK_BOUNTY=1
        TAPE_ECONOMY_V2=1
        TICK_INTERVAL=20
        MAX_TRANSACTIONS=50
        BOLTZMANN_SEED="$seed"
        ACTIVE_MODEL=deepseek-chat
    )
    case "$mode" in
        A)   env_args+=(HOMOGENEOUS_AGENTS=1) ;;
        B)   ;;
        Abl) env_args+=(EXCLUDE_META_PLANNER=1) ;;
        *) echo "unknown mode $mode" >&2; return 1 ;;
    esac
    (cd "$EXP_DIR" && "${env_args[@]}" bash "$EXP_DIR/experiments/minif2f_v4/run_list.sh" n8 "$SAMPLE" "$tag" > "$logfile" 2>&1) &
    echo $!
}

launch_pair() {
    local tag1="$1" seed1="$2" mode1="$3"
    local tag2="$4" seed2="$5" mode2="$6"
    local round="$7"

    echo "===== ROUND $round: $tag1 + $tag2 starting at $(date +%H:%M:%S) ====="
    local pid1 pid2
    pid1=$(run_one "$tag1" "$seed1" "$mode1")
    pid2=$(run_one "$tag2" "$seed2" "$mode2")

    echo "waiting for PIDs $pid1 and $pid2..."
    wait "$pid1" "$pid2"
    echo "round $round complete at $(date +%H:%M:%S)"
}

launch_pair "E1v2_A_s141421"   141421 A     "E1v2_B_s141421"    141421 B     1
launch_pair "E1v2_Abl_s141421" 141421 Abl   "E1v2_A_s31415"     31415  A     2
launch_pair "E1v2_B_s31415"    31415  B     "E1v2_Abl_s31415"   31415  Abl   3
launch_pair "E1v2_A_s2718"     2718   A     "E1v2_B_s2718"      2718   B     4
launch_pair "E1v2_Abl_s2718"   2718   Abl   "E1v2_A_s2357"      2357   A     5
launch_pair "E1v2_B_s2357"     2357   B     "E1v2_Abl_s2357"    2357   Abl   6

echo "===== ALL ROUNDS DONE at $(date +%H:%M:%S) ====="
