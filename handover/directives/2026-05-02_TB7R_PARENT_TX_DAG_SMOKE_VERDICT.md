# Architect Verdict 2026-05-02 — TB-7R parent_tx / DAG / Smoke Ruling

**Date**: 2026-05-02
**Predecessor**: `2026-05-01_TB7R_AUTHORIZATION_VERDICT.md` (initial TB-7R authorization)
**Trigger**: Claude self-assessment 2026-05-02 surfaced two questions —
(1) parent_tx_edges=0 across all 10 smoke runs (verdict §F criterion gap?);
(2) "is the system actually exhibiting agent self-exploration, or just one-step solves?"
**Status**: BINDING

---

## §1 Executive verdict

TB-7R can proceed, **but the parent_tx gate must be rewritten as a
conditional invariant, not an unconditional smoke requirement.**

If a real LLM run solves in a single externalized proposal under B′
complete-tool semantics, then:

- `parent_tx_edges = 0` is valid;
- the golden path is a singleton node;
- there is no DAG defect.

However, the parent_tx plumbing must still be proven by a deterministic
conformance test or a controlled multi-attempt run. **Do not fabricate
edges in natural smoke evidence.**

---

## §2 Binding decisions

1. **Do not block ship** solely because full smoke has zero parent_tx
   edges when all successful runs solved in one externalized proposal.

2. **Add a separate parent_tx conformance test**:
   - attempt_1 = root proposal, parent_tx=None;
   - attempt_2 = same agent/branch, parent_tx=attempt_1;
   - dashboard reconstructs edge attempt_1 → attempt_2.

3. **Replace the unconditional verdict criterion**:
   - OLD: `full smoke must have ≥1 parent_tx edge`
   - NEW: `if a run has ≥2 externalized proposals on the same agent/branch, all non-root attempts must have parent_tx`.

4. **Dashboard must distinguish**:
   - `SingletonGoldenPathValid`
   - `NoMultiAttemptObserved`
   - `MissingParentTxViolation`

5. Golden path rendering must support length=1 path.

6. Do **not** decompose complete-tool output into per-tactic nodes in TB-7R.

7. Do **not** record private CoT.

8. Proposal-level DAG is sufficient for TB-7R; tactic-level DAG is a future TB.

---

## §3 Required tests

1. `singleton_golden_path_has_zero_edges_and_is_valid`
2. `second_attempt_same_branch_has_parent_tx`
3. `missing_parent_on_nonroot_attempt_is_violation`
4. `dashboard_renders_singleton_golden_path`
5. `unsolved_runs_have_no_fake_accepted_nodes`
6. `proposal_count_chain_equals_externalized_proposal_count`

All six implemented at `tests/tb_7r_parent_tx_conformance.rs` — 6/6 pass.

---

## §4 Ship conditions (TB-7R may ship if)

- All seven dashboard indicators remain green ✓
- All real externalized proposals are represented in L4 or L4.E ✓
- Solved runs have `chain_oracle_verified=true` and a rendered golden path ✓
- Unsolved runs have no fake accepted nodes ✓
- Proposal telemetry and proposal payload CIDs resolve ✓
- **Forced parent_tx conformance test passes** ✓ (6/6 in `tests/tb_7r_parent_tx_conformance.rs`)
- README explicitly states that natural `parent_tx_edges=0` occurred because complete-tool runs solved in one proposal ✓ (smoke README §2 updated 2026-05-02)

---

## §5 Do not

- Fabricate parent_tx edges.
- Fail a one-attempt solved run for having no parent edge.
- Launch NodeMarket.
- Implement slash / payout / settlement.
- Start per-tactic decomposition inside TB-7R.
- Treat dashboard as source of truth; it is a materialized view.

---

## §6 Next insight (post-TB-7R)

The most important remaining audit risk is the **coverage denominator**:

> "ChainTape can only prove what reached it. The next hardening step is
> to ensure every LLM response that becomes an externalized proposal is
> counted before submission and must either land in L4/L4.E or fail-closed.
> Without this, a hidden legacy path can still produce unchained proposals."

This is logged as a post-TB-7R follow-up at
`handover/alignment/OBS_TB7R_COVERAGE_DENOMINATOR_2026-05-02.md`.

---

## §7 Cross-references

- TB-7R authorization: `handover/directives/2026-05-01_TB7R_AUTHORIZATION_VERDICT.md`
- TB-7R charter: `handover/tracer_bullets/TB-7R_charter_2026-05-01.md`
- TB-7R smoke evidence: `handover/evidence/tb_7r_smoke_2026-05-02/README.md` (§2 updated)
- Conformance tests: `tests/tb_7r_parent_tx_conformance.rs`
- TRACE_MATRIX orphan registry: `handover/alignment/OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02.md`
- L4 / L4.E ledger separation: `handover/alignment/DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md`
