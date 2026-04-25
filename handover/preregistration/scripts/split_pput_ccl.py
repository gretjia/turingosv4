#!/usr/bin/env python3
"""
PPUT-CCL three-split generator (Phase A2)

Per PREREG_PPUT_CCL_2026-04-26.md § 2.

Deterministic, hash-based 60/20/20 split of MiniF2F/Test (244 problems) into:
  adaptation       (~146 problems) — Phase B-D iteration
  meta_validation  (~49 problems)  — Phase D internal CCL audit
  heldout          (~49 problems)  — Phase E sealed eval (sole touch)

Seed string is FROZEN at commit time of this script. Re-running this script must
produce byte-identical output forever (assuming the source pool is unchanged).

Source pool: MiniF2F/Test only (244 problems).
MiniF2F/Valid (also 244) is reserved as out-of-distribution heldout for future arcs.

Usage:
  python3 handover/preregistration/scripts/split_pput_ccl.py

Output:
  handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json

Pre-conditions enforced:
  - source pool fingerprint matches expected (printed; compare to genesis_payload)
  - SEED string is the canonical "20260426_PPUT_CCL"
  - bucket counts hit (~146 / ~49 / ~49) within ±5; if not, abort

Trust Root note: this script + its output JSON are part of Trust Root per
PREREG § 1.8. Modification post-Phase-A5 commit gate triggers BLOCKER + arc
respin per C-070.
"""
from __future__ import annotations

import hashlib
import json
import os
import sys
from pathlib import Path

SEED: str = "20260426_PPUT_CCL"
SOURCE_DIR: str = "/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/MiniF2F/Test"
OUTPUT_PATH: str = "handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json"

# (low_inclusive, high_exclusive, bucket_name)
# n = sha256(SEED + ":" + pid)[0:8] hex → int → mod 100
BUCKET_RANGES: list[tuple[int, int, str]] = [
    (0, 60, "adaptation"),
    (60, 80, "meta_validation"),
    (80, 100, "heldout"),
]


def bucket_for(pid: str, seed: str = SEED) -> str:
    h = hashlib.sha256(f"{seed}:{pid}".encode("utf-8")).hexdigest()
    n = int(h[:8], 16) % 100
    for low, high, name in BUCKET_RANGES:
        if low <= n < high:
            return name
    raise RuntimeError(f"unreachable: bucket index {n} out of range")


def sha256_of_sorted_list(items: list[str]) -> str:
    """SHA-256 of sorted-newline-joined items, no trailing newline. Canonical form."""
    canonical = "\n".join(sorted(items)).encode("utf-8")
    return hashlib.sha256(canonical).hexdigest()


def main() -> int:
    if not Path(SOURCE_DIR).is_dir():
        print(f"ERROR: source dir not found: {SOURCE_DIR}", file=sys.stderr)
        print("       expected MiniF2F/Test directory at v3 path", file=sys.stderr)
        return 2

    raw_files = sorted(os.listdir(SOURCE_DIR))
    pids = [f[:-5] for f in raw_files if f.endswith(".lean")]
    if not pids:
        print(f"ERROR: no .lean files found in {SOURCE_DIR}", file=sys.stderr)
        return 2

    pids.sort()
    pool_fingerprint = sha256_of_sorted_list(pids)
    print(f"source pool: {len(pids)} problems")
    print(f"source pool SHA-256: {pool_fingerprint}")

    if len(pids) != 244:
        print(
            f"ERROR: expected 244 problems in MiniF2F/Test, got {len(pids)}",
            file=sys.stderr,
        )
        return 3

    buckets: dict[str, list[str]] = {"adaptation": [], "meta_validation": [], "heldout": []}
    for pid in pids:
        buckets[bucket_for(pid)].append(pid)

    for k in buckets:
        buckets[k].sort()

    # Sanity gate per script docstring: ±5 of nominal 146/49/49
    expected = {"adaptation": 146, "meta_validation": 49, "heldout": 49}
    for name, exp in expected.items():
        got = len(buckets[name])
        if abs(got - exp) > 5:
            print(
                f"ERROR: bucket {name} count {got} deviates from nominal {exp} by > 5",
                file=sys.stderr,
            )
            return 4
        print(f"  {name:18s}: {got:3d} (nominal {exp})")

    splits_payload: dict = {
        "schema_version": "1.0",
        "preregistered_in": "handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md",
        "generator_script": "handover/preregistration/scripts/split_pput_ccl.py",
        "seed": SEED,
        "source_dir_canonical": "MiniF2F/Test (244 problems)",
        "source_pool_size": len(pids),
        "source_pool_sha256": pool_fingerprint,
        "bucket_rule": (
            "n = int(sha256(SEED + ':' + pid)[:8], 16) %% 100; "
            "n<60 -> adaptation; 60<=n<80 -> meta_validation; n>=80 -> heldout"
        ),
        "splits": {
            name: {
                "count": len(buckets[name]),
                "sha256": sha256_of_sorted_list(buckets[name]),
                "problem_ids": buckets[name],
            }
            for name in ("adaptation", "meta_validation", "heldout")
        },
        "heldout_sealed_hash": sha256_of_sorted_list(buckets["heldout"]),
        "trust_root_note": (
            "This file + heldout_sealed_hash are part of Trust Root per "
            "PREREG_PPUT_CCL_2026-04-26.md § 1.8. Tampering = BLOCKER."
        ),
    }

    out_path = Path(OUTPUT_PATH)
    out_path.parent.mkdir(parents=True, exist_ok=True)
    with out_path.open("w", encoding="utf-8") as f:
        json.dump(splits_payload, f, indent=2, ensure_ascii=False)
        f.write("\n")

    print(f"\nwrote {out_path}")
    print(f"heldout sealed hash: {splits_payload['heldout_sealed_hash']}")
    print("\nNEXT STEP: copy heldout_sealed_hash + source_pool_sha256 into")
    print("          genesis_payload.toml [pput_accounting_0] section (Phase B7).")
    return 0


if __name__ == "__main__":
    sys.exit(main())
