#!/usr/bin/env bash
# TuringOS v4 — Judge Hook (PreToolUse)
# Combines v3's block-destructive.sh + rule-engine.sh into one entry point.
# Interface: JSON on stdin (Claude Code hook protocol). Exit 0 = allow, exit 2 = block.

set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
TRACES_DIR="$PROJECT_ROOT/traces/sessions"

INPUT=$(cat)
TOOL_NAME=$(echo "$INPUT" | jq -r '.tool_name // empty' 2>/dev/null)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty' 2>/dev/null)
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // empty' 2>/dev/null)
CONTENT=$(echo "$INPUT" | jq -r '.tool_input.new_string // .tool_input.content // empty' 2>/dev/null)

# --- Bash: destructive command checks ---
if [ "$TOOL_NAME" = "Bash" ] && [ -n "$COMMAND" ]; then
    # Block rm -rf on dangerous paths
    if echo "$COMMAND" | grep -qE 'rm\s+(-[a-zA-Z]*r[a-zA-Z]*f|--recursive\s+--force|-[a-zA-Z]*f[a-zA-Z]*r)\s'; then
        if echo "$COMMAND" | grep -qE '(^|\s)(\/|~\/|\.\.\/|\.claude)'; then
            echo "BLOCKED: rm -rf on dangerous path: $COMMAND"
            exit 2
        fi
    fi
    # Block git push --force
    if echo "$COMMAND" | grep -qE 'git\s+push\s+.*--force|git\s+push\s+-f'; then
        echo "BLOCKED: git push --force is prohibited."
        exit 2
    fi
    # Block git reset --hard
    if echo "$COMMAND" | grep -qE 'git\s+reset\s+--hard'; then
        echo "BLOCKED: git reset --hard is prohibited."
        exit 2
    fi
    # Block WAL deletion
    if echo "$COMMAND" | grep -qE 'rm\s.*\.(wal|jsonl)'; then
        echo "BLOCKED: WAL/ledger file deletion is prohibited."
        exit 2
    fi
    # Block sed/awk on kernel constants
    if echo "$COMMAND" | grep -qE '(sed|awk).*kernel\.rs'; then
        echo "BLOCKED: sed/awk on kernel.rs is prohibited. Use Edit tool."
        exit 2
    fi
    exit 0
fi

# --- Edit/Write: rule engine ---
if [ -n "$FILE_PATH" ] && [ -n "$CONTENT" ]; then
    # Skip non-code files (docs, incidents, rules, handover, tests, audit)
    case "$FILE_PATH" in
        *.md|*/incidents/*|*/rules/*|*/handover/*|*/tests/*|*/audit/*) exit 0 ;;
    esac

    # Call the Python rule engine
    if [ -d "$PROJECT_ROOT/rules/active" ]; then
        RESULT=$(echo "$CONTENT" | python3 "$PROJECT_ROOT/rules/engine.py" \
            --file "$FILE_PATH" \
            --rules-dir "$PROJECT_ROOT/rules/active" \
            --log "$PROJECT_ROOT/rules/enforcement.log" \
            --traces-dir "$TRACES_DIR" 2>&1)
        EXIT_CODE=$?
        if [ $EXIT_CODE -eq 2 ]; then
            echo "$RESULT"
            exit 2
        elif [ -n "$RESULT" ]; then
            echo "$RESULT"
        fi
    fi
fi

exit 0
