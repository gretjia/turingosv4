#!/usr/bin/env python3
"""P1 v4 CORRECTED analyzer — 3-layer stats (existence / reliability / causal) over the two
pre-registrations, with compute telemetry + topology decomposition.

Design (post external-audit VETO): on HARD theorems where `single` RELIABLY FAILS (0/seeds), a
crack = a confound-shielded organization win. The autonomous arm is now DECOUPLED (Stage-2 proof
prompt byte-identical to market, enforced by the binary's --self-test), so autonomous-vs-market is
a single-variable (who-picks-the-parent) contrast, not a prompt-context confound.

PREREG_1 (market-Hayek):     market vs {single, parallel, shuffled_price, no_price}
PREREG_2 (autonomous-free):  autonomous vs {single, market, parallel, no_price}
Each counted cell must be verify_chaintape replay-clean AND (for a crack) axiom-clean (the binary's
inline #print-axioms gate already enforces axiom-clean before omega; we re-surface the axiom list).

Usage: python3 scripts/analyze_p1_v4.py --dir handover/evidence/p1_v4_2026-06-02 \
         --arms single,single_restart,single_tree_no_price,parallel,parallel_restart,no_price,shuffled_price,market,autonomous \
         --seeds 1,2,3
(--theorems defaults to the 18-candidate hard set.)
"""
import json, os, argparse, math
from collections import defaultdict

WHITELIST = {"propext", "Classical.choice", "Quot.sound"}
CANDIDATE_HARD = ("lm_deriv1,lm_ineq1,lm_ineq2,lm_coeff_mul,lm_nt_gcd2,lm_median,lm_c,lm_deriv2,"
                  "lm_det_zero,lm_e,lm_f,lm_fact,lm_finset_sup,lm_ineq3,lm_lim1,lm_natdeg_pow,"
                  "lm_nt_cop_cubic,lm_probe1")

def mcnemar_one_sided_greater(b, c):
    n = b + c
    if n == 0:
        return 1.0
    return min(1.0, sum(math.comb(n, k) for k in range(b, n + 1)) / (2 ** n))

def holm(pvals):
    items = sorted(pvals.items(), key=lambda kv: kv[1]); m = len(items); adj = {}; prev = 0.0
    for i, (name, p) in enumerate(items):
        a = min(1.0, (m - i) * p); a = max(a, prev); adj[name] = a; prev = a
    return adj

def wilson(k, n, z=1.96):
    """Wilson score interval (pure-python, no scipy) — matches the constitution's CI helper."""
    if n == 0:
        return (None, None, None)
    p = k / n
    denom = 1 + z * z / n
    center = (p + z * z / (2 * n)) / denom
    half = z * math.sqrt(p * (1 - p) / n + z * z / (4 * n * n)) / denom
    return (p, max(0.0, center - half), min(1.0, center + half))

def jeffreys_mean(k, n):
    """Beta(k+.5, n-k+.5) posterior mean — the reliability point estimate."""
    return (k + 0.5) / (n + 1) if n >= 0 else None

def load(d, thms, arms, seeds):
    solved = defaultdict(dict)      # arm -> (thm,seed) -> 0/1
    axioms = defaultdict(dict)      # arm -> (thm,seed) -> [axioms] (for cracks)
    route = defaultdict(lambda: defaultdict(int))
    compute = defaultdict(lambda: defaultdict(int))  # arm -> field -> sum
    excluded = []
    for thm in thms:
        for arm in arms:
            for s in seeds:
                c = os.path.join(d, f"{thm}__{arm}__s{s}")
                mf, rr = c + ".json", c + ".replay.json"
                if not os.path.exists(mf):
                    excluded.append((thm, arm, s, "no-manifest")); continue
                m = json.load(open(mf))
                rep = json.load(open(rr)).get("economic_state_reconstructed") if os.path.exists(rr) else None
                if rep is not True:
                    excluded.append((thm, arm, s, "replay-not-clean")); continue
                ok = 1 if (m.get("omega_reached") or m.get("verified_count", 0) > 0) else 0
                solved[arm][(thm, s)] = ok
                if ok:
                    # crack axioms live on the verified NODE (inline #print-axioms gate already
                    # guaranteed ⊆ whitelist before omega; we re-surface them for the report).
                    crack_ax = []
                    for nd in m.get("nodes", []):
                        if nd.get("is_verified"):
                            crack_ax = nd.get("axioms", []) or []
                            break
                    axioms[arm][(thm, s)] = crack_ax
                for f in ("proposal_llm_calls", "route_llm_calls", "bear_llm_calls",
                          "proof_prompt_tokens", "route_prompt_tokens", "bear_prompt_tokens",
                          "completion_tokens", "total_model_tokens", "lean_verifies"):
                    compute[arm][f] += m.get(f, 0) or 0
                for k in ("route_valid_index_hit", "route_deliberate_fresh_root",
                          "route_hallucinated_out_of_range"):
                    route[arm][k] += m.get(k, 0) or 0
    return solved, axioms, route, compute, excluded

def solve_count(solved, arm, thm, seeds):
    return sum(solved.get(arm, {}).get((thm, s), 0) for s in seeds)

def cell_count(solved, arm, thm, seeds):
    return sum(1 for s in seeds if (thm, s) in solved.get(arm, {}))

def mcnemar_family(solved, focus, controls, thms_scope, seeds):
    pv, disc = {}, {}
    for y in controls:
        keys = [(t, s) for t in thms_scope for s in seeds
                if (t, s) in solved.get(focus, {}) and (t, s) in solved.get(y, {})]
        b = sum(1 for k in keys if solved[focus][k] == 1 and solved[y][k] == 0)
        c = sum(1 for k in keys if solved[focus][k] == 0 and solved[y][k] == 1)
        pv[f"{focus}>{y}"] = mcnemar_one_sided_greater(b, c); disc[f"{focus}>{y}"] = (b, c, len(keys))
    adj = holm(pv)
    return pv, disc, adj

def confirmed_wins(solved, focus, controls, hard, seeds):
    wins = []
    for thm in hard:
        if solve_count(solved, focus, thm, seeds) >= 1 and \
           all(solve_count(solved, y, thm, seeds) == 0 for y in controls):
            wins.append(thm)
    return wins

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--dir", default="handover/evidence/p1_v4_2026-06-02")
    ap.add_argument("--theorems", default=CANDIDATE_HARD)
    ap.add_argument("--arms", default="single,single_restart,single_tree_no_price,parallel,"
                                      "parallel_restart,no_price,shuffled_price,market,autonomous")
    ap.add_argument("--seeds", default="1,2,3")
    ap.add_argument("--alpha", type=float, default=0.05)
    a = ap.parse_args()
    thms = [x.strip() for x in a.theorems.split(",")]
    arms = [x.strip() for x in a.arms.split(",")]
    seeds = [x.strip() for x in a.seeds.split(",")]

    solved, axioms, route, compute, excluded = load(a.dir, thms, arms, seeds)
    if excluded:
        print(f"=== EXCLUDED {len(excluded)} cells (no-manifest / replay-not-clean) ===")
        for t, ar, s, why in excluded[:60]:
            print(f"  {t}/{ar}/s{s}: {why}")
        print()

    # HARD floor: single solves 0/seeds
    hard = []
    if "single" in arms:
        for thm in thms:
            if cell_count(solved, "single", thm, seeds) > 0 and solve_count(solved, "single", thm, seeds) == 0:
                hard.append(thm)
    print(f"=== HARD floor: {len(hard)}/{len(thms)} theorems where single solved 0/seeds ===")
    print("  hard:", ", ".join(hard) if hard else "(none — single solved everything attempted)")

    print("\n=== per-theorem solve matrix (X=solved, .=fail, -=missing); HARD marked ===")
    print(f"  {'theorem':16} " + " ".join(f"{ar[:11]:^11}" for ar in arms))
    for thm in thms:
        cols = []
        for ar in arms:
            cells = sorted([(s, solved[ar][(thm, s)]) for s in seeds if (thm, s) in solved.get(ar, {})])
            cols.append("".join("X" if v else "." for _, v in cells) or "-")
        print(f"  {thm:16} " + " ".join(f"{c:^11}" for c in cols) + ("  <-HARD" if thm in hard else ""))

    # LAYER 1 — EXISTENCE
    print("\n##### LAYER 1 — EXISTENCE (axiom-clean cracks on single-0 theorems) #####")
    any_crack = False
    for ar in arms:
        if ar == "single":
            continue
        for thm in hard:
            for s in seeds:
                if solved.get(ar, {}).get((thm, s)) == 1:
                    ax = axioms.get(ar, {}).get((thm, s), [])
                    clean = (not ax) or set(ax).issubset(WHITELIST)
                    any_crack = True
                    tag = "AXIOM-CLEAN" if clean else f"AXIOM-DIRTY {ax}"
                    print(f"  *** {ar} CRACKED {thm} (seed {s}) — {tag} — single solved 0 ***")
    if not any_crack:
        print("  (no crack on any single-0 theorem in replay-clean cells)")

    # LAYER 2 — RELIABILITY
    print("\n##### LAYER 2 — RELIABILITY (solve rate + Wilson 95% CI + Jeffreys mean, on HARD set) #####")
    print(f"  {'arm':22} {'k/n':>8}  {'rate':>6}  {'Wilson95':>16}  {'Jeffreys':>8}")
    for ar in arms:
        k = sum(solve_count(solved, ar, t, seeds) for t in hard)
        n = sum(cell_count(solved, ar, t, seeds) for t in hard)
        p, lo, hi = wilson(k, n)
        jm = jeffreys_mean(k, n)
        ci = f"[{lo:.3f},{hi:.3f}]" if lo is not None else "n/a"
        print(f"  {ar:22} {f'{k}/{n}':>8}  {(p if p is not None else 0):>6.3f}  {ci:>16}  {(jm if jm is not None else 0):>8.3f}")

    # LAYER 3 — CAUSAL (the two pre-registrations)
    print("\n##### LAYER 3 — CAUSAL (pre-registered, Holm @ alpha={:.2f}) #####".format(a.alpha))
    for tag, focus, controls in (
        ("PREREG_1 market-Hayek", "market", ["single", "parallel", "shuffled_price", "no_price"]),
        ("PREREG_2 autonomous-freechoice", "autonomous", ["single", "market", "parallel", "no_price"]),
    ):
        if focus not in arms:
            continue
        ctrl = [y for y in controls if y in arms]
        wins = confirmed_wins(solved, focus, ctrl, hard, seeds)
        pv, disc, adj = mcnemar_family(solved, focus, ctrl, hard, seeds)
        print(f"\n  --- {tag}: {focus} vs {{{', '.join(ctrl)}}} ---")
        print(f"    CONFIRMED_WIN theorems (hard & {focus}>=1 & all controls 0): "
              f"{wins if wins else '(none)'}  [{len(wins)}]")
        sig = False
        for name in pv:
            b, c, n = disc[name]
            ok = adj[name] < a.alpha and b > c
            sig = sig or ok
            print(f"    {name:34} {name.split('>')[0]}-only={b} {name.split('>')[1]}-only={c} (n={n})  "
                  f"p_holm={adj[name]:.4f}  {'PASS' if ok else '-'}")
        print(f"    => {tag}: {'SIGNIFICANT win over the control family' if sig else 'NOT significant'}; "
              f"CONFIRMED_WINs={len(wins)}")

    # COMPUTE telemetry (the auditor's parity check, made visible)
    print("\n##### COMPUTE telemetry (proposal/route/bear calls + tokens; the parity check) #####")
    print(f"  {'arm':22} {'proof':>7} {'route':>7} {'bear':>7} {'verifs':>7} {'tot_tok':>10} {'tok/solve(hard)':>16}")
    for ar in arms:
        k = sum(solve_count(solved, ar, t, seeds) for t in hard)
        tot = compute[ar]["total_model_tokens"]
        tps = (tot / k) if k else None
        print(f"  {ar:22} {compute[ar]['proposal_llm_calls']:>7} {compute[ar]['route_llm_calls']:>7} "
              f"{compute[ar]['bear_llm_calls']:>7} {compute[ar]['lean_verifies']:>7} {tot:>10} "
              f"{(f'{tps:.0f}' if tps else 'n/a'):>16}")

    # TOPOLOGY decomposition (solve-rate deltas on HARD set)
    print("\n##### TOPOLOGY decomposition (HARD-set solve rate, isolates each organizational feature) #####")
    contrasts = [
        ("single", "single_restart", "backtrack/root-restart (1 agent)"),
        ("single_restart", "single_tree_no_price", "root-or-last vs own-history tree"),
        ("single_tree_no_price", "parallel", "single-agent tree vs multi independent"),
        ("parallel", "parallel_restart", "backtrack (multi agent)"),
        ("parallel_restart", "no_price", "independent vs shared tree (no price)"),
        ("no_price", "shuffled_price", "random vs shuffled-price node"),
        ("shuffled_price", "market", "shuffled vs REAL price (the PRICE signal)"),
        ("market", "autonomous", "forced softmax vs free-choice (the ROUTER)"),
    ]
    def rate(ar):
        k = sum(solve_count(solved, ar, t, seeds) for t in hard)
        n = sum(cell_count(solved, ar, t, seeds) for t in hard)
        return (k / n) if n else None
    for lo_arm, hi_arm, label in contrasts:
        if lo_arm in arms and hi_arm in arms:
            rl, rh = rate(lo_arm), rate(hi_arm)
            d = (rh - rl) if (rl is not None and rh is not None) else None
            ds = f"{d:+.3f}" if d is not None else "n/a"
            print(f"  {lo_arm:20} -> {hi_arm:20} Δrate={ds:>8}  ({label})")

    # ROUTE honesty (autonomous)
    if "autonomous" in arms:
        r = route["autonomous"]; tot = sum(r.values())
        print("\n##### ROUTE honesty (autonomous free-choice genuine?) #####")
        print(f"  valid_index_hit={r['route_valid_index_hit']} fresh_root={r['route_deliberate_fresh_root']} "
              f"hallucinated={r['route_hallucinated_out_of_range']} (total={tot})")
        if tot:
            hr = r['route_hallucinated_out_of_range'] / tot
            print(f"  => hallucination {hr:.1%}" + ("  WARN: free-routing claim weakened" if hr > 0.2 else "  (low — deliberate)"))

    print("\n=== VERDICT scaffold (held to §17 G1-G6; A never inferred from replay-clean B) ===")
    print("  Fill from the layers above: existence (any axiom-clean crack?), reliability (CIs),")
    print("  causal (PREREG_1 + PREREG_2 significance + CONFIRMED_WINs). Honest claim per the prereg.")

if __name__ == "__main__":
    main()
