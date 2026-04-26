#!/usr/bin/env bash
# TuringOS v4 — Judge Hook (PreToolUse)
# Combines v3's block-destructive.sh + rule-engine.sh into one entry point.
# Interface: JSON on stdin (Claude Code hook protocol). Exit 0 = allow, exit 2 = block.
#
# A0e-fix 2026-04-25 (post Phase A0 dual audit, both auditors CHALLENGE):
# - Fixed multiple constitution.md guard bypass paths (Bash sed -i,
#   symlink basename, empty-content edit). Now constitution.md is the
#   FIRST guard, with realpath resolution.
# - Fixed R-016 git commit -F /tmp/msg bypass: read message from -F file
#   if present.
# - FC-trace: FC3-S3 readonly subgraph + Art. V.1.1 sudo gate + C-074
#   FC-first commit discipline.

set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
TRACES_DIR="$PROJECT_ROOT/traces/sessions"

INPUT=$(cat)
TOOL_NAME=$(echo "$INPUT" | jq -r '.tool_name // empty' 2>/dev/null)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty' 2>/dev/null)
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // empty' 2>/dev/null)
CONTENT=$(echo "$INPUT" | jq -r '.tool_input.new_string // .tool_input.content // empty' 2>/dev/null)

# ──────────────────────────────────────────────────────────────────────
# CONSTITUTION GUARD (FIRST — before any tool-specific branch)
# ──────────────────────────────────────────────────────────────────────
# Per Codex A0e finding #1: previous version had Bash branch exit before
# this guard, allowing `sed -i constitution.md`, `tee constitution.md`,
# etc. to bypass R-018. Fixed by hoisting the guard to the top.
#
# Per Gemini A0e Q1.c: previous version used basename() which is symlink-
# vulnerable. Fixed by realpath resolution before basename comparison.
#
# Per Codex A0e finding #1 (continued): previous version required
# nonempty CONTENT, allowing empty-replacement edits to bypass. Fixed
# by NOT gating on CONTENT for constitution.md.

constitution_target() {
    # Returns 0 if any of the inputs target constitution.md (resolved
    # through symlinks), 1 otherwise.
    local target="$1"
    if [ -z "$target" ]; then return 1; fi
    # Use realpath -m (allow non-existent paths) to handle both existing
    # files and to-be-created scenarios.
    local resolved
    resolved=$(realpath -m -- "$target" 2>/dev/null || echo "$target")
    local expected
    expected=$(realpath -m -- "$PROJECT_ROOT/constitution.md" 2>/dev/null || echo "$PROJECT_ROOT/constitution.md")
    if [ "$resolved" = "$expected" ]; then return 0; fi
    # Also catch by basename for safety (in case realpath fails)
    if [ "$(basename -- "$target")" = "constitution.md" ]; then return 0; fi
    return 1
}

bash_targets_constitution() {
    # Returns 0 if a Bash command is mutating constitution.md.
    local cmd="$1"
    if [ -z "$cmd" ]; then return 1; fi
    # A0e-fix-2 2026-04-25: skip if command is `git ...`. Git itself never
    # mutates constitution.md inline; quoted commit messages may contain
    # literal mutation-pattern text (e.g., "sed -i constitution.md" in a
    # changelog) that would false-positive.
    if echo "$cmd" | grep -qE '^[[:space:]]*git[[:space:]]'; then return 1; fi
    # Common mutation patterns: sed -i, tee, awk -i, > redirect, >> append,
    # python/perl/ruby file write, etc.
    if echo "$cmd" | grep -qE '(sed|awk|perl|tee)[[:space:]].*constitution\.md'; then return 0; fi
    if echo "$cmd" | grep -qE '(>|>>)[[:space:]]*[^|&;]*constitution\.md'; then return 0; fi
    if echo "$cmd" | grep -qE 'cat[[:space:]].*>[[:space:]]*[^|&;]*constitution\.md'; then return 0; fi
    if echo "$cmd" | grep -qE 'rm[[:space:]].*constitution\.md'; then return 0; fi
    if echo "$cmd" | grep -qE 'mv[[:space:]].*[[:space:]]constitution\.md'; then return 0; fi
    return 1
}

# 1. Edit/Write targeting constitution.md → BLOCK (R-018)
if [ -n "$FILE_PATH" ] && constitution_target "$FILE_PATH"; then
    echo "BLOCKED by R-018 (constitution_amendment_sudo): edit targets constitution.md"
    echo "  Per Art. V.1.1 amendment 2026-04-25: sudo applies *only* to constitution.md."
    echo "  To proceed: USER must explicitly type 'I authorize this constitution amendment' (verbatim) in chat."
    echo "  See cases/C-071_constitution_amendment_process.yaml for the 4-step workflow."
    exit 2
fi

# 2. Bash command mutating constitution.md → BLOCK
if [ "$TOOL_NAME" = "Bash" ] && bash_targets_constitution "$COMMAND"; then
    echo "BLOCKED by R-018 (constitution_amendment_sudo): Bash command mutates constitution.md"
    echo "  Detected pattern: command targets constitution.md via sed/tee/awk/redirect/rm/mv"
    echo "  Per Art. V.1.1 + C-071: constitution.md is sudo-only. Use Edit tool with explicit user authorization."
    echo "  Command (truncated): $(echo "$COMMAND" | head -c 200)"
    exit 2
fi

# ──────────────────────────────────────────────────────────────────────
# Bash: destructive command checks + R-016 fc_trace
# ──────────────────────────────────────────────────────────────────────
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

    # R-016 fc_trace_in_commit (added 2026-04-25, A0e-fix expanded):
    # warn on git commit without FC-trace anywhere in resolvable message body.
    # FC-trace: feedback_fc_first_problem_handling memory + case C-074.
    if echo "$COMMAND" | grep -qE 'git\s+commit(\s|$)'; then
        # Resolve message body from multiple sources:
        #   1. -m "..." (HEREDOC or inline)
        #   2. -F /path/to/file
        #   3. interactive editor (no -m, no -F) — can't inspect; warn anyway
        #
        # A0e-fix 2026-04-25 (Codex Q1.d + Gemini Q1.d): previous version
        # only greppedthe COMMAND string for `FC-trace:`. `git commit -F /tmp/msg`
        # would silently bypass. Now we extract and check the actual message.
        msg_check_passed=0

        # Inline -m or HEREDOC: COMMAND itself contains the message
        if echo "$COMMAND" | grep -qE 'FC-trace:'; then
            msg_check_passed=1
        fi

        # -F file: extract path, read content
        if [ "$msg_check_passed" -eq 0 ]; then
            msg_file=$(echo "$COMMAND" | grep -oE '\-F[[:space:]]+[^[:space:];|&]+' | head -1 | sed 's/^-F[[:space:]]*//')
            if [ -n "$msg_file" ] && [ -f "$msg_file" ]; then
                if grep -qE 'FC-trace:' "$msg_file" 2>/dev/null; then
                    msg_check_passed=1
                fi
            fi
        fi

        if [ "$msg_check_passed" -eq 0 ]; then
            echo "WARNING R-016 / FC-first: git commit without 'FC-trace: <FC?-N?>' in message body." >&2
            echo "  Per memory feedback_fc_first_problem_handling + case C-074: every code commit must trace to a FC1/FC2/FC3 element OR explicitly cite orphan justification (cases/Cxxx, PREREG-§n.m)." >&2
            echo "  If fix legitimately doesn't map to flowchart, write 'FC-trace: orphan / <ref>' in message." >&2
            echo "  (A0e-fix: hook now also reads -F file; if you used neither -m nor -F (interactive editor), inspection is best-effort only.)" >&2
            # warn only — does not block; user decides whether to amend
        fi
    fi
    exit 0
fi

# ──────────────────────────────────────────────────────────────────────
# Edit/Write: rule engine
# ──────────────────────────────────────────────────────────────────────
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
