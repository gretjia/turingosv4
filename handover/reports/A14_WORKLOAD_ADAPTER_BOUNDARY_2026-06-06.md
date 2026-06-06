# A14 Workload Adapter Boundary Report

Date: 2026-06-06
Risk: Class 2
Parent plan: `handover/directives/2026-06-05_AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT.md`
Atom: A14 Workload Adapters, Market Research, and Benchmark Boundary

## Scope

This report records the A14 boundary implementation. Workload adapters are user
workload evidence paths. They do not create kernel authority, sequencer
authority, wallet authority, trust-root authority, or benchmark headline
authority.

## Boundary

- Adapter summaries must point at an evidence manifest.
- Structural smoke and participation canary outputs cannot be reported as
  verifier-backed benchmark success.
- Market research headlines require preregistered evidence, declared sample
  size, equalized budgets, ablations, tape-visible route decisions, replay
  command, hidden-verifier shielding, and clean-context audit.
- Raw evidence remains outside this report and is not dumped into main.

## Non-Claims

This atom does not claim OBL-005 final closure, benchmark capability closure,
market performance superiority, public launch readiness, FC3 closure, or any
provider-backed experiment result.

## Trust-Root Landing Note

The `src/lib.rs` trust-root pin is updated because this atom exposes
`pub mod workloads;` with a TRACE_MATRIX FC3 backlink. This keeps boot
verification fail-closed on the current source bytes. The workload module is
registered as a liveness-accounted product workload, not as kernel,
sequencer, settlement, or benchmark headline authority.

## Verification

- `cargo test --test workload_adapter_claim_boundary --test market_preregistration_contract --no-fail-fast -- --test-threads=1`
  exited 0; 10 tests passed.
- `cargo test --test constitution_benchmark_manifest --test constitution_real11_claim_boundary --test constitution_market_autonomy_research_envelope --test constitution_matrix_drift --no-fail-fast`
  exited 0.
- `bash scripts/run_constitution_gates.sh` exited 0 with
  `[k-1-5] total=167 failed=0`.
- `cargo test --workspace --no-fail-fast` exited 0.
- `git diff --check` exited 0.
- Targeted `rustfmt --edition 2021 --check` over the A14 Rust files exited 0.
- A14-owned claim scan over `src/workloads` and this report had no hits for the
  forbidden headline terms.
- `sha256sum src/lib.rs` equals
  `30a7feaadaa4fe41009e3b31d948f5c18b61eba2a37670bc1242f70e6b84fbf2`, matching
  `genesis_payload.toml`.
- Clean-context audit:
  `handover/audits/A14_WORKLOAD_ADAPTER_BOUNDARY_CLEAN_CONTEXT_AUDIT_2026-06-06.md`
  verdict `NO-VIOLATION`.
- After adding the audit artifact, `git diff --check` and the A14-owned
  headline claim scan still exited 0/no output. A first post-audit targeted
  test command hit a linker resource fault (`signal 7`); the same target set was
  rerun with `-j1` and exited 0.
