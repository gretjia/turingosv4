#!/usr/bin/env python3
"""Persistent Lean verify-service — load `import Mathlib` ONCE, serve many verifies.

WHY: the LeanJudge (`src/judges/lean_judge.rs`) currently spawns a FRESH `lean`
process per verify, each re-loading the Mathlib umbrella (~3.3s warm / ~19s cold
single-compile; ~6.7s per full verify incl. the `#print axioms` second run). A
148-run calibration (and the 12x-larger confirmatory) is impractically slow. This
service loads `import Mathlib` ONCE into a base environment, then verifies each
candidate against it via the leanprover-community/repl (driven by lean-interact),
turning ~6.7s/verify into ~0.026s/verify (spike: 8/8 byte-identical verdicts,
~130-260x; see handover spike artifacts).

CONTRACT (drop-in for `LeanJudge::run_lean_source`): the Rust
`PersistentServiceBackend` sends the SAME assembled source it would have written
to a temp `.lean` file, and gets back the SAME `{exit_code, stdout, stderr}` shape
a `lean -DwarningAsError=true <file>` invocation produces — so `verify()`'s
existing parsing (`shield_lean_diagnostic`, `classify_axiom_report`,
`parse_axiom_set`, the axiom-whitelist gate) runs UNCHANGED. verify() keeps the
source-scan (sorry/admit/native_decide/unsafe) + assemble + dedent + axiom-gate as
shared control flow; only the lean-run is swapped. Equivalence is proven by an A/B
oracle (`scripts/lean_verify_ab_oracle.py` / the Rust oracle test) before the fast
path is trusted.

`-DwarningAsError=true` EMULATION: lean promotes warnings (incl. "declaration uses
'sorry'") to errors -> exit 1. The REPL does NOT auto-promote, so we replicate it:
exit_code = 1 if (LeanError OR any message severity in {error, warning} OR sorries
non-empty), else 0. stdout carries info-message data (incl. the `#print axioms`
report line, verbatim from Lean, so parse_axiom_set sees the same text). stderr
carries error/warning diagnostics (for the shielded retry feedback; this text is a
non-canonical hint and is the one field the A/B oracle permits to differ).

PROTOCOL: newline-delimited JSON on stdin/stdout.
  startup ->  {"ready": true, "cold_load_s": <float>, "repl_commit": "<sha>"}
  request <-  {"id": <any>, "source": "<full assembled .lean source>"}
  response -> {"id": <same>, "exit_code": 0|1, "stdout": "...", "stderr": "...",
               "timed_out": false}
  control <-  {"cmd": "ping"} -> {"pong": true} ; {"cmd": "shutdown"} -> exits.

DEPLOYMENT: the host MUST export ELAN_TOOLCHAIN=leanprover/lean4:v4.24.0 (already
installed) before launching, else elan tries to download its DEFAULT toolchain
(v4.31.0) when lake builds the REPL and fails offline. The Rust backend sets this
in the child env. Only Mathlib-importing candidates are supported on this fast
path (BASE_ENV = `import Mathlib`); for any other import set the Rust side falls
back to the process-spawn backend via the feature flag.

Pinned facts (Lean v4.24.0 + mathlib4, spike-verified 2026-06-17): clean proof ->
axioms subset of {propext, Classical.choice, Quot.sound}; native_decide ->
[Lean.ofReduceBool, Lean.trustCompiler]; sorry -> "declaration uses 'sorry'"
warning; hand-axiom -> shows the axiom name.
"""
import json
import os
import sys
import time

MATHLIB_DIR = os.environ.get("TURINGOS_MATHLIB_DIR", "/Users/zephryj/work/mathlib4")
LAKE = os.path.expanduser(os.environ.get("TURINGOS_LAKE_BIN", "~/.elan/bin/lake"))


def _eprint(msg):
    sys.stderr.write(msg + "\n")
    sys.stderr.flush()


def build_server():
    """Build/load the REPL (lean-interact auto-matches repl@v4.24.0) and a server.

    Relies on ELAN_TOOLCHAIN=leanprover/lean4:v4.24.0 being set in the env so lake
    uses the installed toolchain instead of downloading the elan default.
    """
    import lean_interact as li

    cfg = li.LeanREPLConfig(
        project=li.LocalProject(directory=MATHLIB_DIR, auto_build=False),
        lake_path=LAKE,
        verbose=False,
    )
    server = li.LeanServer(cfg)
    return li, server


def load_base_env(li, server):
    """Load `import Mathlib` ONCE; return its env id (BASE_ENV)."""
    resp = server.run(li.Command(cmd="import Mathlib"))
    if isinstance(resp, li.interface.LeanError):
        raise RuntimeError(f"base `import Mathlib` failed: {resp.message}")
    return resp.env


def _strip_imports(source):
    """Drop `import ...` lines — BASE_ENV already has `import Mathlib`. A candidate
    that imports only Mathlib (the calibration pool) is fully covered; a non-Mathlib
    import would be silently dropped, which is why the Rust side gates this backend
    to Mathlib-importing theorems and falls back to process-spawn otherwise."""
    return "\n".join(
        ln for ln in source.splitlines() if not ln.lstrip().startswith("import ")
    )


def run_source(li, server, base_env, source):
    """Run one self-contained source against BASE_ENV; return a `lean -DwarningAsError`
    -shaped {exit_code, stdout, stderr, timed_out}. Self-contained per request: the
    Rust side's second call (source + `#print axioms <name>`) re-defines and prints
    in one command, exactly mirroring the process-spawn path's two fresh-file runs."""
    body_src = _strip_imports(source)
    try:
        resp = server.run(li.Command(cmd=body_src, env=base_env))
    except Exception as e:  # REPL transport / timeout — fail closed as a lean error.
        return {"exit_code": 1, "stdout": "", "stderr": f"repl run failed: {e}",
                "timed_out": False}

    if isinstance(resp, li.interface.LeanError):
        # hard parse/elaboration failure — mirrors a non-zero lean exit + stderr.
        return {"exit_code": 1, "stdout": "", "stderr": (resp.message or "lean error"),
                "timed_out": False}

    msgs = resp.messages or []
    sorries = resp.sorries or []
    infos, errs = [], []
    has_error = has_warning = False
    for m in msgs:
        sev = getattr(m, "severity", "info")
        data = getattr(m, "data", "") or ""
        if sev == "error":
            has_error = True
            errs.append(f"error: {data}")
        elif sev == "warning":
            has_warning = True
            errs.append(f"warning: {data}")
        else:
            infos.append(data)
    # -DwarningAsError=true emulation: warnings (incl. sorry) and any sorries -> exit 1.
    exit_code = 1 if (has_error or has_warning or sorries) else 0
    if sorries and not has_warning:
        # the REPL surfaces `sorry` as a `sorries` entry; lean prints it as the
        # "declaration uses 'sorry'" warning. Synthesize that text so the shared
        # parser/feedback path sees the same diagnostic.
        errs.append("warning: declaration uses 'sorry'")
    return {
        "exit_code": exit_code,
        "stdout": "\n".join(infos),
        "stderr": "\n".join(errs),
        "timed_out": False,
    }


def main():
    out = sys.stdout
    t0 = time.perf_counter()
    li, server = build_server()
    base_env = load_base_env(li, server)
    cold = time.perf_counter() - t0
    repl_commit = os.environ.get("TURINGOS_REPL_COMMIT", "")
    out.write(json.dumps({"ready": True, "cold_load_s": round(cold, 3),
                          "repl_commit": repl_commit}) + "\n")
    out.flush()
    _eprint(f"[lean_verify_service] ready: import Mathlib loaded in {cold:.2f}s")

    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        try:
            req = json.loads(line)
        except Exception as e:
            out.write(json.dumps({"error": f"bad json: {e}"}) + "\n")
            out.flush()
            continue
        cmd = req.get("cmd")
        if cmd == "shutdown":
            break
        if cmd == "ping":
            out.write(json.dumps({"pong": True}) + "\n")
            out.flush()
            continue
        rid = req.get("id")
        source = req.get("source", "")
        res = run_source(li, server, base_env, source)
        res["id"] = rid
        out.write(json.dumps(res) + "\n")
        out.flush()

    try:
        server.kill()
    except Exception:
        pass


if __name__ == "__main__":
    main()
