#!/usr/bin/env bash
# True-suite MATH competition-reasoning current-kernel evidence runner.
#
# Uses a public MATH dataset sample plus a real external LLM through the local
# OpenAI-compatible proxy. The model answer is parsed into a structured claim,
# hashed into CAS, submitted as a signed WorkTx through current ChainTape, and
# replayed through public `turingos verify chaintape`.

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RUN_ID="${1:-math_competition_reasoning_$(date -u +%Y%m%dT%H%M%SZ)}"
RUN_ID="${RUN_ID#handover/evidence/true_suite/}"
RUN_ROOT="$PROJECT_ROOT/handover/evidence/true_suite/$RUN_ID"
RUN_DIR="$RUN_ROOT/math"
LLM_PROXY_URL="${LLM_PROXY_URL:-http://localhost:8080}"
ACTIVE_MODEL="${ACTIVE_MODEL:-deepseek-chat}"
MATH_DATASET_ROWS_URL="${MATH_DATASET_ROWS_URL:-https://datasets-server.huggingface.co/rows}"
MATH_SUBJECT="${MATH_SUBJECT:-algebra}"
MATH_SPLIT="${MATH_SPLIT:-test}"
MATH_SAMPLE_INDEX="${MATH_SAMPLE_INDEX:-0}"

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

echo "[build] cargo build --release --bin turingos --bin verify_chaintape --bin math_competition_reasoning_current_kernel"
(cd "$PROJECT_ROOT" && cargo build --release --bin turingos --bin verify_chaintape --bin math_competition_reasoning_current_kernel)

TURINGOS="$PROJECT_ROOT/target/release/turingos"
HELPER="$PROJECT_ROOT/target/release/math_competition_reasoning_current_kernel"
SAMPLE_JSON="$RUN_DIR/input_capsules/math_sample.json"

echo "[init] turingos init --project $RUN_DIR"
"$TURINGOS" init --project "$RUN_DIR" --template proof --provider deepseek
mkdir -p "$RUN_DIR/input_capsules"

if [[ -n "${MATH_SAMPLE_JSON:-}" ]]; then
    cp "$MATH_SAMPLE_JSON" "$SAMPLE_JSON"
else
    echo "[dataset] downloading public EleutherAI/Hendrycks MATH sample"
    python3 - "$RUN_DIR/input_capsules" "$SAMPLE_JSON" "$MATH_DATASET_ROWS_URL" "$MATH_SUBJECT" "$MATH_SPLIT" "$MATH_SAMPLE_INDEX" <<'PY'
import json
import re
import sys
import urllib.request
import urllib.parse
from pathlib import Path

input_dir = Path(sys.argv[1])
sample_json = Path(sys.argv[2])
rows_url = sys.argv[3]
subject = sys.argv[4]
split = sys.argv[5]
sample_index = int(sys.argv[6])

params = urllib.parse.urlencode({
    "dataset": "EleutherAI/hendrycks_math",
    "config": subject,
    "split": split,
    "offset": sample_index,
    "length": 1,
})
dataset_url = f"{rows_url}?{params}"
with urllib.request.urlopen(dataset_url, timeout=30) as response:
    payload = json.load(response)
rows = payload.get("rows") or []
if not rows:
    raise SystemExit(f"MATH rows API returned no rows: {dataset_url}")
row_idx = rows[0].get("row_idx", sample_index)
row = rows[0].get("row") or {}
source_file = f"datasets-server:{subject}/{split}:{row_idx}"

def boxed(solution: str) -> str:
    marker = r"\boxed{"
    start = solution.rfind(marker)
    if start == -1:
        match = re.search(r"\\boxed\s*\{([^{}]+)\}", solution)
        if match:
            return match.group(1).strip()
        raise SystemExit("MATH solution has no boxed final answer")
    i = start + len(marker)
    depth = 1
    for j, ch in enumerate(solution[i:], start=i):
        if ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0:
                return solution[i:j].strip()
    raise SystemExit("MATH boxed answer is unterminated")

problem = str(row.get("problem") or "").strip()
solution = str(row.get("solution") or "").strip()
if not problem:
    raise SystemExit("MATH row has empty problem")
if not solution:
    raise SystemExit("MATH row has empty solution")
sample = {
    "schema_version": "turingosv4.true_suite.math_sample.v1",
    "sample_id": f"EleutherAI/hendrycks_math:{subject}:{split}:{row_idx}",
    "source_family": "MATH",
    "public_source": "https://huggingface.co/datasets/EleutherAI/hendrycks_math",
    "source_file": source_file,
    "subject": str(row.get("type") or subject).strip(),
    "level": str(row.get("level") or "unknown").strip(),
    "problem": problem,
    "solution": solution,
    "expected_answer": boxed(solution),
    "canary_string": None,
}
sample_json.write_text(json.dumps(sample, indent=2, sort_keys=True) + "\n", encoding="utf-8")
PY
fi

echo "[math] external LLM agent -> CAS answer claim -> signed WorkTx"
"$HELPER" \
    --runtime-repo "$RUN_DIR/runtime_repo" \
    --cas "$RUN_DIR/cas" \
    --run-id "$RUN_ID" \
    --constitution "$PROJECT_ROOT/constitution.md" \
    --sample-json "$SAMPLE_JSON" \
    --llm-proxy-url "$LLM_PROXY_URL" \
    --model "$ACTIVE_MODEL" \
    --out-dir "$RUN_DIR"

cp "$RUN_DIR/runtime_repo/genesis_report.json" "$RUN_DIR/genesis_report.json"

echo "[verify] turingos verify chaintape"
"$TURINGOS" verify chaintape \
    --repo "$RUN_DIR/runtime_repo" \
    --cas "$RUN_DIR/cas" \
    --run-id "$RUN_ID" \
    --out "$RUN_DIR/replay_report.json"

cat > "$RUN_DIR/math_competition_reasoning_run_manifest.json" <<EOF
{
  "schema_version": "turingosv4.true_suite.math_competition_reasoning_run.v1",
  "run_id": "$RUN_ID",
  "git_head": "$(cd "$PROJECT_ROOT" && git rev-parse HEAD)",
  "entrypoint": "scripts/run_true_suite_math_competition_current_kernel.sh",
  "llm_proxy_url": "$LLM_PROXY_URL",
  "active_model": "$ACTIVE_MODEL",
  "math_dataset_rows_url": "$MATH_DATASET_ROWS_URL",
  "math_subject": "$MATH_SUBJECT",
  "math_split": "$MATH_SPLIT",
  "math_sample_index": $MATH_SAMPLE_INDEX,
  "runtime_repo": "$RUN_DIR/runtime_repo",
  "cas": "$RUN_DIR/cas",
  "genesis_report": "$RUN_DIR/genesis_report.json",
  "math_manifest": "$RUN_DIR/math_competition_reasoning_manifest.json",
  "failure_taxonomy": "$RUN_DIR/failure_taxonomy.json",
  "replay_report": "$RUN_DIR/replay_report.json",
  "notes": [
    "MATH data comes from the public EleutherAI/hendrycks_math parquet source unless MATH_SAMPLE_JSON is explicitly supplied",
    "DeepSeek/SiliconFlow access is outside the kernel through the local LLM proxy",
    "raw provider prompt and response are not written to evidence",
    "benchmark accuracy is not treated as liveness closure"
  ]
}
EOF

echo "TRUE-SUITE MATH competition-reasoning evidence: $RUN_DIR"
