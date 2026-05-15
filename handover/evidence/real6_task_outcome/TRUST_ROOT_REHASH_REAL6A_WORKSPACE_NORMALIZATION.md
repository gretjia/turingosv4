# REAL-6A Trust Root Rehash / Workspace Normalization

Date: 2026-05-15
Harness run: `dev_1778811250681_913405`

## Why This File Exists

During REAL-6A verification, `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo` failed because the shared worktree already contained a broad set of modified Trust-Root-pinned files. A prior `cargo fmt --all` also normalized Rust formatting across existing dirty files, changing bytes outside the narrow REAL-6A semantic surface.

To avoid reverting user or pre-existing worktree changes, this normalization rehashed the current bytes for all modified pinned files in `genesis_payload.toml`. This is a Trust Root authority update and must remain visible to the Class-4 audit.

## REAL-6A Semantic Surface

REAL-6A TaskOutcomeMarket intentionally touched these authority or runtime surfaces:

- `experiments/minif2f_v4/src/bin/evaluator.rs`
- `experiments/minif2f_v4/src/drive_task.rs`
- `src/runtime/adapter.rs`
- `src/runtime/mod.rs`
- `src/runtime/real6_task_outcome.rs`
- `src/state/sequencer.rs`
- `src/state/typed_tx.rs`
- `tests/constitution_real6_task_outcome_market.rs`
- `tests/constitution_n2_event_resolve.rs`

Pinned files from that set rehashed in `genesis_payload.toml`:

- `experiments/minif2f_v4/src/bin/evaluator.rs`
- `src/runtime/adapter.rs`
- `src/runtime/mod.rs`
- `src/state/sequencer.rs`
- `src/state/typed_tx.rs`

## Additional Modified Pinned Files Rehashed

These were already dirty and/or byte-normalized by workspace formatting. They were rehashed to preserve current worktree bytes without rollback:

- `experiments/minif2f_v4/src/budget_regime.rs`
- `experiments/minif2f_v4/src/cost_aggregator.rs`
- `experiments/minif2f_v4/src/experiment_mode.rs`
- `experiments/minif2f_v4/src/fc_trace.rs`
- `experiments/minif2f_v4/src/h_vppu_history.rs`
- `experiments/minif2f_v4/src/jsonl_schema.rs`
- `experiments/minif2f_v4/src/lean4_oracle.rs`
- `experiments/minif2f_v4/src/post_hoc_verifier.rs`
- `experiments/minif2f_v4/src/run_id.rs`
- `experiments/minif2f_v4/src/wall_clock.rs`
- `experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs`
- `src/boot.rs`
- `src/bottom_white/cas/schema.rs`
- `src/bottom_white/cas/store.rs`
- `src/bottom_white/ledger/rejection_evidence.rs`
- `src/bottom_white/ledger/system_keypair.rs`
- `src/bottom_white/ledger/transition_ledger.rs`
- `src/bottom_white/tools/registry.rs`
- `src/bus.rs`
- `src/drivers/llm_http.rs`
- `src/economy/escrow_vault.rs`
- `src/economy/ledger.rs`
- `src/economy/monetary_invariant.rs`
- `src/economy/money.rs`
- `src/kernel.rs`
- `src/lib.rs`
- `src/runtime/agent_audit_trail.rs`
- `src/runtime/agent_keypairs.rs`
- `src/runtime/audit_assertions.rs`
- `src/runtime/bootstrap.rs`
- `src/runtime/chain_derived_run_facts.rs`
- `src/runtime/evidence_capsule.rs`
- `src/runtime/proposal_telemetry.rs`
- `src/runtime/run_summary.rs`
- `src/runtime/verify.rs`
- `src/sdk/prompt_guard.rs`
- `src/state/mod.rs`
- `src/state/price_index.rs`
- `src/state/q_state.rs`
- `src/top_white/predicates/registry.rs`
- `src/top_white/predicates/visibility.rs`
- `src/wal.rs`
- `tests/conformance_stubs.rs`
- `tests/fc_alignment_conformance.rs`
- `tests/tb_6_agent_audit_trail.rs`
- `tests/tb_6_run_summary.rs`
- `tests/tb_6_verify_chaintape.rs`
- `tests/tb_7_atom6_chain_backed_smoke.rs`
- `tests/tb_7_authoritative_routing.rs`
- `tests/tb_7_legacy_append_regression.rs`
- `tests/walkthrough_inv3_conservation.rs`

## Verification

Post-normalization Trust Root command:

```bash
cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo
```

Harness evidence:

- initial failing command: `artifacts/command_0024_stdout.txt`
- passing rerun: `artifacts/command_0025_stdout.txt`
- post-VETO evaluator hash correction passing rerun: `artifacts/command_0078_stdout.txt`
- final post-format passing rerun: `artifacts/command_0087_stdout.txt`
- post-R2-VETO evaluator/typed_tx hash correction passing rerun: `artifacts/command_0104_stdout.txt`
- post-R3 exact-dual-reader / seed-NO fail-closed passing rerun: `artifacts/command_0117_stdout.txt`
- final post-YES-resolution fail-closed evaluator hash correction passing rerun: `artifacts/command_0123_stdout.txt`

## Audit Note

The Class-4 reviewer must treat this as a broad Trust Root normalization in a dirty shared worktree, not as evidence that every listed non-REAL-6A file received semantic review in this atom. The ship claim for REAL-6A remains limited to TaskOutcomeMarket timing and lawful market visibility.

The narrowed semantic boundary for implementation review is recorded in:

- `handover/evidence/real6_task_outcome/REAL6A_RESTRICTED_SURFACE_AUDIT_NOTE.md`
