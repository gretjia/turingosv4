#!/usr/bin/env python3
"""EMERGE Stage-1 decision-rule analyzer — evaluates the locked prereg rules against the run JSON.

Reads the lean_emergence Stage-1 output (per-model p_hat + joint matrix) and emits the verdicts the
prereg (handover/preregistration/EMERGE_STAGE1_PREREG_2026-05-31.json) committed BEFORE any result was read:
  A  verifier-coverage: does k-sample + Lean rescue theorems a single shot misses?
  B  heterogeneous combination: does union(models) cover more than the best single model, with >=1
     concrete theorem solved by a NON-globally-best model?
  + the Stage-2 gate (combination-target set size >= 3).

A "solve" = p_hat > 0 (the model produced an axiom-clean Lean proof in at least one of k draws; the harness
already enforced the {propext,Classical.choice,Quot.sound} whitelist, so every counted solve is clean).

Usage: python3 scripts/analyze_emerge_stage1.py /tmp/emerge_stage1.json
Exit 0 always (this is a read-out, not a gate); prints PROCEED-TO-STAGE2 or STAGE1-IS-THE-FINDING.
"""
import json, sys

def main():
    path = sys.argv[1] if len(sys.argv) > 1 else "/tmp/emerge_stage1.json"
    d = json.load(open(path))
    models = d["models"]
    k = d["k_samples"]
    pm = d["per_model"]
    # per-model solved set (p_hat>0) and the per-theorem p_hat
    solved = {m: {t for t, p in pm[m]["p_hat"].items() if p > 0} for m in models}
    all_thms = sorted({t for m in models for t in pm[m]["p_hat"]})
    n = len(all_thms)
    union = set().union(*solved.values()) if solved else set()
    best_m = max(models, key=lambda m: len(solved[m]))
    best_n = len(solved[best_m])

    print(f"=== EMERGE Stage-1 read — {n} theorems, k={k}, models={models} ===\n")

    # ── A: verifier-coverage (pass@1 proxy = mean p_hat; pass@k = solved-at-least-once / n) ──
    print("[A] VERIFIER-COVERAGE  (does sampling + Lean rescue low-p theorems?)")
    a_signal = False
    for m in models:
        mean_p = pm[m]["mean_p_hat"]              # ~ expected single-draw solve fraction (pass@1 proxy)
        passk = len(solved[m]) / n if n else 0.0  # solved at least once in k draws (empirical pass@k)
        lift = passk - mean_p
        flag = "  <-- coverage lift" if lift > 0.02 else ""
        if lift > 0.02:
            a_signal = True
        print(f"    {m:30} pass@1~{mean_p:.3f}  pass@{k}(union)={passk:.3f}  lift=+{lift:.3f}{flag}")
    print(f"    => A {'VALIDATED' if a_signal else 'NULL'}: k-sampling {'raises' if a_signal else 'does NOT raise'} solve-rate over single-shot.\n")

    # ── B: heterogeneous combination ──
    print("[B] HETEROGENEOUS COMBINATION  (union vs best single, at equal per-model budget)")
    for m in models:
        print(f"    {m:30} solved {len(solved[m]):2}/{n}")
    print(f"    best single model: {best_m} = {best_n}/{n}")
    print(f"    UNION(all {len(models)}): {len(union)}/{n}   uplift over best-single = +{len(union)-best_n}")
    # combination-target = solved by union but NOT by the best single model (what combination adds)
    combo_targets = sorted(union - solved[best_m])
    print(f"    combination-target theorems (solved by a NON-best model, not by {best_m}): {len(combo_targets)}")
    for t in combo_targets:
        who = [m for m in models if t in solved[m]]
        print(f"        {t:24} solved by: {who}")
    b_validated = len(union) > best_n and len(combo_targets) >= 1
    print(f"    => B {'VALIDATED' if b_validated else 'NULL'}: heterogeneity {'buys' if b_validated else 'does NOT buy'} coverage over the best single model.\n")

    # ── per-theorem solver multiplicity (who-solves-what map) ──
    print("[map] per-theorem solver multiplicity")
    by_mult = {}
    for t in all_thms:
        who = [m for m in models if t in solved[m]]
        by_mult.setdefault(len(who), []).append(t)
    for mult in sorted(by_mult, reverse=True):
        label = {len(models): "all-solve (easy/robust)", 0: "none-solve (ceiling)"}.get(mult, f"{mult}-of-{len(models)} (differentiating)")
        print(f"    {mult} solver(s) [{label}]: {len(by_mult[mult])}  {by_mult[mult] if mult not in (0,) else ''}")
    print()

    # ── Stage-2 gate ──
    stage2 = b_validated and len(combo_targets) >= 3
    print("=== VERDICT (against locked prereg) ===")
    print(f"  A verifier-coverage: {'VALIDATED' if a_signal else 'NULL'}")
    print(f"  B combination:       {'VALIDATED' if b_validated else 'NULL'}  (uplift +{len(union)-best_n}, targets {len(combo_targets)})")
    if stage2:
        print(f"  -> PROCEED-TO-STAGE2: combination-target set = {len(combo_targets)} (>=3). Stage-2 set: {combo_targets}")
    else:
        print(f"  -> STAGE1-IS-THE-FINDING: combination-target set = {len(combo_targets)} (<3) — next move is DEPTH (thinking-mode / tree-search), not breadth.")
    sys.exit(0)

if __name__ == "__main__":
    main()
