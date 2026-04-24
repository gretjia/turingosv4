#!/usr/bin/env python3
"""ablation_gate.py — enforce N≥3 seeds for any causal claim (C-070).

Usage:
    python3 tools/ablation_gate.py --claim "<claim text>" \
        --seeds handover/evidence/e1_jsonl/E1_ablation_*.jsonl

Exit 0 if ≥ 3 unique boltzmann_seeds found in provided jsonls; exit 1 otherwise.

This is a GATE, not a validator — meant to run as a pre-commit hook before
drafts containing causal language are committed.
"""
import argparse
import glob
import json
import sys


def main() -> int:
    p = argparse.ArgumentParser()
    p.add_argument("--claim", required=False, help="claim text (informational)")
    p.add_argument("--seeds", nargs="+", required=True, help="glob patterns for jsonl files")
    p.add_argument("--min-seeds", type=int, default=3, help="minimum unique seeds required")
    args = p.parse_args()

    expanded = []
    for pat in args.seeds:
        matches = glob.glob(pat)
        if not matches:
            print(f"WARN: no files match: {pat}", file=sys.stderr)
        expanded.extend(matches)

    if not expanded:
        print(f"FAIL: no jsonl files found", file=sys.stderr)
        return 1

    seeds = set()
    for fp in expanded:
        try:
            with open(fp) as f:
                for line in f:
                    d = json.loads(line)
                    s = d.get("boltzmann_seed")
                    if s is not None:
                        seeds.add(s)
        except Exception as e:
            print(f"WARN: could not parse {fp}: {e}", file=sys.stderr)

    n = len(seeds)
    claim = args.claim or "(no claim specified)"
    if n >= args.min_seeds:
        print(f"ABLATION GATE PASS: {n} unique seeds {sorted(seeds)} for claim: {claim}")
        return 0
    else:
        print(f"ABLATION GATE FAIL: only {n} unique seeds {sorted(seeds)}, need ≥ {args.min_seeds}. "
              f"Claim '{claim}' must be demoted to exploratory or more seeds added.",
              file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())
