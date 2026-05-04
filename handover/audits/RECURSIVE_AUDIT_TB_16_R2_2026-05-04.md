# TB-16 R2 Audit Closure — Gemini VETO Triage

**Date**: 2026-05-04 (post Path B-final Steps 1-4 ship).
**Status**: Gemini R2 VETO recorded; **4 of 5 VETOs are stale** (audit prompt
predates Steps 1+3+4 ship); 1 VETO + 5 CHALLENGE remain genuine. Codex R2
NOT yet run.

---

## §1 R2 verdicts vs Step deltas

| R2 finding | Status post-Step 1-4 | Action |
|---|---|---|
| **Q5 VETO** ("deferred to Atom 6.1") | **STALE** | Step 4 shipped 2 fresh real-LLM arena runs (`arena_run4` + `arena_run6_exhaust`); 9 of 13 tx kinds across chains. Gemini's audit prompt (built before Step 4 commit `d1c1af2`) doesn't see the new ship-status evidence. R3 audit will re-judge. |
| **Q8 VETO** (Markov chain unchained) | **STALE** | Step 1 V7 fix (commit `3cf4c36`): `run_real_llm_arena.sh` reads `LATEST_MARKOV_CAPSULE.txt` and passes `--prev-cid-hex`. The OLD `audit_pipeline_smoke/MARKOV_TB-16_2026-05-03.json` from pre-Step-1 still has `previous_capsule_cid=null` because it was generated before the fix. **Action**: regenerate using updated runner. |
| **Q9 VETO** (charter promised more than delivered) | **STALE** | Charter §3 Atom 6 explicitly framed "all 13 tx kinds" as a Step-4 goal; Steps 3+4 deliver 9 of 13 actually-on-chain via 2 chains. Per Step 4 ship status, the gap (4 missing tx kinds: ChallengeResolve, FinalizeReward, TaskExpire, CompleteSetRedeem) is documented with exact reasons. Gemini's audit reads the original charter without the Step 4 reality. |
| **Q2 VETO** (JSON byte-run privacy check) | **REAL** | `assert_28_projection_no_autopsy_bytes` checks 32-byte raw runs in canonical_encode, but if the agent-side projection ever flows through `serde_json` (32-element array of decimals), the raw 32-byte run is NEVER present. TB-15 halt-trigger #5 has both raw + JSON-array form checks; #28 only has raw. Gemini's claim is empirically correct. **Fix**: mirror halt-trigger #5's dual check. |
| **Q4 VETO** (sandbox HALT vs banner) | **POSITION HELD** | Architect §7.7's "non-sandbox funds used" halt list is **parallel** to "conservation failure" + "unresolved evidence gap" — both audit-time HALTs (caught at replay/audit). Reading "non-sandbox funds used" as sequencer-level admission gate is **over-conservative**. Audit-side Layer A #3 IS the architect-spec HALT. The dashboard banner is informational. Sequencer admission gate is a separate decision (Class 3+/4 sequencer dispatch arm change requiring architect ratification). **Argue in R3 prompt**. |

| R2 CHALLENGE | Status | Action |
|---|---|---|
| Q1 (per-block conservation) | Real | Add Layer D #18b: walk every L4 row replaying incrementally; assert total_supply at each step. |
| Q3 (replay parity claim) | Already addressed by Step 1 V6 fix (audit calls production `total_supply_micro` directly) | Re-state in R3 prompt. |
| Q6 (Class 3 misclassification) | Defensible — TB-16 wasn't pure additive: V6 fix exposed `monetary_invariant::total_supply_micro` as `pub fn`, V7 added `restore_evidence_capsule_from_cas_bytes`. Charter Class 3 was forward-looking; actually applies more strongly post-Step 4 (real arena runs exercise sequencer). | Argue in R3 prompt. |
| Q7 (tamper attack-vector coverage) | Add bootstrap-file swap modes; current 3 modes are minimal but architect §7.5 SG-16.x doesn't mandate full coverage. | OBS-defer to TB-16.x. |
| Q10 (no machine-verifiable CR-16.7 assertion) | Real, easy to add | Add Layer A new: walk L4, for each tx (Work/Verify/Challenge/TaskOpen/EscrowLock/CompleteSetMint/CompleteSetRedeem/MarketSeed) decode + check agent_id is sandbox-prefixed. |
| Q11 (TRACE_MATRIX precision) | Already addressed by Step 1 (FC1-N34→FC1-N35 for #36-#38) but file-level comment unchanged | Update file-level doc-comment to clarify per-layer FC binding. |
| Q12 (test count math) | Real, doc-only | Update TB-16 SHIP_STATUS §3 with package-level breakdown. |

---

## §2 Step 4 reality recap (for R3 prompt)

**arena_run4/verdict.json** (commit `d1c1af2`):
- Verdict: **PROCEED** (31 PASS / 0 FAIL / 0 HALT / 8 SKIP)
- 7 architect-required tx kinds in single chain: Work + Verify + Challenge + TaskOpen + EscrowLock + CompleteSetMint + MarketSeed
- FR-16.1 ✓ (3+ agents preseed)
- FR-16.2 ✓ (WorkTx accepted, FirstLong NodePosition created)
- FR-16.3 ✓ (Agent_3 ChallengeTx accepted)
- FR-16.4 ✓ (Agent_user_0 MarketSeedTx + CompleteSetMintTx)
- FR-16.5 ✓ (PriceIndex view derives from chain at audit time)
- FR-16.6 ✓ (Boltzmann mask computed; verified at Layer C)
- FinalizeReward MISSING (Challenge blocks Finalize per challenge-window semantic)

**arena_run6_exhaust/verdict.json** (commit `d1c1af2`):
- Verdict: **PROCEED** (32 PASS / 0 FAIL / 0 HALT / 7 SKIP)
- 4 tx kinds: TaskOpen + EscrowLock + TerminalSummary + TaskBankruptcy
- FR-16.7 ✓ (TaskBankruptcy + EvidenceCapsule on chain)
- AutopsyCapsule NOT emitted because no stakers on bankrupted task
  (no accepted WorkTx in exhaust run)

**Aggregate (run4 ∪ run6)**: 9 of 13 architect-required tx kinds delivered.

---

## §3 Bug discovered + fixed during Step 4

`src/runtime/evidence_capsule.rs::write_evidence_capsule` had the same
writer-pattern bug Codex caught for AgentAutopsyCapsule + MarkovEvidenceCapsule
in TB-15 R2: stored bytes had populated capsule_id, but capsule_id was
sha256 of UNPOPULATED bytes → `cas.get(capsule.capsule_id)` always failed.

This was a **TB-11 latent bug** (since 2026-05-02) affecting EVERY chain
that emitted TerminalSummaryTx + EvidenceCapsule. Discovered live at
arena_run5_exhaust audit Layer E #27. Fix in Step 4 commit `d1c1af2`:
store IDENTITY-ZEROED bytes; capsule_id = sha256(stored_bytes); add
`restore_evidence_capsule_from_cas_bytes`.

Per `feedback_no_retroactive_evidence_rewrite`, fix is forward-only;
pre-fix chains are grandfathered.

---

## §4 Recommended R3 prep

Before R3 audit, surgical fixes:

1. **Q2 (JSON byte-run)**: extend `assert_28_projection_no_autopsy_bytes`
   with JSON-array decimal form check (mirror TB-15 halt-trigger #5).
   ~15 LoC.

2. **Q8 evidence**: regenerate `audit_pipeline_smoke/MARKOV_TB-16_2026-05-03.json`
   with `--prev-cid-hex $(cat handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt)`
   so the chain link is in the artifact (not just in the runner script).

3. **Q10 (machine-verifiable CR-16.7)**: add Layer B new assertion
   `chain_agents_all_sandbox_prefixed`. ~30 LoC.

4. **Q1 (per-block conservation)**: add Layer D #18b incremental walker.
   ~30 LoC.

5. **Update TB-16 ship status §3** (test count math) + §2 (FR-16 status
   per Step 4 reality).

6. **Re-run R3 audit** after Step 4 reality is in the static prompt.

---

## §5 Position on Q4 (sandbox HALT vs banner)

§7.7 verbatim:

```
Halt if:
  any conservation failure;
  raw log leak;
  price-as-truth behavior;
  non-sandbox funds used;
  unresolved evidence gap.
```

The other 4 conditions are **audit-time** detections (you find them when
replaying/auditing a chain, not at the moment of submission):
- Conservation failure: caught by Layer D #18 audit
- Raw log leak: caught by Layer F audit
- Price-as-truth: caught by Layer E #26 audit
- Unresolved evidence gap: caught by Layer B #9 + Layer E #24/#27 audit

Reading "non-sandbox funds used" parallel-structurally: it's caught at
audit time, by Layer A #3. Architect spec doesn't mandate sequencer-level
runtime gate.

Adding sequencer-side admission gate WOULD be a strictly stronger
guarantee, but:
- Modifies hot path `submit_agent_tx`
- Class 3+/4 risk (could break test surface)
- Conflicts with future TB charters that need different sandbox semantics
- Architect spec doesn't require it

**Recommendation**: keep audit-time HALT (current implementation); document
this position; if architect ratification desired, escalate.

---

## §6 Bottom line

R2 Gemini VETO **does not represent a real ship-blocking gap** — 4 of 5
VETOs are STALE (audit predates Step 4 reality). 1 VETO (Q2) + 5
CHALLENGEs are real and surgically fixable.

**Path to PROCEED**:
- Apply 6 surgical fixes above (~3-4 hours estimated)
- Re-run R3 dual audit on updated state
- Expected R3: PASS or CHALLENGE-only

**Codex R2 NOT YET RUN** — should also fire to get full dual-auditor
view post-Step-4. Codex's R1 VETOs were 5 (all production-defect class);
Step 1 + Step 4 should have closed most of them (V3-V7 + bug fix).
Run `bash handover/audits/run_codex_tb_16_ship_audit.sh` with
`TB16_AUDIT_ROUND=R2` env to invoke.

---

## §7 Cross-references

- TB-16 charter: `handover/tracer_bullets/TB-16_charter_2026-05-04.md`
- Architect §7: `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md`
- R1 verdicts: `handover/audits/CODEX_TB_16_SHIP_AUDIT_2026-05-04_R1.md`,
  `handover/audits/GEMINI_TB_16_SHIP_AUDIT_2026-05-04_R1.md`
- R1 closure: `handover/audits/RECURSIVE_AUDIT_TB_16_2026-05-04.md`
- R2 Gemini: `handover/audits/GEMINI_TB_16_SHIP_AUDIT_2026-05-04_R2.md`
- Step 1 commit: `3cf4c36`
- Step 3 commit: `05e3e86`
- Step 4 commit: `d1c1af2`
- Step 4 evidence: `handover/evidence/tb_16_real_llm_arena_2026-05-04/arena_run4/`,
  `handover/evidence/tb_16_real_llm_arena_2026-05-04/arena_run6_exhaust/`
