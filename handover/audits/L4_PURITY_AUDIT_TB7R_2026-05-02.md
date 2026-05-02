# L4 Purity Audit — TB-7R Deliverable A

**Date**: 2026-05-02
**Auditor**: claude (TB-7R Deliverable A)
**Verdict**: **ZERO VIOLATIONS — all reachable L4 accepted entries are pure under TB-7R criteria.**
**OBS file required**: NO (vacuous + verified pass).

---

## Audit criteria (per `feedback_no_retroactive_evidence_rewrite` + verdict B2)

For each L4 accepted entry where `tx_kind == Work`, verify all four:

```text
1. ProposalTelemetry resolves from CAS via WorkTx.proposal_cid
2. ProposalTelemetry.verification_result_cid is Some(...) and resolves
3. The resolved VerificationResult.verified == true
4. VerificationResult.proof_artifact_cid resolves from CAS
```

If any L4 Work entry fails 1-4, that's a purity violation requiring an OBS
file. **NOT** a migration trigger — the L4 chain stays as-is, and the
affected evidence dir is README-annotated as unusable for accepted-state
claim. Per verdict B2: **NO retroactive ledger rewrite.**

L4 entries with `tx_kind != Work` (TaskOpen, EscrowLock, Verify) are
out-of-scope for this audit — they have separate predicates, none of which
enshrine a "Lean-passed" claim into authoritative state.

---

## Audit results by evidence directory

### `handover/evidence/tb_7_7_dag_capable_smoke_2026-05-01/` (TB-7.7 D7)

```
L4 entries:                      3
  ├─ logical_t=1  TaskOpen       (tb7-7-sponsor)        — out of scope
  ├─ logical_t=2  EscrowLock     (tb7-7-sponsor)        — out of scope
  └─ logical_t=3  Work           (Agent_0)              — in scope
        tx_id: worktx-task-n5_mathd_algebra_171_1777652328726-omega-pertactic-1
        tactic: step_complete    branch: Agent_0.b1
        oracle: ✓
```

**Audit on the 1 in-scope Work entry**:

| Criterion | Result | Evidence |
|---|---|---|
| 1. ProposalTelemetry resolves | ✓ | dashboard.txt §2 Gate 5 `proposal_telemetry_cas_retrievable: ✓` |
| 2. verification_result_cid resolves | ✓ | dashboard.txt §3 `chain_oracle_verified: true ✓ (Lean accepted ≥1 proof)` requires VR resolve per `chain_derived_run_facts.rs:332-340` |
| 3. VerificationResult.verified == true | ✓ | same: `chain_oracle_verified=true` requires `vr.verified == true` |
| 4. proof_artifact_cid resolves | ✓ | §7 Golden path renders payload at `[ORACLE]` depth=0; payload is computed from `proof_artifact_cid` lookup |

**Verdict**: PURE (1/1 L4 Work entry passes all 4 criteria).

---

### `handover/evidence/tb_7_chaintape_smoke_2026-05-01/` (TB-7 Atom 6 synthetic-LLM)

```
L4 entries:   1   (TaskOpen — synthetic system signer)
L4.E entries: 6   (zero-stake WorkTx + VerifyTx routed to L4.E by design)
```

In-scope L4 Work entries: **0** (the single L4 entry is TaskOpen).

**Verdict**: PURE (vacuously — no L4 Work entries to audit).

Note: zero L4 Work is the *intended* TB-7 Atom 6 shape per README §2 — zero-stake admission rejection routes every WorkTx to L4.E. Not a purity violation.

---

### `handover/evidence/tb_7_real_smoke_5_problems_2026-05-01/` (5 real-LLM runs)

Each `run_<i>_<problem>/l4_chain_log.txt` shows exactly one transition at `logical_t=1`. Per README §1, every run's L4 is a TaskOpen (no real-LLM WorkTx reaches L4 acceptance under the zero-stake design):

```
run_1_mathd_algebra_107       L4=1 (TaskOpen),  L4.E=3
run_2_mathd_algebra_171       L4=1 (TaskOpen),  L4.E=4 (CAS race blocks dashboard, not purity)
run_3_mathd_algebra_359       L4=1 (TaskOpen),  L4.E=3
run_4_aime_1997_p9            L4=1 (TaskOpen),  L4.E=2
run_5_mathd_numbertheory_5    L4=1 (TaskOpen),  L4.E=4 (CAS race blocks dashboard, not purity)
```

In-scope L4 Work entries across all 5 runs: **0**.

**Verdict**: PURE (vacuously, ×5).

Note: runs 2 + 5 hit the pre-existing CAS index race (already fixed at `c0ec514` TB-7.6). The CAS race blocks dashboard rendering, not L4 purity.

---

## Aggregate verdict

```text
Total evidence dirs audited:            3 (7 distinct runtime_repos counted across run_<i>_*)
Total L4 entries reviewed:              9
Total L4 Work entries (in scope):       1
Purity violations:                      0
OBS file required:                      No
README annotations required:            Yes (per Deliverable E — grandfathering note,
                                              independent of purity outcome)
```

The single in-scope L4 Work entry (in TB-7.7 D7 evidence) passes all four
TB-7R purity criteria. All other L4 entries in reachable evidence are
TaskOpen / EscrowLock — out of purity scope by definition.

The implication for TB-7R is structural: there is **no historical ledger
state to grandfather as "predicate-failed-but-accepted"**. New TB-7R smoke
runs are starting from a clean L4 baseline.

## Cross-references

- TB-7R charter: `handover/tracer_bullets/TB-7R_charter_2026-05-01.md` §3 Deliverable A
- TB-7R authorization: `handover/directives/2026-05-01_TB7R_AUTHORIZATION_VERDICT.md` B2
- L4 / L4.E ledger separation: `handover/alignment/DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md`
- Three-node taxonomy: `handover/alignment/DECISION_ATTEMPT_STATE_REJECTION_NODES_2026-05-01.md`
- chain_oracle_verified computation: `src/runtime/chain_derived_run_facts.rs:199-340`
