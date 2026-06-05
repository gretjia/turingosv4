# A12 Universal Machine Witnesses Preflight

Date: 2026-06-05

Parent plan:
`handover/directives/2026-06-05_AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT.md`

Atom: A12. Universal Machine Witnesses

Document role: Class 0 preflight. This document does not authorize runtime
authority, sequencer admission, typed transaction, predicate authority, economy
settlement, or FC3 ArchitectAI feedback-consumer changes by itself.

## Decision

A12 is a witness suite, not the OS substrate itself. It must not turn existing
narrow flowchart probes into a broad universal-machine claim. Every planned
A12 file is missing, and the parent plan correctly makes A12 depend on A04-A11.

Current repo witnesses are useful but partial:

- FC2 boot/replay/resume has live tests.
- Offline replay has no-network and cross-CID tests.
- Agent-view shielding has hidden-oracle tests.
- Market and economy paths have older focused witnesses.
- FC3 feedback/re-init remains forward-bound; self-bootstrap is proposal-only
  unless a later atom explicitly adds runtime authority.

Safe work now:

- docs-only preflight
- current witness inventory
- explicit non-closure warning for FC3
- atom split and acceptance-command correction

Blocked until predecessors exist:

- claiming universal witness coverage over A04-A11 behavior before those atoms
  exist
- claiming full FC1/FC2/FC3 liveness from existing narrow probes
- treating proposal-only ArchitectAI feedback as runtime self-modification
- making witness code mutate sequencer, typed tx schema, or boot authority

## Current-State Facts

Parent-plan A12 allowed paths:

```text
src/runtime/tc_universal_witness.rs
tests/tc_universal_witness_counter_machine.rs
tests/tc_universal_witness_branching.rs
tests/tc_universal_witness_external_call.rs
tests/tc_universal_witness_market.rs
tests/tc_universal_witness_self_bootstrap.rs
```

Corrected implementation path inventory:

```text
src/runtime/mod.rs
src/runtime/tc_universal_witness.rs
src/runtime/verify.rs
src/runtime/replay.rs
src/runtime/agent_audit_trail.rs
tests/tc_universal_witness_counter_machine.rs
tests/tc_universal_witness_branching.rs
tests/tc_universal_witness_external_call.rs
tests/tc_universal_witness_market.rs
tests/tc_universal_witness_agent_view.rs
tests/tc_universal_witness_self_bootstrap.rs
tests/constitution_flowchart_livenow.rs
tests/offline_replay_no_llm_dependency_static_check.rs
tests/replay_verifies_all_cross_cid_references_resolve.rs
tests/hidden_oracle_not_in_generation_prompt_bytes.rs
tests/hidden_oracle_set_cid_not_in_build_session_view.rs
tests/constitution_shielding_gate.rs
tests/build_session_view_does_not_expose_private_diagnostic_cid.rs
tests/artifact_bundle_serve_rejects_private_diagnostic_cid.rs
tests/rejection_private_diagnostic_not_in_http_body.rs
tests/constitution_fc3_meta.rs
tests/constitution_fc3_evidence_binding.rs
tests/tape_relay_feedback_loop.rs
handover/alignment/TRACE_FLOWCHART_MATRIX.md
```

Write-scope guidance:

```text
src/runtime/mod.rs
  needed only if tc_universal_witness.rs must be crate-visible
src/runtime/verify.rs
src/runtime/replay.rs
src/runtime/agent_audit_trail.rs
  read-only precedents unless a later implementation atom explicitly requires
  a witness hook
src/state/sequencer.rs
src/state/typed_tx.rs
src/bottom_white/cas/schema.rs
src/kernel.rs
src/bus.rs
  out of A12 write scope without explicit higher-risk ratification
```

Existence check:

```text
MISSING src/runtime/tc_universal_witness.rs
MISSING tests/tc_universal_witness_counter_machine.rs
MISSING tests/tc_universal_witness_branching.rs
MISSING tests/tc_universal_witness_external_call.rs
MISSING tests/tc_universal_witness_market.rs
MISSING tests/tc_universal_witness_agent_view.rs
MISSING tests/tc_universal_witness_self_bootstrap.rs
EXISTS src/runtime/mod.rs
EXISTS src/runtime/verify.rs
EXISTS src/runtime/replay.rs
EXISTS src/runtime/agent_audit_trail.rs
EXISTS tests/constitution_flowchart_livenow.rs
EXISTS tests/offline_replay_no_llm_dependency_static_check.rs
EXISTS tests/replay_verifies_all_cross_cid_references_resolve.rs
EXISTS tests/hidden_oracle_not_in_generation_prompt_bytes.rs
EXISTS tests/hidden_oracle_set_cid_not_in_build_session_view.rs
EXISTS tests/constitution_shielding_gate.rs
EXISTS tests/build_session_view_does_not_expose_private_diagnostic_cid.rs
EXISTS tests/artifact_bundle_serve_rejects_private_diagnostic_cid.rs
EXISTS tests/rejection_private_diagnostic_not_in_http_body.rs
EXISTS tests/constitution_fc3_meta.rs
EXISTS tests/constitution_fc3_evidence_binding.rs
EXISTS tests/tape_relay_feedback_loop.rs
EXISTS handover/alignment/TRACE_FLOWCHART_MATRIX.md
```

Dirty-path check for relevant inventory:

```text
pre-existing dirty paths include:
  src/runtime/mod.rs
  src/state/sequencer.rs
  src/state/typed_tx.rs

Implementation must read and preserve those edits. Do not overwrite them as
part of A12 scaffolding.
```

Existing witness facts:

```text
tests/constitution_flowchart_livenow.rs:1
  explicitly says it does not claim full FC1/FC2/FC3 liveness
tests/constitution_flowchart_livenow.rs:5
  exercises FC1 typed sequencer wtool to L4 and L4.E
tests/constitution_flowchart_livenow.rs:7
  exercises FC2 boot, replay verification, and resume bootstrap
tests/constitution_flowchart_livenow.rs:275
  fc2_boot_replay_and_resume_are_live
tests/constitution_flowchart_livenow.rs:327
  fc2_map_reduce_tick_is_tape_visible_and_replay_verified
tests/offline_replay_no_llm_dependency_static_check.rs:1
  offline replay modules must not import LLM/network clients
tests/offline_replay_no_llm_dependency_static_check.rs:42
  no-network static replay check
tests/replay_verifies_all_cross_cid_references_resolve.rs:1
  offline replay verifies cross-CID references resolve in CAS
tests/replay_verifies_all_cross_cid_references_resolve.rs:103
  clean chain replay has no dangling references
tests/hidden_oracle_not_in_generation_prompt_bytes.rs:1
  hidden oracle bytes must not appear in generation prompt bytes
tests/hidden_oracle_set_cid_not_in_build_session_view.rs:1
  hidden scenario-set CID must not appear in BuildSessionView
tests/hidden_oracle_set_cid_not_in_build_session_view.rs:63
  sequencer must not wire C11 BuildStatus into admission
tests/constitution_shielding_gate.rs:1
  shielding gate covers raw stderr, public summaries, private diagnostics, and
  dashboard leakage
tests/tape_relay_feedback_loop.rs:1
  prior rejection feedback is relayed from CAS tape
tests/constitution_fc3_meta.rs:1
  FC3 structural meta tests cover capsule/context-only/proposal-only/veto-only
tests/constitution_fc3_evidence_binding.rs:1
  FC3 evidence binding ties structural rows to real evidence fixtures
handover/alignment/TRACE_FLOWCHART_MATRIX.md:137
  logs -> feedback -> ArchitectAI is still missing
handover/alignment/TRACE_FLOWCHART_MATRIX.md:138
  error -> re-init -> boot is still missing
src/runtime/agent_audit_trail.rs:1
  Agent audit trail records what an agent saw, submitted, and how it was judged
src/runtime/agent_audit_trail.rs:39
  audit records are diagnostic-only; replay still reconstructs QState from L4
```

Non-closure facts:

```text
No current src/runtime/tc_universal_witness.rs.
No current tests/tc_universal_witness_*.
Parent plan originally named W5 agent-view shielding but omitted the dedicated
tests/tc_universal_witness_agent_view.rs path and acceptance command.
Existing flowchart liveness tests are intentionally narrow.
FC3 logs -> feedback -> ArchitectAI remains forward-bound.
FC3 error -> re-init -> boot remains forward-bound unless A06/A13 add it.
```

## Risk Classification

Risk floor: Class 2 for witness-only tests and a non-authoritative witness
helper.

Promote to Class 3 if:

- witness code emits real ChainTape rows
- witness code exercises real economy settlement
- witness code depends on external provider/network behavior
- witness output is consumed as audit/ship evidence for Class 3 atoms

Promote to Class 4 if:

- sequencer admission changes
- typed tx schema or discriminants change
- predicate authority changes
- boot/trust-root authority changes
- FC3 ArchitectAI feedback consumer changes runtime authority
- constitution / flowchart authority changes

## Recommended Contract

A12 should expose witness results, not new authority:

```text
UniversalWitnessRun {
  witness_id: String,
  witness_kind: UniversalWitnessKind,
  input_tape_head: GitOid,
  input_cas_root: Option<GitOid>,
  fixture_cid: Option<Cid>,
  network_policy: "off",
  expected_outcome: WitnessExpectation,
  replay_report_cid: Option<Cid>,
  result: WitnessResult,
}

UniversalWitnessKind =
  CounterMachine
  BranchAndReject
  ExternalCallReplay
  MarketSettlement
  AgentViewShielding
  SelfBootstrapProposalOnly
```

Required invariant:

```text
witness replay reads only tape prefix + CAS.
network_policy must be off for replay.
tampering input tape/CAS must fail the witness.
rejected transitions remain on L4.E or equivalent rejected tape.
self-bootstrap witness emits proposal-only evidence unless FC3 runtime authority
exists and is separately ratified.
```

## Atomized A12 Tasks

### A12.0 Preflight Lock

Description:
Record missing A12 files, existing partial witnesses, predecessor dependencies,
and FC3 non-closure boundaries.

Acceptance:

```bash
for f in \
  handover/directives/2026-06-05_A12_UNIVERSAL_MACHINE_WITNESSES_PREFLIGHT.md \
  handover/directives/2026-06-05_AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT.md
do
  git diff --no-index --check /dev/null "$f" || true
done
```

Expected:

```text
no whitespace errors
```

### A12.1 Counter Machine And Branch/Reject Witnesses

Description:
Add deterministic counter-machine and branch/reject tests after A04/A05/A08
make tape events and predicate receipts available.

Primary paths:

```text
src/runtime/mod.rs
src/runtime/tc_universal_witness.rs
tests/tc_universal_witness_counter_machine.rs
tests/tc_universal_witness_branching.rs
tests/constitution_flowchart_livenow.rs
```

Acceptance:

```bash
cargo test --test tc_universal_witness_counter_machine --no-fail-fast -- --test-threads=1
cargo test --test tc_universal_witness_branching --no-fail-fast -- --test-threads=1
cargo test --test constitution_flowchart_livenow --no-fail-fast -- --test-threads=1
```

Expected:

```text
accepted and rejected branches are reconstructable.
tamper-positive-control fails.
```

### A12.2 External-Call Replay Witness

Description:
After A06 exists, prove replay closes external call intent/terminal state
without calling provider/network.

Primary paths:

```text
tests/tc_universal_witness_external_call.rs
tests/offline_replay_no_llm_dependency_static_check.rs
tests/replay_verifies_all_cross_cid_references_resolve.rs
```

Acceptance:

```bash
cargo test --test tc_universal_witness_external_call --no-fail-fast -- --test-threads=1
cargo test --test offline_replay_no_llm_dependency_static_check --no-fail-fast
cargo test --test replay_verifies_all_cross_cid_references_resolve --no-fail-fast
```

Expected:

```text
no network during replay.
all external-call evidence resolves from tape/CAS.
```

### A12.3 Market And Agent-View Witnesses

Description:
After A07/A09/A10/A11 exist, prove market settlement and agent view shielding
through witness tests.

Primary paths:

```text
tests/tc_universal_witness_market.rs
tests/tc_universal_witness_agent_view.rs
tests/hidden_oracle_not_in_generation_prompt_bytes.rs
tests/hidden_oracle_set_cid_not_in_build_session_view.rs
tests/constitution_shielding_gate.rs
tests/build_session_view_does_not_expose_private_diagnostic_cid.rs
tests/artifact_bundle_serve_rejects_private_diagnostic_cid.rs
tests/rejection_private_diagnostic_not_in_http_body.rs
```

Acceptance:

```bash
cargo test --test tc_universal_witness_market --no-fail-fast -- --test-threads=1
cargo test --test tc_universal_witness_agent_view --no-fail-fast -- --test-threads=1
cargo test --test hidden_oracle_not_in_generation_prompt_bytes --no-fail-fast
cargo test --test hidden_oracle_set_cid_not_in_build_session_view --no-fail-fast
cargo test --features web --test build_session_view_does_not_expose_private_diagnostic_cid --no-fail-fast
cargo test --features web --test artifact_bundle_serve_rejects_private_diagnostic_cid --no-fail-fast
cargo test --features web --test rejection_private_diagnostic_not_in_http_body --no-fail-fast
```

Expected:

```text
settlement follows PredicateReceipt/economy rules.
private oracle/view data is not leaked into agent-visible prompts.
```

### A12.4 Self-Bootstrap Proposal-Only Witness

Description:
Record a proposal-only ArchitectAI/self-improvement witness without claiming
runtime self-modification or FC3 closure.

Primary paths:

```text
tests/tc_universal_witness_self_bootstrap.rs
tests/tape_relay_feedback_loop.rs
tests/constitution_fc3_meta.rs
tests/constitution_fc3_evidence_binding.rs
src/runtime/agent_audit_trail.rs
handover/alignment/TRACE_FLOWCHART_MATRIX.md
```

Acceptance:

```bash
cargo test --test tc_universal_witness_self_bootstrap --no-fail-fast -- --test-threads=1
cargo test --test tape_relay_feedback_loop --no-fail-fast
cargo test --test constitution_fc3_meta --no-fail-fast
cargo test --test constitution_fc3_evidence_binding --no-fail-fast
```

Expected:

```text
self-bootstrap output is proposal-only.
no runtime authority changes.
no full FC3 closure claim.
```

## Full A12 Acceptance

After A04-A11 exist and A12 implementation is complete:

```bash
cargo test --test tc_universal_witness_counter_machine --no-fail-fast -- --test-threads=1
cargo test --test tc_universal_witness_branching --no-fail-fast -- --test-threads=1
cargo test --test tc_universal_witness_external_call --no-fail-fast -- --test-threads=1
cargo test --test tc_universal_witness_market --no-fail-fast -- --test-threads=1
cargo test --test tc_universal_witness_agent_view --no-fail-fast -- --test-threads=1
cargo test --test tc_universal_witness_self_bootstrap --no-fail-fast -- --test-threads=1
cargo test --test constitution_flowchart_livenow --no-fail-fast -- --test-threads=1
cargo test --test offline_replay_no_llm_dependency_static_check --no-fail-fast
cargo test --test replay_verifies_all_cross_cid_references_resolve --no-fail-fast
cargo test --features web --test build_session_view_does_not_expose_private_diagnostic_cid --no-fail-fast
cargo test --features web --test artifact_bundle_serve_rejects_private_diagnostic_cid --no-fail-fast
cargo test --features web --test rejection_private_diagnostic_not_in_http_body --no-fail-fast
cargo test --test constitution_fc3_meta --no-fail-fast
cargo test --test constitution_fc3_evidence_binding --no-fail-fast
bash scripts/run_constitution_gates.sh
git diff --check
```

Expected:

```text
PREDICATES-GREEN
replay byte-identical.
tamper test fails.
no network during replay.
accepted transitions have PredicateReceipt PASS where predicate authority is
part of the witness.
rejected transitions remain on rejected tape.
self-bootstrap witness remains proposal-only unless explicitly ratified.
```

## Hard Blockers

```text
A12-IMPLEMENTABLE-AFTER-A04-A11
```

Hard blockers:

- A04-A11 predecessor atoms are not all implemented.
- All parent-plan A12 files are missing.
- Parent-plan A12 originally omitted the dedicated W5 agent-view witness path.
- Existing flowchart liveness tests explicitly do not claim full FC liveness.
- FC3 feedback/re-init runtime authority is missing.
- Self-bootstrap cannot be more than proposal-only without separate
  high-risk ratification.

Clean-context audit input for a future implementation PR:

```text
Task brief: A12 Universal Machine Witnesses.
Risk class: Class 2 witness-only; promote if runtime/economy/provider authority
is touched.
FC nodes: FC1 full loop, FC2 boot/replay/halt, FC3 logs/archive feedback.
Evidence: A04-A11 predecessor evidence, A12 witness tests, constitution gates.
Verdict domain: NO-VIOLATION | VIOLATION-FOUND | RECONSTRUCTION-FAILURE |
SECOND-SOURCE-DRIFT
```
