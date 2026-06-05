# A07 Agent View Shielding Preflight

Date: 2026-06-05

Parent plan:
`handover/directives/2026-06-05_AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT.md`

Atom: A07. Agent Process Model and View Shielding

Document role: Class 0 preflight. This document does not authorize production
view authority, prompt schema changes, or web/runtime shielding rewrites by
itself.

## Decision

A07 should not claim production completion until A05 provides the generic
TapeEvent/projection contract. Existing role-view, PromptCapsule, and
BuildSessionView code are valuable shielding witnesses, but they are not the
same as a generic `AgentView` derived from an allowed tape prefix.

Safe work now:

- docs-only preflight
- allowed-path correction
- test-shape design
- positive-control inventory

Blocked until A05 exists or is explicitly substituted by a ratified predecessor:

- claiming `AgentView` is derived from ChainTape prefix
- adding a runtime `tc_agent_view` module as authority
- changing PromptCapsule or PromptCapsuleV2 schema shape
- exposing private diagnostic, hidden oracle, raw prompt, raw stdout/stderr, or
  sibling-private chain CIDs in ordinary agent views

## Hard Blockers

- A07-HB1: A07 cannot claim generic AgentView completion before A05 provides a
  tape-prefix projection substrate or a ratified substitute.
- A07-HB2: PromptCapsule/PromptCapsuleV2 schema-shape changes are blocked
  without explicit risk reclassification and ratification.
- A07-HB3: Any ordinary AgentView or HTTP route exposing private diagnostics,
  hidden oracle/test body, raw stderr/stdout, raw prompt internals, or sibling
  private chain CIDs is blocked.

## Current-State Facts

Parent-plan A07 allowed paths as originally written:

```text
src/runtime/tc_agent_view.rs
tests/tc_agent_view_shielding.rs
tests/hidden_oracle_not_in_generation_prompt_bytes.rs
tests/hidden_oracle_set_cid_not_in_build_session_view.rs
tests/build_session_view_does_not_expose_private_diagnostic_cid.rs
```

Corrected implementation path inventory:

```text
src/runtime/tc_agent_view.rs
src/runtime/mod.rs
src/runtime/real5_roles.rs
src/runtime/prompt_capsule.rs
src/runtime/attempt_telemetry.rs
src/runtime/build_session_view.rs
src/runtime/audit_assertions.rs
src/runtime/test_run.rs
src/runtime/rejection_capsule.rs
src/web/artifact_bundle.rs
src/sdk/prompt.rs
src/sdk/your_position.rs
tests/tc_agent_view_shielding.rs
tests/constitution_real5_role_scoped_view.rs
tests/constitution_real5_prompt_capsule_v2.rs
tests/hidden_oracle_not_in_generation_prompt_bytes.rs
tests/hidden_oracle_set_cid_not_in_build_session_view.rs
tests/build_session_view_does_not_expose_private_diagnostic_cid.rs
tests/build_session_view_does_not_expose_test_scenario_set_cid.rs
tests/artifact_bundle_serve_rejects_private_diagnostic_cid.rs
tests/rejection_private_diagnostic_not_in_http_body.rs
```

Existence check:

```text
MISSING src/runtime/tc_agent_view.rs
EXISTS src/runtime/mod.rs
EXISTS src/runtime/real5_roles.rs
EXISTS src/runtime/prompt_capsule.rs
EXISTS src/runtime/attempt_telemetry.rs
EXISTS src/runtime/build_session_view.rs
EXISTS src/runtime/audit_assertions.rs
EXISTS src/runtime/test_run.rs
EXISTS src/runtime/rejection_capsule.rs
EXISTS src/web/artifact_bundle.rs
EXISTS src/sdk/prompt.rs
EXISTS src/sdk/your_position.rs
MISSING tests/tc_agent_view_shielding.rs
EXISTS tests/constitution_real5_role_scoped_view.rs
EXISTS tests/constitution_real5_prompt_capsule_v2.rs
EXISTS tests/hidden_oracle_not_in_generation_prompt_bytes.rs
EXISTS tests/hidden_oracle_set_cid_not_in_build_session_view.rs
EXISTS tests/build_session_view_does_not_expose_private_diagnostic_cid.rs
EXISTS tests/build_session_view_does_not_expose_test_scenario_set_cid.rs
EXISTS tests/artifact_bundle_serve_rejects_private_diagnostic_cid.rs
EXISTS tests/rejection_private_diagnostic_not_in_http_body.rs
```

Dirty-path check for the corrected inventory:

```text
pre-existing dirty paths include:
  src/runtime/mod.rs
  src/runtime/audit_assertions.rs
  src/web/artifact_bundle.rs
  tests/hidden_oracle_not_in_generation_prompt_bytes.rs
  tests/hidden_oracle_set_cid_not_in_build_session_view.rs
  tests/build_session_view_does_not_expose_private_diagnostic_cid.rs

Implementation must read and preserve those edits. Do not overwrite them as
part of A07 scaffolding.
```

Existing view/shielding witnesses:

```text
src/runtime/real5_roles.rs:1
  role assignment, role-scoped derived views, typed role actions, and traces
src/runtime/real5_roles.rs:135
  AgentRoleAssignment includes allowed_tools, risk_budget_micro, view_policy_id
tests/constitution_real5_role_scoped_view.rs:11
  role views are scoped, hashable, and redact raw diagnostics / private internals
tests/constitution_real5_role_scoped_view.rs:58
  visible_context_cid matches visible context bytes
src/runtime/prompt_capsule.rs:75
  PromptCapsule must not carry verbatim prompt bytes
src/runtime/prompt_capsule.rs:107
  PromptCapsuleV2 binds agent id, role, view policy, read-set CIDs, and model
  assignment provenance
src/runtime/attempt_telemetry.rs:363
  AttemptTelemetry links to role/view PromptCapsule CID
src/runtime/build_session_view.rs:70
  BuildSessionView exposes session CIDs and accepted_delivery bool only
src/runtime/build_session_view.rs:264
  TestScenarioSet CID is intentionally not surfaced
src/runtime/test_run.rs:29
  TestRunCapsule keeps test_scenario_set_cid as hidden-oracle state
src/runtime/rejection_capsule.rs:35
  GenerateRejectionCapsule keeps private_diagnostic_cid as a shielded CAS
  pointer behind public summary
src/web/artifact_bundle.rs:82
  artifact bundle route rejects non-artifact-bundle schema CIDs
src/sdk/your_position.rs:13
  per-viewer position surface is designed to avoid cross-agent PnL leakage
src/runtime/audit_assertions.rs:2468
  AgentVisibleProjection must not serialize autopsy private_detail_cid bytes
```

Existing tests are partial witnesses, not generic A07 completion:

```text
tests/hidden_oracle_not_in_generation_prompt_bytes.rs:34
  scenario-set bytes must not appear in generation prompt capsule bytes
tests/hidden_oracle_set_cid_not_in_build_session_view.rs:14
  BuildSessionView has no test_scenario_set_cid field
tests/build_session_view_does_not_expose_private_diagnostic_cid.rs:74
  web build-session view does not expose private_diagnostic_cid
tests/build_session_view_does_not_expose_test_scenario_set_cid.rs:74
  web build-session view does not expose test_scenario_set_cid
tests/artifact_bundle_serve_rejects_private_diagnostic_cid.rs:69
  artifact bundle serve path rejects private diagnostic CIDs
tests/rejection_private_diagnostic_not_in_http_body.rs:212
  generate rejection HTTP body hides private diagnostic CID/body while exposing
  public rejection fields
tests/constitution_real5_prompt_capsule_v2.rs:19
  PromptCapsuleV2 carries role/view data and no raw prompt/completion/COT
tests/constitution_real5_prompt_capsule_v2.rs:90
  externalized attempt references PromptCapsuleV2 CID
```

## Risk Classification

Risk floor: Class 2. A07 creates a generic agent-view contract and shielding
tests.

Promote to Class 3 if:

- ordinary agent views become prompt-construction inputs
- CAS privacy or audit-only metadata is changed
- web or CLI views become authoritative evidence paths
- process/tool capability routing changes

Promote to Class 4 if:

- PromptCapsule / PromptCapsuleV2 schema shape changes
- typed transaction schema changes
- canonical signing payload changes
- sequencer admission or predicate authority changes
- trust-root / constitution / flowchart authority changes

## Recommended Contract

The first implementation contract should be an A05 projection over a bounded
tape prefix, not a broad workspace snapshot.

Minimum generic view:

```text
AgentView {
  agent_id: AgentId,
  role: Option<AgentRole>,
  view_policy_id: String,
  allowed_tape_prefix_head: GitOid,
  visible_context_cid: Cid,
  visible_context_hash: Hash,
  visible_event_cids: Vec<Cid>,
  redacted_fields: Vec<String>,
  denied_cids: Vec<Cid>,
}
```

Required invariant:

```text
visible_context_hash == sha256(canonical_visible_context_bytes)
all visible_event_cids are derivable from allowed_tape_prefix_head
denied_cids never appear in serialized AgentView, prompt bytes, web views, or
ordinary dashboards
```

Do not add fields to PromptCapsuleV2 merely to satisfy A07. If PromptCapsuleV2
cannot carry the necessary witness, use an adjacent CAS sidecar with explicit
schema_id and preserve existing capsule payload shape.

## Atomized A07 Tasks

### A07.0 Preflight Lock

Description:
Record missing files, predecessor dependency, corrected paths, and the boundary
between existing shielding witnesses and the planned generic AgentView.

Acceptance:

```bash
for f in \
  handover/directives/2026-06-05_A07_AGENT_VIEW_SHIELDING_PREFLIGHT.md \
  handover/directives/2026-06-05_AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT.md
do
  git diff --no-index --check /dev/null "$f" || true
done
```

Expected:

```text
no whitespace errors.
A07 preflight states A05 TapeEvent projection is a predecessor.
```

### A07.1 Generic AgentView Projection

Description:
After A05 exists, add `tc_agent_view` tests and module proving AgentView derives
only from the allowed tape prefix plus CAS objects.

Acceptance:

```bash
cargo test --test tc_agent_view_shielding --no-fail-fast -- --test-threads=1
cargo test --test constitution_real5_role_scoped_view --no-fail-fast
cargo test --test constitution_real5_prompt_capsule_v2 --no-fail-fast
git diff --check
```

Expected:

```text
role view is prefix-bound.
future price/event leakage fails.
sibling private chain CID leakage fails.
PromptCapsuleV2 schema shape is unchanged unless explicitly ratified.
```

### A07.2 Hidden Oracle And Private Diagnostic Gates

Description:
Keep existing hidden-oracle and private-diagnostic tests in the A07 acceptance
surface so later implementation cannot narrow the definition of shielding.

Acceptance:

```bash
cargo test --test hidden_oracle_not_in_generation_prompt_bytes --no-fail-fast
cargo test --test hidden_oracle_set_cid_not_in_build_session_view --no-fail-fast
cargo test --features web --test build_session_view_does_not_expose_private_diagnostic_cid --no-fail-fast
cargo test --features web --test build_session_view_does_not_expose_test_scenario_set_cid --no-fail-fast
cargo test --features web --test artifact_bundle_serve_rejects_private_diagnostic_cid --no-fail-fast
cargo test --features web --test rejection_private_diagnostic_not_in_http_body --no-fail-fast
git diff --check
```

Expected:

```text
hidden oracle payload bytes are absent from generation prompt capsules.
test_scenario_set_cid is absent from BuildSessionView and web responses.
private_diagnostic_cid is absent from BuildSessionView and web responses.
artifact bundle and generate-error HTTP routes do not serve private diagnostic
CIDs.
```

## Final Pre-Implementation Gate

A07 implementation may start only when all are true:

- A05 has landed or ratified the generic TapeEvent/projection envelope
- the first code change is a failing `tc_agent_view_shielding` test
- `src/runtime/mod.rs` and existing dirty shielding tests are preserved
- PromptCapsuleV2 schema changes have explicit Class 4 ratification, or no
  PromptCapsuleV2 schema changes are made
- ordinary views remain derived outputs, not Tier 2 facts or sequencer inputs

Clean-context audit input for a future implementation PR:

```text
Task brief: A07 Agent Process Model and View Shielding.
Risk class: Class 2; promote if prompt schema or authority surfaces change.
FC nodes: FC1-N5, FC1-N6, FC1-N7, FC3-N31, FC3 shielding invariants.
Evidence: A05 predecessor evidence, AgentView shielding tests, hidden-oracle
tests, private-diagnostic web tests, constitution gates.
Verdict domain: NO-VIOLATION | VIOLATION-FOUND | RECONSTRUCTION-FAILURE |
SECOND-SOURCE-DRIFT
```
