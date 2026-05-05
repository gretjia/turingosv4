#!/usr/bin/env python3
"""TB-16.x.2.4 Multi-WorkTx + Boltzmann RUNTIME — Gemini Class 3 dual audit.

Per memory feedback_dual_audit + feedback_dual_audit_conflict: VETO > CHALLENGE > PASS.
Architectural strategic angle. Independent of Codex (impl-paranoid).
"""
import json
import os
import sys
import time
import urllib.request
from pathlib import Path

ROOT = Path("/home/zephryj/projects/turingosv4")
ENV_FILES = [ROOT / ".env"]
ROUND = os.environ.get("TB16X24_AUDIT_ROUND", "R1")
OUT = ROOT / f"handover/audits/GEMINI_TB_16_X_2_4_AUDIT_2026-05-05_{ROUND}.md"

if OUT.exists():
    print(f"[gemini tb-16.x.2.4] error: {OUT} already exists; refusing to overwrite", file=sys.stderr)
    sys.exit(2)


def load_env():
    env = {}
    for fp in ENV_FILES:
        if not fp.exists():
            continue
        for line in fp.read_text().splitlines():
            if "=" in line and not line.strip().startswith("#"):
                k, v = line.split("=", 1)
                env.setdefault(k.strip(), v.strip().strip('"').strip("'"))
    return env


env = load_env()
if "GEMINI_API_KEY" not in env:
    print("[gemini tb-16.x.2.4] GEMINI_API_KEY not in .env", file=sys.stderr)
    sys.exit(2)

brief = """# Gemini TB-16.x.2.4 Audit — Multi-WorkTx + Boltzmann RUNTIME (Class 3 dual external audit)

**Role**: skeptical adversarial reviewer. Independent of Codex (impl-paranoid angle). Conservative merge rule (memory `feedback_dual_audit_conflict`): VETO > CHALLENGE > PASS.

**Mandate**: TB-16.x.2.4 (commit b5118fd17b0f8666a25453239104e54406e9f80b on main; pre-audit) shipped under **Class 3 integration smoke envelope** per umbrella charter `TB-16.x.2_charter_2026-05-04.md` §0. **You are the architectural strategic reviewer**; Codex covers implementation paranoia in parallel.

## Audit target

The commit adds:
1. `experiments/minif2f_v4/src/bin/evaluator.rs` — new env-var hook `TURINGOS_FORCE_BOLTZMANN_SEED_WORKTXS=<staker>:<count>:<stake_micro>` that injects N (≥3) real-signed WorkTxs serially. Each iteration: snapshot bus → call `boltzmann_select_parent_v2(snap.price_index, snap.mask_set, &policy, &mut seeded_rng)` → parent_tx fallback (v2_pick OR last-produced-WorkTx OR None) → write ProposalTelemetry to CAS → construct real-signed WorkTx → submit + await commit.

2. `src/runtime/audit_assertions.rs` (already in main from .2.5 commit f1216f0) — Layer E supplemental assertion id=43 `boltzmann_parent_selection_diversity` — computes Shannon entropy of ProposalTelemetry.parent_tx distribution across same-task WorkTxs; halts if entropy < 0.25 (Art II.2.1 alarm threshold) on any task with ≥3 WorkTxs.

3. `genesis_payload.toml` — R-014 rehash for evaluator.rs (`d39c67d1` → `5a989d15`).

4. `handover/tests/scripts/run_tb_16_x_2_4_smoke_2026-05-05.sh` — NEW smoke script with python3 JSON ship-gate (work>=3 + id43 == Pass).

## DEVIATION-FROM-CHARTER under audit

Charter §2 Atom 2.4 declared **STEP_B_PROTOCOL TRIGGERED** because the file plan listed `src/state/sequencer.rs`. The implementation took position (per `feedback_architect_deviation_stance`):
- "verify boltzmann_select_parent_v2 is called in WorkTx admission path" interpretation: charter's intent was sequencer-side admission; v2 selector is ALREADY called proposal-side at evaluator.rs:1828 (`_v2_canonical_pick`).
- For SG-16.x.2.4 (≥3 WorkTx with parent_selection_entropy ≥ 0.5), the parent_tx record lives in ProposalTelemetry (CAS object, proposal-time data) — sequencer-side admission has no parent_tx field on WorkTx.
- No sequencer.rs edit; STEP_B not triggered. Class 3 dual audit STILL applied (this audit).

**Question for you**: is this deviation defensible? Or does it dodge a load-bearing charter requirement (specifically: should the v2 pick be wired into the SEQUENCER's parent_tx selection, not just recorded proposal-side)?

## Smoke evidence

```
verdict.json: passed=35 failed=0 halted=0 skipped=8 verdict=PROCEED
              tx_kind_counts.work=4   ← multi-WorkTx target met
              tape_root.l4_count=7    cas_object_count=24
              feature_coverage.TB-2_work=GREEN
              audit assertion id=23 (accepted_work_predicate_results_true): Pass
              audit assertion id=24 (proposal_telemetry_chain): Pass
              audit assertion id=43 (boltzmann_parent_selection_diversity): Pass
                — Shannon entropy = 0.811 bits over distribution {None: 1, "iter-0": 3}
replay byte-identical
tamper 3/3 detected
```

stderr trace:
```
boltzmann seed iter=0 ... parent_tx=None,         v2_pick=None
boltzmann seed iter=1 ... parent_tx=Some(iter-0), v2_pick=Some(iter-0)
boltzmann seed iter=2 ... parent_tx=Some(iter-0), v2_pick=Some(iter-0)
boltzmann seed iter=3 ... parent_tx=Some(iter-0), v2_pick=Some(iter-0)
```

## Audit questions (Q1..Q12)

**Q1 (STEP_B deviation)**: Is the no-sequencer-touch implementation strategy load-bearing-equivalent to the charter's "verify boltzmann in admission path"? Or does the SG-16.x.2.4 spirit ("≥3 WorkTx with diverse parent_selection_entropy") require sequencer-side enforcement (e.g., a sequencer admission gate that REJECTS WorkTx whose ProposalTelemetry.parent_tx doesn't match the v2 pick)?

**Q2 (entropy degeneracy)**: id=43 evaluates entropy over `parent_tx` distribution across same-task WorkTxs. The smoke produced distribution {None: 1, "iter-0": 3}. The "diversity" comes purely from None on iter 0 vs Some on iter 1+; iter 1, 2, 3 all picked the same parent. Is this a meaningful exercise of mechanism 5 (V3L-14 anti-star-topology)? Or is the smoke a hollow drill that satisfies the gate text without satisfying its intent?

**Q3 (selector determinism)**: The seed RNG `0xB01_72A_4_u64` is hardcoded in evaluator.rs (line ~1311). Is the seeded determinism audit-defensible (replay byte-identical depends on it), or should it be env-var derived for randomized fuzz?

**Q4 (PRice index population at iter 0)**: At iter 0, `snap.price_index` is empty (no NodePosition has been admitted yet — happens at sequencer admission of WorkTx via TB-12 Atom 2 hook). So v2_pick is always None on iter 0. Is the implementation aware? The fallback to `produced_worktx_ids.last()` is None at iter 0 too → parent_tx = None. Is this the correct semantic (root proposal) or a bug masked by SG passing on entropy from None?

**Q5 (proposal_index uniqueness)**: build_for_evaluator_append_with_parent uses `proposal_index = 5 + iter_i`. The .2.5 hook (already on main) uses `proposal_index = 4`. The OMEGA evaluator hot path uses `proposal_count` (variable, runtime-derived). Is there a collision risk between the proposal_index values? Are there downstream readers that key on (run_id, agent_id, proposal_index) tuple uniqueness?

**Q6 (4 WorkTxs vs SG charter ≥3)**: Smoke uses count=4. Charter SG says ≥3 with entropy ≥ 0.5. Why count=4 not count=3? Defensible (headroom) or hidden agenda?

**Q7 (Charter SG threshold ≥ 0.5 vs assertion id=43 threshold 0.25)**: Charter "≥ 0.5" not enforced; id=43 uses 0.25 (the parenthetical "Art II.2.1 alarm threshold"). Is this a silent threshold relaxation that warrants ratification?

**Q8 (test coverage)**: Are there NEW unit tests for the new evaluator.rs hook OR the audit assertion id=43? `cargo test --workspace` is at 915/0/150 — same as .2.5 baseline. Does the existing test infrastructure cover the .2.4 surface, or is this a "smoke test only, no unit test" ship?

**Q9 (CAS bloat)**: Each iteration writes 2 CAS objects (proposal_artifact + ProposalTelemetry). With count=4, that's 8 new CAS objects per smoke run. Acceptable or concerning for production-scale arena runs?

**Q10 (audit assertion id=43 Shannon entropy formula)**: src/runtime/audit_assertions.rs assertion id=43 computes `-sum(p * log2(p))`. Is the formula correct for parent_tx distribution? Should it weight by stake_amount instead of count? Should ROOT (None) be a separate category from Some(tx_id) or merged?

**Q11 (FC-trace claims)**: commit body claims FC1-N36 + FC2-N31 + FC2-N29. Are those flowchart nodes the right ones? FC2-N29 is the boltzmann witness in tests/fc_alignment_conformance.rs:450 — does the .2.4 wire-up actually exercise it at runtime, or just structurally?

**Q12 (regression risk)**: Does the new env-var hook risk altering the BEHAVIOR of existing chains (e.g., by changing balance_t for Agent_user_0 prior to LLM swarm start)? The smoke uses 4×25k = 100k μC stake; Agent_user_0 has 10M μC preseed; consumed = 1% — but does any downstream evaluator path (LLM swarm prompt, Boltzmann selector tick, dashboard render) depend on the EXACT balance_t value of preseeded agents?

Report verdict (per Q1..Q12): VETO / CHALLENGE / PASS for each question + OVERALL recommendation. Cite file:line for every finding. Conclude with VERDICT and conviction (low/medium/high).

---

"""


def append_file(rel: str, lang: str = "rust"):
    fp = ROOT / rel
    if not fp.exists():
        return f"\n## {rel}\n\n(MISSING — file not found at expected path)\n"
    return f"\n## {rel}\n\n```{lang}\n{fp.read_text()}\n```\n"


brief += "# Reference: TB-16.x.2 charter + .2.4 smoke evidence + relevant source\n"
brief += append_file("handover/tracer_bullets/TB-16.x.2_charter_2026-05-04.md", "markdown")
brief += append_file("handover/evidence/tb_16_x_2_4_smoke_2026-05-05/P12_boltzmann_runtime/README.md", "markdown")
brief += append_file("handover/evidence/tb_16_x_2_4_smoke_2026-05-05/P12_boltzmann_runtime/verdict.json", "json")
brief += append_file("handover/evidence/tb_16_x_2_4_smoke_2026-05-05/P12_boltzmann_runtime/boltzmann_trace.txt", "text")
brief += append_file("handover/tests/scripts/run_tb_16_x_2_4_smoke_2026-05-05.sh", "bash")
brief += append_file("src/sdk/actor.rs", "rust")

# evaluator.rs is huge — only include the .2.4 hook section + nearby context
import subprocess
diff = subprocess.run(
    ["git", "show", "b5118fd17b0f8666a25453239104e54406e9f80b", "--",
     "experiments/minif2f_v4/src/bin/evaluator.rs"],
    cwd=str(ROOT), capture_output=True, text=True,
)
brief += "\n## experiments/minif2f_v4/src/bin/evaluator.rs (commit b5118fd diff)\n\n```diff\n"
brief += diff.stdout
brief += "\n```\n"

# Include the new audit assertion id=43 (added in .2.5 commit f1216f0)
diff_assert = subprocess.run(
    ["git", "show", "f1216f0af52e4bd4f4048fff361e9577692cf1f8", "--",
     "src/runtime/audit_assertions.rs"],
    cwd=str(ROOT), capture_output=True, text=True,
)
brief += "\n## src/runtime/audit_assertions.rs (id=43 added in .2.5 commit f1216f0)\n\n```diff\n"
brief += diff_assert.stdout
brief += "\n```\n"

brief += "\n## genesis_payload.toml (.2.4 + .2.5 + .2.3 R-014 chain)\n\n"
genesis = (ROOT / "genesis_payload.toml").read_text().splitlines()
# Find evaluator + adapter lines
for i, line in enumerate(genesis):
    if "evaluator.rs\"" in line or "adapter.rs\"" in line:
        brief += f"L{i+1}: ```\n{line[:1500]}{'...' if len(line) > 1500 else ''}\n```\n\n"

brief += "\n---\n\nGive your INDEPENDENT TB-16.x.2.4 audit. Cite file:line for every finding. Conclude with VERDICT.\n"

print(f"[gemini tb-16.x.2.4] prompt size: {len(brief):,} chars", file=sys.stderr)

url = (
    "https://generativelanguage.googleapis.com/v1beta/models/"
    f"gemini-2.5-pro:generateContent?key={env['GEMINI_API_KEY']}"
)
body = json.dumps({
    "contents": [{"parts": [{"text": brief}]}],
    "generationConfig": {"temperature": 0.2, "maxOutputTokens": 16384},
}).encode()

t0 = time.time()
req = urllib.request.Request(
    url, data=body, headers={"Content-Type": "application/json"}, method="POST"
)
try:
    with urllib.request.urlopen(req, timeout=900) as resp:
        data = json.loads(resp.read())
except Exception as e:
    print(f"[gemini tb-16.x.2.4] API error: {e}", file=sys.stderr)
    sys.exit(1)
elapsed = time.time() - t0
print(f"[gemini tb-16.x.2.4] API returned in {elapsed:.1f}s", file=sys.stderr)

try:
    text = data["candidates"][0]["content"]["parts"][0]["text"]
except (KeyError, IndexError) as e:
    print(f"[gemini tb-16.x.2.4] response shape error: {e}\nfull response: {json.dumps(data, indent=2)[:2000]}", file=sys.stderr)
    sys.exit(1)

header = (
    f"# Gemini TB-16.x.2.4 Audit — Multi-WorkTx + Boltzmann RUNTIME (Class 3 dual external audit)\n"
    f"**Round**: {ROUND}\n"
    f"**Date**: 2026-05-05\n"
    f"**Audit target**: commit b5118fd17b0f8666a25453239104e54406e9f80b (TB-16.x.2.4 pre-audit)\n"
    f"**Test baseline**: cargo test --workspace = 915 PASS / 0 FAILED / 150 ignored\n"
    f"**Trust Root**: GREEN\n"
    f"**Audit envelope**: Class 3 (per umbrella charter §0; high-impact V3L-14 anti-collapse mechanism)\n"
    f"**Elapsed**: {elapsed:.1f}s\n"
    f"**Prompt size**: {len(brief):,} chars\n"
    f"**Audit mode**: architectural strategic (Codex covers impl-paranoid in parallel)\n\n---\n\n"
)
OUT.write_text(header + text)
print(f"[gemini tb-16.x.2.4] saved: {OUT}")
