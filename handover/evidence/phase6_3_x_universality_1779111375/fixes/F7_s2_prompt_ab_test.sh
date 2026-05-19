#!/usr/bin/env bash
# F7 A/B test: swap grill_meta_v1 → grill_meta_v2 and re-run Mrs Chen.
#
# This is the S2-predicate falsifiability experiment: can a prompt-only edit
# fix the slot-extraction-too-conservative + Voss-mirror-loop behavior with
# NO Rust code change?
#
# DO NOT RUN until orchestrator authorizes. Backend must be running with
# v1 mounted; this script swaps v1 → v2 on disk, then a separate wave
# runner must be invoked to re-execute Mrs Chen against the swapped prompt.
#
# Backend may need a restart depending on whether assets/prompts/grill_meta_v1.md
# is hot-read per request or cached at boot — verify before running.
#
# Restore command is echoed at the end. Always restore before shipping.

set -euo pipefail

REPO=/Users/zephryj/work/turingosv4
V1="$REPO/assets/prompts/grill_meta_v1.md"
V2="$REPO/assets/prompts/grill_meta_v2.md"
BACKUP="$V1.f7_backup"

if [[ ! -f "$V2" ]]; then
  echo "ERROR: $V2 missing. Author grill_meta_v2.md first." >&2
  exit 2
fi

if [[ -f "$BACKUP" ]]; then
  echo "ERROR: $BACKUP already exists. A previous F7 run did not restore." >&2
  echo "Inspect manually before proceeding." >&2
  exit 3
fi

echo "[F7] Backing up v1 → v1.f7_backup"
cp "$V1" "$BACKUP"

echo "[F7] Swapping v2 into v1 position"
cp "$V2" "$V1"

echo "[F7] Swap complete. v1 SHA256:"
shasum -a 256 "$V1"

cat <<'EOF'

NEXT STEPS (manual):
  1. If the backend caches the meta-prompt at boot, restart it now.
  2. Re-run Mrs Chen via the Wave 1 runner template:
       (insert wave1 runner invocation here — typically a curl-driven
        scripted session against the spec grill HTTP surface)
  3. Compare new session_log.jsonl against:
       handover/evidence/phase6_3_x_universality_1779111375/wave1/mrs_chen/session_log.jsonl
     Look for: covered_slots growing past ["job"] by turn 5, confidence
     ≥ 0.5 by turn 5, ≤ 1 "听起来你是说" opening per 5 turns.
  4. Optionally re-run P1 + P4 personas to check v2 does not regress them.

WHEN DONE, ALWAYS RESTORE:
  cp /Users/zephryj/work/turingosv4/assets/prompts/grill_meta_v1.md.f7_backup \
     /Users/zephryj/work/turingosv4/assets/prompts/grill_meta_v1.md
  rm  /Users/zephryj/work/turingosv4/assets/prompts/grill_meta_v1.md.f7_backup
  (and restart backend again if it caches the prompt)

EOF
