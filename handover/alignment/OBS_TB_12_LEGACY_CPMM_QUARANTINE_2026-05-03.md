# OBS — TB-12 legacy CPMM quarantine prerequisite for TB-13

**Date**: 2026-05-03; CLOSED 2026-05-04.
**Status**: **RESOLVED** — `src/prediction_market.rs` was excised entirely in
TB-14 Atom 6 (commit closing this OBS); forward-fence test ships at
`tests/tb_13_legacy_cpmm_forward_fence.rs`. See §10 below for the
2026-05-04 architect §3 anti-drift verification.
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
- Architect 2026-05-03 post-TB-12 ruling (TB-13 Atom 0.5 mandate): `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md` Part A §4.2
- TB-13 charter: `handover/tracer_bullets/TB-13_charter_2026-05-03.md`

---

## TB-13 Atom 0.5 update (2026-05-03 evening — forward-fence + label IN PLACE)

**Status update**: open OBS still tracked; forward-fence ship-gate added in TB-13 Atom 0.5; hard removal **carries forward to TB-14 SHIP prerequisite** unchanged.

### What TB-13 Atom 0.5 added

1. **Module-header label** in `src/prediction_market.rs` — ` //! # LEGACY — ...` doc-comment block declaring: not constitutional, not RSP-M, not production market path; lists each constitutional non-compliance (f64 / automatic liquidity / trading semantics); names the carry-forward owner (TB-14 SHIP prerequisite).
2. **Field-level labels** in `src/kernel.rs` — every CPMM-bearing field (`markets`, `bounty_market`, `bounty_lp_seed`) carries `**LEGACY** ...` doc-comments naming the migration path (TB-13 `CompleteSetMintTx` / `ConditionalShareBalances` + TB-14 `PriceIndex`).
3. **Forward-fence ship-gate test** at `tests/tb_13_legacy_cpmm_forward_fence.rs` — three EXACT-named tests per architect §4.2:
   - `legacy_cpm_api_not_imported_by_complete_set` (SG-13.0.1)
   - `no_f64_in_complete_set_or_market_seed` (SG-13.0.2)
   - `prediction_market_legacy_quarantined` (SG-13.0.3)
4. **OBS carry-forward** (this update) — SG-13.0.4 satisfied as "explicitly carried as non-importable legacy".

### What TB-13 Atom 0.5 deliberately did NOT do

- **No retroactive deletion** of `src/prediction_market.rs` or `src/kernel.rs` market scaffolding. Production callers at `src/bus.rs:206 / 327 / 359 / 480-515` and `experiments/minif2f_v4/src/bin/evaluator.rs:1323` plus 10+ test files (`tests/tb_6_*`, `tests/tb_7_*`, `tests/wal_resume.rs`, `tests/fc_alignment_conformance.rs`) would break.
- **No removal** of `pub mod prediction_market;` from `src/lib.rs` for the same reason.
- **No `#[cfg(feature = "legacy_cpmm")]` feature gate** — that would still require touching every consumer; same surface area; defers no work.

This decision is consistent with `feedback_no_retroactive_evidence_rewrite` (forward-binding rules apply going-forward only) and architect §4.2 halting-trigger semantics (which target NEW TB-13 code, not existing scaffolding).

### TB-14 SHIP prerequisite (unchanged)

Per the original action plan above, before TB-14 SHIP one of these MUST be done:

a. Replace `BinaryMarket` consumers with new TB-13 + TB-14 conditional-share + price-signal types, OR
b. Feature-gate behind `#[cfg(feature = "legacy_cpmm")]` (off by default; explicit opt-in for migration tests only), OR
c. Delete outright if no production consumers remain.

Either way: remove `pub mod prediction_market;` from `src/lib.rs`; remove `BinaryMarket` Trust Root entries (if any); ship-gate the migration with a forbidden-token grep that fails on any `BinaryMarket` / `buy_yes` / `f64 reserve` reference outside `cfg(feature = "legacy_cpmm")`.

---

## §10 2026-05-04 architect §3 anti-drift verification — RESOLVED

### §10.1 Architect §3 concern (verbatim)

> 我的第一个路线修正是：TB-13 的 Atom 0.5 必须是 legacy CPMM quarantine。
> 不是"顺手做"，而是 TB-13 的前置 ship gate.

### §10.2 Verification at HEAD `3f7535d` (post-TB-16.x.1)

**File excision (stronger than quarantine)**:

```bash
$ find . -name "prediction_market.rs"
(empty)
$ grep -rn "^mod prediction_market\|pub mod prediction_market" src/
(empty)
```

The file was deleted outright in TB-14 Atom 6 — excision strictly dominates
quarantine. Surviving cross-references in source are all comments
documenting the excision (`src/bus.rs`, `src/kernel.rs`, `src/sdk/actor.rs`,
`src/sdk/snapshot.rs`, `src/state/price_index.rs`).

**Forward-fence test result** (programmatically enforces architect §4.2 HALT
triggers):

```text
$ cargo test --release --test tb_13_legacy_cpmm_forward_fence
test prediction_market_legacy_quarantined ............................ ok
test no_f64_in_complete_set_or_market_seed ........................... ok
test legacy_cpm_api_not_imported_by_complete_set ..................... ok
test audit_dashboard_in_layer_1_scope_but_not_layer_2_scope .......... ok
test discover_by_type_use_catches_unmarked_imports_and_skips_doc_xref . ok
test discover_by_type_use_skips_successor_tb_marker_files ............ ok
test tb_13_scan_lines_handles_marker_and_unmarked_files .............. ok
test tb_13_scan_lines_partial_marker_catches_stealth_type_use ........ ok
test result: ok. 8 passed; 0 failed.
```

**f64 in TB-13/14 economic paths**: 0 hits. The single grep hit at
`src/state/typed_tx.rs:2814` is a sha256 hex literal containing the
substring `0f64`, not a `f64` floating-point type.

### §10.3 Verdict — PASS

| Architect requirement | Shipped state | Verdict |
|---|---|---|
| `src/prediction_market.rs` quarantined | File deleted (excision; stronger) | ✅ |
| TB-13 Atom 0.5 legacy CPMM forward-fence | 8/8 tests PASS at HEAD | ✅ |
| No f64 in CompleteSet/MarketSeed code | 0 hits in fence + grep | ✅ |
| No AMM/CPMM router in TB-13 | Forward-fence covers; 0 hits | ✅ |
| Architect §4.7 forbidden list | All 11 items honored | ✅ |

No follow-up TB or fix needed. Architect 2026-05-03 §3 anti-drift directive
is **CLOSED**. This OBS's TB-14-SHIP-prerequisite from §1 (a/b/c) was
satisfied by **option (c) — delete outright** in TB-14 Atom 6.

### §10.4 Why this paper trail exists

Per `feedback_session_label_codification` and architect §9 anti-drift rules,
audit verifications must leave a paper trail even when the result is
"shipped state already complies". Future reviewers (or architect re-reads)
can confirm the §3 follow-up was actually executed, not silently treated
as "obviously fine".
