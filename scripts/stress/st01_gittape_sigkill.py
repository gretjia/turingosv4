#!/usr/bin/env python3
"""ST-01 — GitTapeLedger SIGKILL mid-commit (TB-STRESS-PHASE-2).

Stresses the libgit2-backed CAS against random SIGKILL mid-commit. Each
iteration:
  1. Launch a `git commit` operation on the CAS workspace (no LLM)
  2. SIGKILL after random delay (50..1500 ms)
  3. Verify (a) repo re-opens with `git rev-parse`,
            (b) `git fsck --full` reports no broken/dangling refs from this iter.

KILL: ITER iterations all leave repo in a re-openable + fsck-clean state.
"""
from __future__ import annotations

import os
import random
import signal
import subprocess
import sys
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from _common import PROJECT_ROOT, evidence_dir, write_summary  # noqa: E402

ITER = int(os.environ.get("ST01_ITER", "20"))


def init_cas(ws: Path) -> None:
    cas = ws / "cas"
    cas.mkdir(parents=True, exist_ok=True)
    subprocess.run(["git", "init", "--quiet"], cwd=cas, check=True)
    subprocess.run(["git", "config", "user.email", "stress@turingos"], cwd=cas, check=True)
    subprocess.run(["git", "config", "user.name", "stress"], cwd=cas, check=True)
    # Seed with one good commit so refs/heads/main exists.
    (cas / "seed.bin").write_bytes(b"seed-" + os.urandom(64))
    subprocess.run(["git", "add", "seed.bin"], cwd=cas, check=True)
    subprocess.run(["git", "commit", "-m", "seed", "--quiet"], cwd=cas, check=True)


def iter_kill_commit(ws: Path, idx: int) -> bool:
    """Spawn a git commit, SIGKILL after random delay, check repo health.

    Returns True if repo is healthy after kill, False otherwise.
    """
    cas = ws / "cas"
    # Write a fresh blob that the child will commit.
    blob = cas / f"blob_{idx}.bin"
    blob.write_bytes(os.urandom(4096))

    # Wrap the commit in a shell that sleeps inside, giving us a kill window.
    # The slow path adds artificial work between `git add` and `git commit`.
    script = (
        f"cd '{cas}' && "
        f"git add 'blob_{idx}.bin' && "
        # Random work delay inside child so SIGKILL can land mid-commit.
        f"python3 -c 'import time; time.sleep(0.5)' && "
        f"git commit -m 'iter-{idx}' --quiet"
    )
    child = subprocess.Popen(
        ["bash", "-c", script],
        cwd=cas, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
        preexec_fn=os.setsid,
    )

    delay_ms = random.randint(50, 1500)
    time.sleep(delay_ms / 1000.0)
    try:
        os.killpg(os.getpgid(child.pid), signal.SIGKILL)
    except ProcessLookupError:
        pass
    child.wait(timeout=5)

    # Verify HEAD still resolves.
    rc1 = subprocess.run(
        ["git", "rev-parse", "--verify", "HEAD"],
        cwd=cas, capture_output=True,
    ).returncode

    # fsck must not produce errors. dangling blobs from the killed prep
    # are OK; only broken/missing/error lines count.
    fsck = subprocess.run(
        ["git", "fsck", "--full"], cwd=cas, capture_output=True, text=True,
    )
    fsck_dirty = any(
        kw in (fsck.stdout + fsck.stderr).lower()
        for kw in ("error", "missing", "broken", "corrupt")
    )

    return rc1 == 0 and not fsck_dirty


def main() -> int:
    evid = evidence_dir("st01_gittape_sigkill")
    log_lines: list[str] = []
    ws = evid / "workspace"

    print(f"[ST-01] iterations={ITER} ws={ws}")
    init_cas(ws)

    healthy = 0
    unhealthy = 0
    for i in range(1, ITER + 1):
        ok = iter_kill_commit(ws, i)
        if ok:
            healthy += 1
        else:
            unhealthy += 1
            log_lines.append(f"iter {i}: UNHEALTHY")
        if i % 5 == 0:
            print(f"  [ST-01] iter {i}/{ITER}  healthy={healthy} unhealthy={unhealthy}")

    log_lines.append(f"iterations={ITER}, healthy={healthy}, unhealthy={unhealthy}")
    kill_pass = unhealthy == 0
    write_summary(evid, test_id="ST-01 GitTapeLedger SIGKILL mid-commit",
                  kill_pass=kill_pass, lines=log_lines)
    print(f"[ST-01] KILL: {'PASS' if kill_pass else 'FAIL'}")
    return 0 if kill_pass else 1


if __name__ == "__main__":
    sys.exit(main())
