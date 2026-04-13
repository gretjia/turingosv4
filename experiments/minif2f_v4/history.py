#!/usr/bin/env python3
"""
PPUT History Tracker — Auto-Research Outer Loop

Reads PPUT result files from logs/ and produces cross-run comparison.
The ArchitectAI reads this to decide what harness changes will maximize PPUT.

PPUT = Progress Per Unit Time
  - GP exists → PPUT = 100% / time_to_omega
  - No GP    → PPUT = 0 (problem too hard for current harness, skip in early stage)
"""

import json
import os
import sys
from pathlib import Path
from collections import defaultdict

LOGS_DIR = Path(__file__).parent / "logs"


def load_pput_results():
    """Load all PPUT JSONL files, sorted by timestamp."""
    runs = []
    for f in sorted(LOGS_DIR.glob("pput_*.jsonl")):
        run = {"file": f.name, "problems": []}
        with open(f) as fh:
            for line in fh:
                line = line.strip()
                if line:
                    try:
                        run["problems"].append(json.loads(line))
                    except json.JSONDecodeError:
                        pass
        if run["problems"]:
            total = len(run["problems"])
            solved = sum(1 for p in run["problems"] if p.get("has_golden_path"))
            pput_sum = sum(p.get("pput", 0) for p in run["problems"])
            run["total"] = total
            run["solved"] = solved
            run["pput_sum"] = pput_sum
            run["avg_pput"] = pput_sum / solved if solved > 0 else 0
            runs.append(run)
    return runs


def categorize_problem(name):
    """Classify problem by type for per-category PPUT analysis."""
    name = name.lower().replace(".lean", "")
    if name.startswith("aime"): return "aime"
    if name.startswith("amc"): return "amc"
    if name.startswith("mathd_algebra"): return "mathd_algebra"
    if name.startswith("mathd_numbertheory"): return "mathd_numtheory"
    if name.startswith("algebra"): return "algebra"
    if name.startswith("numbertheory"): return "numtheory"
    if name.startswith("induction"): return "induction"
    if name.startswith("imo"): return "imo"
    return "other"


def print_history():
    """Print PPUT-centric cross-run history."""
    runs = load_pput_results()

    if not runs:
        print("No PPUT results found in logs/")
        return

    print("=== PPUT Auto-Research History ===\n")

    # Run summary
    print(f"{'Run':<45} {'Total':>5} {'GP':>4} {'PPUT=0':>6} {'Σ PPUT':>8} {'Avg PPUT':>9}")
    print("-" * 80)
    for run in runs:
        print(f"{run['file']:<45} {run['total']:>5} {run['solved']:>4} "
              f"{run['total']-run['solved']:>6} {run['pput_sum']:>7.1f} {run['avg_pput']:>8.2f}")

    # Trend
    if len(runs) > 1:
        first_pput = runs[0]["pput_sum"]
        last_pput = runs[-1]["pput_sum"]
        delta = last_pput - first_pput
        print(f"\nΣ PPUT trend: {first_pput:.1f} → {last_pput:.1f} (Δ = {delta:+.1f})")

    # Per-category analysis (which categories have highest PPUT)
    if runs:
        latest = runs[-1]
        categories = defaultdict(lambda: {"total": 0, "solved": 0, "pput_sum": 0})
        for p in latest["problems"]:
            cat = categorize_problem(p.get("problem", ""))
            categories[cat]["total"] += 1
            if p.get("has_golden_path"):
                categories[cat]["solved"] += 1
                categories[cat]["pput_sum"] += p.get("pput", 0)

        print(f"\n--- Latest Run: Per-Category PPUT ---")
        print(f"{'Category':<20} {'Total':>5} {'GP':>4} {'Σ PPUT':>8} {'Avg':>8}")
        print("-" * 48)
        sorted_cats = sorted(categories.items(),
                           key=lambda x: x[1]["pput_sum"], reverse=True)
        for cat, stats in sorted_cats:
            avg = stats["pput_sum"] / stats["solved"] if stats["solved"] > 0 else 0
            print(f"{cat:<20} {stats['total']:>5} {stats['solved']:>4} "
                  f"{stats['pput_sum']:>7.1f} {avg:>7.2f}")

    # PPUT=0 problems (frozen / too hard for current harness)
    if runs:
        latest = runs[-1]
        zero_problems = [p["problem"] for p in latest["problems"]
                        if not p.get("has_golden_path")]
        gp_problems = sorted(
            [(p["problem"], p["pput"]) for p in latest["problems"]
             if p.get("has_golden_path")],
            key=lambda x: x[1], reverse=True
        )

        if gp_problems:
            print(f"\n--- Top PPUT Problems (focus here) ---")
            for name, pput in gp_problems[:10]:
                print(f"  {name:<40} PPUT={pput:.2f}%/s")

        print(f"\n--- PPUT=0 Problems ({len(zero_problems)} — skip in early stage) ---")
        for name in sorted(zero_problems)[:5]:
            print(f"  {name}")
        if len(zero_problems) > 5:
            print(f"  ... and {len(zero_problems)-5} more")


def export_jsonl():
    """Export cross-run PPUT history as history.jsonl for ArchitectAI."""
    runs = load_pput_results()
    output_path = LOGS_DIR.parent / "history.jsonl"
    with open(output_path, "w") as f:
        for run in runs:
            f.write(json.dumps({
                "file": run["file"],
                "total": run["total"],
                "solved": run["solved"],
                "pput_sum": round(run["pput_sum"], 2),
                "avg_pput": round(run["avg_pput"], 2),
            }) + "\n")
    print(f"Exported to {output_path}")


if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] == "--export":
        export_jsonl()
    else:
        print_history()
