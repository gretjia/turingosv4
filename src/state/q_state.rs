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

use std::collections::{BTreeMap, BTreeSet};

use crate::bottom_white::cas::schema::Cid;

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

/// TRACE_MATRIX WP § 19 RSP-1 — task-market entry id; opaque string.
///
/// **TB-3 home migration (2026-04-30)**: previously defined at
/// `src/state/typed_tx.rs:33-35`. Per WP § 19 RSP-1 ("TaskMarket — 发布任务、
/// 广播价格、锁定奖金") + the TB-3 charter § 4.2 schema migration, `TaskId`
/// is now the canonical `TaskMarketsIndex` key — it belongs alongside
/// `AgentId` / `TxId` in the Q_t identifier layer, not in the typed-tx ABI
/// layer. The move closes a circular-dependency that would have arisen if
/// `q_state.rs` imported `TaskId` from `typed_tx.rs` (which already imports
/// `AgentId` / `TxId` from `q_state.rs`).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct TaskId(pub String);

/// TRACE_MATRIX § 1.1 — reputation snapshot. Signed i64 to permit negative reputation
/// (e.g. post-slash); ledger-of-record lives in `ReputationsIndex` (CO P2.9).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
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
///
/// TB-14 Atom 3 (FC2-N28; architect §5.5 + charter §3 Atom 3): `mask_set`
/// is the global per-round set of parent-attempt-node `TxId`s suppressed
/// in the agent read-view because a child node dominates them by
/// `BoltzmannMaskPolicy.price_margin` (FR-14.5 / FR-14.6). **Read-view
/// mask only**, never deletion (CR-14.3 + halt-trigger #3): the underlying
/// `Tape.nodes()` iteration always yields masked parents. Computed by
/// `compute_mask_set` in `src/state/price_index.rs`. `#[serde(default)]`
/// for backward-compat with pre-TB-14 chain snapshots (deserialize as
/// empty set).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AgentVisibleProjection {
    pub views: BTreeMap<AgentId, NodeId>,
    #[serde(default)]
    pub mask_set: BTreeSet<TxId>,
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
    // TB-14 Atom 2 (2026-05-03): `price_index_t: PriceIndex` removed.
    // The TB-14 derived view is `compute_price_index(econ)` in
    // `src/state/price_index.rs`; not canonical state per architect §5.1.
    /// TB-11 (architect §6.2 ruling 2026-05-02): runs_t — `RunId` → run-summary
    /// entry written by the TerminalSummaryTx dispatch arm. Anchors
    /// architect's RunExhaustedTx semantics on chain-resident state. Each
    /// failed evaluator run produces exactly one entry (idempotency on
    /// run_id). `#[serde(default)]` for backward-compat with pre-TB-11
    /// chain snapshots.
    #[serde(default)]
    pub runs_t: RunsIndex,
    /// TRACE_MATRIX TB-12 (architect 2026-05-03 ruling §3 + §10): node_positions_t
    /// — flat `BTreeMap<TxId, NodePosition>` index. **Canonical** TB-12 source
    /// of truth for exposure records. **NOT a Coin holding** (CR-12.1 + CR-12.2);
    /// NodePosition.amount is NOT counted in `monetary_invariant::total_supply_micro`.
    ///
    /// Architect §3 explicitly REJECTED the nested `node_market_t:
    /// BTreeMap<NodeId, NodeMarketEntry>` shape — that's TB-14 derived view
    /// (price + long_interest + short_interest aggregation), not canonical
    /// state. Avoiding second source-of-truth (architect §3.2 reasoning;
    /// TaskMarket.total_escrow precedent on cache=truth).
    ///
    /// Populated by accept-arm side-effect on accepted WorkTx (FirstLong) +
    /// ChallengeTx (ChallengeShort) per architect §8 Atom 2. VerifyTx writes
    /// nothing here per FR-12.3 + CR-12.8. `#[serde(default)]` for
    /// backward-compat with pre-TB-12 chain snapshots.
    #[serde(default)]
    pub node_positions_t: NodePositionsIndex,
    /// TRACE_MATRIX TB-13 Atom 2 (architect 2026-05-03 post-TB-12 ruling Part A
    /// §4.3 + §4.4 FR-13.1..7 + CR-13.4): conditional collateral per event.
    /// Locked Coin held against outstanding YES_E + NO_E share inventory.
    ///
    /// **IS** a Coin holding per CR-13.4 ("Locked collateral is Coin
    /// holding"); included in the 6-holding `total_supply_micro` sum
    /// (extends the TB-7R 5-holding sum). Mint/seed credit; redeem debit.
    ///
    /// **Complete-set balanced invariant** (Codex round-3 doc remediation
    /// 2026-05-03): the live invariant enforced by
    /// `monetary_invariant::assert_complete_set_balanced` is the
    /// **MIN form**: `min(Σ_yes_shares, Σ_no_shares) == collateral`.
    /// Pre-resolution (mint + seed only): both sides equal collateral
    /// (MIN trivially equals collateral). Post-redeem: the winning side
    /// equals collateral (debited 1:1 with collateral); the losing side
    /// may exceed collateral as stranded zero-value claims. Strict
    /// `Σ_yes == Σ_no == collateral` does NOT hold post-redemption.
    ///
    /// `#[serde(default)]` for backward-compat with pre-TB-13 chain snapshots.
    #[serde(default)]
    pub conditional_collateral_t: ConditionalCollateralIndex,
    /// TRACE_MATRIX TB-13 Atom 2 (architect §4.3 + CR-13.3 + SG-13.2):
    /// conditional share balances per `(owner, event_id, OutcomeSide)`.
    ///
    /// **IS NOT** a Coin holding — shares are CLAIMS against
    /// `conditional_collateral_t[event_id]`; CR-13.3 + SG-13.2 explicit:
    /// shares are NOT counted in `total_supply_micro`. Mint mints equal
    /// YES + NO; seed mints equal YES + NO to provider; redeem debits the
    /// winning side at 1 share = 1 MicroCoin against collateral.
    ///
    /// `#[serde(default)]` for backward-compat with pre-TB-13 chain snapshots.
    #[serde(default)]
    pub conditional_share_balances_t: ConditionalShareBalances,
    /// TRACE_MATRIX TB-15 Atom 3 (architect §6.2 ruling 2026-05-02 + §6.5
    /// SG-15.1 + SG-15.2): per-event autopsy index.
    /// `BTreeMap<EventId, Vec<Cid>>` — for each event with at least one
    /// loss-emission, accumulates the CAS Cids of `AgentAutopsyCapsule`
    /// objects (one per losing agent). **Stores Cids only**, NEVER the
    /// raw capsule bytes — the bytes live in CAS behind
    /// `ObjectType::AgentAutopsyCapsule` (and the audit-only
    /// `private_detail_cid` lives behind `ObjectType::AutopsyPrivateDetail`).
    ///
    /// **NOT projected to `AgentVisibleProjection`** (CR-15.1 + halt-
    /// trigger #1). Sequencer-side index only; surfaces via
    /// dashboard §15 (Atom 6) + ChainTape replay regeneration. Other
    /// Agents cannot retrieve the bytes through their `tape_view_t`
    /// (SG-15.2 + halt-trigger #4).
    ///
    /// `#[serde(default)]` for backward-compat with pre-TB-15 chain snapshots.
    #[serde(default)]
    pub agent_autopsies_t: AutopsyIndex,
    /// TRACE_MATRIX FC1-Append Stage C P-M4 / Phase F.3 (architect manual
    /// §7.5 + remediation directive §1.C row 3): per-event LiquidityPool
    /// (CpmmPool) index. One pool per `EventId`; pool reserves in
    /// `CpmmPool.pool_yes / pool_no` are NOT Coin (architect §7.5 rule 2)
    /// → EXCLUDED from `total_supply_micro`. Created by P-M4
    /// `CpmmPoolTx`; mutated by future P-M5 CpmmSwapTx + P-M6 router.
    /// `#[serde(default)]` for backward-compat with pre-P-M4 snapshots.
    #[serde(default)]
    pub cpmm_pools_t: CpmmPoolsIndex,
    /// TRACE_MATRIX FC1-Append Stage C P-M4 / Phase F.3 (architect manual
    /// §7.5 rule 3 "lp shares are not Coin"): per-`(agent, event_id)` LP
    /// share balance ledger. Provider receives LP shares 1:1 with the
    /// symmetric `seed_yes` units they contributed at pool creation.
    /// LP shares are NOT Coin → EXCLUDED from `total_supply_micro`.
    /// `#[serde(default)]` for backward-compat with pre-P-M4 snapshots.
    #[serde(default)]
    pub lp_share_balances_t: LpShareBalancesIndex,
    /// TRACE_MATRIX TB-N1-AGENT-ECONOMY Phase 2 A4 (charter §2; 2026-05-10):
    /// `(verifier_agent, target_work_tx)` set tracking accepted agent-
    /// submitted VerifyTxs. Used by sequencer step-3.5 to reject duplicate
    /// VerifyTxs from the same agent on the same target (closes the
    /// duplicate-verification griefing surface where an agent could spam
    /// multiple Confirm/Deny VerifyTxs on the same target_work_tx).
    ///
    /// **NOT a Coin holding** — pure set-tracking index; EXCLUDED from
    /// `total_supply_micro`. Pure-additive: pre-A4 chains had no
    /// agent_verifications_t and admitted potentially duplicate Verifies
    /// (handled at claims_t level via line ~1053 already_claimed
    /// suppression); A4 promotes this to a fail-closed admission gate
    /// for telemetry symmetry with A3's StakeBalanceExceeded.
    /// `#[serde(default)]` for backward-compat with pre-A4 chain snapshots.
    #[serde(default)]
    pub agent_verifications_t: AgentVerificationsIndex,
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
///
/// **TB-3 additive field**: `task_id` is the back-reference to the `TaskId`
/// this escrow funds. Required by `assert_task_market_total_escrow_matches_locks`
/// (the cache=truth invariant for `TaskMarketEntry.total_escrow`). Additive
/// serde-default — pre-TB-3 serialized rows deserialize with the empty TaskId.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EscrowEntry {
    #[serde(default = "MicroCoin::zero")]
    pub amount: MicroCoin,
    #[serde(default)]
    pub depositor: AgentId,
    #[serde(default)]
    pub task_id: TaskId,
}

impl Default for EscrowEntry {
    fn default() -> Self {
        Self {
            amount: MicroCoin::zero(),
            depositor: AgentId::default(),
            task_id: TaskId::default(),
        }
    }
}

/// TRACE_MATRIX WP § 2 — tx → stake entry. Full schema lands CO P2.5 ChallengeCourt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct StakesIndex(pub BTreeMap<TxId, StakeEntry>);

/// TRACE_MATRIX WP § 2 — stake entry shape (stub). Full fields land CO P2.5.
///
/// **TB-3 additive field**: `task_id` records the task this stake commits
/// to. Required by the WorkTx admission gate (TB-3 § 3.4 lock-on-accept):
/// when an accepted WorkTx commits its inline `stake` into `stakes_t`, the
/// entry carries the task binding so future RSP-2/3 challenge resolution
/// can route the slash/release. Additive serde-default — pre-TB-3
/// serialized rows deserialize with the empty TaskId.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StakeEntry {
    #[serde(default = "MicroCoin::zero")]
    pub amount: MicroCoin,
    #[serde(default)]
    pub staker: AgentId,
    #[serde(default)]
    pub task_id: TaskId,
}

impl Default for StakeEntry {
    fn default() -> Self {
        Self {
            amount: MicroCoin::zero(),
            staker: AgentId::default(),
            task_id: TaskId::default(),
        }
    }
}

/// TRACE_MATRIX WP § 2 — tx → reward claim. Full schema lands CO P2.6 SettlementEngine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ClaimsIndex(pub BTreeMap<TxId, ClaimEntry>);

/// TRACE_MATRIX WP § 2 — claim entry shape. Extended in TB-8 Atom 1
/// (2026-05-02) per `handover/audits/CHARTER_RATIFICATION_TB_8_2026-05-02.md`
/// §1 Q1 ratification: 6 new fields drive the Atom-3 FinalizeReward dispatch
/// arm without re-traversing stakes_t / escrows_t / L4. All additive; every
/// field carries `#[serde(default)]` so historical rows (TB-3..TB-7R never
/// wrote a ClaimEntry — claims_t was a never-written stub) deserialize
/// cleanly when re-read post-TB-8.
///
/// `status: ClaimStatus` is the terminal-state discriminator: `Open` at
/// claim-creation (Atom-1 writer at VerifyTx OMEGA-Confirm), `Finalized` after
/// the dispatch arm atomically credits the solver. `Slashed` is reserved for
/// post-TB-15 slash-execution territory (per directive 2026-05-02 ruling 13).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimEntry {
    #[serde(default = "MicroCoin::zero")]
    pub amount: MicroCoin,
    #[serde(default)]
    pub claimant: AgentId,
    /// TB-8 Atom 1: which task's escrow funds this claim.
    #[serde(default)]
    pub task_id: TaskId,
    /// TB-8 Atom 1: which `escrows_t` row to debit at finalize time.
    #[serde(default)]
    pub escrow_lock_tx_id: TxId,
    /// TB-8 Atom 1: the accepted WorkTx whose OMEGA-Confirm produced this claim.
    #[serde(default)]
    pub work_tx_id: TxId,
    /// TB-8 Atom 1: the OMEGA-Confirm VerifyTx that triggered claim creation.
    #[serde(default)]
    pub verify_tx_id: TxId,
    /// TB-8 Atom 1: terminal-state discriminator (Open at claim-creation,
    /// Finalized after the Atom-3 dispatch arm credits the solver).
    #[serde(default)]
    pub status: ClaimStatus,
    /// TB-8 Atom 1: logical_t at which finalize becomes legal. TB-8 MVP
    /// uses literal value 0 as the "window-closed-immediately" structural
    /// marker per ratification §1 Q3 (corrected §2.4): the dispatch-arm
    /// gate (`src/state/sequencer.rs::TypedTx::FinalizeReward`) fires only
    /// when this field is > 0 AND `fr.timestamp_logical <=` this field —
    /// at zero, the gate is trivially satisfied. agent-controlled
    /// `verify.timestamp_logical` is intentionally NOT used as the source
    /// (different namespace from sequencer-controlled `fr.timestamp_logical`).
    /// A future TB introducing a real challenge window will set this to a
    /// non-zero value in the sequencer namespace at claim-creation time.
    #[serde(default)]
    pub challenge_window_close_logical_t: u64,
}

impl Default for ClaimEntry {
    fn default() -> Self {
        Self {
            amount: MicroCoin::zero(),
            claimant: AgentId::default(),
            task_id: TaskId::default(),
            escrow_lock_tx_id: TxId::default(),
            work_tx_id: TxId::default(),
            verify_tx_id: TxId::default(),
            status: ClaimStatus::default(),
            challenge_window_close_logical_t: 0,
        }
    }
}

/// TRACE_MATRIX TB-8 charter §3 Atom 1 + Atom 0.5 ratification §1 Q1 —
/// claim terminal-state discriminator.
///
/// `Open` → `Finalized` is the cooperative settlement path (Atom-3
/// FinalizeReward dispatch arm). `Slashed` is reserved for the adversarial
/// path (RSP-3.2 slash, deferred to post-TB-15 territory per directive
/// 2026-05-02 ruling 13). Idempotency at the Atom-3 dispatch arm reads this
/// field — re-finalize on a `Finalized` claim → `ClaimAlreadyFinalized`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum ClaimStatus {
    Open = 0,
    Finalized = 1,
    /// Reserved for RSP-3.2 slash-execution (post-TB-15 per directive 2026-05-02).
    Slashed = 2,
}

impl Default for ClaimStatus {
    fn default() -> Self {
        Self::Open
    }
}

/// TRACE_MATRIX WP § 2 — agent → reputation ledger.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ReputationsIndex(pub BTreeMap<AgentId, Reputation>);

/// TRACE_MATRIX WP § 19 RSP-1 — task → task market. Full schema lands CO P2.1.
///
/// **TB-3 key migration (2026-04-30)**: keyed by `TaskId` (was `TxId`). Per
/// WP § 19 RSP-1 + TB-3 charter § 4.2: TaskMarket is a per-task concept;
/// keying by TaskId reflects the constitutional shape. The TB-2 P0-B option (a)
/// bridge `TxId(task_id.0.clone())` is removed in TB-3 Atom 6.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TaskMarketsIndex(pub BTreeMap<TaskId, TaskMarketEntry>);

/// TRACE_MATRIX WP § 19 RSP-1 — task market entry. Full fields land CO P2.1.
///
/// **TB-3 field migration (2026-04-30)**:
/// - REMOVED `bounty: MicroCoin` — money has migrated to `escrows_t.amount`
///   (each accepted `EscrowLockTx` is a separate `escrows_t` row keyed by
///   the lock tx's TxId; the TaskMarketEntry no longer holds money directly).
/// - ADDED `total_escrow: MicroCoin` — **derived aggregate / cached index,
///   NOT a money holding**. Equals `Σ escrows_t[e].amount where e.task_id ==
///   <this task>`. `monetary_invariant::total_supply_micro` does NOT include
///   this term (else it would double-count every locked bounty). The
///   cache=truth invariant is enforced by `assert_task_market_total_escrow_matches_locks`.
/// - ADDED `escrow_lock_tx_ids: BTreeSet<TxId>` — replay-deterministic
///   provenance: which `EscrowLockTx`s contributed to this task's funding.
/// - ADDED `settlement_rule_hash: Hash` — RSP-3/4 hook (opaque hash for
///   TB-3; full settlement-rule engine lands later).
///
/// Default values (verifier_quorum=1, max_reuse_royalty_fraction=0.10) match
/// the PROJECT_DECISION_MAP § 2.3 spec gap defaults.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskMarketEntry {
    #[serde(default)]
    pub publisher: AgentId,
    /// Derived aggregate; NOT a holding. See doc-comment above.
    #[serde(default = "MicroCoin::zero")]
    pub total_escrow: MicroCoin,
    /// Replay-deterministic provenance for `total_escrow`.
    #[serde(default)]
    pub escrow_lock_tx_ids: BTreeSet<TxId>,
    #[serde(default = "task_market_default_quorum")]
    pub verifier_quorum: u32,
    #[serde(default = "task_market_default_royalty_bp")]
    pub max_reuse_royalty_fraction_basis_points: u16,
    /// RSP-3/4 hook; opaque hash for TB-3.
    #[serde(default)]
    pub settlement_rule_hash: Hash,
    /// TB-11 (architect §6.2): task lifecycle state. Default `Open`
    /// (backward-compat: pre-TB-11 task_markets_t entries deserialize as
    /// Open). Mutated by sequencer dispatch arms:
    ///   - Open → Bankrupt   on TaskBankruptcyTx
    ///   - Open → Expired    on TaskExpireTx (post-deadline refund)
    ///   - any → Finalized   on FinalizeRewardTx (terminal, immutable)
    /// `#[serde(default)]` for forward-compat.
    #[serde(default)]
    pub state: TaskMarketState,
    /// TB-11: logical_t at which TaskBankruptcyTx fired, if any. `None`
    /// while task is Open / Expired / Finalized; `Some(t)` post-bankruptcy.
    /// Used by the bankruptcy idempotency gate in dispatch_transition.
    #[serde(default)]
    pub bankruptcy_at_logical_t: Option<u64>,
    /// TB-11: TaskOpen.timestamp_logical, captured at dispatch time, used
    /// by `tb11_emit_expire_for_eligible` to compute deadline policy
    /// (current_logical_t - opened_at_logical_t > TASK_EXPIRY_LOGICAL_T_DELTA).
    /// Backward-compat: pre-TB-11 entries deserialize at 0; the deadline
    /// check then fires immediately for legacy tasks (intended — legacy
    /// tasks SHOULD be expirable to release any locked bounties).
    #[serde(default)]
    pub opened_at_logical_t: u64,
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
            total_escrow: MicroCoin::zero(),
            escrow_lock_tx_ids: BTreeSet::new(),
            verifier_quorum: 1,
            max_reuse_royalty_fraction_basis_points: 1000, // 0.10 per spec gap default
            settlement_rule_hash: Hash::ZERO,
            state: TaskMarketState::Open,  // TB-11
            bankruptcy_at_logical_t: None, // TB-11
            opened_at_logical_t: 0,        // TB-11
        }
    }
}

/// TRACE_MATRIX TB-11 (2026-05-02 architect ruling §6.2) — task lifecycle
/// discriminator. `Open` is the default initial state set by the TaskOpenTx
/// dispatch arm; transitions are uni-directional under the TB-11 dispatch
/// rules:
///   - `Open` → `Expired` on accepted `TaskExpireTx` (post-deadline refund)
///   - `Open` → `Bankrupt` on accepted `TaskBankruptcyTx` (architect §6.2 death cert)
///   - any non-Finalized → `Finalized` on accepted `FinalizeRewardTx`
///     (terminal; immutable)
///
/// `Bankrupt` and `Expired` are NOT terminal — a Bankrupt task may still
/// be Expired afterward (via `BankruptcyTriggered` reason on the Expire),
/// to free any escrow that was not already refunded. Finalized is terminal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum TaskMarketState {
    Open = 0,
    Expired = 1,
    Bankrupt = 2,
    Finalized = 3,
}

impl Default for TaskMarketState {
    fn default() -> Self {
        Self::Open
    }
}

// ────────────────────────────────────────────────────────────────────────────
// TB-11 (architect §6.2): RunsIndex — chain-resident anchor for
// architect's RunExhaustedTx role.
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX TB-11 — `RunId` → `RunSummaryEntry`. Written by the
/// `TerminalSummaryTx` dispatch arm; anchors architect §6.2 RunExhaustedTx
/// semantics in chain-resident state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RunsIndex(pub BTreeMap<crate::state::typed_tx::RunId, RunSummaryEntry>);

// ────────────────────────────────────────────────────────────────────────────
// TB-12 (architect 2026-05-03 ruling §3 + §8 Atom 1): NodePositionsIndex —
// flat exposure record index. ARCHITECT-RULING DISCIPLINE: this is the
// canonical TB-12 state. No `node_market_t / NodeMarketEntry` is added —
// that's TB-14 derived view. Avoids second source-of-truth risk.
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX TB-12 (architect 2026-05-03 §3 + §8 Atom 1): flat
/// `position_id → NodePosition` index. Architect's §3 ruling chose this
/// over nested `node_market_t: BTreeMap<NodeId, NodeMarketEntry>` to keep a
/// single source of truth in TB-12. Populated by accept-arm side-effect
/// (Atom 2) on accepted WorkTx (FirstLong) + ChallengeTx (ChallengeShort).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct NodePositionsIndex(pub BTreeMap<TxId, crate::state::typed_tx::NodePosition>);

// ────────────────────────────────────────────────────────────────────────────
// TB-13 (architect 2026-05-03 post-TB-12 ruling Part A §4.3 + §4.4):
// ConditionalCollateralIndex + ConditionalShareBalances — Polymarket / CTF
// conditional-share substrate. **Mint identity: 1 locked Coin = 1 YES_E + 1
// NO_E.** The live invariant after redemption uses MIN form (see
// monetary_invariant::assert_complete_set_balanced); strict YES==NO==coll
// only holds pre-resolution / pre-redemption.
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX TB-13 Atom 2 (architect §4.3 + CR-13.4): per-event Coin
/// collateral locked against outstanding YES_E + NO_E share inventory.
///
/// **IS** a Coin holding — included in 6-holding `total_supply_micro` sum
/// at `monetary_invariant::assert_total_ctf_conserved`. Mint/seed credit
/// this map; redeem debits it. The complete-set balanced invariant
/// (`assert_complete_set_balanced`) enforces the MIN form:
/// `min(Σ_yes_shares, Σ_no_shares) == collateral`. Pre-resolution this is
/// equivalent to strict `Σ_yes == Σ_no == collateral`; post-redemption
/// the winning side equals collateral while the losing side may strand
/// above collateral. Codex round-3 doc remediation 2026-05-03.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ConditionalCollateralIndex(pub BTreeMap<crate::state::typed_tx::EventId, MicroCoin>);

/// TRACE_MATRIX TB-13 Atom 2 (architect §4.3 + CR-13.3 + SG-13.2): per-
/// `(owner, event_id)` share balance pair (YES + NO sides).
///
/// **IS NOT** a Coin holding — shares are claims against
/// `ConditionalCollateralIndex[event_id]`. Architect CR-13.3 / SG-13.2
/// explicit: shares are NOT counted in `total_supply_micro`.
///
/// Wire shape: `BTreeMap<AgentId, BTreeMap<EventId, ShareSidePair>>`.
/// Nested-map shape (rather than tuple-key) keeps the structure
/// JSON-friendly (BTreeMap with tuple keys is not serializable through
/// serde_json) while preserving canonical Owner-major / Event-minor
/// ordering for replay determinism.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ConditionalShareBalances(
    pub BTreeMap<AgentId, BTreeMap<crate::state::typed_tx::EventId, ShareSidePair>>,
);

/// TRACE_MATRIX TB-13 Atom 2 (architect §4.3 + FR-13.3): YES + NO share
/// holdings for a `(owner, event_id)` pair. Mint and seed credit
/// equally; redeem debits the winning side.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ShareSidePair {
    pub yes: crate::state::typed_tx::ShareAmount,
    pub no: crate::state::typed_tx::ShareAmount,
}

// ────────────────────────────────────────────────────────────────────────────
// Stage C P-M4 / Phase F.3 — CpmmPool LiquidityPool state (architect §7.5).
// Per remediation directive 2026-05-09 §1.C row 3 (Class-4 STEP_B rebuild
// post-VETO; defect 4 prevention `event_id` NOT `event_id_kind`).
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC1-Append Stage C P-M4 / Phase F.3 (architect manual §7.5
/// rule 3 verbatim "lp shares are not Coin"): LP share count newtype.
///
/// Non-negative `u128` units. LP shares track ownership of `CpmmPool`
/// reserves; they are explicitly NOT a Coin holding (architect §7.5 rule 3).
/// `LpShareBalancesIndex` is therefore EXCLUDED from the
/// `total_supply_micro` 6-holding sum that `assert_no_post_init_mint`
/// guards (per Phase E.3 strict-equality lint).
///
/// Pattern mirror: `ShareAmount` at `src/state/typed_tx.rs:1140` (TB-13).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub struct LpShareAmount {
    pub units: u128,
}

impl LpShareAmount {
    /// TRACE_MATRIX FC1-Append Stage C P-M4 / Phase F.3 (architect manual
    /// §7.5 rule 3 "lp shares are not Coin"): zero LP share amount —
    /// default constructor for empty balance lookups.
    pub const fn zero() -> Self {
        Self { units: 0 }
    }
    /// TRACE_MATRIX FC1-Append Stage C P-M4 / Phase F.3 (architect manual
    /// §7.5 rule 3): build from raw `u128` units. Used by sequencer
    /// pool-creation arm to project `seed_yes.units` (symmetric init
    /// formula `lp_total_shares = seed_yes.units`) into the LP-share
    /// domain at `cpmm_pool_accept_state_root` step 6c.
    pub const fn from_units(units: u128) -> Self {
        Self { units }
    }
}

/// TRACE_MATRIX FC1-Append Stage C P-M4 / Phase F.3 (architect manual §7.5):
/// pool lifecycle status discriminator.
///
/// `Active` — pool open for swaps (P-M5 CPMM swap arm activates against this).
/// `Resolved` — post-event-resolution; reserves frozen (LP-only redemption
///   path lands at P-M9 controlled smoke / future TB).
/// `Closed` — pool drained + retired.
///
/// Default `Active` because pool-creation tx instantiates a usable pool.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum PoolStatus {
    Active = 0,
    Resolved = 1,
    Closed = 2,
}

impl Default for PoolStatus {
    fn default() -> Self {
        Self::Active
    }
}

/// TRACE_MATRIX FC1-Append Stage C P-M4 / Phase F.3 (architect manual §7.5
/// VERBATIM 5-field spec — `event_id` NOT `event_id_kind` per session #27
/// defect 4 lesson; field ORDER + TYPES bound by `tests/constitution_
/// architect_verbatim_struct_binding.rs` E.1 gate).
///
/// State semantics (architect §7.5 rules):
/// - `pool_yes` / `pool_no` are share balances controlled by the pool
///   (NOT held in `conditional_share_balances_t`; pool reserves live
///   inside this struct as `pool.pool_yes / pool.pool_no`).
/// - Pool reserves are NOT Coin (rule 2) → `total_supply_micro`
///   sum EXCLUDES `cpmm_pools_t.values().*.pool_yes/no.units`.
/// - `lp_total_shares` is the total LP-token supply outstanding for this
///   pool; per-agent ownership lives in `lp_share_balances_t`.
/// - `k = pool_yes * pool_no` is the constant-product invariant maintained
///   by future P-M5 swap arms (this struct only declares state shape).
///
/// One pool per `EventId` (sequencer admission rejects double-create with
/// `PoolAlreadyExists`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CpmmPool {
    pub event_id: crate::state::typed_tx::EventId,
    pub pool_yes: crate::state::typed_tx::ShareAmount,
    pub pool_no: crate::state::typed_tx::ShareAmount,
    pub lp_total_shares: LpShareAmount,
    pub status: PoolStatus,
}

/// TRACE_MATRIX FC1-Append Stage C P-M4 / Phase F.3 (architect manual §7.5):
/// per-event CpmmPool index. Keyed by `EventId` — one pool per event in v4
/// (multi-pool-per-event deferred to future TB).
///
/// Pool reserves contained in `CpmmPool.pool_yes / pool.pool_no` are NOT
/// counted in `total_supply_micro`: shares are claims, never Coin
/// (architect §7.5 rule 2 + CR-13.3 / SG-13.2 carry-forward).
/// `#[serde(default)]` for backward-compat with pre-P-M4 chain snapshots.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CpmmPoolsIndex(pub BTreeMap<crate::state::typed_tx::EventId, CpmmPool>);

/// TRACE_MATRIX FC1-Append Stage C P-M4 / Phase F.3 (architect manual §7.5
/// rule 3 "lp shares are not Coin"): per-`(agent, event_id)` LP token
/// balance index.
///
/// Wire shape: `BTreeMap<(AgentId, EventId), LpShareAmount>`. Tuple-key
/// shape (vs nested map) is acceptable here because LP balances are not
/// projected to JSON in the agent-view (Class-4 internal state only); the
/// audit-side `serde_json` projection passes through `BTreeMap` ordering
/// for deterministic replay.
///
/// EXCLUDED from `total_supply_micro` per architect §7.5 rule 3 + Phase E.3
/// strict-equality lint extension.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct LpShareBalancesIndex(
    pub BTreeMap<(AgentId, crate::state::typed_tx::EventId), LpShareAmount>,
);

/// TRACE_MATRIX TB-N1-AGENT-ECONOMY Phase 2 A4 (charter §2; 2026-05-10):
/// `(verifier_agent, target_work_tx)` set tracking accepted agent-submitted
/// VerifyTxs. Closes the duplicate-verification griefing surface — sequencer
/// step-3.5 admission gate rejects with `VerifyDuplicate` if the pair is
/// present.
///
/// Wire shape: `BTreeSet<(AgentId, TxId)>`. Pure-additive set; pre-A4 chain
/// snapshots deserialize with empty set via `#[serde(default)]` on the
/// `EconomicState.agent_verifications_t` field.
///
/// **NOT a Coin holding** — pure set-tracking index; EXCLUDED from
/// `total_supply_micro`. Pre-A4 chains relied on the `claims_t::already_claimed`
/// check (line ~1053) for cross-agent claim suppression; A4 promotes the
/// per-(verifier, target) duplicate check to a fail-closed admission gate
/// for telemetry symmetry with A3's `StakeBalanceExceeded`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AgentVerificationsIndex(pub std::collections::BTreeSet<(AgentId, TxId)>);

/// TRACE_MATRIX TB-11 (architect §6.2) — per-run summary. Sponsored by
/// `task_id`; populated by the `TerminalSummaryTx` dispatch arm with
/// fields drawn from the typed-tx wire payload (Q-derivable on replay).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RunSummaryEntry {
    /// Backref to the task this run was working on.
    pub task_id: TaskId,
    /// Architect §6.2 — terminal outcome. `OmegaAccepted` for happy-path
    /// completion (would also produce a FinalizeReward elsewhere); the 4
    /// failure variants for the architect's "RunExhausted" cases.
    pub run_outcome: crate::state::typed_tx::RunOutcome,
    /// Architect §6.2 — total LLM proposals + Lean attempts in the run.
    pub attempt_count: u64,
    /// Architect §6.2 — CAS reference to the rolled-up evidence bytes.
    /// `None` for OmegaAccepted (success path needs no failure capsule);
    /// `Some` for failure outcomes.
    pub evidence_capsule_cid: Option<Cid>,
    /// Which agent owned the run, if any.
    pub solver_agent: Option<AgentId>,
    /// Logical_t when the TerminalSummaryTx was emitted.
    pub last_logical_t: u64,
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
///
/// **TB-4 additive field**: `target_work_tx` is the back-reference to the
/// `WorkTx.tx_id` this challenge accuses. Required by:
/// (a) RSP-3 settlement (routing slash/release on challenge resolve must
///     find the target's stakes_t entry via this backref);
/// (b) Multi-challenger representability (TB-4 charter § 3.5 + directive Q4):
///     two challenge_cases_t rows with distinct ChallengeTx tx_id keys
///     may share the same `target_work_tx` — without the backref the
///     index can't express that.
/// Additive serde-default — pre-TB-4 has no production challenge_cases_t
/// rows (dispatch arm was NotYetImplemented), so the migration is forward-only.
///
/// **TB-5 additive field**: `status: ChallengeStatus` records resolution
/// outcome without removing the entry from challenge_cases_t. Default = Open.
/// Released zeros bond + flips status to Released (audit trail preserved per
/// charter v2 § 7 Q6 — preserves slash-target reference for TB-6).
/// UpheldDeferred preserves bond + flips status (TB-6 slash routing target).
/// Additive serde-default — pre-TB-5 serialized rows deserialize with
/// status = Open.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChallengeCase {
    #[serde(default)]
    pub challenger: AgentId,
    #[serde(default = "MicroCoin::zero")]
    pub bond: MicroCoin,
    #[serde(default)]
    pub opened_at_round: u64,
    #[serde(default)]
    pub target_work_tx: TxId,
    #[serde(default)]
    pub status: ChallengeStatus, // ← TB-5 NEW
}

/// TRACE_MATRIX TB-5 charter v2 § 4.4 — challenge resolution status.
///
/// **Single source of truth** per Codex round-2 + round-3 Q4 ruling: defined
/// HERE (not in typed_tx.rs); sequencer.rs imports via
/// `use crate::state::q_state::ChallengeStatus;`. The on-wire resolution
/// outcome enum (`ChallengeResolution { Released | UpheldDeferred }`) lives
/// in typed_tx.rs alongside ChallengeResolveTx — that carries the system-
/// emitted resolution outcome on L4. ChallengeStatus is the Q-side case-state
/// tracker that flips on dispatch.
///
/// State machine:
///   Open → Released         (via accepted ChallengeResolveTx{Released})
///   Open → UpheldDeferred   (via accepted ChallengeResolveTx{UpheldDeferred})
///   Released / UpheldDeferred → terminal (AlreadyResolved gate at dispatch)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum ChallengeStatus {
    Open = 0,
    Released = 1,
    UpheldDeferred = 2,
}

impl Default for ChallengeStatus {
    fn default() -> Self {
        Self::Open
    }
}

impl Default for ChallengeCase {
    fn default() -> Self {
        Self {
            challenger: AgentId::default(),
            bond: MicroCoin::zero(),
            opened_at_round: 0,
            target_work_tx: TxId::default(),
            status: ChallengeStatus::Open,
        }
    }
}

// TB-14 Atom 2 (2026-05-03): legacy `pub struct PriceIndex(BTreeMap<TxId,
// MicroCoin>)` removed. The TB-14 derived view is `compute_price_index`
// in `src/state/price_index.rs` (architect §5.1: "price is signal, not
// truth"; charter §7 auto-resolution A: "no second source-of-truth").
// `EconomicState.price_index_t` field also removed at architect §5.2.

/// TRACE_MATRIX TB-15 Atom 3 (architect §6.2 + DECISION_LAMARCKIAN §1.1):
/// per-event autopsy index. `BTreeMap<EventId, Vec<Cid>>` — one Cid per
/// `AgentAutopsyCapsule` emitted on a loss event (TB-15 v0 sole trigger
/// = TaskBankruptcyTx; per-staker capsule emission). **Cids only** —
/// the capsule bytes live in CAS behind `ObjectType::AgentAutopsyCapsule`.
///
/// Sequencer-side index ONLY. NOT projected to `AgentVisibleProjection`
/// (CR-15.1 + halt-trigger #1). Other agents cannot retrieve the bytes
/// through their `tape_view_t` (SG-15.2 + halt-trigger #4).
///
/// BTreeMap iteration order is sorted-by-`EventId` → deterministic →
/// replay-safe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AutopsyIndex(
    pub BTreeMap<crate::state::typed_tx::EventId, Vec<crate::bottom_white::cas::schema::Cid>>,
);

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
    fn economic_state_has_sixteen_sub_fields() {
        // TB-11 (2026-05-02 architect ruling §6.2): 9 → 10 sub-fields with +runs_t.
        // TB-12 (2026-05-03 architect ruling §3 + §8 Atom 1): 10 → 11 sub-fields
        // with +node_positions_t (flat NodePositionsIndex; canonical exposure
        // record state; NOT NodeMarketEntry which is TB-14 derived view).
        // TB-13 Atom 2 (architect 2026-05-03 post-TB-12 ruling Part A §4.3):
        // 11 → 13 sub-fields with +conditional_collateral_t (CR-13.4 Coin
        // holding, included in 6-holding total_supply_micro) +
        // conditional_share_balances_t (CR-13.3 claims, NOT counted in
        // total_supply_micro).
        // TB-14 Atom 2 (2026-05-03 architect ruling §5.1): 13 → 12 sub-fields
        // with -price_index_t (legacy stub removed; TB-14 provides
        // `compute_price_index` pure-fn derived view, not canonical state —
        // "price is signal, not truth"; charter §7 auto-resolution A: no
        // second source-of-truth).
        // TB-15 Atom 3 (2026-05-03 architect ruling §6.2): 12 → 13 sub-fields
        // with +agent_autopsies_t (`AutopsyIndex` = `BTreeMap<EventId, Vec<Cid>>`;
        // sequencer-side per-event Cid index; capsule bytes live in CAS;
        // NOT projected to AgentVisibleProjection per CR-15.1 + halt-trigger #1).
        // Stage C P-M4 / Phase F.3 (architect manual §7.5 + remediation
        // directive 2026-05-09 §1.C row 3): 13 → 15 sub-fields with
        // +cpmm_pools_t (one CpmmPool per EventId; pool reserves NOT Coin
        // per architect §7.5 rule 2) and +lp_share_balances_t (LP token
        // balances; NOT Coin per architect §7.5 rule 3).
        // TB-N1-AGENT-ECONOMY Phase 2 A4 (2026-05-10; charter §2 atom A4):
        // 15 → 16 sub-fields with +agent_verifications_t
        // (AgentVerificationsIndex = BTreeSet<(AgentId, TxId)> for
        // sequencer step-3.5 duplicate-suppression; NOT a Coin holding —
        // pure set; EXCLUDED from total_supply_micro).
        let e = EconomicState::default();
        let s = serde_json::to_value(&e).unwrap();
        let obj = s.as_object().unwrap();
        assert_eq!(
            obj.len(),
            16,
            "EconomicState must have 16 sub-fields post-TB-N1-A4 (was 15 post-Stage-C-P-M4; +agent_verifications_t); got {}",
            obj.len()
        );
        assert!(obj.contains_key("runs_t"), "TB-11 runs_t sub-field missing");
        assert!(
            obj.contains_key("node_positions_t"),
            "TB-12 node_positions_t sub-field missing"
        );
        assert!(
            obj.contains_key("conditional_collateral_t"),
            "TB-13 conditional_collateral_t sub-field missing"
        );
        assert!(
            obj.contains_key("conditional_share_balances_t"),
            "TB-13 conditional_share_balances_t sub-field missing"
        );
        assert!(
            obj.contains_key("agent_autopsies_t"),
            "TB-15 agent_autopsies_t sub-field missing"
        );
        assert!(
            obj.contains_key("cpmm_pools_t"),
            "Stage C P-M4 cpmm_pools_t sub-field missing"
        );
        assert!(
            obj.contains_key("lp_share_balances_t"),
            "Stage C P-M4 lp_share_balances_t sub-field missing"
        );
        assert!(
            obj.contains_key("agent_verifications_t"),
            "TB-N1 A4 agent_verifications_t sub-field missing"
        );
        assert!(
            !obj.contains_key("price_index_t"),
            "TB-14 Atom 2: price_index_t MUST be removed"
        );
    }

    /// TB-12 Atom 1 (architect §8 Atom 1): NodePositionsIndex empty default
    /// serializes to empty BTreeMap; carries no balance information.
    #[test]
    fn node_positions_index_default_is_empty() {
        let idx = NodePositionsIndex::default();
        assert!(idx.0.is_empty());
    }

    /// TB-12 Atom 1: explicit invariant SG-12.8 — no `node_market_t` field
    /// on EconomicState. NodeMarketEntry is TB-14 derived view per architect
    /// §3 ruling.
    #[test]
    fn economic_state_does_not_have_node_market_t_field() {
        let e = EconomicState::default();
        let s = serde_json::to_value(&e).unwrap();
        let obj = s.as_object().unwrap();
        assert!(
            !obj.contains_key("node_market_t"),
            "TB-12 architect §3 ruling: node_market_t must NOT be a canonical \
             EconomicState field. NodeMarketEntry is TB-14 derived view only. \
             Adding node_market_t introduces second source-of-truth (architect §3.2)."
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
        assert_eq!(
            sa, sb,
            "BTreeMap must yield identical bytes regardless of insertion order"
        );
    }

    #[test]
    fn node_id_from_state_root_is_deterministic() {
        let r = Hash::from_bytes([0xAB; 32]);
        let n1 = NodeId::from_state_root(r);
        let n2 = NodeId::from_state_root(r);
        assert_eq!(n1, n2);
        assert_eq!(
            n1.0.len(),
            64,
            "40-byte git SHA hex form would be 40; we use full 32-byte sha256 hex = 64"
        );
    }
}
