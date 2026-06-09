# LIVE-FC1 Phase 7 — REAL heterogeneous swarm multi-LLM run (bounded)

Run id: `livefc1-swarm-20260608T233701Z` (UTC 2026-06-08).
Worktree: `turingosv4-sw` branch `claude/livefc1-swarm`, base `origin/main 340e2bdd`
(Phase 1-6 merged). This run produces the live evidence the LIVE-FC1 gates verify
and PROVES the acceptance off-tape with the Phase 1-6 observe-only mechanisms.

## What ran (REAL, on the canonical ChainTape)

A thin UNPINNED orchestrator (`src/bin/livefc1_swarm_runner.rs`) drove a real swarm
of agents over a 4-task MATH pool through the local `llm_proxy.py`, on ONE shared
canonical ChainTape, with a config matrix of **3 providers × 2 temperatures × 2
prompt variants** (12 real cells). Per cell: a Phase-5 budget check, a Phase-6
brand-GENERIC `ProviderHandleCapsule` anchored on the canonical CAS, a REAL LLM
call, and a `TaskOpen→EscrowLock→WorkTx` spine carrying real token counts. One cell
is a DELIBERATE fault (unroutable model id). FC2 `MapReduceTick` + `TerminalSummary`
emitted at run end.

### LIVE providers (proven at smoke + in-run)
- **DeepSeek** — `deepseek-chat`
- **SiliconFlow** — `Qwen/Qwen2.5-72B-Instruct`
- **DashScope** — `qwen3-8b`

### Real cost (eat-our-own-dogfood, BOUNDED)
- **2,539 total tokens** (1,218 prompt + 1,321 completion), 11 successful requests,
  2 provider errors, 0 retries.
- Blended provider pricing (~$0.1–0.6 / 1M tokens) ⇒ **well under US$0.01 (sub-cent)**.
- Budget ceiling armed at 6,000 micro-units; real spend stayed under it, so
  `budget_check` returned WITHIN on every cell (FC2-HALT arm not triggered THIS run;
  it is exercised by the Phase-5 unit gates and would fire if spend ≥ ceiling).

### Config-matrix × outcome (honest, real numbers)
- `ok` = 5 (DeepSeek 3/3, DashScope 2/4) — answers correct.
- `fault_injected_llm_err` = 1 (deliberate unroutable model → L4.E `LlmError`).
- `parse_fail` = 6 (REAL SiliconFlow Qwen + DashScope *verbose*-prompt cells wrapped
  JSON in prose/markdown and failed strict parse → L4.E `ParseFailed`). This is
  genuine fault-tolerance data, NOT a defect: every failure landed as an L4.E row,
  zero crashes.
- DeepSeek was the most JSON-format-compliant provider; SiliconFlow/DashScope
  verbose cells are the dominant parse-fail source.

## On-tape verification (the crux)

`src/bin/livefc1_swarm_verify.rs` reads ONLY the canonical tape (L4 + L4.E + CAS)
and reconstructs the acceptance via the Phase 1-6 observe-only mechanisms.

- **(A) FC-liveness** (`observe_fc_liveness`):
  - FC1 LIVE: `predicate_gated_advance` (5), `rtool_wtool_bridge` (15),
    `failure_arm/parse_fail` (6), `failure_arm/llm_err` (1).
  - FC2 LIVE: `boot_trust_root_verified`, `map_reduce_tick` (2), `terminal_halt` (1).
  - FC3: disposition `reached_observable_canary` (loop stays open — honest); proposer
    + canary `REACHABLE-not-fired` (this task workload does not drive the governance
    leg), excused, NOT a zombie.
  - **zombie_count = 0** (no claimed-live module without a tape footprint).
- **(B) VPPUT** (`reconstruct_vpput_from_tape`): per-task integer cost (184–248
  tokens) + ticks reconstructed faithfully; **`progress = 0` for every task —
  HONEST**: a math workload drives no Lean oracle, so no `VerificationResult.verified`
  ground-truth witness exists, and the canonical metric is gated 0 without a verified
  golden path. `ranking_by_cost_times_ticks_asc` in `metrics.json` reports which cell
  WOULD have the best VPPUT if a verified golden path existed.
- **(C) heterogeneity** (`distinct_provider_handles_on_tape`): **6 distinct
  brand-free provider handles** on the canonical CAS (3 providers × 2 temperatures;
  the descriptor includes temperature). **Zero brand names on the canonical CAS** —
  the brand→handle mapping lives ONLY in the external `brand_sidecar`.
- **(D) replay** (`replay_roots_match_genesis`): **true** — from-genesis replay via
  the PINNED `verify_chaintape` reconstructs identical state/ledger roots.
- **(E) fault on tape**: L4.E `LlmError` = 1, `ParseFailed` = 6, total L4.E = 7.
  The injected fault is on the tape as an L4.E `LlmError` row (not a crash).

## Files
- `metrics.json` — consolidated: providers, config matrix × outcome, VPPUT ranking,
  fault record, FC-liveness summary, heterogeneity, replay-roots-match, budget,
  real token spend.
- `swarm_run_sidecar.json` — per-cell run sidecar (includes the EXTERNAL
  `brand_sidecar` — the ONLY place brand names appear).
- `verify_metrics.json` — raw on-tape verifier output.
- `runtime_repo/` — canonical L4 (git `refs/chaintape/l4`, 19 entries) + L4.E
  (`rejections.jsonl`, 7 rows) + identities (`agent_pubkeys.json`,
  `pinned_pubkeys.json`, `genesis_report.json`, `initial_q_state.json`).
- `cas/` — CAS index (`.turingos_cas_index.jsonl`); 294 CAS objects in
  `refs/chaintape/cas`.

Inputs: `handover/evidence/_livefc1_swarm_inputs/{math_samples.json,budget_manifest.toml}`.

## Redaction
No API key value and no `sk-` token appears in ANY evidence file or git ref
(verified). No brand name on the canonical CAS (verified). Brands appear only in the
external `brand_sidecar` / `metrics.json` heterogeneity block, per the user directive.

## HONEST A1–A9 acceptance scorecard

| # | Acceptance criterion | Status | Evidence |
|---|---|---|---|
| A1 | Real heterogeneous multi-LLM swarm on the canonical ChainTape | **MET** | 3 live providers, 12 cells, 19 L4 + 7 L4.E on one shared tape |
| A2 | FC1 fired on tape (incl. ≥1 failure arm from the fault) | **MET** | `observe_fc_liveness`: predicate_gated_advance + rtool_wtool_bridge LIVE; llm_err + parse_fail arms LIVE |
| A3 | FC2 boot / tick / terminal on tape | **MET** | FC2 boot=LIVE tick=LIVE terminal=LIVE |
| A4 | FC3 reached-canary-or-honestly-not-fired; no-zombie | **MET (honest, not-fired)** | FC3 disposition `reached_observable_canary`; proposer/canary `REACHABLE-not-fired` excused; **zombie_count=0** |
| A5 | Real integer VPPUT per task + best-VPPUT cell | **MECHANISM-MET; progress honestly 0** | per-task cost/ticks reconstructed; `progress=0` (no Lean oracle on math); best-would-be cell reported |
| A6 | ≥2 distinct provider handles on tape (brand-free heterogeneity) | **MET** | 6 distinct brand-free handles; 0 brand names on canonical CAS |
| A7 | replay_roots_match_genesis = true | **MET** | from-genesis replay via pinned `verify_chaintape` |
| A8 | Injected fault on tape as L4.E (llm_err/parse_fail), not a crash | **MET** | L4.E LlmError=1 (injected) + ParseFailed=6 (real provider format drift) |
| A9 | Bounded real cost under a Phase-5 budget ceiling | **MET** | ceiling armed (6,000 µ); ~2.5k tokens (<US$0.01); budget_check WITHIN every cell |

### Honest caveats
- **A5 progress=0** is the single most important honesty point: VPPUT's numerator is
  ground-truth gated and this is a math benchmark with no Lean oracle, so no task has
  a verified golden path. The metric's cost/ticks denominator is reconstructed
  correctly; the numerator is correctly 0. A formal (Lean) workload would produce
  non-zero VPPUT.
- **A4 FC3 not-fired** is honest: this is a task workload, not a governance run; the
  FC3 proposer/canary/architect leg is reachable but unexercised (excused, not a
  zombie). FC3 remains mechanism-only for this run.
- **FC2-HALT (budget) not triggered** this run because real spend stayed under the
  ceiling. The halt arm is mechanism-only here (exercised by Phase-5 unit gates);
  lowering the ceiling below ~1,000 µ would fire it on-tape.
- Scaling to >50/>100 agents is a pure config change (`max_real_cells` + provider
  list); kept bounded for cost on this first real run.
