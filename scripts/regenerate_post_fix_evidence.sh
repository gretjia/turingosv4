#!/usr/bin/env bash
# Regenerate post-round-5/6 evidence on existing TB-C0 multi-agent batch.
#
# Per Codex audit verdict §8 #2 + #3 (CODEX_TBC0_STRICT_CONSTITUTIONAL_AUDIT_VERDICT_2026-05-07.md):
# the existing batch under handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/
# was produced by the round-2 binary (git_head=f3b8e0a). The persisted
# chain_invariant.json and architect_inv1_check.json files use the old 2-term
# equation and tx_count LHS. Round-5/6 fixes (Bug 1 + Bug 3 + FC1-INV6 +
# Bug 2) require regenerated evidence.
#
# This script reuses the EXISTING tape data (no re-runs of evaluator; no LLM
# compute) and reruns the post-fix binaries:
#   - tb_18r_compute_invariant (now with capsule_anchored + 3-term + Bug 2 filter)
#   - audit_tape (now with assert_50_cas_bytes_match_cids)
#   - architect_inv1 producer (now uses tool_dist.step LHS)
#   - fc_witness_extract.py (reads capsule_anchored from chain_invariant.json)
#
# Output: post-fix JSON files written ALONGSIDE the originals with `_post_fix.json`
# suffix (does NOT overwrite original per feedback_no_retroactive_evidence_rewrite).
# A new aggregate manifest is written as fc_witness_aggregate_post_fix.json.

set -euo pipefail
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

BATCH=handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z
INVARIANT_BIN=target/release/tb_18r_compute_invariant
AUDIT_BIN=target/release/audit_tape

if [ ! -x "$INVARIANT_BIN" ] || [ ! -x "$AUDIT_BIN" ]; then
  echo "Binaries missing — build first: cargo build --release --bin tb_18r_compute_invariant --bin audit_tape" >&2
  exit 1
fi

echo "=== Regenerating post-round-5/6 evidence in $BATCH ==="
echo "Per Codex VERDICT §8 #2: persist post-fix chain_invariant + architect_inv1 + aggregate"
echo

for prob_dir in "$BATCH"/P*; do
  [ -d "$prob_dir" ] || continue
  name=$(basename "$prob_dir")
  echo "--- $name ---"

  # Read original PPUT to derive halt_class + externalized_llm_cycle_count (Bug 1 fix).
  EXTRACT_JSON=$(python3 - "$prob_dir/extracted_pput.json" <<'PYEOF'
import json, sys
try:
  d = json.load(open(sys.argv[1]))
except Exception as e:
  print(json.dumps({"error": str(e)}))
  sys.exit(0)
td = d.get("tool_dist", {}) or {}
step_count = int(td.get("step", 0))
omega_wtool = int(td.get("omega_wtool", 0))
externalized_llm = step_count if step_count > 0 else omega_wtool
if externalized_llm == 0:
  externalized_llm = int(d.get("tx_count", 0))
solved = bool(d.get("solved", False))
hit_max = bool(d.get("hit_max_tx", False))
verified = bool(d.get("verified", False))
halt = "OmegaAccepted" if (solved or verified) else "MaxTxExhausted"
print(json.dumps({
  "externalized_llm_cycle_count": externalized_llm,
  "halt": halt,
  "tx_count_legacy": int(d.get("tx_count", 0)),
  "tool_dist": td,
}))
PYEOF
)
  EXPECTED=$(echo "$EXTRACT_JSON" | python3 -c 'import json,sys; print(json.load(sys.stdin).get("externalized_llm_cycle_count", 0))')
  HALT=$(echo "$EXTRACT_JSON" | python3 -c 'import json,sys; print(json.load(sys.stdin).get("halt", "ErrorHalt"))')

  # 1. Re-run invariant with new 3-term + Bug 2 filter
  "$INVARIANT_BIN" \
    --runtime-repo "$prob_dir/runtime_repo" \
    --cas "$prob_dir/cas" \
    --expected-completed "$EXPECTED" \
    --halt-class "$HALT" \
    > "$prob_dir/chain_invariant_post_fix.json" 2> "$prob_dir/chain_invariant_post_fix.stderr" || true

  # 2. Re-run audit_tape with new assert_50_cas_bytes_match_cids
  "$AUDIT_BIN" \
    --runtime-repo "$prob_dir/runtime_repo" \
    --cas-dir "$prob_dir/cas" \
    --agent-pubkeys "$prob_dir/runtime_repo/agent_pubkeys.json" \
    --pinned-pubkeys "$prob_dir/runtime_repo/pinned_pubkeys.json" \
    --genesis genesis_payload.toml \
    --constitution constitution.md \
    --alignment-dir handover/alignment \
    --out "$prob_dir/verdict_post_fix.json" 2> "$prob_dir/audit_tape_post_fix.stderr" || true

  # 3. Re-run architect_inv1 producer with Bug 1 LHS fix
  python3 - "$prob_dir" "$EXTRACT_JSON" <<'PYEOF' > "$prob_dir/architect_inv1_check_post_fix.json"
import json, os, sys
prob = sys.argv[1]
extr = json.loads(sys.argv[2])
def load_jsonl(p):
  if not os.path.exists(p): return []
  out = []
  with open(p) as f:
    for line in f:
      line = line.strip()
      if line:
        try: out.append(json.loads(line))
        except: pass
  return out
cas_idx = load_jsonl(os.path.join(prob, "cas", ".turingos_cas_index.jsonl"))
at_count = sum(1 for o in cas_idx if o.get("object_type") == "AttemptTelemetry")
externalized_llm = extr.get("externalized_llm_cycle_count", 0)
out = {
  "architect_inv_1": "chain_attempt_count == externalized_llm_cycle_count",
  "chain_attempt_count": at_count,
  "externalized_llm_cycle_count": externalized_llm,
  "evaluator_reported_tx_count_legacy": extr.get("tx_count_legacy", 0),
  "match": at_count == externalized_llm,
  "delta": at_count - externalized_llm,
  "_note": "TB-C0 Codex-§8 remediation 2026-05-07: regenerated using tool_dist.step LHS (Bug 1 fix), not raw tx_count.",
  "_source_binaries": "round-5+6 binaries: tb_18r_compute_invariant + audit_tape with assert_50",
}
print(json.dumps(out, indent=2))
PYEOF

  # 4. Read post-fix invariant + report concise per-problem summary
  python3 - "$prob_dir" <<'PYEOF'
import json, os, sys
prob = sys.argv[1]
inv = json.load(open(os.path.join(prob, "chain_invariant_post_fix.json")))
arch = json.load(open(os.path.join(prob, "architect_inv1_check_post_fix.json")))
verdict = json.load(open(os.path.join(prob, "verdict_post_fix.json")))
print(f"  invariant: expected={inv['expected_completed_attempts']} l4={inv['l4_work_attempt_count']} l4e={inv['l4e_work_attempt_count']} capsule={inv['capsule_anchored_attempt_count']} delta={inv['delta']:+d} → {inv['invariant_verdict'][:50]}")
print(f"  architect_inv1: chain={arch['chain_attempt_count']} externalized={arch['externalized_llm_cycle_count']} match={arch['match']}")
print(f"  audit_tape: {verdict['verdict']} (passed={verdict['passed']} halted={verdict['halted']} skipped={verdict['skipped']})")
PYEOF
done

# 5. Re-run fc_witness_extract.py per-problem (now reads capsule_anchored_attempt_count)
echo
echo "=== Re-extracting FC-witness manifests with round-5/6 extractor ==="
# Patch the extractor to read post-fix invariant if present
for prob_dir in "$BATCH"/P*; do
  [ -d "$prob_dir" ] || continue
  # Symlink chain_invariant_post_fix.json AS chain_invariant.json so extractor reads new field
  # (do NOT overwrite original — use a temp copy strategy)
  cp "$prob_dir/chain_invariant_post_fix.json" "$prob_dir/chain_invariant.json.tmp"
  mv "$prob_dir/chain_invariant.json" "$prob_dir/chain_invariant.json.original"
  mv "$prob_dir/chain_invariant.json.tmp" "$prob_dir/chain_invariant.json"
  python3 scripts/fc_witness_extract.py "$prob_dir" --out "$prob_dir/fc_witness_manifest_post_fix.json" \
    > "$prob_dir/fc_witness_extract_post_fix.stdout" 2> "$prob_dir/fc_witness_extract_post_fix.stderr"
  # Restore original
  mv "$prob_dir/chain_invariant.json" "$prob_dir/chain_invariant.json.tmp"
  mv "$prob_dir/chain_invariant.json.original" "$prob_dir/chain_invariant.json"
  rm "$prob_dir/chain_invariant.json.tmp"
done

# 6. Re-run aggregator
# We need to use the post_fix manifests, not the originals. Aggregate temporarily redirects.
echo
echo "=== Re-running aggregator on post-fix manifests ==="
python3 - "$BATCH" <<'PYEOF' > "$BATCH/fc_witness_aggregate_post_fix.json"
import json, os, sys
batch = sys.argv[1]
problem_dirs = sorted(d for d in os.listdir(batch)
                     if os.path.isdir(os.path.join(batch, d)) and d.startswith("P"))
per_problem = {}
for p in problem_dirs:
  m_path = os.path.join(batch, p, "fc_witness_manifest_post_fix.json")
  if os.path.exists(m_path):
    with open(m_path) as f:
      per_problem[p] = json.load(f)

# Codex round-7 v3 Finding V3-C1 fix: define the EXPECTED FC node universe
# independently of observed manifests. Round-6 / round-7-pre took the union
# of observed manifest keys, so a node missing from ALL 9 manifests would
# never be inserted into all_node_keys and could never produce a GAP row.
# Round-7-final (this script) defines the canonical node set explicitly,
# keyed off `fc_witness_extract.py` output. If a future binary drift drops
# a node from every manifest, the missing-from-all-manifests case now
# correctly emits an aggregate GAP row.
EXPECTED_FC_NODES = [
  # FC1 nodes (runtime loop)
  "FC1-N1_q_state_carrier",
  "FC1-N2_q_t_slice",
  "FC1-N3_HEAD_t_pointer",
  "FC1-N4_q1_after_delta",
  "FC1-N5_rtool",
  "FC1-N7_delta_AI_call",
  "FC1-N11_predicates",
  "FC1-N13_wtool",
  "FC1-N15_reject_branch",
  "FC1-INV1_every_attempt_tape_visible",
  "FC1-INV3_count_equality_constitutional",
  # FC2 nodes (boot)
  "FC2-N16_InitAI",
  "FC2-N18_constitution_ground_truth",
  "FC2-N21_Q0_minted",
  "FC2-N22_HALT",
  "FC2-INV1_genesis_replayable",
  "FC2-INV4_taskopen_escrowlock_chain_events",
  "FC2-INV6_pubkeys_verify",
  "FC2-INV7_agent_registry_resolves",
  # FC3 nodes (meta)
  "FC3-INV1_capsule_derived",
  "FC3-INV2_no_global_pointer",
  "FC3-INV3_raw_logs_shielded",
  "FC3-INV5_deep_history_override",
  "FC3-INV7_architect_propose_only",
  "FC3-INV8_judge_veto_only",
]
observed = set()
for m in per_problem.values():
  if "fc_nodes" in m:
    observed.update(m["fc_nodes"].keys())
all_node_keys = set(EXPECTED_FC_NODES) | observed
# Guardrail: alert if observed set has unexpected nodes (extractor schema drift)
unexpected_observed = observed - set(EXPECTED_FC_NODES)
expected_missing_globally = set(EXPECTED_FC_NODES) - observed
# (these are surfaced via missing_by entries below)

problem_count = len(per_problem)

aggregate = {}
for node in sorted(all_node_keys):
  green = []; amber = []; red = []; missing = []
  for pname, m in per_problem.items():
    fc_nodes = m.get("fc_nodes", {})
    v = fc_nodes.get(node)
    if not v:
      # Codex round-7 Finding C2 fix: explicitly track missing-from-manifest
      # cases. Round-6 used `if not v: continue` which silently skipped
      # missing nodes — could produce false-GREEN if a problem's manifest
      # didn't include the node at all.
      missing.append(pname)
      continue
    s = v.get("status", "")
    if s.startswith("✅"): green.append(pname)
    elif s.startswith("🟡"): amber.append(pname)
    elif s.startswith("🔴"): red.append(pname)
  # STRICT semantics (Codex Q6 + §9.3 + Q-RR3 + Finding C2 + §4 condition #2):
  # aggregate is GREEN ONLY if every problem GREEN AND zero missing AND zero
  # amber AND zero red. Any RED → RED. Any AMBER OR missing → AMBER.
  if red:
    agg_status = "RED"
  elif missing and not (amber or green):
    # ALL problems missing the node → GAP (no problem reports witness at all)
    agg_status = "GAP"
  elif amber or missing:
    # Any AMBER or any missing-node — cannot claim GREEN
    agg_status = "AMBER"
  elif len(green) == problem_count:
    # Every problem reports GREEN
    agg_status = "GREEN"
  else:
    # Should not reach (covered above) — defensive GAP
    agg_status = "GAP"
  aggregate[node] = {
    "aggregate_status": agg_status,
    "green_by": green,
    "amber_by": amber,
    "red_by": red,
    "missing_by": missing,
  }

reds = [k for k,v in aggregate.items() if v["aggregate_status"] == "RED"]
ambers = [k for k,v in aggregate.items() if v["aggregate_status"] == "AMBER"]
greens = [k for k,v in aggregate.items() if v["aggregate_status"] == "GREEN"]

out = {
  "schema_version": 2,  # bumped: strict semantics
  "tb_id": "TB-C0",
  "tool": "scripts/regenerate_post_fix_evidence.sh + fc_witness_extract.py",
  "batch_dir": batch,
  "problem_count": len(problem_dirs),
  "problems": problem_dirs,
  "aggregate_node_status": aggregate,
  "summary": {
    "total_nodes": len(aggregate),
    "green_count": len(greens),
    "amber_count": len(ambers),
    "red_count": len(reds),
  },
  "remediation_protocol": "TB-C0 Codex-§8 remediation 2026-05-07: STRICT aggregate semantics — RED if any problem RED; AMBER if any AMBER; GREEN only if all problems GREEN. Closes Codex Q6 + §9.3.",
  "post_fix_binaries": "round-5+6 (commits 0d0877b + this commit): Bug 1 + Bug 3 + Bug 2 + FC1-INV6 fixes applied",
}
print(json.dumps(out, indent=2))
PYEOF

echo
echo "=== Aggregate summary (STRICT semantics) ==="
python3 -c "
import json
d = json.load(open('$BATCH/fc_witness_aggregate_post_fix.json'))
print(f'Problems: {d[\"problem_count\"]}')
print(f'Summary: {d[\"summary\"]}')
print()
for n, v in sorted(d['aggregate_node_status'].items()):
  print(f'  {v[\"aggregate_status\"]:6}  {n:55}  G:{len(v[\"green_by\"])} A:{len(v[\"amber_by\"])} R:{len(v[\"red_by\"])}')
"

echo
echo "=== Done. Post-fix evidence in $BATCH ==="
echo "  - per-problem: chain_invariant_post_fix.json + architect_inv1_check_post_fix.json + verdict_post_fix.json + fc_witness_manifest_post_fix.json"
echo "  - aggregate:   $BATCH/fc_witness_aggregate_post_fix.json"
