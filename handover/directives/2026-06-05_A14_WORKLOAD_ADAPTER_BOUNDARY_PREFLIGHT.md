# A14 Workload Adapter Boundary Preflight

Date: 2026-06-05

Parent plan:
`handover/directives/2026-06-05_AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT.md`

Atom: A14. Workload Adapters, Market Research, and Benchmark Boundary

Document role: Class 0 preflight. This document does not authorize benchmark
headline claims, real market experiments, provider spending, economy mutation,
kernel authority, or workload-to-kernel promotion by itself.

## Decision

A14 is a workload boundary atom, not a kernel authority atom. The repo has many
older market, Polymarket, MiniF2F, and benchmark-adjacent witnesses, but the
parent-plan A14 adapter paths and tests are missing. A14 must prevent workload
adapters from becoming evidence claims or OS authority before A13 exists.

Safe work now:

- docs-only preflight
- existing market/benchmark witness inventory
- adapter/result classification contract
- preregistration and claim-boundary correction

Blocked until A13 exists:

- running workload adapters as OS substrate evidence
- claiming solve-rate, benchmark success, or market victory
- treating Polymarket/public price as truth or kernel budget authority
- moving workload code into sequencer/kernel/trust-root paths
- dumping raw evidence into main instead of hash-indexed reports/manifests

## Current-State Facts

Parent-plan A14 allowed paths:

```text
experiments/market_p1_lean_router/
src/workloads/lean/
src/workloads/swebench/
tests/workload_adapter_claim_boundary.rs
tests/market_preregistration_contract.rs
handover/reports/<scoped-market-or-benchmark-report>.md
```

Corrected implementation path inventory:

```text
experiments/market_p1_lean_router/
experiments/minif2f_v4/
src/workloads/mod.rs
src/workloads/lean/
src/workloads/swebench/
src/workloads/market_research/
src/workloads/benchmark_boundary.rs
src/runtime/benchmark_manifest.rs
tests/workload_adapter_claim_boundary.rs
tests/market_preregistration_contract.rs
tests/constitution_benchmark_manifest.rs
tests/constitution_market_autonomy_research_envelope.rs
tests/constitution_real11_claim_boundary.rs
tests/constitution_real8_market_ab_benchmark.rs
tests/realworld_polymarket_paper_runner_test.py
scripts/run_realworld_polymarket_paper.py
scripts/run_real8_market_ab_benchmark.sh
handover/directives/market_autonomy_lab/
handover/reports/<scoped-market-or-benchmark-report>.md
```

Write-scope guidance:

```text
src/workloads/*
  workload adapters only; no kernel authority
experiments/*
  experiment/workload quarry only; not production source of truth
handover/reports/*
  derived reports only; no raw evidence blobs and no headline claims
src/state/sequencer.rs
src/state/typed_tx.rs
src/bottom_white/cas/schema.rs
src/kernel.rs
src/bus.rs
src/sdk/tools/wallet.rs
  out of A14 write scope unless separately ratified
```

Existence check:

```text
MISSING experiments/market_p1_lean_router/
MISSING src/workloads/
MISSING src/workloads/lean/
MISSING src/workloads/swebench/
MISSING src/workloads/market_research/
MISSING src/workloads/benchmark_boundary.rs
MISSING tests/workload_adapter_claim_boundary.rs
MISSING tests/market_preregistration_contract.rs
EXISTS experiments/minif2f_v4/
EXISTS src/runtime/benchmark_manifest.rs
EXISTS tests/constitution_benchmark_manifest.rs
EXISTS tests/constitution_market_autonomy_research_envelope.rs
EXISTS tests/constitution_real11_claim_boundary.rs
EXISTS tests/constitution_real8_market_ab_benchmark.rs
EXISTS tests/realworld_polymarket_paper_runner_test.py
EXISTS scripts/run_real8_market_ab_benchmark.sh
EXISTS handover/directives/market_autonomy_lab/
```

Dirty-path check for relevant inventory:

```text
pre-existing dirty paths include:
  handover/reports/P1_REALVALUE_SCOPE_CORRECTION_2026-06-05.md
  handover/reports/PHASE_E_REAL_VALIDATION_2026-05-23.md
  src/state/sequencer.rs
  src/state/typed_tx.rs

Implementation must read and preserve those edits. Do not overwrite them as
part of A14 scaffolding.
```

Existing workload/market/claim-boundary witnesses:

```text
tests/constitution_benchmark_manifest.rs:1
  BenchmarkManifest schema is load-bearing for scaled benchmark batches
tests/constitution_benchmark_manifest.rs:39
  well-formed manifest validates
tests/constitution_benchmark_manifest.rs:63
  missing required fields are ship-blocks
tests/constitution_market_autonomy_research_envelope.rs:1
  market autonomy envelope gates are research-only
tests/constitution_market_autonomy_research_envelope.rs:13
  envelope declares research-only, not ship authorization
tests/constitution_market_autonomy_research_envelope.rs:31
  envelope lists allowed and forbidden surfaces
tests/constitution_market_autonomy_research_envelope.rs:49
  forbidden surfaces include constitution, sequencer, typed_tx, CAS schema,
  kernel, bus, wallet, signing payload, and CAS ObjectType schema
tests/constitution_real11_claim_boundary.rs:4
  forbidden launch/beta/market-proven claims are absent or explicitly forbidden
tests/constitution_real8_market_ab_benchmark.rs:1
  older Formal Market A/B Benchmark gates pin a runner contract before claims
src/runtime/benchmark_manifest.rs:1
  BenchmarkManifest pins scaled benchmark run fields before a batch
src/runtime/benchmark_manifest.rs:7
  missing fields are ship-blocks
scripts/run_real8_market_ab_benchmark.sh:10
  runner is descriptive evidence only and forbids causal overclaim
tests/realworld_polymarket_paper_runner_test.py:63
  realworld runner expects DeepSeek v4 pro thinking-on payload
tests/realworld_polymarket_paper_runner_test.py:78
  public prompt is snapshot-only and orderless
tests/realworld_polymarket_paper_runner_test.py:101
  external market price is signal, not truth
tests/realworld_polymarket_paper_runner_test.py:109
  public summary rejects private or truth leakage
handover/reports/REAL17P21_P22_POLYMARKET_ROBUSTNESS_PIVOT_SUMMARY.md:7
  robustness pivot is not voluntary-emergence claim strengthening
handover/reports/REAL17P21_P22_POLYMARKET_ROBUSTNESS_PIVOT_SUMMARY.md:14
  forced Bull/Bear is red-track positive control, not E2/E3/E4 evidence
experiments/minif2f_v4/src/lib.rs:1
  minif2f_v4 is partial restoration, lean_market binary only
experiments/minif2f_v4/src/bin/lean_market.rs:45
  lean_market has run-task/view-task/view-wallet/view-replay commands
```

Non-closure facts:

```text
No current src/workloads module tree.
No current workload_adapter_claim_boundary test.
No current market_preregistration_contract test.
Existing market/research reports are derived views, not OS substrate evidence.
Existing realworld Polymarket runner tests are paper/adapter boundary tests, not
A14 workload contract.
```

## Risk Classification

Risk floor: Class 2 for workload adapters, claim-boundary tests, and
preregistration documents.

Promote to Class 3 if:

- real provider calls or paid API calls run
- real budget/economy experiments run
- market settlement or wallet state is exercised as evidence
- reports are used for public headline claims
- hidden verifier/oracle shielding is part of the workload run

Promote to Class 4 if:

- workload code changes kernel/sequencer/trust-root authority
- typed tx schema or discriminants change
- canonical signing payload changes
- CAS ObjectType schema changes
- constitution / flowchart authority changes

## Recommended Contract

Workload adapter result:

```text
WorkloadAdapterResult {
  workload_id: String,
  run_id: RunId,
  adapter_kind: WorkloadAdapterKind,
  evidence_manifest_cid: Cid,
  result_classification: AdapterResultClassification,
  verifier_backed_task_pass_count: u64,
  structural_smoke_count: u64,
  participation_canary_count: u64,
  unsupported_claim_count: u64,
}

AdapterResultClassification =
  RealVerifierBacked
  StructuralSmoke
  ParticipationCanary
```

Market preregistration:

```text
MarketPreregistration {
  track: A | B | C | D,
  hypothesis: String,
  mde: String,
  sample_size: u64,
  budget_equalization: String,
  ablations: Vec<String>,
  hidden_verifier_shielding: String,
  route_decision_tape_policy: String,
  replay_command: String,
  headline_claim_allowed: bool,
}
```

Required invariant:

```text
No TASK-PASS without verifier-backed TASK-PASS.
No market victory headline without preregistered evidence.
Benchmark adapter is user workload, not kernel authority.
Raw evidence is archived by hash manifest, not dumped into main.
Public price is signal, not truth.
```

## Atomized A14 Tasks

### A14.0 Preflight Lock

Description:
Record missing workload paths, existing market/benchmark witnesses, claim
boundaries, preregistration needs, and risk boundaries.

Acceptance:

```bash
for f in \
  handover/directives/2026-06-05_A14_WORKLOAD_ADAPTER_BOUNDARY_PREFLIGHT.md \
  handover/directives/2026-06-05_AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT.md
do
  git diff --no-index --check /dev/null "$f" || true
done
```

Expected:

```text
no whitespace errors
```

### A14.1 Adapter Claim Boundary

Description:
Add executable tests that force every adapter output into one of the allowed
classification buckets and prevent unsupported TASK-PASS or PROVEN language.

Primary paths:

```text
src/workloads/mod.rs
src/workloads/benchmark_boundary.rs
tests/workload_adapter_claim_boundary.rs
tests/constitution_benchmark_manifest.rs
tests/constitution_real11_claim_boundary.rs
```

Acceptance:

```bash
cargo test --test workload_adapter_claim_boundary --no-fail-fast -- --test-threads=1
cargo test --test constitution_benchmark_manifest --no-fail-fast
cargo test --test constitution_real11_claim_boundary --no-fail-fast
```

Expected:

```text
No TASK-PASS without verifier-backed TASK-PASS.
No PROVEN/DEFINITIVE/market-beats claim without explicit preregistered evidence.
```

### A14.2 Market Preregistration Contract

Description:
Add market research preregistration checks before any Track A/B/C/D headline
claim or real-budget experiment.

Primary paths:

```text
tests/market_preregistration_contract.rs
handover/directives/market_autonomy_lab/
handover/reports/<scoped-market-or-benchmark-report>.md
```

Acceptance:

```bash
cargo test --test market_preregistration_contract --no-fail-fast -- --test-threads=1
cargo test --test constitution_market_autonomy_research_envelope --no-fail-fast
```

Expected:

```text
Every track declares MDE, sample size, ablations, equal budget, route decisions
on tape, replay, hidden verifier shielding, and clean-context audit requirement.
```

### A14.3 Lean And SWE-Bench Workload Adapters

Description:
After A13 exists, add workload adapters under `src/workloads/` without changing
kernel authority.

Primary paths:

```text
src/workloads/lean/
src/workloads/swebench/
tests/workload_adapter_claim_boundary.rs
```

Acceptance:

```bash
cargo test --test workload_adapter_claim_boundary --no-fail-fast -- --test-threads=1
git diff --check
```

Expected:

```text
adapter outputs are workload evidence only.
kernel/sequencer/trust-root files remain untouched.
```

## Full A14 Acceptance

After A13 exists and A14 implementation is complete:

```bash
cargo test --test workload_adapter_claim_boundary --no-fail-fast -- --test-threads=1
cargo test --test market_preregistration_contract --no-fail-fast -- --test-threads=1
cargo test --test constitution_benchmark_manifest --no-fail-fast
cargo test --test constitution_market_autonomy_research_envelope --no-fail-fast
cargo test --test constitution_real11_claim_boundary --no-fail-fast
cargo test --test constitution_matrix_drift --no-fail-fast
git diff --check
if grep -RInE 'TASK-PASS|PROVEN|DEFINITIVE|causal|market beats' \
  experiments src/workloads handover/reports; then
  echo 'strong workload claim text requires verifier-backed allowlist gate'
  exit 1
fi
```

Expected:

```text
PREDICATES-GREEN
No TASK-PASS without verifier-backed TASK-PASS.
No market victory headline without preregistered evidence.
Raw evidence is archived by hash manifest, not dumped into main.
Benchmark adapters remain user workload paths, not kernel authority paths.
```

## Hard Blockers

```text
A14-IMPLEMENTABLE-AFTER-A13
```

Hard blockers:

- A13 OS E2E does not exist yet.
- Parent-plan A14 adapter paths/tests are missing.
- Existing market and Polymarket files are derived reports/experiments, not
  A14 adapter authority.
- No headline market/benchmark claims may be made without preregistered,
  verifier-backed evidence.
- Workload code must not touch restricted authority surfaces unless separately
  ratified.

Clean-context audit input for a future implementation PR:

```text
Task brief: A14 Workload Adapters, Market Research, and Benchmark Boundary.
Risk class: Class 2 adapters; Class 3 if real provider/budget/economy
experiments run; Class 4 if restricted authority surfaces are touched.
FC nodes: L8 workload adapter boundary, L9 evidence/report boundary, FC3 audit.
Evidence: A13 predecessor evidence, A14 tests, claim-boundary grep output,
market preregistration artifacts.
Verdict domain: NO-VIOLATION | VIOLATION-FOUND | RECONSTRUCTION-FAILURE |
SECOND-SOURCE-DRIFT
```
