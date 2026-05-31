#!/usr/bin/env python3
"""EMERGE Stage-2 decision-rule analyzer — Q1 sampling control + Q2 fair cross-family.

Combines three runs against the locked Stage-2 prereg (EMERGE_STAGE2_PREREG_2026-06-01.json):
  Stage-1 JSON  : v4-pro@k4 + V3.2@k4 (union@k4 = the treatment, 26/44)
  Run A JSON    : deepseek-v4-pro @ k8 (the equal-total-draws control for Q1)
  Run B JSON    : qwen3.7-max @ k4 (the fair cross-family arm for Q2)

Q1 — is the Stage-1 +8 real combination or just more draws?
  REAL        if  v4-pro@k8  <=  union(v4-pro@k4,V3.2@k4) - 3   (best-single at 8 draws still trails the union)
  SAMPLING    if  v4-pro@k8  >=  union@k4 - 1                    (best-single at equal total draws matches it)
Q2 — does a STRONG cross-family model add complementarity?
  HELPS       if  union(v4-pro@k4,V3.2@k4,qwen@k4) > 26  AND  >=1 qwen-only solve.

Usage: python3 scripts/analyze_emerge_stage2.py <stage1.json> [<runA_v4pro_k8.json>] [<runB_qwen_k4.json>]
Runs whichever verdicts the provided files allow (Q2 needs only stage1+runB; Q1 needs stage1+runA).
"""
import json, sys

def solved(d, model):
    pm = d.get("per_model", {})
    if model not in pm:
        raise SystemExit(f"model {model} not in {list(pm.keys())}")
    return {t for t, p in pm[model]["p_hat"].items() if p > 0}

def load(path):
    return json.load(open(path)) if path else None

def main():
    s1 = load(sys.argv[1] if len(sys.argv) > 1 else "/tmp/emerge_stage1.json")
    runA = load(sys.argv[2]) if len(sys.argv) > 2 else None   # v4-pro @ k8
    runB = load(sys.argv[3]) if len(sys.argv) > 3 else None   # qwen3.7-max @ k4

    v4_k4 = solved(s1, "deepseek-v4-pro")
    v32_k4 = solved(s1, "deepseek-ai/DeepSeek-V3.2")
    union_k4 = v4_k4 | v32_k4
    print(f"=== Stage-2 read (Stage-1 baseline: v4-pro@k4={len(v4_k4)}, V3.2@k4={len(v32_k4)}, union@k4={len(union_k4)}) ===\n")

    # ── Q1: sampling control ──
    if runA is not None:
        v4_k8 = solved(runA, "deepseek-v4-pro")
        print("[Q1] SAMPLING CONTROL — best-single at EQUAL TOTAL draws vs the 2-model union")
        print(f"    v4-pro @ k8 (8 draws/thm):           {len(v4_k8)}/44")
        print(f"    union(v4-pro@k4, V3.2@k4) (8 draws):  {len(union_k4)}/44")
        delta = len(union_k4) - len(v4_k8)
        if len(v4_k8) <= len(union_k4) - 3:
            verdict = f"REAL COMBINATION — union beats best-single@k8 by +{delta} at equal total draws (capability, not just sampling)"
        elif len(v4_k8) >= len(union_k4) - 1:
            verdict = f"JUST SAMPLING — best-single@k8 matches the union (Δ={delta}); the Stage-1 +8 was mostly more-draws"
        else:
            verdict = f"AMBIGUOUS — Δ={delta} (between the locked thresholds); report as inconclusive, consider a 2nd seed"
        print(f"    => Q1: {verdict}")
        only_union = sorted(union_k4 - v4_k8)
        print(f"    theorems the union gets that v4-pro@k8 still misses: {only_union}\n")
    else:
        print("[Q1] (skipped — Run A v4-pro@k8 JSON not provided yet)\n")

    # ── Q2: fair cross-family ──
    if runB is not None:
        qwen_k4 = solved(runB, "qwen3.7-max")
        union_3 = union_k4 | qwen_k4
        qwen_only = sorted(qwen_k4 - union_k4)
        print("[Q2] FAIR CROSS-FAMILY — does qwen3.7-max extend the union?")
        print(f"    qwen3.7-max @ k4:                    {len(qwen_k4)}/44  {sorted(qwen_k4)}")
        print(f"    union(v4-pro, V3.2):                 {len(union_k4)}/44")
        print(f"    union(v4-pro, V3.2, qwen3.7-max):    {len(union_3)}/44   (+{len(union_3)-len(union_k4)})")
        print(f"    qwen3.7-max-ONLY solves (no DeepSeek model got them): {len(qwen_only)}  {qwen_only}")
        if len(union_3) > len(union_k4) and qwen_only:
            verdict = f"CROSS-FAMILY HELPS — strong Qwen adds +{len(union_3)-len(union_k4)} the DeepSeek pair cannot reach (refutes the Stage-1 'diversity D~0' once the cross-family model is actually strong)"
        else:
            verdict = "CROSS-FAMILY NULL — even a strong Qwen is subsumed by the DeepSeek pair (diversity D~0 holds beyond strength)"
        print(f"    => Q2: {verdict}\n")
    else:
        print("[Q2] (skipped — Run B qwen3.7-max JSON not provided yet)\n")

    sys.exit(0)

if __name__ == "__main__":
    main()
