# REAL-17P21/P22 Polymarket Robustness Pivot Summary

Date: 2026-05-18

## Updated Goal

Current phase goal is no longer to strengthen the voluntary-emergence claim.
The phase goal is:

```text
检验 CPMM/router、余额扣减、份额发放、守恒、极端买卖序列和 NO/YES 双边路径的鲁棒性。
```

Forced Bull/Bear action is therefore treated as a Red-Track robustness
positive-control. It may exercise market code paths, but it is not voluntary
agent emergence evidence and must not be used for E2/E3/E4 or market-emergence
claim upgrades.

## Implemented Gates

### P21 Voluntary MarketOrderTicket Boundary

Files:

- `src/runtime/market_order_ticket.rs`
- `src/runtime/mod.rs`
- `experiments/minif2f_v4/src/bin/evaluator.rs`
- `tests/constitution_real17_market_order_ticket.rs`
- `tests/constitution_real13a_ev_decision_trace.rs`

What changed:

- Added Generic CAS `MarketOrderTicket` sidecar schema
  `real17p21.market_order_ticket.v1`.
- Ticket requires Bull/Bear role, side, action choice, quote preview,
  EV linkage, PromptCapsule linkage, and public constraints.
- `amount_micro=0` remains a valid voluntary abstain.
- Non-zero mandatory execution is rejected by `forced_nonzero_trade`.
- Evaluator now writes tickets for Bull/Bear EV turns when QState is available.

### P22 Red-Track Forced Bull/Bear Router Robustness

File:

- `tests/constitution_polymarket_smoke.rs`

New test:

```text
red_track_forced_bull_bear_router_sequence_preserves_polymarket_invariants
```

Coverage:

- deterministic Bull YES and Bear NO router sequence;
- tiny 1 micro orders fail closed with `RouterSwapInsufficientPoolOutput`;
- rejected tiny orders do not mutate state root;
- accepted orders debit buyer balance exactly by `pay_coin`;
- accepted orders credit buyer YES/NO shares by `pay_coin + out_shares`;
- pool reserves update according to integer CPMM/router logic;
- complete-set balance and total CTF conservation hold after every accepted tx;
- `k = pool_yes * pool_no` is non-decreasing across accepted router buys;
- final YES and NO aggregate shares equal collateral;
- audit price quotes do not mutate state.

Additional rejection-path gate:

```text
router_rejection_paths_preserve_state_and_report_specific_errors
```

Coverage:

- slippage guard rejects with `RouterSlippageExceeded`;
- insufficient buyer balance rejects with `RouterInsufficientCoinBalance`;
- post-finalization router attempt rejects with `EventNotOpen`;
- each rejection preserves state root and total Coin supply.

Additional near-depletion stress gate:

```text
extreme_forced_router_sequence_near_depletion_preserves_invariants
```

Coverage:

- large forced YES/NO buy sequence near pool depletion;
- tiny no-output orders fail closed with `RouterSwapInsufficientPoolOutput`;
- accepted orders preserve exact buyer balance debit, share credit, pool reserve
  transition, complete-set balance, CTF conservation, and non-decreasing CPMM k;
- rejected no-output orders preserve state root and total Coin supply.

Additional settlement/redeem gate:

```text
forced_router_holdings_redeem_on_yes_and_no_resolution
```

Coverage:

- forced Bull YES position resolves via EventResolve YES and redeems YES;
- forced Bear NO position resolves via EventResolve NO and redeems NO;
- redeem credits Coin 1:1, burns winning-side shares, debits event collateral
  1:1, and preserves total CTF conservation.

Additional settlement rejection gate:

```text
settlement_rejects_wrong_side_and_double_redeem_without_state_mutation
```

Coverage:

- wrong-side redeem after YES resolution rejects with `InvalidResolutionRef`;
- second redeem after winning-side shares are already burned rejects with
  `RedeemMoreThanOwned`;
- both rejection paths preserve state root and total Coin supply.

Additional partial redeem gate:

```text
settlement_partial_redeems_preserve_conservation_until_position_empty
```

Coverage:

- forced winning YES position redeems in two partial slices;
- first partial redeem leaves exact residual winning shares;
- second partial redeem empties the winning position exactly;
- each partial redeem preserves CTF conservation and complete-set balance.

## Verification

Commands run:

```bash
cargo fmt --all -- --check
cargo check --manifest-path experiments/minif2f_v4/Cargo.toml --bin evaluator
cargo test --test constitution_real17_market_order_ticket -- --test-threads=1
cargo test --test constitution_real13a_ev_decision_trace -- --test-threads=1
cargo test --test constitution_polymarket_smoke red_track_forced_bull_bear_router_sequence_preserves_polymarket_invariants -- --test-threads=1
cargo test --test constitution_polymarket_smoke router_rejection_paths_preserve_state_and_report_specific_errors -- --test-threads=1
cargo test --test constitution_polymarket_smoke forced_router_holdings_redeem_on_yes_and_no_resolution -- --test-threads=1
cargo test --test constitution_polymarket_smoke settlement_rejects_wrong_side_and_double_redeem_without_state_mutation -- --test-threads=1
cargo test --test constitution_polymarket_smoke -- --test-threads=1
cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo -- --test-threads=1
```

Observed results:

- `constitution_real17_market_order_ticket`: 2 passed / 0 failed.
- `constitution_real13a_ev_decision_trace`: 37 passed / 0 failed.
- P22 forced Bull/Bear robustness test: 1 passed / 0 failed.
- P22 router rejection-path test: 1 passed / 0 failed.
- P22 near-depletion stress test: included in full smoke, passed.
- P22 forced settlement/redeem test: 1 passed / 0 failed.
- P22 settlement rejection test: 1 passed / 0 failed.
- P22 partial redeem test: included in full smoke, passed.
- Full `constitution_polymarket_smoke`: 7 passed / 0 failed.
- Evaluator binary check passed.
- Trust Root verification passed after envelope-local rehash.

Updated Trust Root hashes:

- `experiments/minif2f_v4/src/bin/evaluator.rs`
  `3d74bd2486f738d0d53fbef0d9195bdc303db0fc77a84fc4257f136aa56a6eb2`
- `src/runtime/mod.rs`
  `05bf7151e9e136620f3dd0af32f368b330dc4ed3533a88f871d969a1dca06126`

## Claim Boundary

This packet supports a robustness statement only:

```text
CPMM/router Red-Track forced Bull/Bear positive-control preserves core
Polymarket invariants for the tested tiny/rejected and accepted YES/NO paths.
```

It does not support any voluntary emergence upgrade.

## Next Robustness Work

Recommended next checks:

1. Add a larger deterministic stress table with near-depletion orders and
   slippage thresholds.
2. Add replay verifier coverage from ledger entries rather than only in-memory
   harness state.
3. Add broader settlement/redeem stress with partial redemptions and replay
   binding to ledger entries.
