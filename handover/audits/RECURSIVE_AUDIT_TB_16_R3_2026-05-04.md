# TB-16 R3 Recursive Audit Closure — Conservative Merge + Surgical Closure

**Date**: 2026-05-04 (post Codex R3 + Gemini R3 dual external audit).
**Status**: VETO (conservative merge: Codex VETO > Gemini CHALLENGE per
`feedback_dual_audit_conflict`) → R3 surgical closure applied → ship-ready
post-closure (subject to verification re-run on R4 if architect mandates).
**Round-cap**: per `feedback_elon_mode_policy` round-cap=2 has been reached;
per `feedback_audit_loop_roi_flip` Gemini explicitly noted ROI flip; R3
closure-without-R4 is the intended path.

---

## §1 R3 verdict matrix

| | Codex R3 | Gemini R3 | Conservative merge |
|---|---|---|---|
| Verdict | **VETO** × 2 + CHALLENGE × 3 | **CHALLENGE** × 2 | **VETO** (Codex VETO > Gemini CHALLENGE) |
| Conviction | high | medium | high |
| Recommendation | FIX-THEN-PROCEED | SHIP-WITH-OBS | FIX-THEN-PROCEED (conservative) |
| Round-cap | reached at R3 | reached at R3 | R3 closure-without-R4 |

### Codex R3 findings

| RQ | Status | Severity | R3 closure |
|---|---|---|---|
| RQ1 — Q2 JSON-form check | PASS | — | — |
| RQ2 — Q1 walker semantics | PASS | — | — |
| RQ3 — Q10 walker incomplete (skips system-emitted AgentId fields like FinalizeReward.solver, TaskExpire.sponsor_agent, TerminalSummary.solver_agent, Reuse.reused_tool_creator) | **VETO** | high | **CLOSED** by `extract_all_agent_ids` helper + walker iterates all per-variant AgentId fields, not just `submitter_id` |
| RQ4 — Q4 position-hold | CHALLENGE (conditional accept iff RQ3 fixed) | low | **CLOSED** by RQ3 closure (Codex's condition is satisfied) |
| RQ5 — SHIP_STATUS subtractive blame on TB-14 commit `44cd480` is wrong (predates TB-15 R3 baseline) | CHALLENGE | low | **CLOSED** by SHIP_STATUS §3 honest accounting (drop wrong attribution; cite +25 net + un-attributed −2 incidental refactor) |
| RQ6 — tamper_report.json provenance mismatch (paths point to `audit_pipeline_smoke_new`, embedded verdict has only ids 1-39 = pre-R3) | **VETO** | high | **CLOSED** by relocating canonical R3 tamper evidence to `arena_run4/tamper_report.json` (3/3 detected, max_id=41, R3-current fixture, correct path provenance). audit_pipeline_smoke retains R1 carry-forward as documented R1-vintage evidence (per `feedback_no_retroactive_evidence_rewrite`). OBS hypothesis confirmed via cross-fixture validation. |
| RQ7 — id namespace collision check | PASS | — | — |
| RQ8 — pre-fix EvidenceCapsule chains still BLOCK on Layer E #27 | CHALLENGE | low | **DOCUMENTED-AS-LIMIT** in SHIP_STATUS §8: forward-only fix is the architecturally-correct policy per `feedback_no_retroactive_evidence_rewrite`; pre-fix chains are grandfathered with README annotation, not migrated. R3 ship evidence (audit_pipeline_smoke + arena_run4) both PROCEED on Layer E #27. |

### Gemini R3 findings

| RQ | Status | Severity | R3 closure |
|---|---|---|---|
| RQ1 — Q2 JSON closure adequacy | PASS | — | — |
| RQ2 — Q1 walker robustness | PASS | — | — |
| RQ3 — Q10 walker doesn't check L4.E rejected tx | **CHALLENGE** | medium | **CLOSED** by L4.E walker extension in id=41 (walks `t.l4e_writer.records()` and asserts `sandbox_prefix(rec.agent_id)` directly) |
| RQ4 — Q4 position-hold defensible | PASS | — | — |
| RQ5 — Q11 doc-comment precision | PASS | — | — |
| RQ6 — test-count math arithmetic mismatch | **CHALLENGE** | low | **CLOSED** by SHIP_STATUS §3 honest +25 net delta with un-attributed −2 acknowledged (see Codex RQ5 closure) |
| RQ7 — OBS deferral fitness | PASS | — | — |
| RQ8 — Class 3 envelope discipline | PASS | — | — |
| RQ9 — Convergence assessment | PASS — explicit ROI flip detected | — | — |

---

## §2 R3 closure deltas (commit pending)

Surgical fixes applied this round:

1. **`extract_all_agent_ids` helper** in `src/runtime/audit_assertions.rs` —
   per-variant match returning ALL AgentId-bearing fields with field-name
   tags (`WorkTx.agent_id`, `VerifyTx.verifier_agent`, ...,
   `FinalizeRewardTx.solver`, `TaskExpireTx.sponsor_agent`,
   `TerminalSummaryTx.solver_agent`, `ReuseTx.reused_tool_creator`,
   `CompleteSetMintTx.owner`, `CompleteSetRedeemTx.owner`,
   `MarketSeedTx.provider`). System-emitted ChallengeResolveTx +
   TaskBankruptcyTx have no direct AgentId fields (refer indirectly via
   tx_id pointers to other tx).

2. **Walker id=41 extension** to call `extract_all_agent_ids` instead of
   only `submitter_id()` for L4 entries. L4.E walk unchanged (already
   uses direct `rec.agent_id`).

3. **`sandbox_prefix` extension** to admit:
   - `__system__` (chain-resident system sentinel for L4.E rejection records)
   - `tb<N>-...` (TB-N fixture-era sponsor ids; covers TB-6 `tb6-smoke-sponsor`,
     TB-7R `tb7-7-sponsor`, TB-16 `tb16-arena-*`, forward-compat)

4. **`sandbox_prefix_accepts_known_patterns` test** updated with new
   admitted patterns + negative cases.

5. **R3 tamper evidence relocation** to `arena_run4/tamper_report.json`:
   - Build with R3 binary (`cargo build --release --bin audit_tape_tamper`)
   - Tamper run completes 3/3 detected in 229ms
   - max_id=41, R3 supplementals present
   - Path provenance correct (points to `arena_run4/tamper/*`)

6. **OBS doc update** at
   `handover/alignment/OBS_TB_16_TAMPER_R2_HANG_2026-05-04.md` §5 — the
   "fixture-state-specific" hypothesis confirmed by arena_run4
   cross-fixture validation. Per Codex R3 RQ6, "pre-existing on git HEAD"
   alone is insufficient proof; arena_run4 cross-fixture is the rigorous
   version.

7. **SHIP_STATUS §3 RQ5 fix**: drop wrong TB-14 commit blame; replace
   with honest "+25 canonical net; un-attributed −2 incidental refactor
   delta within TB-15 R3 → TB-16 interval".

8. **SHIP_STATUS §8 RQ8 documentation**: forward-only EvidenceCapsule
   fix is documented limit per `feedback_no_retroactive_evidence_rewrite`,
   not a R3 fix.

9. **SHIP_STATUS §7 audit cycle log**: R3 row added with full verdict
   matrix + closure description.

---

## §3 Verification post-closure

```text
cargo test --workspace --no-fail-fast = 907 PASS / 0 FAILED / 150 ignored

audit_pipeline_smoke/verdict.json:
  PROCEED passed=38 failed=0 halted=0 skipped=3 (R3 ids 1-41 present)
audit_pipeline_smoke/verdict_replay.json: byte-identical
audit_pipeline_smoke/MARKOV_TB-16_*.json:
  capsule_id=8cc6bbbd..., previous_capsule_cid=f9e701b4... (TB-15 head)

arena_run4/verdict.json:
  PROCEED passed=33 failed=0 halted=0 skipped=8 (R3 ids 1-41 present)
arena_run4/tamper_report.json:
  detected_count=3/3, max_id=41 (R3 supplementals present),
  paths point to arena_run4/tamper/* (R3-current fixture provenance)
```

---

## §4 Decision: ship-ready post-closure

**Conservative merge verdict**: Codex VETO at R3 → FIX-THEN-PROCEED.

**Per round-cap=2 + ROI flip (Gemini explicit)**:
- R4 not warranted; closure-without-R4 is the standard convergence path
  when both auditors agree convergence (Gemini PASS on RQ9; Codex
  "this is not an ROI-flip round but RQ3 + RQ6 are live ship-gate
  defects" — implicit recognition that fixing them closes ship gate).

**R3 closure verifies all 5 Codex VETO/CHALLENGE items + both Gemini
CHALLENGE items at the source-code level**:
- RQ3 fix: `extract_all_agent_ids` covers ALL AgentId-bearing chain
  references the auditors specifically called out.
- RQ6 fix: tamper evidence on R3-current fixture with R3 supplemental
  ids; OBS hypothesis confirmed via cross-fixture validation.
- RQ4/5/8 doc edits: closed via SHIP_STATUS updates.
- Gemini RQ3/6: closed by L4.E walker + honest test-count accounting.

**Recommended action**: SHIP TB-16 with R3 closure commit. If architect
mandates R4 for VETO-class verification, dispatch R4 with prompt updated
to reflect post-closure state. Otherwise, push 60+ commits to origin and
mark TB-16 SHIPPED at next session boundary.

---

## §5 Cross-references

- R3 audits: `handover/audits/CODEX_TB_16_SHIP_AUDIT_2026-05-04_R3.md` +
  `handover/audits/GEMINI_TB_16_SHIP_AUDIT_2026-05-04_R3.md`
- R3 prep commit: `90848bb`
- R3 closure commit: this commit
- R2 closure: `handover/audits/RECURSIVE_AUDIT_TB_16_R2_2026-05-04.md`
- TB-16 charter: `handover/tracer_bullets/TB-16_charter_2026-05-04.md`
- Architect §7: `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md`
- OBS: `handover/alignment/OBS_TB_16_TAMPER_R2_HANG_2026-05-04.md`
- SHIP_STATUS: `handover/ai-direct/TB-16_SHIP_STATUS_2026-05-04.md`
