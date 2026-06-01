#!/usr/bin/env python3
"""P1 hard-problem verdict analyzer — does price-routed tree search CRACK theorems a single chain can't,
and is AUTONOMOUS (agent free-choice) different from MARKET (forced softmax)? Plus route-telemetry honesty.

Design (architect): on HARD theorems where single RELIABLY FAILS (0/N seeds), each theorem a price arm
(autonomous OR market) SOLVES (>=1 seed) is a confound-shielded CRACK. Pairwise EXACT McNemar (one-sided),
matched by (theorem,seed): autonomous-vs-single, market-vs-single, autonomous-vs-market. Every counted cell must
be verify_chaintape replay-clean (economic_state_reconstructed). Route telemetry (autonomous): a hallucinating
model or a latest-index chain-collapse INVALIDATES a "free routing helped" claim (§17 falsifiability).

Usage: python3 scripts/analyze_p1_hardproblem.py --dir handover/evidence/p1_realvalue_v3_2026-06-02 \
         --theorems lm_deriv1,lm_ineq1,lm_ineq2,lm_coeff_mul,lm_nt_gcd2,lm_median --arms autonomous,market,single --seeds 1,2,3
"""
import json, os, argparse, math
from collections import defaultdict

def mcnemar_one_sided_greater(b, c):
    n = b + c
    if n == 0: return 1.0
    return min(1.0, sum(math.comb(n, k) for k in range(b, n + 1)) / (2 ** n))

def holm(pvals):
    items = sorted(pvals.items(), key=lambda kv: kv[1]); m = len(items); adj = {}; prev = 0.0
    for i, (name, p) in enumerate(items):
        a = min(1.0, (m - i) * p); a = max(a, prev); adj[name] = a; prev = a
    return adj

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--dir", default="handover/evidence/p1_realvalue_v3_2026-06-02")
    ap.add_argument("--theorems", default="lm_deriv1,lm_ineq1,lm_ineq2,lm_coeff_mul,lm_nt_gcd2,lm_median")
    ap.add_argument("--arms", default="autonomous,market,single")
    ap.add_argument("--seeds", default="1,2,3")
    ap.add_argument("--alpha", type=float, default=0.05)
    a = ap.parse_args()
    thms=[x.strip() for x in a.theorems.split(",")]; arms=[x.strip() for x in a.arms.split(",")]; seeds=[x.strip() for x in a.seeds.split(",")]

    solved=defaultdict(dict); route=defaultdict(lambda: defaultdict(int)); excluded=[]
    for thm in thms:
        for arm in arms:
            for s in seeds:
                c=os.path.join(a.dir, f"{thm}__{arm}__s{s}"); mf,rr=c+".json",c+".replay.json"
                if not os.path.exists(mf): excluded.append((thm,arm,s,"no-manifest")); continue
                d=json.load(open(mf))
                rep=json.load(open(rr)).get("economic_state_reconstructed") if os.path.exists(rr) else None
                if rep is not True: excluded.append((thm,arm,s,"replay-not-clean")); continue
                solved[arm][(thm,s)] = 1 if (d.get("omega_reached") or d.get("verified_count",0)>0) else 0
                if arm=="autonomous":
                    for k in ("route_valid_index_hit","route_deliberate_fresh_root","route_hallucinated_out_of_range"):
                        route[arm][k]+=d.get(k,0) or 0
    if excluded:
        print(f"=== EXCLUDED {len(excluded)} cells (no-manifest / replay-not-clean) ===")
        for t,ar,s,why in excluded[:40]: print(f"  {t}/{ar}/s{s}: {why}")
        print()

    print("=== per-theorem solve counts (X=solved, .=fail); the HARD set = single solves 0/N ===")
    print(f"  {'theorem':14} " + " ".join(f"{ar:^12}" for ar in arms))
    hard=[]  # theorems where single solved 0
    for thm in thms:
        cols=[];
        for ar in arms:
            cells=sorted([(s,solved[ar][(thm,s)]) for s in seeds if (thm,s) in solved[ar]])
            cols.append("".join("X" if v else "." for _,v in cells) or "-")
        sc=sum(solved.get("single",{}).get((thm,s),0) for s in seeds) if "single" in arms else None
        if sc==0: hard.append(thm)
        print(f"  {thm:14} " + " ".join(f"{c:^12}" for c in cols) + ("   <- HARD (single 0)" if sc==0 else ""))

    # CONFIRMED CRACKS: hard theorems (single 0) that a price arm solved >=1
    print("\n=== CONFIRMED CRACKS — hard theorems (single 0/N) a price arm SOLVED (the confound-shielded headline) ===")
    price_arms=[ar for ar in ("autonomous","market") if ar in arms]
    any_crack=False
    for thm in hard:
        for ar in price_arms:
            k=sum(solved.get(ar,{}).get((thm,s),0) for s in seeds); n=sum(1 for s in seeds if (thm,s) in solved.get(ar,{}))
            if k>0:
                any_crack=True; print(f"  *** {ar} CRACKED {thm} ({k}/{n} seeds) where single solved 0 ***")
    if not any_crack: print("  (none yet — no price arm has cracked a single-0 theorem in the replay-clean cells)")

    # pairwise exact McNemar
    print(f"\n=== pairwise paired McNemar (one-sided, Holm @ alpha={a.alpha}) ===")
    pairs=[(x,y) for x in price_arms for y in (["single"] if "single" in arms else [])] + ([("autonomous","market")] if set(["autonomous","market"]).issubset(arms) else [])
    pv={}; disc={}
    for x,y in pairs:
        keys=[k for k in solved.get(x,{}) if k in solved.get(y,{})]
        b=sum(1 for k in keys if solved[x][k]==1 and solved[y][k]==0)
        c=sum(1 for k in keys if solved[x][k]==0 and solved[y][k]==1)
        pv[f"{x}>{y}"]=mcnemar_one_sided_greater(b,c); disc[f"{x}>{y}"]=(b,c,len(keys))
    adj=holm(pv)
    for name in pv:
        b,c,n=disc[name]; print(f"  {name:24} {name.split('>')[0]}-only={b} {name.split('>')[1]}-only={c} (n={n})  p_holm={adj[name]:.4f}  {'PASS' if adj[name]<a.alpha and b>c else '-'}")

    # route telemetry honesty (autonomous)
    if "autonomous" in arms:
        r=route["autonomous"]; tot=r["route_valid_index_hit"]+r["route_deliberate_fresh_root"]+r["route_hallucinated_out_of_range"]
        print("\n=== route telemetry (autonomous) — is 'free routing' genuine? ===")
        print(f"  valid_index_hit={r['route_valid_index_hit']} deliberate_fresh_root={r['route_deliberate_fresh_root']} hallucinated_out_of_range={r['route_hallucinated_out_of_range']} (total routed={tot})")
        if tot:
            hr=r['route_hallucinated_out_of_range']/tot
            print(f"  => hallucination rate {hr:.1%}" + ("  WARN: high hallucination — free-routing claim weakened" if hr>0.2 else "  (low — routing is deliberate)"))

    print("\n=== VERDICT (held to Verdict B until a CONFIRMED CRACK clears §17 G1-G6) ===")
    if any_crack:
        print("  A-direction: a price-routed arm CRACKED a hard theorem single could not (confound-shielded). Significance + replay + route-honesty gate the headline.")
    else:
        print("  A: no confirmed crack in replay-clean cells yet (report as-is; needs the full sweep / more seeds before NO-GO vs INCONCLUSIVE).")
    print("  B: every counted cell verify_chaintape replay-clean (gated above). A never inferred from B.")

if __name__ == "__main__":
    main()
