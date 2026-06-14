#!/usr/bin/env python3
"""Analyze H-HET-1 carrier pilot: per-(theorem,arm) omega-solve rate over seeds,
which vendor produced each omega (sidecar round-robin attribution), replay status,
self-selection honesty (action_source), short incidence. DIRECTIONAL only."""
import json, glob, os, collections, sys

OUT = sys.argv[1] if len(sys.argv) > 1 else "handover/evidence/het_carrier_pilot_2026-06-15"
ARMS = ["HET", "DSHOMO", "Q397HOMO"]
THMS = ["lm_det_mul", "lm_det_2x2", "lm_det_zero", "lm_det_3x3", "lm_geom_eval"]

def short(m): return {"DeepSeek-V4-Pro":"DS","Qwen3-32B":"Q32","GLM-4.5-Air":"GLM","Qwen3.5-397B-A17B":"Q397"}.get(m,m)

# (thm,arm) -> list of (seed, solved_bool, omega_vendor, replay_ok, axiom_clean, src_mix, short_count)
cells = collections.defaultdict(list)
total=0; replay_fail=0; excluded=[]
for f in sorted(glob.glob(f"{OUT}/*__*__s*.json")):
    if f.endswith(".replay.json"): continue
    base = os.path.basename(f)[:-5]
    try: thm, arm, sd = base.split("__"); seed=int(sd[1:])
    except: continue
    try: m = json.load(open(f))
    except: continue
    total+=1
    roster = m.get("models", [])
    rep = f"{OUT}/{base}.replay.json"
    replay_ok = False
    if os.path.exists(rep):
        try: replay_ok = json.load(open(rep)).get("economic_state_reconstructed") is True
        except: pass
    if not replay_ok: replay_fail+=1; excluded.append(base)
    omega_node = m.get("omega_node")
    omega_vendor=None; axclean=None; srcmix=collections.Counter(); shorts=0
    for n in m.get("nodes", []):
        if n.get("chosen_action")=="short": shorts+=1
        if n.get("action_source"): srcmix[n["action_source"]]+=1
        if n.get("node_tx")==omega_node and n.get("is_verified"):
            # agent index -> vendor via round-robin
            try:
                ai=int(n["by_agent"].split("_")[1]); omega_vendor=short(roster[ai%len(roster)].split("/")[-1]) if roster else "?"
            except: omega_vendor="?"
            axset=set(n.get("axioms",[])); axclean = axset.issubset({"Classical.choice","Quot.sound","propext"}) and bool(axset)
    solved = bool(m.get("omega_reached")) and replay_ok
    cells[(thm,arm)].append((seed, solved, omega_vendor, replay_ok, axclean, dict(srcmix), shorts))

print(f"=== H-HET-1 carrier pilot analysis ({total} cells, replay_fail={replay_fail}) ===\n")
hdr=f"{'theorem':14}" + "".join(f"{a:>12}" for a in ARMS)
print(hdr); print("-"*len(hdr))
arm_solved=collections.Counter()
for thm in THMS:
    row=f"{thm:14}"
    for arm in ARMS:
        recs=sorted(cells.get((thm,arm),[]))
        ns=sum(1 for r in recs if r[1]); tot=len(recs)
        vendors=",".join(sorted({r[2] for r in recs if r[1] and r[2]}))
        arm_solved[arm]+=ns
        cell=f"{ns}/{tot}" + (f"({vendors})" if vendors else "")
        row+=f"{cell:>12}"
    print(row)
print("-"*len(hdr))
print(f"{'SOLVED-TOT':14}"+"".join(f"{arm_solved[a]:>12}" for a in ARMS))

# honesty + soundness
print("\n=== self-selection honesty (action_source mix) + short incidence ===")
agg=collections.Counter(); shorts=0; axbad=[]
for k,recs in cells.items():
    for (seed,solved,ven,rep,axc,src,sh) in recs:
        for s,c in src.items(): agg[s]+=c
        shorts+=sh
        if solved and axc is False: axbad.append((k,seed,ven))
print("  action_source:", dict(agg), "| short-actions:", shorts)
if axbad: print("  ⚠ NON-axiom-clean omega(s):", axbad)
else: print("  all omega proofs axiom-clean ⊆ whitelist (or none yet)")

# H-HET-1 directional verdict
print("\n=== H-HET-1 directional read ===")
gold=["lm_det_mul","lm_det_2x2","lm_det_zero"]
het_gold=sum(1 for t in gold for r in cells.get((t,"HET"),[]) if r[1])
ds_gold =sum(1 for t in gold for r in cells.get((t,"DSHOMO"),[]) if r[1])
print(f"  Goldilocks solves: HET={het_gold}  DSHOMO={ds_gold}  Q397HOMO={sum(1 for t in gold for r in cells.get((t,'Q397HOMO'),[]) if r[1])}")
print(f"  total solved: HET={arm_solved['HET']} DSHOMO={arm_solved['DSHOMO']} Q397HOMO={arm_solved['Q397HOMO']}")
print("  (DIRECTIONAL only — K=3, sidecar attribution pending Art-0.2 §8, no PROVEN/§17-G4.)")
if excluded: print(f"\n  excluded (replay!=clean): {excluded}")
