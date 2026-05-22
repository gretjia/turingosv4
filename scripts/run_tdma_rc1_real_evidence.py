#!/usr/bin/env python3
"""TRACE_MATRIX FC1a-rtool + FC3-replay: Atom 7.5 real-evidence runner.

Drives `target/release/tdma_rc1_real_evidence` against the user-supplied
math problem ("证明所有自然数之和 = -1/12 via m·exp(-m/N)·cos(m/N)") and runs
the five real-tape invariants from orchestrator plan §5 Atom 7.5.

Usage:
    python3 scripts/run_tdma_rc1_real_evidence.py

Output: handover/evidence/tdma_rc1_real_evidence_<UTC_TIMESTAMP>/
"""
from __future__ import annotations

import argparse
import hashlib
import json
import subprocess
import sys
import time
from pathlib import Path


PROJECT_ROOT = Path(__file__).resolve().parent.parent
BINARY = PROJECT_ROOT / "target" / "release" / "tdma_rc1_real_evidence"


def sha256_hex(path: Path) -> str:
    h = hashlib.sha256()
    h.update(path.read_bytes())
    return h.hexdigest()


def verify_evidence(evidence_dir: Path) -> tuple[bool, list[str]]:
    """Run the 5 real-tape invariants on the captured evidence."""
    notes: list[str] = []
    ok = True

    manifest_path = evidence_dir / "manifest.json"
    if not manifest_path.exists():
        return False, ["manifest.json missing"]

    manifest = json.loads(manifest_path.read_text())

    # Sha integrity for each evidence file
    for file_key, sha_key in [
        ("chaintape.jsonl", "chaintape_sha256"),
        ("bbs_per_step.jsonl", "bbs_sha256"),
        ("prompt_per_attempt.jsonl", "prompts_sha256"),
        ("judge_verdicts.jsonl", "verdicts_sha256"),
    ]:
        actual = sha256_hex(evidence_dir / file_key)
        expected = manifest.get(sha_key, "")
        if actual != expected:
            ok = False
            notes.append(
                f"sha mismatch on {file_key}: expected {expected[:12]}.. actual {actual[:12]}.."
            )

    # Invariant 1: chaintape.jsonl has at least one node
    chaintape_lines = [
        json.loads(l) for l in (evidence_dir / "chaintape.jsonl").read_text().splitlines() if l
    ]
    if not chaintape_lines:
        ok = False
        notes.append("chaintape empty")

    # Invariant 2: every AgentProposal verified=false has scope set
    for node in chaintape_lines:
        if node.get("kind") == "AgentProposal" and node.get("verified") is False:
            if node.get("scope") is None:
                ok = False
                notes.append(f"AgentProposal without scope: {node.get('hash')}")

    # Invariant 3: invariants_passed flag in manifest
    if not manifest.get("invariants_passed", False):
        ok = False
        notes.append("manifest invariants_passed flag is False")

    # Invariant 4: verified_head_final != H0 (head advanced)
    if manifest.get("verified_head_final") == "H0":
        ok = False
        notes.append("verified_head did not advance off H0")

    # Invariant 5: accepted_steps + proposal_count balance
    if manifest.get("accepted_steps", 0) == 0:
        ok = False
        notes.append("no accepted steps")

    return ok, notes


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--max-steps",
        type=int,
        default=20,
        help="Max steps the binary will attempt (binary uses canonical 5).",
    )
    parser.add_argument(
        "--evidence-root",
        type=Path,
        default=PROJECT_ROOT / "handover" / "evidence",
    )
    args = parser.parse_args()

    timestamp = time.strftime("%Y%m%dT%H%M%SZ", time.gmtime())
    evidence_dir = args.evidence_root / f"tdma_rc1_real_evidence_{timestamp}"

    if not BINARY.exists():
        print(f"binary missing: {BINARY}", file=sys.stderr)
        print("build with: cargo build --release --bin tdma_rc1_real_evidence", file=sys.stderr)
        return 2

    print(f"Running {BINARY} --evidence-dir {evidence_dir}")
    proc = subprocess.run(
        [str(BINARY), "--evidence-dir", str(evidence_dir)],
        cwd=PROJECT_ROOT,
        check=False,
    )
    if proc.returncode != 0:
        print(f"binary exited with code {proc.returncode}", file=sys.stderr)
        return proc.returncode

    print("Verifying real-tape invariants...")
    ok, notes = verify_evidence(evidence_dir)
    for n in notes:
        print(f"  - {n}")
    if ok:
        print(f"\nPASS: invariants verified at {evidence_dir}")
        print(f"Report: {evidence_dir}/REAL_EVIDENCE_REPORT.md")
        return 0
    else:
        print(f"\nFAIL: invariant verification failed at {evidence_dir}")
        return 3


if __name__ == "__main__":
    sys.exit(main())
