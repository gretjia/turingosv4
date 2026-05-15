# CODEX REAL-6A Implementation Review R4

Date: 2026-05-15

Reviewer: clean-context Codex `gpt-5.5` / `xhigh`

Verdict: `PROCEED`

## Findings

No blocking findings found. I reviewed the Class 4/restricted surfaces against FC1 market/tape traces, FC2 TaskOpen/Escrow seeding/replay authority, and FC3 audit/materialized evidence. The prior R1/R2/R3 VETO items appear closed.

## Key Checks

EventResolve compatibility is now an exact dual-reader, not serde error-string matching: current decode must consume the full slice, legacy decode must also consume the full slice, and legacy maps only the old 6-field wire to YES. See `src/bottom_white/ledger/transition_ledger.rs:686` and tests for corrupt/partial tails in `tests/constitution_real6_task_outcome_market.rs:627`.

Legacy YES signatures cannot authorize NO: the sequencer only attempts the legacy signature path when `outcome == OutcomeSide::Yes`, and the regression test mutates YES to NO and expects rejection. See `src/state/sequencer.rs:791` and `src/state/sequencer.rs:5206`.

REAL-6A fail-closed behavior is present: CAS write failures for `MarketDecisionTrace` exit before final stdout evidence can be produced, seed errors exit under the feature flag, YES resolution exits on skipped/error paths, and NO exhaustion resolution exits on real errors. See `experiments/minif2f_v4/src/bin/evaluator.rs:613`, `experiments/minif2f_v4/src/bin/evaluator.rs:1809`, `experiments/minif2f_v4/src/bin/evaluator.rs:4268`, and `experiments/minif2f_v4/src/bin/evaluator.rs:6288`.

r10 supports the claimed SG-6A subset without overclaiming emergence: it has `task_open=1`, `escrow_lock=1`, `market_seed=1`, `cpmm_pool=1`, `event_resolve=1`, `terminal_summary=1`, `work=0`, `verdict=PROCEED`, and economy assertions green in `handover/evidence/g_phase_real_6a_task_outcome_smoke_r10_20260515T0442Z/aggregate_verdict.json:11`. The stdout shows `hit_max_tx=true` and classified `invest_no_trade_no_perceived_edge:5` in `handover/evidence/g_phase_real_6a_task_outcome_smoke_r10_20260515T0442Z/P000_numbertheory_2pownm1prime_nprime/evaluator.stdout:1`. I also decoded the r10 logical_t=6 EventResolve payload from CAS; the outcome discriminant is `01` (`NO`).

r8 is correctly treated as a non-final solved-path probe: the report says it had `event_resolve=0` and is not used as final SG-6A.6/SG-6A.7 evidence. See `handover/evidence/real6_task_outcome/REAL6A_TASK_OUTCOME_MARKET_REPORT.md:207`.

The Trust Root rehash boundary is explicit and not overclaimed as semantic review of unrelated dirty pinned files. See `handover/evidence/real6_task_outcome/TRUST_ROOT_REHASH_REAL6A_WORKSPACE_NORMALIZATION.md:8` and `handover/evidence/real6_task_outcome/REAL6A_RESTRICTED_SURFACE_AUDIT_NOTE.md:13`.

## Evidence Checked

The recorded harness artifacts match the requested command sequence: command_0109 is intentionally RED, then 0110/0111/0112/0113/0120/0123/0124/0125/0126 are green in `handover/evidence/dev_self_hosting/dev_1778811250681_913405/events.jsonl:110`. Constitution gates report `436 passed, 0 failed, 1 ignored`, and workspace tests exited 0.

## Residual Risks

r10 is a hard-problem NO-path smoke, not a fresh real solved-path YES smoke after the final YES fail-closed hardening. YES is covered by source and unit/constitution gates, but not by final r10 runtime evidence.

The MarketDecisionTrace shared-slot compatibility still stores these traces under `AttemptTelemetry` object type with schema-specific JSON inside; tests cover the classifier, but a future schema migration would make this less transitional.

## Verdict

PROCEED
