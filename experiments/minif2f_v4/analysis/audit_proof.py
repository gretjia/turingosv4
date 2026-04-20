#!/usr/bin/env python3
"""
TuringOS v4 — Phase 0 audit proof re-verifier (C-039 candidate).

Reads a PPUT_RESULT jsonl, finds rows with `gp_proof_file`, and re-runs Lean 4
on each artifact independently of the runtime that produced them. This closes
the audit gap identified after the F-2026-04-20 plan: prior to Phase 0 the
winning proof text was discarded, so 86% solve claims were not externally
re-verifiable. After Phase 0 the artifact lives at
<EXPERIMENT_DIR>/proofs/<theorem>_<ts>_<hash>.lean and contains the full
problem + accepted proof.

Usage:
    python3 audit_proof.py logs/templadder_n8_TIMESTAMP.jsonl
    python3 audit_proof.py logs/...jsonl --sample 5     # spot-check 5 random
    python3 audit_proof.py logs/...jsonl --max 10       # cap to first 10

Exit code 0 = all checked solves re-verify; 1 = at least one mismatch (audit fail);
2 = setup error (missing Lean, missing Mathlib).
"""
import argparse
import json
import os
import random
import subprocess
import sys
import time
from pathlib import Path


def derive_lean_path(minif2f_dir: Path) -> str:
    """Walk .lake/packages/*/.lake/build/lib/lean (Lake 4 layout)."""
    paths = []
    lake = minif2f_dir / ".lake/packages"
    if lake.is_dir():
        for entry in lake.iterdir():
            for cand in [
                entry / ".lake/build/lib/lean",
                entry / "lib/lean",
            ]:
                if cand.is_dir():
                    paths.append(str(cand))
    project_lib = minif2f_dir / ".lake/build/lib/lean"
    if project_lib.is_dir():
        paths.append(str(project_lib))
    return ":".join(paths)


def find_lean_binary() -> str | None:
    explicit = os.environ.get("LEAN_BINARY")
    if explicit and Path(explicit).exists():
        return explicit
    home = os.environ.get("HOME", "/root")
    cand = f"{home}/.elan/toolchains/leanprover--lean4---v4.24.0/bin/lean"
    if Path(cand).exists():
        return cand
    # PATH fallback
    try:
        out = subprocess.run(["which", "lean"], capture_output=True, text=True, timeout=5)
        if out.returncode == 0:
            return out.stdout.strip()
    except Exception:
        pass
    return None


# F-2026-04-20-05: external audit must enforce C-011 forbidden patterns
# independent of Lean (Lean accepts `native_decide` but we don't).
# Mirror lean4_oracle.rs FORBIDDEN_PATTERNS.
FORBIDDEN_PATTERNS = (
    "#eval", "#check", "#reduce", "#exec", "#print",
    "native_decide",
    "IO.Process", "IO.FS", "System.FilePath",
    "run_tac", "unsafe", "dbg_trace", "IO.println",
)


def reverify(lean_bin: str, lean_path: str, proof_file: Path, timeout_s: int = 300) -> tuple[bool, str]:
    """Spawn lean --stdin < proof_file and return (ok, detail)."""
    try:
        with open(proof_file) as f:
            code = f.read()
    except Exception as e:
        return False, f"cannot read artifact: {e}"
    # Check forbidden patterns before Lean runs (C-011 / F-20-05 parity with oracle).
    for pat in FORBIDDEN_PATTERNS:
        if pat in code:
            return False, f"forbidden_pattern: {pat}"
    env = dict(os.environ)
    env["LEAN_PATH"] = lean_path
    try:
        proc = subprocess.run(
            [lean_bin, "--stdin"],
            input=code,
            capture_output=True,
            text=True,
            env=env,
            timeout=timeout_s,
        )
    except subprocess.TimeoutExpired:
        return False, f"lean timeout after {timeout_s}s"
    combined = (proc.stdout or "") + "\n" + (proc.stderr or "")
    # Same acceptance rules as Lean4Oracle::verify_omega_detailed:
    if "declaration uses 'sorry'" in combined:
        return False, "declaration uses sorry"
    if "No goals to be solved" in combined:
        return True, "ok (no goals)"
    if proc.returncode == 0 and "error:" not in combined:
        return True, "ok (exit 0, no error)"
    err_lines = [l for l in combined.splitlines()
                 if "error" in l or "unexpected" in l or "expected" in l]
    detail = " | ".join(err_lines[:3]) if err_lines else combined[:300]
    return False, f"reject(exit={proc.returncode}): {detail}"


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("jsonl", help="PPUT_RESULT jsonl from a batch run")
    ap.add_argument("--sample", type=int, default=None, help="spot-check N random solved rows")
    ap.add_argument("--max", type=int, default=None, help="cap to first N rows")
    ap.add_argument("--minif2f-dir",
                    default=os.environ.get("MINIF2F_DIR",
                                            "/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4"))
    ap.add_argument("--exp-dir",
                    default=os.environ.get("EXPERIMENT_DIR",
                                            str(Path(__file__).resolve().parent.parent)))
    ap.add_argument("--timeout", type=int, default=300)
    args = ap.parse_args()

    lean_bin = find_lean_binary()
    if not lean_bin:
        print("FATAL: no lean binary (set LEAN_BINARY or install elan toolchain)", file=sys.stderr)
        return 2
    lean_path = derive_lean_path(Path(args.minif2f_dir))
    if not lean_path:
        print(f"FATAL: no Mathlib under {args.minif2f_dir}/.lake/packages", file=sys.stderr)
        return 2

    rows = [json.loads(l) for l in open(args.jsonl)]
    solved_with_artifact = [r for r in rows
                            if r.get("has_golden_path") and r.get("gp_proof_file")]
    solved_no_artifact = [r for r in rows
                          if r.get("has_golden_path") and not r.get("gp_proof_file")]
    failed = [r for r in rows if not r.get("has_golden_path")]

    print(f"=== AUDIT: {args.jsonl} ===")
    print(f"  total rows:                 {len(rows)}")
    print(f"  solved (has artifact):      {len(solved_with_artifact)}")
    print(f"  solved (legacy, no artifact): {len(solved_no_artifact)}  ← cannot reverify")
    print(f"  unsolved:                    {len(failed)}")
    print(f"  lean binary:                 {lean_bin}")
    print(f"  LEAN_PATH segments:          {lean_path.count(':') + 1}")
    print()

    targets = list(solved_with_artifact)
    if args.sample is not None:
        random.seed(42)
        targets = random.sample(targets, min(args.sample, len(targets)))
    if args.max is not None:
        targets = targets[:args.max]

    print(f"=== Re-verifying {len(targets)} artifacts ===")
    ok_count, fail_count = 0, 0
    failures = []
    t0 = time.time()
    for i, row in enumerate(targets, 1):
        rel = row["gp_proof_file"]
        proof_file = Path(args.exp_dir) / rel
        name = row["problem"].rsplit("/", 1)[-1].replace(".lean", "")
        if not proof_file.exists():
            print(f"  [{i}/{len(targets)}] {name:<40} MISSING artifact at {proof_file}")
            fail_count += 1
            failures.append((name, "missing artifact"))
            continue
        t1 = time.time()
        ok, detail = reverify(lean_bin, lean_path, proof_file, timeout_s=args.timeout)
        elapsed = time.time() - t1
        if ok:
            print(f"  [{i}/{len(targets)}] {name:<40} VERIFIED ({elapsed:.0f}s)")
            ok_count += 1
        else:
            print(f"  [{i}/{len(targets)}] {name:<40} FAILED ({elapsed:.0f}s): {detail[:120]}")
            fail_count += 1
            failures.append((name, detail))

    total = ok_count + fail_count
    rate = (ok_count / total * 100) if total else 0
    elapsed = time.time() - t0
    print()
    print(f"=== Summary ===")
    print(f"  Re-verified:   {ok_count}/{total} = {rate:.1f}%")
    print(f"  Wall time:     {elapsed:.0f}s")
    if failures:
        print(f"  Failures:")
        for name, detail in failures:
            print(f"    - {name}: {detail[:200]}")
    return 0 if fail_count == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
