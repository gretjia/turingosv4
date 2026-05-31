# Multi-model capability ladder — infrastructure ready for the emergence scaling study

> 2026-05-31. The architect's new main thrust: quantify multi-agent intelligence EMERGENCE (many small
> models ≥ few large models) and build a MATHEMATICAL PREDICTION MODEL (task complexity → required N_agents
> × parameter scale). This needs a real parameter axis. SiliconFlow keys retrieved from omega-vm-hk (authed)
> → proxy reloaded → a real 7B→671B ladder is live. NOT省-token; the thrust is capability-emergence.

## Confirmed live capability ladder (via proxy :8123, http=200 choices=1, 2026-05-31)
| API model string | params | role in the study |
|---|---|---|
| `Qwen/Qwen2.5-7B-Instruct` | 7B | small rung (legacy dense) |
| `Qwen/Qwen3-8B` | 8.2B | small rung (Qwen3 dense, thinking) |
| `Qwen/Qwen3-14B` | 14.8B | mid rung |
| `Qwen/Qwen3-32B` | 32.8B | upper-mid rung (strong math: ~72% AIME) |
| `Qwen/Qwen2.5-72B-Instruct` | 72B | large dense rung |
| `deepseek-ai/DeepSeek-V3.2` | 671B/37B MoE | large frontier rung |
| `deepseek-chat` (local) | — | cheap workhorse |
| `deepseek-reasoner` (local) | — | strong reasoner |

NOT enabled on this key (403 Model disabled, don't use): `deepseek-ai/DeepSeek-R1-Distill-Qwen-7B`,
`Qwen/Qwen3-30B-A3B`. The Qwen3 dense ladder 8B→14B→32B is the cleanest same-architecture axis; Qwen2.5
7B/72B + DeepSeek-V3.2 extend the dynamic range to ~100x params.

## Two clean axes available
- **Same-family dense (isolates parameter count, no arch confound):** Qwen3-8B → Qwen3-14B → Qwen3-32B (~4x).
- **Full dynamic range (for the frontier comparison):** 7B → 72B → 671B (~100x), accepting an arch shift.

## Proxy state
Restarted from /Users/zephryj/work/turingosv4-probe-gpqa/.env (now the full omega-vm-hk .env: 3 SiliconFlow
keys round-robin + DeepSeek + Volcengine + NVIDIA + Gemini + Dashscope). Routing: `Qwen/...` / org-prefixed
→ siliconflow; `deepseek-*` → deepseek. The bin already speaks OpenAI-compatible to :8123, so an agent can
be assigned ANY of these model strings — heterogeneous-model agent pools are now one CLI flag away.

## The study this unblocks (architect's new thrust)
1. **Capability emergence:** does a POOL of N small models (Qwen3-8B × N, coordinated by the TuringOS market)
   match or exceed a single large model (Qwen3-32B / Qwen2.5-72B / DeepSeek-V3.2) on hard Lean theorems —
   compared by SOLVE COUNT/DIFFICULTY (capability ceiling), NOT cost.
2. **Scaling curve + prediction model:** sweep N_agents = 1,2,4,8,16,32 at each model size; fit
   completion-rate vs (N, params) → a mathematical budget function: task complexity → (N_agents, param scale)
   needed to solve it. The neural-scaling-laws analog for multi-agent emergence.
3. Real data, quantified, replayable; every solve Lean + #print-axioms verified.

## Security
.env retrieved via authorized SCP from omega-vm-hk:~/projects/turingosv4/.env to the proxy dir (OUTSIDE the
turingosv4 git repo → never committed). Prior deepseek-only .env backed up as .env.bak-deepseek-only.
