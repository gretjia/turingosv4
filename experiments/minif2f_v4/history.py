#!/usr/bin/env python3
"""
Auto-Research Outer Loop — History Tracker

Reads batch result files from logs/ and produces cross-run comparison.
The ArchitectAI (Claude Opus) reads this to form causal hypotheses
about what to change in the harness.

Meta-Harness insight: 10M tokens of diagnostic history > 0.002M compressed summary.
"""

import json
import os
import sys
from pathlib import Path
from collections import defaultdict

LOGS_DIR = Path(__file__).parent / "logs"


def load_batch_results():
    """Load all batch result JSONL files, sorted by timestamp."""
    results = []
    for f in sorted(LOGS_DIR.glob("batch_*.jsonl")):
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
            solved = sum(1 for p in run["problems"] if p.get("status") == "solved")
            run["total"] = total
            run["solved"] = solved
            run["solve_rate"] = solved / total if total > 0 else 0
            results.append(run)
    return results


def print_history():
    """Print cross-run history for ArchitectAI consumption."""
    runs = load_batch_results()

    if not runs:
        print("No batch results found in logs/")
        return

    print("=== MiniF2F v4 Auto-Research History ===\n")

    # Summary table
    print(f"{'Run':<40} {'Total':>6} {'Solved':>7} {'Rate':>7}")
    print("-" * 62)
    for run in runs:
        print(f"{run['file']:<40} {run['total']:>6} {run['solved']:>7} {run['solve_rate']:>6.1%}")

    # Trend
    if len(runs) > 1:
        first = runs[0]["solve_rate"]
        last = runs[-1]["solve_rate"]
        delta = last - first
        print(f"\nTrend: {first:.1%} → {last:.1%} (Δ = {delta:+.1%})")

    # Per-problem analysis (which problems are consistently solved/unsolved)
    problem_status = defaultdict(list)
    for run in runs:
        for p in run["problems"]:
            problem_status[p["problem"]].append(p.get("status", "unknown"))

    always_solved = [p for p, statuses in problem_status.items()
                     if all(s == "solved" for s in statuses) and len(statuses) > 1]
    never_solved = [p for p, statuses in problem_status.items()
                    if all(s != "solved" for s in statuses) and len(statuses) > 1]
    flaky = [p for p, statuses in problem_status.items()
             if len(set(statuses)) > 1]

    if always_solved:
        print(f"\nAlways solved ({len(always_solved)}): {', '.join(sorted(always_solved)[:5])}...")
    if never_solved:
        print(f"Never solved ({len(never_solved)}): {', '.join(sorted(never_solved)[:5])}...")
    if flaky:
        print(f"Flaky ({len(flaky)}): {', '.join(sorted(flaky)[:5])}...")


def export_jsonl():
    """Export cross-run history as history.jsonl for ArchitectAI."""
    runs = load_batch_results()
    output_path = LOGS_DIR.parent / "history.jsonl"
    with open(output_path, "w") as f:
        for run in runs:
            f.write(json.dumps({
                "file": run["file"],
                "total": run["total"],
                "solved": run["solved"],
                "solve_rate": run["solve_rate"],
            }) + "\n")
    print(f"Exported to {output_path}")


if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] == "--export":
        export_jsonl()
    else:
        print_history()
