//! TB-7R Deliverable B — ChainTape-mode gate for run conditions.
//!
//! Per architect verdict 2026-05-01 §5.6 / B3, when the operator declares
//! ChainTape mode by setting `TURINGOS_CHAINTAPE_PATH`, every code path
//! that produces real LLM proposals MUST route through
//! `bus.submit_typed_tx`. Paths that bypass authoritative routing
//! (e.g. `CONDITION=oneshot` which calls `run_oneshot` instead of the
//! swarm loop) MUST fail-closed rather than silently emitting evidence
//! that cannot be reconstructed from ChainTape + CAS.
//!
//! TRACE_MATRIX FC1 (Runtime State Transition): the gate enforces that
//! Q_t -> rtool -> Agent -> proposal -> predicates -> wtool -> Q_{t+1}
//! is the only authoritative path under ChainTape mode.
//!
//! `FC-trace: Art.I.1 + Art.III.4 + WP-§5.L3/L4`.

/// Conditions known to NOT route through `bus.submit_typed_tx`. In
/// ChainTape mode these MUST fail-closed.
const CHAINTAPE_UNSUPPORTED_CONDITIONS: &[&str] = &["oneshot"];

/// TRACE_MATRIX FC1-N6 (predicate / wtool gate): result of the
/// ChainTape mode-compatibility check.
///
/// `Ok(())` means the condition is either compatible with ChainTape
/// mode OR ChainTape mode is not active (legacy mode).
///
/// `Err(reason)` means ChainTape mode is active AND the condition is
/// known to bypass authoritative routing. Caller MUST exit non-zero.
pub fn chaintape_supports_condition(condition: &str) -> Result<(), String> {
    if std::env::var("TURINGOS_CHAINTAPE_PATH").is_err() {
        return Ok(());
    }
    if CHAINTAPE_UNSUPPORTED_CONDITIONS.contains(&condition) {
        return Err(format!(
            "CONDITION={condition} is not wired through bus.submit_typed_tx; \
             ChainTape mode requires a swarm condition (n1, n3, n5, ...). \
             TB-7R Deliverable B / verdict B3."
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn oneshot_fails_closed_in_chaintape_mode() {
        let _guard = ENV_LOCK.lock().expect("env lock");
        let prev = std::env::var("TURINGOS_CHAINTAPE_PATH").ok();
        std::env::set_var("TURINGOS_CHAINTAPE_PATH", "/tmp/ignored");

        let res = chaintape_supports_condition("oneshot");

        match prev {
            Some(v) => std::env::set_var("TURINGOS_CHAINTAPE_PATH", v),
            None => std::env::remove_var("TURINGOS_CHAINTAPE_PATH"),
        }
        let err = res.expect_err("oneshot in ChainTape mode must fail-closed");
        assert!(err.contains("oneshot"));
        assert!(err.contains("TB-7R"));
    }

    #[test]
    fn oneshot_passes_in_legacy_mode() {
        let _guard = ENV_LOCK.lock().expect("env lock");
        let prev = std::env::var("TURINGOS_CHAINTAPE_PATH").ok();
        std::env::remove_var("TURINGOS_CHAINTAPE_PATH");

        let res = chaintape_supports_condition("oneshot");

        if let Some(v) = prev {
            std::env::set_var("TURINGOS_CHAINTAPE_PATH", v);
        }
        assert!(res.is_ok(), "oneshot in legacy mode must be allowed");
    }

    #[test]
    fn swarm_condition_passes_in_chaintape_mode() {
        let _guard = ENV_LOCK.lock().expect("env lock");
        let prev = std::env::var("TURINGOS_CHAINTAPE_PATH").ok();
        std::env::set_var("TURINGOS_CHAINTAPE_PATH", "/tmp/ignored");

        let res_n1 = chaintape_supports_condition("n1");
        let res_n5 = chaintape_supports_condition("n5");

        match prev {
            Some(v) => std::env::set_var("TURINGOS_CHAINTAPE_PATH", v),
            None => std::env::remove_var("TURINGOS_CHAINTAPE_PATH"),
        }
        assert!(res_n1.is_ok(), "n1 must be allowed in ChainTape mode");
        assert!(res_n5.is_ok(), "n5 must be allowed in ChainTape mode");
    }

    #[test]
    fn unknown_condition_passes_when_not_in_unsupported_list() {
        let _guard = ENV_LOCK.lock().expect("env lock");
        let prev = std::env::var("TURINGOS_CHAINTAPE_PATH").ok();
        std::env::set_var("TURINGOS_CHAINTAPE_PATH", "/tmp/ignored");

        let res = chaintape_supports_condition("future_mode_xyz");

        match prev {
            Some(v) => std::env::set_var("TURINGOS_CHAINTAPE_PATH", v),
            None => std::env::remove_var("TURINGOS_CHAINTAPE_PATH"),
        }
        assert!(
            res.is_ok(),
            "unknown conditions are allowed by default; gate adds them \
             explicitly to CHAINTAPE_UNSUPPORTED_CONDITIONS as their \
             routing is audited"
        );
    }
}
