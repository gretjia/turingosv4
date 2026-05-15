# REAL-6A TaskOutcomeMarket Report

Date: 2026-05-15

Directive source:

- `handover/directives/2026-05-15_REAL5S_REAL6_REAL7_REAL8_REAL9_ARCHITECT_ORIGINAL.md`
- `handover/directives/2026-05-15_REAL5S_REAL6_REAL7_REAL8_REAL9_EXECUTION_PLAN_APPROVED.md`

Harness run:

- `dev_1778811250681_913405`

## Claim Boundary

REAL-6A implements the architect ruling:

```text
TaskOpenTx / EscrowLockTx
-> MarketSeedTx for task_outcome event
```

The event is:

```text
task will be solved within budget/deadline.
```

This report claims only:

```text
TaskOutcomeMarket can be created at task opening,
Trader-visible market turns can produce tape-visible NoTradeReason traces,
exhaustion can resolve the TaskOutcomeMarket as NO on ChainTape,
audit_tape can replay the evidence,
and the price signal stays non-predicate.
```

This report does not claim:

```text
spontaneous trading emergence,
role differentiation,
or market-driven solve improvement.
```

## Implementation Summary

Code paths changed for REAL-6A:

- `src/runtime/real6_task_outcome.rs`: TaskOutcomeEvent helper, deterministic event id, price signal, seed outcome type.
- `src/runtime/adapter.rs`: REAL-6A seed helper, TaskOutcomeMarket router helper, and exhaustion NO-resolution helper.
- `src/runtime/mod.rs`: exports `real6_task_outcome`.
- `src/state/typed_tx.rs`: `EventResolveTx.outcome: OutcomeSide` with old bincode wire compatibility.
- `src/state/sequencer.rs`: system `EventResolve` supports YES and NO outcomes; YES legacy signatures are grandfathered, but legacy signatures cannot authorize NO.
- `experiments/minif2f_v4/src/bin/evaluator.rs`: seeds TaskOutcomeMarket immediately after EscrowLock when `TURINGOS_REAL6_TASK_OUTCOME_MARKET=1`; routes `invest` on `task-<run_id>` to the TaskOutcomeMarket; emits end-of-turn `MarketDecisionTrace::no_trade(NoPerceivedEdge)` when a market-visible role turn does not invest; emits `EventResolveTx outcome=NO` after MaxTxExhausted when the TaskOutcomeMarket remains open.
- `experiments/minif2f_v4/src/drive_task.rs`: seeds TaskOutcomeMarket in the drive-task path.
- `scripts/run_g_phase_batch.sh`: release build now fails closed before `batch_evaluator` launch; this prevents stale release binaries from creating false smoke evidence.
- `tests/constitution_real6_task_outcome_market.rs`: SG-6A fixtures and red/green gates.
- `tests/constitution_n2_event_resolve.rs`: EventResolve compatibility coverage.

Shared-slot hardening retained during REAL-6A:

- `src/runtime/attempt_telemetry.rs`: `read_attempt_telemetry_shared_slot_from_cas`.
- `src/runtime/audit_assertions.rs`: bulk audit walkers skip recognized `MarketDecisionTrace` JSON and fail closed on unknown JSON.
- `src/runtime/chain_derived_run_facts.rs`: ChainDerivedRunFacts uses the shared-slot classifier instead of silently ignoring decode failures.
- `src/bin/audit_dashboard.rs`: run-report model-family walker uses the shared-slot classifier.
- `tests/tb_18r_attempt_telemetry_serialize.rs`: gate proving recognized `MarketDecisionTrace` JSON skips and unknown JSON fails closed.

R2 VETO remediation:

- `src/state/typed_tx.rs`: legacy EventResolve compatibility now defaults missing bincode outcome tails to YES only for EOF / missing-tail errors; malformed outcome tails fail closed.
- `experiments/minif2f_v4/src/bin/evaluator.rs`: `MarketDecisionTrace` CAS writes now use a fail-closed helper. If CAS open/write fails, the evaluator exits before stdout/tool_dist can claim no-trade or submitted-trade evidence.
- `tests/constitution_real6_task_outcome_market.rs`: adds gates for malformed EventResolve outcome tail rejection and non-best-effort MarketDecisionTrace CAS writes.

R3 VETO remediation:

- `src/bottom_white/ledger/transition_ledger.rs`: `canonical_decode::<TypedTx>` now uses an explicit current-or-legacy EventResolve dual reader. Current bincode bytes must consume the entire slice; legacy 6-field EventResolve bytes are accepted as YES; partial/corrupt current outcome tails fail closed instead of relying on serde error strings.
- `experiments/minif2f_v4/src/bin/evaluator.rs`: REAL-6A TaskOutcomeMarket seed, YES resolution, and NO resolution paths are fail-closed when `TURINGOS_REAL6_TASK_OUTCOME_MARKET=1`. Smoke evidence cannot silently continue after seed or resolution failure.
- `tests/constitution_real6_task_outcome_market.rs`: adds source gates for exact dual-reader behavior and fail-closed seed/YES/NO evaluator paths.

## Clean-Context Review R1

Clean-context reviewer:

- File: `handover/audits/CODEX_REAL6A_IMPLEMENTATION_REVIEW.md`
- Reviewer: Codex `gpt-5.5` / `xhigh`
- Verdict: `VETO`

Findings and remediation:

- P0 EventResolve compatibility not proven / non-tail-add risk.
  - Remediation: manual compatibility decode for old EventResolve bincode bytes, old-field-prefix preserved, `outcome` defaults to YES for legacy bytes.
  - Evidence: `command_0063` intentionally RED; `command_0064` passed after compatibility fix.
  - Additional signature evidence: `command_0067` proved legacy YES signatures are grandfathered; legacy authorization is not accepted for NO.

- P0 SG-6A.7 production exhaustion path missing EventResolve NO.
  - Remediation: added `tb_real6a_emit_task_outcome_no_after_exhaustion` and wired evaluator MaxTxExhausted cleanup to emit `EventResolveTx outcome=NO` when REAL-6A is enabled and the task market remains open.
  - Evidence: `command_0065`, `command_0080`, and r6 smoke below.

- P1 Trust Root normalization promoted broad dirty pinned files outside REAL-6A semantic surface.
  - Remediation: claim boundary narrowed and restricted-surface audit note added.
  - Evidence note: `handover/evidence/real6_task_outcome/REAL6A_RESTRICTED_SURFACE_AUDIT_NOTE.md`.

- P2 TraderView coverage thin.
  - Remediation: SG-6A.2 test now requires market signal, pool depth, deadline/budget context, and TraderView public sections for `pool depth`, `PnL`, `balance`, and `recent accepted WorkTx`.
  - Evidence: `command_0080`.

## Clean-Context Review R2

Clean-context reviewer:

- File: `handover/audits/CODEX_REAL6A_IMPLEMENTATION_REVIEW_R2.md`
- Reviewer: Codex `gpt-5.5` / `xhigh`
- Verdict: `VETO`

Findings and remediation:

- P0 EventResolve compatible decode was too permissive.
  - Finding: any outcome-tail decode error could be downgraded to legacy YES.
  - Remediation: only missing/EOF legacy outcome tails default to YES; malformed tails return decode error.
  - Evidence: `command_0092` intentionally RED; `command_0093` passed after fix.

- P1 MarketDecisionTrace CAS writes were still best-effort in evaluator paths.
  - Finding: `write_market_decision_trace_to_cas` errors could be ignored, allowing stdout/tool_dist to claim no-trade evidence without CAS fossilization.
  - Remediation: new `write_market_decision_trace_to_cas_or_exit` helper opens CAS and writes the trace fail-closed; tool_dist no-trade counters are only incremented after successful CAS write in the relevant no-trade paths.
  - Evidence: `command_0092` intentionally RED; `command_0093` passed after fix.

## Clean-Context Review R3

Clean-context reviewer:

- File: `handover/audits/CODEX_REAL6A_IMPLEMENTATION_REVIEW_R3.md`
- Reviewer: Codex `gpt-5.5` / `xhigh`
- Verdict: `VETO`

Findings and remediation:

- P0 EventResolve compatibility still depended on serde error-string interpretation.
  - Finding: the reader could not reliably distinguish legitimate legacy 6-field EventResolve bytes from malformed current bytes by matching `UnexpectedEnd` text.
  - Remediation: `transition_ledger::canonical_decode::<TypedTx>` now special-cases `TypedTx` through an explicit current-or-legacy reader. The current reader must consume all bytes; the legacy reader accepts the old EventResolve shape only as YES. Partial or corrupt current outcome tails fail closed.
  - Evidence: `command_0109` intentionally RED; `command_0110` passed after exact dual-reader remediation; `command_0111` and `command_0113` kept EventResolve and legacy signature coverage green.

- P1 REAL-6A evaluator seed and NO-resolution paths could warn-and-continue under the feature flag.
  - Finding: a run with `TURINGOS_REAL6_TASK_OUTCOME_MARKET=1` could fail to seed or fail to emit NO and still continue with misleading smoke output.
  - Remediation: seed and NO-resolution failures now exit the evaluator fail-closed. A later solved-path probe also exposed YES-resolution warn-and-continue behavior; YES resolution is now fail-closed too when REAL-6A is enabled.
  - Evidence: `command_0109` intentionally RED; `command_0110` passed after seed/NO fail-closed remediation; `command_0120` passed after YES-resolution fail-closed remediation.

## Red-Green Evidence

EventResolve legacy compatibility:

- `command_0063`: intentionally RED legacy EventResolve decode test failed with `UnexpectedEnd { additional: 4 }`.
- `command_0064`: same test passed after manual compatibility decode.
- `command_0067`: sequencer legacy YES signature grandfathering test passed.
- `command_0068`: combined REAL-6A test + legacy signature test passed.
- `command_0069`: `cargo test --test constitution_n2_event_resolve` passed.
- `command_0081`: `cargo test --test constitution_n2_event_resolve` passed after final remediation.
- `command_0083`: `cargo test --lib state::sequencer::tests::event_resolve_legacy_yes_signature_is_grandfathered` passed after final remediation.
- `command_0092`: intentionally RED malformed-outcome-tail test proved corrupt EventResolve outcome bytes were still accepted.
- `command_0093`: same malformed-outcome-tail gate passed after fail-closed decode fix.
- `command_0097`: `cargo test --test constitution_n2_event_resolve` passed after R2 remediation.
- `command_0099`: legacy YES signature grandfathering test passed after R2 remediation.

Build fail-closed evidence:

- `command_0073`: invalid smoke r4 exposed the runner bug: source compile failed, but stale release binary still ran and produced `event_resolve: 0`.
- `command_0076`: intentionally RED runner gate proved `run_g_phase_batch.sh` did not fail closed on build failure.
- `command_0077`: same gate passed after `scripts/run_g_phase_batch.sh` was changed to exit 6 on release build failure before `batch_evaluator`.
- `command_0079`: r6 smoke used the fixed script and a successful fresh release build log.

MarketDecisionTrace CAS fail-closed evidence:

- `command_0092`: intentionally RED gate proved evaluator still ignored `write_market_decision_trace_to_cas` errors.
- `command_0093`: same gate passed after fail-closed CAS write helper replaced best-effort calls.

Shared AttemptTelemetry slot evidence:

- `command_0048`: shared-slot regression test passed for recognized MarketDecisionTrace JSON skip and unknown JSON fail-closed.
- `command_0049`: `audit_tape` over r2 evidence returned `verdict=PROCEED passed=39 failed=0 halted=0 skipped=13`.
- `command_0050`: `cargo test --test tb_18r_attempt_telemetry_serialize` passed.
- `command_0082`: `cargo test --test tb_18r_attempt_telemetry_serialize` passed after final remediation.
- `command_0098`: `cargo test --test tb_18r_attempt_telemetry_serialize` passed after R2 remediation.

## Smoke Evidence

Invalid / superseded smoke attempts:

- r3b: `handover/evidence/g_phase_real_6a_task_outcome_smoke_r3b_20260515T0307Z`
  - `audit_tape=PROCEED`, `invest_no_trade_no_perceived_edge:5`.
  - Superseded because `event_resolve: 0` and R1 reviewer VETO found SG-6A.7 missing in production.

- r4: `handover/evidence/g_phase_real_6a_task_outcome_smoke_r4_20260515T0338Z`
  - Not valid for ship. The run continued after a compile failure and used a stale release binary.

- r5: `handover/evidence/g_phase_real_6a_task_outcome_smoke_r5_20260515T0341Z`
  - Not valid for ship. The run failed preflight with `TRUST_ROOT_TAMPERED`.

- r6: `handover/evidence/g_phase_real_6a_task_outcome_smoke_r6_20260515T0344Z`
  - `audit_tape=PROCEED`, `event_resolve=1`.
  - Superseded by r7 because R2 changed evaluator CAS fail-closed behavior and trust-root bytes.

- r7: `handover/evidence/g_phase_real_6a_task_outcome_smoke_r7_20260515T0410Z`
  - `audit_tape=PROCEED`, `event_resolve=1`.
  - Superseded by R3 exact dual-reader and seed/NO fail-closed remediation.

- r8: `handover/evidence/g_phase_real_6a_task_outcome_smoke_r8_20260515T0431Z`
  - `audit_tape=PROCEED`, but the selected problem solved before exhaustion and produced `work=1`, `market_seed=2`, `cpmm_pool=2`, `event_resolve=0`.
  - Not used as final SG-6A.6/6A.7 evidence. It exposed that the solved path also needed YES-resolution fail-closed hardening.

- r9: `handover/evidence/g_phase_real_6a_task_outcome_smoke_r9_20260515T0436Z`
  - `audit_tape=PROCEED`, `event_resolve=1`, hard-problem NO path.
  - Superseded by r10 because YES-resolution fail-closed source hardening landed after r9.

Valid final post-R3-remediated smoke:

```bash
env \
  TURINGOS_G_PHASE_DIRTY_OK=1 \
  TURINGOS_G_PHASE_LOW_DISK_OK=1 \
  TURINGOS_G_PHASE_N_AGENTS=5 \
  TURINGOS_REAL5_ROLE_ASSIGNMENT=Solver,Trader,Verifier,Challenger,Observer \
  TURINGOS_REAL5_ROLE_VIEWS=1 \
  TURINGOS_REAL6_TASK_OUTCOME_MARKET=1 \
  TURINGOS_MARKET_ARENA_PROMPT=1 \
  MAX_TRANSACTIONS=5 \
  PER_PROBLEM_TIMEOUT_S=600 \
  ACTIVE_MODEL=deepseek-chat \
  bash scripts/run_g_phase_batch.sh \
  g_phase_real_6a_task_outcome_smoke_r10_20260515T0442Z \
  handover/evidence/real6_task_outcome/REAL6A_SMOKE_HARD_PROBLEMS.txt
```

Harness evidence:

- `command_0124`: exit 0.
- Run dir: `handover/evidence/g_phase_real_6a_task_outcome_smoke_r10_20260515T0442Z`
- `run_log.txt`: `batch_exit=0`, `audit_exit=0`, `audit_verdict=PROCEED`, `persistence_passing=true`, `persistence_n_witnessed=3`.
- `cargo_build_release.log`: fresh release build completed before batch launch.
- `aggregate_verdict.json`: `verdict=PROCEED`, `passed=41`, `failed=0`, `halted=0`, `skipped=11`.
- `PERSISTENCE_BINDING_REPORT.json`: `is_passing=true`, `n_witnessed=3`.
- `P000_numbertheory_2pownm1prime_nprime/evaluator.stdout`: `hit_max_tx=true`, `tool_dist` includes `invest_no_trade_no_perceived_edge:5`, `step_reject:1`, `real5_role_policy_violation:4`.

Smoke tx counts from `aggregate_verdict.json`:

```text
task_open: 1
escrow_lock: 1
market_seed: 1
cpmm_pool: 1
buy_with_coin_router: 0
work: 0
event_resolve: 1
terminal_summary: 1
```

Interpretation:

- TaskOutcomeMarket was seeded before accepted WorkTx activity.
- Trader-visible market turns produced classified NoTradeReason records, with MarketDecisionTrace CAS writes fail-closed before stdout/tool_dist claims.
- MaxTxExhausted produced a tape-visible EventResolve NO.
- No forced trade occurred.
- No E2/E3 market emergence claim is made.

## SG-6A Evidence Map

SG-6A.1 TaskOutcomeMarket exists before first WorkTx.

- Source gate: `tests/constitution_real6_task_outcome_market.rs`.
- Smoke evidence: r10 has `task_open=1`, `escrow_lock=1`, `market_seed=1`, `cpmm_pool=1`, `work=0`.

SG-6A.2 TraderView contains active TaskOutcomeMarket.

- Source gate: `tests/constitution_real6_task_outcome_market.rs`.
- Smoke evidence: `invest_no_trade_no_perceived_edge:5` requires market-visible classification rather than `NoPool`.

SG-6A.3 NoPool no longer dominates when task market exists.

- Smoke evidence: r10 `tool_dist` includes `invest_no_trade_no_perceived_edge:5`; no `invest_no_trade_no_pool` dominance.

SG-6A.4 Scripted trader can Buy YES/NO on TaskOutcomeMarket.

- Source fixture: `tests/constitution_real6_task_outcome_market.rs`.

SG-6A.5 Real LLM trader emits MarketDecisionTrace or classified NoTradeReason.

- Smoke evidence: r10 `P000_numbertheory_2pownm1prime_nprime/evaluator.stdout` has `invest_no_trade_no_perceived_edge:5`.
- Audit evidence: r10 `aggregate_verdict.json` is `PROCEED`.

SG-6A.6 EventResolveTx YES if verified proof before budget/deadline.

- Source fixture: `tests/constitution_real6_task_outcome_market.rs`.
- Compatibility gate: `tests/constitution_n2_event_resolve.rs`.

SG-6A.7 EventResolveTx NO if exhausted/deadline without verified proof.

- Source fixture: `tests/constitution_real6_task_outcome_market.rs`.
- Runtime smoke evidence: r10 has `hit_max_tx=true` and `event_resolve: 1`.

SG-6A.8 No ghost liquidity.

- Source fixture: `tests/constitution_real6_task_outcome_market.rs`.
- Audit evidence: r10 `aggregate_verdict.json` economy assertions passed.

SG-6A.9 CTF conserved.

- Audit evidence: r10 assertions `total_supply_conserved` and `total_supply_conserved_per_block` passed.

SG-6A.10 Price never affects Lean predicate.

- Source fixture: `tests/constitution_real6_task_outcome_market.rs`.
- Audit evidence: r10 accepted predicate assertions passed; price remained a view/signal.

## Verification Commands

Final post-remediation evidence:

- `command_0077`: runner build-fail-closed regression passed.
- `command_0078`: Trust Root verify passed after evaluator hash correction.
- `command_0079`: REAL-6A r6 smoke passed.
- `command_0080`: `cargo test --test constitution_real6_task_outcome_market` passed.
- `command_0081`: `cargo test --test constitution_n2_event_resolve` passed.
- `command_0082`: `cargo test --test tb_18r_attempt_telemetry_serialize` passed.
- `command_0083`: `cargo test --lib state::sequencer::tests::event_resolve_legacy_yes_signature_is_grandfathered` passed.
- `command_0085`: `cargo fmt --all` passed.
- `command_0086`: `cargo fmt --all -- --check` passed.
- `command_0087`: Trust Root verify passed after formatting.
- `command_0088`: `bash scripts/run_constitution_gates.sh` passed, `436 passed / 0 failed / 1 ignored`.
- `command_0089`: `cargo test --workspace --no-fail-fast -- --test-threads=1` exited 0.
- `command_0091`: invalid cargo-test invocation; not used as evidence.
- `command_0092`: R2 VETO regression gates intentionally RED.
- `command_0093`: R2 VETO regression gates passed.
- `command_0097`: `cargo test --test constitution_n2_event_resolve` passed after R2 remediation.
- `command_0098`: `cargo test --test tb_18r_attempt_telemetry_serialize` passed after R2 remediation.
- `command_0099`: legacy YES signature grandfathering passed after R2 remediation.
- `command_0100`: fmt check RED; fixed by `command_0101`.
- `command_0102`: `cargo fmt --all -- --check` passed after R2 remediation.
- `command_0103`: Trust Root RED after evaluator hash changed; fixed by rehashing evaluator/typed_tx current bytes.
- `command_0104`: Trust Root verify passed after R2 remediation.
- `command_0105`: REAL-6A r7 smoke passed after R2 remediation.
- `command_0106`: `bash scripts/run_constitution_gates.sh` passed, `436 passed / 0 failed / 1 ignored`.
- `command_0107`: `cargo test --workspace --no-fail-fast -- --test-threads=1` exited 0.
- `command_0109`: R3 VETO regression gates intentionally RED for exact dual-reader and seed/NO fail-closed checks.
- `command_0110`: `cargo test --test constitution_real6_task_outcome_market` passed after R3 remediation.
- `command_0111`: `cargo test --test constitution_n2_event_resolve` passed after R3 remediation.
- `command_0112`: `cargo test --test tb_18r_attempt_telemetry_serialize` passed after R3 remediation.
- `command_0113`: legacy YES signature grandfathering passed after R3 remediation.
- `command_0116`: `cargo fmt --all -- --check` passed after exact dual-reader formatting.
- `command_0117`: Trust Root verify passed after R3 source changes.
- `command_0118`: REAL-6A r8 solved-path smoke passed but is not final SG-6A.6/SG-6A.7 evidence; it exposed missing YES-resolution fail-closed hardening.
- `command_0119`: REAL-6A r9 hard-problem NO smoke passed before YES-resolution fail-closed source hardening.
- `command_0120`: `cargo test --test constitution_real6_task_outcome_market` passed after YES-resolution fail-closed remediation.
- `command_0121`: `cargo test --test constitution_n2_event_resolve` passed after YES-resolution fail-closed remediation.
- `command_0122`: `cargo fmt --all -- --check` passed after YES-resolution fail-closed remediation.
- `command_0123`: Trust Root verify passed after final evaluator hash correction.
- `command_0124`: REAL-6A r10 hard-problem NO smoke passed after final source changes.
- `command_0125`: `bash scripts/run_constitution_gates.sh` passed, `436 passed / 0 failed / 1 ignored`.
- `command_0126`: `cargo test --workspace --no-fail-fast -- --test-threads=1` exited 0.

## Trust Root

Trust Root context remains documented in:

- `handover/evidence/real6_task_outcome/TRUST_ROOT_REHASH_REAL6A_WORKSPACE_NORMALIZATION.md`
- `handover/evidence/real6_task_outcome/REAL6A_RESTRICTED_SURFACE_AUDIT_NOTE.md`

Final Trust Root verification:

- `command_0123`: `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo` passed.
