#!/usr/bin/env python3
"""aggregate_e1v2.py — aggregate Paper 1 v2 E1 re-run results across 4 seeds × 3 conditions.

Input: glob over logs/E1v2_*_n8_*.jsonl
Output: JSON summary + McNemar one-sided/two-sided + Bonferroni-corrected verdicts.

Conditions:
  A   : HOMOGENEOUS_AGENTS=1, single skill (negative control)
  B   : default heterogeneous (Meta-Planner + 3 others) — primary arm
  Abl : EXCLUDE_META_PLANNER=1 (3 skills, no Meta-Planner) — ablation

Family of comparisons (multiplicity = 4, per PREREG § multiplicity_family):
  1. B vs A  (primary heterogeneity effect)
  2. Abl vs A (heterogeneity without Meta-Planner)
  3. B vs Abl (Meta-Planner marginal effect)
  4. global aggregate

Bonferroni α = 0.05 / 4 = 0.0125 for primary one-sided tests.

Usage:
    python3 tools/aggregate_e1v2.py \
        --logs '/home/.../logs/E1v2_*_n8_*.jsonl' \
        --out handover/preregistration/E1v2_RESULTS_2026-04-24.json
"""
from __future__ import annotations
import argparse
import glob
import json
import re
from collections import defaultdict
from math import comb
from pathlib import Path


CONDITIONS = ["A", "B", "Abl"]
SEEDS = [141421, 31415, 2718, 2357]

TAG_RE = re.compile(r"E1v2_(A|B|Abl)_s(\d+)")


def mcnemar_exact(b: int, c: int, one_sided: bool = True) -> float:
    """McNemar exact binomial test.
    b = problems where condition X solved but Y did not
    c = problems where Y solved but X did not
    one_sided: P(B>=b | b+c, 0.5)  (for direction X better than Y)
    two_sided: 2 * min(one_sided, 1 - one_sided + P(exactly))
    """
    n = b + c
    if n == 0:
        return 1.0
    # one-sided: P(X >= b) assuming binomial(n, 0.5)
    one_sided_p = sum(comb(n, k) for k in range(b, n + 1)) / (2 ** n)
    if one_sided:
        return one_sided_p
    # two-sided: double the smaller tail (standard exact)
    other = sum(comb(n, k) for k in range(0, b + 1)) / (2 ** n)
    return min(1.0, 2 * min(one_sided_p, other))


def load_logs(glob_pat: str):
    """Returns dict[(seed, condition)] = {problem: entry}"""
    data: dict[tuple[int, str], dict[str, dict]] = defaultdict(dict)
    for path in sorted(glob.glob(glob_pat)):
        fname = Path(path).name
        m = TAG_RE.search(fname)
        if not m:
            continue
        condition = m.group(1)
        seed = int(m.group(2))
        with open(path) as f:
            for line in f:
                line = line.strip()
                if not line:
                    continue
                try:
                    e = json.loads(line)
                except json.JSONDecodeError:
                    continue
                prob = Path(e["problem"]).stem
                # keep the LAST entry per problem (in case of re-run)
                data[(seed, condition)][prob] = e
    return data


def summarize_run(entries: dict[str, dict]) -> dict:
    """Per-run summary."""
    # Classify each problem
    solved = [p for p, e in entries.items() if e.get("has_golden_path")]
    fail_proper = [p for p, e in entries.items()
                   if not e.get("has_golden_path") and e.get("halt_reason") == "MaxTxExhausted"]
    measurement_error = [p for p, e in entries.items()
                         if e.get("halt_reason") not in ("OmegaAccepted", "MaxTxExhausted")
                         and not e.get("has_golden_path")]
    pput_sum = sum(e.get("pput", 0.0) for e in entries.values())
    pput_solved = [e["pput"] for e in entries.values() if e.get("has_golden_path")]
    return {
        "n_total": len(entries),
        "n_solved": len(solved),
        "n_fail_proper": len(fail_proper),
        "n_measurement_error": len(measurement_error),
        "solved_problems": sorted(solved),
        "measurement_error_problems": sorted(measurement_error),
        "sigma_pput": round(pput_sum, 4),
        "mean_pput_solved": round(sum(pput_solved) / len(pput_solved), 4) if pput_solved else 0.0,
        "build_shas": sorted({e.get("build_sha", "?") for e in entries.values()}),
    }


def paired_mcnemar(entries_x: dict[str, dict], entries_y: dict[str, dict], label_x: str, label_y: str):
    """Per-problem paired comparison."""
    common = sorted(set(entries_x) & set(entries_y))
    b = c = concord_solved = concord_fail = 0
    for p in common:
        ex, ey = entries_x[p], entries_y[p]
        sx = bool(ex.get("has_golden_path"))
        sy = bool(ey.get("has_golden_path"))
        # exclude problems where either side is MEASUREMENT_ERROR
        if ex.get("halt_reason") not in ("OmegaAccepted", "MaxTxExhausted"):
            continue
        if ey.get("halt_reason") not in ("OmegaAccepted", "MaxTxExhausted"):
            continue
        if sx and not sy:
            b += 1
        elif sy and not sx:
            c += 1
        elif sx and sy:
            concord_solved += 1
        else:
            concord_fail += 1
    return {
        "label_x": label_x, "label_y": label_y,
        "n_common_valid": b + c + concord_solved + concord_fail,
        "b_only_x": b, "c_only_y": c,
        "concord_solved": concord_solved, "concord_fail": concord_fail,
        "mcnemar_one_sided_x_gt_y": round(mcnemar_exact(b, c, one_sided=True), 6),
        "mcnemar_one_sided_y_gt_x": round(mcnemar_exact(c, b, one_sided=True), 6),
        "mcnemar_two_sided": round(mcnemar_exact(b, c, one_sided=False), 6),
    }


def pool_entries_by_condition(data):
    """Returns dict[condition] = combined {(seed, prob): entry}."""
    out = {cond: {} for cond in CONDITIONS}
    for (seed, cond), problem_map in data.items():
        for prob, entry in problem_map.items():
            out[cond][(seed, prob)] = entry
    return out


def pooled_mcnemar(pooled, cond_x, cond_y):
    ex = pooled[cond_x]
    ey = pooled[cond_y]
    common = sorted(set(ex) & set(ey))
    b = c = concord_solved = concord_fail = 0
    for k in common:
        ix, iy = ex[k], ey[k]
        if ix.get("halt_reason") not in ("OmegaAccepted", "MaxTxExhausted"):
            continue
        if iy.get("halt_reason") not in ("OmegaAccepted", "MaxTxExhausted"):
            continue
        sx = bool(ix.get("has_golden_path"))
        sy = bool(iy.get("has_golden_path"))
        if sx and not sy:
            b += 1
        elif sy and not sx:
            c += 1
        elif sx and sy:
            concord_solved += 1
        else:
            concord_fail += 1
    n_family = 4  # B-v-A, Abl-v-A, B-v-Abl, global
    alpha_bonf = 0.05 / n_family
    return {
        "cond_x": cond_x, "cond_y": cond_y,
        "n_common_valid": b + c + concord_solved + concord_fail,
        "b_x_only": b, "c_y_only": c,
        "concord_solved": concord_solved, "concord_fail": concord_fail,
        "mcnemar_one_sided_x_gt_y": round(mcnemar_exact(b, c, one_sided=True), 6),
        "mcnemar_two_sided": round(mcnemar_exact(b, c, one_sided=False), 6),
        "bonferroni_alpha": alpha_bonf,
        "rejects_null_one_sided_at_bonf": mcnemar_exact(b, c, one_sided=True) < alpha_bonf,
    }


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--logs", required=True, help="glob pattern for E1v2_*_n8_*.jsonl")
    ap.add_argument("--out", required=True, help="output JSON path")
    args = ap.parse_args()

    data = load_logs(args.logs)
    if not data:
        print(f"no logs matched {args.logs}")
        return 1

    # Per-seed-per-condition summary
    per_run = {}
    for (seed, cond), entries in sorted(data.items()):
        per_run[f"seed{seed}_{cond}"] = summarize_run(entries)

    # Per-seed McNemar: B vs A, Abl vs A, B vs Abl
    per_seed_pairs = {}
    for seed in SEEDS:
        if (seed, "A") in data and (seed, "B") in data:
            per_seed_pairs[f"seed{seed}_BvA"] = paired_mcnemar(data[(seed, "B")], data[(seed, "A")], "B", "A")
        if (seed, "Abl") in data and (seed, "A") in data:
            per_seed_pairs[f"seed{seed}_AblvA"] = paired_mcnemar(data[(seed, "Abl")], data[(seed, "A")], "Abl", "A")
        if (seed, "B") in data and (seed, "Abl") in data:
            per_seed_pairs[f"seed{seed}_BvAbl"] = paired_mcnemar(data[(seed, "B")], data[(seed, "Abl")], "B", "Abl")

    # Pooled (across seeds) McNemar with Bonferroni
    pooled = pool_entries_by_condition(data)
    pooled_tests = {
        "B_vs_A":    pooled_mcnemar(pooled, "B",   "A"),
        "Abl_vs_A":  pooled_mcnemar(pooled, "Abl", "A"),
        "B_vs_Abl":  pooled_mcnemar(pooled, "B",   "Abl"),
    }

    # Overall solve totals
    totals = {cond: sum(1 for v in pooled[cond].values() if v.get("has_golden_path")) for cond in CONDITIONS}
    totals_n = {cond: len(pooled[cond]) for cond in CONDITIONS}
    totals_merr = {cond: sum(1 for v in pooled[cond].values()
                             if v.get("halt_reason") not in ("OmegaAccepted", "MaxTxExhausted")
                             and not v.get("has_golden_path"))
                   for cond in CONDITIONS}

    # Meta-Planner contribution per seed (B - Abl)
    mp_contrib = {}
    for seed in SEEDS:
        if (seed, "B") in data and (seed, "Abl") in data:
            b_solved = sum(1 for e in data[(seed, "B")].values() if e.get("has_golden_path"))
            abl_solved = sum(1 for e in data[(seed, "Abl")].values() if e.get("has_golden_path"))
            mp_contrib[f"seed{seed}"] = {"B_solved": b_solved, "Abl_solved": abl_solved,
                                         "contrib": b_solved - abl_solved}

    out = {
        "generated_at": None,  # filled by caller via shell if needed
        "prereg": "handover/preregistration/PREREG_E1V2_HETEROGENEITY_2026-04-23.md",
        "seeds_seen": sorted({s for (s, _) in data.keys()}),
        "conditions_seen": sorted({c for (_, c) in data.keys()}),
        "per_run": per_run,
        "per_seed_pairs": per_seed_pairs,
        "pooled_tests": pooled_tests,
        "totals_solved_by_condition": totals,
        "totals_n_by_condition": totals_n,
        "totals_measurement_error_by_condition": totals_merr,
        "meta_planner_contribution_by_seed": mp_contrib,
    }

    Path(args.out).write_text(json.dumps(out, indent=2))
    print(f"wrote {args.out}")
    print(json.dumps({
        "seeds_seen": out["seeds_seen"],
        "totals_solved": totals,
        "pooled_B_vs_A_one_sided_p": pooled_tests["B_vs_A"]["mcnemar_one_sided_x_gt_y"],
        "pooled_Abl_vs_A_one_sided_p": pooled_tests["Abl_vs_A"]["mcnemar_one_sided_x_gt_y"],
        "meta_planner_contribution": mp_contrib,
    }, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
