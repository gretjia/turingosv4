#!/bin/bash
# CO0.7' Trust Root mutation governance hook
#
# Verifies every commit that touched genesis_payload.toml has a corresponding
# user-signed git tag (per B-1 governance gate, RATIFICATION_2026-04-27.md § 3).
#
# Usage:
#   bash scripts/check_tr_ratification_chain.sh
#   bash scripts/check_tr_ratification_chain.sh --strict    # fail (exit 1) on first unsigned mutation
#   bash scripts/check_tr_ratification_chain.sh --since=COMMIT  # only check from COMMIT onward
#
# Exit codes:
#   0 — all TR mutations ratified
#   1 — one or more TR mutations lack signed tag (in --strict mode)
#   2 — environment misconfigured (no SSH allowed_signers, etc.)
#
# Output: human-readable report; also emits machine-readable JSON to stderr if --json.

set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"
TR_FILE="$REPO_ROOT/genesis_payload.toml"

STRICT=0
SINCE="0dd0d35"  # Phase A8 baseline; TR governance starts post-Phase A8
EMIT_JSON=0

for arg in "$@"; do
    case "$arg" in
        --strict) STRICT=1 ;;
        --since=*) SINCE="${arg#*=}" ;;
        --json) EMIT_JSON=1 ;;
    esac
done

# Verify SSH allowed_signers exists (required for git verify-tag)
if ! git config --get gpg.ssh.allowedSignersFile > /dev/null 2>&1; then
    echo "ERROR: gpg.ssh.allowedSignersFile not configured. Run:" >&2
    echo "  echo \"\$(git config user.email) namespaces=\\\"git\\\" \$(cat ~/.ssh/id_ed25519_*.pub)\" > ~/.config/git/allowed_signers" >&2
    echo "  git config gpg.ssh.allowedSignersFile ~/.config/git/allowed_signers" >&2
    exit 2
fi

# Find all commits touching genesis_payload.toml since SINCE
TR_COMMITS=$(git log --format='%H' --follow "$SINCE..HEAD" -- "$TR_FILE")

if [ -z "$TR_COMMITS" ]; then
    echo "No TR mutations since $SINCE."
    exit 0
fi

# Find all signed tags
SIGNED_TAGS=$(git tag --list 'v4-ratify-*' 'v4-syskey-*' 'v4-co-*' 2>/dev/null)

# For each TR mutation commit, find a signed tag that covers it (i.e., tag's commit ancestry includes this commit)
UNSIGNED=()
SIGNED=()
for commit in $TR_COMMITS; do
    short=$(git rev-parse --short "$commit")
    msg=$(git log -1 --format='%s' "$commit")

    found_tag=""
    for tag in $SIGNED_TAGS; do
        # Check if commit is ancestor of tag's target
        tag_target=$(git rev-parse "$tag^{commit}" 2>/dev/null || true)
        if [ -n "$tag_target" ] && git merge-base --is-ancestor "$commit" "$tag_target" 2>/dev/null; then
            # Verify the tag's signature
            if git verify-tag "$tag" > /dev/null 2>&1; then
                found_tag="$tag"
                break
            fi
        fi
    done

    if [ -n "$found_tag" ]; then
        SIGNED+=("$short|$found_tag|$msg")
    else
        UNSIGNED+=("$short|$msg")
    fi
done

# Report
echo "=== TR Mutation Ratification Chain Report ==="
echo "Range: $SINCE..HEAD"
echo "Total TR mutations: $(echo "$TR_COMMITS" | wc -l)"
echo "Ratified (covered by valid signed tag): ${#SIGNED[@]}"
echo "Unratified (NO valid signed tag): ${#UNSIGNED[@]}"
echo

if [ ${#SIGNED[@]} -gt 0 ]; then
    echo "✓ RATIFIED:"
    for entry in "${SIGNED[@]}"; do
        IFS='|' read -r short tag msg <<< "$entry"
        echo "  $short  ← $tag  ($msg)"
    done
    echo
fi

if [ ${#UNSIGNED[@]} -gt 0 ]; then
    echo "⚠ UNRATIFIED (require user signed tag):"
    for entry in "${UNSIGNED[@]}"; do
        IFS='|' read -r short msg <<< "$entry"
        echo "  $short  ($msg)"
    done
    echo
    echo "To ratify, run:"
    echo "  git tag -s v4-ratify-2026-XX-XX-<short> -m \"<reason>\" <commit>"
    echo "  git push origin v4-ratify-2026-XX-XX-<short>"
    echo "  bash scripts/check_tr_ratification_chain.sh   # re-verify"
    echo
fi

if [ $EMIT_JSON -eq 1 ]; then
    {
        echo "{"
        echo "  \"range\": \"$SINCE..HEAD\","
        echo "  \"total\": $(echo "$TR_COMMITS" | wc -l),"
        echo "  \"ratified\": ${#SIGNED[@]},"
        echo "  \"unratified\": ${#UNSIGNED[@]}"
        echo "}"
    } >&2
fi

if [ $STRICT -eq 1 ] && [ ${#UNSIGNED[@]} -gt 0 ]; then
    exit 1
fi

exit 0
