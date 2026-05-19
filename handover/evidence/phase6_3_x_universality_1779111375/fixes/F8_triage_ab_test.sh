#!/usr/bin/env bash
# F8 A/B test: swap grill_triage_blackbox_v1 -> v2.
# Backend hot-reads triage prompt per request, no restart needed.
# DO NOT run until orchestrator authorizes.

set -euo pipefail

REPO=/Users/zephryj/work/turingosv4
V1="$REPO/assets/prompts/grill_triage_blackbox_v1.md"
V2="$REPO/assets/prompts/grill_triage_blackbox_v2.md"
BACKUP="$V1.f8_backup"

if [[ ! -f "$V2" ]]; then echo "ERROR: $V2 missing"; exit 2; fi
if [[ -f "$BACKUP" ]]; then echo "ERROR: $BACKUP exists; previous run did not restore"; exit 3; fi

cp "$V1" "$BACKUP"
cp "$V2" "$V1"
echo "[F8] swapped. v1 (now v2 content) SHA256:"
shasum -a 256 "$V1"
echo
echo "RESTORE: cp $BACKUP $V1 && rm $BACKUP"
