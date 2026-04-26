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
    # R-016 fc_trace_in_commit (added 2026-04-25): warn on git commit without FC-trace
    # FC-trace: feedback_fc_first_problem_handling memory + Art. V.1 (alignment)
    if echo "$COMMAND" | grep -qE 'git\s+commit\s'; then
        if ! echo "$COMMAND" | grep -qE 'FC-trace:'; then
            echo "WARNING R-016 / FC-first: git commit without 'FC-trace: <FC?-N?>' in message body." >&2
            echo "  Per memory feedback_fc_first_problem_handling: every code commit must trace to a FC1/FC2/FC3 element OR explicitly cite orphan justification (cases/Cxxx, PREREG-§n.m)." >&2
            echo "  If fix legitimately doesn't map to flowchart, write 'FC-trace: orphan / <ref>' in message." >&2
            # warn only — does not block; user decides whether to amend
        fi
    fi
    exit 0
fi

# --- constitution.md: ALWAYS goes through rule engine (R-018 sudo gate) ---
# Bypasses the skip-list below. Rationale: Art. V.1.1 amendment 2026-04-25 made
# constitution.md the only sudo-required file; R-018 enforces the gate. Without
# this special-case, the skip-list's '*.md' pattern would silently let
# constitution.md edits through (which actually happened during the 2026-04-25
# session — see commit c061450 amendment).
if [ -n "$FILE_PATH" ] && [ "$(basename "$FILE_PATH")" = "constitution.md" ] && [ -n "$CONTENT" ]; then
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
