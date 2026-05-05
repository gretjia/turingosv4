#!/usr/bin/env bash
# TB-18 Atom H M1 — Full heldout subset benchmark batch
# (architect §B.9.3 verbatim M1 phase: 50-100 problems / n1 + n3 /
#  all failures produce EvidenceCapsule / dashboard batch report)
#
# This is a WRAPPER around run_m0_minif2f_harness with M1 parameters:
#   - 50 problems (experiments/minif2f_v4/combined50.txt)
#   - n=1 (single-agent run_swarm path)
#   - max_tx=20 + per-LLM-call budget (atom A)
#   - 600s per-problem timeout (architect spec; loosened from M0 retry's 120s)
#   - chain-backed (TURINGOS_CHAINTAPE_PATH + preseed)
#   - no market hooks; no Boltzmann
#
# **PRECONDITIONS** (script enforces; refuses to run if missing):
#   1. handover/audits/CODEX_MICRO_AUDIT_TB_18_PRE_H_VERDICT_*.md exists
#      with verdict containing "PASS" OR "CHALLENGE-resolved" (NOT "VETO")
#   2. environment variable TB18_M1_USER_AUTH_GO=1 (explicit user "go")
#
# **NEVER RUN** without G0 PASS + user authorization. Per charter §2 amendment
# 2026-05-05: atom H requires explicit user "go" (G0 PASS necessary but not
# sufficient).
#
# Estimated wall-clock envelope: 50 problems × max(120s avg, 600s upper) ≈
# 100 min..8.3 hours sequential. Budget: $US per-problem TBD per
# DeepSeek pricing × 50.
#
# Usage:
#   TB18_M1_USER_AUTH_GO=1 \
#   bash handover/tests/scripts/run_tb_18_atom_h_m1_2026-05-05.sh \
#        [--out-dir <path>] \
#        [--manifest-path <path>] \
#        [--skip-build]
#
# Exit codes:
#   0 — all 50 chains audited (any verdict)
#   1 — script-level setup/build failure
#   2 — invalid args / preconditions / G0 gate / user-auth gate

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
SCRIPT_DIR="$PROJECT_ROOT/handover/tests/scripts"
DEFAULT_OUT_DIR="$PROJECT_ROOT/handover/evidence/tb_18_minif2f_m1_$(date -u +%Y-%m-%dT%H-%M-%SZ)"
DEFAULT_MANIFEST_PATH="$PROJECT_ROOT/handover/manifests/TB-18_M1_BENCHMARK_MANIFEST.json"
DEFAULT_PROBLEMS_FILE="$PROJECT_ROOT/experiments/minif2f_v4/combined50.txt"
DEFAULT_M0_RUNNER="$SCRIPT_DIR/run_m0_minif2f_harness_2026-05-05.sh"

OUT_DIR="$DEFAULT_OUT_DIR"
MANIFEST_PATH="$DEFAULT_MANIFEST_PATH"
SKIP_BUILD=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    --out-dir) OUT_DIR="$2"; shift 2 ;;
    --manifest-path) MANIFEST_PATH="$2"; shift 2 ;;
    --skip-build) SKIP_BUILD=1; shift ;;
    -h|--help)
      grep -E "^# " "$0" | sed 's/^# //'; exit 0 ;;
    *) echo "[m1] unknown arg: $1" >&2; exit 2 ;;
  esac
done

# Gate 1: G0 verdict file present + PASS or CHALLENGE-resolved
echo "[m1] gate 1: G0 verdict check..."
G0_VERDICT_GLOB=("$PROJECT_ROOT"/handover/audits/CODEX_MICRO_AUDIT_TB_18_PRE_H_VERDICT_*.md)
if [ ! -e "${G0_VERDICT_GLOB[0]}" ]; then
  echo "[m1] BLOCKED: no G0 verdict file matching CODEX_MICRO_AUDIT_TB_18_PRE_H_VERDICT_*.md" >&2
  echo "[m1] G0 trigger artifact: handover/audits/CODEX_MICRO_AUDIT_TB_18_PRE_H_REQUEST_2026-05-05.md" >&2
  echo "[m1] User must run /ultrareview or external Codex audit; verdict file MUST land before M1." >&2
  exit 2
fi

G0_VERDICT_FILE="${G0_VERDICT_GLOB[0]}"
echo "[m1]   verdict file: $G0_VERDICT_FILE"

if grep -qiE "OVERALL[: ]+VETO|verdict[: ]+VETO" "$G0_VERDICT_FILE"; then
  echo "[m1] BLOCKED: G0 verdict contains VETO. Resolve before M1." >&2
  exit 2
fi

if ! grep -qiE "OVERALL[: ]+PASS|OVERALL[: ]+CHALLENGE|PASS|CHALLENGE.{0,40}resolv" "$G0_VERDICT_FILE"; then
  echo "[m1] BLOCKED: G0 verdict does not contain PASS or CHALLENGE-resolved marker. Inspect: $G0_VERDICT_FILE" >&2
  exit 2
fi

echo "[m1]   G0 verdict gate: OPEN ✓"

# Gate 2: explicit user "go"
echo "[m1] gate 2: user authorization check..."
if [ "${TB18_M1_USER_AUTH_GO:-0}" != "1" ]; then
  echo "[m1] BLOCKED: TB18_M1_USER_AUTH_GO=1 not set." >&2
  echo "[m1] Per charter §2 amendment 2026-05-05: atom H requires explicit user 'go'." >&2
  echo "[m1] Re-invoke with: TB18_M1_USER_AUTH_GO=1 bash $0 ..." >&2
  exit 2
fi
echo "[m1]   user authorization gate: OPEN ✓"

# Gate 3: manifest path exists
echo "[m1] gate 3: manifest check..."
if [ ! -f "$MANIFEST_PATH" ]; then
  echo "[m1] BLOCKED: manifest not found at $MANIFEST_PATH" >&2
  exit 2
fi

# Compute manifest_id (sha256 over canonical JSON minus the manifest_id field itself)
MANIFEST_ID=$(python3 -c "
import hashlib, json
with open('$MANIFEST_PATH') as f:
    d = json.load(f)
d.pop('manifest_id', None)
canon = json.dumps(d, sort_keys=True, separators=(',', ':')).encode()
print(hashlib.sha256(canon).hexdigest())
")
echo "[m1]   manifest_id: $MANIFEST_ID"

# Gate 4: turingosv4 HEAD matches manifest commit (drift detection)
echo "[m1] gate 4: turingosv4 commit drift check..."
HEAD_SHA=$(git -C "$PROJECT_ROOT" rev-parse --short HEAD)
MANIFEST_COMMIT=$(python3 -c "import json; print(json.load(open('$MANIFEST_PATH'))['turingosv4_commit'])")
if [ "$HEAD_SHA" != "$MANIFEST_COMMIT" ]; then
  echo "[m1] BLOCKED: HEAD ($HEAD_SHA) differs from manifest turingosv4_commit ($MANIFEST_COMMIT)." >&2
  echo "[m1] Per anti_drift_contract: re-freeze manifest at current HEAD before M1." >&2
  exit 2
fi
echo "[m1]   commit drift gate: HEAD == manifest ✓"

# Gate 5: M0 runner present
if [ ! -f "$DEFAULT_M0_RUNNER" ]; then
  echo "[m1] BLOCKED: M0 runner not found at $DEFAULT_M0_RUNNER" >&2
  exit 2
fi

# Gate 6: problems file present + matches manifest count
PROBLEM_COUNT=$(wc -l < "$DEFAULT_PROBLEMS_FILE")
if [ "$PROBLEM_COUNT" -ne 50 ]; then
  echo "[m1] BLOCKED: $DEFAULT_PROBLEMS_FILE has $PROBLEM_COUNT problems (expected 50)" >&2
  exit 2
fi

# Stage manifest copy + frozen problems file in OUT_DIR
mkdir -p "$OUT_DIR"
cp "$MANIFEST_PATH" "$OUT_DIR/M1_RUN_MANIFEST.json"
cp "$DEFAULT_PROBLEMS_FILE" "$OUT_DIR/m1_problems.txt"
PROBLEMS_SHA256=$(sha256sum "$OUT_DIR/m1_problems.txt" | awk '{print $1}')
python3 -c "
import json
with open('$OUT_DIR/M1_RUN_MANIFEST.json') as f: d = json.load(f)
d['manifest_id'] = '$MANIFEST_ID'
d['frozen_problems_sha256'] = '$PROBLEMS_SHA256'
d['frozen_at_invocation_utc'] = '$(date -u +%Y-%m-%dT%H:%M:%SZ)'
d['head_at_invocation'] = '$HEAD_SHA'
with open('$OUT_DIR/M1_RUN_MANIFEST.json', 'w') as f: json.dump(d, f, indent=2)
"
echo "[m1] frozen manifest at $OUT_DIR/M1_RUN_MANIFEST.json"

# Delegate to M0 runner with M1 params
EXTRA_ARGS=()
[ "$SKIP_BUILD" -eq 1 ] && EXTRA_ARGS+=(--skip-build)

echo "[m1] launching M0 runner with M1 parameters (50 problems × n=1 × max_tx=20 × 600s)..."
echo "[m1]   out_dir = $OUT_DIR"
echo "[m1]   problems = $DEFAULT_PROBLEMS_FILE (50)"
echo "[m1]   estimated wall-clock: 1.5-8 hours sequential"

bash "$DEFAULT_M0_RUNNER" \
  --out-dir "$OUT_DIR" \
  --problems-file "$DEFAULT_PROBLEMS_FILE" \
  --max-tx 20 \
  --per-problem-timeout-s 600 \
  "${EXTRA_ARGS[@]}"

RC=$?

echo "[m1] M0 runner exited rc=$RC"
echo "[m1] evidence dir: $OUT_DIR"
echo "[m1] Next steps:"
echo "[m1]   1. Run packaging script (TBD; mirror tb_18_package_m0_evidence.sh for sampled M1)"
echo "[m1]   2. Generate MINIF2F_M1_BENCHMARK_REPORT.md aggregating PPUT/halt-reason/reputation distributions"
echo "[m1]   3. Stage atom H sub-stage 1 commit"
echo "[m1]   4. Begin M2 manifest draft (n5 / 100+ problems)"

exit $RC
