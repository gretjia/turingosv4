#!/usr/bin/env python3
"""TP-2 counted-sweep analyzer — the two-level verdict (A price-causal efficiency / B governance).

Reads per-(arm, seed) lean_hayek_alloc.v2 manifests, computes banked@B per arm, runs paired Wilcoxon
signed-rank (market vs each foil) with Holm-Bonferroni, and emits the verdict against the locked prereg
(T2_SHARED_STATE_PREREG_2026-06-01.json, v2 — the confound-free shared-state design; v1
T2_COUNTED_SWEEP_PREREG is superseded). Verdict A requires market > coordinator AND market > shuffled
(PRIMARY) AND market > flatbid, all Holm-p<0.05, every headline cell replay-green. A is NEVER inferred from B.

Usage: python3 scripts/analyze_t2_sweep.py --dir handover/evidence/t2_shared_sweep_2026-06-01 \
         --prefix t2s --seeds 8,9,10,11,12,13,14,15,16,17,18,19 --arms market,coordinator,shuffled,flatbid,random,index
Read-out only (never a gate); prints the per-arm table + comparisons + GO/INCONCLUSIVE/NO-GO.
"""
import json, sys, glob, os, itertools, argparse
from collections import defaultdict

def wilcoxon_one_sided_greater(diffs):
    """Paired Wilcoxon signed-rank, H1: median(diff) > 0. Prefers scipy (exact for small n); else a
    CONTINUITY-CORRECTED normal approximation (conservative). QC 2026-06-01: the uncorrected approx was
    anti-conservative for small n (n=5 all-positive gave 0.022 vs exact ~0.031) and is not used for headlines."""
    d = [x for x in diffs if x != 0]
    n = len(d)
    if n == 0:
        return 1.0
    try:
        from scipy.stats import wilcoxon  # exact/auto for the counted sweep when installed
        return float(wilcoxon(d, alternative="greater", zero_method="wilcox").pvalue)
    except Exception:
        pass
    import math
    s = sorted(range(n), key=lambda i: abs(d[i]))
    rk = [0.0]*n
    j = 0
    while j < n:                                  # average ranks for |d| ties
        k = j
        while k+1 < n and abs(d[s[k+1]]) == abs(d[s[j]]):
            k += 1
        avg = (j + k)/2.0 + 1.0
        for t in range(j, k+1):
            rk[s[t]] = avg
        j = k+1
    w_plus = sum(rk[i] for i in range(n) if d[i] > 0)
    mean = n*(n+1)/4.0
    var = n*(n+1)*(2*n+1)/24.0
    if var == 0:
        return 1.0
    z = (w_plus - mean - 0.5) / math.sqrt(var)    # -0.5 continuity correction (upper tail) → conservative
    return 0.5 * math.erfc(z / math.sqrt(2))

def holm(pvals):
    """Holm-Bonferroni; returns dict name->adjusted_p."""
    items = sorted(pvals.items(), key=lambda kv: kv[1])
    m = len(items)
    adj = {}
    prev = 0.0
    for i, (name, p) in enumerate(items):
        a = min(1.0, (m - i) * p)
        a = max(a, prev)
        adj[name] = a
        prev = a
    return adj

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--dir", default="handover/evidence/t2_shared_sweep_2026-06-01")
    ap.add_argument("--prefix", default="t2s")
    ap.add_argument("--seeds", default="8,9,10,11,12,13,14,15,16,17,18,19")
    ap.add_argument("--arms", default="market,coordinator,shuffled,flatbid,random,index")
    ap.add_argument("--alpha", type=float, default=0.05)
    a = ap.parse_args()
    seeds = [s.strip() for s in a.seeds.split(",")]
    arms = [x.strip() for x in a.arms.split(",")]

    banked = defaultdict(dict)   # arm -> seed -> banked@B
    replay = defaultdict(dict)
    excluded = []                # (arm, seed, reason) — pre-committed exclusions
    for arm in arms:
        for seed in seeds:
            mf = os.path.join(a.dir, f"{a.prefix}_{arm}_{seed}.json")
            rr = os.path.join(a.dir, f"{a.prefix}_rr_{arm}_{seed}.json")
            if not os.path.exists(mf):
                excluded.append((arm, seed, "no-manifest (e.g. coordinator hard-fail)")); continue
            d = json.load(open(mf))
            # audit MAJOR-4 (refined by the calibration pilot): banked@B = banked by repairs STARTED under B
            # (the gate stops new repairs once reasoner_tok >= B), so a cell legitimately overshoots by ONE
            # in-progress repair (<= max_tokens 600 + margin). Exclude only EGREGIOUS overshoot (> B + 800 =
            # a bug/runaway), not the inherent bounded one-repair overshoot. Consistent + symmetric across arms.
            rct = d.get("reasoner_completion_tokens", 0); rbt = d.get("reasoner_budget_tok", 10**12)
            if isinstance(rct,(int,float)) and isinstance(rbt,(int,float)) and rct > rbt + 800:
                excluded.append((arm, seed, f"egregious over-budget reasoner_tok {rct} > B+800 ({rbt}+800)")); continue
            rep = (json.load(open(rr)).get("replay_clean") if os.path.exists(rr) else None)
            if rep is False:
                excluded.append((arm, seed, "replay-FAIL")); continue   # replay-fail cells excluded from headline
            banked[arm][seed] = d.get("banked_at_B")
            replay[arm][seed] = rep
    if excluded:
        print("=== EXCLUDED cells (pre-committed: hard-fail / over-budget / replay-fail) ===")
        for arm, seed, why in excluded:
            print(f"  {arm} seed{seed}: {why}")
        print()

    print("=== per-arm banked@B (and replay) ===")
    for arm in arms:
        row = [f"{banked[arm].get(s,'-')}{'' if replay[arm].get(s) in (True,None) else '(REPLAY-FAIL)'}" for s in seeds]
        vals = [v for v in banked[arm].values() if isinstance(v,int)]
        mean = sum(vals)/len(vals) if vals else 0
        allgreen = all(replay[arm].get(s) in (True,None) for s in seeds)
        print(f"  {arm:13} seeds[{','.join(row)}]  mean={mean:.2f}  replay_all_green={allgreen}")

    if "market" not in arms:
        print("\n(no market arm — cannot compute Verdict A)"); return
    foils = [f for f in ["coordinator","shuffled","flatbid"] if f in arms]
    print(f"\n=== Verdict A causal gates: market > {foils} (paired Wilcoxon, Holm @ alpha={a.alpha}) ===")
    pvals = {}; deltas = {}
    common = [s for s in seeds if isinstance(banked['market'].get(s),int)]
    for f in foils:
        diffs = [banked['market'][s]-banked[f][s] for s in common if isinstance(banked[f].get(s),int)]
        pvals[f] = wilcoxon_one_sided_greater(diffs)
        deltas[f] = sum(diffs)/len(diffs) if diffs else 0
    adj = holm(pvals)
    npos = {f: sum(1 for s in common if isinstance(banked[f].get(s),int) and banked['market'][s]-banked[f][s] > 0) for f in foils}
    nneg = {f: sum(1 for s in common if isinstance(banked[f].get(s),int) and banked['market'][s]-banked[f][s] < 0) for f in foils}
    for f in foils:
        print(f"  market vs {f:12} mean_delta={deltas[f]:+.2f}  (+{npos[f]}/-{nneg[f]} seeds)  p_holm={adj[f]:.4f}  {'PASS' if adj[f]<a.alpha and deltas[f]>0 else 'fail'}")
    print(f"  (n={len(common)} paired seeds)")

    # Per prereg: distinguish a DIRECTION breach (delta<=0 → genuine NO-GO for that contrast) from a
    # positive-but-nonsignificant delta (underpowered → INCONCLUSIVE, NOT NO-GO). A NEVER inferred from B.
    a_go = all((adj[f]<a.alpha and deltas[f]>0) for f in foils)
    breached = [f for f in foils if deltas[f] < 0]             # market STRICTLY WORSE → direction wrong (real NO-GO)
    underpowered = [f for f in foils if deltas[f] >= 0 and not (adj[f] < a.alpha and deltas[f] > 0)]  # tie/positive-nonsig
    print("\n=== VERDICT (vs locked prereg) ===")
    if a_go:
        print("  A (price-causal efficiency): GO — market > coordinator AND shuffled AND flatbid (Holm-p<alpha, dir>0)")
    elif breached:
        tag = "PRIMARY firewall" if "shuffled" in breached else "efficiency gate"
        print(f"  A: NO-GO ({tag}) — market did NOT out-bank {', '.join(f'{f}(d={deltas[f]:+.2f})' for f in breached)}: the win, if any, is not price coordination" if "shuffled" in breached
              else f"  A: NO-GO (efficiency) — market <= {', '.join(f'{f}(d={deltas[f]:+.2f})' for f in breached)} (central planning / structure allocates >= as well as price)")
    else:
        print(f"  A: INCONCLUSIVE — direction correct on all foils but underpowered at alpha={a.alpha}: {', '.join(f'{f}(d={deltas[f]:+.2f},p={adj[f]:.3f})' for f in underpowered)}")
        print("     (prereg: positive-but-nonsignificant delta = inconclusive, NOT NO-GO; needs more seeds / MDE check)")
    print("  B (institutional governance): held iff every market cell is replay-green + Sybil-resistant +")
    print("     Goodhart-shielded + failures-on-tape (checked separately; A is NEVER inferred from B).")
    sys.exit(0)

if __name__ == "__main__":
    main()
