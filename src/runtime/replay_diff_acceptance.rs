//! LIVE-FC1 Phase 6 — from-genesis replay-diff acceptance helper.
//!
//! **Why this exists** (LIVE-FC1 Phase 6, Deliverable B): a swarm-run
//! acceptance check needs a single boolean witness that the chaintape, when
//! replayed FROM GENESIS, reconstructs the SAME `state_root`/`ledger_root` that
//! the tape recorded. This is the "replay-diff = 0" acceptance: a real run is
//! accepted only if its recorded roots survive an independent from-genesis
//! reconstruction.
//!
//! **Reuse, don't reimplement** (binding): the actual replay verifier is the
//! trust-root-PINNED [`crate::runtime::verify::verify_chaintape`] (TB-6 Atom 4).
//! This helper does NOT reimplement the verifier — it CALLS it and reduces its
//! [`ReplayReport`](crate::runtime::verify::ReplayReport) to the from-genesis
//! root-equality acceptance boolean. `verify_chaintape` re-opens the runtime
//! repo + CAS, resolves the initial `QState` (from `initial_q_state.json` if
//! present, else `QState::genesis()`), replays the L4 chain entry-by-entry
//! through `replay_full_transition_*`, and reconstructs the final
//! `state_root`/`ledger_root`. `ledger_root_verified` is exactly the
//! "every entry's `parent_ledger_root` chains to the prior `resulting_ledger_root`
//! and the recorded roots match the reconstructed fold" indicator.
//!
//! **Observe-only / no-mutation**: this helper takes a `&LoadedTape` (or raw
//! paths) by shared reference, performs a read-only replay, mutates no
//! `QState`/`EconomicState`, advances no head, and changes no sequencer
//! admission or L4/L4.E predicate. There is NO `std::fs::write`.
//!
//! TRACE_MATRIX FC3-N1 (replay verifier) + FC1-N34 (from-tape acceptance):
//! nested as a `#[path]` submodule under the UNPINNED `agent_scheduler.rs`
//! (genesis-pinned-count 0), keeping `runtime/mod.rs` byte-identical — ZERO
//! pinned files change.

use std::path::Path;

use crate::runtime::audit_assertions::LoadedTape;
use crate::runtime::verify::{verify_chaintape, ReplayReport, VerifyError, VerifyOptions};

/// TRACE_MATRIX FC3-N1 + FC1-N34: run the PINNED `verify_chaintape` from-genesis
/// replay over an explicit `runtime_repo` + `cas_dir` and return whether the
/// reconstructed `state_root`/`ledger_root` equal the recorded roots.
///
/// Acceptance = the three from-genesis root-reconstruction indicators all pass:
/// - `ledger_root_verified` — recorded `resulting_ledger_root` fold matches,
/// - `state_reconstructed` — replay produced a `QState` (no root divergence),
/// - `economic_state_reconstructed` — the economic projection replayed clean.
///
/// These three are the from-genesis "roots match" witnesses; the signature /
/// CAS-retrievability indicators are intentionally NOT folded in here so this
/// helper is a precise ROOT-EQUALITY acceptance (a separate concern from
/// signature provenance). Returns `Err` only for I/O / manifest issues that
/// block replay from starting (propagated from `verify_chaintape`), never for a
/// mere root mismatch (which is `Ok(false)`).
pub fn replay_roots_match_genesis_at_paths(
    runtime_repo: &Path,
    cas_dir: &Path,
) -> Result<bool, VerifyError> {
    let report = verify_chaintape(runtime_repo, cas_dir, &VerifyOptions::default())?;
    Ok(roots_match(&report))
}

/// TRACE_MATRIX FC3-N1 + FC1-N34: the `&LoadedTape` ergonomic entry-point for
/// the swarm acceptance. Replays FROM GENESIS via the PINNED `verify_chaintape`
/// over the tape's `runtime_repo` + `cas_dir` and asserts the reconstructed
/// `state_root`/`ledger_root` equal the recorded roots.
///
/// Reuses the existing verifier path; does NOT reimplement replay. Observe-only:
/// `tape` is borrowed `&`; nothing is mutated. Returns `false` on any I/O /
/// manifest error so a broken or unreadable tape is conservatively NOT
/// accepted (fail-closed acceptance). Callers needing to distinguish a root
/// mismatch from an I/O failure should use
/// [`replay_roots_match_genesis_at_paths`] and inspect the `Err`.
pub fn replay_roots_match_genesis(tape: &LoadedTape) -> bool {
    replay_roots_match_genesis_at_paths(&tape.runtime_repo, &tape.cas_dir).unwrap_or(false)
}

/// TRACE_MATRIX FC3-N1 + FC1-N34: reduce a full `ReplayReport` to the
/// from-genesis ROOT-EQUALITY acceptance boolean. Kept as a pure, testable
/// projection so the acceptance rule is a single auditable expression over the
/// PINNED verifier's output.
pub fn roots_match(report: &ReplayReport) -> bool {
    report.ledger_root_verified && report.state_reconstructed && report.economic_state_reconstructed
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::verify::{ReplayReport, ReplayReportDetail};

    fn report_with(
        ledger_root_verified: bool,
        state_reconstructed: bool,
        economic_state_reconstructed: bool,
    ) -> ReplayReport {
        ReplayReport {
            l4_entries: 0,
            l4e_entries: 0,
            ledger_root_verified,
            system_signatures_verified: true,
            state_reconstructed,
            economic_state_reconstructed,
            cas_payloads_retrievable: true,
            agent_signatures_verified: true,
            proposal_telemetry_cas_retrievable: true,
            run_id: "test-run".to_string(),
            epoch: 0,
            detail: ReplayReportDetail {
                final_state_root_hex: None,
                final_ledger_root_hex: None,
                head_commit_oid_hex: None,
                l4e_last_hash_hex: String::new(),
                replay_failure: None,
                initial_q_state_loaded_from_disk: false,
            },
        }
    }

    /// All three from-genesis root indicators pass ⇒ accept.
    #[test]
    fn roots_match_when_all_three_pass() {
        assert!(roots_match(&report_with(true, true, true)));
    }

    /// A ledger-root divergence (recorded ≠ reconstructed) ⇒ reject. This is the
    /// non-vacuous arm: the acceptance must FAIL when the from-genesis fold
    /// disagrees with the recorded roots.
    #[test]
    fn roots_mismatch_when_ledger_root_diverges() {
        assert!(!roots_match(&report_with(false, true, true)));
    }

    /// State-reconstruction failure (root divergence mid-replay) ⇒ reject.
    #[test]
    fn roots_mismatch_when_state_not_reconstructed() {
        assert!(!roots_match(&report_with(true, false, true)));
    }

    /// Economic-state reconstruction failure ⇒ reject.
    #[test]
    fn roots_mismatch_when_econ_not_reconstructed() {
        assert!(!roots_match(&report_with(true, true, false)));
    }

    /// The acceptance is a strict AND: signature/CAS indicators are NOT part of
    /// the root-equality acceptance, so a report that fails ONLY a non-root
    /// indicator still accepts on roots (the helper's scope is root equality).
    #[test]
    fn signature_indicator_is_out_of_root_scope() {
        let mut r = report_with(true, true, true);
        r.system_signatures_verified = false;
        r.agent_signatures_verified = false;
        assert!(
            roots_match(&r),
            "roots_match is a root-equality predicate, not a signature predicate"
        );
    }
}
