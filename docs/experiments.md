# TuringOS v4 Experiment Guide

## Principle
Each experiment is an independent Cargo project importing the Core SDK.
Never mix code between experiments.

## Boot Script
```bash
./scripts/boot-experiment.sh <project_name> <theorem_name> <lean_problem_file>
```

## Environment Variables
- `SILICONFLOW_API_KEY` — Primary SiliconFlow (heterogeneous-LLM provider)
- `SILICONFLOW_API_KEY_SECONDARY` — Secondary SF key (separate rate-limit pool)
- `SILICONFLOW_API_KEY_TERTIARY` — Tertiary SF key (Phase A atom A7 added)
- `DEEPSEEK_API_KEY` — DeepSeek official (Phase B+C single-model backbone)
- `DASHSCOPE_API_KEY` — Aliyun Dashscope (Qwen catalog)
- `VOLCENGINE_API_KEY` — Volcengine Ark (alternate provider)
- `GEMINI_API_KEY` — External audit (Phase A8 dual review)
- `LLM_PROXY_URL` — Default `http://localhost:8080`; evaluator routes here

## LLM Proxy (Phase A atom A7)
The Rust evaluator does NOT call cloud APIs directly (V3L-25: TLS deadlock).
All requests route through `src/drivers/llm_proxy.py` — an OpenAI-compatible
local HTTP server with per-provider multi-key round-robin and token metering.

```bash
# Start proxy (reads keys from .env or shell env)
set -a; . .env; set +a
python3 src/drivers/llm_proxy.py --port 8080

# Provider auto-detected by model id:
#   "deepseek-*"          → deepseek
#   "Qwen/...", "siliconflow:..." → siliconflow
#   "qwen3-*"             → dashscope
# Force with --provider <name>.
```

The three SiliconFlow keys split concurrent traffic across separate
rate-limit pools — V3L-27 (N=30 → 401/429 collapse) was single-key.
Per-key request distribution is observable at `GET /stats`.

### Smoke probe
```bash
bash scripts/smoke_siliconflow.sh    # 3 calls × ~50 tokens; PASS / FAIL per key
```

### SiliconFlow model catalog (A7-verified working as of 2026-04-26)
- `Qwen/Qwen2.5-7B-Instruct` — smallest stable; smoke target; ~1.5s latency
- Phase D heterogeneous candidates: see SiliconFlow's catalog at
  https://docs.siliconflow.cn/ (login required). Pin model snapshot id +
  date in `genesis_payload.toml [pput_accounting_0]` before any Phase D
  batch (F-2026-04-22-08 drift defense).

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
