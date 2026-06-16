#!/usr/bin/env python3
"""Q2 (OBL-018): BINDING positive control for the flip-rate harness.

Takes known-GOOD pool reference_bodies (E1 proved they Verify), deliberately
first-line-shallows them (line 1 flush, siblings still indented — the de-align shape the
conservative `dedent` cannot recover), and emits them as a verify set. het_dealign_
exposure `verify` MUST report these as flips (dedent FAILS, realign VERIFIES). If they do
NOT flip, the flip-rate harness is broken and any 0/N result over real data is
non-discriminative (calibration must BIND).
"""
import json
import sys

POOL = "tests/fixtures/lean_theorems_pool.jsonl"
OUT = sys.argv[1] if len(sys.argv) > 1 else "/tmp/q2_poscontrol.jsonl"
# theorems E1 verified clean (have known-good reference_body + mathlib preamble)
TARGETS = ["lm_det_zero", "lm_c", "lm_coeff_mul", "lm_e", "lm_lim1", "lm_nt_cop_cubic"]


def first_line_shallow(body: str) -> str:
    lines = body.split("\n")
    if not lines:
        return body
    lines[0] = lines[0].lstrip()
    return "\n".join(lines)


bank = {}
for l in open(POOL):
    d = json.loads(l)
    bank[d.get("id")] = d

n = 0
with open(OUT, "w") as out:
    for t in TARGETS:
        d = bank.get(t)
        if not d:
            continue
        ref = d.get("reference_body") or ""
        pre = d.get("preamble") or ""
        if not ref.strip() or not pre.strip():
            continue
        shallow = first_line_shallow(ref)
        if shallow == ref:
            # already col0 first line; force a shallow shape by trimming line 1 anyway
            pass
        out.write(json.dumps({
            "id": f"POSCTRL_{t}",
            "theorem": t,
            "body": shallow,
            "preamble": pre,
        }) + "\n")
        n += 1

print(json.dumps({"positive_control_size": n, "targets": TARGETS, "out": OUT}, indent=2))
