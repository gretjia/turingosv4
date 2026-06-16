#!/usr/bin/env python3
"""Audited autoresearch loop — driver core (GATE-1 + breakers + loop tape).

Design: handover/AUTORESEARCH_LOOP_DESIGN_2026-06-16.md.
Operator constraint: NEVER spend on a buggy or invalid run. So this driver is
deterministic and zero-LLM; the only paid step it runs is the positive-control
probe (cached on the harness hash). It refuses to authorize execution until
GATE-1 passes and the driver breakers are clear. EXECUTE (the carrier) and
GATE-2 (the independent audit Workflow) are invoked by the outer driver around
this core; this file owns everything that must be bypass-proof and free.

Subcommands:
  init      create the loop tape from a config
  preflight GATE-1 pre-flight validity gate (exit 0 iff all pass)
  breakers  driver circuit-breaker check for a candidate atom (exit 0 iff clear)
  record    append an iteration record (execute result + GATE-2 verdict + decision)
  status    print loop status + the next allowed action

Pure stdlib. The loop tape (loop_state.json) is the single source of truth;
everything else is a derived view (Art 0.2).
"""
import argparse, hashlib, json, subprocess, sys, time
from pathlib import Path

# ---------- tape io ----------

def load(p): return json.loads(Path(p).read_text())
def save(p, s): Path(p).write_text(json.dumps(s, indent=2) + "\n")
def sha256_file(p):
    h = hashlib.sha256()
    h.update(Path(p).read_bytes())
    return h.hexdigest()
def atom_hash(atom):
    return hashlib.sha256(json.dumps(atom, sort_keys=True).encode()).hexdigest()[:16]

def emit(obj, ok):
    print(json.dumps(obj, indent=2))
    sys.exit(0 if ok else 1)

# ---------- init ----------

def cmd_init(a):
    cfg = load(a.config)
    state_path = cfg["loop_state"]
    if Path(state_path).exists() and not a.force:
        emit({"error": f"{state_path} exists; use --force"}, False)
    # record baseline harness hashes (V1): the eval/verifier files the worker must NOT alter
    harness = {f: sha256_file(f) for f in cfg["harness_files"]}
    state = {
        "loop_id": cfg["loop_id"],
        "goal_predicate": cfg["goal_predicate"],   # machine-checkable success condition (+ ref)
        "anchor": cfg["anchor"],
        "config": cfg["config"],
        "harness_baseline": harness,
        "positive_control": {"valid_for_hash": None, "at": None, "result": None},
        "spent": {"tokens": 0, "usd": 0.0, "started_at": None},
        "iterations": [],
        "seen_hashes": {},
        "no_progress_streak": 0,
        "status": "initialized",
        "dead_reason": None,
    }
    save(state_path, state)
    emit({"ok": True, "loop_id": state["loop_id"], "harness_files": list(harness),
          "state": state_path}, True)

# ---------- GATE-1 pre-flight ----------

def _harness_digest(state):
    return hashlib.sha256(
        json.dumps(state["harness_baseline"], sort_keys=True).encode()).hexdigest()[:16]

def cmd_preflight(a):
    state = load(a.state)
    cfg = state["config"]
    checks, fails = [], []

    def chk(name, ok, detail=""):
        checks.append({"check": name, "ok": bool(ok), "detail": detail})
        if not ok:
            fails.append(name)

    # V1 harness integrity: pinned eval/verifier bytes unchanged since init.
    cur = {f: (sha256_file(f) if Path(f).exists() else None) for f in state["harness_baseline"]}
    drift = [f for f, h in cur.items() if h != state["harness_baseline"][f]]
    chk("V1_harness_integrity", not drift, f"drift={drift}" if drift else "pinned bytes intact")

    # V6 scope/reachability (deterministic, zero-LLM).
    for probe in cfg.get("reachability", []):
        rc = subprocess.run(probe["cmd"], shell=True, capture_output=True, text=True).returncode
        chk(f"V6_reach::{probe['name']}", rc == 0, probe.get("hint", ""))
    # no restricted-surface in the working diff (driver must not be silently editing §6)
    if cfg.get("forbid_restricted_surface"):
        diff = subprocess.run(["git", "diff", "--name-only"], capture_output=True, text=True).stdout.split()
        bad = [f for f in diff for s in cfg["restricted_surfaces"] if f.endswith(s)]
        chk("V6_no_restricted_surface", not bad, f"touched={bad}" if bad else "clean")

    # V4 evidence contract: goal predicate is bound + non-placeholder.
    gp = state["goal_predicate"]
    placeholder = any(t in json.dumps(gp).upper() for t in ["TBD", "<", "PLACEHOLDER", "FIXME"])
    chk("V4_evidence_contract", bool(gp) and not placeholder,
        "goal predicate bound, no placeholder" if not placeholder else "placeholder in goal predicate")

    # V5 budget-binding declared (the per-experiment estimate must exist before spend).
    chk("V5_budget_binding_declared", "budget_binding" in cfg and cfg["budget_binding"].get("estimate_cmd"),
        "budget-binding estimate hook present" )

    # V2 positive control (the canonical 'rule out harness bug first' probe).
    # Cached: valid as long as the harness bytes are unchanged (ties V1<->V2). Re-run on drift/--force/stale.
    pc = state["positive_control"]
    hd = _harness_digest(state)
    stale = (pc["at"] is None) or (a.max_age_s and (time.time() - pc["at"] > a.max_age_s))
    need = a.force or drift or pc["valid_for_hash"] != hd or stale
    if not cfg.get("positive_control"):
        chk("V2_positive_control", False, "no positive_control configured")
    elif not need:
        chk("V2_positive_control", pc["result"] == "PASS",
            f"cached PASS (hash {hd}, age {int(time.time()-pc['at'])}s)")
    else:
        ok, detail = _run_control(cfg["positive_control"], want_pass=True)
        state["positive_control"] = {"valid_for_hash": hd, "at": time.time(),
                                     "result": "PASS" if ok else "FAIL"}
        save(a.state, state)
        chk("V2_positive_control", ok, detail)

    # V3 negative control (the gate can actually say no), if configured.
    if cfg.get("negative_control"):
        ok, detail = _run_control(cfg["negative_control"], want_pass=False)
        chk("V3_negative_control", ok, detail)

    passed = not fails
    emit({"gate": "GATE-1", "pass": passed, "fails": fails, "checks": checks,
          "harness_digest": hd}, passed)

def _run_control(ctl, want_pass):
    """Run a control command and verify its outcome.
    want_pass=True  : a known-solvable probe MUST verify (else harness is broken).
    want_pass=False : a known-bad probe MUST be rejected (else harness can't discriminate).
    mode='manifest' (default): assert a field in a written manifest JSON.
    mode='exit'    : assert the command's exit code (the cmd itself encodes the discrimination,
                     e.g. an axiom-gate test that exits 0 iff bad proofs are rejected)."""
    r = subprocess.run(ctl["cmd"], shell=True, capture_output=True, text=True,
                       timeout=ctl.get("timeout_s", 600))
    if ctl.get("mode") == "exit":
        exp_rc = ctl.get("expect_exit", 0)
        ok = (r.returncode == exp_rc)
        return ok, f"exit={r.returncode} (want {exp_rc}); {r.stdout[-120:].strip()}"
    if r.returncode != 0 and want_pass:
        return False, f"control cmd rc={r.returncode}: {r.stderr[-200:]}"
    try:
        man = load(ctl["manifest"])
    except Exception as e:
        return False, f"manifest unreadable: {e}"
    field, exp = ctl["assert_field"], ctl["assert_value"]
    got = man.get(field)
    solved = (got == exp)
    ok = solved if want_pass else (not solved)
    return ok, f"{field}={got} (want {'==' if want_pass else '!='} {exp})"

# ---------- driver breakers ----------

def cmd_breakers(a):
    state = load(a.state)
    cfg = state["config"]
    trips = []
    n = len(state["iterations"])

    if cfg.get("max_iterations") is not None and n >= cfg["max_iterations"]:
        trips.append(f"iteration_cap (n={n} >= {cfg['max_iterations']})")
    # cumulative budget: caps may be null (no total cap, per operator) — velocity still guards.
    bt = cfg.get("token_cap"); bu = cfg.get("usd_cap")
    if bt is not None and state["spent"]["tokens"] >= bt:
        trips.append(f"token_cap ({state['spent']['tokens']} >= {bt})")
    if bu is not None and state["spent"]["usd"] >= bu:
        trips.append(f"usd_cap ({state['spent']['usd']} >= {bu})")
    # duplicate-input hash (catches the $47K ping-pong in cycle 1)
    if a.atom_hash:
        seen = state["seen_hashes"].get(a.atom_hash, 0)
        if seen >= cfg.get("dup_hash_K", 2):
            trips.append(f"duplicate_input_hash ({a.atom_hash} seen {seen}x)")
    # no-progress streak
    if state["no_progress_streak"] >= cfg.get("no_progress_K", 3):
        trips.append(f"no_progress (streak={state['no_progress_streak']} >= {cfg.get('no_progress_K',3)})")
    # cost velocity (if we have spend history)
    started = state["spent"]["started_at"]
    if started and cfg.get("velocity_usd_per_hr"):
        hrs = max((time.time() - started) / 3600.0, 1e-6)
        rate = state["spent"]["usd"] / hrs
        if rate > cfg["velocity_usd_per_hr"]:
            trips.append(f"cost_velocity (${rate:.2f}/hr > ${cfg['velocity_usd_per_hr']}/hr)")

    clear = not trips
    emit({"gate": "BREAKERS", "clear": clear, "trips": trips,
          "iteration": n, "spent": state["spent"]}, clear)

# ---------- record an iteration ----------

VERDICTS = {"CONTINUE", "FIX-RETRY", "STOP-SUCCESS", "STOP-DEAD", "ESCALATE-HUMAN"}

def cmd_record(a):
    state = load(a.state)
    rec = load(a.iter_json)
    v = rec.get("decision")
    if v not in VERDICTS:
        emit({"error": f"decision must be one of {sorted(VERDICTS)}; got {v!r}"}, False)
    rec["n"] = len(state["iterations"])
    rec["ts"] = time.time()
    ah = rec.get("atom_hash")
    if ah:
        state["seen_hashes"][ah] = state["seen_hashes"].get(ah, 0) + 1
    # spend accounting
    sp = rec.get("spend", {})
    if state["spent"]["started_at"] is None:
        state["spent"]["started_at"] = time.time()
    state["spent"]["tokens"] += sp.get("tokens", 0)
    state["spent"]["usd"] += sp.get("usd", 0.0)
    # no-progress: advanced? (the recorder asserts whether this iteration advanced the goal)
    if rec.get("advanced") is True:
        state["no_progress_streak"] = 0
    else:
        state["no_progress_streak"] += 1
    state["iterations"].append(rec)
    # terminal verdicts update loop status
    if v == "STOP-SUCCESS":
        state["status"] = "stopped-success"
    elif v == "STOP-DEAD":
        state["status"] = "stopped-dead"; state["dead_reason"] = rec.get("dead_reason")
    elif v == "ESCALATE-HUMAN":
        state["status"] = "escalated"; state["dead_reason"] = rec.get("escalate_reason")
    else:
        state["status"] = "running"
    save(a.state, state)
    emit({"ok": True, "iteration": rec["n"], "decision": v, "status": state["status"],
          "no_progress_streak": state["no_progress_streak"], "spent": state["spent"]}, True)

# ---------- status / next allowed action ----------

def cmd_status(a):
    state = load(a.state)
    n = len(state["iterations"])
    last = state["iterations"][-1] if n else None
    nxt = {
        "initialized": "run `preflight`; if pass + breakers clear, propose iteration 1 atom",
        "running": "run `preflight` + `breakers` for the next atom; if clear, EXECUTE then GATE-2 audit",
        "stopped-success": "DONE — goal predicate met",
        "stopped-dead": f"DEAD — {state['dead_reason']}",
        "escalated": f"PARKED — human-only gate: {state['dead_reason']} (resume after the human clears it)",
    }.get(state["status"], "unknown")
    print(json.dumps({
        "loop_id": state["loop_id"], "status": state["status"], "iterations": n,
        "spent": state["spent"], "no_progress_streak": state["no_progress_streak"],
        "last_decision": last["decision"] if last else None,
        "next_allowed_action": nxt,
    }, indent=2))

# ---------- cli ----------

def main():
    p = argparse.ArgumentParser(description="Audited autoresearch loop driver core")
    sub = p.add_subparsers(dest="cmd", required=True)
    s = sub.add_parser("init"); s.add_argument("--config", required=True); s.add_argument("--force", action="store_true"); s.set_defaults(fn=cmd_init)
    s = sub.add_parser("preflight"); s.add_argument("--state", required=True); s.add_argument("--force", action="store_true"); s.add_argument("--max-age-s", type=int, default=1800, dest="max_age_s"); s.set_defaults(fn=cmd_preflight)
    s = sub.add_parser("breakers"); s.add_argument("--state", required=True); s.add_argument("--atom-hash", default=None, dest="atom_hash"); s.set_defaults(fn=cmd_breakers)
    s = sub.add_parser("record"); s.add_argument("--state", required=True); s.add_argument("--iter-json", required=True, dest="iter_json"); s.set_defaults(fn=cmd_record)
    s = sub.add_parser("status"); s.add_argument("--state", required=True); s.set_defaults(fn=cmd_status)
    a = p.parse_args()
    a.fn(a)

if __name__ == "__main__":
    main()
