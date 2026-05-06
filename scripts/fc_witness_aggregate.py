#!/usr/bin/env python3
"""
TB-C0 FC-witness aggregator.

Runs `scripts/fc_witness_extract.py` across every problem subdirectory in a
batch (e.g., a Phase-3 evidence dir with P01..P07) and emits an aggregate
FC-witness manifest. For each FC node enumerated in `TRACE_FLOWCHART_MATRIX.md`,
reports:

  - GREEN_BY:  list of problems that produced a green witness
  - AMBER_BY:  list of problems with partial/structural witness
  - RED_BY:    list of problems that exhibit a code bug for this node
  - GAP:       FC nodes that NO problem witnesses (need a different test problem)

Per `feedback_real_problems_not_designed`: for any GAP node, the remediation
is to FIND a real existing problem (MiniF2F / Mathlib / Putnam / IMO / research-
paper / web research) that exercises the path — not to synthesize one.

Usage:
    python3 scripts/fc_witness_aggregate.py <batch_dir> [--out <manifest>]

Where <batch_dir> is the parent of P01_*/P02_*/... like:
    handover/evidence/tb_18r_phase_3_2026-05-06T14-13-55Z/
"""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from pathlib import Path


def main() -> int:
    p = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    p.add_argument("batch_dir", type=Path)
    p.add_argument(
        "--out",
        type=Path,
        default=None,
        help="aggregate manifest path (default: <batch_dir>/fc_witness_aggregate.json)",
    )
    args = p.parse_args()

    if not args.batch_dir.exists():
        print(f"ERROR: {args.batch_dir} does not exist", file=sys.stderr)
        return 2

    problem_dirs = sorted(
        d for d in args.batch_dir.iterdir() if d.is_dir() and d.name.startswith("P")
    )
    if not problem_dirs:
        print(f"ERROR: no P*/ subdirectories in {args.batch_dir}", file=sys.stderr)
        return 2

    extractor = Path(__file__).parent / "fc_witness_extract.py"
    per_problem: dict[str, dict] = {}

    for pd in problem_dirs:
        # Run extractor; capture manifest
        result = subprocess.run(
            ["python3", str(extractor), str(pd)], capture_output=True, text=True
        )
        manifest_path = pd / "fc_witness_manifest.json"
        if manifest_path.exists():
            with manifest_path.open() as f:
                per_problem[pd.name] = json.load(f)
        else:
            per_problem[pd.name] = {"error": "no manifest produced", "stderr": result.stderr[:500]}

    # Aggregate per-FC-node status across problems
    all_node_keys: set[str] = set()
    for pname, m in per_problem.items():
        if "fc_nodes" in m:
            all_node_keys.update(m["fc_nodes"].keys())

    aggregate: dict[str, dict] = {}
    for node in sorted(all_node_keys):
        green_by: list[str] = []
        amber_by: list[str] = []
        red_by: list[str] = []
        for pname, m in per_problem.items():
            if "fc_nodes" not in m:
                continue
            v = m["fc_nodes"].get(node)
            if not v:
                continue
            s = v.get("status", "")
            if s.startswith("✅"):
                green_by.append(pname)
            elif s.startswith("🟡"):
                amber_by.append(pname)
            elif s.startswith("🔴"):
                red_by.append(pname)
        if green_by:
            agg_status = "GREEN"
        elif amber_by and not red_by:
            agg_status = "AMBER"
        elif red_by:
            agg_status = "RED"
        else:
            agg_status = "GAP"
        aggregate[node] = {
            "aggregate_status": agg_status,
            "green_by": green_by,
            "amber_by": amber_by,
            "red_by": red_by,
        }

    # Gap analysis
    gaps = [k for k, v in aggregate.items() if v["aggregate_status"] in ("GAP",)]
    reds = [k for k, v in aggregate.items() if v["aggregate_status"] == "RED"]
    ambers_no_green = [
        k for k, v in aggregate.items() if v["aggregate_status"] == "AMBER" and not v["green_by"]
    ]

    out = {
        "schema_version": 1,
        "tb_id": "TB-C0",
        "tool": "scripts/fc_witness_aggregate.py",
        "batch_dir": str(args.batch_dir),
        "problem_count": len(problem_dirs),
        "problems": [pd.name for pd in problem_dirs],
        "aggregate_node_status": aggregate,
        "summary": {
            "total_nodes": len(aggregate),
            "green_count": sum(1 for v in aggregate.values() if v["aggregate_status"] == "GREEN"),
            "amber_count": sum(1 for v in aggregate.values() if v["aggregate_status"] == "AMBER"),
            "red_count": sum(1 for v in aggregate.values() if v["aggregate_status"] == "RED"),
            "gap_count": len(gaps),
        },
        "gaps_for_remediation": gaps,
        "red_nodes_indicating_code_bug": reds,
        "ambers_lacking_any_green_witness": ambers_no_green,
        "remediation_protocol": (
            "Per feedback_real_problems_not_designed: for any GAP / persistent AMBER node, "
            "find a REAL existing problem (MiniF2F / Mathlib / Putnam / IMO / research-paper / "
            "web research) that exercises the path; do NOT synthesize. For RED nodes, escalate "
            "the code bug per OBS_TBC0_FC1_INV3_THREE_BUGS_2026-05-06.md."
        ),
    }

    out_path = args.out or args.batch_dir / "fc_witness_aggregate.json"
    with out_path.open("w") as f:
        json.dump(out, f, indent=2)

    print(f"=== FC-witness aggregate for {args.batch_dir} ===")
    print(f"Wrote: {out_path}")
    s = out["summary"]
    print(
        f"Summary: {s['total_nodes']} FC nodes; "
        f"{s['green_count']} GREEN, {s['amber_count']} AMBER, "
        f"{s['red_count']} RED, {s['gap_count']} GAP"
    )
    print()
    print("Per-node status:")
    for node, v in aggregate.items():
        print(
            f"  {v['aggregate_status']:6}  {node}  "
            f"(green:{len(v['green_by'])} amber:{len(v['amber_by'])} red:{len(v['red_by'])})"
        )

    if reds:
        print()
        print(f"RED nodes ({len(reds)}) — code bug to fix:")
        for r in reds:
            print(f"  - {r}: red on problems {aggregate[r]['red_by']}")

    if gaps:
        print()
        print(f"GAP nodes ({len(gaps)}) — need real problem to exercise:")
        for g in gaps:
            print(f"  - {g}")

    return 0


if __name__ == "__main__":
    sys.exit(main())
