#!/usr/bin/env python3
"""Alignment coverage % — Karpathy-loop ad-hoc audit (2026-04-29 install).

Counts (covered / total) pub symbols in src/**/*.rs where covered = has a
`/// TRACE_MATRIX <id>: <role>` doc-comment within the 5 lines above the
pub line. This is the Karpathy-loop side-effect-of-TBs metric tracking
TuringOS v4 progress toward 100% constitution + whitepaper alignment.

R-022 hook (CO1.13.2) enforces the invariant on NEW pubs at commit-time;
this script gives the cumulative baseline against legacy pubs.

Usage:
  scripts/alignment_coverage.py        # human-readable summary
  scripts/alignment_coverage.py --json # machine-readable JSON
"""
from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SRC = ROOT / "src"

# Match official TRACE_MATRIX_v3 § E.2 denominator: only fn/struct/enum/trait/const.
# Excludes mod (organizational), type (alias), static (rare, mostly consts).
# Forward-lenient: looks 5 lines above the pub line for /// TRACE_MATRIX backlink
# (vs official semantic-block walk which is stricter). Drift typically <5 pct
# points; trend is monotone-equivalent. See README in handover/tracer_bullets/.
PUB_RE = re.compile(
    r"^\s*pub(?:\([^)]*\))?\s+"
    r"(?:async\s+|unsafe\s+|extern\s+\"\w+\"\s+)*"
    r"(fn|struct|enum|trait|const)\s+(\w+)"
)
TRACE_RE = re.compile(r"^\s*///\s*TRACE_MATRIX\s+")


def scan() -> tuple[int, int, list[str]]:
    total = 0
    covered = 0
    missing: list[str] = []
    for rs in sorted(SRC.rglob("*.rs")):
        try:
            lines = rs.read_text().splitlines()
        except OSError:
            continue
        for i, line in enumerate(lines):
            m = PUB_RE.match(line)
            if not m:
                continue
            total += 1
            kind, name = m.group(1), m.group(2)
            has_backlink = any(
                TRACE_RE.match(lines[j])
                for j in range(max(0, i - 5), i)
            )
            if has_backlink:
                covered += 1
            else:
                rel = rs.relative_to(ROOT)
                missing.append(f"{rel}:{i+1} pub {kind} {name}")
    return total, covered, missing


def main() -> None:
    total, covered, missing = scan()
    pct = (covered / total) if total else 0.0
    if "--json" in sys.argv:
        print(json.dumps({
            "total_pub_syms": total,
            "covered_pub_syms": covered,
            "alignment_coverage_pct": round(pct * 100, 2),
            "missing_count": len(missing),
            "missing_top10": missing[:10],
        }, indent=2))
        return
    print(f"alignment_coverage: {covered}/{total} = {pct*100:.2f}%")
    print(f"missing: {len(missing)} pub symbols (showing top 10):")
    for m in missing[:10]:
        print(f"  {m}")
    if len(missing) > 10:
        print(f"  ... and {len(missing) - 10} more")


if __name__ == "__main__":
    main()
