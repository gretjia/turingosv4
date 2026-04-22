#!/usr/bin/env python3
"""Phase 9 multi-seed aggregator — Gate 9→10 decision tool.

PPUT-first per C-052 + Report Standard per CLAUDE.md. Consumes N seed
jsonls from 9.A (dual) or 9.B (step-only), outputs aggregate table +
Wilson CI + Gate verdict.

Usage:
  python3 phase9_aggregate.py --label dual seed1.jsonl seed2.jsonl ...
  python3 phase9_aggregate.py --label step_only seed1.jsonl ...

Gate 9→10 (from REGISTRATION_PHASE_9_2026-04-22.md § 4):
  Main (必过): Mean PPUT (solved-only, all seeds combined) Wilson 95% CI
               lower bound ≥ 5.0
  Aux (全部必过):
    - Σdepth≥10 PPUT > 0.5 across seeds AND depth≥10 solves ≥ 2
    - pairwise_diversity_mean ≥ 0.25 across seeds
    - reputation p50 > 0 per seed
    - Law 2 proptest 10K tx全绿 (reported separately, not here)
    - halt_reason_distribution 至少 3 种 reason 跨 seeds
"""
import argparse
import json
import math
import sys
from collections import Counter
from pathlib import Path


def wilson_ci(successes: int, n: int, z: float = 1.96) -> tuple[float, float]:
    if n == 0:
        return (0.0, 0.0)
    p = successes / n
    denom = 1 + z**2 / n
    center = (p + z**2 / (2 * n)) / denom
    half = z * math.sqrt(p * (1 - p) / n + z**2 / (4 * n**2)) / denom
    return (max(0.0, center - half), min(1.0, center + half))


def mean_ci_normal(values: list[float], z: float = 1.96) -> tuple[float, float, float]:
    n = len(values)
    if n == 0:
        return (0.0, 0.0, 0.0)
    mean = sum(values) / n
    if n < 2:
        return (mean, mean, mean)
    variance = sum((v - mean) ** 2 for v in values) / (n - 1)
    se = math.sqrt(variance / n)
    return (mean, max(0.0, mean - z * se), mean + z * se)


def percentile(values: list[float], p: float) -> float:
    if not values:
        return 0.0
    s = sorted(values)
    k = (len(s) - 1) * p / 100.0
    f, c = math.floor(k), math.ceil(k)
    if f == c:
        return s[int(k)]
    return s[f] * (c - k) + s[c] * (k - f)


def load_seed(path: Path) -> list[dict]:
    return [json.loads(line) for line in path.open() if line.strip()]


def seed_summary(tag: str, rows: list[dict]) -> dict:
    solved = [r for r in rows if r.get("has_golden_path")]
    total = len(rows)
    n_solved = len(solved)
    sigma_pput = sum(r.get("pput", 0.0) for r in rows)
    solved_pputs = [r["pput"] for r in solved]
    mean_pput_solved, ms_lo, ms_hi = mean_ci_normal(solved_pputs)

    deep = [r for r in solved if r.get("gp_node_count", 0) >= 10]
    sigma_depth10 = sum(r.get("pput", 0.0) for r in deep)

    # Aux fields (Phase 9 § 0): may be absent on pre-9.0 rows — skip None.
    diversities = [
        r["pairwise_diversity_mean"]
        for r in rows
        if r.get("pairwise_diversity_mean") is not None
    ]
    div_mean = sum(diversities) / len(diversities) if diversities else None

    # reputation p50 across all agents-authors-credited in this run
    rep_counts: list[int] = []
    for r in rows:
        rep = r.get("reputation_at_end") or {}
        rep_counts.extend(rep.values())
    rep_p50 = percentile([float(c) for c in rep_counts], 50) if rep_counts else 0.0

    halt_reasons = Counter()
    for r in rows:
        hr = r.get("halt_reason")
        if hr:
            halt_reasons[hr] += 1

    return {
        "tag": tag,
        "total": total,
        "n_solved": n_solved,
        "sigma_pput": sigma_pput,
        "mean_pput_solved": mean_pput_solved,
        "mean_pput_solved_ci": (ms_lo, ms_hi),
        "n_depth10": len(deep),
        "sigma_depth10_pput": sigma_depth10,
        "div_mean": div_mean,
        "rep_p50": rep_p50,
        "halt_reasons": dict(halt_reasons),
        "solved_pputs": solved_pputs,
    }


def gate_9_verdict(seeds: list[dict]) -> tuple[str, list[str]]:
    """Gate 9→10 per REGISTRATION § 4."""
    # Combine solved PPUTs across seeds for primary criterion.
    all_solved_pputs: list[float] = []
    total_depth10 = 0
    total_sigma_depth10 = 0.0
    divs: list[float] = []
    rep_p50s: list[float] = []
    all_reasons: set[str] = set()
    for s in seeds:
        all_solved_pputs.extend(s["solved_pputs"])
        total_depth10 += s["n_depth10"]
        total_sigma_depth10 += s["sigma_depth10_pput"]
        if s["div_mean"] is not None:
            divs.append(s["div_mean"])
        rep_p50s.append(s["rep_p50"])
        all_reasons.update(s["halt_reasons"].keys())

    primary_mean, _, _ = mean_ci_normal(all_solved_pputs)
    _, primary_lo, _ = mean_ci_normal(all_solved_pputs)
    primary_pass = primary_lo >= 5.0

    depth_pass = (total_sigma_depth10 > 0.5) and (total_depth10 >= 2)
    div_pass = bool(divs) and (sum(divs) / len(divs)) >= 0.25
    rep_pass = all(p > 0 for p in rep_p50s)
    reason_pass = len(all_reasons) >= 3

    reasons: list[str] = []
    if not primary_pass:
        reasons.append(
            f"PRIMARY: Mean PPUT (solved) CI lower {primary_lo:.3f} < 5.0"
        )
    if not depth_pass:
        reasons.append(
            f"depth≥10: Σ={total_sigma_depth10:.2f} (<0.5) or count={total_depth10} (<2)"
        )
    if not div_pass:
        reasons.append(
            f"diversity mean across seeds = "
            f"{(sum(divs)/len(divs) if divs else 0):.3f} < 0.25 "
            f"(or not reported)"
        )
    if not rep_pass:
        reasons.append("reputation p50 == 0 in one or more seeds")
    if not reason_pass:
        reasons.append(
            f"halt_reason distribution has only {len(all_reasons)} distinct "
            f"reason(s): {sorted(all_reasons)}"
        )

    if all([primary_pass, depth_pass, div_pass, rep_pass, reason_pass]):
        return ("PASS", [])
    return ("FAIL", reasons)


def main(argv: list[str]) -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--label", default="phase9",
                    help="label for this group (e.g. 'dual' / 'step_only')")
    ap.add_argument("seeds", nargs="+", help="jsonl files, one per seed")
    args = ap.parse_args(argv[1:])

    summaries = []
    for path in args.seeds:
        rows = load_seed(Path(path))
        s = seed_summary(Path(path).stem, rows)
        summaries.append(s)
        print(f"\n=== {s['tag']} ({args.label}) ===")
        print(f"  solved:             {s['n_solved']}/{s['total']}")
        print(f"  ΣPPUT:              {s['sigma_pput']:.2f}")
        print(f"  Mean PPUT (solved): {s['mean_pput_solved']:.3f} "
              f"CI [{s['mean_pput_solved_ci'][0]:.3f}, "
              f"{s['mean_pput_solved_ci'][1]:.3f}]")
        print(f"  depth≥10 count:     {s['n_depth10']}")
        print(f"  Σdepth≥10 PPUT:     {s['sigma_depth10_pput']:.3f}")
        print(f"  diversity mean:     "
              f"{s['div_mean']:.3f}" if s['div_mean'] is not None else "N/A")
        print(f"  reputation p50:     {s['rep_p50']:.1f}")
        print(f"  halt_reasons:       {s['halt_reasons']}")

    verdict, reasons = gate_9_verdict(summaries)
    print(f"\n=== Gate 9→10 verdict ({args.label}, combined {len(summaries)} seeds) ===")
    print(f"  {verdict}")
    for r in reasons:
        print(f"    - {r}")
    return 0 if verdict == "PASS" else 1


if __name__ == "__main__":
    sys.exit(main(sys.argv))
