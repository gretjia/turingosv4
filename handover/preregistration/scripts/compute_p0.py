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

    Reads the v2 RunAggregate field `progress` (jsonl_schema.rs:96). Falls
    back to legacy `has_golden_path` for pre-v2 rows. The earlier audit
    found this function was reading a non-existent `progress_verified`
    field — Codex Q3, fixed 2026-04-25.
    """
    if "progress" in row and row["progress"] is not None:
        return int(row["progress"]) == 1
    return bool(row.get("has_golden_path", False))


PREREG_SEEDS = (31415, 2718)
PREREG_N_PROBLEMS = 144


def compute(
    control_rows: list[dict],
    treatment_rows: list[dict],
    *,
    expected_n_problems: int = PREREG_N_PROBLEMS,
    expected_seeds: tuple[int, ...] = PREREG_SEEDS,
) -> dict:
    """PREREG § 5.5 estimator. Strict-complete: requires every (problem, seed)
    pair present in BOTH control and treatment, exact seed set, no missing
    `calibration_*` tags. Audit-fix 2026-04-25 (Codex B2 + Gemini Q3.d): the
    prior silently-skip behaviour biased p_0 by dropping incomplete pairs.
    """
    def index(rows: list[dict], mode: str) -> dict[tuple[str, int], dict]:
        out: dict[tuple[str, int], dict] = {}
        for i, r in enumerate(rows):
            pid = r.get("calibration_problem_id")
            seed = r.get("calibration_seed")
            if pid is None or seed is None:
                sys.exit(
                    f"ERROR: {mode} row {i} missing calibration_problem_id/seed — "
                    "runner stamping bug; refuse to compute p_0 on incomplete data"
                )
            key = (pid, seed)
            if key in out:
                sys.exit(
                    f"ERROR: {mode} duplicate row for (problem={pid}, seed={seed}) — "
                    "runner emitted twice; refuse to compute p_0 on duplicated data"
                )
            out[key] = r
        return out

    c = index(control_rows, "control")
    t = index(treatment_rows, "treatment")

    # Strict completeness: control and treatment key sets must be identical
    # AND must equal expected pre-registered (problem × seed) grid.
    expected_seed_set = set(expected_seeds)
    c_seeds = {seed for _, seed in c.keys()}
    t_seeds = {seed for _, seed in t.keys()}
    if c_seeds != expected_seed_set or t_seeds != expected_seed_set:
        sys.exit(
            f"ERROR: seed mismatch — expected {sorted(expected_seed_set)}; "
            f"control={sorted(c_seeds)}, treatment={sorted(t_seeds)}"
        )

    c_problems = {pid for pid, _ in c.keys()}
    t_problems = {pid for pid, _ in t.keys()}
    if c_problems != t_problems:
        only_c = c_problems - t_problems
        only_t = t_problems - c_problems
        sys.exit(
            f"ERROR: problem set mismatch between control and treatment — "
            f"only_in_control={sorted(only_c)[:5]}{'...' if len(only_c) > 5 else ''}, "
            f"only_in_treatment={sorted(only_t)[:5]}{'...' if len(only_t) > 5 else ''}"
        )

    if len(c_problems) != expected_n_problems:
        sys.exit(
            f"ERROR: expected exactly {expected_n_problems} problems per PREREG § 5.5; "
            f"got {len(c_problems)}. Refuse to compute p_0 on partial batch."
        )

    expected_pair_count = expected_n_problems * len(expected_seed_set)
    if len(c) != expected_pair_count or len(t) != expected_pair_count:
        sys.exit(
            f"ERROR: expected exactly {expected_pair_count} pairs per mode; "
            f"got control={len(c)}, treatment={len(t)}."
        )

    pairs = sorted(c.keys())

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

    # Denominator is the pre-registered count (audit-fix 2026-04-25 Codex
    # B2): if strict-completeness above passed, len(pairs)/len(seeds) ==
    # expected_n_problems by construction. Using the PREREG constant
    # makes the divide-by intent unambiguous.
    n_problems = expected_n_problems
    assert len({pid for pid, _ in pairs}) == n_problems
    p0 = sum(per_problem_regression.values()) / n_problems

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
