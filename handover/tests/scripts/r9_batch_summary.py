#!/usr/bin/env python3
"""Write the TB-18R R9 batch summary from per-problem invariant JSON."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out-dir", required=True)
    parser.add_argument("--problems-file", required=True)
    parser.add_argument("--max-tx", required=True, type=int)
    parser.add_argument("--per-problem-timeout-s", required=True, type=int)
    parser.add_argument("--git-head", required=True)
    parser.add_argument("--run-timestamp-utc", required=True)
    return parser.parse_args()


def load_problems(path: Path) -> list[str]:
    return [line.strip() for line in path.read_text().splitlines() if line.strip()]


def problem_result(out_dir: Path, idx: int, name: str) -> dict[str, Any]:
    tag = f"P{idx:02d}_{name}"
    invariant_path = out_dir / tag / "chain_invariant.json"
    if not invariant_path.exists():
        return {"tag": tag, "error": "no chain_invariant.json"}

    with invariant_path.open() as f:
        invariant = json.load(f)

    return {
        "tag": tag,
        "l4": invariant.get("l4_work_attempt_count"),
        "l4e": invariant.get("l4e_work_attempt_count"),
        "delta": invariant.get("delta"),
        "evaluable": invariant.get("r4_invariant_equation_evaluable"),
        "invariant_verdict": invariant.get("invariant_verdict"),
    }


def main() -> int:
    args = parse_args()
    out_dir = Path(args.out_dir)
    problems = load_problems(Path(args.problems_file))
    summary = {
        "phase": "TB-18R G2 round-2 R9",
        "problem_count": len(problems),
        "max_transactions_per_problem": args.max_tx,
        "per_problem_timeout_s": args.per_problem_timeout_s,
        "git_head": args.git_head,
        "run_timestamp_utc": args.run_timestamp_utc,
        "per_problem_results": [
            problem_result(out_dir, i + 1, name) for i, name in enumerate(problems)
        ],
    }
    (out_dir / "R9_BATCH_SUMMARY.json").write_text(
        json.dumps(summary, indent=2) + "\n"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
