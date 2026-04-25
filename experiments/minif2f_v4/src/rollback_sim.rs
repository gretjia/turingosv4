// PPUT-CCL Phase B B7-extra — synthetic rollback simulation.
//
// Constitutional anchor (TRACE_MATRIX_v1 § 7.2): the
// `--simulate-rollback-at-tx-50` toggle (PREREG § 5.5) is realized as
// "every proposal from tx 50 onward is vetoed", which is constitutionally
// the **FC1-E18** edge (∏p=0 → Q_t preservation) repeated for tx
// 50..max_transactions. The run then exhausts naturally and exits via
// the existing **FC2-N22 HALT** with `HaltReason::MaxTxExhausted` — no
// new HaltReason variant is introduced and no new constitutional surface
// is created.
//
// For efficiency, the swarm loop short-circuits at tx == threshold
// instead of running ~150 guaranteed-vetoed iterations. The short-circuit
// is observably equivalent: identical exit state, identical cost
// accumulator (no extra LLM calls would have happened in vetoed tx),
// identical wall-clock close. The only observable difference is
// `tx_count` stamped at threshold rather than `max_transactions` — a
// useful diagnostic signal that distinguishes a calibration-treatment
// run from a real exhaustion.
//
// Threat model: the threshold is fixed at 50 per PREREG § 5.5 frozen
// spec. The env var `SIMULATE_ROLLBACK_AT_TX_50` is a binary toggle
// (`"1"` to enable). The threshold is intentionally not exposed as a
// runtime parameter — pre-registration discipline (C-070) requires that
// what we calibrate is exactly what is committed in genesis_payload.toml.

/// PREREG § 5.5: the synthetic rollback fires at this transaction index
/// in the swarm loop. Frozen — must match the value committed in the
/// pre-registration hash chain.
pub const ROLLBACK_TX_THRESHOLD: u64 = 50;

/// Env var name read by the evaluator. `"1"` enables the toggle; any
/// other value (or absence) is "off".
pub const ROLLBACK_ENV_VAR: &str = "SIMULATE_ROLLBACK_AT_TX_50";

/// True iff the calibration treatment toggle is enabled in the current
/// process environment.
pub fn rollback_simulation_enabled() -> bool {
    std::env::var(ROLLBACK_ENV_VAR)
        .ok()
        .as_deref()
        == Some("1")
}

/// True iff the swarm loop should short-circuit at this `tx` index. The
/// short-circuit is constitutionally equivalent to "synthetic ∏p=0 from
/// here, naturally exhaust at `max_transactions`" — see module header.
///
/// `enabled` is a parameter (not read from env) so unit tests can drive
/// the predicate without process-global state.
pub fn should_simulate_rollback(tx: u64, enabled: bool) -> bool {
    enabled && tx == ROLLBACK_TX_THRESHOLD
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fires_at_threshold_when_enabled() {
        assert!(should_simulate_rollback(50, true));
    }

    #[test]
    fn does_not_fire_before_threshold() {
        for tx in [0_u64, 1, 25, 49] {
            assert!(!should_simulate_rollback(tx, true), "tx={tx}");
        }
    }

    #[test]
    fn does_not_fire_after_threshold() {
        // Constitutional reading: at tx > 50, the synthetic ∏p has already
        // begun returning Reject; the loop continues but accumulates no
        // progress. Short-circuit fires exactly once at tx == threshold,
        // not on every tx after.
        for tx in [51_u64, 60, 100, 199] {
            assert!(!should_simulate_rollback(tx, true), "tx={tx}");
        }
    }

    #[test]
    fn never_fires_when_disabled() {
        for tx in [0_u64, 49, 50, 51, 199] {
            assert!(!should_simulate_rollback(tx, false), "tx={tx}");
        }
    }

    #[test]
    fn threshold_constant_matches_prereg() {
        // PREREG § 5.5 freezes the threshold at 50. If this assertion ever
        // fails, the codebase has drifted from the pre-registration hash
        // chain — recompute Trust Root and dual-audit before continuing.
        assert_eq!(ROLLBACK_TX_THRESHOLD, 50);
    }

    #[test]
    fn env_var_name_matches_prereg() {
        // PREREG § 5.5 names the toggle `--simulate-rollback-at-tx-50`;
        // the env-var equivalent (the v4 evaluator does not use clap)
        // mirrors that name uppercased + underscored.
        assert_eq!(ROLLBACK_ENV_VAR, "SIMULATE_ROLLBACK_AT_TX_50");
    }
}
