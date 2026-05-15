# REAL-6A Restricted Surface Audit Note

Date: 2026-05-15

Harness run: `dev_1778811250681_913405`

Related review:

- `handover/audits/CODEX_REAL6A_IMPLEMENTATION_REVIEW.md`

## Why This Note Exists

The R1 clean-context Codex review returned `VETO` with a P1 finding:

```text
Trust Root normalization promotes broad dirty pinned files outside the REAL-6A semantic surface.
Passing Trust Root verify proves current bytes match the manifest; it does not prove those extra authority bytes were semantically audited for this atom.
```

That finding is accepted. The broad rehash is treated as dirty-worktree
normalization, not as semantic proof for every pinned file.

## REAL-6A Semantic Review Surface

The REAL-6A semantic claim is limited to TaskOutcomeMarket timing and lawful
market visibility. The files that require REAL-6A semantic review are:

- `src/runtime/real6_task_outcome.rs`
- `src/runtime/adapter.rs`
- `src/runtime/mod.rs`
- `src/state/typed_tx.rs`
- `src/state/sequencer.rs`
- `src/runtime/attempt_telemetry.rs`
- `src/runtime/audit_assertions.rs`
- `src/runtime/chain_derived_run_facts.rs`
- `src/bin/audit_dashboard.rs`
- `experiments/minif2f_v4/src/bin/evaluator.rs`
- `experiments/minif2f_v4/src/drive_task.rs`
- `scripts/run_g_phase_batch.sh`
- `tests/constitution_real6_task_outcome_market.rs`
- `tests/constitution_n2_event_resolve.rs`
- `tests/tb_18r_attempt_telemetry_serialize.rs`

Restricted / Class-4-adjacent files in that list:

- `src/state/typed_tx.rs`
- `src/state/sequencer.rs`
- `src/bottom_white/cas/schema.rs` was not semantically changed for REAL-6A, but remains a restricted surface in the dirty worktree and must not be treated as REAL-6A evidence.
- `src/kernel.rs`, `src/bus.rs`, and `src/sdk/tools/wallet.rs` were part of the broader dirty pinned set, but are not REAL-6A semantic changes.

## Audit Boundary

The implementation reviewer should inspect the semantic surface above for:

- EventResolve wire compatibility and signing compatibility.
- YES/NO resolution authority.
- Exhaustion/deadline NO-resolution emission.
- Feature-flagged seed, YES-resolution, and NO-resolution fail-closed behavior.
- No price-as-truth.
- No ghost liquidity / CTF conservation.
- Shared AttemptTelemetry slot fail-closed behavior.
- Smoke runner fail-closed release build behavior.

The reviewer should not interpret the broad Trust Root rehash as evidence that
all dirty pinned files were designed, changed, or semantically approved under
REAL-6A.

## Evidence

Final verification after remediation:

- `command_0077`: runner build-fail-closed regression passed.
- `command_0092`: R2 VETO regression gates intentionally failed before the final fix.
- `command_0093`: `cargo test --test constitution_real6_task_outcome_market` passed after EventResolve corrupt-tail and MarketDecisionTrace fail-closed fixes.
- `command_0097`: `cargo test --test constitution_n2_event_resolve` passed.
- `command_0098`: `cargo test --test tb_18r_attempt_telemetry_serialize` passed.
- `command_0099`: EventResolve legacy YES signature grandfathering passed.
- `command_0104`: Trust Root verify passed.
- `command_0105`: valid REAL-6A r7 smoke passed.
- `command_0106`: constitution gates passed.
- `command_0107`: workspace tests exited 0.
- `command_0109`: R3 VETO regression gates intentionally failed before exact dual-reader and seed/NO fail-closed fixes.
- `command_0110`: REAL-6A gate passed after R3 remediation.
- `command_0111`: EventResolve gate passed after R3 remediation.
- `command_0113`: legacy YES signature grandfathering passed after R3 remediation.
- `command_0117`: Trust Root verify passed after R3 remediation.
- `command_0118`: solved-path smoke exposed missing YES-resolution fail-closed hardening and is not final SG-6A evidence.
- `command_0120`: REAL-6A gate passed after YES-resolution fail-closed remediation.
- `command_0123`: final Trust Root verify passed.
- `command_0124`: final REAL-6A r10 hard-problem smoke passed.
- `command_0125`: final constitution gates passed.
- `command_0126`: final workspace tests exited 0.
