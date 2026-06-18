# H-HET-1 Preregistration Template

**NO prereg → NO multi-hour/multi-vendor/paid run (architect audit Gate H)**

---

## Hypothesis
State the primary scientific hypothesis being tested; what specific market-outcome difference are you expecting to observe between agent types?

## Theorem Set
List all theorems, lemmas, or proof targets that must be derived/verified; include reference to the Lean proof bank or external verification method.

## Goldilocks Difficulty Band
Describe how this theorem set was selected to fall into the "not-too-easy, not-too-hard" regime (deepseek-v4-pro fails solo, but system-with-heterogeneous-agents solves); cite the selection criterion and evidence from pilot runs.

## Baseline
Specify market structure: homogeneous (single agent type, single vendor) or heterogeneous (multiple agent types from different vendors); justify choice.

## Agent / Model Roster
List exactly 4 model vendors and their configurations:
1. **deepseek-ai/DeepSeek-V4-Pro** — [version/config/inference params]
2. **Qwen/Qwen3-32B** — [version/config/inference params]
3. **zai-org/GLM-4.5-Air** — [version/config/inference params]
4. **Qwen/Qwen3.5-397B-A17B** — [version/config/inference params]

## Budget Cap
Specify maximum micro-USD spend; must be based on token-count pilot + 2× safety margin; no overages permitted.

## Stopping Rule
Define the precise condition that terminates the experiment: {success threshold | iteration limit | timeout | market saturation | first vendor bankruptcy}.

## Primary Metric
Single scalar outcome that directly tests the hypothesis; must be mechanically measurable (e.g., theorems_solved, avg_cost_per_theorem, latency_p99).

## Secondary Metrics
List 1–3 supporting metrics (e.g., cost-per-success, vendor failure rate, proof-quality score).

## Exclusion Rule
Define which runs, agents, or results are invalid and excluded from analysis (e.g., timeout > 1h, parse_error_count > 3, vendor_unreachable for > 10 min).

## Commit Hash (Frozen)
Git commit SHA of the harness / agent code / tape-generator being tested; must match the deployed binary exactly.

## Tape / WAL Hash
SHA256 digest of the frozen workload-allocation log (WAL) or deterministic tape generator seed; ensures reproducibility and audit trail.

---

**Signed off by Architect (Gate H approved):** ___________________  
**Date:** ___________________
