#!/usr/bin/env python3
"""
PPUT-CCL Phase C atom C3 — H1-H4 statistical analyzer.

Per PREREG_PPUT_CCL_2026-04-26.md § 5.2 + § 9:
  - Independent unit: per-problem (n=10); 2 seeds = noise-reducing replicates
  - For each pair (Full, Mode_alt) where Mode_alt ∈
    {SoftLaw, Panopticon, Amnesia, Homogeneous}:
      paired_binary[problem] = 1 if mean(VPPUT_Full)_problem > mean(VPPUT_Alt)_problem
                                else 0  (mean over 2 seeds)
  - Test: McNemar one-sided exact binomial on the 10 paired-binary observations
  - Decision: Holm-Bonferroni at family-wise α = 0.05, N_max = 34 (PREREG § 9.1)
    Smallest threshold = 0.05/34 ≈ 0.00147 → requires 10/10 paired wins.

Descriptive secondary endpoints (no α correction; PREREG § 5.2 secondary):
  - Per-mode mean VPPUT / tokens / wall_clock_ms / verifier_wait_ms
  - SoftLaw H1 gap: mean(pput_runtime) - mean(pput_verified)
  - Per-mode solve count / verified count

Usage:
  python3 handover/preregistration/scripts/analyze_c3_h1_h4.py <glob>
  e.g.
  python3 handover/preregistration/scripts/analyze_c3_h1_h4.py \\
    "experiments/minif2f_v4/logs/c2_phase_c_ablation_20260426T*__*.jsonl"

Reads all matching jsonl files (one row per file, as written by
run_c2_phase_c_ablation.sh), groups by (mode, problem_id, seed),
emits the report on stdout. Exit 0 on success regardless of stat
outcomes; exit 2 on data-shape failure (missing modes, < 10 problems
per mode, etc.).

Filename convention from runner:
  <prefix>__<mode>_<problem_id>_seed<seed>.jsonl
"""
from __future__ import annotations

import argparse
import glob
import json
import math
import re
import sys
from collections import defaultdict
from typing import Iterable

ALL_MODES = ["full", "soft_law", "homogeneous", "panopticon", "amnesia"]
ALT_MODES = [m for m in ALL_MODES if m != "full"]

# PREREG § 6 C2: hard-10 sample size + 2 frozen Boltzmann seeds.
HARD_N = 10
EXPECTED_SEEDS = {31415, 2718}

# PREREG § 9.1: family-wise α + N_max for Holm.
ALPHA_FAMILY = 0.05
N_MAX = 34

# Filename pattern: <prefix>__<mode>_<problem>_seed<seed>.jsonl
# Mode names contain underscores (soft_law) so the regex is anchored to the
# allowed-mode prefix.
FNAME_RE = re.compile(
    r".*__(?P<mode>"
    + "|".join(ALL_MODES)
    + r")_(?P<problem>.+)_seed(?P<seed>\d+)\.jsonl$"
)


def parse_filename(path: str) -> tuple[str, str, int] | None:
    m = FNAME_RE.match(path)
    if m is None:
        return None
    return m.group("mode"), m.group("problem"), int(m.group("seed"))


def load_rows(file_glob: str) -> list[dict]:
    rows: list[dict] = []
    paths = sorted(glob.glob(file_glob))
    for p in paths:
        meta = parse_filename(p)
        if meta is None:
            print(f"[warn] skipping non-conforming filename: {p}", file=sys.stderr)
            continue
        mode, problem, seed = meta
        try:
            with open(p, "r", encoding="utf-8") as f:
                line = f.readline().strip()
            if not line:
                print(f"[warn] empty file, skipping: {p}", file=sys.stderr)
                continue
            row = json.loads(line)
        except (OSError, json.JSONDecodeError) as e:
            print(f"[warn] failed to parse {p}: {e}", file=sys.stderr)
            continue
        # Filename-derived axes are authoritative; jsonl values must match.
        row["_filename_mode"] = mode
        row["_filename_problem"] = problem
        row["_filename_seed"] = seed
        rows.append(row)
    return rows


def group_means_per_problem(rows: list[dict]) -> dict[str, dict[str, dict[str, float]]]:
    """
    Returns {mode: {problem_id: {field: mean_over_seeds}}}.
    Fields: pput_verified, pput_runtime, total_run_token_count,
            total_wall_time_ms, verifier_wait_ms, solved (as int 0/1),
            verified (as int 0/1).
    """
    bucket: dict[tuple[str, str], list[dict]] = defaultdict(list)
    for r in rows:
        key = (r["_filename_mode"], r["_filename_problem"])
        bucket[key].append(r)

    out: dict[str, dict[str, dict[str, float]]] = defaultdict(dict)
    for (mode, problem), seed_rows in bucket.items():
        agg = {
            "pput_verified": _mean(seed_rows, "pput_verified"),
            "pput_runtime": _mean(seed_rows, "pput_runtime"),
            "total_run_token_count": _mean(seed_rows, "total_run_token_count"),
            "total_wall_time_ms": _mean(seed_rows, "total_wall_time_ms"),
            "verifier_wait_ms": _mean(seed_rows, "verifier_wait_ms"),
            "solved": _mean_bool(seed_rows, "solved"),
            "verified": _mean_bool(seed_rows, "verified"),
            "n_seeds": float(len(seed_rows)),
        }
        out[mode][problem] = agg
    return out


def _mean(rows: list[dict], key: str) -> float:
    vals = [float(r.get(key) or 0) for r in rows]
    return sum(vals) / len(vals) if vals else 0.0


def _mean_bool(rows: list[dict], key: str) -> float:
    vals = [1.0 if r.get(key) else 0.0 for r in rows]
    return sum(vals) / len(vals) if vals else 0.0


def paired_binary_outcomes(
    means: dict[str, dict[str, dict[str, float]]],
    full_mode: str,
    alt_mode: str,
    field: str = "pput_verified",
) -> list[int]:
    """
    Per PREREG § 5.2 H1-H4: paired_binary[p] = 1 iff
    mean(VPPUT_Full)_p > mean(VPPUT_Alt)_p.
    """
    full_rows = means.get(full_mode, {})
    alt_rows = means.get(alt_mode, {})
    common = sorted(set(full_rows.keys()) & set(alt_rows.keys()))
    out = []
    for p in common:
        full_v = full_rows[p][field]
        alt_v = alt_rows[p][field]
        out.append(1 if full_v > alt_v else 0)
    return out


def binomial_one_sided_p(b: int, n: int, p: float = 0.5) -> float:
    """
    P(X >= b | n, p) — one-sided upper-tail exact binomial p-value.
    Used for the McNemar paired sign test (PREREG § 9.4 worked example).
    """
    if b < 0 or n < 0 or b > n:
        raise ValueError(f"invalid binomial inputs: b={b} n={n}")
    total = 0.0
    for x in range(b, n + 1):
        total += math.comb(n, x) * (p ** x) * ((1 - p) ** (n - x))
    return total


def holm_thresholds(num_tests: int, n_max: int = N_MAX, alpha: float = ALPHA_FAMILY) -> list[float]:
    """
    Holm-Bonferroni stepwise: i-th smallest p-value is compared against
    α / (n_max - i + 1). PREREG § 9.2 — uses N_MAX (not realized num_tests),
    conservative.
    """
    return [alpha / (n_max - i + 1) for i in range(1, num_tests + 1)]


def holm_decisions(p_values: list[float], n_max: int = N_MAX, alpha: float = ALPHA_FAMILY) -> list[bool]:
    """
    Returns a list of bools aligned with p_values input order.
    True = reject null; False = fail to reject (per PREREG § 9.2 stop-on-first-fail).
    """
    indexed = sorted(enumerate(p_values), key=lambda t: t[1])
    decisions = [False] * len(p_values)
    for rank, (orig_idx, p) in enumerate(indexed, start=1):
        threshold = alpha / (n_max - rank + 1)
        if p > threshold:
            break
        decisions[orig_idx] = True
    return decisions


def render_report(rows: list[dict]) -> str:
    out: list[str] = []
    out.append("=" * 72)
    out.append("PPUT-CCL Phase C atom C3 — H1-H4 McNemar paired sign test analyzer")
    out.append("PREREG_PPUT_CCL_2026-04-26.md § 5.2 + § 9")
    out.append("=" * 72)
    out.append("")

    # Data shape diagnostics.
    by_mode: dict[str, list[dict]] = defaultdict(list)
    for r in rows:
        by_mode[r["_filename_mode"]].append(r)
    out.append(f"Total rows loaded:   {len(rows)}")
    out.append(f"Modes observed:      {sorted(by_mode.keys())}")
    for m in ALL_MODES:
        problems = {r["_filename_problem"] for r in by_mode.get(m, [])}
        seeds = {r["_filename_seed"] for r in by_mode.get(m, [])}
        rows_ct = len(by_mode.get(m, []))
        synth_ct = sum(1 for r in by_mode.get(m, []) if r.get("_synthetic_failure"))
        out.append(
            f"  {m:13s} rows={rows_ct:3d}  problems={len(problems):2d}  "
            f"seeds={sorted(seeds)}  synthetic_failures={synth_ct}"
        )
    out.append("")

    # Data shape gates.
    fatal = []
    for m in ALL_MODES:
        problems = {r["_filename_problem"] for r in by_mode.get(m, [])}
        if len(problems) != HARD_N:
            fatal.append(f"mode={m} has {len(problems)} problems, expected {HARD_N}")
        seeds = {r["_filename_seed"] for r in by_mode.get(m, [])}
        if seeds != EXPECTED_SEEDS:
            fatal.append(f"mode={m} seeds={sorted(seeds)}, expected {sorted(EXPECTED_SEEDS)}")
    if fatal:
        out.append("DATA-SHAPE GATE FAILED:")
        for f in fatal:
            out.append(f"  - {f}")
        out.append("")
        return "\n".join(out)
    out.append("Data-shape gate: PASS (5 modes × 10 problems × 2 seeds = 100 rows)")
    out.append("")

    # Per-mode means (descriptive endpoint).
    means = group_means_per_problem(rows)
    out.append("─" * 72)
    out.append("Per-mode descriptive endpoints (mean across 10 problems × 2 seeds):")
    out.append("─" * 72)
    out.append(
        f"  {'mode':13s} {'pput_verified':>15s} {'pput_runtime':>14s} "
        f"{'tokens_avg':>11s} {'wall_ms_avg':>12s} {'verify_ms_avg':>13s} "
        f"{'solve_rate':>11s} {'verify_rate':>12s}"
    )
    for m in ALL_MODES:
        if m not in means:
            continue
        problems = means[m]
        n_p = len(problems)
        if n_p == 0:
            continue
        pv = sum(d["pput_verified"] for d in problems.values()) / n_p
        pr = sum(d["pput_runtime"] for d in problems.values()) / n_p
        tk = sum(d["total_run_token_count"] for d in problems.values()) / n_p
        wm = sum(d["total_wall_time_ms"] for d in problems.values()) / n_p
        vm = sum(d["verifier_wait_ms"] for d in problems.values()) / n_p
        sv = sum(d["solved"] for d in problems.values()) / n_p
        vr = sum(d["verified"] for d in problems.values()) / n_p
        out.append(
            f"  {m:13s} {pv:15.6e} {pr:14.6e} {tk:11.0f} {wm:12.0f} "
            f"{vm:13.0f} {sv:11.3f} {vr:12.3f}"
        )
    out.append("")

    # SoftLaw H1 gap (descriptive secondary; PREREG § 5.2):
    if "soft_law" in means:
        sl = means["soft_law"]
        sl_pr = sum(d["pput_runtime"] for d in sl.values()) / len(sl)
        sl_pv = sum(d["pput_verified"] for d in sl.values()) / len(sl)
        gap = sl_pr - sl_pv
        out.append(
            f"SoftLaw H1 detection signal:"
            f" gap = pput_runtime ({sl_pr:.6e}) - pput_verified ({sl_pv:.6e}) = {gap:.6e}"
        )
        out.append("  Expected: gap > 0 (runtime fakes accept; Lean truth on ph leg)")
        out.append("")

    # H1-H4: McNemar one-sided exact binomial.
    out.append("─" * 72)
    out.append("H1-H4 inferential tests (PREREG § 5.2 Table; primary endpoint = pput_verified):")
    out.append("─" * 72)

    hypotheses = [
        ("H1", "soft_law", "Soft Law has lower verified-PPUT than Full"),
        ("H2", "panopticon", "Panopticon has lower H-VPPUT than Full"),
        ("H3", "amnesia", "Amnesia has lower H-VPPUT than Full"),
        ("H4", "homogeneous", "Homogeneous swarm has lower H-VPPUT than Full"),
    ]

    p_values: list[float] = []
    h_records: list[dict] = []
    for h_id, alt_mode, descr in hypotheses:
        outcomes = paired_binary_outcomes(means, "full", alt_mode, "pput_verified")
        n = len(outcomes)
        b = sum(outcomes)
        if n == 0:
            out.append(f"  {h_id} ({alt_mode}): NO PAIRED DATA — skipped")
            p_values.append(1.0)
            h_records.append({"h_id": h_id, "alt": alt_mode, "n": 0, "b": 0, "p": 1.0})
            continue
        p = binomial_one_sided_p(b, n)
        p_values.append(p)
        h_records.append({"h_id": h_id, "alt": alt_mode, "n": n, "b": b, "p": p, "descr": descr})

    # Holm decisions at N_MAX = 34 (PREREG § 9.2 conservative).
    decisions = holm_decisions(p_values, n_max=N_MAX, alpha=ALPHA_FAMILY)
    smallest_threshold = ALPHA_FAMILY / N_MAX

    out.append(
        f"Holm-Bonferroni at family-wise α={ALPHA_FAMILY}, N_max={N_MAX} "
        f"(conservative per PREREG § 9.2);"
    )
    out.append(
        f"smallest threshold = {smallest_threshold:.6f}; "
        f"requires 10/10 paired wins to reject (§ 9.4 worked example)."
    )
    out.append("")
    out.append(
        f"  {'ID':3s} {'Alt-mode':12s} {'paired_b/n':>10s} {'p-value':>11s} "
        f"{'reject?':>8s}  hypothesis"
    )
    for rec, reject in zip(h_records, decisions):
        sig_str = "REJECT" if reject else "fail"
        ratio_str = "{}/{}".format(rec.get("b", 0), rec.get("n", 0))
        out.append(
            f"  {rec['h_id']:3s} {rec['alt']:12s} "
            f"{ratio_str:>10s} "
            f"{rec['p']:11.6f} {sig_str:>8s}  {rec.get('descr', '')}"
        )
    out.append("")

    # Phase C overall decision (PREREG § 7 Gate C: H1-H4 each pass at Bonferroni α).
    n_rejected = sum(decisions)
    out.append("─" * 72)
    out.append(
        f"Phase C Gate C (PREREG § 7): H1-H4 each pass at Bonferroni α — "
        f"{n_rejected}/4 rejected at N_max=34 Holm-Bonferroni."
    )
    if n_rejected == 4:
        out.append("Gate C: PASS (all 4 hypotheses rejected at family-wise α=0.05)")
    elif n_rejected >= 1:
        out.append(
            f"Gate C: PARTIAL — {n_rejected} of 4 rejected. "
            f"Per `feedback_phased_checkpoint`: prefer NEGATIVE finding to N enlargement."
        )
    else:
        out.append(
            "Gate C: FAIL — 0/4 rejected. NEGATIVE finding per pre-reg discipline."
        )
    out.append("─" * 72)
    out.append("")

    return "\n".join(out)


def main() -> int:
    parser = argparse.ArgumentParser(
        description="C3 H1-H4 analyzer for PPUT-CCL Phase C ablation jsonl rows."
    )
    parser.add_argument(
        "glob",
        help="File glob for jsonl rows; e.g. "
        "'experiments/minif2f_v4/logs/c2_phase_c_ablation_*__*.jsonl'",
    )
    args = parser.parse_args()

    rows = load_rows(args.glob)
    if not rows:
        print(f"FATAL: no rows loaded from glob: {args.glob}", file=sys.stderr)
        return 2

    report = render_report(rows)
    print(report)
    return 0


if __name__ == "__main__":
    sys.exit(main())
