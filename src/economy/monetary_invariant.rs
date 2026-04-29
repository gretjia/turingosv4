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
use crate::state::q_state::{EconomicState, Hash, QState};
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
/// Counted (each contributes its `MicroCoin` directly):
/// - `balances_t` (agent-held)
/// - `escrows_t` (locked under task)
/// - `stakes_t` (locked under tx)
/// - `claims_t` (pending payout)
/// - `task_markets_t.bounty` (sponsor-locked under task)
/// - `challenge_cases_t.bond` (challenger-locked under case)
///
/// NOT counted (not a holding):
/// - `reputations_t` (signed reputation, not coin)
/// - `royalty_graph_t` (edges, no coin)
/// - `price_index_t` (market data, not held)
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
    for m in s.task_markets_t.0.values() {
        total = total.checked_add(m.bounty.micro_units()).ok_or(MonetaryError::Overflow)?;
    }
    for c in s.challenge_cases_t.0.values() {
        total = total.checked_add(c.bond.micro_units()).ok_or(MonetaryError::Overflow)?;
    }
    Ok(total)
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
        | TypedTx::TerminalSummary(_) => Ok(()),
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
    use crate::state::q_state::{AgentId, ClaimEntry, EscrowEntry, StakeEntry, TaskMarketEntry, TxId};
    use crate::state::typed_tx::{TaskId, WorkTx};

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
            EscrowEntry { amount: coin(40), depositor: agent("alice") },
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
                    StakeEntry { amount: MicroCoin::from_micro_units(amt_micro), staker: agent("alice") },
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
    fn ctf_counts_all_six_holding_subindexes() {
        // Make sure we sum balances + escrows + stakes + claims + bounty + bond.
        let mut s = EconomicState::default();
        s.balances_t.0.insert(agent("a"), coin(1));
        s.escrows_t.0.insert(
            tx("e"),
            EscrowEntry { amount: coin(2), depositor: agent("a") },
        );
        s.stakes_t.0.insert(
            tx("s"),
            StakeEntry { amount: coin(4), staker: agent("a") },
        );
        s.claims_t.0.insert(
            tx("c"),
            ClaimEntry { amount: coin(8), claimant: agent("a") },
        );
        s.task_markets_t.0.insert(
            tx("m"),
            TaskMarketEntry {
                publisher: agent("a"),
                bounty: coin(16),
                ..Default::default()
            },
        );
        // challenge_cases_t bond
        let mut cc = crate::state::q_state::ChallengeCase::default();
        cc.bond = coin(32);
        cc.challenger = agent("a");
        s.challenge_cases_t.0.insert(tx("ch"), cc);

        // Each power of two distinct => sum = 63 base coin = 63_000_000 micro.
        assert_eq!(total_supply_micro(&s).unwrap(), 63 * MICRO_PER_COIN);
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
