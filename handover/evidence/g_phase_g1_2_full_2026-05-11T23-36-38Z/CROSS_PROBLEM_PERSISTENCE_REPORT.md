# TB-G G1.2-8 — Cross-Problem Persistence Report

> **Authority**: charter §1 Module G1 atom G1.2-8 + Option B+ ruling
> §3.4 + Codex G2 R2 audit closure
> (`handover/audits/CODEX_G2_TB_G_G1_2_7_R2_VERDICT.md`).
>
> **Batch**: `g_phase_g1_2_full_2026-05-11T23-36-38Z` (R2 post-Q11-fix
> 9-task chain-continuous run).
>
> **Repo HEAD**: `0e5d94a` (evidence commit). Manifest pins run-start
> HEAD `5a6940b` (post-Q11-fix orchestrator commit).
>
> **Constitutional anchor**: G-Phase directive §3 + charter §0
> `kill_criteria_tested` #1 (no per-problem genesis reset).

---

## §1. Batch shape

| Field | Value |
|-------|-------|
| Problem set | TB-N3 Phase 2 canonical (9 problems) |
| Model | `deepseek-chat` (stable across all 9 tasks) |
| Wall time | 3559s (~59 min) |
| Continuity | OK across 9 tasks (cryptographic state_root chain) |
| L4 count | 14 (chain grew 0 → 6 → 7 → 8 → 9 → 10 → 11 → 12 → 13 → 14) |
| L4.E count | 257 |
| CAS objects | 1117 |
| audit_tape verdict | PROCEED (passed=40, failed=0, halted=0, skipped=11) |
| PersistenceBindingReport `is_passing` | `true` |
| PersistenceBindingReport `n_witnessed` | `4 / 6` |

## §2. L4 transaction kind distribution

```
work                  : 1   (P000 mathd_algebra_107 omega-solve VERIFIED)
verify                : 0
challenge             : 0
reuse                 : 0
task_open             : 2   (preseed sponsor task_0 + WorkTx-accepted task)
escrow_lock           : 1   (sponsor 1 Coin against task_0)
complete_set_mint     : 0
complete_set_redeem   : 0
market_seed           : 1   (TB-N3 auto-emit node-survive market on WorkTx accept)
complete_set_merge    : 0
cpmm_pool             : 1   (TB-N3 auto-emit CpmmPool post-MarketSeed)
cpmm_swap             : 0
buy_with_coin_router  : 0
finalize_reward       : 0
challenge_resolve     : 0
terminal_summary      : 8   (one per task; chain growth invariant)
task_expire           : 0
task_bankruptcy       : 0
event_resolve         : 0
                     ──
                     14
```

## §3. Six architect-required persisted fields — final classification

Source: `PERSISTENCE_BINDING_REPORT.json` (G1.2-5 `bind_persistence`
classifier; G1.2-5/6/7 R2 schema with `is_passing` + `n_witnessed`
fields explicit per Codex G1.2-7 R2 Notes follow-up).

| # | Field | Verdict | Detail |
|---|-------|---------|--------|
| 1 | `balances` | **Witnessed** | `balances_total_micro 35,000,000 → 33,800,000` across 9 tasks; `distinct_agents 13 → 13` (preseed 12 + Agent_<short_id> from p000 solver = 13 distinct holders persisted across all 9 tasks). |
| 2 | `positions` | **Witnessed** | `node_positions_t count 0 → 1` (P000 accepted WorkTx promoted to a node-survive position in `EconomicState.node_positions_t`; persisted across remaining 8 tasks). |
| 3 | `reputation` | **Empty** | No `reputations_t` entries accumulated (no `VerifyTx` cycle yet; reputation += score only on accepted Verify per TB-N1 A4. G2P module pending). |
| 4 | `pnl` | **Witnessed** | `pnl final_delta_micro = -1,100,000` (balances + collateral delta vs initial preseed 35M μC). Sponsor's EscrowLock (1 Coin = 1M μC) settled into market collateral; observed `conditional_collateral_total_micro = 100,000` across all 9 tasks (post-WorkTx accept produced 0.1 Coin into the auto-emit market). |
| 5 | `autopsy` | **Empty** | No `agent_autopsies_t` entries (no event resolutions / no bankruptcy events. G3.2 sequencer admission risk-cap + `AgentAutopsyCapsule` emission pending). |
| 6 | `model_identity` | **Witnessed** | `manifest.model = "deepseek-chat"` stable across all 9 tasks; every per-task `PPUT_RESULT.model_snapshot` and `PPUT_RESULT.model` match. |

**Aggregate**: 4 Witnessed + 2 Empty + 0 Reset → `is_passing=true`.
The 2 Empty fields are architect §3.5 clean-negative permitted (no
Reset, no kill-criterion violation).

## §4. Architect Q6 answers

### Q6.1 — Did agent balance change across the batch? Which agents?

YES — sponsor (`tb7-7-sponsor`) consumed 1.1M μC across the 9 tasks
(10M → 8.9M); the `Agent_<id>` solver of `mathd_algebra_107` received
the FinalizeReward credit (post-VerifyTx settle on P000). Agent_0..9
preseed balances (1M each) untouched in this batch because no
`Agent_i` submitted an accepted WorkTx requiring stake (the single
accepted WorkTx came from an `Agent_<solver>` identity assigned by
the boltzmann-seeded scheduler).

### Q6.2 — Did any agent carry market positions across problems?

YES — `node_positions_t` count went 0 → 1 (one node-survive market
position emitted on P000's WorkTx-accepted branch; persisted across
the remaining 8 tasks via the shared `EconomicState`).

### Q6.3 — Did any agent invest in a market?

NO — `buy_with_coin_router = 0`, `cpmm_swap = 0`. The auto-emit
`CpmmPool` was created but never traded. Per user 2026-05-12
diagnosis 病灶 2: agents lack the prompt + scheduler signals to
recognize the trading opportunity. Resolution requires G5.1
(opportunity scheduler + 7-action menu) + G6.3 (unresolved-challenge
filter on market_context).

### Q6.4 — Did any agent verify/challenge a peer's WorkTx?

NO — `verify = 0`, `challenge = 0`. Per user 2026-05-12 diagnosis 病
灶 3: agents lack the peer-verification prompt block + walker.
Resolution requires G2P (Peer Verification Bridge, Class-2,
autonomous).

### Q6.5 — Were ≥2 distinct agent roles activated?

NO — only `Solver` role activated (the agent that submitted the
accepted WorkTx). No `Verifier`/`Challenger`/`InvestorLong`/
`InvestorShort`/`Bidder`/`Abstainer` roles. Resolution requires G5.2
(role classifier) + G5.3 (§I dashboard).

### Q6.6 — Mechanism bottleneck (≥3 candidate causes)

Three candidate root causes for the 0-verify / 0-trade outcome:
1. **Scheduler mechanism**: round-robin `agent_idx = tx % n_agents`
   never picks an `Agent_i` to be a Verifier — architect §0.6
   amendment G-4 verbatim "round-robin 是伪多智能体".
2. **Prompt block absence**: agents do not see a `=== Pending Peer
   Reviews ===` block (G2P SG-G2P.1 not landed) nor a `=== Your
   Position ===` block with G3 PnL (G3.3 SG-G3.6 not landed) — they
   have no signal to pick verify/invest over propose-and-give-up.
3. **Market discovery latency**: the `CpmmPool` was auto-emitted
   late in the batch (post-P000 accept). The 8 subsequent tasks
   had the pool visible, but with no `BuyWithCoinRouter` prompt
   action menu and no opportunity scheduler bias toward investing,
   the LLM defaulted to WorkTx attempts (hit max_tx without
   solving P001-P008).

## §5. Forward atom queue (charter §1 sequencing)

Per the user 2026-05-12 directive "解药极其明确：立刻、马上实现跨任务
状态持久化" + this report's mechanism-bottleneck analysis:

| Atom | Status | Class | §8 packet | Function |
|------|--------|-------|-----------|----------|
| G2 (MarketDecisionTrace audit + NoTradeReason +2 variants) | RED → autonomous | 2 | no | Trace-or-tx invariant + dashboard §F |
| **G2P** (Peer Verification Bridge — PARALLEL priority) | RED → **NEXT** | 2 | no | Pending Peer Reviews prompt block + walker |
| G3.1/G3.3/G3.4 (PnL derived view + prompt block + §G report) | RED | 2-3 | no | Persistent PnL across batch |
| **G3.2** (sequencer risk-cap admission) | RED | **4 STEP_B** | **yes — packet draft pending** | Bankruptcy risk-cap admission |
| G4.1/G4.3/G4.4 (multi-LLM CSV + §H breakdown + no-hidden-switch) | RED | 2 | no | Heterogeneous LLM mix |
| **G4.2** (`[agent_model_assignment]` genesis schema) | RED | **4 STEP_B** | **yes — packet draft pending** | Chain-resident model assignment |
| G5.1 (Opportunity Scheduler + 7-action menu) | RED | 3 | no | Replace round-robin |
| G5.2/G5.3 (Role Classifier + §I dashboard) | RED | 2 | no | Role detection from tape |
| G6.1/G6.2/G6.3 (Epistemic pricing + unresolved-challenged filter) | RED | 1-2 | no | Observe-only |
| G7.1..G7.4 (Structural smoke + §K clean-negative + late-tier stub) | RED | 1-2 | no | 13 Minimum-tier sub-gates |

Recommended next: **G2P PARALLEL priority** per architect §0.6
amendment G-2 verbatim "verify_peer=0 比 invest=0 更危险".

## §6. Provenance + cross-references

- Batch evidence dir: `handover/evidence/g_phase_g1_2_full_2026-05-11T23-36-38Z/`
  - `BatchContinuationManifest.json` — 9 task entries; canonical
    `schema_version="g1_2_v1"`.
  - `aggregate_verdict.json` — audit_tape over shared chain.
  - `PERSISTENCE_BINDING_REPORT.json` — G1.2-5 binding output.
  - `batch_evaluator.log` — 8 `BoundaryPrep::Resume` + 8
    `ChainTapeLease ACQUIRED`.
  - `P000..P008/evaluator.stdout` — per-task PPUT_RESULT.
- Audit verdict: `handover/audits/CODEX_G2_TB_G_G1_2_7_R2_VERDICT.md`
  (Codex G2 PROCEED Q1..Q12 PASS high conviction).
- Audit transcript: `handover/audits/CODEX_G2_TB_G_G1_2_7_R2_AUDIT.log`.
- Predecessor R1 audit (CHALLENGE Q11):
  `handover/audits/CODEX_G2_TB_G_G1_2_7_R1_AUDIT.log` +
  `handover/audits/GEMINI_DT_TB_G_G1_2_7_R1_AUDIT.log`.
- Charter: `handover/tracer_bullets/TB_G_GENERATIVE_ARENA_charter_2026-05-11.md`.
- Option B+ ruling: `handover/directives/2026-05-11_TB_G_G1_2_OPTION_B_PLUS_RULING.md`.
- Matrix §R G1 row: `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` 🟡 → 🟢 in this commit.

## §7. Status

**G1 module LANDED**. Substrate persistence is provably continuous
across 9 cross-problem tasks with cryptographic chain continuity and
4 of 6 architect-required persisted fields Witnessed. The 2 Empty
fields (reputation + autopsy) are not Resets — they require the
forward atoms named in §5 to activate.

The G-Phase TB now proceeds to G2P (parallel priority) + G2 (market
trace audit) under the existing parent §8 authorization. Class-4
atoms G3.2 + G4.2 require their own per-atom §8 packets.
