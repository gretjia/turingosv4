# TuringOS v4 Experiment Guide

## Principle
Each experiment is an independent Cargo project importing the Core SDK.
Never mix code between experiments.

## Boot Script
```bash
./scripts/boot-experiment.sh <project_name> <theorem_name> <lean_problem_file>
```

## Environment Variables
- `SILICONFLOW_API_KEY` — Primary SiliconFlow
- `SILICONFLOW_API_KEY_SECONDARY` — Secondary SF (separate rate limits)
- `DEEPSEEK_API_KEY` — DeepSeek official

## Workflow
```
Human → problem description
  → LLM → Lean 4 formalization (only step requiring intelligence)
  → Human confirms spec
  → boot-experiment.sh (fully automated)
  → Swarm runs autonomously
  → Monitor: tail -f /tmp/<project>_run1.log
```

## WAL Preservation
Boot script preserves WAL files across runs for cross-epoch knowledge inheritance.
Critical: persist run tapes BEFORE next experiment (/tmp/ is ephemeral).

## Key Results (v3)
- zeta_sum_proof: OMEGA in 8 tx, 4-step proof, ~5 min
- zeta_regularization: OMEGA in 51 min, Step 12
- number_theory_min: OMEGA via `decide` tactic
