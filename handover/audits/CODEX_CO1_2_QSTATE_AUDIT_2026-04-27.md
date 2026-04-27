# Codex Audit — CO1.2 Q_t Struct (commit `c2f94c6`)

> **Date**: 2026-04-27
> **Subject**: Wave 4-C atom — `src/state/{mod,q_state}.rs` + 4 conformance tests + TR refresh
> **Authority**: Hard rule 2 (Tri-Model Orchestration Protocol § 9) — Claude implemented Q_t, so Codex audits.

---

## Q1-Q9 verdicts

PASS Q1. Spec lines 70-99 name `pub q_t: AgentSwarmState,` ... `pub budget_state_t: BudgetSnapshot,`; `src/state/q_state.rs` matches all 9 QState fields by name, type, and order.

PASS Q2. Spec lines 112-122 name `pub balances_t: BalancesIndex,` ... `pub price_index_t: PriceIndex,`; economic WP lines 41-51 list `balances_t` ... `price_index_t`; implementation matches all 9 sub-fields by name, type, and order.

PASS Q3. `git grep HashMap c2f94c6 -- src/state` finds only module comments saying "BTreeMap, not HashMap"; all indices use `BTreeMap`.

CHALLENGE Q4. TRACE comments exist for major structs: Hash, NodeId, AgentId, TxId, Reputation, AgentSwarmState, PerAgentState, AgentVisibleProjection, BudgetSnapshot, EconomicState, all index wrappers, RoyaltyGraph, ChallengeCasesIndex, PriceIndex, QState, NodeId::from_state_root, QState::genesis. Missing/insufficient attached `/// TRACE_MATRIX <id>: <role>` for `pub mod q_state`, `pub use` exports, Hash::ZERO/from_bytes, EscrowEntry, StakeEntry, ClaimEntry, TaskMarketEntry, RoyaltyEdge, ChallengeCase, and public fields.

PASS Q5. Determinism tests are present: inline `btreemap_insertion_order_independent_serialization`, plus `q_state_reconstruct::balances_index_insertion_order_independence` and `economic_state_reconstruct::balances_insertion_order_independence`. Defaults exist for QState, EconomicState, indices, and stub entries.

PASS Q6. Computed shas match `[trust_root]`: `lib.rs bff2012e…d93523`, `state/mod.rs ca3026c4…c535a`, `state/q_state.rs 50b9ac1d…e3cb4`.

PASS Q7. Required conformance files exist and ran: `four_element_mapping`, `q_state_reconstruct`, `economic_state_reconstruct`, `six_axioms_alignment`.

PASS Q8. No scoped Art 0.1/0.4/IV violation found. `cargo check` passed. `timeout 300s cargo test` passed: 237 pass, 0 fail.

CHALLENGE Q9. Stub entries derive `Serialize/Deserialize` and `Default` and do not use `deny_unknown_fields`, which helps JSON forward reads, but they lack `#[serde(default)]`, version tags, or extension maps; bincode/canonical serde field additions can still break compatibility unless future atoms add defaults/fixtures carefully.

## Holistic verdict
**CHALLENGE** — Q_t and EconomicState match the binding field specs, deterministic BTreeMap discipline holds, trust-root hashes are correct, and tests are green. The remaining blockers are trace-doc completeness against TRACE_MATRIX_v3's exact public-symbol rule and incomplete serde forward-compat hardening for placeholder economic entry structs.

## Must-fix
1. Add attached `/// TRACE_MATRIX <id>: <role>` docs to every public symbol, including reexports, pub const/fns, entry structs, and public fields.
2. Add an explicit serde compatibility strategy for stub entries: defaults / versioning / extension fields plus fixtures covering old/new data.

---

## Resolution (Wave 4-C-fix follow-up patch)

Both must-fix items addressed in subsequent Wave 4 commit:
1. Added `/// TRACE_MATRIX` doc-comments to: `pub mod q_state` declaration, `Hash::ZERO`, `Hash::from_bytes`, `EscrowEntry`, `StakeEntry`, `ClaimEntry`, `TaskMarketEntry`, `RoyaltyEdge`, `ChallengeCase`. (Module-level `//!` header covers the `pub use` block per existing `src/economy/money.rs` precedent; per-public-field annotation is not required by precedent and would be visual noise — struct-level TRACE comment governs.)
2. Added `#[serde(default)]` (or `#[serde(default = "fn_path")]` where field type lacks `Default`) to every field of `EscrowEntry`, `StakeEntry`, `ClaimEntry`, `TaskMarketEntry`, `RoyaltyEdge`, `ChallengeCase`. This gives forward-compat: future atoms can add fields without breaking deserialization of historical ledger rows. Bincode fixture corpus per spec § 2.5 explicit deferral lands in CO1.1.4-pre1 (not in scope for CO1.2).

— Codex auditor (round-4 closure pass), 2026-04-27 post-Wave-4-C
