# DECISION — CPMM Mint-and-Swap Router — 2026-05-02

**Authority**: architect directive 2026-05-02, ruling 6 + ruling 11.2 of Part C; CPMM derivation in Part C §3.
- Source: `handover/directives/2026-05-02_lossless_constitution_polymarket_directive.md`
- Verbatim: `..._part_C_updated_final_ruling.md` §3
**Status**: ratified by user authorization 2026-05-02.
**Sequencing**: lands in code at TB-13 (CPMM Router), AFTER TB-12 (CompleteSet) and TB-11 (NodePosition / PriceIndex). NOT in TB-8/TB-9/TB-10/TB-12.

---

## §1 Decision

```text
poolY * poolN = k                 (constant product)

buy_yes(payC):
  outY  = payC * poolY / (payC + poolN)
  getY  = payC + outY
  priceY = payC / getY

buy_no(payC) symmetric (swap Y ↔ N).
```

The router is **explicit-collateral**. Every Coin going through the router must trace to either:
- `payC` from a debit on the buyer's `balances_t`, OR
- `LiquiditySeedTx` / `LiquidityDepositTx` from a treasury / LP / sponsor `balances_t` debit (per `DECISION_MARKET_SEED_NO_GHOST_LIQUIDITY_2026-05-02.md`).

---

## §2 Router flow (verbatim from directive Part C §3)

```text
User pays payC Coin.

Step 1 — Mint complete set (NOT free mint):
  CompleteSetMintTx:
    balances_t[buyer]                  -= payC
    conditional_collateral_t[event]    += payC
    share_balances[buyer].YES_e         += payC
    share_balances[buyer].NO_e          += payC
  TotalCoin unchanged.

Step 2 — Buyer keeps payC YES.

Step 3 — Router pushes payC NO into pool:
  dN = payC

Step 4 — Constant product determines YES out:
  (poolY + dY) * (poolN + payC) = poolY * poolN
  dY = - payC * poolY / (payC + poolN)

Step 5 — Buyer receives extra YES from pool:
  outY = -dY = payC * poolY / (payC + poolN)
  getY = payC + outY  (kept YES + swapped YES)

Step 6 — Pool reserves update:
  poolY1 = poolY - outY
  poolN1 = poolN + payC

Step 7 — Invariant check:
  poolY1 * poolN1 == poolY * poolN  (modulo rounding rules; see §4)
```

---

## §3 buy_no symmetry

```text
buy_no(payC):
  Step 1 — same CompleteSetMintTx (mint payC YES + payC NO)
  Step 2 — buyer keeps payC NO
  Step 3 — router pushes payC YES into pool: dY = payC
  Step 4 — solve dN = - payC * poolN / (payC + poolY)
  Step 5 — outN = -dN; getN = payC + outN
  Step 6 — poolY1 = poolY + payC; poolN1 = poolN - outN
  Step 7 — invariant.
```

---

## §4 Rounding rule (TB-13 implementation requirement)

CPMM math operates on `MicroCoin` (integer). Rounding must be deterministic and conservative — pool can never lose Coin to rounding:

```text
outY rounded DOWN (buyer receives floor; pool retains rounding crumbs).
dY computed exactly from outY: dY = -outY (so pool reserve update is exact).
After every buy:
   poolY1 * poolN1  >=  poolY * poolN   (k may grow by rounding crumbs, never shrink)
```

This is the standard Uniswap-V2-style conservative-rounding rule. Test suite must include rounding-direction assertion.

---

## §5 Hard prohibitions

```text
1. NO ghost liquidity.
   - All CompleteSetMint must come from buyer or LP balances_t debit.
   - Pool reserves must originate from MarketSeedTx / LiquidityDepositTx with explicit funding source.

2. NO supply expansion.
   - YES/NO shares are claims, not Coin.
   - Coin TotalSupply unchanged across any router transaction.

3. NO router-emitted system tx.
   - Router is a state mutation triggered by user-submitted MarketBuyTx (or equivalent).
   - User signs the buy; router computes the trade; sequencer applies atomically.

4. NO trading before TB-13.
   - Even if TB-11 PriceIndex exists earlier, no MarketBuyTx / MarketSellTx is dispatched until TB-13.

5. NO LP withdrawal before pool unwind.
   - LP shares represent claim on pool reserves; withdrawal computes pro-rata reserve withdrawal,
     not a fixed Coin amount.
```

---

## §6 Test obligations (when TB-13 ships)

```text
1. Constant-product invariant test:
     for buy_yes(payC) and buy_no(payC) over a range of (poolY, poolN, payC):
       poolY1 * poolN1 >= poolY * poolN   (rounding crumbs ≥ 0)

2. Mathematica-derivation match test:
     buy_yes formula matches:
       outY = payC * poolY / (payC + poolN)
     unit values verified against the directive's Mathematica derivation.

3. Symmetry test:
     buy_no(payC) on (poolY=A, poolN=B) ≡ buy_yes(payC) on (poolY=B, poolN=A) under Y↔N swap.

4. Slippage monotonicity:
     larger payC → strictly larger priceY (more slippage).

5. No supply increase:
     monetary_invariant::total_supply_micro unchanged across any router invocation.

6. Replay determinism:
     same chain → identical pool state.
```

---

## §7 Open follow-ups (NOT TB-13 blockers)

- LP fees: optional in TB-13. Default: zero fee. Fee model deferred to TB-14+ if needed.
- Concentrated liquidity (Uniswap V3-style): out of scope; CompleteSet + V2-style pool is the v1.x ceiling.
- AMM-vs-CLOB: per directive Part C §RSP-M4, the future TuringOS direction may be CLOB-first (closer to current Polymarket trading), with AMM as optional. TB-13 ships the optional AMM. CLOB belongs to a separate decision record if/when prioritized.
