#!/usr/bin/env bash
# Aggregate TB-16 post-R3 Round 2 v2 results into SUMMARY.md.
# Format inspired by v3 Zeta ζ-Sum Run 6 visualization (DAG + golden
# path + role activity + market signals) but using v4's actual chain
# primitives (TB-9 agent identity + TB-11 NodePositions + TB-13
# CompleteSet + TB-14 PriceIndex + TB-15 Autopsy + TB-16 SANDBOX).

set -uo pipefail
cd /home/zephryj/projects/turingosv4

OUT_BASE="handover/evidence/tb_16_real_llm_arena_2026-05-04/post_r3_round2"
SUMMARY="$OUT_BASE/SUMMARY.md"

python3 - <<'PYEOF' > "$SUMMARY"
import json
import re
from pathlib import Path

OUT_BASE = Path("handover/evidence/tb_16_real_llm_arena_2026-05-04/post_r3_round2")
PROBLEMS = ["P1_baseline", "P2_challenge", "P3_completeset", "P4_bankruptcy",
            "P5_aime_hard", "P6_triple_probe", "P7_baseline_b", "P8_completeset_b"]

MECHANISM_ASSERTIONS = {
    "1_real_tape_persistence":      [4, 5, 6, 7, 12],
    "2_append_mechanism":            [4],
    "3_git_mechanism":               [8, 9, 10, 11, 14],
    "4_economic_mechanism":          [17, 18, 19, 20, 21, 22, 40],
    "5_boltzmann":                   [26],
    "6_information_shielding":       [28, 29, 30, 31, 39, 41],
    "7_broadcasting_mechanism":      [32, 33, 34, 35],
}

MECH_NAMES = {
    "1_real_tape_persistence":      "Real tape persistence",
    "2_append_mechanism":            "Append mechanism",
    "3_git_mechanism":               "Git mechanism",
    "4_economic_mechanism":          "Economic mechanism",
    "5_boltzmann":                   "Boltzmann scheduler",
    "6_information_shielding":       "Information shielding",
    "7_broadcasting_mechanism":      "Broadcasting mechanism",
}

def load(p, name):
    f = OUT_BASE / p / name
    if not f.exists() or f.stat().st_size == 0:
        return None
    txt = f.read_text()
    if name.endswith(".json"):
        try:
            return json.loads(txt)
        except Exception:
            return None
    return txt

def replay_match(p):
    a = OUT_BASE / p / "verdict.json"
    b = OUT_BASE / p / "verdict_replay.json"
    if not a.exists() or not b.exists():
        return None
    return a.read_bytes() == b.read_bytes()

def wilson_ci(k, n, z=1.96):
    if n == 0: return (0.0, 0.0)
    p = k/n; denom = 1 + z*z/n
    center = (p + z*z/(2*n)) / denom
    half = z * ((p*(1-p)/n + z*z/(4*n*n))**0.5) / denom
    return (max(0.0, center-half), min(1.0, center+half))

def section(dashboard, header, until=None):
    """Extract a section from dashboard.txt by header."""
    if dashboard is None:
        return None
    lines = dashboard.splitlines()
    out = []
    capture = False
    for line in lines:
        if line.startswith(header):
            capture = True
            out.append(line)
            continue
        if capture:
            if until and line.startswith(until):
                break
            out.append(line)
    return "\n".join(out)

print("# TB-16 Post-R3 Round 2 v2 — Constitutional Conformance Test Battery SUMMARY")
print()
print("**Date**: 2026-05-04 (post R3 closure commit `ce64d61` + runner-fix `8b5c94a`)")
print("**Problem set**: 8 × MiniF2F (mathd_algebra_171/11/96/67, mathd_numbertheory_961,")
print("                  aime_1997_p9, amc12b_2020_p5, triple-probe on mathd_algebra_171)")
print("**Swarm size**: N=5 (CONDITION=n5)")
print("**MAX_TX**: 20 per problem")
print("**LLM**: deepseek-v4-flash via proxy localhost:18080")
print("**Lean oracle**: turingosv3/experiments/minif2f_data_lean4 (mathlib cached)")
print("**Preseed**: TURINGOS_CHAINTAPE_PRESEED=1 enabled (full TB-10 user-task-mode path)")
print()
print("---")
print()

# §1 PROBLEM OUTCOME TABLE
print("## §1 Per-problem outcome (v3-style scaling table)")
print()
print("| Problem | Probe | Solved | Verified | tx_count | wall_ms | tokens | PPUT | L4 / L4.E | Tx kinds in chain |")
print("|---|---|---|---|---|---|---|---|---|---|")

problem_data = {}
n_attempted = 0
n_solved = 0
puts = []
for p in PROBLEMS:
    v = load(p, "verdict.json")
    pput = load(p, "pput_result.json")
    problem_data[p] = (v, pput)
    if v is None:
        print(f"| {p} | — | ✗ | — | — | — | — | — | — | — |")
        continue
    counts = v.get("tx_kind_counts", {})
    nonzero = sorted(k for k,n in counts.items() if n > 0)
    kinds_str = "+".join(nonzero) if nonzero else "—"
    tape = v.get("tape_root", {})
    l4 = tape.get("l4_count", 0); l4e = tape.get("l4e_count", 0)
    if pput:
        n_attempted += 1
        solved = pput.get("solved", False)
        verified = pput.get("verified", False)
        if solved:
            n_solved += 1
            puts.append(pput.get("pput", 0.0))
        probe_short = "vanilla"
        if "P2" in p: probe_short = "FORCE_CHAL=A2"
        elif "P3" in p: probe_short = "CSEED=1M"
        elif "P4" in p: probe_short = "FORCE_BANKR=A0"
        elif "P6" in p: probe_short = "triple"
        elif "P8" in p: probe_short = "CSEED=1.5M"
        print(f"| {p} | {probe_short} | {'✓' if solved else '✗'} | {'✓' if verified else '✗'} | {pput.get('tx_count','?')} | {pput.get('total_wall_time_ms','?')} | {pput.get('total_run_token_count','?')} | {pput.get('pput',0.0):.3f} | {l4}/{l4e} | {kinds_str} |")
    else:
        print(f"| {p} | — | — | — | — | — | — | — | {l4}/{l4e} | {kinds_str} |")
print()

# §2 PPUT distribution + Wilson CI
print("## §2 Capability signal (Art. I.2 三大统计信号)")
print()
total_pput_solved = sum(puts)
mean_pput = (total_pput_solved/len(puts)) if puts else 0.0
lo, hi = wilson_ci(n_solved, n_attempted)
print(f"- **Σ PPUT (solved only)**: {total_pput_solved:.4f}")
print(f"- **Mean PPUT (solved only)**: {mean_pput:.4f}")
print(f"- **Solve rate**: {n_solved}/{n_attempted} = {(n_solved/n_attempted*100 if n_attempted else 0):.1f}%")
print(f"- **95% Wilson CI**: [{lo*100:.1f}%, {hi*100:.1f}%]")
print()
print("> N=8 is too small for tight CI. Per `project_pput_ccl_arc`, full")
print("> H-VPPUT is heldout-49 with N>=20 runs/problem; this is FEASIBILITY")
print("> signal proving the architecture wires correctly end-to-end.")
print()
print("---")
print()

# §3 Tx-kind union
print("## §3 Tx-kind union across 8 chains")
print()
print("| TxKind | P1 | P2 | P3 | P4 | P5 | P6 | P7 | P8 | Total |")
print("|---|---|---|---|---|---|---|---|---|---|")
ALL_KINDS = ["work","verify","challenge","reuse","task_open","escrow_lock",
             "complete_set_mint","complete_set_redeem","market_seed",
             "finalize_reward","challenge_resolve","terminal_summary",
             "task_expire","task_bankruptcy"]
covered = set()
for k in ALL_KINDS:
    row = [k]
    total = 0
    for p in PROBLEMS:
        v = problem_data.get(p, (None, None))[0]
        if v is None:
            row.append("—"); continue
        n = v.get("tx_kind_counts", {}).get(k, 0)
        total += n
        if n > 0: covered.add(k)
        row.append(str(n) if n > 0 else "·")
    row.append(f"**{total}**" if total > 0 else "**0**")
    print("| " + " | ".join(row) + " |")
print()
print(f"**Tx kinds covered (union)**: {len(covered)} of 13 architect-required")
print(f"**Covered**: {', '.join(sorted(covered))}")
print(f"**Missing**: {', '.join(sorted(set(ALL_KINDS) - covered))}")
print()
print("---")
print()

# §4 7-Mechanism × FC × Audit conformance
print("## §4 7-Mechanism × Constitution × Flowchart × Audit conformance")
print()
print("Per-problem outcome for each mechanism's covering audit assertions.")
print("Legend: ✓ Pass · ○ Skipped (assertion not applicable to this chain) · ✗F Fail · ✗H Halt")
print()

for mech_id, ids in MECHANISM_ASSERTIONS.items():
    name = MECH_NAMES[mech_id]
    print(f"### Mechanism {mech_id[0]} — {name}")
    print()
    print("| id | name | P1 | P2 | P3 | P4 | P5 | P6 | P7 | P8 |")
    print("|---|---|---|---|---|---|---|---|---|---|")
    for assertion_id in ids:
        ref_v = None
        for p in PROBLEMS:
            v = problem_data.get(p, (None, None))[0]
            if v is not None:
                ref_v = v; break
        aname = "?"
        if ref_v:
            for a in ref_v.get("assertions", []):
                if a["id"] == assertion_id:
                    aname = a["name"]; break
        row = [str(assertion_id), aname]
        for p in PROBLEMS:
            v = problem_data.get(p, (None, None))[0]
            if v is None:
                row.append("—"); continue
            outcome = "?"
            for a in v.get("assertions", []):
                if a["id"] == assertion_id:
                    outcome = {"Pass":"✓", "Skipped":"○", "Fail":"✗F", "Halt":"✗H"}.get(a["result"], "?")
                    break
            row.append(outcome)
        print("| " + " | ".join(row) + " |")
    print()
print("---")
print()

# §5 Per-problem chain DAG (proposal flow + golden path) — drawn from dashboard.txt
print("## §5 Per-problem chain DAG (Proposal flow + Golden path)")
print()
for p in PROBLEMS:
    print(f"### {p}")
    print()
    dash = load(p, "dashboard.txt")
    if dash is None:
        print("(no dashboard)\n"); continue
    flow = section(dash, "§5 Proposal flow", "§6 Branch lineage")
    branch = section(dash, "§6 Branch lineage", "§7 Golden path")
    golden = section(dash, "§7 Golden path", "§8 Cross-checks")
    # Strip the header lines for compactness, keep the table
    print("```")
    if flow: print(flow.strip())
    print()
    if branch: print(branch.strip())
    print()
    if golden: print(golden.strip())
    print("```")
    print()
print("---")
print()

# §6 NodePositions + PriceIndex (TB-11 + TB-14 market signals)
print("## §6 Market signals (TB-11 NodePositions + TB-14 PriceIndex)")
print()
for p in PROBLEMS:
    print(f"### {p}")
    print()
    dash = load(p, "dashboard.txt")
    if dash is None:
        print("(no dashboard)\n"); continue
    pos = section(dash, "§13 TB-12 Node exposure", "§14")
    price = section(dash, "§14 TB-14 PriceIndex", "§15")
    print("```")
    if pos: print(pos.strip())
    print()
    if price: print(price.strip())
    print("```")
    print()
print("---")
print()

# §7 Autopsy + Markov (TB-15)
print("## §7 Privacy + broadcast (TB-15 Autopsy + Markov)")
print()
for p in PROBLEMS:
    print(f"### {p}")
    print()
    dash = load(p, "dashboard.txt")
    if dash is None: continue
    autopsy = section(dash, "§15 TB-15 Autopsy", "§16")
    sandbox = section(dash, "§16 TB-16 SANDBOX", None)
    print("```")
    if autopsy: print(autopsy.strip())
    print()
    if sandbox: print(sandbox.strip())
    print("```")
    print()
print("---")
print()

# §8 Tamper detection
print("## §8 Tamper detection (FC2 git-mechanism)")
print()
print("| Problem | flip_l4_byte | flip_cas_byte | truncate_l4_ref | total |")
print("|---|---|---|---|---|")
for p in PROBLEMS:
    t = load(p, "tamper_report.json")
    if t is None:
        print(f"| {p} | — | — | — | — |")
        continue
    cells = []
    for r in t.get("tamper_results", []):
        cells.append("✓" if r.get("detected") else "✗")
    while len(cells) < 3:
        cells.append("—")
    total = t.get("detected_count", "?")
    print(f"| {p} | {cells[0]} | {cells[1]} | {cells[2]} | {total}/3 |")
print()
print("---")
print()

# §9 Replay determinism (FC1 invariant)
print("## §9 Replay determinism (FC1 — chain replayable from disk-only inputs)")
print()
print("| Problem | verdict.json == verdict_replay.json (byte-id) |")
print("|---|---|")
for p in PROBLEMS:
    rep = replay_match(p)
    s = "✓" if rep else ("✗" if rep is not None else "—")
    print(f"| {p} | {s} |")
print()
print("---")
print()

# §10 Markov capsule chain (FC3 broadcast)
print("## §10 Markov capsule chain (FC3 broadcast — every session ≤ TB-15 head)")
print()
print("| Problem | capsule_id (16hex) | previous_capsule_cid (16hex) |")
print("|---|---|---|")
for p in PROBLEMS:
    f = OUT_BASE / p / "MARKOV_TB-16_2026-05-03.json"
    if not f.exists():
        print(f"| {p} | — | — |"); continue
    cap = json.loads(f.read_text())
    cid = bytes(cap.get("capsule_id", [])).hex()[:16] + "..." if cap.get("capsule_id") else "—"
    prev = bytes(cap.get("previous_capsule_cid", [])).hex()[:16] + "..." if cap.get("previous_capsule_cid") else "—"
    print(f"| {p} | `{cid}` | `{prev}` |")
print()
print("---")
print()

# §11 Bottom-line
print("## §11 Bottom-line conformance verdict")
print()
total_pass = total_fail = total_halt = total_skip = 0
all_proceed = True
all_replay = True
for p in PROBLEMS:
    v = problem_data.get(p, (None, None))[0]
    if v is None: continue
    total_pass += v.get("passed", 0)
    total_fail += v.get("failed", 0)
    total_halt += v.get("halted", 0)
    total_skip += v.get("skipped", 0)
    if v.get("verdict") != "PROCEED":
        all_proceed = False
    if not replay_match(p):
        all_replay = False

print(f"- **Aggregate audit_tape passed**: {total_pass}")
print(f"- **Aggregate failed**: {total_fail}")
print(f"- **Aggregate halted**: {total_halt}")
print(f"- **Aggregate skipped**: {total_skip}")
print(f"- **All chains PROCEED**: {'✓' if all_proceed else '✗'}")
print(f"- **All chains replay byte-identical**: {'✓' if all_replay else '✗'}")
print()
verdict = "PROCEED" if (all_proceed and all_replay and total_fail == 0 and total_halt == 0) else "BLOCK"
print(f"## VERDICT: {verdict}")
print()
print("All 7 mechanisms exercised on a fresh real-LLM substrate post R3 closure")
print("with TURINGOS_CHAINTAPE_PRESEED=1 enabling the full user-task-mode path:")
print("- mechanism 1-3 (tape/append/git): every chain audited from disk-only inputs + replay determinism + tamper detection")
print("- mechanism 4 (economic): id=18 + id=40 verify total_supply conserved at every prefix step; id=20 escrow matches locks")
print("- mechanism 5 (Boltzmann): structural fence id=26 + sequencer source has zero TB-14 type refs")
print("- mechanism 6 (info shielding): id=28 (raw + JSON-array form privacy scan) + id=39 (no LLM self-narrative) + id=41 (sandbox-prefix walker on L4 + L4.E + ALL AgentId fields)")
print("- mechanism 7 (broadcast): MarkovEvidenceCapsule chained to TB-15 head, flowchart_hashes parsed from TRACE_FLOWCHART_MATRIX")
PYEOF

echo "Wrote $SUMMARY"
