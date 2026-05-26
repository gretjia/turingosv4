#!/usr/bin/env bash
# True-suite GPQA science-reasoning current-kernel evidence runner.
#
# Uses a public GPQA dataset sample plus a real external LLM through the local
# OpenAI-compatible proxy. The model answer is parsed into a structured claim,
# hashed into CAS, submitted as a signed WorkTx through current ChainTape, and
# replayed through public `turingos verify chaintape`.

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RUN_ID="${1:-gpqa_science_reasoning_$(date -u +%Y%m%dT%H%M%SZ)}"
RUN_ID="${RUN_ID#handover/evidence/true_suite/}"
RUN_ROOT="$PROJECT_ROOT/handover/evidence/true_suite/$RUN_ID"
RUN_DIR="$RUN_ROOT/gpqa"
LLM_PROXY_URL="${LLM_PROXY_URL:-http://localhost:8080}"
ACTIVE_MODEL="${ACTIVE_MODEL:-deepseek-chat}"
GPQA_DATASET_URL="${GPQA_DATASET_URL:-https://raw.githubusercontent.com/idavidrein/gpqa/main/dataset.zip}"
GPQA_DATASET_PASSWORD="${GPQA_DATASET_PASSWORD:-deserted-untie-orchid}"
GPQA_DATASET_SPLIT="${GPQA_DATASET_SPLIT:-gpqa_diamond.csv}"
GPQA_SAMPLE_INDEX="${GPQA_SAMPLE_INDEX:-0}"

if [[ -e "$RUN_DIR" ]]; then
    echo "ERROR: evidence directory already exists: $RUN_DIR" >&2
    exit 2
fi

if [[ -f "$PROJECT_ROOT/.env" ]]; then
    set -a
    # shellcheck disable=SC1091
    source "$PROJECT_ROOT/.env"
    set +a
elif [[ -f "/home/zephryj/projects/turingosv4/.env" ]]; then
    set -a
    # shellcheck disable=SC1091
    source "/home/zephryj/projects/turingosv4/.env"
    set +a
fi

if [[ -n "$(cd "$PROJECT_ROOT" && git status --porcelain | grep -vE '^\?\? handover/evidence/' | head -1)" ]]; then
    echo "ERROR: working tree has non-evidence changes; run /runner-preflight before evidence runners" >&2
    exit 3
fi

if ! curl -sS --max-time 5 "$LLM_PROXY_URL/health" | grep -q '"status": "ok"'; then
    echo "ERROR: LLM proxy $LLM_PROXY_URL/health not OK" >&2
    echo "Start one with: python3 src/drivers/llm_proxy.py --port 8080" >&2
    exit 4
fi

echo "[build] cargo build --release --bin turingos --bin verify_chaintape --bin gpqa_science_reasoning_current_kernel --bin full_system_augment_current_kernel --bin full_system_participation_current_kernel"
(cd "$PROJECT_ROOT" && cargo build --release --bin turingos --bin verify_chaintape --bin gpqa_science_reasoning_current_kernel --bin full_system_augment_current_kernel --bin full_system_participation_current_kernel)

TURINGOS="$PROJECT_ROOT/target/release/turingos"
HELPER="$PROJECT_ROOT/target/release/gpqa_science_reasoning_current_kernel"
AUGMENT="$PROJECT_ROOT/target/release/full_system_augment_current_kernel"
PARTICIPATION="$PROJECT_ROOT/target/release/full_system_participation_current_kernel"
SAMPLE_JSON="$RUN_DIR/input_capsules/gpqa_sample.json"

echo "[init] turingos init --project $RUN_DIR"
"$TURINGOS" init --project "$RUN_DIR" --template proof --provider deepseek
mkdir -p "$RUN_DIR/input_capsules"

if [[ -n "${GPQA_SAMPLE_JSON:-}" ]]; then
    cp "$GPQA_SAMPLE_JSON" "$SAMPLE_JSON"
else
    echo "[dataset] downloading public GPQA dataset sample"
    python3 - "$RUN_DIR/input_capsules" "$SAMPLE_JSON" "$GPQA_DATASET_URL" "$GPQA_DATASET_PASSWORD" "$GPQA_DATASET_SPLIT" "$GPQA_SAMPLE_INDEX" "$RUN_ID" <<'PY'
import csv
import hashlib
import json
import random
import sys
import urllib.request
import zipfile
from pathlib import Path

input_dir = Path(sys.argv[1])
sample_json = Path(sys.argv[2])
dataset_url = sys.argv[3]
password = sys.argv[4].encode("utf-8")
split = sys.argv[5]
sample_index = int(sys.argv[6])
run_id = sys.argv[7]

zip_path = input_dir / "gpqa_dataset.zip"
extract_dir = input_dir / "gpqa_dataset"
urllib.request.urlretrieve(dataset_url, zip_path)
with zipfile.ZipFile(zip_path) as zf:
    zf.extractall(extract_dir, pwd=password)

csv_candidates = sorted(extract_dir.rglob(split))
if not csv_candidates:
    raise SystemExit(f"GPQA split not found after extraction: {split}")
csv_path = csv_candidates[0]
with csv_path.open(newline="", encoding="utf-8") as fh:
    rows = list(csv.DictReader(fh))
if not rows:
    raise SystemExit(f"GPQA split has no rows: {csv_path}")
row = rows[sample_index % len(rows)]

correct = row["Correct Answer"].strip()
raw_choices = [
    ("correct", correct),
    ("incorrect_1", row["Incorrect Answer 1"].strip()),
    ("incorrect_2", row["Incorrect Answer 2"].strip()),
    ("incorrect_3", row["Incorrect Answer 3"].strip()),
]
seed = hashlib.sha256((row.get("Record ID", "") + run_id).encode("utf-8")).hexdigest()
rng = random.Random(seed)
rng.shuffle(raw_choices)
labels = ["A", "B", "C", "D"]
choices = {label: text for label, (_, text) in zip(labels, raw_choices)}
correct_choice = next(label for label, (kind, _) in zip(labels, raw_choices) if kind == "correct")
sample = {
    "schema_version": "turingosv4.true_suite.gpqa_sample.v1",
    "sample_id": row.get("Record ID") or f"{split}:{sample_index}",
    "source_family": "GPQA",
    "public_source": "https://github.com/idavidrein/gpqa",
    "source_file": split,
    "high_level_domain": row.get("High-level domain") or None,
    "subdomain": row.get("Subdomain") or None,
    "question": row["Question"].strip(),
    "choices": choices,
    "correct_choice": correct_choice,
    "correct_answer": correct,
    "canary_string": row.get("Canary String") or None,
}
sample_json.write_text(json.dumps(sample, indent=2, sort_keys=True) + "\n", encoding="utf-8")
PY
    rm -f "$RUN_DIR/input_capsules/gpqa_dataset.zip"
    rm -rf "$RUN_DIR/input_capsules/gpqa_dataset"
fi

echo "[gpqa] external LLM agent -> CAS answer claim -> signed WorkTx"
"$HELPER" \
    --runtime-repo "$RUN_DIR/runtime_repo" \
    --cas "$RUN_DIR/cas" \
    --run-id "$RUN_ID" \
    --constitution "$PROJECT_ROOT/constitution.md" \
    --sample-json "$SAMPLE_JSON" \
    --llm-proxy-url "$LLM_PROXY_URL" \
    --model "$ACTIVE_MODEL" \
    --out-dir "$RUN_DIR"

echo "[augment] append market + FC3 participation rows to the same ChainTape"
"$AUGMENT" \
    --runtime-repo "$RUN_DIR/runtime_repo" \
    --cas "$RUN_DIR/cas" \
    --run-id "$RUN_ID" \
    --constitution "$PROJECT_ROOT/constitution.md" \
    --out-dir "$RUN_DIR"

cp "$RUN_DIR/runtime_repo/genesis_report.json" "$RUN_DIR/genesis_report.json"

echo "[verify] turingos verify chaintape"
"$TURINGOS" verify chaintape \
    --repo "$RUN_DIR/runtime_repo" \
    --cas "$RUN_DIR/cas" \
    --run-id "$RUN_ID" \
    --out "$RUN_DIR/replay_report.json"

"$PARTICIPATION" \
    --run-id "$RUN_ID" \
    --family-id "gpqa_science_reasoning" \
    --entrypoint "scripts/run_true_suite_gpqa_science_reasoning_current_kernel.sh" \
    --runtime-repo "$RUN_DIR/runtime_repo" \
    --cas "$RUN_DIR/cas" \
    --replay-report "$RUN_DIR/replay_report.json" \
    --genesis-report "$RUN_DIR/genesis_report.json" \
    --domain-manifest "$RUN_DIR/gpqa_science_reasoning_manifest.json" \
    --fc3-index "$RUN_DIR/governance_capsule_index.json" \
    --require-full-system \
    --out "$RUN_DIR/full_system_participation.json"

cat > "$RUN_DIR/gpqa_science_reasoning_run_manifest.json" <<EOF
{
  "schema_version": "turingosv4.true_suite.gpqa_science_reasoning_run.v1",
  "run_id": "$RUN_ID",
  "git_head": "$(cd "$PROJECT_ROOT" && git rev-parse HEAD)",
  "entrypoint": "scripts/run_true_suite_gpqa_science_reasoning_current_kernel.sh",
  "llm_proxy_url": "$LLM_PROXY_URL",
  "active_model": "$ACTIVE_MODEL",
  "gpqa_dataset_url": "$GPQA_DATASET_URL",
  "gpqa_dataset_split": "$GPQA_DATASET_SPLIT",
  "gpqa_sample_index": $GPQA_SAMPLE_INDEX,
  "runtime_repo": "$RUN_DIR/runtime_repo",
  "cas": "$RUN_DIR/cas",
  "genesis_report": "$RUN_DIR/genesis_report.json",
  "gpqa_manifest": "$RUN_DIR/gpqa_science_reasoning_manifest.json",
  "failure_taxonomy": "$RUN_DIR/failure_taxonomy.json",
  "full_system_augmentation_manifest": "$RUN_DIR/full_system_augmentation_manifest.json",
  "governance_capsule_index": "$RUN_DIR/governance_capsule_index.json",
  "replay_report": "$RUN_DIR/replay_report.json",
  "full_system_participation": "$RUN_DIR/full_system_participation.json",
  "notes": [
    "GPQA data comes from the public idavidrein/gpqa source unless GPQA_SAMPLE_JSON is explicitly supplied",
    "DeepSeek/SiliconFlow access is outside the kernel through the local LLM proxy",
    "raw provider prompt and response are not written to evidence",
    "benchmark accuracy is not treated as liveness closure"
  ]
}
EOF

echo "TRUE-SUITE GPQA science-reasoning evidence: $RUN_DIR"
