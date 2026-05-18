#!/bin/bash
# Backend Observer for W8.1 Final Validation
# Polls every 15s: log_stream, process_tree, workspace_evolution
# Special focus: sessions/ MUST appear in tmp/phase7_active/, NOT repo root

set -u
EVD="/Users/zephryj/work/turingosv4/handover/evidence/stage_phase7_w8_1_final_validation_20260518T044722Z/backend_observer"
LOG_SRC="/private/tmp/turingos_web_live.log"
SERVER_PID=92364
REPO_ROOT="/Users/zephryj/work/turingosv4"
WORKSPACE="${REPO_ROOT}/tmp/phase7_active"
DURATION_SEC=3600  # 60 min max
INTERVAL=15

LOG_OUT="${EVD}/log_stream.txt"
PROC_OUT="${EVD}/process_tree.txt"
WS_OUT="${EVD}/workspace_evolution.txt"
EVENTS_OUT="${EVD}/notable_events.txt"
P2_OUT="${EVD}/p2_check.txt"
START_TIME=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

echo "# Backend Observer started ${START_TIME}" > "${LOG_OUT}"
echo "# Backend Observer started ${START_TIME}" > "${PROC_OUT}"
echo "# Backend Observer started ${START_TIME}" > "${WS_OUT}"
echo "# Notable events log ${START_TIME}" > "${EVENTS_OUT}"
echo "# P2 sessions/ location check ${START_TIME}" > "${P2_OUT}"

# Track last log offset so we only emit new lines
LAST_OFFSET=$(stat -f %z "${LOG_SRC}" 2>/dev/null || echo 0)
echo "Initial log offset: ${LAST_OFFSET}" >> "${LOG_OUT}"

ITER=0
DEADLINE=$(( $(date +%s) + DURATION_SEC ))
while [ "$(date +%s)" -lt "${DEADLINE}" ]; do
    ITER=$((ITER + 1))
    TS=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

    # ---- log_stream: tail new bytes from server log ----
    CUR_SIZE=$(stat -f %z "${LOG_SRC}" 2>/dev/null || echo 0)
    if [ "${CUR_SIZE}" -gt "${LAST_OFFSET}" ]; then
        echo "=== [${TS}] iter=${ITER} new_bytes=$((CUR_SIZE - LAST_OFFSET)) ===" >> "${LOG_OUT}"
        tail -c +$((LAST_OFFSET + 1)) "${LOG_SRC}" | head -c $((CUR_SIZE - LAST_OFFSET)) >> "${LOG_OUT}"
        echo "" >> "${LOG_OUT}"
        LAST_OFFSET=${CUR_SIZE}

        # Scan for notable events in new chunk
        NEW_CHUNK=$(tail -c $((CUR_SIZE - 0)) "${LOG_SRC}" | tail -200)
        echo "${NEW_CHUNK}" | grep -E "generate_attempt_(started|failed)|generate_complete|has_playfield|verify|retry|llm_request|qwen|ERROR|panic" | tail -20 | while read -r LINE; do
            [ -n "${LINE}" ] && echo "[${TS}] ${LINE}" >> "${EVENTS_OUT}"
        done
    fi

    # ---- process_tree ----
    if kill -0 "${SERVER_PID}" 2>/dev/null; then
        ALIVE="ALIVE"
    else
        ALIVE="DEAD"
        echo "[${TS}] SERVER PID ${SERVER_PID} IS DEAD" >> "${EVENTS_OUT}"
    fi
    echo "[${TS}] iter=${ITER} pid=${SERVER_PID} status=${ALIVE}" >> "${PROC_OUT}"
    ps -p "${SERVER_PID}" -o pid,etime,rss,vsz,command 2>/dev/null >> "${PROC_OUT}" || echo "(process gone)" >> "${PROC_OUT}"
    echo "" >> "${PROC_OUT}"

    # ---- workspace_evolution ----
    echo "=== [${TS}] iter=${ITER} ===" >> "${WS_OUT}"
    if [ -d "${WORKSPACE}" ]; then
        echo "tmp/phase7_active tree:" >> "${WS_OUT}"
        find "${WORKSPACE}" -maxdepth 4 -type d 2>/dev/null | sort >> "${WS_OUT}"
        # Sessions listing
        if [ -d "${WORKSPACE}/sessions" ]; then
            echo "--- sessions/ contents:" >> "${WS_OUT}"
            ls -la "${WORKSPACE}/sessions/" 2>/dev/null | head -30 >> "${WS_OUT}"
            for SID in "${WORKSPACE}/sessions"/*/; do
                [ -d "${SID}" ] || continue
                echo "  session: $(basename "${SID}")" >> "${WS_OUT}"
                ls -la "${SID}" 2>/dev/null | head -10 >> "${WS_OUT}"
                [ -f "${SID}/state.json" ] && echo "    state.json next_step: $(grep -o '"next_step"[^,}]*' "${SID}/state.json" 2>/dev/null)" >> "${WS_OUT}"
            done
        fi
    else
        echo "tmp/phase7_active does not exist yet" >> "${WS_OUT}"
    fi
    echo "" >> "${WS_OUT}"

    # ---- P2 check: sessions/ must be in workspace, not repo root ----
    REPO_ROOT_SESSIONS_EXIST=0
    REPO_ROOT_CAS_EXIST=0
    [ -d "${REPO_ROOT}/sessions" ] && REPO_ROOT_SESSIONS_EXIST=1
    [ -d "${REPO_ROOT}/cas" ] && REPO_ROOT_CAS_EXIST=1
    WORKSPACE_SESSIONS_EXIST=0
    [ -d "${WORKSPACE}/sessions" ] && WORKSPACE_SESSIONS_EXIST=1
    echo "[${TS}] iter=${ITER} repo_root_sessions=${REPO_ROOT_SESSIONS_EXIST} repo_root_cas=${REPO_ROOT_CAS_EXIST} workspace_sessions=${WORKSPACE_SESSIONS_EXIST}" >> "${P2_OUT}"
    if [ "${REPO_ROOT_SESSIONS_EXIST}" -eq 1 ] || [ "${REPO_ROOT_CAS_EXIST}" -eq 1 ]; then
        echo "[${TS}] !!! P2 REGRESSION: sessions/cas appeared at repo root !!!" >> "${EVENTS_OUT}"
    fi

    sleep "${INTERVAL}"
done

echo "# Backend Observer terminated $(date -u +"%Y-%m-%dT%H:%M:%SZ")" >> "${LOG_OUT}"
echo "# Backend Observer terminated $(date -u +"%Y-%m-%dT%H:%M:%SZ")" >> "${PROC_OUT}"
echo "# Backend Observer terminated $(date -u +"%Y-%m-%dT%H:%M:%SZ")" >> "${WS_OUT}"
