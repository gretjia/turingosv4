# Polymarket Absorption — REJECT/ABSORB classification (architect insight 2026-05-02)

**Source**: Updated Final Ruling (Part C) of `handover/directives/2026-05-02_lossless_constitution_polymarket_directive.md`.
**Status**: INSIGHT — pending authorization to create the four mandated decision records.
**Sequencing**: NodeMarket / Polymarket track starts after TB-10 (Lean Proof Task Market MVP). NOT a v1.0 blocker.

---

## §1 Why a separate Polymarket absorption layer

The Polymarket / Conditional Token Framework / CPMM math is mathematically clean and gives us:

```text
- price as broadcast statistical signal (Art. II.2)
- first-long / short exposure mapping (WorkTx / ChallengeTx)
- coin conservation through CompleteSet algebra
- adversarial liquidity provider (LP) market structure
```

But the work-notes that triggered this absorption contained one fatal misframing: **"each new node automatically gets 100 YES + 100 NO"**. Implemented literally, this resurrects ghost liquidity and violates two Laws (基本法):

```text
Law: Information is Free / Only Investment Costs Money
Law: 1 Coin = 1 YES + 1 NO  (CTF conservation)
Law: on_init is the sole legal mint point
```

This insight file freezes the REJECT/ABSORB classification so future TBs cannot accidentally re-import the unguarded form.

---

## §2 ABSORB list

| Item | Where it lands | Constraint |
|---|---|---|
| `1 locked Coin = 1 YES_E + 1 NO_E` | TB-12 CompleteSet | Locked Coin is the holding; YES/NO shares are claims, not Coin supply |
| `WorkTx.stake = FirstLong exposure` | already in TB-3/TB-4 inline (no new variant) | Per `feedback_wp_vs_roadmap_reconciliation` — WP shape wins |
| `ChallengeTx.stake = Short exposure` | TB-12 wiring | Same precedent |
| `VerifyTx.bond = responsibility bond` | NOT a market position | Bond ≠ exposure — verifier carries duty-of-care, not directional bet |
| CPMM `poolY * poolN = k` math | TB-13 Router | Math correct; LP must come from `MarketSeedTx` debit |
| `buy_yes(payC) → outY = payC * poolY / (payC + poolN)` | TB-13 router formula | Constant-product invariant test required |
| `buy_no` symmetric | TB-13 router formula | Same |
| Lamarckian Autopsy | TB-15 (modified) | Private agent-scoped read view; evidence-derived, not LLM self-report |
| Kelly Criterion fractional cap | TB-15 risk policy suggestion | NOT protocol enforcement — protocol enforces only `max_position_size` / `max_drawdown` / `max_leverage = 1` |
| Boltzmann masking with child-price > parent-price | TB-14 (with predicate guard) | Read-view / scheduler only; ChainTape never deletes parent |

---

## §3 REJECT list (literal form)

| Item | Why rejected | Salvage path |
|---|---|---|
| "Each new node automatically gets 100 YES + 100 NO" | **Ghost liquidity** — no source-of-funds; violates `on_init`-唯一铸币点 + `1 Coin = 1 YES + 1 NO` | Rewrite as `MarketSeedTx` debiting `MarketMakerBudget` allocated by `on_init` |
| "System market-maker can be 0-loss → no bounty needed" | Adverse selection vs informed Agent → CPMM LP loses in expectation; protocol cannot guarantee 0-loss | Bounty + market are **complementary**, not substitutes. Bounty drives baseline labor; market drives price discovery |
| Dynamic Pari-Mutuel (DPMM) with pro-rata payout (maker-protection at expense of redemption) | Different market class (not CTF); changes `1 winning YES → 1 Coin` redemption to `pro-rata of pool`; breaks Agent risk calculus | Reserve as RSP-M7 experimental, NOT v1 core |
| Single price as oracle of two truths (符合规范 + 离目标更近) | Goodhart conflation — masking signal mixes acceptance probability with progress probability | Split into `P_accept(node)` and `P_progress(node)`; scheduler combines with explicit weights |

---

## §4 Four mandated decision records

Per directive §11. Each will live under `handover/alignment/`:

```text
1. DECISION_POLYMARKET_CORE_2026-05-02.md
     Body:
       1 Coin locked = 1 YES_E + 1 NO_E
       YES/NO shares are claims, not Coin supply
       Price is statistical signal (Art. II.2), not truth
       Truth = predicates + ChallengeResolveTx, NOT price

2. DECISION_CPMM_MINT_AND_SWAP_2026-05-02.md
     Body:
       poolY * poolN = k
       buy_yes(payC):
         outY = payC * poolY / (payC + poolN)
         getY = payC + outY
         priceY = payC / getY
       buy_no symmetric
       Router flow: split → keep target → swap reverse → receive extra target
       Constant product invariant required test
       LP source must come from MarketSeedTx (next record)

3. DECISION_MARKET_SEED_NO_GHOST_LIQUIDITY_2026-05-02.md
     Body:
       No automatic YES/NO injection on node creation.
       MarketSeedTx must debit explicit budget (treasury / LP / sponsor).
       on_init may allocate MarketMakerBudget; node seeding consumes it.
       If budget exhausted, node ships without seeded liquidity (graceful degrade).
       LiquiditySeedTx structure (canonical):
         { tx_id, parent_state_root, event_id, provider, collateral_amount,
           pool_y_amount, pool_n_amount, lp_shares_out, signature }
       Invariant: TotalCoin unchanged; ConditionalCollateral increases by collateral_amount.

4. DECISION_LAMARCKIAN_AUTOPSY_BOLTZMANN_2026-05-02.md
     Body:
       Autopsy:
         - Triggered on bankruptcy / liquidation
         - Derived from ChainTape evidence (positions, trades, prices, slippage,
           resolution, L4/L4.E entries) — NOT LLM self-report
         - Lands in agent-scoped read view (private)
         - Public summary aggregable only as typical-error broadcast
           when N similar autopsies cluster
         - Format: AgentAutopsyCapsule { agent_id, event_id, loss_reason_class,
           violated_risk_rule, suggested_policy_patch, evidence_cids,
           public_summary, private_detail_cid }
       Boltzmann masking:
         - Mask is read-view / scheduler policy ONLY
         - ChainTape never deletes the masked parent
         - Mask predicate: child_price > parent_price + margin
                        AND child_verification_status >= parent
                        AND child not under unresolved challenge
                        AND child liquidity >= min_threshold (anti-manipulation)
         - Otherwise parent remains in candidate set
         - Scheduler may sample parents with epsilon probability for exploration
       Price never overrides predicate.
       Predicate failure → masking has no effect; node is rejected regardless.
```

---

## §5 Sequencing summary

```text
v1.0 path (Lean Proof Task Market):
  TB-8  payout
  TB-9  durable identity
  TB-10 Lean Proof Task Market MVP
  TB-11 NodeMarket Decision + NodePosition (NO trading)
  TB-12 CompleteSet + MarketSeedTx
  TB-13 CPMM Router
  TB-14 PriceIndex + Boltzmann masking
  TB-15 Lamarckian Autopsy + Markov Log Loom
  TB-16 Beta with market signals
  v1.0 ship

v1.1+ (post-launch):
  TB-17 Full market trading (MarketBuyTx / MarketSellTx / LP positions)
  RSP-M7 experimental DPMM (if needed)
  public chain anchoring
  multi-org / MetaTape / royalty
```

The order is enforced by dependency: trading needs identity (TB-9), market position requires settlement primitive (TB-8), CompleteSet requires `on_init` budget allocation, and so on.
