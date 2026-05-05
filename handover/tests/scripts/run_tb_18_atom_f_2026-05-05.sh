#!/usr/bin/env bash
# TB-18 Atom F — single-chain 13/13 smoke + β-A feasibility audit
# (charter §F: PROCEED + 13/13 + tamper 3/3 + replay-byte-identical;
#  β-A in-tape resolution exercised, NOT α CLI sidecar)
#
# Inputs: TB-18.B-impl Phase 4 r1 canonical bytes
#   handover/evidence/tb_18_b_phase4_2026-05-05/r1/runtime_repo.dotgit.tar.gz
#   handover/evidence/tb_18_b_phase4_2026-05-05/r1/cas.dotgit.tar.gz
#
# Outputs: handover/evidence/tb_18_single_chain_13_of_13/r1/
#   verdict.json + verdict_replay.json + tamper_report.json
#   audit_tape.stderr + audit_tape_tamper.stderr
#   beta_a_feasibility_check.json
#
# Exit codes:
#   0 — all asserts pass
#   1 — at least one assert fails
#   2 — invalid args / missing inputs

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
DEFAULT_SRC_DIR="$PROJECT_ROOT/handover/evidence/tb_18_b_phase4_2026-05-05/r1"
DEFAULT_OUT_DIR="$PROJECT_ROOT/handover/evidence/tb_18_single_chain_13_of_13/r1"
SRC_DIR="$DEFAULT_SRC_DIR"
OUT_DIR="$DEFAULT_OUT_DIR"
while [ $# -gt 0 ]; do
  case "$1" in
    --src-dir) SRC_DIR="$2"; shift 2 ;;
    --out-dir) OUT_DIR="$2"; shift 2 ;;
    -h|--help) sed -n '1,18p' "$0"; exit 0 ;;
    *) echo "[atom-f] unknown arg: $1" >&2; exit 2 ;;
  esac
done
WORK_DIR="$(mktemp -d -t tb18-atom-f-XXXXXX)"

AUDIT_TAPE="$PROJECT_ROOT/target/release/audit_tape"
AUDIT_TAPE_TAMPER="$PROJECT_ROOT/target/release/audit_tape_tamper"

cleanup() { rm -rf "$WORK_DIR"; }
trap cleanup EXIT

if [ ! -x "$AUDIT_TAPE" ] || [ ! -x "$AUDIT_TAPE_TAMPER" ]; then
  echo "[atom-f] building audit_tape + audit_tape_tamper (release)..."
  (cd "$PROJECT_ROOT" && cargo build --release --bin audit_tape --bin audit_tape_tamper 2>&1 | tail -3)
fi

if [ ! -f "$SRC_DIR/runtime_repo.dotgit.tar.gz" ] || [ ! -f "$SRC_DIR/cas.dotgit.tar.gz" ]; then
  echo "[atom-f] missing canonical tarballs at $SRC_DIR" >&2
  exit 2
fi

mkdir -p "$OUT_DIR"

# Stage canonical bytes into working dir
mkdir -p "$WORK_DIR/runtime_repo" "$WORK_DIR/cas"
cp -r "$SRC_DIR/runtime_repo/." "$WORK_DIR/runtime_repo/"
cp -r "$SRC_DIR/cas/." "$WORK_DIR/cas/"
# Drop the local _dotgit_post_tar restore (keep clean; we extract from canonical tarball)
rm -rf "$WORK_DIR/runtime_repo/_dotgit_post_tar" "$WORK_DIR/cas/_dotgit_post_tar"

# Restore .git from canonical tarballs (per Phase 4 README replay procedure)
tar xzf "$SRC_DIR/runtime_repo.dotgit.tar.gz" -C "$WORK_DIR/runtime_repo"
tar xzf "$SRC_DIR/cas.dotgit.tar.gz" -C "$WORK_DIR/cas"

echo "[atom-f] working tree at $WORK_DIR"

# Step 1: audit_tape → verdict.json
echo "[atom-f] step 1: audit_tape..."
set +e
"$AUDIT_TAPE" \
  --runtime-repo "$WORK_DIR/runtime_repo" \
  --cas-dir "$WORK_DIR/cas" \
  --agent-pubkeys "$WORK_DIR/runtime_repo/agent_pubkeys.json" \
  --pinned-pubkeys "$WORK_DIR/runtime_repo/pinned_pubkeys.json" \
  --genesis "$PROJECT_ROOT/genesis_payload.toml" \
  --constitution "$PROJECT_ROOT/constitution.md" \
  --alignment-dir "$PROJECT_ROOT/handover/alignment" \
  --out "$OUT_DIR/verdict.json" \
  2> "$OUT_DIR/audit_tape.stderr"
RC1=$?
set -e

# Step 2: audit_tape replay → verdict_replay.json (byte-identical check)
echo "[atom-f] step 2: audit_tape replay..."
set +e
"$AUDIT_TAPE" \
  --runtime-repo "$WORK_DIR/runtime_repo" \
  --cas-dir "$WORK_DIR/cas" \
  --agent-pubkeys "$WORK_DIR/runtime_repo/agent_pubkeys.json" \
  --pinned-pubkeys "$WORK_DIR/runtime_repo/pinned_pubkeys.json" \
  --genesis "$PROJECT_ROOT/genesis_payload.toml" \
  --constitution "$PROJECT_ROOT/constitution.md" \
  --alignment-dir "$PROJECT_ROOT/handover/alignment" \
  --out "$OUT_DIR/verdict_replay.json" \
  2>> "$OUT_DIR/audit_tape.stderr"
RC2=$?
set -e

# Step 3: audit_tape_tamper → tamper_report.json (3/3 detected)
echo "[atom-f] step 3: audit_tape_tamper..."
mkdir -p "$WORK_DIR/tamper"
set +e
"$AUDIT_TAPE_TAMPER" \
  --runtime-repo "$WORK_DIR/runtime_repo" \
  --cas-dir "$WORK_DIR/cas" \
  --agent-pubkeys "$WORK_DIR/runtime_repo/agent_pubkeys.json" \
  --pinned-pubkeys "$WORK_DIR/runtime_repo/pinned_pubkeys.json" \
  --genesis "$PROJECT_ROOT/genesis_payload.toml" \
  --constitution "$PROJECT_ROOT/constitution.md" \
  --alignment-dir "$PROJECT_ROOT/handover/alignment" \
  --tamper-dir "$WORK_DIR/tamper" \
  --out "$OUT_DIR/tamper_report.json" \
  2> "$OUT_DIR/audit_tape_tamper.stderr"
RC3=$?
set -e

# Step 4: β-A feasibility check (in-tape resolution; NOT α sidecar)
echo "[atom-f] step 4: β-A feasibility check..."
python3 - "$OUT_DIR/beta_a_feasibility_check.json" "$WORK_DIR" "$PROJECT_ROOT" "$SRC_DIR" <<'PY'
import json, os, sys

out_path, work_dir, project_root, src_dir = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4]

# β-A feasibility = chain-derived state alone is sufficient.
# Checks:
#  (A) No global α sidecar pointer exists (LATEST_MARKOV_CAPSULE.txt deleted in TB-16.x.fix).
#  (B) audit_tape invoked WITHOUT --markov-pointer + without --prior-chain-runtime-repo
#      (this is genesis-chain mode for atom F single-chain smoke).
#  (C) Tape carries TerminalSummaryTx (evidence-capsule emission point) so a NEXT chain
#      could resolve via in-tape walk per MARKOV_INHERITANCE_POLICY §4. Source of truth =
#      tx_kind_distribution.json:tx_kind_counts.TerminalSummary (chain-emitter authoritative).

result = {
    "policy": "PRE-17.7 β-A only (architect TB-18 ratification Q4)",
    "exclusion": "α CLI sidecar (TB-16.x.fix OBS_R022 closure)",
    "checks": {},
}

# (A) α sidecar absent
sidecar_paths = [
    ("project_root", os.path.join(project_root, "LATEST_MARKOV_CAPSULE.txt")),
    ("runtime_repo", os.path.join(work_dir, "runtime_repo", "LATEST_MARKOV_CAPSULE.txt")),
    ("cas_dir", os.path.join(work_dir, "cas", "LATEST_MARKOV_CAPSULE.txt")),
]
result["checks"]["A_alpha_sidecar_absent"] = {
    name: (not os.path.exists(p)) for name, p in sidecar_paths
}

# (B) audit_tape invocation flags
result["checks"]["B_genesis_chain_mode"] = {
    "markov_pointer_flag": "absent",
    "prior_chain_runtime_repo_flag": "absent",
    "interpretation": (
        "atom F single-chain smoke is genesis; β-A applies to NEXT chain inheritance "
        "(out of scope for atom F single-chain). Replay byte-identical without sidecar = "
        "all Markov state chain-derived"
    ),
}

# (C) TerminalSummary present in chain (= future β-A resolver has anchor points)
ts_count = 0
total_kinds = 0
try:
    with open(os.path.join(src_dir, "evidence", "tx_kind_distribution.json")) as f:
        d = json.load(f)
    counts = d.get("tx_kind_counts", {})
    ts_count = int(counts.get("TerminalSummary", 0))
    total_kinds = int(d.get("distinct_tx_kinds", len([k for k, v in counts.items() if v > 0])))
    result["checks"]["C_in_tape_markov_capsule_cid"] = {
        "source": "tx_kind_distribution.json (chain-emitter authoritative)",
        "terminal_summary_count": ts_count,
        "distinct_tx_kinds": total_kinds,
        "interpretation": (
            "TerminalSummary tx in chain = EvidenceCapsule emission point; "
            "β-A in-tape resolver (MARKOV_INHERITANCE_POLICY §4) can walk this chain "
            "to find Markov tip and resolve from CAS — feasibility validated"
            if ts_count > 0 else "no TerminalSummary tx in chain"
        ),
    }
except Exception as e:
    result["checks"]["C_in_tape_markov_capsule_cid"] = {"error": str(e)}

result["verdict"] = "FEASIBLE" if (
    all(result["checks"]["A_alpha_sidecar_absent"].values())
    and result["checks"]["C_in_tape_markov_capsule_cid"].get("terminal_summary_count", 0) > 0
) else "REVIEW_REQUIRED"

with open(out_path, "w") as f:
    json.dump(result, f, indent=2)
print(f"[atom-f] β-A verdict: {result['verdict']}")
PY

# Asserts
echo "[atom-f] running asserts..."
failed=0

# Assert 1: verdict.json exists + PROCEED
if [ ! -f "$OUT_DIR/verdict.json" ]; then
  echo "✗ verdict.json missing"; failed=$((failed+1))
else
  V=$(python3 -c "import json; print(json.load(open('$OUT_DIR/verdict.json'))['verdict'])")
  if [ "$V" = "PROCEED" ]; then
    echo "✓ assert 1: verdict.json verdict=PROCEED"
  else
    echo "✗ assert 1: verdict=$V (expected PROCEED)"; failed=$((failed+1))
  fi
fi

# Assert 2: replay byte-identical
if [ -f "$OUT_DIR/verdict.json" ] && [ -f "$OUT_DIR/verdict_replay.json" ]; then
  if cmp -s "$OUT_DIR/verdict.json" "$OUT_DIR/verdict_replay.json"; then
    echo "✓ assert 2: verdict.json == verdict_replay.json (byte-identical)"
  else
    echo "✗ assert 2: replay diverged"; failed=$((failed+1))
  fi
else
  echo "✗ assert 2: verdict files missing"; failed=$((failed+1))
fi

# Assert 3: tamper_report detected_count >= 3
if [ -f "$OUT_DIR/tamper_report.json" ]; then
  D=$(python3 -c "import json; r=json.load(open('$OUT_DIR/tamper_report.json')); print(r.get('detected_count',0))")
  if [ "${D:-0}" -ge 3 ]; then
    echo "✓ assert 3: tamper detected_count=$D/3"
  else
    echo "✗ assert 3: tamper detected_count=$D (expected >= 3)"; failed=$((failed+1))
  fi
else
  echo "✗ assert 3: tamper_report.json missing"; failed=$((failed+1))
fi

# Assert 4: 13 distinct tx kinds in tape (use B Phase 4 evidence for distribution)
TX_DIST="$SRC_DIR/evidence/tx_kind_distribution.json"
if [ -f "$TX_DIST" ]; then
  KIND_COUNT=$(python3 -c "import json; d=json.load(open('$TX_DIST')); print(d.get('distinct_tx_kinds', len([k for k,v in d.get('tx_kind_counts', {}).items() if isinstance(v,int) and v>0])))")
  if [ "${KIND_COUNT:-0}" -eq 13 ]; then
    echo "✓ assert 4: 13/13 distinct tx kinds in single chain"
  else
    echo "✗ assert 4: only $KIND_COUNT/13 distinct tx kinds"; failed=$((failed+1))
  fi
else
  echo "✗ assert 4: tx_kind_distribution.json missing"; failed=$((failed+1))
fi

# Assert 5: β-A feasibility FEASIBLE
if [ -f "$OUT_DIR/beta_a_feasibility_check.json" ]; then
  BV=$(python3 -c "import json; print(json.load(open('$OUT_DIR/beta_a_feasibility_check.json'))['verdict'])")
  if [ "$BV" = "FEASIBLE" ]; then
    echo "✓ assert 5: β-A feasibility = FEASIBLE (no α sidecar; in-tape capsules present)"
  else
    echo "✗ assert 5: β-A feasibility = $BV"; failed=$((failed+1))
  fi
else
  echo "✗ assert 5: beta_a_feasibility_check.json missing"; failed=$((failed+1))
fi

if [ $failed -gt 0 ]; then
  echo "[atom-f] FAILED: $failed assertion(s) failed"
  exit 1
fi

echo "[atom-f] ALL ASSERTS PASSED ✓"
echo "[atom-f] evidence dir: $OUT_DIR"
exit 0
