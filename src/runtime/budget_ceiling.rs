//! LIVE-FC1 Phase 5 — BUDGET HARD-CEILING admission (the Turing fuel = FC2-HALT).
//!
//! §8 token: `APPROVE-BUDGET-HARD-CEILING-FROM-MANIFEST` (binding §3/§4 contract:
//! `handover/section8/APPROVE_BUDGET_HARD_CEILING_FROM_MANIFEST_2026-06-08.md`).
//!
//! ── WHY (Turing-completeness fuel) ─────────────────────────────────────────
//! A Turing-complete substrate cannot self-halt (the halting problem). An
//! EXTERNAL integer resource bound forces termination: when cumulative
//! tape-derived spend reaches a signed/user-approved ceiling, every further
//! proposal is REJECTED with no head advance — the FC2 terminal HALT
//! (`constitution.md:584/654`) emerges from a per-tick admission refusal at the
//! FC1 predicate/admission arm (`constitution.md:653`). The run stops because it
//! runs out of fuel, not because it decided to.
//!
//! ── WHAT THIS MODULE IS (and is NOT) ───────────────────────────────────────
//! This is the UNPINNED mechanism the §8 packet's PREFERRED form authorizes
//! (A-ALLOW-1/2/3): a deterministic, integer-only ceiling reader + a tape-derived
//! integer SPEND reconstruction + a pure pre-admission CHECK. It edits ZERO
//! genesis-pinned files. It REUSES — and adds NO new — pinned discriminant:
//!   * the ceiling field `BudgetSnapshot.cost_ceiling_microcoin` (`q_state.rs:148`,
//!     PINNED, read-only) — read, never reshaped;
//!   * the admission reject class `RejectionClass::BudgetExceeded`
//!     (`typed_tx.rs:174`, PINNED, already defined, previously unwired) — the
//!     canonical reject-class STRING this module surfaces is derived FROM that
//!     existing variant (`reject_class_label`), inventing no new variant.
//!
//! ── INTEGER-ONLY (G-GUARD-1, hard) ─────────────────────────────────────────
//! Ceiling = `MicroCoin(i64)` micro-units. Spend = integer token accumulation
//! (`u64`), reused VERBATIM from the Phase-2 VPPUT `C_i` reconstruction
//! (`vpput_reconstruction::reconstruct_vpput_from_tape` → per-task `cost_tokens`,
//! which already sums prompt+completion+tool tokens over ACCEPTED L4 **and**
//! L4.E-rejected WorkTx — failed branches counted). There is NO `f64` anywhere
//! on this path. The spend→ceiling comparison is a single integer `>=`.
//!
//! ── FORWARD-ONLY (no regression, hard) ─────────────────────────────────────
//! `cost_ceiling_microcoin == 0` means UNLIMITED — exactly today's behavior. A
//! zero ceiling NEVER produces a budget reject; [`budget_check`] returns
//! [`BudgetVerdict::Unlimited`] and the caller admits precisely as before. Only a
//! POSITIVE ceiling with `spend >= ceiling` produces
//! [`BudgetVerdict::Exceeded`].
//!
//! ── FAIL-CLOSED (G-GUARD-2, hard) ──────────────────────────────────────────
//! A POSITIVE ceiling whose spend sum overflows `u64`, or a manifest that is
//! present-but-unreadable/invalid, fails CLOSED: overflow saturates to
//! `u64::MAX` (≥ any positive ceiling → Exceeded), and an unreadable signed
//! manifest is a hard error the caller must surface, never a silent
//! "no ceiling → proceed" (`feedback_admission_fail_closed_default`). A ceiling
//! that is simply ABSENT (no budget manifest configured) is the UNLIMITED case
//! — that is the documented forward-only default, distinct from a manifest that
//! exists but cannot be parsed.
//!
//! ── DETERMINISTIC + TAPE-RECONSTRUCTABLE ───────────────────────────────────
//! Same tape + same ceiling ⇒ same verdict. The ceiling is read from a
//! signed/user-approved budget manifest FILE (a separate unpinned TOML, NOT
//! `genesis_payload.toml`); the spend is reconstructed from the canonical tape.
//! No RNG, no wall-clock, no sidecar mutable counter.
//!
//! ── CHECKPOINT-RESUME ──────────────────────────────────────────────────────
//! The halt is RESUMABLE. The tape is append-only and the verified head is NOT
//! advanced on a budget reject, so raising the ceiling (a new approved budget
//! manifest) lets the previously-halted proposal admit on the next tick from the
//! last accepted head. [`budget_check`] is a pure function of `(spend, ceiling)`:
//! the same spend against a higher ceiling flips `Exceeded → Within`.
//!
//! Access path:
//! `crate::runtime::agent_scheduler::budget_ceiling::*` (nested as a `#[path]`
//! submodule of the UNPINNED `src/runtime/agent_scheduler.rs`, pin-count 0).

use std::path::Path;

use serde::Deserialize;

use crate::economy::money::MicroCoin;
use crate::ledger::{ImmutableTapeLedger, TapeNode};
use crate::runtime::agent_scheduler::vpput_reconstruction::reconstruct_vpput_from_tape;
use crate::runtime::audit_assertions::LoadedTape;
use crate::state::typed_tx::RejectionClass;

/// The canonical reject-class label a budget breach surfaces, derived FROM the
/// EXISTING pinned [`RejectionClass::BudgetExceeded`] (`typed_tx.rs:174`) — NOT a
/// new discriminant. The kernel membrane stamps this string into the rejection
/// receipt (`reject_class`) so an auditor reconstructs the budget gate from the
/// L4.E rejection record alone (mirroring the arg-taint membrane's
/// `"ArgTaintIntoPrivilegedSink"` label). Reusing the pinned variant's `Debug`
/// name guarantees the label can never drift from the pinned enum without a
/// compile error here.
/// TRACE_MATRIX FC1a-predicates: budget reject receipt marker (reuses pinned class).
pub fn reject_class_label() -> String {
    // Sourced from the pinned variant itself — if the variant is ever renamed or
    // removed, this line fails to compile, so the label is provably the pinned
    // `RejectionClass::BudgetExceeded` and nothing else.
    format!("{:?}", RejectionClass::BudgetExceeded)
}

// ─────────────────────────────────────────────────────────────────────────
// §3 A-ALLOW-1 — signed / user-approved budget-manifest reader
// ─────────────────────────────────────────────────────────────────────────

/// The on-disk signed/user-approved budget manifest (a SEPARATE unpinned TOML,
/// NOT `genesis_payload.toml`). The sole field this leg reads is the integer
/// cost ceiling in MicroCoin micro-units. Forward-only: omit the field (or set
/// it to 0) to declare UNLIMITED — today's behavior.
///
/// INTEGER-ONLY: `cost_ceiling_micro_units` is an `i64` (parsed directly into
/// `MicroCoin`); a TOML float here is a parse error (serde rejects `1.5` for an
/// `i64`), so no `f64` can sneak onto the money path via the manifest.
/// TRACE_MATRIX FC2-economic-tick: signed budget-manifest schema.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct BudgetManifest {
    /// Integer cost ceiling in MicroCoin micro-units. `0` (or absent) ⇒ UNLIMITED
    /// (forward-only no-op). A POSITIVE value arms the hard ceiling. Negative is
    /// rejected at load (a ceiling cannot be negative).
    #[serde(default)]
    pub cost_ceiling_micro_units: i64,
}

/// Error loading the signed budget manifest. Every variant is fail-CLOSED at the
/// call site (the caller must NOT fall back to "no ceiling → proceed" on a
/// present-but-invalid manifest). An ABSENT manifest is NOT an error here — the
/// caller uses [`BudgetManifest::unlimited`] for the documented forward-only
/// default and only loads when a budget file is actually configured.
/// TRACE_MATRIX FC2-economic-tick: budget-manifest load error (fail-closed).
#[derive(Debug)]
pub enum BudgetManifestError {
    /// The manifest file could not be read (I/O error).
    Io(std::io::Error),
    /// The manifest bytes are not valid TOML / not the expected schema.
    Parse(String),
    /// The ceiling value is negative (a ceiling cannot be below zero).
    NegativeCeiling(i64),
}

impl std::fmt::Display for BudgetManifestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BudgetManifestError::Io(e) => write!(f, "budget manifest io error: {e}"),
            BudgetManifestError::Parse(e) => write!(f, "budget manifest parse error: {e}"),
            BudgetManifestError::NegativeCeiling(v) => {
                write!(f, "budget manifest ceiling is negative: {v}")
            }
        }
    }
}

impl std::error::Error for BudgetManifestError {}

impl BudgetManifest {
    /// The forward-only default: NO ceiling configured ⇒ UNLIMITED (today's
    /// behavior). This is what the caller uses when no budget manifest file is
    /// supplied — distinct from a manifest that EXISTS but cannot be parsed
    /// (that is a hard [`BudgetManifestError`], fail-closed).
    /// TRACE_MATRIX FC2-economic-tick: forward-only unlimited default manifest.
    pub fn unlimited() -> Self {
        Self {
            cost_ceiling_micro_units: 0,
        }
    }

    /// Parse a budget manifest from in-memory TOML bytes (the integrity check of
    /// the file's signature is the caller's responsibility — this is the pure,
    /// deterministic parse step). Uses the structured TOML parser (no ad-hoc
    /// string parsing, per `AGENTS.md §12`). Fail-closed: a negative ceiling is
    /// rejected; a non-integer ceiling is a serde parse error (no `f64`).
    /// TRACE_MATRIX FC2-economic-tick: deterministic manifest parse.
    pub fn from_toml_str(bytes: &str) -> Result<Self, BudgetManifestError> {
        let manifest: BudgetManifest =
            toml::from_str(bytes).map_err(|e| BudgetManifestError::Parse(e.to_string()))?;
        if manifest.cost_ceiling_micro_units < 0 {
            return Err(BudgetManifestError::NegativeCeiling(
                manifest.cost_ceiling_micro_units,
            ));
        }
        Ok(manifest)
    }

    /// Read + parse a budget manifest from a FILE path (the signed/user-approved
    /// budget manifest, a separate file — NEVER `genesis_payload.toml`). Reading
    /// the bytes and parsing them is deterministic; fail-closed on I/O or parse
    /// error (the caller must surface, not silently proceed unlimited).
    /// TRACE_MATRIX FC2-economic-tick: signed budget-manifest file reader.
    pub fn from_file(path: &Path) -> Result<Self, BudgetManifestError> {
        let bytes = std::fs::read_to_string(path).map_err(BudgetManifestError::Io)?;
        Self::from_toml_str(&bytes)
    }

    /// The integer cost ceiling as a [`MicroCoin`] (the shape of the PINNED
    /// `BudgetSnapshot.cost_ceiling_microcoin` field this populates at run init).
    /// `0` ⇒ unlimited. INTEGER-ONLY.
    /// TRACE_MATRIX FC2-economic-tick: manifest → BudgetSnapshot.cost_ceiling.
    pub fn ceiling_micro(&self) -> MicroCoin {
        MicroCoin::from_micro_units(self.cost_ceiling_micro_units)
    }
}

// ─────────────────────────────────────────────────────────────────────────
// §3 A-ALLOW-2 — tape-derived integer SPEND (reuses the Phase-2 VPPUT C_i)
// ─────────────────────────────────────────────────────────────────────────

/// Reconstruct the cumulative integer token SPEND from a canonical
/// [`LoadedTape`] by REUSING the Phase-2 VPPUT `C_i` cost reconstruction. The
/// total spend is `Σ task.cost_tokens` over every task row produced by
/// [`reconstruct_vpput_from_tape`] — each `cost_tokens` already sums
/// prompt+completion+tool tokens across ACCEPTED L4 AND L4.E-rejected WorkTx
/// (failed branches counted, tool stdout included). This is the SOLE spend
/// authority: there is no second, independently-summed ledger (no
/// second-source-of-truth, `constitution.md:61`).
///
/// FAIL-CLOSED: an overflowing sum saturates to `u64::MAX`, which is `>=` any
/// positive ceiling ⇒ a saturated spend HALTS rather than silently wrapping to a
/// small number that would pass the gate.
///
/// Pure read; observe-only; deterministic. Same tape ⇒ same spend.
/// TRACE_MATRIX FC1-N34: tape-derived integer spend (reuses VPPUT C_i).
pub fn loaded_tape_spend_tokens(tape: &LoadedTape) -> u64 {
    // `held_out_task_ids` is irrelevant to the spend sum (it only affects the
    // held-out AGGREGATE, not per-task cost), so we pass an empty split.
    let recon = reconstruct_vpput_from_tape(tape, &[]);
    recon
        .tasks
        .iter()
        .fold(0u64, |acc, t| acc.saturating_add(t.cost_tokens))
}

/// Reconstruct the cumulative integer token SPEND from the LIVE in-loop kernel
/// tape (`ImmutableTapeLedger`). The live FC1 loop holds an in-memory tape — not
/// the heavyweight on-disk [`LoadedTape`] — so the kernel-side membrane sums the
/// integer `token_count` recorded on EVERY node (`dump_all_nodes`), which is the
/// SAME `C_i` quantity: every externalized attempt (accepted StateAccepted AND
/// failed-branch AgentProposal) contributes its token cost. Nodes that carry no
/// recorded cost (`token_count == None`) contribute `0`, exactly like a CAS-miss
/// in the canonical `C_i` reconstruction.
///
/// FAIL-CLOSED: a saturating sum (overflow ⇒ `u64::MAX` ⇒ `>=` any positive
/// ceiling ⇒ HALT).
///
/// Pure read; observe-only; deterministic. Same tape ⇒ same spend.
/// TRACE_MATRIX FC1-N34 + FC1a-tape_t: live-loop tape-derived integer spend.
pub fn live_tape_spend_tokens<L: ImmutableTapeLedger>(tape: &L) -> u64 {
    tape.dump_all_nodes().iter().fold(0u64, |acc, (_, node)| {
        acc.saturating_add(node.token_count.unwrap_or(0) as u64)
    })
}

/// Sum the integer `token_count` over an explicit node slice — the pure kernel
/// of [`live_tape_spend_tokens`], exposed so a gate can exercise the spend math
/// on a constructed node set without standing up a full ledger. FAIL-CLOSED
/// saturating sum. INTEGER-ONLY.
/// TRACE_MATRIX FC1a-tape_t: pure node-slice spend reconstruction.
pub fn node_slice_spend_tokens(nodes: &[TapeNode]) -> u64 {
    nodes.iter().fold(0u64, |acc, node| {
        acc.saturating_add(node.token_count.unwrap_or(0) as u64)
    })
}

// ─────────────────────────────────────────────────────────────────────────
// §3 A-ALLOW-3 — the pure pre-admission budget CHECK (the FC2-HALT seam)
// ─────────────────────────────────────────────────────────────────────────

/// The verdict of the pre-admission budget check. A pure, deterministic function
/// of `(spend, ceiling)` only — no I/O, no clock, no RNG.
/// TRACE_MATRIX FC1a-predicates + FC2-HALT: budget admission verdict.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BudgetVerdict {
    /// `cost_ceiling_microcoin == 0` ⇒ UNLIMITED (forward-only no-op). The caller
    /// admits exactly as before; NO budget reject is ever produced.
    Unlimited,
    /// A POSITIVE ceiling AND `spend < ceiling`: the proposal is WITHIN budget;
    /// the caller proceeds to its ordinary admission (taint + predicates).
    Within {
        spend_micro: i64,
        ceiling_micro: i64,
    },
    /// A POSITIVE ceiling AND `spend >= ceiling`: the FC2-HALT. The caller MUST
    /// refuse the advance and route to the existing non-advancing rejection path
    /// with [`reject_class_label`] (`RejectionClass::BudgetExceeded`). RESUMABLE:
    /// re-checking the same `spend` against a raised ceiling flips this to
    /// `Within`.
    Exceeded {
        spend_micro: i64,
        ceiling_micro: i64,
    },
}

impl BudgetVerdict {
    /// True iff this verdict is the FC2-HALT (a positive-ceiling breach). The
    /// caller branches on this to decide reject-vs-proceed.
    /// TRACE_MATRIX FC2-economic-tick: FC2-HALT branch predicate for the caller.
    pub fn is_exceeded(&self) -> bool {
        matches!(self, BudgetVerdict::Exceeded { .. })
    }
}

/// THE pre-admission budget check (A-ALLOW-3). Pure, deterministic, integer-only.
///
/// `spend_tokens` is the tape-derived integer token spend (from
/// [`loaded_tape_spend_tokens`] / [`live_tape_spend_tokens`]); `ceiling` is the
/// integer MicroCoin ceiling read from the signed manifest into the PINNED
/// `BudgetSnapshot.cost_ceiling_microcoin` field. One MicroCoin micro-unit is
/// charged per spent token (the canonical 1-token ↔ 1-micro accounting that
/// keeps both quantities on the same integer axis with no `f64` conversion).
///
/// Decision (FORWARD-ONLY + FAIL-CLOSED):
///   * `ceiling == 0`            ⇒ [`BudgetVerdict::Unlimited`] (today's behavior).
///   * `ceiling > 0`, `spend  < ceiling` ⇒ [`BudgetVerdict::Within`].
///   * `ceiling > 0`, `spend >= ceiling` ⇒ [`BudgetVerdict::Exceeded`] (FC2-HALT).
///   * `ceiling < 0` is treated as UNLIMITED here defensively — the manifest
///     reader already rejects a negative ceiling at load
///     ([`BudgetManifest::from_toml_str`]), so a negative value never reaches a
///     live run; this arm exists only so the pure function is total.
///
/// The spend is compared in MicroCoin micro-units: `spend_micro = spend_tokens`
/// (1 micro per token), saturated into `i64` (fail-closed: an `i64`-overflowing
/// spend saturates to `i64::MAX`, which is `>=` any positive ceiling ⇒ HALT).
/// TRACE_MATRIX FC1a-predicates + FC2-HALT: pure budget admission decision.
pub fn budget_check(spend_tokens: u64, ceiling: MicroCoin) -> BudgetVerdict {
    let ceiling_micro = ceiling.micro_units();
    // Forward-only: a non-positive ceiling is UNLIMITED (no reject ever).
    if ceiling_micro <= 0 {
        return BudgetVerdict::Unlimited;
    }
    // 1 token ↔ 1 MicroCoin micro-unit, saturated into i64 (fail-closed on a
    // spend that would overflow i64 → i64::MAX → >= any positive ceiling → HALT).
    let spend_micro: i64 = spend_tokens.try_into().unwrap_or(i64::MAX);
    if spend_micro >= ceiling_micro {
        BudgetVerdict::Exceeded {
            spend_micro,
            ceiling_micro,
        }
    } else {
        BudgetVerdict::Within {
            spend_micro,
            ceiling_micro,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── reject-class label is the PINNED variant, not a new discriminant ──
    #[test]
    fn reject_label_is_the_pinned_budget_exceeded_variant() {
        // Reusing the existing RejectionClass::BudgetExceeded (typed_tx.rs:174).
        assert_eq!(reject_class_label(), "BudgetExceeded");
        assert_eq!(
            reject_class_label(),
            format!("{:?}", RejectionClass::BudgetExceeded)
        );
    }

    // ── FORWARD-ONLY: zero ceiling = unlimited, never rejects ──
    #[test]
    fn zero_ceiling_is_unlimited_no_reject() {
        // Even a huge spend against a zero ceiling is UNLIMITED (today's behavior).
        assert_eq!(budget_check(0, MicroCoin::zero()), BudgetVerdict::Unlimited);
        assert_eq!(
            budget_check(u64::MAX, MicroCoin::zero()),
            BudgetVerdict::Unlimited
        );
        assert!(!budget_check(u64::MAX, MicroCoin::zero()).is_exceeded());
    }

    // ── POSITIVE ceiling: within stays within, at/over halts ──
    #[test]
    fn positive_ceiling_halts_at_or_over_spend() {
        let ceiling = MicroCoin::from_micro_units(100);
        // spend strictly below ceiling → Within (proceed).
        assert_eq!(
            budget_check(99, ceiling),
            BudgetVerdict::Within {
                spend_micro: 99,
                ceiling_micro: 100
            }
        );
        // spend EXACTLY at ceiling → Exceeded (>= is the halt boundary; the
        // ceiling is a HARD cap, not a soft one).
        assert!(budget_check(100, ceiling).is_exceeded());
        // spend over ceiling → Exceeded.
        assert!(budget_check(250, ceiling).is_exceeded());
    }

    // ── CHECKPOINT-RESUME: raising the ceiling flips Exceeded → Within ──
    #[test]
    fn raising_ceiling_resumes_previously_halted_spend() {
        let spend = 100u64;
        let low = MicroCoin::from_micro_units(100);
        let high = MicroCoin::from_micro_units(1_000);
        // Same spend halts at the low ceiling …
        assert!(budget_check(spend, low).is_exceeded());
        // … and admits (Within) once the approved ceiling is raised. The verdict
        // is a pure function of (spend, ceiling): no head was advanced on the
        // halt, so the previously-halted proposal admits from the same head.
        assert_eq!(
            budget_check(spend, high),
            BudgetVerdict::Within {
                spend_micro: 100,
                ceiling_micro: 1_000
            }
        );
    }

    // ── FAIL-CLOSED: an i64-overflowing spend saturates to a halt ──
    #[test]
    fn overflowing_spend_fails_closed_to_halt() {
        // u64 spend beyond i64::MAX saturates to i64::MAX, which is >= any
        // positive ceiling → HALT (never wraps to a small passing number).
        let ceiling = MicroCoin::from_micro_units(100);
        assert!(budget_check(u64::MAX, ceiling).is_exceeded());
    }

    // ── node-slice / live-tape spend is the integer token sum (failed branches included) ──
    #[test]
    fn node_slice_spend_sums_token_count_over_all_nodes() {
        use crate::ledger::NodeKind;
        let node = |tc: Option<usize>, verified: bool| TapeNode {
            id: "n".into(),
            hash: "h".into(),
            kind: if verified {
                NodeKind::StateAccepted
            } else {
                NodeKind::AgentProposal
            },
            verified,
            parent: None,
            scope: None,
            attempt_ordinal: None,
            reject_class: None,
            token_count: tc,
            payload: serde_json::json!({}),
            created_at_unix_ms: 0,
        };
        // Accepted node (10) + failed-branch node (7) + a None-cost node (0) = 17.
        // Failed branches MUST count (mirrors VPPUT C_i).
        let nodes = vec![
            node(Some(10), true),
            node(Some(7), false),
            node(None, false),
        ];
        assert_eq!(node_slice_spend_tokens(&nodes), 17);
        // Dropping the failed branch lowers the spend (non-vacuous: the failed
        // branch genuinely contributes).
        let dropped = vec![node(Some(10), true), node(None, false)];
        assert_eq!(node_slice_spend_tokens(&dropped), 10);
    }

    // ── manifest reader: integer-only, forward-only default, fail-closed ──
    #[test]
    fn manifest_parses_integer_ceiling() {
        let m = BudgetManifest::from_toml_str("cost_ceiling_micro_units = 5000").unwrap();
        assert_eq!(m.cost_ceiling_micro_units, 5000);
        assert_eq!(m.ceiling_micro(), MicroCoin::from_micro_units(5000));
    }

    #[test]
    fn manifest_absent_field_is_unlimited_forward_only() {
        // An empty manifest (no ceiling field) defaults to 0 = UNLIMITED.
        let m = BudgetManifest::from_toml_str("").unwrap();
        assert_eq!(m.cost_ceiling_micro_units, 0);
        assert_eq!(m.ceiling_micro(), MicroCoin::zero());
        assert_eq!(BudgetManifest::unlimited(), m);
    }

    #[test]
    fn manifest_rejects_negative_ceiling_fail_closed() {
        let err = BudgetManifest::from_toml_str("cost_ceiling_micro_units = -1").unwrap_err();
        assert!(matches!(err, BudgetManifestError::NegativeCeiling(-1)));
    }

    #[test]
    fn manifest_rejects_float_ceiling_no_f64() {
        // A TOML float for an i64 field is a parse error → no f64 on the money path.
        let err = BudgetManifest::from_toml_str("cost_ceiling_micro_units = 1.5").unwrap_err();
        assert!(matches!(err, BudgetManifestError::Parse(_)));
    }

    #[test]
    fn manifest_rejects_garbage_fail_closed() {
        let err = BudgetManifest::from_toml_str("this is not toml = = =").unwrap_err();
        assert!(matches!(err, BudgetManifestError::Parse(_)));
    }
}
