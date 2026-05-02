# DECISION — MarketSeedTx, NO Ghost Liquidity — 2026-05-02

**Authority**: architect directive 2026-05-02, ruling 7 + ruling 8 + ruling 11.3 of Part C.
- Source: `handover/directives/2026-05-02_lossless_constitution_polymarket_directive.md`
- Verbatim trigger: `..._part_C_updated_final_ruling.md` §2.2 + §4 ("「每个新节点自动注入 YES/NO 各 100」必须重写")
**Status**: ratified by user authorization 2026-05-02.
**Why this record exists**: the Polymarket work-notes referenced in the directive contained the literal phrase "每个新节点被创造的时候，系统自动往里面注入 YES NO 代币各 100 个". Implemented literally, this is **ghost liquidity** — YES/NO shares created without locked Coin collateral. This decision freezes the rejection so future TBs cannot re-import the unguarded form.

---

## §1 Decision

```text
PROHIBITED:
  Any path by which YES/NO shares (or pool reserves) appear in state
  without an antecedent Coin debit on a debit-source account.

REQUIRED:
  Every YES/NO share that exists in state must trace back through
  a chain of CompleteSetMintTx → debit on some balances_t account.

  Every pool reserve unit must trace back to either:
    (a) a buyer's MarketBuyTx via the CPMM router, OR
    (b) a LiquiditySeedTx / LiquidityDepositTx with an explicit
        provider account (treasury, LP, sponsor) debit.
```

---

## §2 LiquiditySeedTx / MarketSeedTx canonical structure

```rust
LiquiditySeedTx {
    tx_id: TxId,
    parent_state_root: StateRoot,
    event_id: EventId,
    provider: Either<AgentId, TreasuryId>,     // explicit funding source
    collateral_amount: MicroCoin,              // Coin debited from provider
    pool_y_amount: ShareAmount,                // = collateral_amount
    pool_n_amount: ShareAmount,                // = collateral_amount
    lp_shares_out: LpShareAmount,              // claim on pool to provider
    signature: Either<ProviderSig, RuntimeSig>,
}
```

Atomic effect:

```text
balances_t[provider]                  -= collateral_amount
conditional_collateral_t[event_id]    += collateral_amount
pool_y_reserve[event_id]              += collateral_amount
pool_n_reserve[event_id]              += collateral_amount
lp_shares[event_id, provider]         += lp_shares_out

TotalCoin unchanged.
```

LP shares are **claims on pool reserves**, NOT Coin. They are NOT counted in `monetary_invariant::total_supply_micro`.

---

## §3 `on_init` MarketMakerBudget allowance

If TuringOS wants the *behavior* of "every new node has initial YES/NO seed liquidity" — which is desirable for price discovery on freshly-created nodes — it MUST be funded explicitly:

```text
on_init genesis:
  treasury_market_maker_budget = K  Coin
  (K is constitution-bounded; e.g., a fraction of total_supply_micro
   reserved for protocol-provided liquidity)

at node creation, if MarketMakerBudget >= seed_amount:
  emit LiquiditySeedTx(provider=Treasury, collateral_amount=seed_amount)
  treasury_market_maker_budget -= seed_amount
else:
  node ships WITHOUT seeded liquidity (graceful degradation; LPs may seed later)
```

Two consequences:

```text
1. The "automatic" feel is preserved IF budget allows.
2. The constitution invariant is preserved ALWAYS — Treasury can run out;
   it cannot create Coin from nothing.
```

`on_init` is the constitution's only legal mint point (Law 2 in `constitution.md:160`). MarketMakerBudget is allocated AT `on_init` and consumed thereafter; never refilled by future mint.

---

## §4 ProtocolMMRiskBudget (TB-13 forward consideration)

The CPMM LP — including the Treasury when it acts as protocol market-maker — can lose Coin to informed traders (adverse selection). This is fundamental to constant-product markets.

The directive Part C §2.4 absorbs this as an explicit `ProtocolMMRiskBudget`:

```rust
ProtocolMMBudget {
    treasury_account: TreasuryId,
    max_loss_per_event: MicroCoin,
    max_total_loss_per_epoch: MicroCoin,
    realized_pnl: MicroCoin,
    mark_to_market_pnl: MicroCoin,
}
```

Constraints:

```text
- Loss debits Treasury balances_t (not future mint).
- Treasury cannot be overdrawn — if seeded budget is exhausted, market makers stop.
- Max-loss thresholds gate further seeding / quoting.
- Do NOT use this to justify removing RSP bounty / escrow.
  Bounty + market are complementary (per directive ruling 9).
```

---

## §5 Hard prohibitions

```text
1. NO automatic per-node mint of YES/NO without explicit collateral.
2. NO Treasury overdraft — the budget must be ≥ 0 at all times.
3. NO future-mint compensation for LP loss — losses come from existing
   Treasury balance; if depleted, protocol stops market-making, full stop.
4. NO removal of RSP bounty.
   "系统做市商可以 0 亏损 → 不要悬赏金" is REJECTED.
   Bounty drives baseline labor; market drives price discovery; both coexist.
5. NO per-node quietly-replenished budget — refills require explicit
   policy decision, not silent accounting.
```

---

## §6 Test obligations (when TB-12 ships LiquiditySeedTx)

```text
1. Seed-without-debit panic:
   - LiquiditySeedTx with provider balance < collateral_amount → InsufficientFunds error.
   - No state mutation occurs on this path.

2. Treasury exhaustion behavior:
   - When MarketMakerBudget = 0, node creation MUST succeed but emit no seed tx.
   - Smoke test: create N nodes after exhaustion; verify ConditionalCollateralIndex unchanged.

3. Coin conservation across seed:
   - Before LiquiditySeedTx: TotalCoin = T.
   - After: TotalCoin = T (Coin moved from balances_t to conditional_collateral_t, not created).

4. LP shares NOT in supply:
   - Sum of lp_shares does not contribute to total_supply_micro.

5. Reverse direction (LiquidityWithdrawTx — TB-13+):
   - LP withdrawal is pro-rata on pool reserves, not fixed Coin.
   - Withdrawn Coin returns to provider balances_t; lp_shares burn.
   - Net Coin change = 0 across mint+withdraw.
```

---

## §7 Constitutional alignment

```text
constitution.md:159  Law 1: Information is Free
constitution.md:160  Law 2: Only Investment Costs Money — 1 Coin = 1 YES + 1 NO; on_init 是唯一合法铸币点
```

Every clause of this decision RESTATES Law 2's `on_init`-唯一合法铸币点 in operational terms. No constitution amendment is required. Any future "easy fix" that proposes to relax this clause must enter a Class 4 sudo-only flow.
