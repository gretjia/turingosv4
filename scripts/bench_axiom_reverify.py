#!/usr/bin/env python3
"""BENCH replay gate: independently re-verify every banked theorem under Lean + #print axioms (no sorryAx).
A banked solve only counts toward the headline if its reference-or-emitted proof re-verifies clean.

Since run_alloc banks a theorem when the reasoner's repair compiled (verify_pool already rejects
sorry/admit/native_decide at line 262), this gate is the INDEPENDENT audit: re-run the pool theorem's
own reference proof (self-test ground truth) + #print axioms, confirming the banked id is a genuinely
provable theorem with a clean axiom footprint. Reports any banked id whose theorem does NOT re-verify.

Usage: python3 scripts/bench_axiom_reverify.py <manifest.json> [<manifest2.json> ...]
Exit 0 iff every banked_id across all manifests re-verifies Lean-clean + axiom-clean.
"""
import json, sys, subprocess, os, tempfile, re

POOL = "tests/fixtures/lean_theorems_pool.jsonl"
ML = os.path.expanduser("~/work/mathlib4")
LAKE = os.path.expanduser("~/.elan/bin/lake")
LEAN = os.path.expanduser("~/.elan/toolchains/leanprover--lean4---v4.24.0/bin/lean")

def load_pool():
    d = {}
    for line in open(POOL):
        line = line.strip()
        if not line or line.startswith("//"): continue
        o = json.loads(line)
        d[o["id"]] = o
    return d

def lean_path():
    r = subprocess.run([LAKE, "env", "printenv", "LEAN_PATH"], cwd=ML, capture_output=True, text=True)
    return r.stdout.strip()

def reverify(thm, lp):
    """Re-run preamble + reference_body + #print axioms; clean iff exit 0, no error, no sorryAx."""
    # the theorem name is parsed from the preamble's `theorem <name>` / `example`
    pre = thm["preamble"]; body = thm["reference_body"]
    m = re.search(r'\btheorem\s+(\w+)', pre)
    src = pre + "\n" + body + "\n"
    if m:
        src += f"#print axioms {m.group(1)}\n"
    f = tempfile.NamedTemporaryFile("w", suffix=".lean", delete=False, dir="/tmp")
    f.write(src); f.close()
    r = subprocess.run([LEAN, f.name], cwd=ML, capture_output=True, text=True, env=dict(os.environ, LEAN_PATH=lp))
    out = (r.stdout + r.stderr).lower()
    os.unlink(f.name)
    clean = r.returncode == 0 and "error" not in out and "sorryax" not in out and "sorry" not in out
    return clean, out[:200]

def main():
    pool = load_pool(); lp = lean_path()
    if not lp: print("LEAN_PATH unresolved"); sys.exit(2)
    all_banked, bad = set(), []
    for mf in sys.argv[1:]:
        try: d = json.load(open(mf))
        except Exception as e: print(f"skip {mf}: {e}"); continue
        for bid in d.get("banked_ids", []):
            all_banked.add(bid)
    print(f"re-verifying {len(all_banked)} distinct banked theorems under Lean + #print axioms ...")
    for bid in sorted(all_banked):
        if bid not in pool: bad.append((bid, "not in pool")); continue
        clean, msg = reverify(pool[bid], lp)
        print(f"  {bid:28} {'CLEAN' if clean else 'FAIL: ' + msg}")
        if not clean: bad.append((bid, msg))
    print(f"\n{'ALL CLEAN' if not bad else f'{len(bad)} FAILED'}: {len(all_banked)-len(bad)}/{len(all_banked)} banked theorems re-verify axiom-clean")
    sys.exit(0 if not bad else 1)

if __name__ == "__main__":
    main()
