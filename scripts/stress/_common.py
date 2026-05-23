"""TB-STRESS-PHASE-2 — shared helper module for stress-test runners.

Each runner writes evidence under handover/evidence/stress_<id>_<UTC_TS>/
with summary.md + run.log + any binary captures. KILL line is the final
line of summary.md so the orchestrator can grep it cheaply.
"""
from __future__ import annotations

import hashlib
import json
import os
import subprocess
import sys
import time
from datetime import datetime, timezone
from pathlib import Path

PROJECT_ROOT = Path(__file__).resolve().parent.parent.parent


def ts_utc() -> str:
    return datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ")


def evidence_dir(test_id: str) -> Path:
    d = PROJECT_ROOT / "handover" / "evidence" / f"stress_{test_id}_{ts_utc()}"
    d.mkdir(parents=True, exist_ok=True)
    return d


def write_summary(evid: Path, *, test_id: str, kill_pass: bool, lines: list[str]) -> None:
    body = [
        f"# {test_id} — TB-STRESS-PHASE-2",
        "",
        f"Timestamp (UTC): {ts_utc()}",
        f"Evidence dir: {evid}",
        "",
        "## Run notes",
        *lines,
        "",
        "## KILL",
        "PASS" if kill_pass else "FAIL",
        "",
    ]
    (evid / "summary.md").write_text("\n".join(body))


def sha256_file(p: Path) -> str:
    h = hashlib.sha256()
    h.update(p.read_bytes())
    return h.hexdigest()


def run_cmd(cmd: list[str], *, cwd: Path | None = None, timeout: float = 60.0, env_extra: dict[str, str] | None = None) -> tuple[int, str, str]:
    env = os.environ.copy()
    if env_extra:
        env.update(env_extra)
    try:
        p = subprocess.run(
            cmd, cwd=cwd or PROJECT_ROOT, env=env, capture_output=True,
            text=True, timeout=timeout,
        )
        return p.returncode, p.stdout, p.stderr
    except subprocess.TimeoutExpired as e:
        return -1, e.stdout or "", e.stderr or "timeout"


def cargo_bin_path(name: str, *, release: bool = False) -> Path:
    sub = "release" if release else "debug"
    return PROJECT_ROOT / "target" / sub / name


def ensure_built(bin_name: str, *, release: bool = False) -> Path:
    """Build a binary if missing; return path."""
    p = cargo_bin_path(bin_name, release=release)
    if p.exists():
        return p
    cmd = ["cargo", "build", "--bin", bin_name, "--quiet"]
    if release:
        cmd.append("--release")
    print(f"  building {bin_name}...", file=sys.stderr)
    rc = subprocess.call(cmd, cwd=PROJECT_ROOT)
    if rc != 0:
        raise RuntimeError(f"cargo build {bin_name} failed (rc={rc})")
    return p
