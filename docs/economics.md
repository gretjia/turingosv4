# TuringOS v4 Economic Engine — Reward Settlement Protocol (RSP)

**Status**: rewritten 2026-04-29 to align with constitutional Laws + RSP-0/RSP-1 ground rules. Supersedes the prior "APMM Mint-and-Swap router / per-node 1000 YES + 1000 NO injection" framing, which conflicted with the unique-mint axiom.

**Authority**: external audit 2026-04-29 (CF-2) + user `gretjia` chat authorization. Cross-references: `constitution.md` Art-Laws.1+2; `handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md` § P3 + § RSP-N.

---

## 1. Constitutional Laws

### Law 1: Information is Free

Agents may read authorized views, search Mathlib / public task signals / their own balance, inspect public market state, and **think locally** without debiting core Coin.

Specifically, all of the following operations carry zero core-Coin cost:

- `rtool` read of agent's own materialized view;
- `search` over CAS / public task index / public predicate registry;
- private draft, private chain-of-thought, sandbox simulation;
- inspection of `economic_state_t.balances_t` for one's own agent_id.

Physical compute may be quota-limited (token budget, wall-clock cap, scheduler priority), but reading or thinking **cannot mutate** `economic_state_t`.

### Law 2: Only Investment Costs Money

Any operation that changes — or attempts to change — global state or economic state MUST lock stake or escrow first. Specifically:

| Transaction | Required pre-lock |
|---|---|
| `task_open_tx` | `escrow_lock_tx` over the bounty pool |
| `work_tx` | event-bound YES stake of the proposing agent |
| `verify_tx` | verifier bond |
| `challenge_tx` | event-bound NO stake of the challenger |
| `settlement_tx` | `escrow_sufficient` + `payout_sum_valid` + `monetary_invariant` |

A transaction missing its required pre-lock is rejected by `dispatch_transition` and recorded in the L4.E rejection-evidence ledger (NOT in the accepted transition ledger).

### CTF Conservation

For a specific event E:

```
1 locked Coin = 1 YES_E + 1 NO_E
```

YES_E and NO_E are **event-bound claims** over a single locked Coin. They are not new money; the locking step does not mint, and the resolution step does not destroy — it merely redirects the locked Coin between balances per the predicate verdict + slash policy.

### Unique Mint

`on_init` is the **only** legal base-Coin injection. Post-init mint is invalid and must fail-closed at predicate / dispatch / settlement layers.

Any liquidity, bounty, reward, or settlement payout MUST come from one of:

- an explicit `balances_t` entry of the funding agent (debit);
- an explicit `escrows_t` entry locked by a prior `escrow_lock_tx`;
- an explicit `stakes_t` entry locked by a prior `yes_stake_tx` / `no_stake_tx` / verifier bond;
- a `slash_pool` entry produced by a prior `slash_tx` that already passed the challenge resolution.

Payouts from un-attributable "ghost liquidity" are forbidden.

---

## 2. RSP State Machine

The lifecycle of a bounty is:

```
OPEN
 → SUBMITTED
 → VERIFIED
 → PROVISIONAL_ACCEPTED
 → CHALLENGE_WINDOW
 → FINALIZED
 → PAID
```

Failure path:

```
SUBMITTED → REJECTED → STAKE_SLASHED
```

Challenge-success path:

```
PROVISIONAL_ACCEPTED → CHALLENGED → REVERTED / COMPENSATED → SLASHED → CHALLENGER_REWARDED
```

State transitions are dispatched by `dispatch_transition` (§ src/state/sequencer.rs). Each transition consumes a `TypedTx` (Work / Verify / Challenge / FinalizeReward / Reuse / TaskExpire / TerminalSummary), validates the relevant pre-lock + monetary invariant, and either emits an accepted transition (advancing `state_root` + `ledger_root`) or appends to L4.E rejection-evidence (no `state_root` advance).

---

## 3. Reward Formula

A finalized payout to participant `i` is:

```
reward_i = Finalize(
    Escrow(task)
    × Accept(tx_i)
    × Attribution(tx_i, ContributionDAG)
    × Survival(challenge_window)
    × Utility(post_acceptance_metrics)
    × Constitution(Q_t)
)
```

Each multiplicand has a strict zero condition:

- `Escrow(task) = 0` if the task never got a valid `escrow_lock_tx`;
- `Accept(tx_i) = 0` if Lean / predicate bundle did not return PASS;
- `Attribution(tx_i, ContributionDAG) = 0` if the agent has no edge in the DAG (self-claim is ignored);
- `Survival(challenge_window) = 0` until the window closes with no upheld challenge;
- `Utility(...) = 0` until post-acceptance utility predicates are evaluated;
- `Constitution(Q_t) = 0` if any constitutional invariant is violated by the payout.

Hence: `reward_i = 0` whenever any leg is zero. There is no "good faith" override.

---

## 4. RSP-N Micro-versions (P3 internal sequencing)

P3 RSP Economy Core ships in 8 micro-versions. Each must independently green its Exit list before the next begins. Cross-reference: `handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md` § 6.

### RSP-0 — monetary invariant

- Files: `src/economy/monetary_invariant.rs`, `src/economy/ctf.rs`.
- Predicates: `read_is_free`, `no_post_init_mint`, `coin_conservation_valid`, `yes_no_split_valid`.
- Exit: `total_coin(EconomicState)` invariant after `on_init`; `rtool` + `think` deduct zero core Coin; any `mint_tx` after `on_init` returns `MonetaryError::PostInitMint`.

### RSP-1 — task escrow + WorkTx YES stake

- Files: `src/economy/escrow_vault.rs`, `src/economy/stake_manager.rs`, `src/economy/task_market.rs`.
- New tx: `task_open_tx`, `escrow_lock_tx`, `yes_stake_tx`.
- Exit: a task without `escrow_lock_tx` is not admitted to the official market; `work_tx` without YES stake is rejected by `dispatch_transition`.

### RSP-2 — verifier bond + challenge_tx NO stake

- Files: `src/economy/challenge_court.rs` (new), updates to `stake_manager.rs`.
- New tx: `verify_tx`, `challenge_tx`, `no_stake_tx`.
- Exit: independent verification required; challenger NO stake required; failed challenge slashes challenger; successful challenge slashes solver and any verifier that passed the bad work.

### RSP-3 — challenge window + provisional reward

- Files: `src/economy/settlement_engine.rs` (new).
- New tx: `provisional_accept_tx`, `challenge_resolve_tx`.
- Exit: predicate-PASS produces only `provisional_accept`, never full payout; full settlement requires the challenge window to close with no upheld challenge.

### RSP-4 — Contribution DAG + settlement_tx

- Files: `src/economy/contribution_dag.rs` (new), `src/economy/settlement_engine.rs`.
- New tx: `settlement_tx`, `slash_tx`, `reputation_update_tx`.
- Exit: rewards derived from the Contribution DAG (not from agent self-claim); `payout_sum ≤ escrow_pool`; agent self-claim "我贡献 90%" is ignored.

### RSP-5 — deferred impact bonus + reuse royalty

- Files: `src/economy/royalty_index.rs` (new).
- Exit: tools or predicates that get reused trigger a royalty edge; royalty has cap, decay, and bug-clawback.

### RSP-6 — price index + risk market

- Files: `src/economy/price_index.rs` (new).
- Exit: price signals affect task priority but cannot override predicate or constitution failure; YES/NO risk price is informational, not authoritative.

### RSP-7 — public settlement adapter

- Bridge to P7 Public Settlement Network.
- Out of scope until P5 MetaTape v1 + P6 multi-org are at least partial green.

---

## 5. Forbidden Legacy Semantics

The following constructs from prior versions of this document and from prior code paths are **forbidden** under RSP. Any code path that resembles them must either be removed or be reframed with explicit treasury / escrow / stake / slash-pool funding sources:

- **Per-node automatic YES/NO injection** at task creation. (Was: "每个新节点系统自动注入 1000 YES + 1000 NO 做市".) This is post-init mint by another name.
- **APMM Mint-and-Swap as a price-oracle mint mechanism**. (Was: "APMM Mint-and-Swap router".) AMM-style market making is permitted in future product lines ONLY if liquidity is funded from an explicit treasury debit, sponsor escrow, or LP stake — never as silent mint.
- **Ghost liquidity** — any liquidity in the task market or risk market without a tracked source in `economic_state_t`.
- **Reward by token count** — paying agents per emitted token.
- **Reward by runtime** — paying agents per wall-clock second.
- **Reward by self-claimed contribution** — reward formula must derive `Attribution(...)` from the Contribution DAG, not from agent self-report.
- **Full payout before challenge window closes** — only `provisional_accept` is permitted on predicate-PASS.
- **Price signal overriding predicate / constitution failure** — price information is informational; it cannot finalize a transition the predicate gate rejected.

A future product line that wants prediction-market liquidity (CPMM / LMSR / etc.) MUST present its funding as an explicit transfer between `balances_t`, `escrows_t`, and `stakes_t`, AND pass `monetary_invariant`.

---

## 6. Note on "Digital Property Rights"

Earlier drafts framed "Digital Property Rights" (independent agent skill paths, species-evolution metaphor) as a parallel economic Law alongside "Information is Free" and "Only Investment Costs Money". That framing is **not constitutional**: the only economic Laws are Law 1 + Law 2 + CTF + Unique-Mint above.

Digital-property concepts (per-agent reputation, reuse royalty graph, agent-specialization indices) are **product-line concerns** that live in P3 RSP-5 (`royalty_index.rs`), P4 Information Loom (`reputation_index.rs`), and P8 Autonomous Agent Economy (`agent_specialization.py`). They are downstream of the four Laws, not parallel to them.

---

## 7. Cross-references

- Constitution: `constitution.md` Art-Laws.1+2 + Art. II.2 (broadcast price signal).
- Roadmap: `handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md` § P3 + § 6 (RSP-N).
- Audit driving this rewrite: `handover/audits/2026-04-29_external_audit.md` § CF-2.
- TB-1 charter: `handover/tracer_bullets/TB-1_recharter_2026-04-29.md` Day-2 (RSP-0 monetary invariant).
- Code home for RSP modules: `src/economy/` (currently scaffold-only at the time of this rewrite; population begins at TB-1 Day-2).
