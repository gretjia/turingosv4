#!/usr/bin/env python3
"""prereg_check.py — validate a PREREG file has all mandatory sections.

Per C-070. Usage:
    python3 tools/prereg_check.py handover/preregistration/PREREG_E1V2_*.md

Exit 0 on pass; 1 on missing field; 2 on fingerprint mismatch.
"""
import sys
import re
import hashlib
from pathlib import Path

MANDATORY = [
    "experiment_id",
    "date_created",
    "committed_commit_sha",
    "primary_endpoint",
    "directional_hypothesis",
    "sample_construction",
    "stopping_rule",
    "multiplicity_family",
    "what_would_falsify",
]

PRIMARY_SUBKEYS = ["statistic", "sample", "threshold"]
SAMPLE_SUBKEYS = ["source_pool", "pool_construction_rule", "selection_rule", "fingerprint"]


def fingerprint_of(path: Path) -> str:
    h = hashlib.sha256()
    for line in path.read_text().splitlines():
        line = line.strip()
        if line and not line.startswith("#"):
            h.update(line.encode() + b"\n")
    return h.hexdigest()[:16]


def main() -> int:
    if len(sys.argv) != 2:
        print("usage: prereg_check.py <PREREG_file>", file=sys.stderr)
        return 1
    prereg = Path(sys.argv[1])
    if not prereg.exists():
        print(f"missing: {prereg}", file=sys.stderr)
        return 1

    text = prereg.read_text()

    # Check top-level mandatory keys (either `key:` at line start OR `## key` markdown header)
    def has_key(k: str) -> bool:
        return bool(
            re.search(rf"^{re.escape(k)}\s*:", text, re.M)
            or re.search(rf"^##\s+{re.escape(k)}\b", text, re.M)
        )
    missing = [k for k in MANDATORY if not has_key(k)]
    if missing:
        print(f"MISSING mandatory keys: {missing}", file=sys.stderr)
        return 1

    # Check primary_endpoint subkeys
    pri_block_match = (
        re.search(r"^##\s+primary_endpoint\b(.+?)(?=^##\s+\w+|\Z)", text, re.M | re.S)
        or re.search(r"^primary_endpoint\s*:(.+?)(?=^\w+\s*:|\Z)", text, re.M | re.S)
    )
    if pri_block_match:
        pri = pri_block_match.group(1)
        miss = [k for k in PRIMARY_SUBKEYS if not re.search(rf"\b{k}\s*:", pri)]
        if miss:
            print(f"primary_endpoint missing subkeys: {miss}", file=sys.stderr)
            return 1

    # Check sample_construction subkeys + fingerprint
    samp_block_match = (
        re.search(r"^##\s+sample_construction\b(.+?)(?=^##\s+\w+|\Z)", text, re.M | re.S)
        or re.search(r"^sample_construction\s*:(.+?)(?=^\w+\s*:|\Z)", text, re.M | re.S)
    )
    if samp_block_match:
        samp = samp_block_match.group(1)
        miss = [k for k in SAMPLE_SUBKEYS if not re.search(rf"\b{k}\s*:", samp)]
        if miss:
            print(f"sample_construction missing subkeys: {miss}", file=sys.stderr)
            return 1
        # Cross-check fingerprint if sample file referenced
        sample_file_match = re.search(r"selection_script\s*:\s*([^\n#]+)", samp)
        fingerprint_match = re.search(r"fingerprint\s*:\s*([0-9a-f]+)", samp)
        # Parse sample_construction to find the actual sample file path
        sample_path_match = re.search(
            r"sample\s*:\s*([^\n#]+)",
            pri_block_match.group(1) if pri_block_match else ""
        )
        if sample_path_match and fingerprint_match:
            sample_fname = sample_path_match.group(1).strip().strip('"').strip("'")
            # Resolve relative to repo root
            repo_root = Path(__file__).resolve().parent.parent
            candidates = list(repo_root.rglob(sample_fname)) + list(repo_root.rglob(f"*{sample_fname}"))
            if candidates:
                actual_fp = fingerprint_of(candidates[0])
                claimed_fp = fingerprint_match.group(1).strip()
                if not actual_fp.startswith(claimed_fp) and not claimed_fp.startswith(actual_fp):
                    print(f"FINGERPRINT MISMATCH: claimed={claimed_fp} actual={actual_fp} for {candidates[0]}",
                          file=sys.stderr)
                    return 2

    print(f"PREREG check PASS: {prereg.name}")
    eid_match = re.search(r"experiment_id\s*:\s*([^\n#]+)", text)
    if eid_match:
        print(f"  experiment_id: {eid_match.group(1).strip()}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
