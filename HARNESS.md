# TuringOS Project Harness — Constitutional Harness Engineering

## 0. Purpose

The harness exists to make the constitution executable.

The goal is not to maximize tests.
The goal is to ensure no code path can claim TuringOS validity without satisfying:

- Flowchart 1: runtime loop
- Flowchart 2: boot / genesis
- Flowchart 3: meta / Markov

This harness replaces the slow pattern:

```
plan -> atom -> audit -> discover violation
```

with:

```
constitution gate -> real run -> debug -> fix
```

***

## 1. Harness layers

### Layer H0 — Source-of-truth matrix

Files:

- `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`
- `handover/alignment/TRACE_FLOWCHART_MATRIX.md`

Each row:

- clause_id
- constitution text / flowchart node
- code surface
- test name
- real witness
- status
- kill condition

A clause with only text coverage is `NOT-LANDED`.

### Layer H1 — Static constitution gates

Targets:

- no `f64` money path
- no global Markov pointer
- no legacy authoritative append
- no system tx agent ingress
- no shadow/canonical ID mix
- no dashboard source-of-truth
- no memory-only preseed

Command:

```
bash scripts/run_constitution_gates.sh
```

### Layer H2 — Parser / manifest gates

Targets:

- `genesis_report`
- `PromptCapsule`
- `AttemptTelemetry`
- `LeanResult`
- `EvidenceCapsule`
- `MarkovEvidenceCapsule`
- `HEAD_t`
- `BenchmarkManifest`
- `EvidencePackagingPolicy`

### Layer H3 — Real-run gates

Targets:

- P38
- P49
- M0 mini-batch

Invariants:

```
evaluator_reported_completed_llm_calls
=
  l4_work_attempt_count
+ l4e_work_attempt_count
+ capsule_anchored_attempt_count
```

### Layer H4 — Replay gates

Targets:

- replay from `genesis_report` + ChainTape + CAS + agent registry + system pubkeys
- dashboard regeneration
- `economic_state` reconstruction
- `HEAD_t` reconstruction

### Layer H5 — Audit gates

Run only after H1–H4 pass.

- Codex implementation audit
- Gemini architecture audit if available
- VETO > CHALLENGE > PASS

***

## 2. Required persistent tests

### FC1 Runtime

- `fc1_every_externalized_attempt_is_tape_visible`
- `fc1_predicate_pass_goes_l4`
- `fc1_predicate_fail_goes_l4e`
- `fc1_no_legacy_authoritative_append`
- `fc1_attempt_count_equals_tape_count`
- `fc1_no_fake_accepted_nodes`
- `fc1_dashboard_not_source_of_truth`

### FC2 Boot

- `fc2_genesis_report_exists`
- `fc2_run_replayable_from_genesis_tape_cas`
- `fc2_no_memory_only_preseed`
- `fc2_on_init_only_mint`
- `fc2_taskopen_escrowlock_are_chain_events`
- `fc2_system_pubkeys_verify`
- `fc2_agent_registry_resolves`

### FC3 Meta

- `fc3_capsule_derived_from_tape_cas`
- `fc3_no_global_markov_pointer`
- `fc3_latest_capsule_context_only`
- `fc3_deep_history_requires_override`
- `fc3_raw_logs_not_in_agent_read_view`
- `fc3_no_automatic_predicate_mutation`

### Predicate

- `predicate_result_is_binary`
- `predicate_failure_cannot_enter_l4`
- `predicate_pass_required_for_l4`
- `lean_verified_required_for_verified_worktx`
- `price_never_overrides_predicate`

### Shielding

- `raw_lean_stderr_not_in_agent_read_view`
- `private_diagnostic_cid_not_serialized_publicly`
- `evidence_capsule_raw_logs_audit_only`
- `prompt_capsule_redacts_hidden_fields`
- `dashboard_does_not_leak_private_failure_detail`

### Economy

- `economy_read_is_free`
- `economy_write_requires_stake_or_escrow`
- `economy_no_post_init_mint`
- `economy_total_coin_conserved`
- `economy_complete_set_yes_no_not_coin`
- `economy_no_ghost_liquidity`
- `economy_wallet_read_only_projection`
- `economy_no_f64_money_path`
- `system_tx_not_agent_submittable`

### Tape canonical

- `no_parallel_ledger_source_of_truth`
- `no_shadow_tape_authoritative_parent`
- `canonical_txid_not_shadow_id`
- `dashboard_regenerates_from_tape_cas`
- `chain_derived_facts_not_evaluator_stdout`
- `all_externalized_attempts_have_cas_payload`
- `all_lean_results_have_cas_payload`

***

## 3. Strategic blocker harness

### G-009 HEAD_t

Immediate C1 witness:

```
HEAD_t {
  state_root,
  l4_head,
  l4e_head,
  cas_root,
  economic_state_root,
  run_id
}
```

Tests:

- `head_t_advances_on_l4`
- `head_t_does_not_advance_on_l4e_only`
- `head_t_reconstructs_from_replay`
- `dashboard_reads_head_t_derived_state`

Later C2:

- libgit2 refs for L4 / L4.E / CAS

### G-012 PCP soundness

Corpus:

- valid proofs
- mutated invalid proofs
- sorry insertion
- type mismatch
- wrong theorem
- off-by-one arithmetic
- irrelevant theorem
- partial tactic accepted but final invalid
- parse-invalid output

Tests:

- `pcp_valid_passes`
- `pcp_mutated_invalid_fails`
- `pcp_sorry_blocked`
- `pcp_invalid_never_l4`
- `pcp_invalid_routes_l4e_or_capsule`

### G-016 / G-019 / G-021 / G-028 Prompt persistence

Default:

- `PromptCapsule` + CAS + L4 / L4.E anchor

Tests:

- `prompt_capsule_created_for_attempt`
- `prompt_capsule_hash_stable`
- `prompt_capsule_redacts_hidden_fields`
- `prompt_capsule_referenced_by_attempt_telemetry`
- `verbatim_prompt_not_public_by_default`

***

## 4. Runner policy

Before any real run:

```
/runner-preflight
```

Then:

- smoke 1 problem
- batch P38 / P49 / M0
- constitution gates
- evidence report

Never run large benchmark before:

- P38 / P49 attempt equality green
- M0 green
- constitution gates green
- HEAD_t C1 green
- PromptCapsule green
- PCP synthetic corpus green

***

## 5. Evidence package

Every evidence run must include:

- `genesis_report.json`
- `runtime_repo/`
- `cas/`
- `agent_registry.json`
- `system_pubkeys.json`
- `chain_derived_run_facts.json`
- `attempt_count_equality_report.json`
- `audit_tape_report.json`
- `dashboard_report.json`
- `constitution_gate_report.json`
- `README.md`

For benchmarks:

- `BenchmarkManifest.json`
- `EvidencePackagingPolicy.md`
- `sampled_replay_manifest.json`

***

## 6. Kill gates

Stop immediately if:

- attempt equality mismatch
- fake accepted node
- predicate failure in L4
- Lean reject only in stdout
- dashboard requires stdout for core facts
- memory-only preseed
- global Markov pointer
- CTF conservation failure
- `f64` money path
- system tx accepted from agent ingress
- shadow / canonical ID mismatch

***

## 7. Audit policy

External audit is after evidence.

- H1–H4 green -> audit
- H1–H4 red -> fix, no audit

Class 3/4 final audit:

- Codex + Gemini if available
- VETO > CHALLENGE > PASS

A VETO fix must have remediation directive.

***

## 8. Loop mode

AI coder may run autonomously only inside these constraints:

- Class 0/1: autonomous to completion
- Class 2: autonomous until evidence gate
- Class 3: autonomous until pre-ship audit
- Class 4: stop before code unless explicit ratification

Any constitution gate failure terminates loop mode.
