#!/usr/bin/env python3
"""TP-2 routing-efficiency diagnostic — COMPLEMENTS analyze_t2_sweep.py (which does the Wilcoxon verdict).

The raw banked@B mixes the SHARED free-bank (identical across arms) with the routable repairs. The actual
routing question is: of the budget-limited REPAIRABLE residuals, what fraction did each arm's ORDER capture
within B? This normalizes for per-seed routing ROOM (some seeds have repairable=1 = no routing room; others
have repairable=6 = lots), which raw banked@B hides. It also reports the price-informativeness read the
role-correction frame needs: if market ~ random/shuffled here, the betting PRICE is the weak link to improve
(make it aggregate repair-success), NOT evidence price-coordination is impossible.

routing_capture[arm,seed] = (banked_at_B - free_banked) / repairable      # fraction of routable budget captured
  - free_banked + repairable are SHARED (per-seed constants); only the captured numerator varies by arm ORDER.
  - market capturing a higher fraction than shuffled/flatbid/random => the PRICE ranked successful repairs
    earlier (informative price). market ~ those floors => price uninformative on this substrate.

Usage: python3 scripts/analyze_t2_routing_efficiency.py --dir <dir> --prefix t2s --seeds 8,..,19 \
         --arms market,coordinator,shuffled,flatbid,random,index
Read-out only (never a gate).
"""
import json, os, argparse
from collections import defaultdict

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--dir", default="handover/evidence/t2_shared_sweep_2026-06-01")
    ap.add_argument("--prefix", default="t2s")
    ap.add_argument("--seeds", default="8,9,10,11,12,13,14,15,16,17,18,19")
    ap.add_argument("--arms", default="market,coordinator,shuffled,flatbid,random,index")
    a = ap.parse_args()
    seeds = [s.strip() for s in a.seeds.split(",")]
    arms = [x.strip() for x in a.arms.split(",")]

    cap = defaultdict(dict)        # arm -> seed -> routing_capture fraction
    captured = defaultdict(dict)   # arm -> seed -> (banked - free) absolute
    room = {}                      # seed -> (free, repairable)
    for seed in seeds:
        # repairable/free are shared; read from any present arm (prefer market)
        for probe in ["market"] + arms:
            mf = os.path.join(a.dir, f"{a.prefix}_{probe}_{seed}.json")
            if os.path.exists(mf):
                d = json.load(open(mf))
                room[seed] = (d.get("free_banked"), d.get("repairable")); break
        for arm in arms:
            mf = os.path.join(a.dir, f"{a.prefix}_{arm}_{seed}.json")
            if not os.path.exists(mf):
                continue
            d = json.load(open(mf))
            b, fr, rp = d.get("banked_at_B"), d.get("free_banked"), d.get("repairable")
            if None in (b, fr, rp):
                continue
            captured[arm][seed] = b - fr
            cap[arm][seed] = (b - fr) / rp if rp > 0 else None

    print("=== per-seed routing ROOM (free banked + repairable = the routable budget) ===")
    routable_seeds = []
    for seed in seeds:
        fr, rp = room.get(seed, (None, None))
        flag = "" if (rp or 0) >= 1 else "  (NO routing room — excluded from capture mean)"
        if (rp or 0) >= 1: routable_seeds.append(seed)
        print(f"  seed {seed}: free={fr} repairable={rp}{flag}")
    print(f"  => {len(routable_seeds)}/{len(seeds)} seeds have routing room (repairable>=1)")

    print("\n=== routing CAPTURE = (banked@B - free) / repairable  [fraction of routable budget the ORDER captured] ===")
    for arm in arms:
        vals = [cap[arm][s] for s in routable_seeds if cap[arm].get(s) is not None]
        abs_cap = [captured[arm][s] for s in routable_seeds if captured[arm].get(s) is not None]
        mean = sum(vals)/len(vals) if vals else 0
        row = ",".join(f"{cap[arm].get(s):.2f}" if cap[arm].get(s) is not None else "-" for s in routable_seeds)
        print(f"  {arm:13} mean_capture={mean:.3f}  (abs repairs captured: mean={sum(abs_cap)/len(abs_cap):.2f})  [{row}]")

    if "market" in arms:
        print("\n=== price-informativeness read (market capture vs the no-/destroyed-price floors) ===")
        m = [cap['market'][s] for s in routable_seeds if cap['market'].get(s) is not None]
        mm = sum(m)/len(m) if m else 0
        for floor in ["shuffled", "flatbid", "random", "index"]:
            if floor in arms:
                f = [cap[floor][s] for s in routable_seeds if cap[floor].get(s) is not None and cap['market'].get(s) is not None]
                mf = sum(f)/len(f) if f else 0
                tag = "price INFORMATIVE" if mm - mf > 0.05 else ("price ~ floor (WEAK — betting is the lever)" if abs(mm-mf) <= 0.05 else "market BELOW floor")
                print(f"  market({mm:.3f}) - {floor}({mf:.3f}) = {mm-mf:+.3f}  => {tag}")
        print("  (role-correction frame: market~floor here => improve betting to aggregate repair-success;")
        print("   NOT evidence price-coordination is impossible.)")

if __name__ == "__main__":
    main()
