# Real-World No-Zombie Coverage Contract

Date: 2026-05-25

Authority: `constitution.md` + fresh current-kernel ChainTape/CAS evidence.
Derived contract: `tests/fixtures/liveness/realworld_liveness_coverage.toml`.
Broad AGI family contract:
`tests/fixtures/liveness/broad_agi_true_suite_manifest.toml`.

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
| `market_external_agent_fresh` | market/economy | provider-backed external agent decision, signed ChainTape/CAS market action, conservation, replay |
| `market_ab_performance_fresh` | market/economy benchmark | pinned arm configs, CAS/replay, no causal overclaim |
| `generate_artifact_chain_fresh` | user spec to artifact | ChainTape/CAS anchored spec/generate + artifact bundle CID |
| `tdma_real_proof_fresh` | TDMA/proof | real task attempts, bounded prompts, judge verdicts, durable TDMA tape replay-style hash verification |
| `fc3_governance_reinit_fresh` | FC3 governance/re-init | LogFeedbackArchive, ArchitectAI, Veto-AI, Reinit typed L4/CAS |
| `replay_cas_tamper_repair_current` | replay/CAS integrity | replay + tamper reports over runtime repo/CAS |
| `boot_cli_current_kernel_fresh` | boot/CLI | fresh workspace boot, genesis report, resume/replay |

## Broad AGI Families

The domain contract above is not enough by itself. The broad-suite manifest
adds benchmark-family coverage so the final evidence cannot be satisfied by
only the old 15-question session or by a single product workload. The required
families are GAIA, GPQA, MATH/formal proof, SWE-bench-style coding repair,
WebArena, Mind2Web, OSWorld, ToolBench, Cybench, TuringOS market/economy, and
memory/feedback/re-init. Each family must declare FC trace, risk class, entry
boundary, evidence shape, failure taxonomy, and anti-contamination guards.
Leaderboard score is model capability evidence only; it is never module
liveness proof.

## Current Status

`OPEN_REAL_WORLD_COVERAGE_PENDING`.

## Execution Layer

The first executable true-suite runner is
`scripts/run_true_suite_boot_cli_current_kernel.sh` for
`boot_cli_current_kernel_fresh`. It uses only public/user-facing or current
runtime boot surfaces: `turingos init`, the small
`boot_cli_current_kernel_fresh` helper around `build_chaintape_sequencer`, and
`turingos verify chaintape`. It does not use the historical evaluator,
`lean_market`, scripted market buys, or old TDMA `MemoryTapeLedger` evidence.

The market/economy execution layer adds
`scripts/run_true_suite_market_external_agent.sh` for
`market_external_agent_fresh`. The runner uses a real DeepSeek/SiliconFlow
provider only through the local OpenAI-compatible proxy
(`src/drivers/llm_proxy.py`). The provider-backed agent stays outside the
kernel: its JSON decision is parsed by the runner helper, signed through
`AgentKeypairRegistry`, submitted as `BuyWithCoinRouterTx`, and verified by
public `turingos verify chaintape`. Evidence records only hashes plus parsed
decision/tx metadata, not raw provider prompt or response bytes. This lights
the market/economy domain without importing external-agent simulation into the
kernel or resurrecting old scripted REAL fixtures.

The generate/artifact execution layer adds
`scripts/run_true_suite_generate_artifact_current_kernel.sh` for
`generate_artifact_chain_fresh`. The runner uses the same external LLM proxy
boundary, then drives public CLI surfaces: `turingos spec` creates a CAS spec
capsule, `turingos generate --from-capsule` reads that canonical input and
writes an ArtifactBundleManifest + generated file CIDs to CAS, and accepted
work lands as typed ChainTape entries verified by public
`turingos verify chaintape`. The runner records an `artifact_bundle_cid.json`
index so artifact delivery can be replay-linked without treating DOM/API
smoke output as constitutional evidence.

The TDMA/proof execution layer adds
`scripts/run_true_suite_tdma_current_kernel.sh` for `tdma_real_proof_fresh`.
The runner uses the same external LLM proxy boundary, then drives public
`turingos tdma run` with the durable `GitTapeLedger` backend. This evidence is
deliberately not described as bottom-white L4 ChainTape: TDMA is a bounded
proof-work tape. The runner records `manifest.json`, `chaintape.jsonl`,
`per_attempt_probes.jsonl`, `tdma_tape.git/`, and `replay_report.json`, where
the report verifies stage completion, prompt-budget safety, no raw-stderr
prompt leakage, and the manifest hashes for the tape/probe files.

The replay/CAS integrity execution layer adds
`scripts/run_true_suite_replay_cas_tamper_current_kernel.sh` for
`replay_cas_tamper_repair_current`. The runner creates a fresh workspace
through public `turingos init`, uses the current runtime boot helper to emit
ChainTape/CAS evidence, verifies the original tape through public
`turingos verify chaintape`, runs `audit_tape_tamper` against temporary forks,
and verifies the original tape again after tamper testing. The expected final
evidence includes `replay_report.json`, `tamper_report.json`,
`post_tamper_replay_report.json`, and a run manifest tying those reports to
the current source head.

The next implementation phase must produce the fresh evidence directory named
by the contract. Each final task row must include ChainTape/CAS evidence, or an
explicitly named domain tape for TDMA, plus replay/verifier output;
`runtime_repo` snapshots are supplemental only. Until then, no report may claim
that every retained code group is necessary, sufficient, and live under
real-world AGI tasks.
