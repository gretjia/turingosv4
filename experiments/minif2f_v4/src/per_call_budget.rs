//! TB-18 Atom A — Per-LLM-call budget enforcement (architect 2026-05-05
//! ruling §3 Atom A + OBS_M0_DEEPSEEK_DRIFT §5.1 + FR-18.2 + FR-18.3).
//!
//! ## Why this module exists
//!
//! `OBS_M0_DEEPSEEK_DRIFT` (filed 2026-05-05 from M0 r1 batch) documented a
//! silent-hang failure mode: DeepSeek-chat occasionally returns 200 OK with
//! 14-output-token / 37-character trivial responses. The evaluator's outer
//! loop receives `Ok(GenerateResponse)` (HTTP success), treats the empty
//! "tactic" as a proposal, fails to converge, and immediately requests the
//! next response. Because the tx-counter only advances on accepted proposals,
//! the run never reaches `MaxTxExhausted`; the only escape is the external
//! `timeout 600` wrapper. This silently burns hours of LLM budget per task.
//!
//! Architect ruling §3 Atom A scope:
//!
//! ```text
//! drive_task(chain, task_spec)
//! per-LLM-call budget
//! RunOutcome::DegradedLLM
//! EvidenceCapsule on degradation
//! ```
//!
//! Architect §2.5 (evidence-not-backdoor):
//!
//! > `RunOutcome::DegradedLLM` 必须伴随 EvidenceCapsule + TerminalSummary +
//! > budget counters; no payout; no fake accepted. 否则它会变成"LLM 不稳定，
//! > 所以跳过记录"的后门.
//!
//! ## What this module enforces
//!
//! Three orthogonal caps per `LLMCallBudgetTracker`:
//!
//! 1. **Per-call wall-clock** (`per_call_wallclock_seconds`): individual
//!    `client.generate()` round-trip MUST return within this duration.
//!    Today this is enforced by `ResilientLLMClient`'s timeout (configured
//!    via constructor); this struct documents the contract for atom B's
//!    callers and provides a configurable surface.
//!
//! 2. **Per-call output-token-floor** (`token_floor_threshold` +
//!    `consecutive_trivial_response_cap`): when a response's
//!    `completion_tokens < token_floor_threshold`, increment a
//!    consecutive-trivial counter; when the counter reaches
//!    `consecutive_trivial_response_cap`, halt with `DegradedLLM`. A
//!    substantive response (completion_tokens ≥ floor) RESETS the counter.
//!
//! 3. **Aggregate per-run wall-clock cap** (`aggregate_per_run_wallclock_seconds`):
//!    cumulative time across all LLM calls in a single run MUST stay below
//!    this cap. Internal enforcement via this struct converts the architect
//!    §B.9 M0 spec's external `timeout 600` from a primary cap into a safety
//!    net.
//!
//! ## Safety net vs primary control
//!
//! Per FR-18.2 verbatim: "Per-LLM-call budget is enforced and emits
//! RunOutcome::DegradedLLM, not silent timeout." The external `timeout`
//! command remains as a safety net (in case this module's enforcement
//! misfires), but the EXPECTED halt path on drift is internal: the tracker
//! flags the trivial-response pattern and the caller sets
//! `terminal_exhaustion_reason = ExhaustionReason::DegradedLLM`, which then
//! flows through the canonical Atom E propagation pipeline to
//! `EvidenceCapsule.outcome` + `TerminalSummary.run_outcome`.

use std::time::{Duration, Instant};

/// TB-18 Atom A: Per-LLM-call budget envelope. Frozen at run start; passed
/// through the call chain into `LLMCallBudgetTracker::on_response`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PerCallBudget {
    /// Per-call wall-clock cap (seconds). Default 60s. Configurable via env
    /// `TURINGOS_PER_CALL_WALLCLOCK_S`.
    pub per_call_wallclock_seconds: u64,
    /// Token-floor threshold. Responses with `completion_tokens` strictly
    /// below this value count as "trivial". Default 30 (the M0 r1 P02
    /// drift had 14-token responses; 30 is comfortably above noise floor
    /// and below normal substantive responses which routinely run 100+).
    /// Configurable via env `TURINGOS_PER_CALL_TOKEN_FLOOR`.
    pub token_floor_threshold: u32,
    /// Consecutive-trivial-response cap. After this many consecutive
    /// trivial responses, halt with DegradedLLM. Default 10 (allows brief
    /// drift episodes to recover; M0 r1 saw 30+ consecutive in P02 hang).
    /// Configurable via env `TURINGOS_PER_CALL_CONSECUTIVE_TRIVIAL_CAP`.
    pub consecutive_trivial_response_cap: u32,
    /// Aggregate per-run wall-clock cap (seconds). Default 600 (architect
    /// §B.9 M0 spec). Configurable via env `TURINGOS_AGGREGATE_RUN_WALLCLOCK_S`.
    /// Internal enforcement converts external `timeout 600` from primary
    /// cap to safety net.
    pub aggregate_per_run_wallclock_seconds: u64,
}

impl Default for PerCallBudget {
    /// Architect-recommended defaults (FR-18.2 + OBS_M0 §5.1):
    /// `(60, 30, 10, 600)`.
    fn default() -> Self {
        Self {
            per_call_wallclock_seconds: 60,
            token_floor_threshold: 30,
            consecutive_trivial_response_cap: 10,
            aggregate_per_run_wallclock_seconds: 600,
        }
    }
}

impl PerCallBudget {
    /// Read budget from env vars; missing vars use Default values.
    /// Per `feedback_no_workarounds_strict_constitution`: any malformed
    /// value is FAIL-CLOSED (returns Err) — silent fallback to default
    /// would mask operator misconfiguration on a Class 3 budget surface.
    pub fn from_env() -> Result<Self, String> {
        let mut budget = Self::default();
        if let Ok(raw) = std::env::var("TURINGOS_PER_CALL_WALLCLOCK_S") {
            budget.per_call_wallclock_seconds = raw
                .parse::<u64>()
                .map_err(|e| format!("TURINGOS_PER_CALL_WALLCLOCK_S parse: {e}"))?;
        }
        if let Ok(raw) = std::env::var("TURINGOS_PER_CALL_TOKEN_FLOOR") {
            budget.token_floor_threshold = raw
                .parse::<u32>()
                .map_err(|e| format!("TURINGOS_PER_CALL_TOKEN_FLOOR parse: {e}"))?;
        }
        if let Ok(raw) = std::env::var("TURINGOS_PER_CALL_CONSECUTIVE_TRIVIAL_CAP") {
            budget.consecutive_trivial_response_cap = raw.parse::<u32>().map_err(|e| {
                format!("TURINGOS_PER_CALL_CONSECUTIVE_TRIVIAL_CAP parse: {e}")
            })?;
        }
        if let Ok(raw) = std::env::var("TURINGOS_AGGREGATE_RUN_WALLCLOCK_S") {
            budget.aggregate_per_run_wallclock_seconds = raw
                .parse::<u64>()
                .map_err(|e| format!("TURINGOS_AGGREGATE_RUN_WALLCLOCK_S parse: {e}"))?;
        }
        Ok(budget)
    }
}

/// TB-18 Atom A: Verdict returned by `LLMCallBudgetTracker::on_response`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BudgetVerdict {
    /// Continue: response is substantive (or within trivial-tolerance).
    Continue,
    /// Halt with DegradedLLM: consecutive trivial-response cap reached.
    /// Caller MUST set `terminal_exhaustion_reason = DegradedLLM` and
    /// break out of the LLM loop to the EvidenceCapsule emission cleanup.
    HaltDegradedLLM { consecutive_trivial: u32 },
    /// Halt with WallClockCap: aggregate per-run wall-clock cap reached.
    /// Caller MUST set `terminal_exhaustion_reason = WallClockCap` and
    /// break out of the LLM loop. (Note: this is the chain-level
    /// WallClockCap variant; per-call wall-clock is enforced by the
    /// underlying ResilientLLMClient timeout, separate path.)
    HaltWallClockCap { aggregate_seconds: u64 },
}

/// TB-18 Atom A: Per-run tracker carrying the consecutive-trivial counter
/// + aggregate wall-clock accumulator. Construct ONCE per `drive_task`
/// invocation; call `on_response` after every `client.generate(...)` Ok
/// path to update counters and learn the verdict.
#[derive(Debug)]
pub struct LLMCallBudgetTracker {
    budget: PerCallBudget,
    consecutive_trivial: u32,
    run_start: Instant,
    /// Total LLM-call count across the run (substantive + trivial). For
    /// EvidenceCapsule budget-counters surfacing per FR-18.3.
    total_calls: u64,
    /// Trivial-response count across the run (regardless of whether
    /// consecutive). Useful for atom H aggregate reporting.
    trivial_calls: u64,
}

impl LLMCallBudgetTracker {
    pub fn new(budget: PerCallBudget) -> Self {
        Self {
            budget,
            consecutive_trivial: 0,
            run_start: Instant::now(),
            total_calls: 0,
            trivial_calls: 0,
        }
    }

    /// Record an LLM response and learn the budget verdict. Caller invokes
    /// after every successful `client.generate(...)` round-trip.
    pub fn on_response(&mut self, completion_tokens: u32) -> BudgetVerdict {
        self.total_calls += 1;
        let trivial = completion_tokens < self.budget.token_floor_threshold;
        if trivial {
            self.trivial_calls += 1;
            self.consecutive_trivial += 1;
            if self.consecutive_trivial >= self.budget.consecutive_trivial_response_cap {
                return BudgetVerdict::HaltDegradedLLM {
                    consecutive_trivial: self.consecutive_trivial,
                };
            }
        } else {
            self.consecutive_trivial = 0;
        }
        let elapsed = self.run_start.elapsed();
        if elapsed >= Duration::from_secs(self.budget.aggregate_per_run_wallclock_seconds) {
            return BudgetVerdict::HaltWallClockCap {
                aggregate_seconds: elapsed.as_secs(),
            };
        }
        BudgetVerdict::Continue
    }

    pub fn total_calls(&self) -> u64 {
        self.total_calls
    }
    pub fn trivial_calls(&self) -> u64 {
        self.trivial_calls
    }
    pub fn consecutive_trivial(&self) -> u32 {
        self.consecutive_trivial
    }
    pub fn elapsed(&self) -> Duration {
        self.run_start.elapsed()
    }
    pub fn budget(&self) -> PerCallBudget {
        self.budget
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn substantive_response_does_not_halt() {
        let mut t = LLMCallBudgetTracker::new(PerCallBudget::default());
        for _ in 0..100 {
            assert_eq!(t.on_response(500), BudgetVerdict::Continue);
        }
        assert_eq!(t.consecutive_trivial(), 0);
        assert_eq!(t.trivial_calls(), 0);
        assert_eq!(t.total_calls(), 100);
    }

    #[test]
    fn intermittent_trivial_does_not_halt() {
        // Default cap = 10 consecutive. 5 consecutive then a substantive
        // resets. Should never halt.
        let mut t = LLMCallBudgetTracker::new(PerCallBudget::default());
        for _ in 0..5 {
            assert_eq!(t.on_response(14), BudgetVerdict::Continue);
        }
        assert_eq!(t.consecutive_trivial(), 5);
        assert_eq!(t.on_response(500), BudgetVerdict::Continue);
        assert_eq!(t.consecutive_trivial(), 0);
        for _ in 0..5 {
            assert_eq!(t.on_response(14), BudgetVerdict::Continue);
        }
        assert_eq!(t.consecutive_trivial(), 5);
    }

    #[test]
    fn consecutive_trivial_cap_halts_with_degraded_llm() {
        let mut t = LLMCallBudgetTracker::new(PerCallBudget::default()); // cap=10
        // 9 trivials: still continuing.
        for _ in 0..9 {
            assert_eq!(t.on_response(14), BudgetVerdict::Continue);
        }
        assert_eq!(t.consecutive_trivial(), 9);
        // 10th trivial: halt.
        match t.on_response(14) {
            BudgetVerdict::HaltDegradedLLM {
                consecutive_trivial,
            } => {
                assert_eq!(consecutive_trivial, 10);
            }
            other => panic!("expected HaltDegradedLLM at 10th consecutive trivial, got {other:?}"),
        }
    }

    #[test]
    fn osb_m0_drift_signature_30_consecutive_halts() {
        // Architect §2.5 + OBS_M0 §3 reference scenario: P02 saw 30+
        // consecutive 14-output-token responses across 5+ minutes.
        // With default cap=10, halt MUST fire at the 10th — far before
        // the 30th, far before the external timeout.
        let mut t = LLMCallBudgetTracker::new(PerCallBudget::default());
        let mut halted_at: Option<u32> = None;
        for i in 1..=30 {
            match t.on_response(14) {
                BudgetVerdict::HaltDegradedLLM { .. } => {
                    halted_at = Some(i);
                    break;
                }
                BudgetVerdict::Continue => {}
                other => panic!("unexpected verdict at iteration {i}: {other:?}"),
            }
        }
        assert_eq!(halted_at, Some(10));
    }

    #[test]
    fn custom_threshold_via_constructor() {
        let budget = PerCallBudget {
            per_call_wallclock_seconds: 60,
            token_floor_threshold: 5,
            consecutive_trivial_response_cap: 3,
            aggregate_per_run_wallclock_seconds: 600,
        };
        let mut t = LLMCallBudgetTracker::new(budget);
        // 4-token response is trivial under threshold=5.
        assert_eq!(t.on_response(4), BudgetVerdict::Continue);
        assert_eq!(t.on_response(4), BudgetVerdict::Continue);
        match t.on_response(4) {
            BudgetVerdict::HaltDegradedLLM {
                consecutive_trivial,
            } => assert_eq!(consecutive_trivial, 3),
            other => panic!("expected halt at 3rd, got {other:?}"),
        }
    }

    #[test]
    fn budget_default_matches_architect_spec() {
        let b = PerCallBudget::default();
        assert_eq!(b.per_call_wallclock_seconds, 60);
        assert_eq!(b.token_floor_threshold, 30);
        assert_eq!(b.consecutive_trivial_response_cap, 10);
        assert_eq!(b.aggregate_per_run_wallclock_seconds, 600);
    }
}
