#!/usr/bin/env bash
# A7 A/B test: swap grill_triage_blackbox_v1 -> v3.
# Backend hot-reads triage prompt per request, no restart needed.
# DO NOT run until orchestrator authorizes (architect /ultraplan 2026-05-19).
#
# Mirrors the F7/F8 sibling-swap pattern. The active v1 path is replaced
# with v3 content; original v1 is preserved at $V1.a7_backup for restore.
#
# Restore command is printed at the end of a successful swap.
#
# This script is INTENTIONALLY non-destructive: it refuses to overwrite an
# existing backup (which would indicate a previous swap was not restored).

set -euo pipefail

REPO=/Users/zephryj/work/turingosv4
V1="$REPO/assets/prompts/grill_triage_blackbox_v1.md"
V3="$REPO/assets/prompts/grill_triage_blackbox_v3.md"
BACKUP="$V1.a7_backup"

if [[ ! -f "$V1" ]]; then
  echo "ERROR: active v1 prompt missing: $V1" >&2
  exit 2
fi

if [[ ! -f "$V3" ]]; then
  echo "ERROR: v3 sibling prompt missing: $V3" >&2
  exit 2
fi

if [[ -f "$BACKUP" ]]; then
  echo "ERROR: backup already exists at $BACKUP" >&2
  echo "       Previous A7 swap was not restored. Restore first:" >&2
  echo "         cp $BACKUP $V1 && rm $BACKUP" >&2
  exit 3
fi

# Also refuse to run if either of the parallel F8 backup or other sibling
# backups are present — implies an unresolved A/B is in flight.
for OTHER_BACKUP in "$V1.f8_backup"; do
  if [[ -f "$OTHER_BACKUP" ]]; then
    echo "ERROR: conflicting backup present: $OTHER_BACKUP" >&2
    echo "       A different A/B (likely F8) is still active." >&2
    echo "       Restore that one first before running A7." >&2
    exit 4
  fi
done

# Capture pre-swap state for the verdict.json record.
echo "[A7] pre-swap v1 SHA256:"
shasum -a 256 "$V1"
echo "[A7] v3 sibling SHA256:"
shasum -a 256 "$V3"
echo "[A7] v1 byte size: $(wc -c < "$V1")"
echo "[A7] v3 byte size: $(wc -c < "$V3")"

# Atomic-ish swap: copy first to backup, then overwrite v1 with v3 content.
cp "$V1" "$BACKUP"
cp "$V3" "$V1"

echo
echo "[A7] swap complete."
echo "[A7] active v1 path now contains v3 content. SHA256:"
shasum -a 256 "$V1"
echo
echo "[A7] backup of original v1 preserved at:"
echo "       $BACKUP"
echo
echo "RESTORE COMMAND (run after Π4 A/B is complete):"
echo "  cp $BACKUP $V1 && rm $BACKUP"
echo
echo "[A7] Backend hot-reads the triage prompt per request; no restart needed."
echo "[A7] Next: orchestrator runs Π4 re-runs for M4/M5/M6/M7/M8 (S9 negctrl)."
