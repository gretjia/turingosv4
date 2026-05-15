//! Escrow vault — TB-1 Day-2 P3 RSP-0.
//!
//! Charter authority:
//! - `handover/tracer_bullets/TB-1_recharter_2026-04-29.md` Day-2.
//! - ROADMAP P3 Exit 5: an escrow lock is taken before `work_tx` is admitted.
//! - ROADMAP P3 Exit 6, 8: acceptance produces only `provisional_accept`,
//!   not full payout; `settlement_tx.payout_sum ≤ escrow_pool`.
//!
//! Constitutional authority:
//! - 基本法 1 (Coin 守恒): payouts come from pre-locked escrow; sum may
//!   not exceed locked amount; residual returns to sponsor.
//! - Inv 3 (escrow only): write-side mutations require prior escrow lock.
//!
//! Scope (RSP-0 micro-version):
//! - In-memory `BTreeMap<TaskId, VaultEntry>` keyed by task.
//! - `lock_escrow` records sponsor-locked supply for one task.
//! - `release_escrow` distributes payouts, asserts `Σ payouts ≤ locked`,
//!   computes residual-to-sponsor.
//! - No I/O, no L4 emission. Wiring into `dispatch_transition` /
//!   `SettlementEngine` lands in TB-1 Day-3 / TB-2 (RSP-1).
//!
//! Out of scope (deferred):
//! - Per-claim sub-vault accounting (RSP-2 SettlementEngine).
//! - Multi-sponsor co-escrow (RSP-3 ChallengeCourt).
//! - Persistence to L4 / state.db (TB-1 Day-3 wiring).
//!
//! /// TRACE_MATRIX 基本法 1 + Inv 3 + ROADMAP P3:5/P3:6/P3:8: escrow vault.

use std::collections::BTreeMap;

use crate::economy::money::MicroCoin;
use crate::state::q_state::{AgentId, TaskId};

// ────────────────────────────────────────────────────────────────────────────
// VaultEntry — one task's escrow record (status + payout log)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX P3:5 — the lifecycle status of a task's escrow.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EscrowStatus {
    /// Funds locked under the task; awaiting release.
    Locked,
    /// Funds distributed; payout log is final; residual already
    /// computed and surfaced via [`VaultEntry::residual_to_sponsor`].
    Released,
}

/// TRACE_MATRIX P3:5 — single task's escrow record.
///
/// Distinct from `state::q_state::EscrowEntry` (which is the L4-state
/// tx-keyed projection). This vault is the operational task-keyed working
/// view used by the sequencer / SettlementEngine before flushing to L4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VaultEntry {
    pub task_id: TaskId,
    pub sponsor: AgentId,
    pub locked_amount: MicroCoin,
    pub status: EscrowStatus,
    /// Per-recipient payouts, populated on release.
    pub payouts: BTreeMap<AgentId, MicroCoin>,
    /// `locked_amount − Σ payouts`, populated on release.
    pub residual_to_sponsor: MicroCoin,
}

/// TRACE_MATRIX P3:5 — receipt returned by [`EscrowVault::lock_escrow`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EscrowReceipt {
    pub task_id: TaskId,
    pub sponsor: AgentId,
    pub locked_amount: MicroCoin,
}

/// TRACE_MATRIX P3:6/P3:8 — outcome returned by [`EscrowVault::release_escrow`].
///
/// Captures the per-recipient distribution and the residual that returns
/// to the sponsor. Caller is responsible for crediting the residual back
/// to the sponsor's `BalancesIndex` entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseOutcome {
    pub task_id: TaskId,
    pub sponsor: AgentId,
    pub paid_total: MicroCoin,
    pub residual_to_sponsor: MicroCoin,
}

// ────────────────────────────────────────────────────────────────────────────
// EscrowError — vault-operation errors
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX P3 RSP-0 — errors returned by `EscrowVault` operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EscrowError {
    /// `lock_escrow` called twice for the same `task_id` without an
    /// intervening release; rejected to keep escrow per-task unique.
    AlreadyLocked { task_id: TaskId },
    /// `release_escrow` called for a `task_id` with no prior lock.
    NotFound { task_id: TaskId },
    /// `release_escrow` called for a `task_id` already released.
    AlreadyReleased { task_id: TaskId },
    /// Σ payouts exceeds `locked_amount` (Inv 3 violation).
    PayoutExceedsLocked {
        task_id: TaskId,
        locked_micro: i64,
        requested_micro: i64,
    },
    /// A negative amount was passed (lock or payout); rejected at the
    /// vault layer to keep monetary math non-negative-by-default.
    NegativeAmount,
    /// i64 overflow while summing payouts or computing residual.
    Overflow,
}

impl std::fmt::Display for EscrowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyLocked { task_id } => {
                write!(f, "escrow already locked for task {:?}", task_id.0)
            }
            Self::NotFound { task_id } => {
                write!(f, "no escrow lock for task {:?}", task_id.0)
            }
            Self::AlreadyReleased { task_id } => {
                write!(f, "escrow already released for task {:?}", task_id.0)
            }
            Self::PayoutExceedsLocked {
                task_id,
                locked_micro,
                requested_micro,
            } => {
                write!(
                    f,
                    "payout exceeds locked for task {:?}: locked={} micro, requested={} micro",
                    task_id.0, locked_micro, requested_micro
                )
            }
            Self::NegativeAmount => write!(f, "negative monetary amount rejected"),
            Self::Overflow => write!(f, "i64 overflow in escrow arithmetic"),
        }
    }
}

impl std::error::Error for EscrowError {}

// ────────────────────────────────────────────────────────────────────────────
// EscrowVault — task-keyed in-memory vault (RSP-0)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX P3 RSP-0 — minimum-viable in-memory escrow vault.
///
/// One entry per `TaskId`. Locking is one-shot per task in RSP-0
/// (multi-sponsor co-escrow is RSP-3). Release is one-shot and final.
#[derive(Debug, Clone, Default)]
pub struct EscrowVault {
    entries: BTreeMap<TaskId, VaultEntry>,
}

impl EscrowVault {
    /// TRACE_MATRIX P3 RSP-0 — empty vault constructor.
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }

    /// TRACE_MATRIX P3 RSP-0 — number of recorded tasks (diagnostics-only accessor).
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// TRACE_MATRIX P3 RSP-0 — emptiness predicate (diagnostics-only accessor).
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// TRACE_MATRIX P3 RSP-0 — read-only entry lookup; backs residual / payout assertions.
    pub fn get(&self, task_id: &TaskId) -> Option<&VaultEntry> {
        self.entries.get(task_id)
    }

    /// TRACE_MATRIX P3:5 — record sponsor-locked supply for one task.
    ///
    /// Returns `EscrowError::AlreadyLocked` if a prior lock exists for the
    /// same `task_id` (whether `Locked` or `Released`); a released task
    /// cannot be re-locked under RSP-0 semantics.
    /// Returns `EscrowError::NegativeAmount` for negative `amount`.
    pub fn lock_escrow(
        &mut self,
        task_id: TaskId,
        sponsor: AgentId,
        amount: MicroCoin,
    ) -> Result<EscrowReceipt, EscrowError> {
        if amount.is_negative() {
            return Err(EscrowError::NegativeAmount);
        }
        if self.entries.contains_key(&task_id) {
            return Err(EscrowError::AlreadyLocked { task_id });
        }
        let entry = VaultEntry {
            task_id: task_id.clone(),
            sponsor: sponsor.clone(),
            locked_amount: amount,
            status: EscrowStatus::Locked,
            payouts: BTreeMap::new(),
            residual_to_sponsor: MicroCoin::zero(),
        };
        self.entries.insert(task_id.clone(), entry);
        Ok(EscrowReceipt {
            task_id,
            sponsor,
            locked_amount: amount,
        })
    }

    /// TRACE_MATRIX P3:6/P3:8 — distribute payouts, assert `Σ payouts ≤ locked`,
    /// compute residual-to-sponsor, mark task `Released`.
    ///
    /// Empty `payouts` is permitted (full residual returns to sponsor — the
    /// `TaskExpire` shape).
    /// Negative payout amounts are rejected (`NegativeAmount`).
    /// `Σ payouts > locked` → `PayoutExceedsLocked` and the entry is left
    /// in `Locked` status (operation atomic-rejected).
    pub fn release_escrow(
        &mut self,
        task_id: &TaskId,
        payouts: &BTreeMap<AgentId, MicroCoin>,
    ) -> Result<ReleaseOutcome, EscrowError> {
        let entry = self
            .entries
            .get_mut(task_id)
            .ok_or_else(|| EscrowError::NotFound {
                task_id: task_id.clone(),
            })?;

        if entry.status == EscrowStatus::Released {
            return Err(EscrowError::AlreadyReleased {
                task_id: task_id.clone(),
            });
        }

        let mut total_paid = MicroCoin::zero();
        for amt in payouts.values() {
            if amt.is_negative() {
                return Err(EscrowError::NegativeAmount);
            }
            total_paid = total_paid.checked_add(*amt).ok_or(EscrowError::Overflow)?;
        }

        if total_paid.micro_units() > entry.locked_amount.micro_units() {
            return Err(EscrowError::PayoutExceedsLocked {
                task_id: task_id.clone(),
                locked_micro: entry.locked_amount.micro_units(),
                requested_micro: total_paid.micro_units(),
            });
        }

        let residual = entry
            .locked_amount
            .checked_sub(total_paid)
            .ok_or(EscrowError::Overflow)?;

        entry.payouts = payouts.clone();
        entry.residual_to_sponsor = residual;
        entry.status = EscrowStatus::Released;

        Ok(ReleaseOutcome {
            task_id: task_id.clone(),
            sponsor: entry.sponsor.clone(),
            paid_total: total_paid,
            residual_to_sponsor: residual,
        })
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Tests
// ────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn agent(s: &str) -> AgentId {
        AgentId(s.to_string())
    }

    fn task(s: &str) -> TaskId {
        TaskId(s.to_string())
    }

    fn coin(n: i64) -> MicroCoin {
        MicroCoin::from_coin(n).unwrap()
    }

    // ── lock ────────────────────────────────────────────────────────────────

    #[test]
    fn lock_records_entry_and_returns_receipt() {
        let mut v = EscrowVault::new();
        let r = v
            .lock_escrow(task("t1"), agent("alice"), coin(100))
            .unwrap();
        assert_eq!(r.task_id, task("t1"));
        assert_eq!(r.sponsor, agent("alice"));
        assert_eq!(r.locked_amount, coin(100));
        let e = v.get(&task("t1")).unwrap();
        assert_eq!(e.status, EscrowStatus::Locked);
        assert_eq!(e.locked_amount, coin(100));
        assert_eq!(e.residual_to_sponsor, MicroCoin::zero());
        assert!(e.payouts.is_empty());
    }

    #[test]
    fn lock_rejects_double_lock_same_task() {
        let mut v = EscrowVault::new();
        v.lock_escrow(task("t1"), agent("alice"), coin(100))
            .unwrap();
        let r = v.lock_escrow(task("t1"), agent("bob"), coin(50));
        assert_eq!(
            r,
            Err(EscrowError::AlreadyLocked {
                task_id: task("t1")
            })
        );
    }

    #[test]
    fn lock_rejects_negative_amount() {
        let mut v = EscrowVault::new();
        let r = v.lock_escrow(task("t1"), agent("alice"), MicroCoin::from_micro_units(-1));
        assert_eq!(r, Err(EscrowError::NegativeAmount));
    }

    // ── release ─────────────────────────────────────────────────────────────

    #[test]
    fn release_overpayout_rejected() {
        // Charter Day-2 unit: "escrow over-payout rejected".
        let mut v = EscrowVault::new();
        v.lock_escrow(task("t1"), agent("alice"), coin(100))
            .unwrap();
        let mut payouts = BTreeMap::new();
        payouts.insert(agent("solver"), coin(60));
        payouts.insert(agent("verifier"), coin(50));
        let r = v.release_escrow(&task("t1"), &payouts);
        assert!(matches!(r, Err(EscrowError::PayoutExceedsLocked { .. })));
        // Atomic rejection: entry still Locked, no payouts recorded.
        let e = v.get(&task("t1")).unwrap();
        assert_eq!(e.status, EscrowStatus::Locked);
        assert!(e.payouts.is_empty());
    }

    #[test]
    fn release_underpayout_residual_returns_to_sponsor() {
        // Charter Day-2 unit: "escrow under-payout accepted (residual returns to sponsor)".
        let mut v = EscrowVault::new();
        v.lock_escrow(task("t1"), agent("alice"), coin(100))
            .unwrap();
        let mut payouts = BTreeMap::new();
        payouts.insert(agent("solver"), coin(60));
        payouts.insert(agent("verifier"), coin(10));
        let outcome = v.release_escrow(&task("t1"), &payouts).unwrap();
        assert_eq!(outcome.paid_total, coin(70));
        assert_eq!(outcome.residual_to_sponsor, coin(30));
        assert_eq!(outcome.sponsor, agent("alice"));
        let e = v.get(&task("t1")).unwrap();
        assert_eq!(e.status, EscrowStatus::Released);
        assert_eq!(e.residual_to_sponsor, coin(30));
        assert_eq!(e.payouts.len(), 2);
    }

    #[test]
    fn release_exact_payout_zero_residual() {
        let mut v = EscrowVault::new();
        v.lock_escrow(task("t1"), agent("alice"), coin(100))
            .unwrap();
        let mut payouts = BTreeMap::new();
        payouts.insert(agent("solver"), coin(70));
        payouts.insert(agent("verifier"), coin(30));
        let outcome = v.release_escrow(&task("t1"), &payouts).unwrap();
        assert_eq!(outcome.paid_total, coin(100));
        assert_eq!(outcome.residual_to_sponsor, MicroCoin::zero());
    }

    #[test]
    fn release_empty_payouts_full_residual_to_sponsor() {
        // TaskExpire shape: deadline lapsed, no winners; full bounty refunds.
        let mut v = EscrowVault::new();
        v.lock_escrow(task("t1"), agent("alice"), coin(100))
            .unwrap();
        let payouts = BTreeMap::new();
        let outcome = v.release_escrow(&task("t1"), &payouts).unwrap();
        assert_eq!(outcome.paid_total, MicroCoin::zero());
        assert_eq!(outcome.residual_to_sponsor, coin(100));
    }

    #[test]
    fn release_unknown_task_rejected() {
        let mut v = EscrowVault::new();
        let r = v.release_escrow(&task("t1"), &BTreeMap::new());
        assert_eq!(
            r,
            Err(EscrowError::NotFound {
                task_id: task("t1")
            })
        );
    }

    #[test]
    fn release_after_release_rejected() {
        let mut v = EscrowVault::new();
        v.lock_escrow(task("t1"), agent("alice"), coin(100))
            .unwrap();
        v.release_escrow(&task("t1"), &BTreeMap::new()).unwrap();
        let r = v.release_escrow(&task("t1"), &BTreeMap::new());
        assert_eq!(
            r,
            Err(EscrowError::AlreadyReleased {
                task_id: task("t1")
            })
        );
    }

    #[test]
    fn release_negative_payout_rejected() {
        let mut v = EscrowVault::new();
        v.lock_escrow(task("t1"), agent("alice"), coin(100))
            .unwrap();
        let mut payouts = BTreeMap::new();
        payouts.insert(agent("solver"), MicroCoin::from_micro_units(-1));
        let r = v.release_escrow(&task("t1"), &payouts);
        assert_eq!(r, Err(EscrowError::NegativeAmount));
        // Atomic-reject: still Locked.
        assert_eq!(v.get(&task("t1")).unwrap().status, EscrowStatus::Locked);
    }

    // ── multi-task isolation ────────────────────────────────────────────────

    #[test]
    fn multi_task_independent() {
        let mut v = EscrowVault::new();
        v.lock_escrow(task("t1"), agent("alice"), coin(100))
            .unwrap();
        v.lock_escrow(task("t2"), agent("bob"), coin(50)).unwrap();
        assert_eq!(v.len(), 2);
        let mut p1 = BTreeMap::new();
        p1.insert(agent("solver1"), coin(40));
        v.release_escrow(&task("t1"), &p1).unwrap();
        // t2 must still be locked.
        assert_eq!(v.get(&task("t2")).unwrap().status, EscrowStatus::Locked);
        assert_eq!(v.get(&task("t1")).unwrap().status, EscrowStatus::Released);
    }
}
