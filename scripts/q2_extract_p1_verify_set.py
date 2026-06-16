#!/usr/bin/env python3
"""Q2 (OBL-018): build the FLIP-RATE verify set from P1 laterday manifests.

Emits one JSONL line per Lean-reject FAILED node whose body_preview is COMPLETE
(length < 120 cap, i.e. the full body, re-compilable) and multi-line, with the theorem's
preamble looked up from the het pool (every P1 theorem is present there). het_dealign_
exposure's `verify` mode then re-runs real Lean on dedent(body) vs realign(body) to count
Failed→Verified flips — the actual de-align false negatives the 门0 fix cures.

Theorem id is taken from the manifest FILENAME (lm_<thm>__<arm>__s<seed>.json), which is
more reliable than parsing node_tx across arm-naming variants.
"""
import json
import glob
import os
import sys

P1_DIR = "handover/evidence/p1_v4_laterday_full_2026-06-03"
POOL = "tests/fixtures/lean_theorems_pool.jsonl"
CAP = 120  # body_preview truncation length observed in the manifests
OUT = sys.argv[1] if len(sys.argv) > 1 else "/tmp/q2_p1_verify_set.jsonl"

preamble_by_id = {}
for l in open(POOL):
    d = json.loads(l)
    if d.get("id") and d.get("preamble"):
        preamble_by_id[d["id"]] = d["preamble"]

def theorem_of(cell):
    # lm_<thm>__<arm>__s<seed>.json → lm_<thm>
    base = os.path.basename(cell)
    return base.split("__", 1)[0]

emitted = 0
skipped_no_preamble = 0
by_thm = {}
with open(OUT, "w") as out:
    for mf in sorted(glob.glob(os.path.join(P1_DIR, "lm_*__*__s*.json"))):
        try:
            d = json.load(open(mf))
        except Exception:
            continue
        if not isinstance(d, dict) or "nodes" not in d:
            continue
        cell = os.path.basename(mf)
        thm = theorem_of(cell)
        preamble = preamble_by_id.get(thm)
        for nd in d.get("nodes", []):
            if nd.get("is_verified"):
                continue
            rc = (nd.get("reject_class") or "").lower()
            if "lean" not in rc:
                continue
            body = nd.get("body_preview") or ""
            if not body.strip() or len(body) >= CAP or "\n" not in body:
                continue  # only COMPLETE (untruncated) multi-line bodies are re-compilable
            if not preamble:
                skipped_no_preamble += 1
                continue
            out.write(json.dumps({
                "id": nd.get("node_tx"),
                "theorem": thm,
                "cell": cell,
                "body": body,
                "preamble": preamble,
            }) + "\n")
            emitted += 1
            by_thm[thm] = by_thm.get(thm, 0) + 1

print(json.dumps({
    "verify_set_size": emitted,
    "skipped_no_preamble": skipped_no_preamble,
    "by_theorem": dict(sorted(by_thm.items(), key=lambda kv: -kv[1])),
    "out": OUT,
}, indent=2))
