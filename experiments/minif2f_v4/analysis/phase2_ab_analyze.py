#!/usr/bin/env python3
"""Phase 2 A/B analysis — Gate 8→9 decision tool.

PPUT-first per C-052. Reports ΣPPUT + Mean PPUT (Wilson CI) + depth≥10
histogram + halt_reason distribution + paired Δ. Prints Gate verdict.

Usage:
  python3 phase2_ab_analyze.py <baseline_jsonl> <experiment_jsonl>
"""
import json, sys, math
from pathlib import Path


def wilson_ci(successes: int, n: int, z: float = 1.96) -> tuple[float, float]:
    """Wilson score CI for a binomial proportion. Returns (lo, hi)."""
    if n == 0:
        return (0.0, 0.0)
    p = successes / n
    denom = 1 + z**2 / n
    center = (p + z**2 / (2 * n)) / denom
    half = z * math.sqrt(p * (1 - p) / n + z**2 / (4 * n**2)) / denom
    return (max(0.0, center - half), min(1.0, center + half))


def mean_ci(values: list[float], z: float = 1.96, clamp_to_nonneg: bool = True) -> tuple[float, float, float]:
    """Mean + normal-approximation CI. Returns (mean, lo, hi).

    `clamp_to_nonneg=True` is for proportion-like metrics (PPUT ≥ 0) where
    lower bound should not go negative. Set to False for signed deltas.
    """
    n = len(values)
    if n == 0:
        return (0.0, 0.0, 0.0)
    mean = sum(values) / n
    if n < 2:
        return (mean, mean, mean)
    variance = sum((v - mean) ** 2 for v in values) / (n - 1)
    se = math.sqrt(variance / n)
    lo = mean - z * se
    if clamp_to_nonneg:
        lo = max(0.0, lo)
    return (mean, lo, mean + z * se)


def load_jsonl(path: Path) -> list[dict]:
    rows = []
    for line in path.open():
        line = line.strip()
        if line:
            rows.append(json.loads(line))
    return rows


def summarize(tag: str, rows: list[dict]) -> dict:
    solved = [r for r in rows if r.get("has_golden_path")]
    total = len(rows)
    n_solved = len(solved)
    sigma_pput = sum(r.get("pput", 0.0) for r in rows)

    solved_pputs = [r["pput"] for r in solved]
    mean_pput_all, _, _ = mean_ci([r.get("pput", 0.0) for r in rows])
    mean_pput_solved, mp_lo, mp_hi = mean_ci(solved_pputs)

    solve_rate = n_solved / total if total else 0
    sr_lo, sr_hi = wilson_ci(n_solved, total)

    depths = [r.get("gp_node_count", 0) for r in solved]
    max_depth = max(depths) if depths else 0
    deep = [r for r in solved if r.get("gp_node_count", 0) >= 10]
    sigma_depth10 = sum(r.get("pput", 0.0) for r in deep)

    gp_paths = {}
    for r in solved:
        p = r.get("gp_path", "?")
        gp_paths[p] = gp_paths.get(p, 0) + 1

    return {
        "tag": tag,
        "total": total,
        "solved": n_solved,
        "solve_rate": solve_rate,
        "solve_rate_ci": (sr_lo, sr_hi),
        "sigma_pput": sigma_pput,
        "mean_pput_all": mean_pput_all,
        "mean_pput_solved": mean_pput_solved,
        "mean_pput_solved_ci": (mp_lo, mp_hi),
        "max_depth": max_depth,
        "n_depth10": len(deep),
        "sigma_depth10_pput": sigma_depth10,
        "gp_paths": gp_paths,
    }


def print_summary(s: dict) -> None:
    print(f"\n=== {s['tag']} ===")
    print(f"  problems:             {s['total']}")
    print(f"  solved:               {s['solved']}/{s['total']} "
          f"(Wilson CI: [{s['solve_rate_ci'][0]:.3f}, {s['solve_rate_ci'][1]:.3f}])")
    print(f"  ΣPPUT:                {s['sigma_pput']:.3f}")
    print(f"  Mean PPUT (solved):   {s['mean_pput_solved']:.3f} "
          f"(CI: [{s['mean_pput_solved_ci'][0]:.3f}, {s['mean_pput_solved_ci'][1]:.3f}])")
    print(f"  Mean PPUT (all):      {s['mean_pput_all']:.3f}")
    print(f"  Max depth:            {s['max_depth']}")
    print(f"  depth≥10 solves:      {s['n_depth10']}")
    print(f"  Σdepth≥10 PPUT:       {s['sigma_depth10_pput']:.3f}")
    print(f"  gp_paths:             {s['gp_paths']}")


def paired_delta(baseline: list[dict], experiment: list[dict]) -> dict:
    """Same-problem-id paired comparison for McNemar etc."""
    by_id_b = {Path(r["problem"]).stem: r for r in baseline}
    by_id_e = {Path(r["problem"]).stem: r for r in experiment}
    common = sorted(set(by_id_b) & set(by_id_e))
    pput_deltas = []
    both_solved = 0
    only_b = 0
    only_e = 0
    neither = 0
    for pid in common:
        rb = by_id_b[pid]
        re = by_id_e[pid]
        pput_deltas.append(re.get("pput", 0.0) - rb.get("pput", 0.0))
        sb = rb.get("has_golden_path", False)
        se = re.get("has_golden_path", False)
        if sb and se:
            both_solved += 1
        elif sb and not se:
            only_b += 1
        elif not sb and se:
            only_e += 1
        else:
            neither += 1
    total_delta = sum(pput_deltas)
    # Paired Δ is signed; don't clamp lower bound to 0.
    mean_delta, d_lo, d_hi = mean_ci(pput_deltas, clamp_to_nonneg=False)
    return {
        "n_paired": len(common),
        "total_pput_delta": total_delta,
        "mean_pput_delta": mean_delta,
        "mean_pput_delta_ci": (d_lo, d_hi),
        "both_solved": both_solved,
        "only_baseline": only_b,
        "only_experiment": only_e,
        "neither": neither,
    }


def gate_verdict(s_b: dict, s_e: dict, p: dict) -> str:
    """Gate 8→9 verdict — aligned with DECISION_TREE_GATE_8_TO_PHASE_9_2026-04-22.md § 4.1.

    PASS:
      - Paired ΔPPUT CI does NOT fully lie below -0.05
      - AND one of:
        (a) ΣPPUT_exp >= 0.90 * ΣPPUT_main
        (b) solve_count_exp >= solve_count_main - 1 (1-solve tolerance)

    INCONCLUSIVE:
      - Paired ΔPPUT CI crosses 0 AND ΣPPUT gap > 10%
      - → re-run seed 2

    FAIL:
      - Paired ΔPPUT CI lies entirely below -0.10
      - OR ΣPPUT gap > 25%
      - → HOLD + diagnose
    """
    delta_lo, delta_hi = p["mean_pput_delta_ci"]
    sigma_main = s_b["sigma_pput"]
    sigma_exp = s_e["sigma_pput"]
    sigma_ratio = sigma_exp / sigma_main if sigma_main > 0 else 1.0
    sigma_gap = 1 - sigma_ratio
    solve_delta = s_e["solved"] - s_b["solved"]

    # FAIL first (strongest signal)
    if delta_hi < -0.10:
        return (f"FAIL: Paired ΔPPUT CI upper {delta_hi:+.3f} < -0.10 (severe regression)")
    if sigma_gap > 0.25:
        return (f"FAIL: ΣPPUT gap {sigma_gap*100:.1f}% > 25% (severe regression)")

    # PASS: delta CI not fully below -0.05 AND (sigma OK OR solve count OK)
    delta_not_below = delta_hi >= -0.05
    sigma_ok = sigma_ratio >= 0.90
    solves_ok = solve_delta >= -1
    if delta_not_below and (sigma_ok or solves_ok):
        reasons = []
        if sigma_ok:
            reasons.append(f"ΣPPUT ratio {sigma_ratio*100:.1f}% ≥ 90%")
        if solves_ok:
            reasons.append(f"solves Δ {solve_delta:+d} ≥ -1")
        return f"PASS: Δ CI upper {delta_hi:+.3f} ≥ -0.05; " + " OR ".join(reasons)

    # INCONCLUSIVE: CI crosses 0 + sigma gap > 10%
    if delta_lo < 0 < delta_hi and sigma_gap > 0.10:
        return (f"INCONCLUSIVE: Δ CI crosses 0 ({delta_lo:+.3f}, {delta_hi:+.3f}) + "
                f"ΣPPUT gap {sigma_gap*100:.1f}% > 10% → re-run seed 2")

    # Default: borderline — report what failed
    reasons = []
    if not delta_not_below:
        reasons.append(f"Δ CI upper {delta_hi:+.3f} < -0.05")
    if not sigma_ok:
        reasons.append(f"ΣPPUT ratio {sigma_ratio*100:.1f}% < 90%")
    if not solves_ok:
        reasons.append(f"solves Δ {solve_delta:+d} < -1")
    return "INCONCLUSIVE (borderline): " + "; ".join(reasons) + " → re-run seed 2"


def main(argv: list[str]) -> int:
    if len(argv) != 3:
        print(__doc__, file=sys.stderr)
        return 2
    baseline_rows = load_jsonl(Path(argv[1]))
    experiment_rows = load_jsonl(Path(argv[2]))
    s_b = summarize("BASELINE (main, pre-Phase-8)", baseline_rows)
    s_e = summarize("EXPERIMENT (post-Phase-8, R1-α)", experiment_rows)
    print_summary(s_b)
    print_summary(s_e)
    p = paired_delta(baseline_rows, experiment_rows)
    print(f"\n=== Paired Δ (same-problem) ===")
    print(f"  paired N:             {p['n_paired']}")
    print(f"  ΣPPUT Δ (exp-base):   {p['total_pput_delta']:+.3f}")
    print(f"  mean PPUT Δ:          {p['mean_pput_delta']:+.3f} "
          f"(CI: [{p['mean_pput_delta_ci'][0]:+.3f}, {p['mean_pput_delta_ci'][1]:+.3f}])")
    print(f"  both solved:          {p['both_solved']}")
    print(f"  only baseline solved: {p['only_baseline']}")
    print(f"  only experiment:      {p['only_experiment']}")
    print(f"  neither:              {p['neither']}")

    verdict = gate_verdict(s_b, s_e, p)
    print(f"\n=== Gate 8→9 verdict ===")
    print(f"  {verdict}")
    return 0 if verdict == "PASS" else 1


if __name__ == "__main__":
    sys.exit(main(sys.argv))
