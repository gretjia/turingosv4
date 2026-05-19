#!/usr/bin/env bash
# A8 — Synthesis v1↔v2 A/B swap script
#
# Class:    1 (prompt swap; reversible)
# Scope:    assets/prompts/grill_synthesis_{zh,en}.md
# Forbidden: any Rust source, any backend, any test
#
# This script swaps the SYNTHESIS prompt files between v1 and v2 for an A/B run
# against the M5 P7-Traditional evidence to test D-NEW-1 (synthesis-layer
# hallucination) hypothesis. See A8_playback_v2_design.md §3 for expected
# outcomes.
#
# IMPORTANT: at the time of writing, cmd_spec.rs::system_prompt() (src/bin/
# turingos/cmd_spec.rs:1505–1554) carries the v1 synthesis prompt as an
# INLINE Rust string literal — the .md files in assets/prompts/ are
# documentation mirrors, not the runtime source of truth. A pure file swap
# therefore changes the documented prompt but NOT the binary's behaviour.
#
# Two modes are supported:
#   - --mode=docs   : swap only the .md files (default; safe, no rebuild)
#   - --mode=binary : also patch the inline Rust literal and rebuild
#                     (requires Class-2 architect ratification — DO NOT run
#                     without explicit sign-off; the script will refuse
#                     unless TURINGOS_A8_BINARY_SWAP_OK=1 is set)
#
# Usage:
#   bash A8_playback_v2_ab_test.sh on   [--mode=docs|binary]   # activate v2
#   bash A8_playback_v2_ab_test.sh off  [--mode=docs|binary]   # restore v1
#   bash A8_playback_v2_ab_test.sh status
#
# All file ops are idempotent: re-running `on` when already on, or `off` when
# already off, is a no-op + diagnostic print.

set -euo pipefail

# ── Resolve repo root from script location ───────────────────────────────────
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../../../.." && pwd)"

PROMPTS_DIR="${REPO_ROOT}/assets/prompts"
ZH_V1="${PROMPTS_DIR}/grill_synthesis_zh.md"
ZH_V2="${PROMPTS_DIR}/grill_synthesis_zh_v2.md"
EN_V1="${PROMPTS_DIR}/grill_synthesis_en.md"
EN_V2="${PROMPTS_DIR}/grill_synthesis_en_v2.md"

# Backup paths (created on first `on`, restored on `off`)
BACKUP_DIR="${SCRIPT_DIR}/.a8_v1_backup"
ZH_BACKUP="${BACKUP_DIR}/grill_synthesis_zh.md"
EN_BACKUP="${BACKUP_DIR}/grill_synthesis_en.md"

# State marker file
STATE_FILE="${BACKUP_DIR}/state"

# ── Arg parse ───────────────────────────────────────────────────────────────
ACTION="${1:-status}"
MODE="docs"
for arg in "${@:2}"; do
    case "$arg" in
        --mode=docs)   MODE="docs" ;;
        --mode=binary) MODE="binary" ;;
        *) echo "unknown arg: $arg" >&2; exit 2 ;;
    esac
done

# ── Pre-flight ──────────────────────────────────────────────────────────────
require_file() {
    [[ -f "$1" ]] || { echo "FATAL: missing required file: $1" >&2; exit 3; }
}

require_file "$ZH_V1"
require_file "$ZH_V2"
require_file "$EN_V1"
require_file "$EN_V2"

# ── State helpers ───────────────────────────────────────────────────────────
read_state() {
    if [[ -f "$STATE_FILE" ]]; then
        cat "$STATE_FILE"
    else
        echo "off"
    fi
}

write_state() {
    mkdir -p "$BACKUP_DIR"
    printf '%s\n' "$1" > "$STATE_FILE"
}

# ── Actions ─────────────────────────────────────────────────────────────────
do_on() {
    local cur
    cur="$(read_state)"
    if [[ "$cur" == "on" ]]; then
        echo "[a8] already ON (state=$cur); no-op"
        return 0
    fi

    echo "[a8] activating v2 synthesis prompts (mode=$MODE)"

    mkdir -p "$BACKUP_DIR"
    cp -p "$ZH_V1" "$ZH_BACKUP"
    cp -p "$EN_V1" "$EN_BACKUP"

    # Docs-mode swap: overwrite the v1 .md files with v2 content.
    cp -p "$ZH_V2" "$ZH_V1"
    cp -p "$EN_V2" "$EN_V1"
    echo "[a8] .md files swapped (v2 content now at v1 paths; v1 backed up to $BACKUP_DIR)"

    if [[ "$MODE" == "binary" ]]; then
        if [[ "${TURINGOS_A8_BINARY_SWAP_OK:-0}" != "1" ]]; then
            echo "[a8] REFUSING binary-mode swap without TURINGOS_A8_BINARY_SWAP_OK=1" >&2
            echo "[a8] Binary-mode swap modifies src/bin/turingos/cmd_spec.rs (Class 2)" >&2
            echo "[a8] and requires explicit architect ratification." >&2
            do_off >/dev/null 2>&1 || true
            exit 4
        fi
        echo "[a8] binary-mode: patching cmd_spec.rs::system_prompt() inline literal..."
        echo "[a8] (this requires the v2 .md content to be embedded into the Rust string;"
        echo "[a8]  TODO: implement when architect confirms preferred wiring strategy —"
        echo "[a8]  options: (a) include_str! the .md at compile time, (b) read at runtime"
        echo "[a8]  via std::fs, (c) hand-patch the literal.)"
        echo "[a8] binary-mode is a stub; v2 will NOT affect binary behaviour until wired."
    fi

    write_state "on"
    echo "[a8] state=on (mode=$MODE)"
}

do_off() {
    local cur
    cur="$(read_state)"
    if [[ "$cur" == "off" ]]; then
        echo "[a8] already OFF (state=$cur); no-op"
        return 0
    fi

    echo "[a8] restoring v1 synthesis prompts"
    if [[ -f "$ZH_BACKUP" ]]; then
        cp -p "$ZH_BACKUP" "$ZH_V1"
    else
        echo "[a8] WARN: zh v1 backup missing; manual restore required" >&2
    fi
    if [[ -f "$EN_BACKUP" ]]; then
        cp -p "$EN_BACKUP" "$EN_V1"
    else
        echo "[a8] WARN: en v1 backup missing; manual restore required" >&2
    fi

    write_state "off"
    echo "[a8] state=off"
}

do_status() {
    local cur
    cur="$(read_state)"
    echo "[a8] state=$cur"
    echo "[a8] v1 zh: $ZH_V1 ($(wc -c <"$ZH_V1" | tr -d ' ') B)"
    echo "[a8] v1 en: $EN_V1 ($(wc -c <"$EN_V1" | tr -d ' ') B)"
    echo "[a8] v2 zh: $ZH_V2 ($(wc -c <"$ZH_V2" | tr -d ' ') B)"
    echo "[a8] v2 en: $EN_V2 ($(wc -c <"$EN_V2" | tr -d ' ') B)"
    if [[ -d "$BACKUP_DIR" ]]; then
        echo "[a8] backup dir present: $BACKUP_DIR"
        ls -la "$BACKUP_DIR" 2>/dev/null || true
    fi
    echo
    echo "[a8] NOTE: cmd_spec.rs::system_prompt() (src/bin/turingos/cmd_spec.rs:1505)"
    echo "[a8]       carries an inline Rust string literal of v1 — pure file swap"
    echo "[a8]       affects docs only, NOT runtime behaviour. Binary swap requires"
    echo "[a8]       TURINGOS_A8_BINARY_SWAP_OK=1 and architect ratification."
}

case "$ACTION" in
    on)     do_on ;;
    off)    do_off ;;
    status) do_status ;;
    *) echo "usage: $0 {on|off|status} [--mode=docs|binary]" >&2; exit 2 ;;
esac
