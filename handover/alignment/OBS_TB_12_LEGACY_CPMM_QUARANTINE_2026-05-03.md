# OBS — TB-12 legacy CPMM quarantine prerequisite for TB-13

**Date**: 2026-05-03.
**Status**: OBS (observation; tracked for future TB).
**Triggered by**: Codex TB-12 ship audit Q5 CHALLENGE
(`handover/audits/CODEX_TB_12_SHIP_AUDIT_2026-05-03.md`).
**Audit verdict**: CHALLENGE on Q5 resolved as out-of-scope-for-TB-12
(see RECURSIVE_AUDIT_TB_12_2026-05-03.md §10).

## Summary

`src/prediction_market.rs` (345 lines) is legacy Tier 0 CPMM
scaffolding from early v4 (pre-2026-05 architect ruling on
TB-13/TB-14 trajectory). It violates the post-2026-05 architect
forbidden list:

- **f64 arithmetic** in `BinaryMarket` (yes_reserve / no_reserve / k /
  lp_total) — architect 2026-05-02 directive Part C line 1574 + §9.4
  TB-13 CR-13.5 explicit no-f64-mutation rule.
- **Automatic liquidity** via constant-product market-maker — architect
  §9.4 TB-13 forbidden list "No automatic liquidity. No ghost
  liquidity."
- **Trading semantics** (`buy_yes` / `BuyOutcome`) — architect §9.4
  TB-12 forbidden + §9.4 TB-13 + TB-14 forbidden.

Consumed by `src/kernel.rs:9-67`:
```rust
use crate::prediction_market::{BinaryMarket, MarketError};
pub markets: HashMap<NodeId, BinaryMarket>,
pub bounty_market: Option<BinaryMarket>,
```

## TB-12 boundary

TB-12 = Node Exposure Index added zero new code touching
`prediction_market.rs` or its kernel.rs consumers. Verified via
`grep -rn "BinaryMarket\|prediction_market" $(git diff 6ab165c..HEAD --name-only)`
returning empty for TB-12 commits (5ada28d → f4bff3f).

NodePosition (TB-12 atom 1) is a SEPARATE flat index canonical to
EconomicState; it does NOT consume or extend BinaryMarket.

## Roadmap replacement

Per architect 2026-05-02 supplementary directive
(`handover/directives/2026-05-02_TB11_TO_TB17_SUPPLEMENTARY_DIRECTIVE.md`):

- **TB-13 CompleteSet + MarketSeedTx**: introduces integer-math
  CTF-conserving YES/NO conditional shares. Replaces BinaryMarket's
  CPMM YES/NO accounting.
- **TB-14 PriceIndex v0**: computes price as
  `long_interest / (long_interest + short_interest)` from
  `node_positions_t` (TB-12 schema). NO automatic liquidity. NO CPMM.
  Replaces BinaryMarket's price discovery + its CPMM math.

After TB-14 ships, `src/prediction_market.rs` + `src/kernel.rs` market
scaffolding becomes architecturally dead. **Quarantine /
deprecation / removal is required before TB-14 SHIP** to prevent
the new architecture from inheriting f64 / automatic-liquidity
artefacts.

## Action plan (TB-13 prerequisite)

A future TB-13 atom 0.5 (carry-forward, mirroring TB-12 Atom 0.5
carry-forward pattern) MUST:

1. Audit which `src/kernel.rs` paths still consume `BinaryMarket`.
2. Either:
   a. Replace `BinaryMarket` consumers with new TB-13 CompleteSet
      conditional-share types, OR
   b. Feature-gate the legacy paths behind `#[cfg(feature = "legacy_cpmm")]`
      (off by default; explicit opt-in for migration tests only), OR
   c. Delete outright if no production consumers remain.
3. Remove `pub mod prediction_market;` from `src/lib.rs` once consumers
   are gone.
4. Remove the `BinaryMarket` Trust Root manifest entries (if any).
5. Ship-gate the migration with a forbidden-token grep that fails on
   any `BinaryMarket` / `buy_yes` / `f64 reserve` reference outside
   `cfg(feature = "legacy_cpmm")`.

## Why this is OBS-tracked, not blocker

Per `feedback_no_retroactive_evidence_rewrite`:

> New evidence requirements ... apply going-forward only. NEVER rewrite
> old ledger roots ... fabricate genesis_report into old dirs, or relabel
> old `evaluator-attested` results as `chain-oracle-derived`.

The architect's 2026-05-02 + 2026-05-03 forbidden-token rules are
forward-binding for NEW code in TB-12 onward. They do NOT auto-remove
pre-existing v3-style scaffolding. TB-12 honored the rule by adding
zero new trading code. Quarantine of pre-existing scaffolding is the
TB-13 prerequisite that the architect's ruling implies via
"replace, not extend" semantics.

## Cross-references

- Codex audit doc: `handover/audits/CODEX_TB_12_SHIP_AUDIT_2026-05-03.md` Q5
- TB-12 recursive self-audit §10 remediation log: `handover/audits/RECURSIVE_AUDIT_TB_12_2026-05-03.md`
- Architect supplementary directive (TB-13 + TB-14 spec): `handover/directives/2026-05-02_TB11_TO_TB17_SUPPLEMENTARY_DIRECTIVE.md`
- Architect 2026-05-03 ruling (TB-12 forbidden list): `handover/directives/2026-05-03_TB12_NODE_EXPOSURE_INDEX_ARCHITECT_RULING.md` §9.4
