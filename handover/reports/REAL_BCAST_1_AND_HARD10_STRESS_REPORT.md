# REAL-BCAST-1 Librarian Broadcast + hard10 Stress Report

Date: 2026-05-16
Worktree: `/home/zephryj/projects/turingosv4-real12-action-probes`
Branch: `codex/real12-economic-action-probes`

## Summary

REAL-BCAST-1 has a clean implementation/evidence path after explicit user
authorization for the Trust Root rehash. The Librarian Broadcast Loop now
generates CAS-backed, role-cropped digest evidence and injects bounded notices
through PromptCapsule read sets without introducing new TypedTx, CAS
ObjectType, full async, raw-log broadcast, or dashboard-as-truth.

The expanded hard10 real-problem stress run answers the user's concern about
too-easy tests: harder problems do create multi-append pressure, partial proof
progress, repeated rejections, many market review windows, and a large volume
of EVDecisionTrace/LibrarianDigest evidence. It still does not produce live
agent economic action.

Conclusion:

```text
REAL-BCAST-1: broadcast substrate works under smoke and hard10 pressure.
E2: NOT ACHIEVED.
Weak point exposed: live agents repeatedly classify opportunities as NegativeEV / abstain, even with market review windows and Librarian notices.
```

## Scope And Constitutional Boundary

Touched FC nodes/invariants:

```text
FC1: role-scoped broadcast prompt loop
FC2: PromptCapsule / CAS replay binding
FC3: dashboard and audit as materialized views
Art. II: broadcasting + shielding
Art. 0.2: no parallel source of truth
```

Class-4 event:

```text
Trust Root rehash for modified pinned files and new runtime module.
```

Forbidden surfaces not touched:

```text
TypedTx schema/discriminants
sequencer admission
canonical signing payload
CAS ObjectType schema
constitution/flowchart text
genesis authority semantics beyond Trust Root pinned hash rehash
```

## Implementation Evidence

Key files:

```text
src/runtime/librarian_broadcast.rs
src/runtime/market_review.rs
src/runtime/mod.rs
experiments/minif2f_v4/src/bin/evaluator.rs
src/bin/audit_dashboard.rs
tests/constitution_librarian_*.rs
tests/constitution_real13b_market_review_window.rs
genesis_payload.toml
```

Design artifacts:

```text
handover/directives/2026-05-16_REAL_BCAST_1_LIBRARIAN_BROADCAST_LOOP_ARCHITECT_ORIGINAL.md
handover/directives/2026-05-16_REAL_BCAST_1_LIBRARIAN_BROADCAST_LOOP_EXECUTION_PLAN.md
handover/directives/2026-05-16_REAL13_STATUS_SYNC_FOR_ARCHITECT.md
handover/directives/2026-05-16_REAL_BCAST_1_CLASS4_RATIFICATION.md
handover/directives/REAL_BCAST_1_LIBRARIAN_BROADCAST_LOOP_CHARTER.md
handover/alignment/DECISION_LIBRARIAN_DIGEST_MATERIALIZED_VIEW.md
handover/alignment/DECISION_LIBRARIAN_BARRIERED_ASYNC_BROADCAST.md
handover/alignment/OBS_REAL_BCAST_1_TRUST_ROOT_REHASH_REQUIRED.md
```

Self-hosting run:

```text
handover/evidence/dev_self_hosting/dev_1778925622575_2782689
```

## Verification Commands

Trust Root:

```text
cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo -- --test-threads=1
exit=0
evidence: handover/evidence/dev_self_hosting/dev_1778925622575_2782689/artifacts/command_0001_*
```

Targeted REAL-BCAST tests:

```text
cargo test \
  --test constitution_librarian_source_scope \
  --test constitution_librarian_selector \
  --test constitution_librarian_digest \
  --test constitution_librarian_role_projector \
  --test constitution_librarian_prompt_injection \
  --test constitution_librarian_half_async \
  --test constitution_librarian_no_raw_leakage \
  --test constitution_librarian_real_evidence_binding \
  --test constitution_real13b_market_review_window
exit=0
evidence: handover/evidence/dev_self_hosting/dev_1778925622575_2782689/artifacts/command_0002_*
```

Constitution gates:

```text
bash scripts/run_constitution_gates.sh
exit=0
Totals: 461 passed, 0 failed, 1 ignored
PASS: all gates GREEN
evidence: handover/evidence/dev_self_hosting/dev_1778925622575_2782689/artifacts/command_0006_*
```

Workspace tests:

```text
cargo test --workspace --no-fail-fast -- --test-threads=1
exit=0
evidence: handover/evidence/dev_self_hosting/dev_1778925622575_2782689/artifacts/command_0007_*
```

## REAL-BCAST A/B Smoke

Arm A:

```text
run: handover/evidence/real_bcast_1_ab_A_20260516T100140Z
problem_set: mini
MAX_TRANSACTIONS=8
PER_PROBLEM_TIMEOUT_S=360
TURINGOS_REAL_BCAST_LIBRARIAN=0
audit_tape: PROCEED
librarian_digest_cas_count: 0
librarian_role_crop_cas_count: 0
librarian_shielding_verdict: PASS
ev_decision_trace_total_cas: 8
market_review_summary_cas_count: 8
live_non_scripted_router_tx_count: 0
E2: NOT ACHIEVED
```

Arm B:

```text
run: handover/evidence/real_bcast_1_ab_B_20260516T100140Z
problem_set: mini
MAX_TRANSACTIONS=8
PER_PROBLEM_TIMEOUT_S=360
TURINGOS_REAL_BCAST_LIBRARIAN=1
audit_tape: PROCEED
librarian_digest_cas_count: 14
librarian_role_crop_cas_count: 14
librarian_shielding_verdict: PASS
ev_decision_trace_total_cas: 8
market_review_summary_cas_count: 8
live_non_scripted_router_tx_count: 0
E2: NOT ACHIEVED
```

Interpretation:

```text
The ON arm proves CAS-backed LibrarianDigest/role crop evidence is generated
and visible to audit_dashboard. This is not a causal performance claim.
```

## Expanded hard10 Stress Test

Run:

```text
handover/evidence/real_bcast_1_hard10_B_20260516T100140Z
problem_set: handover/preregistration/sample_E1v2_hard10_S20260423.txt
problem_count: 10
MAX_TRANSACTIONS=30
PER_PROBLEM_TIMEOUT_S=900
TURINGOS_REAL_BCAST_LIBRARIAN=1
audit_tape: PROCEED
persistence_passing: true
persistence_n_witnessed: 5
```

Aggregate dashboard/audit metrics:

```text
tx_count: 213
task_outcome_market_count: 10
structural_market_tx_count: 24
resolution_tx_count: 10
market_seed: 12
cpmm_pool: 12
buy_with_coin_router: 0
agent_economic_action_tx_count: 0
scripted_or_unproven_router_tx_count: 0
event_resolve: 10
work: 2
verify: 4
ev_decision_trace_total_cas: 106
ev_decision_trace_bull_count_cas: 53
ev_decision_trace_bear_count_cas: 53
ev_decision_trace_abstain_count_cas: 106
ev_decision_reason_NegativeEV: 106
market_review_summary_cas_count: 106
librarian_digest_cas_count: 261
librarian_role_crop_cas_count: 261
librarian_shielding_verdict: PASS
```

Per-problem summary:

```text
P000 algebra_bleqa_apbon2msqrtableqambsqon8b: solved=false tx=30 partial=0 rejects=6 windows=12 ev=12 no_edge=30
P001 amc12_2000_p12: solved=false tx=30 partial=1 rejects=5 windows=12 ev=12 no_edge=30
P002 amc12_2000_p6: solved=false tx=30 partial=4 rejects=2 windows=12 ev=12 no_edge=30
P003 amc12b_2021_p13: solved=false tx=30 partial=4 rejects=2 windows=12 ev=12 no_edge=30
P004 imo_1962_p2: solved=false tx=30 partial=1 rejects=5 windows=12 ev=12 no_edge=30
P005 mathd_algebra_208: solved=false tx=30 partial=6 rejects=0 windows=12 ev=12 no_edge=30
P006 mathd_algebra_246: solved=true tx=8 partial=1 rejects=0 windows=4 ev=4 no_edge=7
P007 mathd_algebra_270: solved=true tx=13 partial=1 rejects=1 windows=6 ev=6 no_edge=12
P008 mathd_algebra_332: solved=false tx=30 partial=0 rejects=6 windows=12 ev=12 no_edge=30
P009 numbertheory_2pownm1prime_nprime: solved=false tx=30 partial=3 rejects=3 windows=12 ev=12 no_edge=30
```

Interpretation:

```text
The previous concern was correct: mini tasks were too easy to test the full
architecture. hard10 creates multi-turn pressure:

- 8/10 tasks unsolved.
- 7/10 tasks hit the 30 tx cap or near sustained pressure.
- Partial progress exists and is externalized, not private CoT.
- Market review windows fire repeatedly.
- Librarian digest volume is large and shielded.
- Live non-scripted router tx remains zero.
```

## Findings

1. Broadcasting substrate gap is now closed at the MVP level.

Evidence:

```text
Arm B librarian_digest_cas_count=14, role_crop=14, shielding=PASS.
hard10 librarian_digest_cas_count=261, role_crop=261, shielding=PASS.
PromptCapsule read_set linkage is covered by constitution_librarian_prompt_injection.
Half-async digest/epoch contract is covered by constitution_librarian_half_async.
```

2. The architecture now has enough difficult-task evidence to study partial progress.

Evidence:

```text
hard10 partial progress counts include 1,4,4,1,6,1,1,3 across several tasks.
mathd_algebra_208 is especially informative: solved=false, tx=30, partial=6, rejects=0.
```

This is not black-box thought. It is externalized attempt + LeanResult evidence.

3. E2 remains absent under stronger pressure.

Evidence:

```text
hard10 buy_with_coin_router=0
hard10 agent_economic_action_tx_count=0
hard10 ev_decision_trace_buy_yes_count_cas=0
hard10 ev_decision_trace_buy_no_count_cas=0
hard10 ev_decision_reason_NegativeEV=106
```

This means the next bottleneck is no longer "did agents receive broadcasted
evidence?" The bottleneck is that the EV/action policy still classifies every
market review as negative EV or abstain.

4. Harder tests should be the new default for architecture probes.

Evidence:

```text
mini A/B: 3 tasks, 8 EV traces, 8 review summaries.
hard10: 10 tasks, 106 EV traces, 106 review summaries, 261 Librarian digests.
```

The hard10 profile reveals phenomena that mini cannot: repeated partial
progress, sustained unresolved tasks, and many review windows over one shared
runtime_repo.

## Recommendation

Do not claim:

```text
spontaneous market emergence
causal performance improvement
E2
E3
model ranking
price-as-truth
```

Do claim narrowly:

```text
REAL-BCAST-1 MVP is implemented and verified.
LibrarianDigest/role crops are CAS-backed, shielded, and dashboard-regenerable.
hard10 produces the richer multi-turn evidence needed for architecture testing.
Current live economic behavior remains abstain-only under EVDecisionTrace.
```

Recommended next mechanism work:

```text
REAL-13A follow-up: EV/action policy refinement.
Focus on why every review emits NegativeEV:
  - price/agent probability calibration
  - amount/risk cap threshold
  - actionable-market definition
  - whether NoPerceivedEdge/NegativeEV is over-conservative
  - whether private alpha or expected probability estimates are too weak

Then rerun hard10 with:
  MAX_TRANSACTIONS=30 or 50
  PER_PROBLEM_TIMEOUT_S=900 or 1200
  Librarian ON/OFF A/B if comparing broadcast effect
```

## Evidence Paths

```text
handover/evidence/dev_self_hosting/dev_1778925622575_2782689
handover/evidence/real_bcast_1_ab_A_20260516T100140Z
handover/evidence/real_bcast_1_ab_B_20260516T100140Z
handover/evidence/real_bcast_1_hard10_B_20260516T100140Z
```

