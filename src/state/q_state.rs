//! Q_t — system state vector per `STATE_TRANSITION_SPEC v1.4 § 1.1`.
//!
//! TRACE_MATRIX Art 0.1 — 四要素映射: `QState` provides the tape/control mapping.
//! TRACE_MATRIX Art 0.4 — Q_t version-controlled: `head_t` = git commit SHA in Path B substrate.
//! TRACE_MATRIX Art IV — Boot: `QState::genesis` is the starting state of every runtime.
//! TRACE_MATRIX WP § 0 axiom 1 — state monotonicity: Q_t evolves only via accepted transitions.
//! TRACE_MATRIX WP § 4 — 9-component system state.
//! TRACE_MATRIX WP § 2 economic — `EconomicState` 9 sub-fields (CO1.2.2).
//!
//! **BTreeMap, not HashMap, everywhere** (Inv determinism;
//! Codex flagged `kernel.rs:187-204` HashMap nondeterminism in round-2).
//!
//! Sub-types whose entry shapes are scoped to later atoms (CO P2.x economic engine,
//! CO1.7 transition_ledger) are intentionally minimal here — full schemas land per atom,
//! but the *index typing* (BTreeMap newtype shells) freezes here so Q_t is total.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::economy::money::MicroCoin;

// ────────────────────────────────────────────────────────────────────────────
// Newtype primitives — minimal, deterministic, serde-ready.
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 1.1 — generic 32-byte hash (sha256). State / ledger / registry roots.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Hash(pub [u8; 32]);

impl Hash {
    /// TRACE_MATRIX § 1.1 — additive identity (genesis state-root, ledger-root, etc.).
    pub const ZERO: Hash = Hash([0u8; 32]);

    /// TRACE_MATRIX § 1.1 — construct from a 32-byte digest (sha256 output).
    pub fn from_bytes(b: [u8; 32]) -> Self {
        Hash(b)
    }
}

impl Default for Hash {
    fn default() -> Self {
        Hash::ZERO
    }
}

/// TRACE_MATRIX Art 0.4 — `head_t` = git commit SHA in Path B substrate (40 hex chars).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct NodeId(pub String);

impl NodeId {
    /// TRACE_MATRIX § 3 — pseudocode `NodeId::from_state_root(state_root)` constructor.
    /// Concrete derivation (commit-tree-of-state-root) lands in CO1.7 transition_ledger.
    pub fn from_state_root(state_root: Hash) -> Self {
        let mut s = String::with_capacity(64);
        for byte in state_root.0.iter() {
            s.push_str(&format!("{:02x}", byte));
        }
        NodeId(s)
    }
}

/// TRACE_MATRIX § 1.1 — agent identity (string, opaque to Q_t).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct AgentId(pub String);

/// TRACE_MATRIX § 1.1 — accepted-transaction id (string, opaque to Q_t).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct TxId(pub String);

/// TRACE_MATRIX § 1.1 — reputation snapshot. Signed i64 to permit negative reputation
/// (e.g. post-slash); ledger-of-record lives in `ReputationsIndex` (CO P2.9).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct Reputation(pub i64);

// ────────────────────────────────────────────────────────────────────────────
// AgentSwarmState + PerAgentState — spec § 1.1 verbatim.
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 1.1 — agent swarm sub-state.
/// MUST be reconstructible from L4 transition ledger replay.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AgentSwarmState {
    pub agents: BTreeMap<AgentId, PerAgentState>,
    pub current_round: u64,
}

/// TRACE_MATRIX § 1.1 — per-agent runtime state.
/// `retry_counter_for_current_task` resets on accept; persists across rejections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PerAgentState {
    pub reputation_snapshot: Reputation,
    pub last_accepted_tx: Option<TxId>,
    pub retry_counter_for_current_task: u32,
}

// ────────────────────────────────────────────────────────────────────────────
// AgentVisibleProjection — Inv 10 Goodhart shield (CO P2.7 visibility runtime).
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 1.1 — agent-visible projection of tape filtered by per-agent
/// visibility policy (Inv 10 Goodhart shield; `top_white::predicates::visibility`).
///
/// `views`: per-agent filtered head pointer; full filtering machinery lands in CO P2.7.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AgentVisibleProjection {
    pub views: BTreeMap<AgentId, NodeId>,
}

// ────────────────────────────────────────────────────────────────────────────
// BudgetSnapshot — global compute / cost / wall-clock budget.
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 1.1 — global budget snapshot:
/// cost ceiling (MicroCoin), wall clock remaining (ms), compute cap remaining.
/// Exhaustion → halt_reason ∈ {WallClockCap, ComputeCapViolated, MaxTxExhausted}.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BudgetSnapshot {
    pub cost_ceiling_microcoin: MicroCoin,
    pub wall_clock_remaining_ms: u64,
    pub compute_cap_remaining: u64,
}

impl Default for BudgetSnapshot {
    fn default() -> Self {
        Self {
            cost_ceiling_microcoin: MicroCoin::zero(),
            wall_clock_remaining_ms: 0,
            compute_cap_remaining: 0,
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// EconomicState — WP § 2 economic, 9 sub-fields. Atom CO1.2.2.
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX WP § 2 economic — 9-sub-field economic state. Each sub-index
/// is a BTreeMap newtype; entry shapes (Escrow / Stake / Claim / TaskMarket /
/// RoyaltyEdge / ChallengeCase) are minimal-but-typed here and fully fleshed
/// in the owning atoms (CO P2.1-2.6).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct EconomicState {
    pub balances_t: BalancesIndex,
    pub escrows_t: EscrowsIndex,
    pub stakes_t: StakesIndex,
    pub claims_t: ClaimsIndex,
    pub reputations_t: ReputationsIndex,
    pub task_markets_t: TaskMarketsIndex,
    pub royalty_graph_t: RoyaltyGraph,
    pub challenge_cases_t: ChallengeCasesIndex,
    pub price_index_t: PriceIndex,
}

/// TRACE_MATRIX WP § 2 — agent → balance ledger. Concrete entry: `MicroCoin` (CO1.0a).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct BalancesIndex(pub BTreeMap<AgentId, MicroCoin>);

/// TRACE_MATRIX WP § 2 — tx → escrow entry. Full schema lands CO P2.2 EscrowVault.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct EscrowsIndex(pub BTreeMap<TxId, EscrowEntry>);

/// TRACE_MATRIX WP § 2 — escrow entry shape (stub). Full fields land CO P2.2.
/// `#[serde(default)]` on each field gives forward-compat: future atoms can add
/// fields without breaking deserialization of historical ledger rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EscrowEntry {
    #[serde(default = "MicroCoin::zero")]
    pub amount: MicroCoin,
    #[serde(default)]
    pub depositor: AgentId,
}

impl Default for EscrowEntry {
    fn default() -> Self {
        Self { amount: MicroCoin::zero(), depositor: AgentId::default() }
    }
}

/// TRACE_MATRIX WP § 2 — tx → stake entry. Full schema lands CO P2.5 ChallengeCourt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct StakesIndex(pub BTreeMap<TxId, StakeEntry>);

/// TRACE_MATRIX WP § 2 — stake entry shape (stub). Full fields land CO P2.5.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StakeEntry {
    #[serde(default = "MicroCoin::zero")]
    pub amount: MicroCoin,
    #[serde(default)]
    pub staker: AgentId,
}

impl Default for StakeEntry {
    fn default() -> Self {
        Self { amount: MicroCoin::zero(), staker: AgentId::default() }
    }
}

/// TRACE_MATRIX WP § 2 — tx → reward claim. Full schema lands CO P2.6 SettlementEngine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ClaimsIndex(pub BTreeMap<TxId, ClaimEntry>);

/// TRACE_MATRIX WP § 2 — claim entry shape (stub). Full fields land CO P2.6.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimEntry {
    #[serde(default = "MicroCoin::zero")]
    pub amount: MicroCoin,
    #[serde(default)]
    pub claimant: AgentId,
}

impl Default for ClaimEntry {
    fn default() -> Self {
        Self { amount: MicroCoin::zero(), claimant: AgentId::default() }
    }
}

/// TRACE_MATRIX WP § 2 — agent → reputation ledger.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ReputationsIndex(pub BTreeMap<AgentId, Reputation>);

/// TRACE_MATRIX WP § 2 — tx → task market. Full schema lands CO P2.1.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TaskMarketsIndex(pub BTreeMap<TxId, TaskMarketEntry>);

/// TRACE_MATRIX WP § 2 — task market entry shape (stub). Full fields land CO P2.1.
/// Default values (verifier_quorum=1, max_reuse_royalty_fraction=0.10) match the
/// PROJECT_DECISION_MAP § 2.3 spec gap defaults.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskMarketEntry {
    #[serde(default)]
    pub publisher: AgentId,
    #[serde(default = "MicroCoin::zero")]
    pub bounty: MicroCoin,
    #[serde(default = "task_market_default_quorum")]
    pub verifier_quorum: u32,
    #[serde(default = "task_market_default_royalty_bp")]
    pub max_reuse_royalty_fraction_basis_points: u16,
}

fn task_market_default_quorum() -> u32 {
    1
}
fn task_market_default_royalty_bp() -> u16 {
    1000
}

impl Default for TaskMarketEntry {
    fn default() -> Self {
        Self {
            publisher: AgentId::default(),
            bounty: MicroCoin::zero(),
            verifier_quorum: 1,
            max_reuse_royalty_fraction_basis_points: 1000, // 0.10 per spec gap default
        }
    }
}

/// TRACE_MATRIX WP § 2 — directed royalty edges (reuse depth attribution).
/// Full attribution algebra lands CO P2.4.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RoyaltyGraph(pub BTreeMap<TxId, Vec<RoyaltyEdge>>);

/// TRACE_MATRIX WP § 2 — single royalty edge (ancestor → reuse weight). Stub; CO P2.4.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RoyaltyEdge {
    #[serde(default)]
    pub ancestor: TxId,
    #[serde(default)]
    pub fraction_basis_points: u16,
}

/// TRACE_MATRIX WP § 2 — tx → challenge case. Full schema lands CO P2.5.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ChallengeCasesIndex(pub BTreeMap<TxId, ChallengeCase>);

/// TRACE_MATRIX WP § 2 — challenge case shape (stub). Full fields land CO P2.5.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChallengeCase {
    #[serde(default)]
    pub challenger: AgentId,
    #[serde(default = "MicroCoin::zero")]
    pub bond: MicroCoin,
    #[serde(default)]
    pub opened_at_round: u64,
}

impl Default for ChallengeCase {
    fn default() -> Self {
        Self { challenger: AgentId::default(), bond: MicroCoin::zero(), opened_at_round: 0 }
    }
}

/// TRACE_MATRIX WP § 2 — tx → posted price (last accepted price index).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PriceIndex(pub BTreeMap<TxId, MicroCoin>);

// ────────────────────────────────────────────────────────────────────────────
// QState — § 1.1 verbatim, 9 fields.
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 1.1 — system state Q_t. 9 fields per WP § 4 + economic § 2 amendment.
///
/// Reconstructibility: every field is derivable from L4 transition ledger replay
/// (Art IV Boot 公理).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct QState {
    /// Agent swarm sub-state (tape head per agent + per-agent reputation snapshots).
    pub q_t: AgentSwarmState,
    /// Current ChainTape head pointer = git commit SHA in Path B substrate.
    pub head_t: NodeId,
    /// Materialized state Merkle root (git tree root in Path B).
    pub state_root_t: Hash,
    /// Agent-visible projection of tape filtered by per-agent visibility policy.
    pub tape_view_t: AgentVisibleProjection,
    /// L4 Transition Ledger root (Merkle root of all accepted tx so far).
    pub ledger_root_t: Hash,
    /// L1 Predicate Registry root.
    pub predicate_registry_root_t: Hash,
    /// L2 Tool Registry root.
    pub tool_registry_root_t: Hash,
    /// Economic state (WP § 2 amendment, 9 sub-fields).
    pub economic_state_t: EconomicState,
    /// Global budget snapshot.
    pub budget_state_t: BudgetSnapshot,
}

impl QState {
    /// TRACE_MATRIX Art IV Boot — genesis Q_t. All zero / empty;
    /// roots populated by `boot::verify_trust_root` and the `state_root_t` published
    /// in `genesis_payload.toml [constitution_root]`.
    pub fn genesis() -> Self {
        QState::default()
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Inline determinism tests (round-trip + insertion-order independence).
// Conformance tests proper live in tests/{four_element_mapping, q_state_reconstruct,
// economic_state_reconstruct, six_axioms_alignment}.rs per TRACE_MATRIX_v3.
// ────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genesis_q_state_is_total_and_default() {
        let g = QState::genesis();
        assert_eq!(g, QState::default());
        assert_eq!(g.q_t.current_round, 0);
        assert!(g.q_t.agents.is_empty());
        assert_eq!(g.head_t, NodeId::default());
        assert_eq!(g.state_root_t, Hash::ZERO);
    }

    #[test]
    fn nine_field_count_via_serde_json() {
        // Sanity that QState has exactly 9 top-level fields.
        let s = serde_json::to_value(QState::genesis()).unwrap();
        let obj = s.as_object().expect("object");
        assert_eq!(
            obj.len(),
            9,
            "QState must have exactly 9 fields per WP § 4; got {}",
            obj.len()
        );
        for k in &[
            "q_t",
            "head_t",
            "state_root_t",
            "tape_view_t",
            "ledger_root_t",
            "predicate_registry_root_t",
            "tool_registry_root_t",
            "economic_state_t",
            "budget_state_t",
        ] {
            assert!(obj.contains_key(*k), "QState missing field {}", k);
        }
    }

    #[test]
    fn economic_state_has_nine_sub_fields() {
        let e = EconomicState::default();
        let s = serde_json::to_value(&e).unwrap();
        let obj = s.as_object().unwrap();
        assert_eq!(
            obj.len(),
            9,
            "EconomicState must have 9 sub-fields per WP § 2; got {}",
            obj.len()
        );
    }

    #[test]
    fn btreemap_insertion_order_independent_serialization() {
        // Insertion-order independence (Inv determinism).
        let mut a = BalancesIndex::default();
        a.0.insert(AgentId("alice".into()), MicroCoin::from_coin(10).unwrap());
        a.0.insert(AgentId("bob".into()), MicroCoin::from_coin(20).unwrap());

        let mut b = BalancesIndex::default();
        b.0.insert(AgentId("bob".into()), MicroCoin::from_coin(20).unwrap());
        b.0.insert(AgentId("alice".into()), MicroCoin::from_coin(10).unwrap());

        let sa = serde_json::to_string(&a).unwrap();
        let sb = serde_json::to_string(&b).unwrap();
        assert_eq!(sa, sb, "BTreeMap must yield identical bytes regardless of insertion order");
    }

    #[test]
    fn node_id_from_state_root_is_deterministic() {
        let r = Hash::from_bytes([0xAB; 32]);
        let n1 = NodeId::from_state_root(r);
        let n2 = NodeId::from_state_root(r);
        assert_eq!(n1, n2);
        assert_eq!(n1.0.len(), 64, "40-byte git SHA hex form would be 40; we use full 32-byte sha256 hex = 64");
    }
}
