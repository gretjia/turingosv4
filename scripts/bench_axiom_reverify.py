#!/usr/bin/env python3
"""BENCH replay gate: independently re-verify every banked theorem under Lean + #print axioms, asserting each axiom footprint ⊆ {propext, Classical.choice, Quot.sound} (rejects sorryAx and native_decide compiler-trust axioms).
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
LEAN = os.path.expanduser("~/.elan/bin/lean")  # version-agnostic elan shim (matches LAKE); resolves the live toolchain

# Architect's clean-axiom whitelist: a banked theorem may depend ONLY on these axioms. Anything else
# (sorryAx, or Lean.ofReduceBool / Lean.trustCompiler pulled in by native_decide) fails the gate.
AXIOM_WHITELIST = frozenset({"propext", "Classical.choice", "Quot.sound"})

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

def axiom_set(out):
    """Parse `#print axioms <name>` output into (axioms:set, found:bool).
    Lean prints `'<name>' depends on axioms: [a, b, ...]` or `... does not depend on any axioms`."""
    if "does not depend on any axioms" in out:
        return set(), True
    m = re.search(r"depends on axioms:\s*\[([^\]]*)\]", out)
    if not m:
        return set(), False
    return {a.strip() for a in m.group(1).split(",") if a.strip()}, True

def reverify(thm, lp):
    """Re-run preamble + reference_body + #print axioms. CLEAN iff Lean exits 0 with no error and no
    sorry, AND the printed axiom set is a SUBSET of AXIOM_WHITELIST. The positive whitelist check is
    required because sorry-free does NOT imply axiom-clean: `native_decide` compiles with exit 0 and no
    "error"/"sorry" in output yet pulls in Lean.ofReduceBool / Lean.trustCompiler (compiler trust axioms)."""
    pre = thm["preamble"]; body = thm["reference_body"]
    m = re.search(r'\btheorem\s+(\w+)', pre)
    if m:
        name = m.group(1)
        src = pre + "\n" + body + "\n"
    else:
        # anonymous `example` → give it a name so its axiom footprint is audited too (no silent blind spot)
        name = "_bench_reverify"
        src, n = re.subn(r'(?m)^(\s*)example\b', rf'\1theorem {name}', pre, count=1)
        if n == 0:
            return False, "no `theorem`/`example` declaration to audit axioms for"
        src += "\n" + body + "\n"
    src += f"#print axioms {name}\n"
    f = tempfile.NamedTemporaryFile("w", suffix=".lean", delete=False, dir="/tmp")
    f.write(src); f.close()
    r = subprocess.run([LEAN, f.name], cwd=ML, capture_output=True, text=True, env=dict(os.environ, LEAN_PATH=lp))
    out = r.stdout + r.stderr
    os.unlink(f.name)
    low = out.lower(); flat = " ".join(out.split())
    # (1) Lean must compile clean. sorry/admit compile with exit 0 but emit "uses 'sorry'", so check text too.
    if r.returncode != 0 or "error" in low or "sorry" in low:
        return False, "lean-unclean: " + flat[:180]
    # (2) Positive axiom whitelist — the architect's honesty requirement, stronger than a sorry/error grep.
    axs, found = axiom_set(out)
    if not found:
        return False, "axiom footprint not reported by #print axioms: " + flat[:180]
    extra = axs - AXIOM_WHITELIST
    if extra:
        return False, "axioms outside whitelist {propext,Classical.choice,Quot.sound}: " + ",".join(sorted(extra))
    return True, "axioms=[" + ", ".join(sorted(axs)) + "]"

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
        print(f"  {bid:28} {('CLEAN ' + msg) if clean else ('FAIL: ' + msg)}")
        if not clean: bad.append((bid, msg))
    print(f"\n{'ALL CLEAN' if not bad else f'{len(bad)} FAILED'}: {len(all_banked)-len(bad)}/{len(all_banked)} banked theorems re-verify axiom-clean")
    sys.exit(0 if not bad else 1)

if __name__ == "__main__":
    main()
