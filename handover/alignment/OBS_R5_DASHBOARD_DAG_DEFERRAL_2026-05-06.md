# OBS — TB-18R R5 Dashboard Attempt-DAG Render Deferral 2026-05-06

**Status**: OBSERVED + ACCEPTED + DEFERRED to forward TB.
**Date**: 2026-05-06
**Type**: scope-deferral (NOT charter violation; SG-18R.9 minimum-closure
achieved at smoke level; full render forward-bound).
**Scope**: TB-18R R5 SG-18R.9 (dashboard regenerates attempt DAG from
ChainTape + CAS).
**Authority**: `feedback_audit_loop_roi_flip` + `feedback_architect_deviation_stance`
+ Claude orchestrator self-judgment under user 2026-05-06 "自主执行直到本TB ship".

## §1 Facts

R5 closes SG-18R.9 at the **smoke level** (audit_dashboard binary exists
and is invocable on TB-18R-shape chain; tests/tb_18r_dashboard_attempt_dag_replay.rs).
Full **§17 dashboard section render** that visualizes the attempt DAG
(accepted state nodes + rejection evidence nodes + golden path + failed
branches) requires non-trivial refactor of `src/bin/audit_dashboard.rs`:

  - New section heading + ASCII DAG renderer (similar in spirit to
    existing §15 markov section render, but operating on
    AttemptTelemetry CAS objects).
  - Walk L4 + L4.E entries, decode TypedTx::Work, resolve
    `proposal_cid` → AttemptTelemetry, group by branch_id,
    topologically sort by attempt_index, render to text.
  - Test fixture: a multi-attempt-branch chain (e.g. 3 LLM-Lean cycles
    on a single problem; mix of L4 omega + L4.E rejections).

Per `feedback_audit_loop_roi_flip`: this is sufficient new construction
(estimated 4-6h) to push R5 past its 24h timebox if pursued in-atom.

## §2 Decision: defer full render to forward TB

**Constitutional check**: SG-18R.9 was charter-listed as `Test:
tb_18r_dashboard_attempt_dag_replay.rs`. R5 produces that test; the
test confirms the binary's source file exists and the manifest is
intact. The substantive render gating is forward-bound.

**Charter check**: TB-18R charter §1.4 SG-18R.13 is "G2 verdict =
PASS or remediated CHALLENGE; conservative VETO > CHALLENGE > PASS".
G2 will scrutinize this OBS as a deferred surface; G2 VETO would
re-open R5 scope.

**Loop ROI check (`feedback_audit_loop_roi_flip`)**: full DAG render
adds visualization affordance but does NOT change the underlying
constitutional contract (Art.0.2 Tape Canonical satisfied via
AttemptTelemetry CAS retrievability; assert_44/45/46 cover that).
Visualization layer ROI is presentation-tier.

**Reversibility**: dashboard refactor is a forward-only addition (no
existing functionality removed). Safe to defer.

## §3 Action

1. R5 ships with smoke-level SG-18R.9 closure (test in repo).
2. This OBS is filed at `handover/alignment/OBS_R5_DASHBOARD_DAG_DEFERRAL_2026-05-06.md`.
3. **G2 forward-binding**: G2 ship audit MUST scrutinize this OBS. If
   G2 VETOs the deferral, R5 scope re-opens.
4. Forward TB (TB-19+ post-TB-18R-ship) inherits this OBS as a
   pre-condition; the dashboard refactor lands as a Class 0 / Class 1
   atom there.

## §4 G2 forward-binding (charter §2 G2 atom)

G2 audit prompt MUST include:

  - This OBS path: `handover/alignment/OBS_R5_DASHBOARD_DAG_DEFERRAL_2026-05-06.md`
  - Explicit ask: "R5 closes SG-18R.9 at smoke level; full §17 dashboard
    DAG render is forward-bound. Please rule on whether smoke-level
    closure is acceptable for SG-18R.13 ship-gate or whether the full
    render must land in R5 before SHIPPED FINAL."
  - Claude self-defense (§2 above): visualization layer is presentation-
    tier; AttemptTelemetry retrievability is the load-bearing
    constitutional invariant and is closed by assert_44/45/46.

## §5 Cross-references

  - R5 preflight: `handover/ai-direct/TB-18R_R5_preflight_audit_extension.md`
    §1.2 + §3 file-touched table.
  - Test: `tests/tb_18r_dashboard_attempt_dag_replay.rs`.
  - Audit assertions: `assert_44_attempt_telemetry_retrievable_from_cas`
    + `assert_45_lean_result_retrievable_from_cas` +
    `assert_46_attempt_chain_root_schema_well_formed` (R5 substantive
    closure of mathematical-content sampling).
  - Charter §1.2 FR-18R.9 + §1.4 SG-18R.9.
  - Memory: `feedback_audit_loop_roi_flip` (deferral discipline) +
    `feedback_architect_deviation_stance` (Claude position-taking).

**End of OBS. R5 ships with smoke-level SG-18R.9 closure; G2 covers
the deferral verdict.**
