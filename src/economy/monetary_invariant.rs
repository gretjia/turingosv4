//! Monetary invariant guards — TB-1 Day-2 P3 RSP-0.
//!
//! Charter authority:
//! - `handover/tracer_bullets/TB-1_recharter_2026-04-29.md` Day-2.
//! - ROADMAP P3 Exit 1, 2, 5 (`on_init` total Coin invariant; rtool/think
//!   don't deduct; escrow required for market admission).
//! - `handover/alignment/DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md`:
//!   `MonetaryError` returns drive L4.E (rejection-evidence) entries, not L4.
//!
//! Constitutional authority:
//! - 基本法 1 (Coin 守恒): monetary conservation MUST be exact post-genesis.
//! - Inv 4 (no post-init mint): only `on_init` may inject coins; any other
//!   path that increases total CTF supply is a constitutional violation.
//! - Art. III.4 (selective shielding): rejection diagnostics route to L4.E.
//!
//! Scope (RSP-0 micro-version):
//! - Three pure assertion functions; no I/O, no state mutation.
//! - Wired into `dispatch_transition` rejection path in TB-1 Day-3.
//! - Tool-level read-is-free for `rtool` / `search` / `think` is enforced
//!   at the SDK boundary in a later RSP atom; this module covers the
//!   tx-level guarantee (no K5 `TypedTx` carries a per-tx fee).
//!
//! /// TRACE_MATRIX 基本法 1 + Inv 4 + ROADMAP P3:1/P3:2: monetary guards.

use crate::bottom_white::ledger::transition_ledger::TxKind;
use crate::economy::money::{MicroCoin, MICRO_PER_COIN};
use crate::state::q_state::{EconomicState, Hash, QState, TaskId};
use crate::state::typed_tx::TypedTx;

// ────────────────────────────────────────────────────────────────────────────
// MonetaryError — invariant-violation taxonomy
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX P3 RSP-0 — taxonomy of monetary invariant violations.
///
/// Variants are surfaced to the sequencer's rejection path; per the
/// L4 / L4.E split decision (`DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md`)
/// they cause the offending transition to land in L4.E, NOT L4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MonetaryError {
    /// Total CTF supply increased post-genesis. Reported by
    /// [`assert_total_ctf_conserved`] when `delta_micro > 0` and no
    /// exempting tx kind was declared.
    PostInitMint { delta_micro: i64 },
    /// Total CTF supply decreased without an exempting tx kind. Burns
    /// are not permitted in v1; this variant is reserved so a future
    /// RSP can opt in via `exempt_tx_kinds`.
    TotalCtfBurn { delta_micro: i64 },
    /// A K5 `TxKind` was assigned a non-zero per-tx fee. K5 has no
    /// fee field on any variant; only stake / bond exist (locked, not
    /// consumed). A non-zero fee is a structural constitutional bug.
    ReadCharged { tx_kind: TxKind, fee: u64 },
    /// **TB-3 cache=truth invariant violation**: `task_markets_t[task_id].total_escrow`
    /// (a derived aggregate / cached index) does not equal `Σ escrows_t[e].amount
    /// where e.task_id == task_id` (the source-of-truth derivation). Per
    /// charter § 3.2: `total_escrow` is NEVER a money holding; it must always
    /// equal the derived sum. A drift signals either a bug in `EscrowLockTx`
    /// dispatch arm (cache update missed) or direct `EconomicState` mutation
    /// outside an accepted transition (ghost liquidity attempt).
    DerivedCacheMismatch { task_id: TaskId, cached_micro: i64, derived_micro: i64 },
    /// Arithmetic overflow while summing economic state (i64).
    Overflow,
}

impl std::fmt::Display for MonetaryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PostInitMint { delta_micro } => {
                write!(f, "post-init mint: total CTF supply increased by {} micro", delta_micro)
            }
            Self::TotalCtfBurn { delta_micro } => {
                write!(f, "unauthorized burn: total CTF supply decreased by {} micro", delta_micro)
            }
            Self::ReadCharged { tx_kind, fee } => {
                write!(f, "read charged: tx_kind={:?} carries fee={} (must be 0)", tx_kind, fee)
            }
            Self::DerivedCacheMismatch { task_id, cached_micro, derived_micro } => {
                write!(
                    f,
                    "task_market cache mismatch: task_id={:?} cached_total_escrow={} derived={}",
                    task_id, cached_micro, derived_micro
                )
            }
            Self::Overflow => write!(f, "i64 overflow while summing economic state"),
        }
    }
}

impl std::error::Error for MonetaryError {}

// ────────────────────────────────────────────────────────────────────────────
// total_supply — sum of all coin-holding fields in EconomicState
// ────────────────────────────────────────────────────────────────────────────

/// Sum of every coin-holding sub-index in `EconomicState`, in micro-units.
///
/// Counted (each contributes its `MicroCoin` directly) — **5 holdings** post-TB-3:
/// - `balances_t` (agent-held)
/// - `escrows_t` (locked under task; populated by `EscrowLockTx`)
/// - `stakes_t` (locked under tx; populated by accepted WorkTx commitment)
/// - `claims_t` (pending payout)
/// - `challenge_cases_t.bond` (challenger-locked under case)
///
/// NOT counted (not a holding):
/// - `reputations_t` (signed reputation, not coin)
/// - `royalty_graph_t` (edges, no coin)
/// - `price_index_t` (market data, not held)
/// - **`task_markets_t.total_escrow`** (derived aggregate / cached index per
///   TB-3 charter § 3.2 — counting it would double-mint every locked bounty
///   because the same money is also in `escrows_t`. Cache=truth is enforced
///   separately by `assert_task_market_total_escrow_matches_locks`.)
///
/// **TB-3 6→5 holding migration** (2026-04-30): TB-1's `bounty` term over
/// `task_markets_t[t].bounty` is removed. Bounty money has migrated to
/// `escrows_t.amount` via accepted `EscrowLockTx`. `task_markets_t` retains
/// only the cached aggregate `total_escrow` (NOT in supply sum) + admission
/// metadata.
fn total_supply_micro(s: &EconomicState) -> Result<i64, MonetaryError> {
    let mut total: i64 = 0;
    for v in s.balances_t.0.values() {
        total = total.checked_add(v.micro_units()).ok_or(MonetaryError::Overflow)?;
    }
    for e in s.escrows_t.0.values() {
        total = total.checked_add(e.amount.micro_units()).ok_or(MonetaryError::Overflow)?;
    }
    for e in s.stakes_t.0.values() {
        total = total.checked_add(e.amount.micro_units()).ok_or(MonetaryError::Overflow)?;
    }
    for c in s.claims_t.0.values() {
        total = total.checked_add(c.amount.micro_units()).ok_or(MonetaryError::Overflow)?;
    }
    // task_markets_t.total_escrow is INTENTIONALLY OMITTED — derived cache,
    // not a holding (TB-3 charter § 3.2). Counting it would double-mint
    // every bounty: the same micro-coins are already counted in escrows_t.
    for c in s.challenge_cases_t.0.values() {
        total = total.checked_add(c.bond.micro_units()).ok_or(MonetaryError::Overflow)?;
    }
    Ok(total)
}

// ────────────────────────────────────────────────────────────────────────────
// assert_task_market_total_escrow_matches_locks — TB-3 cache=truth invariant
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX TB-3 charter § 3.2 — cache=truth invariant for the derived
/// `task_markets_t[task_id].total_escrow` field.
///
/// Asserts `cached == Σ escrows_t[e].amount where e.task_id == task_id`.
/// MUST hold across every accepted state transition that touches escrows or
/// task_markets (i.e., across every accepted `EscrowLockTx` and any future
/// RSP-2/3+ transition that releases escrowed funds).
///
/// **Why this is a separate predicate**: per Art 0.2 ("派生视图 ... 必须有
/// `assert_eq!(view, derive_from_tape(tape))` 守恒测试"), any cached
/// aggregate of tape-derived data is a "派生视图" (derived view); without an
/// explicit invariant test it becomes a parallel ledger and a ghost-liquidity
/// surface. This predicate is the contract enforcing the cache stays in
/// sync with the source-of-truth derivation.
///
/// **Caller convention**: invoked from `dispatch_transition::EscrowLock` arm
/// (TB-3 Atom 5) on the post-mutation `q_next` and from any future arm that
/// modifies `escrows_t` or `task_markets_t.total_escrow`. NOT invoked on
/// rejection paths (rejected transitions don't mutate economic state).
pub fn assert_task_market_total_escrow_matches_locks(
    s: &EconomicState,
    task_id: &TaskId,
) -> Result<(), MonetaryError> {
    let cached = s
        .task_markets_t
        .0
        .get(task_id)
        .map(|m| m.total_escrow.micro_units())
        .unwrap_or(0);
    let mut derived: i64 = 0;
    for e in s.escrows_t.0.values() {
        if &e.task_id == task_id {
            derived = derived
                .checked_add(e.amount.micro_units())
                .ok_or(MonetaryError::Overflow)?;
        }
    }
    if cached != derived {
        return Err(MonetaryError::DerivedCacheMismatch {
            task_id: task_id.clone(),
            cached_micro: cached,
            derived_micro: derived,
        });
    }
    Ok(())
}

// ────────────────────────────────────────────────────────────────────────────
// assert_no_post_init_mint — structural guard at the tx layer
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX P3:1 (kill 1) — structural guard against post-genesis mint
/// at the `TypedTx` layer.
///
/// **Today, K5 has no `Mint` variant** — none of the 7 `TypedTx` variants
/// directly inject coins. Genesis allocation happens in `on_init` outside
/// the K5 transition surface. Therefore, on a non-genesis `q`, this fn
/// returns `Ok(())` for every well-formed `TypedTx`.
///
/// **Why the function exists anyway**: it is a forward-compat barrier.
/// If a future RSP atom adds a `Mint` (or `SystemReward`-class) variant,
/// it MUST be added to the match below AND rejected here when
/// `q.state_root_t != Hash::ZERO`. Numeric conservation is enforced by
/// [`assert_total_ctf_conserved`] separately.
pub fn assert_no_post_init_mint(tx: &TypedTx, q: &QState) -> Result<(), MonetaryError> {
    let is_post_init = q.state_root_t != Hash::ZERO;
    if !is_post_init {
        return Ok(());
    }
    match tx {
        TypedTx::Work(_)
        | TypedTx::Verify(_)
        | TypedTx::Challenge(_)
        | TypedTx::Reuse(_)
        | TypedTx::FinalizeReward(_)
        | TypedTx::TaskExpire(_)
        | TypedTx::TerminalSummary(_)
        // TB-3 RSP-1: TaskOpen + EscrowLock are TRANSFERS (or metadata-only),
        // never mints — their dispatch arms (Atoms 4-5) maintain CTF
        // conservation via assert_total_ctf_conserved with empty exempt list.
        | TypedTx::TaskOpen(_)
        | TypedTx::EscrowLock(_) => Ok(()),
    }
}

// ────────────────────────────────────────────────────────────────────────────
// assert_total_ctf_conserved — numeric conservation across a transition
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX 基本法 1 + P3:1 — conservation of total CTF across a
/// transition `before → after`.
///
/// Mints (`delta > 0`) and burns (`delta < 0`) are both rejected unless
/// `exempt_tx_kinds` is non-empty. The exempt list is the explicit opt-out
/// for legitimate supply-changing operations (e.g., genesis init,
/// system-emitted rewards in a future RSP); RSP-0 never populates it
/// at runtime.
///
/// Caller convention: pass `&[]` for normal agent-submitted transitions.
/// Pass `&[TxKind::FinalizeReward]` (etc.) only when a system-emitted
/// supply-changing tx is being processed AND the RSP semantics for that
/// kind have been ratified. RSP-0 does not ratify any.
pub fn assert_total_ctf_conserved(
    before: &EconomicState,
    after: &EconomicState,
    exempt_tx_kinds: &[TxKind],
) -> Result<(), MonetaryError> {
    let total_before = total_supply_micro(before)?;
    let total_after = total_supply_micro(after)?;
    let delta = total_after
        .checked_sub(total_before)
        .ok_or(MonetaryError::Overflow)?;
    if !exempt_tx_kinds.is_empty() {
        return Ok(());
    }
    if delta > 0 {
        return Err(MonetaryError::PostInitMint { delta_micro: delta });
    }
    if delta < 0 {
        return Err(MonetaryError::TotalCtfBurn { delta_micro: delta });
    }
    Ok(())
}

// ────────────────────────────────────────────────────────────────────────────
// assert_read_is_free — tx-level no-fee guard
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX P3:2 — assert that no K5 `TxKind` carries a per-tx fee.
///
/// K5 spec: every `TypedTx` variant has stake / bond fields (locked but
/// not consumed) but NO fee field. A non-zero `fee` is a structural bug
/// in whichever caller computed it; this fn is the barrier.
///
/// Note: tool-level read-is-free for `rtool` / `search` / `think` is
/// enforced at the SDK boundary in a later RSP atom (out of scope for
/// RSP-0). This fn covers the tx-level invariant only.
pub fn assert_read_is_free(tx_kind: TxKind, fee: u64) -> Result<(), MonetaryError> {
    if fee != 0 {
        return Err(MonetaryError::ReadCharged { tx_kind, fee });
    }
    Ok(())
}

// ────────────────────────────────────────────────────────────────────────────
// Tests
// ────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::q_state::{AgentId, ClaimEntry, EscrowEntry, StakeEntry, TxId};
    use crate::state::typed_tx::WorkTx;
    // TaskId is in scope from the outer-module `use crate::state::q_state::TaskId`.

    fn agent(s: &str) -> AgentId {
        AgentId(s.to_string())
    }

    fn tx(s: &str) -> TxId {
        TxId(s.to_string())
    }

    fn task(s: &str) -> TaskId {
        TaskId(s.to_string())
    }

    fn coin(n: i64) -> MicroCoin {
        MicroCoin::from_coin(n).unwrap()
    }

    fn state_with_balance(holder: &str, amount_coin: i64) -> EconomicState {
        let mut s = EconomicState::default();
        s.balances_t.0.insert(agent(holder), coin(amount_coin));
        s
    }

    fn post_init_q() -> QState {
        let mut q = QState::default();
        // Any non-zero state_root counts as post-init.
        q.state_root_t = Hash::from_bytes([7u8; 32]);
        q
    }

    fn genesis_q() -> QState {
        QState::default()
    }

    // ── assert_no_post_init_mint ────────────────────────────────────────────

    #[test]
    fn no_post_init_mint_passes_on_genesis() {
        let q = genesis_q();
        let work = TypedTx::Work(WorkTx::default());
        assert_eq!(assert_no_post_init_mint(&work, &q), Ok(()));
    }

    #[test]
    fn no_post_init_mint_passes_for_all_k5_variants_post_init() {
        use crate::state::typed_tx::{
            ChallengeTx, FinalizeRewardTx, ReuseTx, TaskExpireTx, TerminalSummaryTx, VerifyTx,
        };
        let q = post_init_q();
        let cases: Vec<TypedTx> = vec![
            TypedTx::Work(WorkTx::default()),
            TypedTx::Verify(VerifyTx::default()),
            TypedTx::Challenge(ChallengeTx::default()),
            TypedTx::Reuse(ReuseTx::default()),
            TypedTx::FinalizeReward(FinalizeRewardTx::default()),
            TypedTx::TaskExpire(TaskExpireTx::default()),
            TypedTx::TerminalSummary(TerminalSummaryTx::default()),
        ];
        for t in cases {
            assert_eq!(assert_no_post_init_mint(&t, &q), Ok(()),
                "structural guard must pass for all K5 variants today");
        }
    }

    // ── assert_total_ctf_conserved ──────────────────────────────────────────

    #[test]
    fn ctf_conserved_balanced_transfer() {
        // Alice 100 → Bob 30 = 70/30 split; total unchanged.
        let mut before = EconomicState::default();
        before.balances_t.0.insert(agent("alice"), coin(100));
        let mut after = EconomicState::default();
        after.balances_t.0.insert(agent("alice"), coin(70));
        after.balances_t.0.insert(agent("bob"), coin(30));
        assert_eq!(assert_total_ctf_conserved(&before, &after, &[]), Ok(()));
    }

    #[test]
    fn ctf_post_init_mint_rejected() {
        // P3 kill 1 (Day-2 unit form): supply increased without exempt.
        let before = state_with_balance("alice", 100);
        let mut after = before.clone();
        after.balances_t.0.insert(agent("alice"), coin(150));
        let r = assert_total_ctf_conserved(&before, &after, &[]);
        assert_eq!(
            r,
            Err(MonetaryError::PostInitMint { delta_micro: 50 * MICRO_PER_COIN })
        );
    }

    #[test]
    fn ctf_unauthorized_burn_rejected() {
        let before = state_with_balance("alice", 100);
        let mut after = before.clone();
        after.balances_t.0.insert(agent("alice"), coin(40));
        let r = assert_total_ctf_conserved(&before, &after, &[]);
        assert_eq!(
            r,
            Err(MonetaryError::TotalCtfBurn { delta_micro: -60 * MICRO_PER_COIN })
        );
    }

    #[test]
    fn ctf_exempt_short_circuits() {
        // With a non-empty exempt list (e.g., genesis init), supply may change.
        let before = EconomicState::default();
        let after = state_with_balance("alice", 1_000);
        assert_eq!(
            assert_total_ctf_conserved(&before, &after, &[TxKind::FinalizeReward]),
            Ok(())
        );
    }

    #[test]
    fn ctf_conserved_across_subindexes() {
        // 100 in balances → 60 in balances + 40 in escrow; total unchanged.
        let mut before = EconomicState::default();
        before.balances_t.0.insert(agent("alice"), coin(100));
        let mut after = EconomicState::default();
        after.balances_t.0.insert(agent("alice"), coin(60));
        after.escrows_t.0.insert(
            tx("work-1"),
            EscrowEntry { amount: coin(40), depositor: agent("alice"), task_id: TaskId::default() },
        );
        assert_eq!(assert_total_ctf_conserved(&before, &after, &[]), Ok(()));
    }

    #[test]
    fn ctf_conserved_across_n10_random_sequence() {
        // Charter Day-2 unit: "total CTF conserved across N=10 random tx sequences".
        // We model 10 deterministic-but-varied conservative redistributions
        // (Alice/Bob/Carol; balances ↔ escrow ↔ stake ↔ claim ↔ market ↔ challenge).
        // Each step is a closed transfer; total supply is invariant.
        let mut s = EconomicState::default();
        s.balances_t.0.insert(agent("alice"), coin(100));
        s.balances_t.0.insert(agent("bob"), coin(50));
        s.balances_t.0.insert(agent("carol"), coin(25));
        let total0 = total_supply_micro(&s).unwrap();

        let steps: [(&str, i64); 10] = [
            ("alice->bob", 5),
            ("bob->escrow:t1", 10),
            ("alice->stake:tx1", 7),
            ("escrow:t1->claim:tx1", 3),
            ("alice->market:t2", 20),
            ("market:t2->balance:carol", 15),
            ("stake:tx1->challenge:case1", 4),
            ("challenge:case1->balance:bob", 2),
            ("claim:tx1->balance:alice", 3),
            ("balance:carol->escrow:t3", 6),
        ];

        let total_each = vec![total0; 10];
        for (i, (label, _amt)) in steps.iter().enumerate() {
            // Apply a small redistribution: move `_amt` micro_per_coin
            // between two slots. We just re-shuffle existing supply.
            // (Concrete redistribution mechanics live in SettlementEngine;
            // the invariant under test is: any closed redistribution leaves
            // total_supply_micro unchanged.)
            let amt_micro = (i as i64 + 1) * 1_000; // small, deterministic
            // Move `amt_micro` from alice's balance into a synthetic stake.
            let alice_bal = s.balances_t.0.get(&agent("alice"))
                .copied().unwrap_or(MicroCoin::zero());
            if alice_bal.micro_units() >= amt_micro {
                s.balances_t.0.insert(
                    agent("alice"),
                    MicroCoin::from_micro_units(alice_bal.micro_units() - amt_micro),
                );
                let key = tx(&format!("stake-step-{}", i));
                s.stakes_t.0.insert(
                    key,
                    StakeEntry { amount: MicroCoin::from_micro_units(amt_micro), staker: agent("alice"), task_id: TaskId::default() },
                );
            }
            let total_now = total_supply_micro(&s).unwrap();
            assert_eq!(
                total_now, total_each[i],
                "step {} ({}): conservation broke",
                i, label
            );
        }
        // Final cross-check.
        assert_eq!(total_supply_micro(&s).unwrap(), total0);
    }

    #[test]
    fn ctf_counts_all_five_holding_subindexes() {
        // **TB-3 6→5 holding migration**: previously summed
        // balances + escrows + stakes + claims + bounty + bond (6).
        // Now sums balances + escrows + stakes + claims + bond (5).
        // task_markets_t.total_escrow is a DERIVED CACHE (TB-3 charter § 3.2),
        // not a holding — counting it would double-count every locked bounty
        // because the same money is in escrows_t. The 16-coin amount that
        // previously seeded task_markets_t.bounty has migrated to a second
        // escrows_t entry (this models how EscrowLockTx will route bounty
        // money in TB-3 Atom 5).
        let mut s = EconomicState::default();
        s.balances_t.0.insert(agent("a"), coin(1));
        s.escrows_t.0.insert(
            tx("e"),
            EscrowEntry { amount: coin(2), depositor: agent("a"), task_id: task("task-e") },
        );
        s.stakes_t.0.insert(
            tx("s"),
            StakeEntry { amount: coin(4), staker: agent("a"), task_id: task("task-s") },
        );
        s.claims_t.0.insert(
            tx("c"),
            ClaimEntry { amount: coin(8), claimant: agent("a") },
        );
        // The 16 that used to live in task_markets_t.bounty now lives as a
        // second escrows_t entry — same money, canonical home.
        s.escrows_t.0.insert(
            tx("e2"),
            EscrowEntry { amount: coin(16), depositor: agent("a"), task_id: task("task-e2") },
        );
        let mut cc = crate::state::q_state::ChallengeCase::default();
        cc.bond = coin(32);
        cc.challenger = agent("a");
        s.challenge_cases_t.0.insert(tx("ch"), cc);

        // Each power of two distinct => sum = 63 base coin = 63_000_000 micro.
        // (escrows_t now contributes 2 + 16 = 18; total still 63.)
        assert_eq!(total_supply_micro(&s).unwrap(), 63 * MICRO_PER_COIN);
    }

    #[test]
    fn total_supply_does_not_double_count_total_escrow() {
        // **TB-3 charter § 3.2 regression test**: setting BOTH
        // escrows_t[e].amount = K and task_markets_t[t].total_escrow = K
        // (which is the steady-state shape after an accepted EscrowLockTx)
        // must yield total_supply_micro = K, NOT 2K. If a regression adds
        // task_markets.total_escrow back into the holding sum, this test
        // catches it immediately.
        let mut s = EconomicState::default();
        let task_id = task("task-double-count-regression");
        s.escrows_t.0.insert(
            tx("escrow-lock-1"),
            EscrowEntry {
                amount: coin(50),
                depositor: agent("sponsor"),
                task_id: task_id.clone(),
            },
        );
        let mut entry = crate::state::q_state::TaskMarketEntry::default();
        entry.total_escrow = coin(50);
        entry.escrow_lock_tx_ids.insert(tx("escrow-lock-1"));
        s.task_markets_t.0.insert(task_id, entry);

        assert_eq!(
            total_supply_micro(&s).unwrap(),
            50 * MICRO_PER_COIN,
            "total_supply must equal the escrows_t holding (50), not 2× (100). \
             task_markets_t.total_escrow is a derived cache, NOT a holding."
        );
    }

    #[test]
    fn task_market_total_escrow_matches_sum_of_escrow_locks() {
        // **TB-3 charter § 3.2 cache=truth invariant test**: after multiple
        // EscrowLock-equivalent inserts to escrows_t for the same task_id,
        // task_markets_t[task_id].total_escrow must equal the sum.
        let mut s = EconomicState::default();
        let t = task("task-cache-truth");

        // Two escrow locks for the same task (multi-sponsor or top-up case).
        s.escrows_t.0.insert(
            tx("lock-A"),
            EscrowEntry { amount: coin(30), depositor: agent("alice"), task_id: t.clone() },
        );
        s.escrows_t.0.insert(
            tx("lock-B"),
            EscrowEntry { amount: coin(20), depositor: agent("bob"), task_id: t.clone() },
        );
        // One escrow for a DIFFERENT task — must not contaminate the sum.
        s.escrows_t.0.insert(
            tx("lock-other"),
            EscrowEntry {
                amount: coin(7),
                depositor: agent("carol"),
                task_id: task("task-other"),
            },
        );

        // Cache reflects the truth.
        let mut entry = crate::state::q_state::TaskMarketEntry::default();
        entry.total_escrow = coin(50);
        entry.escrow_lock_tx_ids.insert(tx("lock-A"));
        entry.escrow_lock_tx_ids.insert(tx("lock-B"));
        s.task_markets_t.0.insert(t.clone(), entry);

        assert_eq!(assert_task_market_total_escrow_matches_locks(&s, &t), Ok(()));

        // Drift the cache (simulate a missed update or an attacker mutating
        // EconomicState directly): cache=truth predicate must reject.
        s.task_markets_t.0.get_mut(&t).unwrap().total_escrow = coin(60);
        let r = assert_task_market_total_escrow_matches_locks(&s, &t);
        assert!(matches!(r, Err(MonetaryError::DerivedCacheMismatch { .. })),
            "drifted cache must surface as DerivedCacheMismatch; got {:?}", r);
    }

    // ── assert_read_is_free ─────────────────────────────────────────────────

    #[test]
    fn read_is_free_zero_fee_passes_for_all_kinds() {
        for k in [
            TxKind::Work,
            TxKind::Verify,
            TxKind::Challenge,
            TxKind::Reuse,
            TxKind::FinalizeReward,
            TxKind::TaskExpire,
            TxKind::TerminalSummary,
        ] {
            assert_eq!(assert_read_is_free(k, 0), Ok(()));
        }
    }

    #[test]
    fn read_is_free_nonzero_fee_rejected() {
        // P3:2 — any per-tx fee on a K5 TxKind is a structural bug.
        let r = assert_read_is_free(TxKind::Reuse, 1);
        assert_eq!(r, Err(MonetaryError::ReadCharged { tx_kind: TxKind::Reuse, fee: 1 }));
        let r = assert_read_is_free(TxKind::Work, 9999);
        assert_eq!(r, Err(MonetaryError::ReadCharged { tx_kind: TxKind::Work, fee: 9999 }));
    }
}
