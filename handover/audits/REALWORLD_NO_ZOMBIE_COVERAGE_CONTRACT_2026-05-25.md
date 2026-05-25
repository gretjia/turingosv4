# Real-World No-Zombie Coverage Contract

Date: 2026-05-25

Authority: `constitution.md` + fresh current-kernel ChainTape/CAS evidence.
Derived contract: `tests/fixtures/liveness/realworld_liveness_coverage.toml`.

This artifact does not close OBL-005. It defines the evidence shape required to
close it. Historical evidence remains candidate-only until a fresh current-main
suite lights every retained production group and every required true-problem
domain.

## Findings

Three read-only researchers inspected the merged #146 liveness manifest,
historical evidence, real-task tests, and runner scripts.

- Credible substrate evidence exists for canonical ChainTape/CAS/replay and
  boot trust-root invariants.
- Weak or missing final evidence remains for FC3 runtime meta-role runs,
  ToolRegistry root/tool-log boundary, non-scripted live market action,
  TDMA verified true-problem success, and product spec/generate ChainTape
  anchoring.
- Reusable practices: pinned problem/model/budget manifests, arm diff
  allowlists, no-forced-trade/no-price-as-truth/no-ghost-liquidity boundaries,
  replay/tamper checks, TDMA bounded prompt guards, and mechanical artifact
  validation.
- Contamination risks: scripted buys, scripted AttemptPrediction, router
  positive controls, session-CAS-only product demos, historical contaminated
  REAL-8X notes, stdout dashboards, and ignored/smoke tests.

## Required True-Problem Domains

| Task | Domain | Required Fresh Evidence |
| --- | --- | --- |
| `market_action_minif2f_fresh` | market/economy | non-scripted ChainTape/CAS market action, conservation, replay |
| `market_ab_performance_fresh` | market/economy benchmark | pinned arm configs, CAS/replay, no causal overclaim |
| `generate_artifact_chain_fresh` | user spec to artifact | ChainTape/CAS anchored spec/generate + artifact bundle CID |
| `tdma_real_proof_fresh` | TDMA/proof | real task attempts, bounded prompts, judge verdicts, replay |
| `fc3_governance_reinit_fresh` | FC3 governance/re-init | LogFeedbackArchive, ArchitectAI, Veto-AI, Reinit typed L4/CAS |
| `replay_cas_tamper_repair_current` | replay/CAS integrity | replay + tamper reports over runtime repo/CAS |
| `boot_cli_current_kernel_fresh` | boot/CLI | fresh workspace boot, genesis report, resume/replay |

## Current Status

`OPEN_REAL_WORLD_COVERAGE_PENDING`.

The next implementation phase must produce the fresh evidence directory named
by the contract. Each final task row must include ChainTape or CAS evidence plus
replay/verifier output; `runtime_repo` snapshots are supplemental only. Until
then, no report may claim that every retained code group is necessary,
sufficient, and live under real-world AGI tasks.
