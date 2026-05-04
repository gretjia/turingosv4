#!/usr/bin/env python3
"""TB-15 Lamarckian Autopsy + Markov EvidenceCapsule — Gemini Class 2 ship audit.

Per memory feedback_dual_audit + feedback_dual_audit_conflict: VETO > CHALLENGE > PASS.
Architectural strategic angle. Independent of Codex (impl-paranoid).
TB-15 shipped under Class 2 self-audit envelope (charter §4); user retroactively
requested dual audit on 2026-05-04 to verify the Class 2 envelope held + the
new sequencer hook is sound.
"""
import json
import os
import sys
import time
import urllib.request
from pathlib import Path

ROOT = Path("/home/zephryj/projects/turingosv4")
ENV_FILES = [ROOT / ".env"]
ROUND = os.environ.get("TB15_AUDIT_ROUND", "R2")
OUT = ROOT / f"handover/audits/GEMINI_TB_15_SHIP_AUDIT_2026-05-04_{ROUND}.md"

if OUT.exists():
    print(f"[gemini tb-15] error: {OUT} already exists; refusing to overwrite", file=sys.stderr)
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
    print("[gemini tb-15] GEMINI_API_KEY not in .env", file=sys.stderr)
    sys.exit(2)

# ── Brief ──
brief = """# Gemini TB-15 R2 Ship Audit — POST-REMEDIATION

## R2 CONTEXT — R1 verdict + remediation summary (READ THIS FIRST)

**R1 dual-audit verdicts (2026-05-04 R1)**:
- Gemini R1: **VETO Q12** (replay-determinism — pre-TB-15 chain replay would generate spurious autopsies; needs activation gate) + **CHALLENGE Q7** (`flowchart_hashes` field missing from MarkovEvidenceCapsule per literal SG-15.7)
- Codex R1: CHALLENGE × 5 (Q3 --no-cas, Q4 override-not-live, Q5 byte-window weak, Q8/RQ7 flowchart_hashes [overlap with Gemini Q7], Q9 dashboard not regenerable)

**R2 remediations applied** (verify each architecturally):

1. **Q12 VETO closure**: NEW `pub const TB15_AUTOPSY_ACTIVATION_LOGICAL_T: u64 = 0;` + `pub fn is_autopsy_active_at(timestamp_logical: u64) -> bool` in `src/runtime/autopsy_capsule.rs`. Both dispatch arm (Step 3.5) and apply_one (Stage 3.5) wrapped in the gate. Default const=0 keeps fresh chains active; pre-TB-15 migration would override. Verification baseline: ZERO production chains have TaskBankruptcyTx.

2. **Q7 (flowchart_hashes) closure**: NEW field `pub flowchart_hashes: Vec<Hash>` on `MarkovEvidenceCapsule` (additive, `#[serde(default)]`). NEW `read_flowchart_hashes_from_matrix` parser for `handover/alignment/TRACE_FLOWCHART_MATRIX.md` §2 (4 canonical SHA-anchored hashes 1a/1b/2/3). Generator binary populates the field; halt-trigger #2 strengthened. R2 capsule emitted at `handover/evidence/tb_15_markov_capsule_2026-05-04/MARKOV_TB-15-R2_2026-05-03.json` with all 4 hashes.

3. **Q3 (CAS residency)** — Codex finding closure: R2 generator dropped `--no-cas`; CAS-resident in `/tmp/tb15-r2-cas/.git/objects/`.

4. **Q4 (live override gate)** — Codex finding closure: NEW `--include-prior-capsules N` arg; values >0 actively call `try_deep_history_read_with_override_check(override_set_from_env())`. Deny path returns exit code 3.

5. **Q5 (byte-window scan)** — Codex finding closure: halt-trigger #5 strengthened to scan canonical Cid array text form + raw 32-byte run + canonical_encode bytes.

6. **Q9 (dashboard not regenerable)** — Codex finding: OBS-deferred to TB-16 (`handover/alignment/OBS_TB_15_DASHBOARD_LIVE_REGEN_TB16_2026-05-04.md`). Privacy contract HOLDS structurally; live rebuild = TB-16 controlled-arena scope per ship status doc.

**R2 baseline**:
- `cargo test --workspace` = 881 PASS / 0 fail / 150 ignored (+3 new tests)
- R2 capsule_id `a94ae8848f66d1335285bd9800cf2dd161ea69fa601502cbe3a2bccc26fe3132`
- Trust Root: GREEN (sequencer.rs rehashed 9fa59362 → f4119a41)
- All 6 halt-triggers GREEN

## R2 mandate

You are the architectural strategic reviewer. Re-audit each R1 finding for closure:
- **Q12 (your VETO)**: is the activation gate STRUCTURALLY sound? `is_autopsy_active_at(timestamp_logical)` gates both sites identically. Does this prevent retroactive evidence generation per `feedback_no_retroactive_evidence_rewrite`? Are there architectural concerns with the `const = 0` default (e.g., compile-time-only override is brittle for pre-TB-15 migration scenarios)? Should it be a chain-resident marker instead?
- **Q7 (your CHALLENGE)**: are flowchart_hashes correctly wired? Do they fully discharge the literal SG-15.7 spec ("constitution hash AND flowchart hashes")? Is the parser robust against future TRACE_FLOWCHART_MATRIX.md format changes?
- **Other R1 findings**: do the other 4 Codex CHALLENGEs (Q3/Q4/Q5/Q9) also have credible architectural closure, or did Codex see something you missed in R1?
- **NEW findings welcome**: are there cross-cutting concerns introduced by the R2 changes? (e.g., the new `flowchart_hashes` field changes capsule serialization — does this break backward-compat with the R1 ship Markov capsule? The activation gate adds a runtime branch — does it have determinism implications?)

Per `feedback_audit_loop_roi_flip`: if R2 CHALLENGEs shift from production-code defects to test-scaffold edges, recommend ship-with-OBS rather than R3.

---

## Original R1 audit context (verbatim, for reference) — R2 mandate above takes precedence

# Gemini TB-15 Ship Audit — Lamarckian Autopsy + Markov EvidenceCapsule (Class 2 envelope, retroactive dual audit, architectural strategic review)

**Role**: skeptical adversarial reviewer. Independent of Codex (impl-paranoid angle). Conservative merge rule (memory `feedback_dual_audit_conflict`): VETO > CHALLENGE > PASS.

**Mandate**: TB-15 shipped under **Class 2 self-audit envelope** per charter §4 ("Class 2 = self-audit per feedback_dual_audit hybrid-by-risk-class"). User retroactively requested dual audit on 2026-05-04. **You are the architectural strategic reviewer**; Codex covers implementation paranoia in parallel. **Round cap = 2** per feedback_elon_mode_policy. **ROI flip stop** per feedback_audit_loop_roi_flip if R2 challenges shift to test-scaffold edges.

## Audit target — architect §6 spec verbatim (THIS IS THE GROUND TRUTH)

```text
4. TB-15 — Lamarckian Autopsy + Markov EvidenceCapsule

4.1 目标
把失败、爆仓、亏损、反复错误转化为私有学习与 Markov capsule，
而不是全局 raw-log 污染。

4.2 新增对象
pub struct AgentAutopsyCapsule {
    pub capsule_id: Cid,
    pub agent_id: AgentId,
    pub event_id: EventId,
    pub loss_amount: MicroCoin,
    pub loss_reason_class: LossReasonClass,
    pub violated_risk_rule: Option<RiskRuleId>,
    pub suggested_policy_patch: Option<Cid>,
    pub evidence_cids: Vec<Cid>,
    pub public_summary: String,
    pub private_detail_cid: Cid,
}
pub struct MarkovEvidenceCapsule {
    pub capsule_id: Cid,
    pub previous_capsule_cid: Option<Cid>,
    pub constitution_hash: Hash,
    pub l4_root: Hash,
    pub l4e_root: Hash,
    pub cas_root: Hash,
    pub typical_errors: Vec<TypicalErrorSummary>,
    pub unresolved_obs: Vec<ObsId>,
    pub next_session_context_cid: Cid,
}

4.3 Functional requirements
FR-15.1 Loss / bankruptcy / failed market event creates AgentAutopsyCapsule.
FR-15.2 Autopsy uses ChainTape/CAS evidence, not self-narration.
FR-15.3 MarkovEvidenceCapsule generated at end of TB/run.
FR-15.4 Next InitAI context defaults to constitution + latest capsule.
FR-15.5 Markov override is required for deep history reads.
FR-15.6 Public summary can broadcast typical error; private detail remains scoped.

4.4 Constitutional requirements
CR-15.1 Raw failure logs are not broadcast globally.
CR-15.2 Autopsy is private/scoped unless error becomes typical.
CR-15.3 ArchitectAI may propose improvements from logs, but cannot mutate constitution.
CR-15.4 JudgeAI/VetoAI remains veto-only.
CR-15.5 Capsules are evidence compression, not hidden source of truth.
CR-15.6 Markov default prevents context poisoning.

4.5 Ship gates
SG-15.1 Failed/losing agent gets private AutopsyCapsule.
SG-15.2 Raw private details do not enter other Agent read view.
SG-15.3 Latest Markov capsule can bootstrap next session.
SG-15.4 Deep-history read without override fails.
SG-15.5 Typical error broadcast uses summary, not raw log.
SG-15.6 Dashboard can regenerate capsule summary from ChainTape + CAS.
SG-15.7 Markov capsule references constitution hash and flowchart hashes.
SG-15.8 Autopsy does not mutate predicates/tools automatically.

4.6 Forbidden
No global raw autopsy broadcast.
No forced prompt stuffing of all past failures.
No automatic predicate mutation.
No MetaTape self-modification.
No constitution change.
No hidden source-of-truth capsule.
No private loss detail in public read view.

4.7 Loop-mode instruction
Class 2 unless touching Agent read-view gating deeply; Class 3 if
privacy/security gates are modified. Halt if raw logs leak to general
read view or Markov capsule becomes hidden source of truth.
```

## Architect-mandated audit questions (CR-15.x conformance + cross-cutting)

These are YOUR primary mandate. Cite file:line for every finding.

1. **CR-15.1 + CR-15.2 architectural soundness**: Is the privacy boundary structurally enforced, or only by convention? Specifically, the AutopsyIndex (`agent_autopsies_t: BTreeMap<EventId, Vec<Cid>>` on EconomicState) lives sequencer-side. The claim is "NOT projected to AgentVisibleProjection." Verify that ANY future code path adding autopsy bytes to a Projection field would have to clear a structural gate (halt-trigger #1 file-scan), AND that no current code path (sequencer / bus / dashboard / evaluator) projects the bytes. Walk the full dependency graph from `EconomicState.agent_autopsies_t` to any agent-readable surface. Is this enforcement provably tight, or is there a gap?

2. **CR-15.3 + CR-15.4 (three-power separation)**: Architect §1.2 hard prohibition C: "NO bypass of VetoAI for permission changes. Even if Autopsy recommends 'remove this Agent's market access', the actual permission change must go through the meta loop: ArchitectAI proposal → JudgeAI/VetoAI → canary." TB-15 ships `suggested_policy_patch: Option<Cid>` as an opaque pointer. Is there ANY current code path that auto-reads this field and applies it? Or is it strictly a write-only suggestion that requires a separate (not-yet-shipped) MetaTape consumer? Is the architectural separation tight, or is the field a hidden coupling waiting to become a backdoor?

3. **CR-15.5 (capsules are evidence compression, not hidden source of truth)**: Every field in MarkovEvidenceCapsule MUST be derivable from chain + CAS at generation time. Walk each field of the shipped struct (capsule_id / previous_capsule_cid / constitution_hash / l4_root / l4e_root / cas_root / typical_errors / unresolved_obs / next_session_context_cid / sha256 / created_at_logical_t / tb_tag): is each derivable from on-disk artifacts alone? Are there any "creator-only" fields that introduce a hidden source of truth?

4. **CR-15.6 (Markov default prevents context poisoning)**: The default-deny gate is binary-level (`TURINGOS_MARKOV_OVERRIDE=1` env). Architect §1.2 hard prohibition A: "NO global broadcast of raw liquidation logs." Is the binary-level fence sufficient when the broader system (future InitAI agent) has not yet been built? Or is this a structural gap between "substrate ships" (TB-15) and "agent-side honoring" (P5 v1)? Charter §1.3 explicitly defers agent-side enforcement — is this defer ratified-by-spec or unflagged-by-spec?

5. **FR-15.1 trigger-site coverage**: Architect spec says "Loss / bankruptcy / failed market event creates AgentAutopsyCapsule." TB-15 v0 wires ONE trigger: TaskBankruptcyTx. Charter §1.2 + §7-A defers SlashLoss / ChallengeUnsuccessful / VerifierBondLost to RSP-3.2 / RSP-4. Is "loss" semantically discharged by Bankruptcy alone? Or is the v0 spec compliance partial (and if partial, is it honestly labeled as partial)?

6. **FR-15.4 + FR-15.5 (Markov chain integrity)**: The MarkovEvidenceCapsule chain links via `previous_capsule_cid: Option<Cid>`. Genesis Markov capsule has None. Is the chain structurally tamper-evident? If an adversary swaps a middle capsule (replaces its bytes in CAS with crafted bytes), is the swap detectable? (Hint: capsule_id = sha256(canonical_bytes_with_capsule_id_zeroed); CAS Cid = sha256(stored_bytes); the chain's `previous_capsule_cid` pins each predecessor's identity.)

7. **SG-15.7 spec literal "constitution hash AND flowchart hashes"**: SG-15.7 reads "Markov capsule references constitution hash AND flowchart hashes." The shipped MarkovEvidenceCapsule struct has only `constitution_hash: Hash` — no `flowchart_hashes: Vec<Hash>` or similar. Is this a spec deviation, an honest deferred gap, or implicit (the canonical flowchart hashes are derivable from `previous_capsule_cid` chain alone via TRACE_FLOWCHART_MATRIX)? **CHALLENGE WELCOME**: should TB-15 have included a `flowchart_hashes: Vec<Hash>` field, or was the omission justified by the existence of `handover/alignment/TRACE_FLOWCHART_MATRIX.md` SHA-anchoring (architect 2026-05-02 ruling)?

8. **Class 2 envelope discipline**: Charter §4 declared Class 2 = self-audit OK. Promotion-to-Class-3 trigger = "modify AgentVisibleProjection field set OR add a sequencer dispatch arm beyond the single TaskBankruptcyTx hook." TB-15 added a NEW dispatch arm hook (Step 3.5 in TaskBankruptcyTx) AND a NEW apply_one stage hook (Stage 3.5). Did this exceed the Class 2 envelope? Should the envelope have been tripped to Class 3, mandating dual audit at ship time (rather than retroactively)? Take an explicit position.

## Architectural strategic questions (Class 2-vs-Class 3 envelope re-litigation)

9. **Cross-cutting impact of EconomicState 12 → 13 sub-fields**: 4 test fixtures + 1 in-module assertion required updating to track the new `agent_autopsies_t` field. Search the codebase (use the source files I'm attaching) for OTHER hard-coded "12" constants related to EconomicState. Is the bump complete, or are there silent stale assertions that would surface only on a future bump to 14?

10. **Tape-canonical (Art.0.2) extension**: AgentAutopsyCapsule + MarkovEvidenceCapsule canonical bytes ARE the CAS objects referenced by their capsule_id (the TB-11 EvidenceCapsule pattern). Is the canonical encode/decode round-trip provably stable across serde version bumps? If a future serde change reorders struct fields, would replay-determinism break?

11. **TB-16 forward compat**: TB-15 ships substrate; TB-16 (Controlled Market Smoke Arena) will exercise it via real-LLM run. Does the TB-15 schema admit any TB-16 evolution gracefully (additive `#[serde(default)]` for new capsule fields), or are there fixed-shape commitments that would force breaking changes if TB-16 needs richer evidence types?

12. **"Going-forward only" discipline (feedback_no_retroactive_evidence_rewrite)**: TB-15 explicitly defers historical-event autopsy backfilling. Is this discipline correctly scoped? Specifically: a TaskBankruptcyTx that occurred BEFORE TB-15 ship would not have triggered autopsy emission. Is that pre-TB-15 chain still REPLAYABLE post-TB-15 deployment without producing spurious autopsy entries? (Replay-determinism contract: the dispatch arm is now `Step 3.5`-augmented; would replaying a pre-TB-15 chain produce autopsies that did not exist in the original L4?)

## Verdict format

End your audit with one of:

```text
## VERDICT: PASS
(All Q1-Q12 cleared; Class 2 envelope held; ship is clean.)
```

```text
## VERDICT: CHALLENGE
- Q<id> CHALLENGE: <one-line reason + line refs>
- <repeat per challenge>
(round-2 will trigger feedback_elon_mode_policy auto-execute on
determinate-best surgical patch.)
```

```text
## VERDICT: VETO
- Q<id> VETO: <one-line BLOCKING reason + line refs>
(VETO blocks ship per feedback_dual_audit_conflict; VETO > CHALLENGE > PASS.)
```

Include conviction (low/medium/high) + recommendation (PROCEED to SHIP /
FIX-THEN-PROCEED / REDESIGN / RETRO-CLASS-3-PROMOTION).

---

"""


def append_file(rel: str, lang: str = "rust"):
    fp = ROOT / rel
    if not fp.exists():
        return f"\n## {rel}\n\n(MISSING — file not found at expected path)\n"
    return f"\n## {rel}\n\n```{lang}\n{fp.read_text()}\n```\n"


# ── Reference docs ──
brief += "# Reference: Charter + Architect spec + Decision + Ship status + First Markov capsule\n"
brief += append_file("handover/tracer_bullets/TB-15_charter_2026-05-03.md", "markdown")
brief += append_file("handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md", "markdown")
brief += append_file("handover/alignment/DECISION_LAMARCKIAN_AUTOPSY_BOLTZMANN_2026-05-02.md", "markdown")
brief += append_file("handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md", "markdown")
brief += append_file("handover/evidence/tb_15_markov_capsule_2026-05-03/README.md", "markdown")
brief += append_file("handover/evidence/tb_15_markov_capsule_2026-05-03/MARKOV_TB-15_2026-05-03.json", "json")

# ── TB-15 source files under audit ──
brief += "\n\n---\n\n# TB-15 source code under audit\n"
for rel in [
    "src/runtime/autopsy_capsule.rs",
    "src/runtime/markov_capsule.rs",
    "src/bin/generate_markov_capsule.rs",
    "src/runtime/mod.rs",
    "src/state/q_state.rs",
    "src/state/typed_tx.rs",
    "src/state/sequencer.rs",
    "src/bottom_white/cas/schema.rs",
    "src/bin/audit_dashboard.rs",
    "tests/tb_15_halt_triggers.rs",
    "tests/fc_alignment_conformance.rs",
    "tests/economic_state_reconstruct.rs",
    "tests/q_state_reconstruct.rs",
    "tests/six_axioms_alignment.rs",
]:
    lang = "markdown" if rel.endswith(".md") else "rust"
    brief += append_file(rel, lang)

brief += "\n---\n\nGive your INDEPENDENT TB-15 ship audit. Be paranoid about Q1-Q12 (the architect's mandated CR conformance + privacy contract + Markov chain integrity + Class 2 envelope discipline). Cite file:line for every finding. Conclude with VERDICT.\n"

print(f"[gemini tb-15] prompt size: {len(brief):,} chars", file=sys.stderr)

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
    print(f"[gemini tb-15] API error: {e}", file=sys.stderr)
    sys.exit(1)
elapsed = time.time() - t0
print(f"[gemini tb-15] API returned in {elapsed:.1f}s", file=sys.stderr)

# Extract text
try:
    text = data["candidates"][0]["content"]["parts"][0]["text"]
except (KeyError, IndexError) as e:
    print(f"[gemini tb-15] response shape error: {e}\nfull response: {json.dumps(data, indent=2)[:2000]}", file=sys.stderr)
    sys.exit(1)

header = (
    f"# Gemini TB-15 Ship Audit — Lamarckian Autopsy + Markov EvidenceCapsule (Class 2 retroactive dual audit)\n"
    f"**Round**: {ROUND}\n"
    f"**Date**: 2026-05-04\n"
    f"**Test baseline**: cargo test --workspace = 878 PASS / 0 FAILED / 150 ignored (TB-15 ship commit 2337381)\n"
    f"**Halt-trigger battery**: 6/6 GREEN (tests/tb_15_halt_triggers.rs)\n"
    f"**Trust Root**: GREEN (6 rehashes propagated correctly)\n"
    f"**Original audit envelope**: Class 2 self-audit per charter §4 (no Codex/Gemini at ship)\n"
    f"**Retroactive dual audit**: requested by user 2026-05-04 to verify Class 2 envelope held\n"
    f"**Elapsed**: {elapsed:.1f}s\n"
    f"**Prompt size**: {len(brief):,} chars\n"
    f"**Audit mode**: architectural strategic (Codex covers impl-paranoid in parallel)\n\n---\n\n"
)
OUT.write_text(header + text)
print(f"[gemini tb-15] saved: {OUT}")
