#!/usr/bin/env python3
"""TB-16 Controlled Market Smoke Arena — Gemini Class 3 ship audit.

Per memory feedback_dual_audit + feedback_dual_audit_conflict: VETO > CHALLENGE > PASS.
Architectural strategic angle. Independent of Codex (impl-paranoid).
TB-16 shipped under Class 3 integration smoke envelope (architect §7.7);
external audit MANDATORY at ship.
"""
import json
import os
import sys
import time
import urllib.request
from pathlib import Path

ROOT = Path("/home/zephryj/projects/turingosv4")
ENV_FILES = [ROOT / ".env"]
ROUND = os.environ.get("TB16_AUDIT_ROUND", "R1")
OUT = ROOT / f"handover/audits/GEMINI_TB_16_SHIP_AUDIT_2026-05-04_{ROUND}.md"

if OUT.exists():
    print(f"[gemini tb-16] error: {OUT} already exists; refusing to overwrite", file=sys.stderr)
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
    print("[gemini tb-16] GEMINI_API_KEY not in .env", file=sys.stderr)
    sys.exit(2)

# ── Brief ──
brief = """# Gemini TB-16 Ship Audit — Controlled Market Smoke Arena (Class 3, architectural strategic review)

**Role**: skeptical adversarial reviewer. Independent of Codex (impl-paranoid angle). Conservative merge rule (memory `feedback_dual_audit_conflict`): VETO > CHALLENGE > PASS.

**Mandate**: TB-16 shipped under **Class 3 integration smoke envelope** per architect §7.7 ("AI coder may implement autonomously, but ship requires external audit"). **You are the architectural strategic reviewer**; Codex covers implementation paranoia in parallel. **Round cap = 2** per feedback_elon_mode_policy. **ROI flip stop** per feedback_audit_loop_roi_flip if R2 challenges shift to test-scaffold edges.

## Audit target — architect §7 spec verbatim (THIS IS THE GROUND TRUTH)

```text
7. TB-16 — Controlled Market Smoke Arena

7.1 目标
在受控沙盒中跑通: compute + position + complete set + price + mask + autopsy.
仍不开放真实市场。

7.2 Scenario
Lean task; multiple Agents; WorkTx FirstLong; ChallengeTx Short;
CompleteSet share inventory; PriceIndex updates; Boltzmann scheduler
selects next candidate; some agents lose positions; Autopsy generated.

7.3 Functional requirements
FR-16.1 At least 3 agents participate.
FR-16.2 At least one WorkTx creates FirstLongPosition.
FR-16.3 At least one ChallengeTx creates ShortPosition.
FR-16.4 At least one CompleteSetMintTx exists.
FR-16.5 At least one price update occurs.
FR-16.6 At least one Boltzmann mask event occurs.
FR-16.7 At least one AutopsyCapsule is generated.

7.4 Constitutional requirements
CR-16.1 Total Coin conserved.
CR-16.2 No ghost liquidity.
CR-16.3 No price overriding predicates.
CR-16.4 No raw failure broadcast.
CR-16.5 No real user funds.
CR-16.6 All activity replayable from ChainTape + CAS.
CR-16.7 All market activity is sandbox-labeled.

7.5 Ship gates
SG-16.1 Controlled market smoke produces replayable ChainTape.
SG-16.2 Dashboard shows positions, prices, masks, autopsies.
SG-16.3 No fake accepted nodes.
SG-16.4 Unsolved tasks show failure evidence / bankruptcy anchors.
SG-16.5 All market balances conserved.
SG-16.6 No unresolved evidence gaps.
SG-16.7 At least one loss -> autopsy path.
SG-16.8 Sandbox flag prevents real-money interpretation.

7.6 Forbidden
No public chain. No real-money market. No external domain.
No unbounded leverage. No AMM trading unless explicitly scoped.
No DPMM / pro-rata. No medical/legal/financial domains.
No production user funds.

7.7 Loop-mode instruction
Risk class: Class 3 integration smoke.
AI coder may implement autonomously, but ship requires external audit.
Halt if: any conservation failure; raw log leak; price-as-truth
behavior; non-sandbox funds used; unresolved evidence gap.
```

## Atom 6 v0 SCOPE NOTE (you are auditing this honestly)

The Atom 6 evidence shipped uses the TB-13 real-LLM smoke fixture as the chain-backed substrate for the audit-pipeline smoke. **A fresh 6-task arena run that exercises FR-16.2..7 on a single chain is deferred to Atom 6.1** (multi-task chain-continuation evaluator extension) + mathlib build (user-side `lake exe cache get`). The infrastructure (audit_assertions, audit_tape, audit_tape_tamper, comprehensive_arena scaffold, dashboard live regen + §16 banner) is ship-grade; the **fresh arena execution** is gated.

## Architect-mandated audit questions (CR-16.x conformance + cross-cutting)

These are YOUR primary mandate. Cite file:line for every finding.

1. **CR-16.1 + CR-16.5 (conservation + sandbox-only funds)**: TB-16 reuses `runtime::bootstrap::default_pput_preseed_pairs()` (30M μC genesis on_init mint). Verify the design admits NO post-init mint path (assert_no_post_init_mint enforced via Layer D #17 in audit_assertions). Are there any TB-16-specific code paths (sandbox arena bootstrap, comprehensive_arena helper, run_real_llm_arena.sh seeding) that could mint Coin outside on_init? Walk the bootstrap → sequencer pipeline.

2. **CR-16.4 (no raw failure broadcast) — privacy contract**: Audit `assert_28_projection_no_autopsy_bytes` (Layer F #28). The check canonical-encodes `q.tape_view_t` (AgentVisibleProjection) and scans for 32-byte runs of any private_detail_cid. CHALLENGE: serde_json serializes [u8;32] as JSON array of decimals — is the byte-run check meaningful? Does the canonical encoding flow match what Agents actually receive in their prompt path?

3. **CR-16.6 (replayability) — audit-from-tape contract**: The audit_tape binary is the system-level acceptance gate. It MUST NOT consult live Sequencer state. Verify input set: runtime_repo + cas_dir + agent_pubkeys + pinned_pubkeys + genesis + constitution + markov_pointer + alignment_dir. Are any of these inputs actually live-process artifacts in disguise (e.g. cached state.db)?

4. **CR-16.7 (sandbox banner) — SG-16.8 enforcement**: render_section_16 in audit_dashboard.rs renders SANDBOX banner when `report.sandbox_run` is true. detect_sandbox_run scans agent_pubkeys + L4 walk for sandbox prefixes. STRATEGIC QUESTION: should a chain with MIXED sandbox + non-sandbox IDs trip a HALT (per architect §7.7 "non-sandbox funds used"), or just render the banner? Currently it renders the banner if ANY sandbox match — but what if a real wallet leaks in?

5. **FR-16.2..7 spec compliance**: Architect §7 mandates the arena exercise WorkTx FirstLong + ChallengeTx Short + CompleteSetMintTx + price update + Boltzmann mask + Autopsy. Atom 6 v0 ships infrastructure but DEFERS fresh arena execution to Atom 6.1. STRATEGIC POSITION: is "infrastructure ready, fresh arena gated" acceptable for ship per architect §7.7, or does §7.7 require an actual 13-tx-kind chain before architect signoff? Take an explicit position.

6. **Class 3 envelope discipline**: TB-16 declared Class 3 from the charter. Per memory `feedback_risk_class_audit`, Class 3 = production wire-up (sequencer dispatch + git2 chain + LLM solver attestation). Did TB-16 actually MODIFY production sequencer dispatch arms? OR did it only add audit-side READ paths (audit_assertions, audit_tape) + ADDITIVE dashboard sections? If it's all read-side additive, should the envelope have been Class 2 self-audit (with retro-Class-3 promotion only if fresh arena execution at Atom 6.1 modifies dispatch)?

7. **Tamper detection (Layer H)**: audit_tape_tamper produces 3 corruption modes. flip_l4_byte picks the FIRST non-empty file in .git/objects/ — this could be a tree object, not a commit blob. Does verify_chaintape's pipeline catch tree corruption? STRATEGIC QUESTION: is the 3-mode set (flip L4 / flip CAS / truncate L4 ref) sufficient coverage of the tamper space? What about: replay against pinned_pubkeys.json swap, agent_pubkeys.json swap, L4.E injection (insert a row that would break the chain), CAS Cid collision attack?

8. **Markov chain continuity break**: TB-16 audit_pipeline_smoke emitted `MARKOV_TB-16_2026-05-03.json` with previous_capsule_cid=null (genesis). But TB-15 already shipped a Markov capsule. Why didn't TB-16 chain to TB-15? Is this an intentional choice (different evidence dirs = different chains, separate Markov heads) or a CR-15.5 violation (capsules must be evidence compression, not isolated islands)?

## Architectural strategic questions (cross-cutting)

9. **Atom 6.1 charter integrity**: Charter §3 Atom 6 said the ship gate is "all 13 tx_kinds present". Atom 6 ships with Task-A-only fixture coverage + an "Atom 6.1 follow-up" deferral. Did the ship status §4 honestly characterize this as a deficit, or did it bury the gap? Per memory `feedback_no_fake_menus`, was the original charter wording aligned with what was actually delivered? Or did the charter promise more than Atom 6 delivers?

10. **38-assertion battery completeness**: audit_assertions ships 38 named pure-fns + 1 supplemental (no_llm_self_narrative_in_autopsy = #39). STRATEGIC QUESTION: are there architect-mandated invariants that DON'T have a corresponding assertion? Walk the architect §7.4 CR-16.x list — for each, is there a 1-to-1 assertion in the battery?

11. **R-022 backlinks**: All 53 pub symbols carry `/// TRACE_MATRIX FC1-N34 + FC2-N31`. STRATEGIC QUESTION: this is doc-comment compliance, but is the FC trace actually correct? Tamper assertions (#36-#38) belong to FC1-N35 (audit_tape_tamper binary), not FC1-N34. Should the per-symbol annotations be more granular?

12. **Test count drift**: ship status §3 says "905 passed / 0 failed / 150 ignored" with "+25 over TB-15 baseline 759". 759 + 25 = 784, not 905. The discrepancy comes from sub-package tests (minif2f_v4 + gix_capability_spike) included in 905 but not in 759. STRATEGIC QUESTION: is the ship-status §3 wording misleading, or is it accepted convention? Per memory `feedback_workspace_test_canonical`, the canonical metric is `cargo test --workspace`. Should ship-status §3 cite the package-level breakdown instead of a misleading "+25 net"?

## Verdict format

End your audit with one of:

```text
## VERDICT: PASS
(All Q1-Q12 cleared; Class 3 envelope held; ship is clean.)
```

```text
## VERDICT: CHALLENGE
- Q<id> CHALLENGE: <one-line reason + line refs>
(round-2 will trigger feedback_elon_mode_policy auto-execute on
determinate-best surgical patch.)
```

```text
## VERDICT: VETO
- Q<id> VETO: <one-line BLOCKING reason + line refs>
(VETO blocks ship per feedback_dual_audit_conflict; VETO > CHALLENGE > PASS.)
```

Include conviction (low/medium/high) + recommendation (PROCEED to SHIP /
FIX-THEN-PROCEED / REDESIGN / RETRO-CLASS-2-DOWNGRADE).

---

"""


def append_file(rel: str, lang: str = "rust"):
    fp = ROOT / rel
    if not fp.exists():
        return f"\n## {rel}\n\n(MISSING — file not found at expected path)\n"
    return f"\n## {rel}\n\n```{lang}\n{fp.read_text()}\n```\n"


# ── Reference docs ──
brief += "# Reference: Charter + Architect spec + Design + Ship status + Audit pipeline evidence\n"
brief += append_file("handover/tracer_bullets/TB-16_charter_2026-05-04.md", "markdown")
brief += append_file("handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md", "markdown")
brief += append_file("handover/tests/REAL_LLM_COMPREHENSIVE_AUDIT_FROM_TAPE_DESIGN_2026-05-04.md", "markdown")
brief += append_file("handover/ai-direct/TB-16_SHIP_STATUS_2026-05-04.md", "markdown")
brief += append_file("handover/evidence/tb_16_real_llm_arena_2026-05-04/README.md", "markdown")
brief += append_file("handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/verdict.json", "json")
brief += append_file("handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper_report.json", "json")

# ── TB-16 source files under audit ──
brief += "\n\n---\n\n# TB-16 source code under audit\n"
for rel in [
    "src/runtime/audit_assertions.rs",
    "src/bin/audit_tape.rs",
    "src/bin/audit_tape_tamper.rs",
    "src/bin/audit_dashboard.rs",
    "experiments/minif2f_v4/src/bin/comprehensive_arena.rs",
    "tests/tb_16_halt_triggers.rs",
    "tests/tb_16_audit_tape_binary.rs",
    "tests/tb_16_dashboard_live_regen.rs",
    "experiments/minif2f_v4/tests/tb_16_comprehensive_arena_smoke.rs",
    "handover/tests/scripts/run_real_llm_arena.sh",
    "handover/tests/scripts/audit_tape_smoke_test.sh",
]:
    if rel.endswith(".rs"):
        lang = "rust"
    elif rel.endswith(".sh"):
        lang = "bash"
    elif rel.endswith(".md"):
        lang = "markdown"
    else:
        lang = "text"
    brief += append_file(rel, lang)

brief += "\n---\n\nGive your INDEPENDENT TB-16 ship audit. Be paranoid about Q1-Q12 (architect's mandated CR conformance + privacy contract + audit-from-tape contract + Class 3 envelope discipline). Cite file:line for every finding. Conclude with VERDICT.\n"

print(f"[gemini tb-16] prompt size: {len(brief):,} chars", file=sys.stderr)

# ── Call ──
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
    print(f"[gemini tb-16] API error: {e}", file=sys.stderr)
    sys.exit(1)
elapsed = time.time() - t0
print(f"[gemini tb-16] API returned in {elapsed:.1f}s", file=sys.stderr)

# Extract text
try:
    text = data["candidates"][0]["content"]["parts"][0]["text"]
except (KeyError, IndexError) as e:
    print(f"[gemini tb-16] response shape error: {e}\nfull response: {json.dumps(data, indent=2)[:2000]}", file=sys.stderr)
    sys.exit(1)

header = (
    f"# Gemini TB-16 Ship Audit — Controlled Market Smoke Arena (Class 3 dual external audit)\n"
    f"**Round**: {ROUND}\n"
    f"**Date**: 2026-05-04\n"
    f"**Test baseline**: cargo test --workspace = 905 PASS / 0 FAILED / 150 ignored (TB-16 Atom 6 ship commit 3300fe2)\n"
    f"**Halt-trigger battery**: 13/13 GREEN (tests/tb_16_halt_triggers.rs)\n"
    f"**Trust Root**: GREEN (2 rehashes propagated correctly)\n"
    f"**Audit envelope**: Class 3 integration smoke (architect §7.7 — external audit MANDATORY at ship)\n"
    f"**Atom 6 v0 scope note**: infrastructure shipped + audit-pipeline smoke verified end-to-end on chain-backed real-LLM tape (TB-13 fixture); fresh 6-task arena execution gated on Atom 6.1 (multi-task chain continuation) + mathlib build\n"
    f"**Elapsed**: {elapsed:.1f}s\n"
    f"**Prompt size**: {len(brief):,} chars\n"
    f"**Audit mode**: architectural strategic (Codex covers impl-paranoid in parallel)\n\n---\n\n"
)
OUT.write_text(header + text)
print(f"[gemini tb-16] saved: {OUT}")
