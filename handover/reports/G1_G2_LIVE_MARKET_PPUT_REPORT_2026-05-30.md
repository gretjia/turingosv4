# G1/G2 — Live-LLM Constitutional Market: capability benchmark (PPUT) report

> 2026-05-30 · branch `claude/swebench-agi-benchmark` (PR #216) · binary `src/bin/g1_market_live_agent.rs`
> ALL runs are LIVE (DeepSeek `deepseek-chat` via local proxy :8123, NO scripted stubs) and **replay-verified** (`verify_chaintape`).
> Task: ζ-regularization (1+2+3+… = −1/12). Judge: deterministic ζ step-judge (`--strict` = multi-milestone gate).

## 1. What G1 proves (capability, above G0's activation)

The v4 constitutional priced-DAG market, driven by **live** DeepSeek agents, reaches **OMEGA** (a completed ζ proof) with **real price discovery** and **real verifier settlement**, on a replay-verifiable substrate. This is the first capability-level result on v4 — and it **reproduces the v3 ζ Run-6 mechanism on a far more rigorous substrate**.

Each agent, per turn: reads a **shielded** market view (node ids + integer-rational prices + recent steps; no judge internals, no other balances) → calls DeepSeek to propose a proof STEP with a self-reported confidence → the deterministic ζ judge verdicts it → an accepted step becomes a **priced per-task node** whose WorkTx-Long stake is **scaled by confidence** (so prices DISCOVER, vs G0's flat 0.67) + a ChallengeTx-Short prices each node. Parent selection is **price-driven** (`boltzmann_select_parent_v2`). OMEGA settles the market via `emit_system_tx(EventResolve)`.

## 2. PPUT (architect's definition) + scale curve + cost frontier

PPUT = golden-path progress (token count) per unit time; **0 if no completion** (no golden path). All runs below reached OMEGA and are replay-verified.

| run | agents | nodes | distinct prices | golden-path tokens | wall (s) | **PPUT** | cost (USD) |
|---|---|---|---|---|---|---|---|
| N=4 (stop@OMEGA) | 4 | 6 | 5 | 3107 | 25.7 | **121.0** | $0.0009 |
| N=8 (stop@OMEGA) | 8 | 9 | 5 | 3177 | 73.7 | **43.1** † | $0.0022 |
| N=16 (stop@OMEGA) | 16 | 7 | 3 | 2600 | 19.1 | **136.4** | $0.0014 |
| N=30 (stop@OMEGA) | 30 | 6 | 3 | 1819 | 15.8 | **115.1** | $0.0008 |
| **N=8 STRICT verifier** | 8 | 6 | 3 | 2262 | 17.7 | **127.5** | $0.0011 |
| N=8 continue (rich DAG) | 8 | 35 | 6 | 2195 | 282.7 | 10.2 | $0.0073 |

† N=8 is a flakiness outlier: 15 of 24 LLM calls returned malformed JSON that run → slow → low PPUT. Not a scale effect.

**Findings (honest):**
- **PPUT is roughly FLAT (~115–136) across 4→30 agents** for this task. OMEGA is reached fast regardless of agent count, so scale neither helps nor hurts much — **the ζ-style scale benefit (90 agents) only manifests on HARD tasks needing deep collective search; this task (permissive-ish judge + capable model) is too easy to show it.**
- **Cost frontier: every OMEGA run costs < $0.003** (deepseek-chat flash; 4k–10k tokens). The headline is **collective reasoning to OMEGA at ~$0.001**.
- **The STRICT verifier works**: even requiring the accepted chain to pass real derivation milestones (define S(N) → series sum x/(1−x)² → asymptotic/Euler-Maclaurin) before OMEGA, the live market reaches OMEGA (PPUT=127.5). So OMEGA is not merely "claims −1/12" — the proof progresses through stages.
- **`continue-past-omega` (rich DAG)**: 35 nodes, max_branching=21 (price-driven boltzmann convergence), 6 distinct prices, multiple OMEGA completions — a ζ-like emergent tree, but PPUT drops to 10 (off-golden-path exploration is costly).

## 3. vs v3 ζ Run-6 — better and worse

**Better (v4):** rigorous substrate (integer-money CPMM, Ed25519 signed typed-tx, ChainTape L4, **deterministic replay reconstruction** — every run verified; v3 was f64 + self-declared OMEGA + not replayable); **higher proof quality** (coherent derivation S(N)→series→real-part→−1/12, vs v3's repeated sentence); far faster/cheaper (16 s / $0.001 vs 50 min / 90 agents).

**Worse / not yet:** scale (35 nodes vs ζ 648; 30 agents vs 90); **shallow price game** (model overconfidence → stakes cluster; ~half the challenges hit `monetary_invariant` at scale → those nodes default to price 1.0; no ζ-style 52-bet contested whales / net-Bear emergence); **shallow golden path** (3 steps — the judge is permissive enough that OMEGA is easy); LLM JSON malformation at scale (15/24 one run).

## 4. Industry positioning (LOW confidence)

This is a MATH-proof market at ~$0.001/OMEGA, PPUT~120 — **NOT directly comparable** to OpenHands/Amazon Q SWE-bench *coding* numbers (different task, different model, a permissive deterministic judge, math not code). It establishes a NEW frontier point — **extremely cheap collective reasoning** — but a real leaderboard coordinate requires a **hard task + a strong external verifier** (next).

## 5. Recommendation (the honest next step)

To turn this from a working capability *demo* into a defensible capability *benchmark*:
1. **Strong external verifier**: ζ deterministic judge → **Lean kernel** (math) or **real Docker SwebenchTestJudge** (coding, needs `swebench` venv; Docker available). Then OMEGA is rigorous, PPUT measures real capability, and the **scale benefit becomes visible** (hard tasks reward many agents).
2. **Fix challenge `monetary_invariant` at scale** + induce calibrated confidence / explicit bet actions → deeper price game (toward ζ's contestation).
3. **Then** scale to 30–90 agents on the hard-verifier task → a real PPUT scale curve + a defensible industry coordinate.
4. **PPUT denominator**: confirmed as golden-path tokens / wall-time (this report). For cross-scale cost comparison, the cost frontier (USD/OMEGA) is reported alongside.

## 6. Evidence (replay-verified, in /tmp; not committed per no-sidecar)
`g1run2` (4×4, PPUT 1710 under old formula / 121 corrected), `g2_n{4,8}`, `g2b_n{16,30}`, `g2_strict`, `g1run3` (rich DAG). Each: `verify_chaintape` → ledger/system/agent signatures verified, state + economic_state reconstructed, `replay_failure=null`.
