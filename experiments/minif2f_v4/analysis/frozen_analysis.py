#!/usr/bin/env python3
"""Frozen analysis for v3.1 experiment.

Primary metric: SolveRate = solves / N_sample (aborts = failures)
Secondary: Aggregate_PPUT = 100 * solves / sum(time_secs for ALL problems incl aborts)
Diagnostic: per-problem PPUT distribution

Pairwise rules:
  strict_win: |solves(A) - solves(B)| >= 3
  equivalent: |solves(A) - solves(B)| <= 1
  gray: gap of 2

Paired-subset: problems where ALL conditions completed (no abort/measurement_error).

Assertion: this script must NOT run inside evaluator (Art. III.4 Goodhart).
"""
import json, os, sys, argparse
from pathlib import Path
from collections import defaultdict

# Anti-Goodhart assertion
if os.environ.get('CONDITION'):
    raise RuntimeError(
        "frozen_analysis.py must NOT run inside evaluator context. "
        "CONDITION env is set, indicating in-loop execution. ABORT.")

ABORT_TIME = 900.0  # wall seconds (matches run_list.sh timeout)

def load_jsonl(path: str, expected_sample: list[str]) -> dict:
    """Load jsonl; return {problem_name: record or None (=abort)}."""
    results = {name: None for name in expected_sample}
    with open(path) as f:
        for line in f:
            line = line.strip()
            if not line: continue
            d = json.loads(line)
            name = d['problem'].split('/')[-1].replace('.lean', '')
            if name in results:
                results[name] = d
    return results


def compute(records: dict) -> dict:
    """Compute primary + secondary on full sample (aborts count as fail)."""
    total = len(records)
    solved = sum(1 for r in records.values() if r and r.get('has_golden_path'))
    total_time = sum((r['time_secs'] if r else ABORT_TIME) for r in records.values())
    solve_rate = solved / total if total else 0.0
    agg_pput = 100.0 * solved / total_time if total_time else 0.0
    return {
        'N': total,
        'solves': solved,
        'aborts': sum(1 for r in records.values() if r is None),
        'solve_rate': solve_rate,
        'aggregate_pput': agg_pput,
        'total_time': total_time,
    }


def paired_subset(conditions: dict) -> set:
    """Problems where ALL conditions completed (non-None)."""
    names = set(next(iter(conditions.values())).keys())
    for recs in conditions.values():
        names &= {n for n, r in recs.items() if r is not None}
    return names


def compute_paired(records: dict, subset: set) -> dict:
    sub = {n: r for n, r in records.items() if n in subset}
    solved = sum(1 for r in sub.values() if r.get('has_golden_path'))
    total_time = sum(r['time_secs'] for r in sub.values())
    return {
        'N_paired': len(sub),
        'solves_paired': solved,
        'solve_rate_paired': solved / len(sub) if sub else 0.0,
        'aggregate_pput_paired': 100.0 * solved / total_time if total_time else 0.0,
    }


def pairwise_verdict(a_solves: int, b_solves: int, label_a: str, label_b: str) -> str:
    diff = a_solves - b_solves
    if abs(diff) >= 3:
        winner = label_a if diff > 0 else label_b
        return f"STRICT_WIN {winner} (+{abs(diff)})"
    if abs(diff) <= 1:
        return f"EQUIVALENT (|Δ|={abs(diff)})"
    return f"GRAY (|Δ|=2, not conclusive)"


def fixture_test():
    """Self-test with synthetic jsonl."""
    import tempfile
    fixture = [
        {"problem": "/x/mathd_algebra_1.lean", "has_golden_path": True, "time_secs": 100, "pput": 1.0},
        {"problem": "/x/mathd_algebra_2.lean", "has_golden_path": True, "time_secs": 50, "pput": 2.0},
        {"problem": "/x/mathd_algebra_3.lean", "has_golden_path": False, "time_secs": 200, "pput": 0.0},
        {"problem": "/x/mathd_algebra_4.lean", "has_golden_path": True, "time_secs": 150, "pput": 0.66},
        {"problem": "/x/mathd_algebra_5.lean", "has_golden_path": False, "time_secs": 300, "pput": 0.0},
        # mathd_algebra_6 missing → abort
    ]
    with tempfile.NamedTemporaryFile('w', suffix='.jsonl', delete=False) as f:
        for d in fixture: f.write(json.dumps(d) + '\n')
        path = f.name
    sample = [f'mathd_algebra_{i}' for i in range(1, 7)]  # 6 problems
    recs = load_jsonl(path, sample)

    # Verify load
    assert recs['mathd_algebra_1']['has_golden_path'] == True
    assert recs['mathd_algebra_6'] is None, "abort detection"

    # Primary: 3 solved / 6 total = 0.5
    m = compute(recs)
    assert m['N'] == 6 and m['solves'] == 3 and m['aborts'] == 1
    assert abs(m['solve_rate'] - 0.5) < 1e-9
    # total_time = 100+50+200+150+300+900(abort) = 1700
    assert abs(m['total_time'] - 1700.0) < 1e-6
    # agg_pput = 100*3/1700 = 0.1764...
    assert abs(m['aggregate_pput'] - 100*3/1700) < 1e-9

    # Pairwise verdict
    assert "EQUIVALENT" in pairwise_verdict(10, 10, 'A', 'B')
    assert "STRICT_WIN A" in pairwise_verdict(15, 10, 'A', 'B')
    assert "GRAY" in pairwise_verdict(10, 8, 'A', 'B')

    os.unlink(path)
    print("fixture_test: all assertions passed")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--sample', required=False, default=None,
                    help='Path to sample.txt (names of expected problems)')
    ap.add_argument('--oneshot', help='oneshot jsonl')
    ap.add_argument('--n1', help='n1 jsonl')
    ap.add_argument('--n3', help='n3 jsonl')
    ap.add_argument('--fixture-test', action='store_true')
    args = ap.parse_args()

    if args.fixture_test:
        fixture_test()
        return

    if not (args.sample and args.oneshot and args.n1 and args.n3):
        print("Need --sample, --oneshot, --n1, --n3", file=sys.stderr)
        sys.exit(1)

    sample = [l for l in Path(args.sample).read_text().split('\n')
              if l and not l.startswith('#')]

    conds = {}
    for label, path in [('oneshot', args.oneshot), ('n1', args.n1), ('n3', args.n3)]:
        conds[label] = load_jsonl(path, sample)

    print(f"# Frozen Analysis v3.1 — N_sample={len(sample)}")
    print(f"# Sample seed=74677 (external source: BTC/USD @ 2026-04-15)")
    print()

    # Primary (all-sample, aborts = fail)
    print("## Primary: SolveRate (aborts counted as failures)\n")
    print(f"{'Condition':<10}{'N':>5}{'Solves':>8}{'Aborts':>8}{'SolveRate':>11}{'Agg_PPUT':>11}{'Σtime':>9}")
    for label, recs in conds.items():
        m = compute(recs)
        print(f"{label:<10}{m['N']:>5}{m['solves']:>8}{m['aborts']:>8}{m['solve_rate']:>11.3f}{m['aggregate_pput']:>11.3f}{int(m['total_time']):>9}")

    print("\n## Pairwise verdicts (SolveRate, N=50)\n")
    for a, b in [('n3','oneshot'), ('n1','oneshot'), ('n3','n1')]:
        sa = compute(conds[a])['solves']
        sb = compute(conds[b])['solves']
        print(f"  {a} vs {b}: {pairwise_verdict(sa, sb, a, b)}")

    # Secondary (paired subset)
    subset = paired_subset(conds)
    print(f"\n## Secondary: paired-subset analysis (problems where all 3 completed)\n")
    print(f"Paired N: {len(subset)}")
    print(f"{'Condition':<10}{'solves':>8}{'solve_rate':>12}{'agg_pput':>11}")
    for label, recs in conds.items():
        p = compute_paired(recs, subset)
        print(f"{label:<10}{p['solves_paired']:>8}{p['solve_rate_paired']:>12.3f}{p['aggregate_pput_paired']:>11.3f}")

    print("\n## Pairwise (paired-subset)\n")
    for a, b in [('n3','oneshot'), ('n1','oneshot'), ('n3','n1')]:
        sa = compute_paired(conds[a], subset)['solves_paired']
        sb = compute_paired(conds[b], subset)['solves_paired']
        print(f"  {a} vs {b}: {pairwise_verdict(sa, sb, a, b)}")


if __name__ == '__main__':
    main()
