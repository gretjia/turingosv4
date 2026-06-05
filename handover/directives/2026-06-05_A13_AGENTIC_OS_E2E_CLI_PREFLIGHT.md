# A13 Agentic OS E2E CLI Preflight

Date: 2026-06-05

Parent plan:
`handover/directives/2026-06-05_AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT.md`

Atom: A13. Agentic OS v0 E2E CLI

Document role: Class 0 preflight. This document does not authorize boot
trust-root changes, OS runtime authority, provider/network execution, economy
settlement, sequencer admission, or typed transaction changes by itself.

## Decision

A13 is not implementable yet. All parent-plan A13 source, test, and fixture
paths are missing. The current CLI has `replay`, `verify chaintape`,
`generate`, `llm`, `spec`, and related commands, but it does not have the
planned `turingos boot` or `turingos os run/replay/audit` contract.

Existing `turingos replay`, `turingos verify chaintape`, and `turingos generate`
are useful precedents. They are not OS v0 E2E.

Safe work now:

- docs-only preflight
- CLI surface inventory
- fixture-path correction
- network-off and no-benchmark-claim contract

Blocked until predecessors exist:

- implementing OS v0 E2E before A03-A12 exist
- claiming OS substrate success from existing generate/replay smoke tests
- running real provider/network calls in the first E2E
- claiming benchmark solve-rate or market victory
- wiring economy settlement without A09/A10

## Current-State Facts

Parent-plan A13 allowed paths:

```text
src/bin/turingos/cmd_boot.rs
src/bin/turingos/cmd_os.rs
src/runtime/os_run.rs
tests/os_boot_to_replay_e2e.rs
tests/os_market_settlement_e2e.rs
tests/os_agent_view_e2e.rs
fixtures/os/hello_agentic_task.json
```

Corrected implementation path inventory:

```text
src/bin/turingos.rs
src/bin/turingos/cmd_boot.rs
src/bin/turingos/cmd_os.rs
src/runtime/mod.rs
src/runtime/os_run.rs
src/bin/turingos/cmd_replay.rs
src/bin/turingos/cmd_verify_chaintape.rs
src/bin/turingos/cmd_generate.rs
src/bin/turingos/cmd_llm.rs
src/bin/turingos/chat_client.rs
src/bin/turingos/cmd_audit_tape.rs
src/bin/turingos/cmd_audit_dashboard.rs
src/boot.rs
src/main.rs
src/web/router.rs
src/runtime/verify.rs
src/runtime/replay.rs
tests/os_boot_to_replay_e2e.rs
tests/os_market_settlement_e2e.rs
tests/os_agent_view_e2e.rs
tests/cli_replay_smoke.rs
tests/cli_verify_chaintape_smoke.rs
fixtures/os/hello_agentic_task.json
```

Write-scope guidance:

```text
src/bin/turingos.rs
  required to register cmd_boot/cmd_os; currently dirty and must be preserved
src/runtime/mod.rs
  required only if src/runtime/os_run.rs must be crate-visible; currently dirty
src/bin/turingos/cmd_replay.rs
src/bin/turingos/cmd_verify_chaintape.rs
src/runtime/verify.rs
src/runtime/replay.rs
  read-only precedents unless A13 explicitly reuses helpers
src/bin/turingos/cmd_generate.rs
src/bin/turingos/cmd_llm.rs
src/bin/turingos/chat_client.rs
  provider/network precedents; first A13 E2E must run network-off
src/bin/turingos/cmd_audit_tape.rs
src/bin/turingos/cmd_audit_dashboard.rs
  read-only audit precedents; not `turingos os audit`
src/boot.rs
src/main.rs
  boot-like Trust Root verification precedent; not user-facing `turingos boot`
src/state/sequencer.rs
src/state/typed_tx.rs
src/bottom_white/cas/schema.rs
src/kernel.rs
src/bus.rs
  out of A13 write scope without explicit higher-risk ratification
```

Existence check:

```text
MISSING src/bin/turingos/cmd_boot.rs
MISSING src/bin/turingos/cmd_os.rs
MISSING src/runtime/os_run.rs
MISSING tests/os_boot_to_replay_e2e.rs
MISSING tests/os_market_settlement_e2e.rs
MISSING tests/os_agent_view_e2e.rs
MISSING fixtures/os/hello_agentic_task.json
MISSING fixtures/
EXISTS src/bin/turingos.rs
EXISTS src/runtime/mod.rs
EXISTS src/bin/turingos/cmd_replay.rs
EXISTS src/bin/turingos/cmd_verify_chaintape.rs
EXISTS src/bin/turingos/cmd_generate.rs
EXISTS src/bin/turingos/cmd_llm.rs
EXISTS src/bin/turingos/chat_client.rs
EXISTS src/bin/turingos/cmd_audit_tape.rs
EXISTS src/bin/turingos/cmd_audit_dashboard.rs
EXISTS src/boot.rs
EXISTS src/main.rs
EXISTS src/web/router.rs
EXISTS src/runtime/verify.rs
EXISTS src/runtime/replay.rs
EXISTS tests/cli_replay_smoke.rs
EXISTS tests/cli_verify_chaintape_smoke.rs
```

Dirty-path check for relevant inventory:

```text
pre-existing dirty paths include:
  src/bin/turingos.rs
  src/runtime/mod.rs
  src/state/sequencer.rs
  src/state/typed_tx.rs

Implementation must read and preserve those edits. Do not overwrite them as
part of A13 scaffolding.
```

Existing CLI/runtime witnesses:

```text
src/bin/turingos.rs:37
  command modules are registered through explicit path modules
src/bin/turingos.rs:106
  SUBCOMMANDS table is append-only registry style
src/bin/turingos.rs:169
  existing `replay` subcommand
src/bin/turingos.rs:139
  existing `verify chaintape` subcommand
src/bin/turingos/cmd_replay.rs:1
  `turingos replay` handler covers ChainTape replay and CAS-only offline replay
src/bin/turingos/cmd_replay.rs:6
  `--offline` path is CAS-only and no-network
src/bin/turingos/cmd_replay.rs:75
  offline replay parses workspace/session and calls runtime replay
src/bin/turingos/cmd_verify_chaintape.rs:1
  verify chaintape wrapper exists
src/bin/turingos/cmd_verify_chaintape.rs:23
  verify chaintape is read-only; no sequencer call and no CAS write
src/bin/turingos/cmd_audit_tape.rs:23
  audit tape is read-only over ChainTape/CAS evidence
src/bin/turingos/cmd_audit_dashboard.rs:26
  audit dashboard is a materialized view, never authority
src/boot.rs:97
  verify_trust_root exists as lower-level boot verifier
src/main.rs:12
  main verifies Trust Root at process start and panics on tamper
src/bin/turingos/cmd_generate.rs:1
  generate reads spec/CAS and calls the LLM; it is not OS v0 run
src/bin/turingos/cmd_generate.rs:308
  existing parallel worker flag is CLI fan-out, not A11/A13 OS scheduler
src/bin/turingos/cmd_llm.rs:1
  llm command configures two-model provider setup
src/bin/turingos/chat_client.rs:1
  SiliconFlow HTTP client is real provider/network surface
src/web/router.rs:140
  web market view is a read-only projection over per-session evidence
tests/cli_replay_smoke.rs:1
  replay CLI smoke exists
tests/cli_verify_chaintape_smoke.rs:1
  verify chaintape CLI smoke exists
```

Non-closure facts:

```text
No current `turingos boot` command.
No current `turingos os` command family.
No current src/runtime/os_run.rs.
No current OS fixture directory.
No current unified `--network off` OS-run gate.
Existing replay/generate commands do not produce the full A13 artifact set.
```

## Risk Classification

Risk floor: Class 2 for network-off CLI scaffolding and deterministic fixture
E2E.

Promote to Class 3 if:

- real provider/network calls are enabled
- real economy/capability calls are enabled
- real market settlement is exercised
- generated evidence is used as ship-path proof for Class 3 atoms

Promote to Class 4 if:

- boot trust-root authority changes
- sequencer admission changes
- typed tx schema or discriminants change
- canonical signing payload changes
- CAS schema authority changes
- trust-root / constitution / flowchart authority changes

## Recommended Contract

A13 CLI contract:

```bash
turingos boot --verify-manifest
turingos os run --task fixtures/os/hello_agentic_task.json --policy single_tree --market on --network off
turingos os replay --run-dir <run-dir>
turingos os audit --run-dir <run-dir>
```

Run manifest:

```text
DerivedArtifactRef {
  path: Path,
  content_hash_or_cid: String,
  derived_from_tape_head: GitOid,
  derived_from_cas_root: Option<GitOid>,
  artifact_kind: String,
  replay_recipe: String,
}

OsRunManifest {
  run_id: RunId,
  network_policy: "off",
  task_fixture_cid: Cid,
  git_tape_repo: Path,
  replay_report_path: Path,
  predicate_receipts_path: Path,
  external_call_receipts_path: Path,
  economy_projection_path: Path,
  agent_view_audit_path: Path,
  derived_artifacts: Vec<DerivedArtifactRef>,
}
```

Required invariant:

```text
first E2E is network-off.
mock provider or deterministic fixture only.
pending external calls == 0 at clean halt.
no benchmark solve-rate claim.
no market victory claim.
all artifacts are replayable from ChainTape/L4 + CAS or explicitly marked as
derived/audit-only.
every derived artifact includes derived_from_tape_head, content hash or CID,
and replay recipe.
```

## Atomized A13 Tasks

### A13.0 Preflight Lock

Description:
Record missing A13 files, current CLI surface, dirty-path conflicts, and
network-off/no-claim boundaries.

Acceptance:

```bash
for f in \
  handover/directives/2026-06-05_A13_AGENTIC_OS_E2E_CLI_PREFLIGHT.md \
  handover/directives/2026-06-05_AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT.md
do
  git diff --no-index --check /dev/null "$f" || true
done
```

Expected:

```text
no whitespace errors
```

### A13.1 CLI Scaffolding

Description:
After A03-A12 are ready, add `boot` and `os` command routing without enabling
real provider calls.

Primary paths:

```text
src/bin/turingos.rs
src/bin/turingos/cmd_boot.rs
src/bin/turingos/cmd_os.rs
src/runtime/mod.rs
src/runtime/os_run.rs
tests/os_boot_to_replay_e2e.rs
fixtures/os/hello_agentic_task.json
```

Acceptance:

```bash
cargo test --test os_boot_to_replay_e2e --no-fail-fast -- --test-threads=1
cargo test --test cli_replay_smoke --no-fail-fast
cargo test --test cli_verify_chaintape_smoke --no-fail-fast
git diff --check
```

Expected:

```text
turingos boot --verify-manifest is present.
turingos os run/replay/audit help is present.
network-off deterministic fixture run produces a run_manifest.json.
```

### A13.2 OS Run Replay And Audit Artifacts

Description:
Wire the deterministic OS run to produce the full A13 artifact set and verify
network-off replay.

Primary paths:

```text
src/runtime/os_run.rs
tests/os_boot_to_replay_e2e.rs
```

Acceptance:

```bash
cargo test --test os_boot_to_replay_e2e --no-fail-fast -- --test-threads=1
bash scripts/run_constitution_gates.sh
```

Expected:

```text
run_manifest.json exists.
git_tape_repo/ exists and git fsck --full passes.
replay_report.json says deterministic.
external_call.pending == 0.
each derived artifact derived_from_tape_head == final ChainTape/L4 head.
each derived artifact has content hash or CID and replay recipe.
unsupported_task_success_claim_count == 0.
```

### A13.3 Market And Agent-View E2E

Description:
After A07/A09/A10/A11/A12 are ready, add market-settlement and agent-view E2E
tests. These tests remain network-off.

Primary paths:

```text
tests/os_market_settlement_e2e.rs
tests/os_agent_view_e2e.rs
```

Acceptance:

```bash
cargo test --test os_market_settlement_e2e --no-fail-fast -- --test-threads=1
cargo test --test os_agent_view_e2e --no-fail-fast -- --test-threads=1
```

Expected:

```text
economy_projection.conservation_ok == true.
hidden_leak_count == 0.
agent view audit derives from scoped tape/CAS evidence.
```

## Full A13 Acceptance

After A03-A12 exist and A13 implementation is complete:

```bash
cargo test --test os_boot_to_replay_e2e --no-fail-fast -- --test-threads=1
cargo test --test os_market_settlement_e2e --no-fail-fast -- --test-threads=1
cargo test --test os_agent_view_e2e --no-fail-fast -- --test-threads=1
cargo test --test cli_replay_smoke --no-fail-fast
cargo test --test cli_verify_chaintape_smoke --no-fail-fast
cargo test --workspace --no-fail-fast
bash scripts/run_constitution_gates.sh
cargo test --test constitution_matrix_drift --no-fail-fast
git diff --check
```

Expected:

```text
PREDICATES-GREEN
run_manifest.json
git_tape_repo/
replay_report.json
predicate_receipts.jsonl
external_call_receipts.jsonl
economy_projection.json
agent_view_audit.json
git fsck --full passes.
replay_report.deterministic == true.
external_call.pending == 0.
economy_projection.conservation_ok == true.
hidden_leak_count == 0.
unsupported_task_success_claim_count == 0.
```

## Hard Blockers

```text
A13-IMPLEMENTABLE-AFTER-A03-A12
```

Hard blockers:

- A03-A12 predecessor atoms are not all implemented.
- All parent-plan A13 source/test/fixture files are missing.
- Existing CLI lacks `boot` and `os` command family.
- Existing generate/replay paths are not OS v0 E2E.
- First E2E must remain network-off and benchmark-claim-free.

Clean-context audit input for a future implementation PR:

```text
Task brief: A13 Agentic OS v0 E2E CLI.
Risk class: Class 2 network-off fixture; Class 3 if real provider/economy
calls are enabled; Class 4 if restricted authority surfaces are touched.
FC nodes: FC1 full loop, FC2 boot/halt/replay, FC3 log archive.
Evidence: A03-A12 predecessor evidence, A13 E2E tests, workspace tests,
constitution gates, generated artifact paths.
Verdict domain: NO-VIOLATION | VIOLATION-FOUND | RECONSTRUCTION-FAILURE |
SECOND-SOURCE-DRIFT
```
