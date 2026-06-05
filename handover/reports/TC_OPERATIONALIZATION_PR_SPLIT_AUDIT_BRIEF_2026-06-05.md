# TuringOS-TC PR Split and Audit Brief

Date: 2026-06-05

Document type: PR split audit brief

Worktree under review: `/Users/zephryj/work/turingosv4-tc-operationalization`

Branch: `codex/turingos-tc-operationalization`

Base / recorded HEAD: `39233aa7c868f0e9b37a7a29eb426279f41cf032`

Audience: external auditor, architect, and orchestrator deciding how to convert
the current uncommitted worktree into reviewable PRs.

## Executive Recommendation

Do not open the current worktree as one PR.

The current work is useful and evidence-bearing, but it is not yet in PR shape.
It mixes core runtime changes, benchmark scaffolding, current-kernel task
adapters, test repairs, TC planning documents, audit reports, and historical
evidence handling in one large dirty worktree.

The correct next step is a PR split pass:

1. Preserve this report plus the benchmark packet as audit evidence.
2. Create narrowly scoped PRs from the current worktree, each with its own file
   list, risk class, gate commands, and clean-context audit requirement.
3. Re-run verification after each split PR is staged. Do not rely on the
   monolithic worktree gates as merge evidence for a smaller PR.
4. Keep the claim boundary unchanged: the current benchmark supports
   `FLOW-PASS` and `SYSTEM-PASS`, not `TASK-PASS`.

## Current Worktree Inventory

The worktree is intentionally not clean.

Current local inventory from `git status --porcelain`:

```text
M  178 tracked modified files
?? 99 untracked files
```

Current diff stat:

```text
178 files changed, 7694 insertions(+), 2277 deletions(-)
```

Tracked modified file categories:

```text
src/runtime/*                  2
src/bin/turingos/*             7
src/bin/* current-kernel bins  30
src/judges/*                   9
src/web/*                      7
src other                      13
tests/*                        104
scripts/*                      2
handover/*                     1
OBLIGATIONS.md                 1
trust-root-adjacent files      2
```

Untracked file categories:

```text
handover/directives/*          65
handover/reports/*             4
handover/audits/*              2
scripts/*                      2
src/*                          10
tests/*                        16
```

High-risk or trust-root-adjacent paths present in the modified set:

```text
build.rs
genesis_payload.toml
src/git_tape_ledger.rs
```

These are not necessarily wrong, but they make a monolithic PR inappropriate.
They need focused review and per-PR gates.

## Benchmark Evidence Boundary

Primary benchmark report:

```text
handover/reports/CONSTITUTIONAL_FULL_FLOW_BENCHMARK_PACKET_2026-06-05.md
sha256 d719811f21d39fb9e32f4840e6738eb3e5db4e316959c10125a6105158998762
```

Primary packet JSON:

```text
/tmp/turingos_real_task_benchmark_20260605/full_flow_packet_context_budgetfix_rerun2/constitutional_full_flow_benchmark_packet.json
sha256 1eec4428e6c3d6f5d73bf2f779bb54b3a5be1018ba0f0d2935034425b6bb91c1
```

Full-system participation:

```text
/tmp/turingos_real_task_benchmark_20260605/full_flow_packet_context_budgetfix_rerun2/full_system_participation.json
sha256 bf87f554db41b8e41b0a811917a0c793e8f033d76d65ca48eeb87540b616ab5a
```

Replay report:

```text
/tmp/turingos_real_task_benchmark_20260605/full_flow_packet_context_budgetfix_rerun2/replay_report.json
sha256 9e199f1a11d73d756a50d8751862b1415d91358d59b8ed9562f4cb1364b03508
```

Real TDMA/SWE-bench evidence:

```text
/tmp/turingos_real_task_benchmark_20260605/tdma_evidence_flask5063_context_budgetfix/manifest.json
sha256 7bb878632f0f71b21191ea922983f9bac7c829ebadee715fadde0a562bf2f1cf

/tmp/turingos_real_task_benchmark_20260605/tdma_evidence_flask5063_context_budgetfix/per_attempt_probes.jsonl
sha256 ff4aefacd91a877ea2c842e3af033c84e244029ba8882fd9df9c122fb4599d5f
```

Supported claim:

```text
FLOW-PASS:   FC1/FC2/FC3 node-level runtime receipts were present.
SYSTEM-PASS: replay, CAS retrieval, signatures, and packet consistency passed.
```

Unsupported claim:

```text
TASK-PASS: not supported.
```

The real task verdict is:

```json
{
  "kind": "TASK-FAIL",
  "source": "swebench_tdma_hidden_test_verifier",
  "benchmark_verdict": "unresolved",
  "closure_scope": "real_tdma_swebench",
  "final_closure_possible": false,
  "stages_completed": 0,
  "stages_total": 1,
  "total_attempts": 3,
  "leak_in_any_prompt": false
}
```

The SWE-bench instance was `pallets__flask-5063`. The official harness excerpt
recorded `resolved=false`, with remaining failing tests:

```text
tests/test_cli.py::TestRoutes::test_subdomain
tests/test_cli.py::TestRoutes::test_host
```

Market participation was real and tape-visible, but it was anchored to a
system-participation canary after the SWE-bench domain work was rejected:

```json
{
  "domain_accepted_work_tx_id": null,
  "domain_rejected_work_l4e_count": 1,
  "domain_manifest_work_tx_landed": false,
  "market_anchor_source": "system_participation_canary_after_domain_rejection",
  "system_canary_work_tx_id": "worktx-full-system-participation-canary-full-system-canary"
}
```

This distinction is load-bearing. The packet is evidence that market signals
were present in the constitutional full-flow run. It is not evidence that the
failed SWE-bench patch became an accepted market-backed work product.

## Audit Evidence

Clean-context benchmark audit:

```text
handover/audits/CONSTITUTIONAL_FULL_FLOW_BENCHMARK_CLEAN_CONTEXT_AUDIT_2026-06-05.md
Final verdict: PROCEED
```

Important audit history:

```text
Initial verdict: CHALLENGE
Challenge: report under-disclosed that market participation was canary anchored,
not accepted-domain-WorkTx anchored.
Remediation: report now embeds the canary anchor fields and explicitly states
that market receipt is not evidence of accepted SWE-bench market-backed work.
Final: PROCEED
```

Obligation witness:

```text
handover/audits/CONSTITUTIONAL_FULL_FLOW_BENCHMARK_OBLIGATION_WITNESS_2026-06-05.md
Final verdict: OBL-ALL-CLOSED
```

Verification gates recorded in the benchmark report:

```text
cargo test --test constitution_matrix_drift --no-fail-fast
bash scripts/run_constitution_gates.sh
[k-1-5] total=165 failed=0

cargo test --test cli_benchmark_full_flow --no-fail-fast -- --test-threads=1
cargo test swebench_test_judge --lib --no-fail-fast
cargo test distiller_in_budget --lib --no-fail-fast
cargo test --bin turingos swebench_prompt --no-fail-fast
cargo test --test constitution_production_module_liveness every_exported_module_has_exactly_one_liveness_group --no-fail-fast -- --test-threads=1
cargo test --test constitution_obligation_repair_reconciliation --no-fail-fast -- --test-threads=1
git diff --check
```

Observed focused results recorded:

```text
cli_benchmark_full_flow: 3/3 passed
swebench_test_judge lib tests: 11/11 passed
distiller focused tests: 2/2 passed
turingos swebench_prompt tests: 2/2 passed
production_module_liveness targeted gate: 1/1 passed
obligation reconciliation: 3/3 passed
git diff --check: no output
restricted surface scan: no restricted source file matched
```

These are strong evidence for the benchmark packet. They are not sufficient to
merge the full uncommitted worktree as one PR.

## Proposed PR Split

### PR 0: Auditor Decision Packet

Purpose:

Keep only decision-facing artifacts that let auditors and architects decide how
to proceed.

Suggested files:

```text
handover/reports/TC_OPERATIONALIZATION_PR_SPLIT_AUDIT_BRIEF_2026-06-05.md
handover/reports/CONSTITUTIONAL_FULL_FLOW_BENCHMARK_PACKET_2026-06-05.md
handover/audits/CONSTITUTIONAL_FULL_FLOW_BENCHMARK_CLEAN_CONTEXT_AUDIT_2026-06-05.md
handover/audits/CONSTITUTIONAL_FULL_FLOW_BENCHMARK_OBLIGATION_WITNESS_2026-06-05.md
```

Classification: report / audit evidence.

Risk class: Class 0 unless the report is used as a shipping gate for code.

Gate:

```bash
git diff --check -- handover/reports/TC_OPERATIONALIZATION_PR_SPLIT_AUDIT_BRIEF_2026-06-05.md \
  handover/reports/CONSTITUTIONAL_FULL_FLOW_BENCHMARK_PACKET_2026-06-05.md \
  handover/audits/CONSTITUTIONAL_FULL_FLOW_BENCHMARK_CLEAN_CONTEXT_AUDIT_2026-06-05.md \
  handover/audits/CONSTITUTIONAL_FULL_FLOW_BENCHMARK_OBLIGATION_WITNESS_2026-06-05.md
```

Merge condition:

Architect agrees that these reports are evidence records, not claims that
TuringOS solved the benchmark task.

### PR 1: Constitutional Full-Flow Benchmark CLI

Purpose:

Add the real `turingos benchmark full-flow run` CLI path that produces an
auditable packet from existing ChainTape/CAS/current-kernel helpers and refuses
to label unresolved tasks as `TASK-PASS`.

Core files:

```text
src/bin/turingos.rs
src/bin/turingos/cmd_benchmark_full_flow.rs
src/bin/turingos/common.rs
tests/cli_benchmark_full_flow.rs
```

Likely supporting files:

```text
src/sdk/sanitized_runner.rs
src/judges/swebench_test_judge.rs
src/tdma_runner.rs
src/distiller.rs
tests/support/mod.rs
```

Classification: benchmark scaffolding with CLI runtime integration.

Risk class: Class 2, because it wires evidence-bearing CLI behavior but should
not alter sequencer admission, typed transaction schema, or kernel authority.

Key source excerpt:

```rust
let task_verdict = if let Some(task_evidence_dir) = &args.task_evidence_dir {
    task_verdict_from_tdma_evidence(task_evidence_dir)?
} else {
    task_verdict_from_domain(&domain)
};
if args.require_task_pass && task_verdict.kind != "TASK-PASS" {
    write_packet(...)?;
    return Err("--require-task-pass set but task verifier did not return TASK-PASS".into());
}
```

Acceptance criteria:

```text
The CLI emits FLOW-PASS/SYSTEM-PASS only when FC1/FC2/FC3 receipts and replay
checks are present.
The CLI emits TASK-PASS only when the attached task evidence proves task pass.
The CLI records TASK-FAIL/unresolved for unresolved SWE-bench evidence.
The CLI does not call unresolved structural smoke a task success.
```

Gate:

```bash
cargo test --test cli_benchmark_full_flow --no-fail-fast -- --test-threads=1
cargo test swebench_test_judge --lib --no-fail-fast
cargo test --bin turingos swebench_prompt --no-fail-fast
git diff --check
```

Follow-up benchmark gate:

```bash
target/debug/turingos benchmark full-flow run \
  --run-dir <fresh-run-dir> \
  --run-id <fresh-run-id> \
  --constitution constitution.md \
  --sample-json <real-swebench-sample-json> \
  --llm-proxy-url <proxy-url> \
  --model <model> \
  --task-evidence-dir <fresh-tdma-evidence-dir>
```

Merge condition:

Clean-context auditor confirms no false `TASK-PASS`, no hidden-test leakage, and
clear canary/domain-market distinction if canary remains.

### PR 2: Full-System Current-Kernel Benchmark Adapters

Purpose:

Keep the current-kernel domain adapters and FC participation helpers separate
from the CLI packet code. These binaries are scaffolding for coverage and
benchmark orchestration, not core kernel authority.

Candidate files:

```text
src/bin/full_system_augment_current_kernel.rs
src/bin/full_system_participation_current_kernel.rs
src/bin/fc3_governance_reinit_current_kernel.rs
src/bin/g0_market_activation_current_kernel.rs
src/bin/market_external_agent_current_kernel.rs
src/bin/swebench_live_coding_repair_current_kernel.rs
src/bin/osworld_computer_use_current_kernel.rs
src/bin/webarena_web_agent_current_kernel.rs
src/bin/mind2web_browser_action_current_kernel.rs
src/bin/toolbench_api_tool_use_current_kernel.rs
src/bin/cybench_security_sandbox_current_kernel.rs
src/bin/gaia_general_assistant_current_kernel.rs
src/bin/gpqa_science_reasoning_current_kernel.rs
src/bin/math_competition_reasoning_current_kernel.rs
```

Classification: benchmark scaffolding / workload adapters.

Risk class: Class 2 if they remain external binaries that generate evidence.
Escalate if they mutate canonical admission, typed tx schema, trust root, or
sequencer logic.

Acceptance criteria:

```text
Each adapter is explicit about whether it is real verifier-backed, structural
smoke, or participation canary.
No adapter can upgrade a task to accepted L4 state without predicate evidence.
No adapter leaks hidden tests or private diagnostics into agent prompts.
```

Gate:

```bash
cargo test --test constitution_production_module_liveness every_exported_module_has_exactly_one_liveness_group --no-fail-fast -- --test-threads=1
cargo test --test cli_benchmark_full_flow --no-fail-fast -- --test-threads=1
```

Merge condition:

Architect accepts the adapter taxonomy and the report language does not imply
task success where only node participation was proven.

### PR 3: Boot Trust Root Manifest

Purpose:

Add boot-time trust-root manifest checks for constitution hash, trust-root
payload hashes, predicate hashes, and ref contract visibility.

Candidate files:

```text
src/bin/turingos/cmd_boot.rs
src/runtime/boot_trust_root_manifest.rs
tests/constitution_tc_boot_trust_root_manifest.rs
build.rs
genesis_payload.toml
```

Classification: core runtime / trust-root.

Risk class: Class 3 by default; treat `build.rs` and `genesis_payload.toml` as
trust-root-adjacent. Escalate to Class 4 if any constitution/flowchart authority
or canonical signing payload is changed.

Acceptance criteria:

```text
turingos boot --verify-manifest passes on the valid fixture.
turingos boot --verify-constitution-hash passes on the valid fixture.
turingos boot --verify-predicates passes on the valid fixture.
At least one SHA mismatch fixture fails closed.
No boot check can be disabled silently to make CI green.
```

Gate:

```bash
cargo test --test constitution_tc_boot_trust_root_manifest --no-fail-fast -- --test-threads=1
bash scripts/run_constitution_gates.sh
```

Merge condition:

Clean-context auditor validates that no mutable derived view becomes a trust
root and that the failure mode is closed.

### PR 4: Path B Git Tape / ChainTape Hardening

Purpose:

Harden authority ref movement and resume identity around the Git-backed tape.
This is core substrate work and should be isolated from benchmark adapters.

Candidate files:

```text
src/git_tape_ledger.rs
tests/tc_git_tape_ledger_hardening.rs
tests/git_tape_ledger_roundtrip.rs
```

Possible supporting file if touched by the split:

```text
src/bottom_white/ledger/transition_ledger.rs
```

Classification: core runtime / substrate.

Risk class: Class 3. Escalate if sequencer admission, typed tx wire schema, or
canonical signing payloads are touched.

Acceptance criteria:

```text
Authoritative ref movement errors are not swallowed with ignored `let _`.
Reopen append resumes as tn-N+1.
Accepted and rejected heads reconstruct independently.
git fsck --full passes on generated test repositories.
```

Gate:

```bash
cargo test --test tc_git_tape_ledger_hardening --no-fail-fast -- --test-threads=1
cargo test --test git_tape_ledger_roundtrip --no-fail-fast -- --test-threads=1
git fsck --full
```

Merge condition:

Independent audit confirms no source of truth moved outside ChainTape/CAS and
no ref failure can be mistaken for success.

### PR 5: Tape-Canonical Facts and External Call Outbox

Purpose:

Add the generic durable side-effect model: Intent followed by exactly one
Result, Failure, or Abandoned terminal. This is the path toward universal
capability accounting without putting provider price or model brand into the
kernel.

Candidate files:

```text
src/runtime/tc_tape_canonical.rs
src/runtime/external_call.rs
src/drivers/llm_http.rs
tests/tc_tape_canonical_repairs.rs
tests/tc_external_call_records.rs
```

Classification: core runtime support / side-effect gateway.

Risk class: Class 2 or Class 3 depending on whether provider calls become
production evidence. Treat as Class 3 for merge discipline.

Key source excerpt:

```rust
pub struct ExternalCallIntent {
    pub intent_id: String,
    pub logical_call_id: String,
    pub call_site: String,
    pub run_id: String,
    pub request_hash: String,
    pub provider: String,
    pub model: Option<String>,
    pub redacted_request_cid: String,
    pub idempotency_key: String,
    pub timeout_ms: u64,
    pub logical_t: u64,
}

pub enum ExternalCallTerminal {
    Result { result_hash: String, usage: Usage, status: u16, provider_request_id: Option<String> },
    Failure { class: String, retryable: bool, public_summary: String },
    Abandoned { reason: String, may_have_spent: bool },
}
```

Acceptance criteria:

```text
Intent count == Result + Failure + Abandoned for clean claim.
Pending intents fail clean halt.
Crash states map to deterministic terminal states.
Replay does not call network or LLM.
Provider brand and price remain outside kernel accounting; token counts are the
kernel budget primitive.
```

Gate:

```bash
cargo test --test tc_external_call_records --no-fail-fast -- --test-threads=1
cargo test --test tc_tape_canonical_repairs --no-fail-fast -- --test-threads=1
cargo test distiller_in_budget --lib --no-fail-fast
```

Merge condition:

Reliability auditor confirms the outbox is durable and no unresolved `Pending`
can be reported as clean.

### PR 6: Agent View Shielding

Purpose:

Ensure agent prompt views are scoped, reconstructable, and shielded from raw
stderr, hidden theorem bodies, private diagnostics, and benchmark leakage.

Candidate files:

```text
src/runtime/tc_agent_view.rs
tests/tc_agent_view_shielding.rs
tests/hidden_oracle_not_in_generation_prompt_bytes.rs
tests/hidden_oracle_set_cid_not_in_build_session_view.rs
tests/build_session_view_does_not_expose_private_diagnostic_cid.rs
```

Classification: runtime security / prompt shielding.

Risk class: Class 2, potentially Class 3 if prompt leakage would affect
production evidence or benchmark validity.

Acceptance criteria:

```text
Raw Lean stderr and private diagnostics are absent from ordinary agent views.
Hidden verifier/test bodies do not leak into route/proof prompts.
Shielding tests include positive controls that would fail if raw text leaked.
```

Gate:

```bash
cargo test --test tc_agent_view_shielding --no-fail-fast -- --test-threads=1
cargo test --test hidden_oracle_not_in_generation_prompt_bytes --no-fail-fast
cargo test --test hidden_oracle_set_cid_not_in_build_session_view --no-fail-fast
```

Merge condition:

Security or data-integrity auditor confirms no new global prompt view becomes
an unscoped derived source of truth.

### PR 7: Lean Micro-State Workload Adapter

Purpose:

Add Lean step-state structures and fixture stepping while keeping Lean outside
the kernel. Lean is a workload/verifier layer. It must not become the TuringOS
kernel or a privileged truth source.

Candidate files:

```text
src/judges/lean_micro_state.rs
src/judges/lean_judge.rs
src/judges/mod.rs
tests/tc_lean_micro_state_contract.rs
```

Classification: workload adapter / verifier integration.

Risk class: Class 2 unless it touches kernel authority.

Key source excerpt:

```rust
pub struct GoalState {
    pub theorem_id: String,
    pub state_id: String,
    pub parent_state_id: Option<String>,
    pub goals: Vec<GoalView>,
    pub imports_hash: String,
    pub preamble_hash: String,
    pub lean_version: String,
    pub mathlib_rev: Option<String>,
}

pub enum LeanStepOutcome {
    Advanced { next: GoalState },
    Complete { proof_script: String },
    Failed { class: LeanStepError, feedback: String },
    Timeout,
    Rejected { class: CleanlinessReject },
}
```

Acceptance criteria:

```text
No `Verified` state exists before final LeanJudge recertification.
intro/simp fixture advances or completes.
Backtracking works through state ids.
Raw stderr is not exposed in public prompt view.
Final proof acceptance remains through LeanJudge and axiom report.
```

Gate:

```bash
cargo test --test tc_lean_micro_state_contract --no-fail-fast -- --test-threads=1
cargo test lean_micro_state --lib --no-fail-fast
cargo test lean_judge --lib --no-fail-fast
```

Merge condition:

Formal-methods auditor confirms Lean is not represented as kernel authority and
that proof finality remains predicate/verifier-gated.

### PR 8: G0 Completeness, Fair Dovetail, Queue Isolation, Universal Witnesses

Purpose:

Add bounded G0 completeness scaffolding, strict even/odd scheduler traces,
queue-isolation tests, crash matrix, and universal-machine witnesses. This
supports bounded search completeness claims, not broad "PROVEN TC" claims.

Candidate files:

```text
src/runtime/g0_completeness.rs
src/runtime/tc_universal_witness.rs
src/runtime/tc_crash_matrix.rs
tests/tc_g0_completeness.rs
tests/tc_universal_witnesses.rs
tests/tc_crash_matrix.rs
tests/tc_difficulty_ladder.rs
tests/tc_prereg_parity.rs
```

Classification: benchmark / search runtime scaffolding.

Risk class: Class 2. Escalate if scheduler or sequencer admission semantics are
changed in production paths.

Key source excerpt:

```rust
const BLOCKED_ATOMS: &[&str] = &[
    "native_decide",
    "decide",
    "omega",
    "sorry",
    "admit",
    "aesop",
    "simp_all",
];

pub enum Lane {
    EvenEnumerator,
    OddHeuristic,
}
```

Acceptance criteria:

```text
Known-rank corpus count is exact.
Witness index i is attempted by even tick 2*i.
Market on/off/shuffled produces byte-identical even-lane trace.
Poisoned odd queue cannot skip, pop, reorder, or mask even candidates.
Universal witness replay is byte-identical and tamper tests fail as expected.
```

Gate:

```bash
cargo test --test tc_g0_completeness --no-fail-fast -- --test-threads=1
cargo test --test tc_universal_witnesses --no-fail-fast -- --test-threads=1
cargo test --test tc_crash_matrix --no-fail-fast -- --test-threads=1
cargo test --test tc_difficulty_ladder --no-fail-fast
cargo test --test tc_prereg_parity --no-fail-fast
```

Merge condition:

Formal-methods auditor confirms the claim remains bounded G0 completeness, not
unbounded theorem-proving or AGI capability.

### PR 9: TC Task Packets, Dirty-Tree Preservation, Audit Export

Purpose:

Preserve the large TC operationalization plan, low-reasoning task packets,
dirty-tree quarry manifest, and audit-packet export helpers without mixing them
into runtime PRs.

Candidate files:

```text
handover/directives/TC_OPERATIONALIZATION_FULL_EXECUTION_PLAN_2026-06-04.md
handover/directives/TC_000_PATH_B_DECISION.md
handover/directives/TC_001_VETO_AI_SCOPE_LOCK.md
handover/directives/TC_002_BOOT_TRUST_ROOT_MANIFEST.md
handover/directives/tc_taskpackets_2026-06-04/**
handover/directives/tc_ladder_2026-06-04/**
handover/directives/tc_prereg_2026-06-04/**
handover/reports/TC_Q_DIRTY_TREE_PRESERVATION_2026-06-04.yaml
handover/reports/TC_CLEAN_CHECKOUT_REPLAY_2026-06-04.md
handover/reports/TC_FULL_AUDIT_PACKET_MANIFEST_2026-06-04.md
scripts/export_tc_audit_packet.sh
scripts/tc_clean_checkout_replay.sh
tests/tc_dirty_tree_preservation_manifest.rs
tests/tc_clean_checkout_replay_contract.rs
tests/tc_audit_packet_export.rs
tests/tc_operationalization_docs.rs
```

Classification: plans, orchestration, audit packaging, evidence preservation.

Risk class: Class 0 for docs, Class 1/2 for scripts/tests.

Acceptance criteria:

```text
Dirty worktree preservation manifest validates.
No historical dirty branch becomes the new constitutional base.
Clean checkout replay script uses a clean base and produces fresh evidence.
Audit packet exporter refuses missing required evidence.
```

Gate:

```bash
cargo test --test tc_dirty_tree_preservation_manifest --no-fail-fast
cargo test --test tc_clean_checkout_replay_contract --no-fail-fast
cargo test --test tc_audit_packet_export --no-fail-fast
cargo test --test tc_operationalization_docs --no-fail-fast
```

Merge condition:

Auditor agrees the documents are guidance/evidence and do not silently ratify
Class 4 changes.

## Items That Should Not Be Included in Any First PR

Do not include as ordinary PR content:

```text
.DS_Store
large historical P1 evidence residues from /Users/zephryj/work/turingosv4
unreviewed raw benchmark logs not referenced by a packet manifest
old reports that make stronger claims than the current evidence supports
any local credential, provider key, or raw authorization header
```

Do not merge any source change that exists only to make a benchmark pass
temporarily. If a bug is found, repair the system mechanism and rerun the
evidence path.

## Current Shortcomings and Open Risks

### 1. Monolithic diff is not reviewable

The current 178 tracked modified files plus 99 untracked files make it hard for
an auditor to distinguish runtime semantics from scaffolding and reports.

Required fix:

Split PRs before opening anything non-draft.

### 2. Real task remains unresolved

The SWE-bench task was real and hidden-test verified, but it did not pass.

Current evidence:

```text
TASK-FAIL/unresolved
stages_completed=0/1
total_attempts=3
remaining failures:
  tests/test_cli.py::TestRoutes::test_subdomain
  tests/test_cli.py::TestRoutes::test_host
```

Required fix:

Future benchmark packet for a stronger claim must include at least one real
domain task with `TASK-PASS`, or must explicitly remain a flow/system packet.

### 3. Market participation is canary anchored

Market participation was real and on tape, but not attached to an accepted
SWE-bench domain `WorkTx`.

Required fix:

Next benchmark should exercise market participation on a domain task path. If
the domain task is rejected, the report may still show full-system canary
coverage, but cannot claim market-backed task acceptance.

### 4. PR evidence must be regenerated after split

The recorded gates passed on the active monolithic worktree. After splitting,
each PR has a different diff and must rerun its own targeted and broad gates.

Required fix:

Each PR body should include:

```text
exact staged file list
risk class
touched FC nodes
targeted test output
constitution gate output if runtime/evidence-bearing
clean-context audit verdict for Class 2+ shipping paths
```

### 5. Kernel boundary must stay generic

Lean, SWE-bench, OSWorld, WebArena, and market price tables are workload or
derived-economic layers. They must not become kernel authority.

Current correct design constraint:

```text
kernel budget = prompt tokens, completion tokens, total tokens, stage count,
attempt count, timeout/verifier limits where generic.

provider brand, provider channel, per-token price, market PnL, currency
conversion = non-kernel derived accounting.
```

Required fix:

Reject any PR that puts Lean-specific proof state, SWE-bench-specific semantics,
or provider price into kernel admission logic.

### 6. Inherited JudgeAI concern is not closed by this work

The clean-context auditor recorded an inherited, non-OBL-015 concern around
JudgeAI tactic calibration and Bus-adjacent oracle acceptance paths. It was
explicitly out of scope for the benchmark packet.

Required fix:

If reopened, handle as a separate restricted-surface governance/JudgeAI task.
Do not bury it inside benchmark scaffolding.

### 7. Broad claim/secret grep is noisy

The benchmark report notes that broad changed-file scans are noisy because tests
and historical reports intentionally mention forbidden claim words, credential
environment variable names, and raw-stderr shielding terms.

Required fix:

Per PR, run focused greps and classify every hit:

```bash
grep -RInE 'PROVEN|DEFINITIVE|causal|isolated lever|X > Y' handover src tests
grep -RInE 'raw.*stderr|Lean.*stderr|api[_-]?key|Authorization|Bearer' handover src tests
```

Any actual secret, hidden verifier leakage, or unsupported headline blocks the
PR.

## Suggested Review Order

Recommended order:

1. PR 0: reports/audit decision packet.
2. PR 1: full-flow benchmark CLI.
3. PR 2: current-kernel benchmark adapters.
4. PR 3: boot trust-root manifest.
5. PR 4: Git tape hardening.
6. PR 5: tape-canonical facts and external-call outbox.
7. PR 6: agent view shielding.
8. PR 7: Lean micro-state workload adapter.
9. PR 8: G0 completeness and universal witnesses.
10. PR 9: TC task packets and audit export.

Reasoning:

PR 0 and PR 1 preserve the most recent evidence and expose the CLI entrypoint.
PR 2 then separates workload adapter scaffolding from kernel work. PR 3-5 cover
trust, substrate, and side-effect durability. PR 6-8 cover safety and bounded
search claims. PR 9 preserves plans and large audit scaffolding without mixing
them into runtime semantics.

## Decision Questions for Architect / Auditor

1. Should the first code PR be the full-flow benchmark CLI, or should the first
   PR be docs-only evidence preservation?
2. Is `build.rs` / `genesis_payload.toml` use in the boot trust-root PR Class 3
   enough, or does the architect require explicit Class 4 ratification?
3. Should the canary market path remain in PR 1 as a disclosed participation
   mechanism, or be withheld until a domain-anchored market benchmark is ready?
4. Should Lean micro-state land before or after G0 completeness, given the
   kernel-boundary rule that Lean is a workload/verifier layer only?
5. Which real-world task should be the next domain-anchored benchmark target:
   the same `pallets__flask-5063` task, a smaller SWE-bench task, or a different
   OSWorld/WebArena/ToolBench-style task?

## Final Recommendation

Open no monolithic PR.

Authorize a split pass. The safest first step is PR 0 plus PR 1:

```text
PR 0: decision packet and audit reports only.
PR 1: turingos benchmark full-flow CLI and minimal tests.
```

Do not claim TuringOS solved a real SWE-bench task until a future packet records
`TASK-PASS` from the real task verifier. Do not claim market-backed task success
until market participation is attached to an accepted domain task rather than a
system-participation canary.

The current work is valuable because it created a reproducible evidence path
and exposed the exact gaps. Its next value comes from being split into small
reviewable PRs, not from being merged as one large change.
