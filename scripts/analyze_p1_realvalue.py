#!/usr/bin/env python3
"""P1 real-value analyzer — does non-local price-routed tree search out-SOLVE the honest baselines?

Reads per-(theorem, arm, seed) lean_market_agent manifests + verify_chaintape replay reports from the P1
sweep dir. Primary outcome = SOLVED (omega_reached / verified_count>0) — binary, least gameable. The natural
paired test for matched binary outcomes is EXACT McNemar (one-sided, H1: market solves where the baseline
doesn't, more often than vice-versa), Holm-Bonferroni over {single, shuffled_price, no_price}. A cell is counted
ONLY if its replay is economic_state_reconstructed=true (forensic rule: replay-recompute, not byte-only).

Verdict A (the constitution's real value) GO iff market out-solves single AND shuffled_price AND no_price,
all Holm-p<alpha. NO-GO if market does NOT out-solve a baseline (b<=c). INCONCLUSIVE if direction-correct but
underpowered. A is the upside; B (replay-sound, Sybil-resistant governance) is the floor; A is never inferred from B.

Usage: python3 scripts/analyze_p1_realvalue.py --dir handover/evidence/p1_realvalue_2026-06-01 \
         --theorems lm_commute_pow,lm_sum_cubes,lm_ineq2 --arms market,single,shuffled_price,no_price \
         --seeds 1,2,3,4,5,6
"""
import json, os, argparse, math
from collections import defaultdict

def mcnemar_one_sided_greater(b, c):
    """Exact one-sided McNemar: P(X >= b | X ~ Binom(b+c, 0.5)). H1: market better (b discordant for market)."""
    n = b + c
    if n == 0:
        return 1.0
    # upper tail P(X >= b)
    p = sum(math.comb(n, k) for k in range(b, n + 1)) / (2 ** n)
    return min(1.0, p)

def holm(pvals):
    items = sorted(pvals.items(), key=lambda kv: kv[1]); m = len(items); adj = {}; prev = 0.0
    for i, (name, p) in enumerate(items):
        a = min(1.0, (m - i) * p); a = max(a, prev); adj[name] = a; prev = a
    return adj

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--dir", default="handover/evidence/p1_realvalue_2026-06-01")
    ap.add_argument("--theorems", default="lm_commute_pow,lm_sum_cubes,lm_ineq2")
    ap.add_argument("--arms", default="market,single,shuffled_price,no_price")
    ap.add_argument("--seeds", default="1,2,3,4,5,6")
    ap.add_argument("--alpha", type=float, default=0.05)
    a = ap.parse_args()
    thms = [x.strip() for x in a.theorems.split(",")]
    arms = [x.strip() for x in a.arms.split(",")]
    seeds = [x.strip() for x in a.seeds.split(",")]

    solved = defaultdict(dict)      # arm -> (thm,seed) -> 0/1
    excluded = []
    for thm in thms:
        for arm in arms:
            for s in seeds:
                c = os.path.join(a.dir, f"{thm}__{arm}__s{s}")
                mf, rr = c + ".json", c + ".replay.json"
                if not os.path.exists(mf):
                    excluded.append((thm, arm, s, "no-manifest")); continue
                d = json.load(open(mf))
                rep = json.load(open(rr)).get("economic_state_reconstructed") if os.path.exists(rr) else None
                if rep is not True:
                    excluded.append((thm, arm, s, "replay-not-clean")); continue
                sv = 1 if (d.get("omega_reached") or (d.get("verified_count", 0) > 0)) else 0
                solved[arm][(thm, s)] = sv
    if excluded:
        print(f"=== EXCLUDED {len(excluded)} cells (no-manifest / replay-not-clean) ===")
        for thm, arm, s, why in excluded[:30]:
            print(f"  {thm}/{arm}/s{s}: {why}")
        print()

    print("=== per-arm solve-rate (replay-clean cells only) ===")
    for arm in arms:
        vals = list(solved[arm].values()); n = len(vals); k = sum(vals)
        print(f"  {arm:15} solved {k}/{n}" + (f"  ({k/n:.2f})" if n else ""))
    print("\n  per-theorem solve counts (arm: [thm=solved/seeds ...]):")
    for arm in arms:
        parts = []
        for thm in thms:
            cells = [solved[arm].get((thm, s)) for s in seeds if (thm, s) in solved[arm]]
            parts.append(f"{thm.replace('lm_','')}={sum(cells)}/{len(cells)}")
        print(f"    {arm:15} {' '.join(parts)}")

    if "market" not in arms:
        print("\n(no market arm — cannot compute Verdict A)"); return
    foils = [f for f in ["single", "shuffled_price", "no_price"] if f in arms]
    print(f"\n=== Verdict A: market out-SOLVES {foils} (paired exact McNemar, Holm @ alpha={a.alpha}) ===")
    pv, disc = {}, {}
    for f in foils:
        keys = [k for k in solved["market"] if k in solved[f]]
        b = sum(1 for k in keys if solved["market"][k] == 1 and solved[f][k] == 0)  # market-only solves
        c = sum(1 for k in keys if solved["market"][k] == 0 and solved[f][k] == 1)  # foil-only solves
        pv[f] = mcnemar_one_sided_greater(b, c); disc[f] = (b, c, len(keys))
    adj = holm(pv)
    for f in foils:
        b, c, n = disc[f]
        print(f"  market vs {f:15} market-only={b} {f}-only={c} (n={n} paired)  p_holm={adj[f]:.4f}  "
              f"{'PASS' if adj[f] < a.alpha and b > c else 'fail'}")

    a_go = all((adj[f] < a.alpha and disc[f][0] > disc[f][1]) for f in foils)
    breached = [f for f in foils if disc[f][0] < disc[f][1]]       # market out-solved BY the foil (direction wrong)
    tie = [f for f in foils if disc[f][0] == disc[f][1]]
    print("\n=== VERDICT (held to Verdict B until A passes with fair baselines + replay-recompute) ===")
    if a_go:
        print("  A (price-causal NON-LOCAL tree search): GO — market out-solves single AND shuffled_price AND no_price")
    elif breached:
        print(f"  A: NO-GO — market did NOT out-solve {', '.join(breached)} (foil solved more): "
              f"price/non-locality is not the lever there")
    else:
        und = [f for f in foils if not (adj[f] < a.alpha and disc[f][0] > disc[f][1])]
        print(f"  A: INCONCLUSIVE — direction ok but underpowered/tied at alpha={a.alpha}: "
              f"{', '.join(f'{f}(+{disc[f][0]}/-{disc[f][1]},p={adj[f]:.3f})' for f in (und or tie))}")
        print("     (more seeds/theorems needed; a persistent tie at adequate power leans NO-GO)")
    print("  B (governance floor): every counted cell verify_chaintape economic_state_reconstructed=true (gated above).")

if __name__ == "__main__":
    main()
