> ⚠️ **CORRECTION 2026-06-01** — the reputation port shows price routes to specialists + defunds Sybils, but constitutional DEFUNDING is not actually implemented (the sequencer locks stake on admit and does NOT slash on reject); the headline carries no fair baseline. Honest status: Verdict-B governance demo, not a price-causal result.
>
> Full evidence + the systematic fix: `handover/reports/SESSION_FORENSIC_RETROSPECTIVE_2026-06-01.md`.
> External claims are held to **Verdict B only** until the real-value experiment (lean_market_agent, non-local price-routed tree search) passes with fair baselines + tape-recompute.

---

# P3 — constitutional port: the reputation economy runs verify_chaintape-green

> 2026-05-31. The diagnostic agent-economy (proven 10/10 on real Lean) ported to the REAL constitutional
> substrate (ChainTape L4 + CAS + sequencer + EconomicState). The upgrade that matters: the economy run
> is now **verify_chaintape-GREEN** — reconstructable from L4 + CAS alone, not an inline-JSONL diagnostic.
> Class 2, FC1/2/3 untouched, no §6 surface. Bin: `src/bin/reputation_constitutional.rs`.

## What was achieved
`reputation_constitutional` reuses g1_market_live_agent's exact real-tx adapter pattern (genesis_with_balances,
make_real_task_open / escrow_lock / worktx_signed_by, submit_await, compute_price_index over the real
EconomicState, emit_system_tx EventResolve). A stream of proof tasks is routed to agents by on-chain wealth;
each task becomes a real on-chain node (TaskOpen + EscrowLock + WorkTx with capital staked); honest
specialists close their family (predicate_passes=true → WorkTx admitted), Sybils/wrong agents fail
(predicate_passes=false → WorkTx rejected). Settlement via EventResolve.

**verify_chaintape result (multiple runs, exit 0):**
```
ledger_root_verified: true     state_reconstructed: true
system_signatures_verified: true   economic_state_reconstructed: true
agent_signatures_verified: true    cas_payloads_retrievable: true
proposal_telemetry_cas_retrievable: true     replay_failure: null
```
All seven constitutional indicators green. The economy's wealth, prices, stakes, nodes, and settlement are
deterministically reconstructable from the frozen ChainTape — the Art. 0.2 requirement. This is the
"diagnostic-real → constitutional-real" upgrade flagged as the open P3 step.

## What the real substrate TAUGHT us (honest finding — a semantic the diagnostic abstracted)
The diagnostic's economics were: a winning bet COMPOUNDS wealth, a losing/Sybil bet DRAINS wealth →
Sybil defunding. The real sequencer's WorkTx economics are different and more conservative:
- a SUBMITTED WorkTx LOCKS its stake (escrow) pending settlement;
- a REJECTED WorkTx (failed predicate — what a Sybil produces) locks NOTHING — capital is untouched.

So on a raw admit/reject pass, the honest specialist that submits a real WorkTx temporarily shows LOWER
free balance (stake locked: 980k of 1M) while a Sybil whose WorkTx is rejected keeps its full 1M. The
"Sybil-defunding" the diagnostic showed requires the SETTLEMENT phase to slash the failed party's locked
stake — which in the per-task-node model means resolving each node's market (EventResolve per node), and
the slashing economics live in the sequencer's settlement path. That is Class 3/4 (sequencer settlement
semantics / §6) and is the correct next atom, NOT a Class-2 wire-up.

**This is the value of porting to the full system:** the diagnostic proved the economic THESIS (capital-at-
risk price routing beats baselines, 10/10); the constitutional port proves the substrate REPLAYS that
economy verifiably AND surfaces the exact remaining gap — per-node settlement slashing is where the
diagnostic's "defunding" becomes constitutional. The thesis is unchanged; its constitutional realization
needs one more (Class 3/4, §8-gated) settlement atom.

## Status of the integration
- ✅ DONE (Class 2): the reputation economy runs through the real sequencer; WorkTx/Escrow/TaskOpen on
  real ChainTape; prices from real EconomicState; **verify_chaintape-green** (7/7 indicators, replay_failure
  null). The result is now constitutionally reconstructable.
- ⏭ NEXT (Class 3/4, §8-gated — flagged, not done): per-node EventResolve settlement that slashes a
  rejected/failed bettor's locked stake, so the diagnostic's wealth-drain Sybil-defunding becomes a
  constitutional economic outcome (not just an admission-rejection). This touches sequencer settlement and
  is correctly deferred to an architect-ratified Class-4 atom.

## Discipline
FC1/FC2/FC3 hashes unchanged (matrix_drift 3/3). No §6 surface edited (the port only CALLS existing
adapters). Integer money. Liveness 12/12 (bin registered). Real DeepSeek-free deterministic competence
(the diagnostic already established competence via real Lean; this bin isolates the constitutional
economics). verify_chaintape exit 0 on every run. PR-only.
