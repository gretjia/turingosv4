#!/usr/bin/env bash
# Paper 1 v2 reproducer entrypoint.
#
# Without args: runs the pre-registered 4-seed × 3-condition sweep from § 8.2.
# With args:    passes through to run_list.sh.
#
# Requires DEEPSEEK_API_KEY env.

set -euo pipefail

: "${DEEPSEEK_API_KEY:?set DEEPSEEK_API_KEY to a valid deepseek.com key}"
cd /work/turingosv4
source /work/build_sha.env
export BUILD_SHA

if [ "$#" -gt 0 ]; then
    exec bash experiments/minif2f_v4/run_list.sh "$@"
fi

SAMPLE=handover/preregistration/sample_E1v2_hard10_S20260423.txt
for seed in 141421 31415 2718 2357; do
    for mode_label in A B Abl; do
        case "$mode_label" in
            A)   mode_env=(HOMOGENEOUS_AGENTS=1) ;;
            B)   mode_env=() ;;
            Abl) mode_env=(EXCLUDE_META_PLANNER=1) ;;
        esac
        tag="E1v2_${mode_label}_s${seed}"
        echo ">>> $tag (seed=$seed, mode=$mode_label)"
        env TURING_STEP_ONLY=0 TEMP_LADDER=1 HAYEK_BOUNTY=1 TAPE_ECONOMY_V2=1 \
            TICK_INTERVAL=20 MAX_TRANSACTIONS=50 \
            BOLTZMANN_SEED="$seed" ACTIVE_MODEL=deepseek-chat \
            "${mode_env[@]}" \
            bash experiments/minif2f_v4/run_list.sh n8 "$SAMPLE" "$tag"
    done
done

echo "===== reproducer done. Aggregating ====="
python3 tools/aggregate_e1v2.py \
    --logs '/work/turingosv4/experiments/minif2f_v4/logs/E1v2_*_n8_*.jsonl' \
    --out  /out/E1v2_RESULTS.json
echo "===== wrote /out/E1v2_RESULTS.json ====="
