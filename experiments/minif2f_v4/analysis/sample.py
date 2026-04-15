#!/usr/bin/env python3
"""Stratified deterministic sample from MiniF2F test set.

External seed provenance:
  CoinGecko bitcoin/usd @ 2026-04-15T00:57Z = 74677
  Kraken XBTUSD cross-check: 74716.50 (thousand-digit consistent)
  Researcher (Claude) did NOT choose this number.

Hamilton apportionment ensures integer quotas sum exactly to N_SAMPLE.

Usage:
  python sample.py                  # produces sample_N50_S74677.txt
  python sample.py --verify         # re-runs and checks reproducibility
  python sample.py --fixture-test   # runs self-test with seed=42
"""
import os, sys, random, argparse, hashlib
from pathlib import Path
from collections import Counter

EXTERNAL_SEED = 74677
N_SAMPLE = 50

# Hamilton quotas (pre-computed from v3.1 plan, frozen)
QUOTAS = {
    'mathd_algebra': 14,
    'mathd_numbertheory': 12,
    'amc': 9,
    'algebra': 4,
    'imo': 4,
    'induction': 2,
    'aime': 3,
    'numbertheory': 2,
}
assert sum(QUOTAS.values()) == N_SAMPLE, "Quota sum != N_SAMPLE"


def classify(name: str) -> str:
    n = name.lower()
    # IMPORTANT: longest prefixes first
    for pref in ('mathd_algebra', 'mathd_numbertheory'):
        if n.startswith(pref):
            return pref
    for pref in ('aime', 'induction', 'imo', 'numbertheory', 'algebra'):
        if n.startswith(pref):
            return pref
    if n.startswith('amc'):
        return 'amc'
    return 'other'


def enumerate_test(minif2f_dir: str) -> list[str]:
    test_dir = Path(minif2f_dir) / 'MiniF2F' / 'Test'
    names = sorted(f.stem for f in test_dir.glob('*.lean'))
    assert len(names) == 244, f"Expected 244 test problems, got {len(names)}"
    return names


def stratified_sample(names: list[str], seed: int, quotas: dict) -> list[str]:
    """Deterministic stratified sample using Python random with given seed."""
    rng = random.Random(seed)
    by_cat = {cat: [] for cat in quotas}
    for n in names:
        c = classify(n)
        if c in by_cat:
            by_cat[c].append(n)
    result = []
    for cat, quota in quotas.items():
        pool = by_cat[cat]
        if len(pool) < quota:
            raise ValueError(f"Category {cat} pool={len(pool)} < quota={quota}")
        chosen = rng.sample(pool, quota)
        result.extend(sorted(chosen))
    return result


def fingerprint(items: list[str]) -> str:
    """Hash of sorted problem names — for freeze verification."""
    h = hashlib.sha256()
    for s in sorted(items):
        h.update(s.encode())
        h.update(b'\n')
    return h.hexdigest()[:16]


def fixture_test():
    """Verify determinism + quota math + classifier on a known fixture."""
    # Use a synthetic pool to avoid dependency on real minif2f files
    pool = [f'{cat}_{i:03d}' for cat in QUOTAS for i in range(100)]
    # Synthetic "numbertheory" collides with "mathd_numbertheory" prefix — guard
    # the classifier test separately:
    assert classify('mathd_numbertheory_001') == 'mathd_numbertheory', "prefix order"
    assert classify('numbertheory_001') == 'numbertheory', "prefix order"
    assert classify('mathd_algebra_999') == 'mathd_algebra', "prefix order"
    assert classify('algebra_xyz') == 'algebra', "prefix order"
    assert classify('amc12a_2002_p6') == 'amc', "amc variants"

    # Hamilton quotas: sum check
    total_proportional = 70+60+45+18+20+8+15+8
    assert total_proportional == 244
    assert sum(QUOTAS.values()) == 50

    # Determinism: same seed → same sample
    s1 = stratified_sample(pool, seed=42, quotas=QUOTAS)
    s2 = stratified_sample(pool, seed=42, quotas=QUOTAS)
    assert s1 == s2, "determinism broken"

    # Different seeds → different samples
    s3 = stratified_sample(pool, seed=43, quotas=QUOTAS)
    assert s1 != s3, "seed sensitivity broken"

    # Quotas respected per category
    counts = Counter(classify(x) for x in s1)
    for cat, q in QUOTAS.items():
        assert counts[cat] == q, f"quota mismatch {cat}: {counts[cat]} != {q}"

    print("fixture_test: all assertions passed")
    return True


def main():
    p = argparse.ArgumentParser()
    p.add_argument('--minif2f-dir', default=os.environ.get('MINIF2F_DIR',
                   '/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4'))
    p.add_argument('--output', default=None)
    p.add_argument('--fixture-test', action='store_true')
    p.add_argument('--verify', action='store_true')
    args = p.parse_args()

    if args.fixture_test:
        ok = fixture_test()
        sys.exit(0 if ok else 1)

    names = enumerate_test(args.minif2f_dir)
    sample = stratified_sample(names, EXTERNAL_SEED, QUOTAS)
    fp = fingerprint(sample)

    out_path = args.output or f'/home/zephryj/projects/turingosv4/experiments/minif2f_v4/analysis/sample_N{N_SAMPLE}_S{EXTERNAL_SEED}.txt'

    if args.verify:
        # Compare against existing file
        if not os.path.exists(out_path):
            print(f"No existing sample at {out_path}; nothing to verify")
            sys.exit(2)
        existing = [l for l in Path(out_path).read_text().strip().split('\n')
                    if l and not l.startswith('#')]
        assert existing == sample, f"Sample drifted!\nexisting[0..3]={existing[:3]}\nsample[0..3]={sample[:3]}"
        print(f"verify: MATCH (fp={fp}, N={len(sample)})")
        sys.exit(0)

    with open(out_path, 'w') as f:
        f.write(f"# Sample N={N_SAMPLE} seed={EXTERNAL_SEED} (external: BTC/USD @ 2026-04-15T00:57Z)\n")
        f.write(f"# Fingerprint: {fp}\n")
        for n in sample:
            f.write(n + '\n')
    print(f"Wrote {len(sample)} problems to {out_path}")
    print(f"Fingerprint: {fp}")

    # Show category breakdown
    counts = Counter(classify(x) for x in sample)
    for cat in QUOTAS:
        print(f"  {cat}: {counts[cat]}")


if __name__ == '__main__':
    main()
