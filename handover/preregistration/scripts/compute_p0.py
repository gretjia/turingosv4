#!/usr/bin/env python3
"""PPUT-CCL B7-extra — compute p_0 from calibration jsonl.

PREREG § 5.5 estimator:
    For each (problem, seed): regression_p_seed = 1 iff control SOLVED
                              AND treatment UNSOLVED.
    Per-problem regression:   max over the 2 seeds (worst case).
    p_0:                      sum_p regression_p / N_problems.

Sanity gate: if p_0 > 0.10, ABORT — toggle too aggressive (PREREG § 5.5 ceiling).

Usage:
    compute_p0.py --control <control.jsonl> --treatment <treatment.jsonl>
"""

from __future__ import annotations

import argparse
import hashlib
import json
import sys
from collections import defaultdict
from pathlib import Path


def load_jsonl(path: Path) -> list[dict]:
    rows = []
    with path.open() as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            rows.append(json.loads(line))
    return rows


def solved(row: dict) -> bool:
    """PREREG § 1.3 progress = 1 iff Lean ground-truth verifies golden_path.

    The B4 split (`progress_runtime` vs `progress_verified`) frames the verified
    leg as authoritative. Fall back to `has_golden_path` for legacy rows.
    """
    if "progress_verified" in row and row["progress_verified"] is not None:
        return int(row["progress_verified"]) == 1
    return bool(row.get("has_golden_path", False))


def compute(control_rows: list[dict], treatment_rows: list[dict]) -> dict:
    # Index by (problem_id, seed). calibration_problem_id and calibration_seed
    # are stamped by run_p0_calibration.sh. Be defensive: skip rows missing
    # either tag.
    def index(rows):
        out = {}
        for r in rows:
            pid = r.get("calibration_problem_id")
            seed = r.get("calibration_seed")
            if pid is None or seed is None:
                continue
            out[(pid, seed)] = r
        return out

    c = index(control_rows)
    t = index(treatment_rows)

    pairs = sorted(set(c.keys()) & set(t.keys()))
    if not pairs:
        sys.exit("ERROR: no overlapping (problem, seed) pairs between control and treatment")

    # Per-problem worst-case regression (max over seeds).
    per_problem_regression: dict[str, int] = defaultdict(int)
    n_pairs = 0
    n_control_solved = 0
    n_treatment_solved = 0
    n_regression_pairs = 0
    for pid, seed in pairs:
        cr = c[(pid, seed)]
        tr = t[(pid, seed)]
        cs = solved(cr)
        ts = solved(tr)
        n_pairs += 1
        if cs:
            n_control_solved += 1
        if ts:
            n_treatment_solved += 1
        regression = 1 if (cs and not ts) else 0
        if regression:
            n_regression_pairs += 1
        if regression > per_problem_regression[pid]:
            per_problem_regression[pid] = regression

    n_problems = len({pid for pid, _ in pairs})
    p0 = sum(per_problem_regression.values()) / n_problems if n_problems else 0.0

    return {
        "n_problems": n_problems,
        "n_pairs": n_pairs,
        "n_control_solved": n_control_solved,
        "n_treatment_solved": n_treatment_solved,
        "n_regression_pairs": n_regression_pairs,
        "n_regression_problems_max_seed": sum(per_problem_regression.values()),
        "p0": p0,
        "p0_ceiling": 0.10,
        "ceiling_pass": p0 <= 0.10,
    }


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--control", required=True, type=Path)
    ap.add_argument("--treatment", required=True, type=Path)
    ap.add_argument("--out-json", type=Path, default=None,
                    help="Write structured result to this path")
    args = ap.parse_args()

    control_rows = load_jsonl(args.control)
    treatment_rows = load_jsonl(args.treatment)

    result = compute(control_rows, treatment_rows)
    print(json.dumps(result, indent=2))

    if args.out_json:
        args.out_json.write_text(json.dumps(result, indent=2) + "\n")

    # Hash the calibration jsonl pair for the genesis_payload.toml freeze step.
    h = hashlib.sha256()
    for path in (args.control, args.treatment):
        h.update(path.read_bytes())
    print(f"\n[freeze] baseline_regression_jsonl_sha256 (control+treatment, in order):")
    print(f"  {h.hexdigest()}")

    if not result["ceiling_pass"]:
        print(
            f"\nERROR: p_0 = {result['p0']:.4f} > 0.10 — ABORT per PREREG § 5.5 ceiling.",
            file=sys.stderr,
        )
        return 2
    return 0


if __name__ == "__main__":
    sys.exit(main())
