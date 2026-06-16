#!/usr/bin/env python3
"""Q2 (OBL-018): extract Lean-reject Failed proof bodies from P1 laterday manifests.

Per-node the manifest carries verdict/is_verified/reject_class/feedback/body_preview.
We emit one JSONL line per Lean-reaching FAILED node so het_dealign_exposure can measure
how many the 门0 realign fix would assemble differently (potential cured false negatives).

Honest scope: body_preview is truncated (~first ~120 chars), so a single-line preview
cannot reveal a line-2 indentation; exposure over those is a LOWER bound. We record
`multiline` so the downstream tool separates the rigorous (multi-line) subset.
"""
import json
import glob
import os
import sys

P1_DIR = "handover/evidence/p1_v4_laterday_full_2026-06-03"
OUT = sys.argv[1] if len(sys.argv) > 1 else "/tmp/q2_p1_failed_bodies.jsonl"

cells = 0
lean_failed = 0
multiline = 0
unsolved = 0
verified_total = 0
nodes_total = 0

with open(OUT, "w") as out:
    for mf in sorted(glob.glob(os.path.join(P1_DIR, "lm_*__*__s*.json"))):
        try:
            d = json.load(open(mf))
        except Exception:
            continue
        if not isinstance(d, dict) or "nodes" not in d:
            continue
        cells += 1
        cell = os.path.basename(mf)
        for nd in d.get("nodes", []):
            nodes_total += 1
            if nd.get("is_verified"):
                verified_total += 1
                continue
            rc = (nd.get("reject_class") or "").lower()
            if "lean" not in rc:  # only proofs that REACHED Lean (not parse/bear failures)
                continue
            body = nd.get("body_preview") or ""
            if not body.strip():
                continue
            lean_failed += 1
            fb = nd.get("feedback") or ""
            if "unsolved goals" in fb:
                unsolved += 1
            if "\n" in body:
                multiline += 1
            out.write(json.dumps({
                "id": nd.get("node_tx"),
                "cell": cell,
                "body": body,
                "feedback": fb[:90],
                "multiline": "\n" in body,
            }) + "\n")

print(json.dumps({
    "p1_cells": cells,
    "nodes_total": nodes_total,
    "verified_total": verified_total,
    "lean_reject_failed_nodes": lean_failed,
    "multiline_previews": multiline,
    "unsolved_goals_failures": unsolved,
    "out": OUT,
}, indent=2))
