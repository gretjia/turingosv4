#!/usr/bin/env bash
# Aggregate PPUT_RESULT lines from each per-problem log into a single JSONL.
set -e
DIR="$(dirname "$0")"
OUT="$DIR/batch_results.jsonl"
: > "$OUT"
for log in "$DIR"/*.log; do
  pid="$(basename "$log" .log)"
  ppm="$(grep -oE 'PPUT_RESULT:\{[^}]*\}(?:[^}]*\})*' "$log" | head -1 | sed 's/^PPUT_RESULT://')"
  if [ -z "$ppm" ]; then
    # fallback: find the {"schema_version":"v2.0",...} JSON line in full
    ppm="$(grep -oE 'PPUT_RESULT:\{.*\}' "$log" | head -1 | sed 's/^PPUT_RESULT://')"
  fi
  if [ -n "$ppm" ]; then
    echo "$ppm" >> "$OUT"
  else
    echo "{\"problem_id\":\"$pid\",\"error\":\"no PPUT_RESULT in log\"}" >> "$OUT"
  fi
done
echo "Aggregated $(wc -l < "$OUT") rows → $OUT"
