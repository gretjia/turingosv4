#!/usr/bin/env python3
"""BENCH aggregation + pre-registered go/no-go: Wilcoxon signed-rank (paired by seed) + Holm-Bonferroni.

Reads per-(arm,seed) run manifests, computes banked_at_B per arm, runs the pre-registered comparisons:
  market vs solo / shuffled / confgreedy / skeptic_rerank (paired across seeds), Holm-corrected, alpha=0.05.
Primary metric = banked_at_B (solve rate at equal reasoner budget). Secondary = cost_of_pass_micro_usd.

Usage: python3 scripts/bench_aggregate.py <glob-dir-of-manifests>
Manifests named like alloc_<arm>_s<seed>.json. Exit 0 = GO, 1 = NO-GO/WEAK-GO (reported either way).
"""
import json, sys, os, glob, statistics
from collections import defaultdict

def wilcoxon_signed_rank(diffs):
    """One-sided Wilcoxon signed-rank: H1 = median(diffs) > 0. Returns (W+, n, approx p one-sided)."""
    nz = [d for d in diffs if d != 0]
    n = len(nz)
    if n == 0: return (0.0, 0, 1.0)
    ranks = sorted(range(n), key=lambda i: abs(nz[i]))
    rank_val = [0.0]*n; i = 0
    # average ranks for ties in |d|
    srt = sorted(abs(nz[i]) for i in range(n))
    pos = {}
    j = 0
    while j < n:
        k = j
        while k < n and srt[k] == srt[j]: k += 1
        avg = (j + k + 1) / 2.0  # 1-based avg rank
        for idx in range(j, k): pos.setdefault(srt[j], []).append(avg)
        j = k
    # assign
    used = defaultdict(int)
    wplus = 0.0
    for d in nz:
        a = abs(d); r = pos[a][used[a]]; used[a]+=1
        if d > 0: wplus += r
    # normal approx (n>=10 ok; for small n this is approximate — report alongside raw win count)
    mean = n*(n+1)/4.0
    var = n*(n+1)*(2*n+1)/24.0
    if var == 0: return (wplus, n, 1.0)
    z = (wplus - mean) / (var**0.5)
    # one-sided p (H1: W+ large)
    import math
    p = 0.5 * math.erfc(z / (2**0.5))
    return (wplus, n, p)

def holm(pvals_named, alpha=0.05):
    """Holm-Bonferroni: returns dict name->reject(bool)."""
    items = sorted(pvals_named.items(), key=lambda kv: kv[1])
    m = len(items); out = {}
    for i,(name,p) in enumerate(items):
        thresh = alpha/(m-i)
        out[name] = (p <= thresh)
    return out

def main():
    d = sys.argv[1] if len(sys.argv)>1 else "/tmp/bench"
    files = glob.glob(os.path.join(d, "*.json"))
    # arm -> seed -> banked_at_B
    data = defaultdict(dict); cost = defaultdict(dict)
    for f in files:
        try: m = json.load(open(f))
        except: continue
        if m.get("schema","").startswith("lean_hayek_alloc"):
            arm = m["policy"]; seed = m["seed"]
            data[arm][seed] = m.get("banked_at_B", m.get("banked",0))
            cost[arm][seed] = m.get("cost_of_pass_micro_usd", 0)
    arms = sorted(data)
    print("=== banked_at_B by arm (mean ± sd over seeds) ===")
    for a in arms:
        v = list(data[a].values())
        if v: print(f"  {a:16} mean={statistics.mean(v):.2f} sd={statistics.pstdev(v):.2f} n={len(v)}  {sorted(v)}")
    if "market" not in data: print("no market arm"); sys.exit(1)
    print("\n=== pre-registered paired comparisons (market − foil, by seed) ===")
    pvals = {}
    foils = ["solo","shuffled","confgreedy","skeptic_rerank"]
    for foil in foils:
        if foil not in data: continue
        seeds = sorted(set(data["market"]) & set(data[foil]))
        diffs = [data["market"][s]-data[foil][s] for s in seeds]
        if not diffs: continue
        wins = sum(1 for x in diffs if x>0); ties=sum(1 for x in diffs if x==0)
        wplus,n,p = wilcoxon_signed_rank(diffs)
        pvals[f"market>{foil}"] = p
        print(f"  market vs {foil:14}: mean_uplift={statistics.mean(diffs):+.2f} wins={wins}/{len(diffs)} (ties {ties}) Wilcoxon p={p:.4f}")
    rej = holm(pvals, 0.05)
    print("\n=== Holm-Bonferroni (alpha=0.05) ===")
    for k,v in pvals.items(): print(f"  {k}: p={v:.4f} reject_null={rej[k]}")
    # GO logic (pre-registered)
    mkt = statistics.mean(data["market"].values()) if data["market"] else 0
    solo = statistics.mean(data["solo"].values()) if "solo" in data else 0
    go_solo = rej.get("market>solo",False) and (mkt-solo)>=3
    go_shuf = rej.get("market>shuffled",False)
    go_conf = rej.get("market>confgreedy",False)
    go_skep = rej.get("market>skeptic_rerank",False)
    print("\n=== VERDICT ===")
    print(f"  (1) market>solo +>=3 & p<.05: {go_solo}  (mean uplift {mkt-solo:+.2f})")
    print(f"  (2) market>shuffled p<.05:    {go_shuf}")
    print(f"  (3) market>confgreedy p<.05:  {go_conf}")
    print(f"  (4) market>skeptic_rerank:    {go_skep}")
    GO = go_solo and go_shuf and go_conf and go_skep
    print(f"  ==> {'GO — economy beats single-strong + all foils at equal budget' if GO else 'NO-GO / WEAK-GO — report honestly (see secondary cost-of-pass)'}")
    # secondary cost-of-pass
    print("\n=== SECONDARY: cost-of-pass micro-USD/solve (lower better) ===")
    for a in arms:
        v=[x for x in cost[a].values() if x>0 and x<10**12]
        if v: print(f"  {a:16} median={statistics.median(v):.0f}")
    sys.exit(0 if GO else 1)

if __name__ == "__main__":
    main()
