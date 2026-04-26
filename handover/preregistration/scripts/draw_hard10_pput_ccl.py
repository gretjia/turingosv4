#!/usr/bin/env python3
"""
PPUT-CCL hard-10 deterministic draw (Phase C C-pre1)

Per PREREG_PPUT_CCL_2026-04-26.md § 6 C2:
  random.Random("hard10_pput_ccl_seed").sample(adaptation_set, 10)

The 10 problem IDs + their fingerprint are the per-mode test sample for
Phase C ablation (5 modes × 10 problems × 2 Boltzmann seeds = 100 jsonl rows).

Seed string is FROZEN at commit time. Re-running this script must produce
byte-identical output forever (assuming PPUT_CCL_SPLITS_2026-04-26.json's
adaptation list is unchanged, which is itself FROZEN per PREREG § 1.8 / § 2.1).

Usage:
  python3 handover/preregistration/scripts/draw_hard10_pput_ccl.py

Output:
  handover/preregistration/PPUT_CCL_HARD10_2026-04-26.json

Trust Root note: this script + its output JSON enter Trust Root once committed.
"""
from __future__ import annotations

import hashlib
import json
import random
import sys
from pathlib import Path

SEED: str = "hard10_pput_ccl_seed"
SPLITS_PATH: str = "handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json"
OUTPUT_PATH: str = "handover/preregistration/PPUT_CCL_HARD10_2026-04-26.json"
EXPECTED_ADAPTATION_COUNT: int = 144
HARD_K: int = 10


def sha256_of_sorted_list(items: list[str]) -> str:
    """SHA-256 of sorted-newline-joined items, no trailing newline. Canonical form."""
    canonical = "\n".join(sorted(items)).encode("utf-8")
    return hashlib.sha256(canonical).hexdigest()


def main() -> int:
    splits_path = Path(SPLITS_PATH)
    if not splits_path.is_file():
        print(f"ERROR: splits file not found: {SPLITS_PATH}", file=sys.stderr)
        print(
            "       run handover/preregistration/scripts/split_pput_ccl.py first",
            file=sys.stderr,
        )
        return 2

    with splits_path.open("r", encoding="utf-8") as f:
        splits_payload = json.load(f)

    adaptation = splits_payload["splits"]["adaptation"]["problem_ids"]
    adaptation_sha256_committed = splits_payload["splits"]["adaptation"]["sha256"]
    adaptation_sha256_recomputed = sha256_of_sorted_list(adaptation)

    if adaptation_sha256_recomputed != adaptation_sha256_committed:
        print(
            "ERROR: adaptation sha256 drift — committed split has been tampered",
            file=sys.stderr,
        )
        print(f"  committed:  {adaptation_sha256_committed}", file=sys.stderr)
        print(f"  recomputed: {adaptation_sha256_recomputed}", file=sys.stderr)
        return 3

    if len(adaptation) != EXPECTED_ADAPTATION_COUNT:
        print(
            f"ERROR: adaptation count {len(adaptation)} != expected {EXPECTED_ADAPTATION_COUNT}",
            file=sys.stderr,
        )
        return 4

    # PREREG § 6 C2: random.Random("hard10_pput_ccl_seed").sample(adaptation_set, 10)
    # adaptation list is sorted in the splits JSON (see split_pput_ccl.py line ~98);
    # sorted-input feed is required for byte-determinism across machines / Python builds.
    rng = random.Random(SEED)
    drawn = rng.sample(sorted(adaptation), HARD_K)

    drawn_sorted = sorted(drawn)
    fingerprint = sha256_of_sorted_list(drawn_sorted)

    print(f"adaptation pool: {len(adaptation)} problems (sha256 {adaptation_sha256_recomputed})")
    print(f"hard-{HARD_K} draw seed: {SEED!r}")
    print(f"hard-{HARD_K} fingerprint (sorted+newline+sha256): {fingerprint}")
    print()
    print(f"hard-{HARD_K} sample (sorted):")
    for pid in drawn_sorted:
        print(f"  {pid}")

    payload: dict = {
        "schema_version": "1.0",
        "preregistered_in": "handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md",
        "preregistered_section": "§ 6 Phase C C2",
        "generator_script": "handover/preregistration/scripts/draw_hard10_pput_ccl.py",
        "seed": SEED,
        "draw_function": "random.Random(seed).sample(sorted(adaptation_problem_ids), 10)",
        "source_split_file": SPLITS_PATH,
        "source_split_bucket": "adaptation",
        "source_adaptation_count": len(adaptation),
        "source_adaptation_sha256": adaptation_sha256_recomputed,
        "hard_k": HARD_K,
        "hard_sample_sha256": fingerprint,
        "problem_ids": drawn_sorted,
        "boltzmann_seeds": [31415, 2718],
        "boltzmann_seeds_provenance": "PREREG § 6 C2 — frozen at PREREG round-4 commit",
        "trust_root_note": (
            "This file + hard_sample_sha256 are part of Trust Root per "
            "PREREG_PPUT_CCL_2026-04-26.md § 6 C2. Tampering = BLOCKER + Phase C respin."
        ),
    }

    out_path = Path(OUTPUT_PATH)
    out_path.parent.mkdir(parents=True, exist_ok=True)
    with out_path.open("w", encoding="utf-8") as f:
        json.dump(payload, f, indent=2, ensure_ascii=False)
        f.write("\n")

    print(f"\nwrote {out_path}")
    print(f"hard-{HARD_K} sealed hash: {fingerprint}")
    print()
    print("NEXT STEP: add this file to genesis_payload.toml [trust_root]")
    print("           and regenerate Trust Root SHA-256 for it (Phase C C-pre1).")
    return 0


if __name__ == "__main__":
    sys.exit(main())
