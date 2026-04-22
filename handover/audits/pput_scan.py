#!/usr/bin/env python3
"""PPUT raw scan — the authoritative reproduce script for PPUT_RAW_DATA_2026-04-22.md.

Usage: python3 handover/audits/pput_scan.py

Output: Markdown table to stdout. Each row comes directly from one jsonl file;
no aggregation, no Phase labeling, no inference. Per C-066.
"""
import json
from pathlib import Path

logs_dir = Path(__file__).parent.parent.parent / "experiments/minif2f_v4/logs/"

print("| Timestamp | Solved | ΣPPUT | MeanPPUT | MaxDepth | Depth≥10 | Σdepth≥10 | gp_paths | tools |")
print("|---|---|---|---|---|---|---|---|---|")

for f in sorted(logs_dir.glob("templadder_n8_*.jsonl")):
    rows = [json.loads(l) for l in f.open()]
    if not rows:
        continue
    solved = [r for r in rows if r['has_golden_path']]
    if not solved:
        continue
    sigma = sum(r['pput'] for r in rows)
    mean_s = sum(r['pput'] for r in solved) / len(solved)
    max_d = max(r['gp_node_count'] for r in solved)
    deep = [r for r in solved if r['gp_node_count'] >= 10]
    deep_sigma = sum(r['pput'] for r in deep)
    gp_paths = sorted(set(r.get('gp_path', '?') for r in solved))
    tool_agg = {}
    for r in solved:
        for k, v in r.get('tool_dist', {}).items():
            tool_agg[k] = tool_agg.get(k, 0) + v
    keys = ['complete', 'append', 'step', 'omega_wtool', 'step_partial_ok', 'complete_via_tape']
    tools_str = ','.join(f"{k}={tool_agg.get(k, 0)}" for k in keys if tool_agg.get(k, 0) > 0)
    ts = f.stem.replace('templadder_n8_', '')
    print(f"| {ts} | {len(solved)}/{len(rows)} | {sigma:.2f} | {mean_s:.3f} | {max_d} | {len(deep)} | {deep_sigma:.2f} | {'/'.join(gp_paths)} | {tools_str} |")
