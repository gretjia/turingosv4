#!/usr/bin/env bash
# TuringOS v4 — Build Check Hook (PostToolUse Edit|Write)
# Runs cargo check on core file edits. Silent on success, exit 2 on failure.

set -uo pipefail

INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty' 2>/dev/null)

[ -z "$FILE_PATH" ] && exit 0

# Only check core Rust files
case "$FILE_PATH" in
    */kernel.rs|*/bus.rs|*/wallet.rs|*/tool.rs|*/prediction_market.rs|*/ledger.rs|*/Cargo.toml)
        ;;
    *)
        exit 0
        ;;
esac

PROJECT_ROOT="$(git rev-parse --show-toplevel 2>/dev/null)"
[ -z "$PROJECT_ROOT" ] && exit 0

# Silent on success, output errors on failure
OUTPUT=$(cd "$PROJECT_ROOT" && cargo check --quiet 2>&1)
if [ $? -ne 0 ]; then
    echo "BUILD FAILED after editing $FILE_PATH:"
    echo "$OUTPUT"
    exit 2
fi

exit 0
