# DECISION — Polymarket Core (CTF semantics) — 2026-05-02

**Authority**: architect directive 2026-05-02, ruling 2 + ruling 11.1 of Part C.
- Source: `handover/directives/2026-05-02_lossless_constitution_polymarket_directive.md`
- Verbatim: `handover/directives/..._part_C_updated_final_ruling.md` §2.2, §13.1, §14
**Status**: ratified by user authorization 2026-05-02 (option C, "you decide to best match directive" → C1 create all four).
**Supersedes**: nothing (first formal Polymarket-core decision in v4).
**Companion records** (same dated batch):
- `DECISION_CPMM_MINT_AND_SWAP_2026-05-02.md`
- `DECISION_MARKET_SEED_NO_GHOST_LIQUIDITY_2026-05-02.md`
- `DECISION_LAMARCKIAN_AUTOPSY_BOLTZMANN_2026-05-02.md`

---

## §1 Decision

```text
1 locked Coin  =  1 YES_E  +  1 NO_E

YES/NO shares are CLAIMS, not Coin supply.
Locked Coin is the holding.
Price is statistical signal (Art. II.2), NEVER truth.
Truth is decided by predicates + ChallengeResolveTx, NEVER by price.
```

---

## §2 Construct mapping

| TuringOS construct | Polymarket / CTF analogue | Constraint |
|---|---|---|
| `WorkTx.stake` | proposer's first-long YES exposure | inline (no new TypedTx variant) — per `feedback_wp_vs_roadmap_reconciliation` |
| `ChallengeTx.stake` | challenger's short / NO exposure | inline |
| `VerifyTx.bond` | responsibility bond | NOT a market position |
| `ConditionalCollateralIndex` (TB-12) | locked Coin per event | grows by `MarketSeedTx`/`CompleteSetMintTx` |
| `ShareBalancesIndex` (TB-12) | YES/NO claim ledger | NOT counted in Coin supply |
| `PriceIndex` (TB-11) | price signal | derived view; never authoritative |

---

## §3 Constitutional alignment

Verified against `constitution.md`:

```text
constitution.md:159  Law 1: Information is Free
constitution.md:160  Law 2: Only Investment Costs Money — 1 Coin = 1 YES + 1 NO (CTF 守恒); on_init 是唯一合法铸币点
```

This decision RESTATES Law 2 in Polymarket terminology. It does NOT amend the constitution. Per directive ruling 15: "Do not modify constitution.md unless explicitly sudo-authorized."

---

## §4 Hard prohibitions

```text
1. NO ghost liquidity.
   - YES/NO shares NEVER created without locked Coin collateral.
   - "Each new node automatically gets 100 YES + 100 NO" is REJECTED literally.

2. NO supply expansion.
   - YES + NO redemption = exactly 1 Coin (1 winning side gets 1 Coin; losing side gets 0).
   - TotalCoin invariant unchanged across CompleteSetMint / Merge / Redeem.

3. NO price-as-truth.
   - Boltzmann masking can use price as scheduling hint.
   - Price NEVER overrides predicate failure.
   - Price NEVER moves a node from L4.E to L4.

4. NO market replacing predicates.
   - Lean / VerificationResult / ChallengeResolveTx remain authoritative.
   - Even when market price approaches 1.0 for YES, predicate failure still rejects the proposal.
```

---

## §5 When this lands in code

```text
TB-11   NodePosition + PriceIndex (no CTF mutation; just exposure index)
TB-12   CompleteSet + MarketSeedTx (this decision becomes executable code)
TB-13   CPMM Router (per DECISION_CPMM_MINT_AND_SWAP)
TB-14   Boltzmann masking (per DECISION_LAMARCKIAN_AUTOPSY_BOLTZMANN)
```

NOT in code before TB-12. TB-8/TB-9/TB-10 are bounty/payout/identity/Lean-market-MVP — they implicitly assume Coin conservation but do not yet exercise CompleteSet algebra.

---

## §6 Test obligations (when TB-12 ships)

```text
1. CompleteSetMintTx invariant:
     before:  balances_t[agent] = X, conditional_collateral_t[event] = C
     after:   balances_t[agent] = X - n, conditional_collateral_t[event] = C + n
              share_balances[agent].YES_e += n, share_balances[agent].NO_e += n
     TotalCoin unchanged.

2. CompleteSetMergeTx invariant: inverse of mint.

3. CompleteSetRedeemTx invariant:
     after event resolves to YES: share_balances.YES_e * 1 → balances_t[agent]
                                  share_balances.NO_e  * 0
     conditional_collateral_t[event] -= redeemed_amount
     TotalCoin unchanged.

4. No agent can submit CompleteSet system tx — agent submits Mint/Merge,
   system emits Redeem after resolution.

5. ShareBalances NOT included in monetary_invariant::total_supply_micro.

6. Replay determinism: same chain → same ConditionalCollateralIndex + ShareBalancesIndex.
```

---

## §7 Open follow-ups (NOT TB-12 blockers)

- `E_progress` market (price of "node moves toward final goal") deferred to RSP-M7+.
- DPMM / pro-rata payout deferred to RSP-M7 experimental — must NOT be mixed with CTF semantics.
- Multi-event correlated CTF (settlement_rule_hash interpretation) deferred to TB-11+ scope per existing TB-8 forbidden-line.
